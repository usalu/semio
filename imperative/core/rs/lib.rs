//! ⚙️ Imperative core: path host and WASM session.

pub use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, EffectLogEntry, Executor, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry, register};
pub use neural_engine::{Dictionary, Registry};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #region 🔖Document
/// 📍 Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

/// 📄 Imperative path document envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeDocument {
    pub schema: String,
    pub path: Path,
    #[serde(default)]
    pub seed: Dictionary,
}

impl Default for ImperativeDocument {
    fn default() -> Self {
        Self { schema: "imperative.document".into(), path: Path::new(), seed: Dictionary::new() }
    }
}

//#region 🔖Operation
/// @emoji ✂️ A step-collection edit at a `PathRef` — root path or a nested `control.*` step's slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeOperation {
    pub path_ref: PathRef,
    pub collection: vcs::CollectionOperation<String, Step, Dictionary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImperativeDiff(pub Option<ImperativeOperation>);

impl vcs::OperationDiff<ImperativeDocument> for ImperativeDiff {
    fn apply(&self, projection: &ImperativeDocument) -> ImperativeDocument {
        let mut next = projection.clone();
        if let Some(operation) = &self.0 {
            if let Some(steps) = resolve_steps_mut(&mut next, &operation.path_ref) {
                vcs::apply_collection_operation(steps, &operation.collection);
            }
            prune_empty_slot(&mut next, &operation.path_ref);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }
}

impl vcs::Operation<ImperativeDocument> for ImperativeOperation {
    type Diff = ImperativeDiff;

    fn diff(&self, _projection: &ImperativeDocument) -> Self::Diff {
        ImperativeDiff(Some(self.clone()))
    }

    fn backwards(&self, projection: &ImperativeDocument) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeOperation { path_ref: self.path_ref.clone(), collection: vcs::invert_collection_operation(steps, &self.collection) }],
            None => Vec::new(),
        }
    }
}

/// 🔎 Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as empty.
fn resolve_steps<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&document.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = document.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map(|path| path.steps.as_slice()).unwrap_or(&[]))
}

fn resolve_steps_mut<'a>(document: &'a mut ImperativeDocument, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut document.path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = document.path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹 Drops a nested slot's `bodies` entry once it's empty, so an emptied slot is bit-identical to
/// a never-touched one — required for `Add` then `Remove` to be a true, exact inverse pair.
fn prune_empty_slot(document: &mut ImperativeDocument, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else {
        return;
    };
    if let Some(owner_step) = document.path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|path| path.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}
//#endregion 🔖Operation

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer shared by `ImperativeDocument`'s `.imperative` DSL and by
/// `ImperativeOperation`'s compact single-line op-log encoding — both share the same `step`/dictionary
/// grammar (a step's nested `body <slot> { ... }` blocks recurse through the same step grammar; a
/// dictionary value that is itself a nested dictionary recurses through the same value grammar).
/// Whitespace (including newlines) is never significant to the parser — `print_dsl` inserts
/// newlines/indentation purely for readability, `print_op` renders the identical grammar on one line.
/// See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod imperative_text {
    use super::{ImperativeDocument, ImperativeOperation, Path, PathRef, Step};
    use neural_engine::{Atom, Dictionary, Value};
    use std::collections::BTreeMap;
    use vcs::CollectionOperation;

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: vcs::TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` and
    /// `.` are ordinary word characters — `key=value` and `state.set` each collapse into one token.
    fn lex(input: &str) -> Result<Vec<Lexed>, vcs::TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line = 1u32;
        let mut col = 1u32;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' | '\r' => {
                    i += 1;
                    col += 1;
                }
                '\n' => {
                    i += 1;
                    line += 1;
                    col = 1;
                }
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    let (start_line, start_col) = (line, col);
                    i += 1;
                    col += 1;
                    let mut s = String::new();
                    let mut closed = false;
                    while i < chars.len() {
                        let ch = chars[i];
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => s.push('\n'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                other => {
                                    s.push('\\');
                                    s.push(other);
                                }
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '"' {
                            i += 1;
                            col += 1;
                            closed = true;
                            break;
                        } else if ch == '\n' {
                            s.push(ch);
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            s.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    if !closed {
                        return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: vcs::TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: vcs::TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> vcs::TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_lbrace(&self) -> bool {
            matches!(self.peek(), Tok::LBrace)
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn at_keyword(&self, keyword: &str) -> bool {
            matches!(self.peek(), Tok::Word(w) if w == keyword)
        }

        fn expect_word(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(vcs::TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), vcs::TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(vcs::TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_lbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        fn expect_str(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }

        /// 🔑 Peeks the key of a pending `key=`/`key=value` word token without consuming it.
        fn peek_key(&self) -> Option<String> {
            match self.peek() {
                Tok::Word(w) => w.split_once('=').map(|(key, _)| key.to_string()),
                _ => None,
            }
        }

        /// 🗝️ Consumes a `key=`/`key=value` word token whose key must equal `key`, returning the
        /// inline suffix — empty when the value is a separate following token (a quoted string or a
        /// `{ }` block), non-empty when it is a bareword value collapsed into the same token.
        fn expect_kv(&mut self, key: &str) -> Result<String, vcs::TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            let (found, rest) = word
                .split_once('=')
                .ok_or_else(|| vcs::TextError::expected(format!("expected '{key}=...', found '{word}'"), span, format!("{key}=...")))?;
            if found != key {
                return Err(vcs::TextError::expected(format!("expected '{key}=...', found '{found}=...'"), span, format!("{key}=...")));
            }
            Ok(rest.to_string())
        }

        fn expect_kv_str(&mut self, key: &str) -> Result<String, vcs::TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if !rest.is_empty() {
                return Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), span, "string"));
            }
            self.expect_str()
        }

        fn expect_kv_word(&mut self, key: &str) -> Result<String, vcs::TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if rest.is_empty() {
                return Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), span, "word"));
            }
            Ok(rest)
        }

        fn expect_kv_opt_word(&mut self, key: &str) -> Result<Option<String>, vcs::TextError> {
            let word = self.expect_kv_word(key)?;
            Ok(if word == "-" { None } else { Some(word) })
        }

        fn expect_kv_usize(&mut self, key: &str) -> Result<usize, vcs::TextError> {
            let span = self.span();
            let word = self.expect_kv_word(key)?;
            word.parse::<usize>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
        }

        fn expect_kv_dict(&mut self, key: &str) -> Result<Dictionary, vcs::TextError> {
            let span = self.span();
            let rest = self.expect_kv(key)?;
            if !rest.is_empty() {
                return Err(vcs::TextError::expected(format!("field '{key}' must be a '{{ }}' block"), span, "{ }"));
            }
            parse_dict(self)
        }
    }
    //#endregion Parser

    //#region Dictionary
    fn quote(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    /// 🔢 Renders a decimal so it always carries a `.` — the round-trip discriminant between
    /// `Atom::Integer` and `Atom::Decimal` used by {@link parse_atom}.
    fn fmt_decimal(value: f64) -> String {
        if value.is_nan() {
            return "nan".to_string();
        }
        if value.is_infinite() {
            return if value > 0.0 { "inf".to_string() } else { "-inf".to_string() };
        }
        let printed = value.to_string();
        if printed.contains('.') {
            printed
        } else {
            format!("{printed}.0")
        }
    }

    fn parse_atom(word: &str, span: vcs::TextSpan) -> Result<Atom, vcs::TextError> {
        match word {
            "null" => Ok(Atom::Null),
            "true" => Ok(Atom::Boolean(true)),
            "false" => Ok(Atom::Boolean(false)),
            "nan" => Ok(Atom::Decimal(f64::NAN)),
            "inf" => Ok(Atom::Decimal(f64::INFINITY)),
            "-inf" => Ok(Atom::Decimal(f64::NEG_INFINITY)),
            _ if word.contains('.') || word.contains('e') || word.contains('E') => {
                word.parse::<f64>().map(Atom::Decimal).map_err(|_| vcs::TextError::expected(format!("invalid decimal '{word}'"), span, "number"))
            }
            _ => word.parse::<i64>().map(Atom::Integer).map_err(|_| vcs::TextError::expected(format!("invalid integer '{word}'"), span, "number")),
        }
    }

    fn print_atom(atom: &Atom) -> String {
        match atom {
            Atom::Null => "null".to_string(),
            Atom::Boolean(value) => value.to_string(),
            Atom::Integer(value) => value.to_string(),
            Atom::Decimal(value) => fmt_decimal(*value),
            Atom::String(value) => quote(value),
        }
    }

    fn parse_value(p: &mut Parser) -> Result<Value, vcs::TextError> {
        if p.at_lbrace() {
            return Ok(Value::Dictionary(parse_dict(p)?));
        }
        if let Tok::Str(_) = p.peek() {
            return Ok(Value::Atom(Atom::String(p.expect_str()?)));
        }
        let span = p.span();
        let word = p.expect_word()?;
        Ok(Value::Atom(parse_atom(&word, span)?))
    }

    /// 📥 Parses a `{ key=value ... }` dictionary block — `key=value` collapses into one bareword
    /// token, `key=` alone means the value is a separate following token (a quoted string or a
    /// nested `{ }`, itself parsed by recursing back into this function).
    fn parse_dict(p: &mut Parser) -> Result<Dictionary, vcs::TextError> {
        p.expect_lbrace()?;
        let mut dict = Dictionary::new();
        while !p.at_rbrace() {
            let span = p.span();
            let word = p.expect_word()?;
            let (key, rest) = word
                .split_once('=')
                .ok_or_else(|| vcs::TextError::expected(format!("expected 'key=value', found '{word}'"), span, "key=value"))?;
            let value = if rest.is_empty() { parse_value(p)? } else { Value::Atom(parse_atom(rest, span)?) };
            dict = dict.insert(key.to_string(), value);
        }
        p.expect_rbrace()?;
        Ok(dict)
    }

    fn print_value(value: &Value) -> String {
        match value {
            Value::Atom(atom) => print_atom(atom),
            Value::Dictionary(dict) => print_dict(dict),
        }
    }

    fn print_dict(dict: &Dictionary) -> String {
        let parts: Vec<String> = dict.keys().map(|key| format!("{key}={}", print_value(dict.get(key).expect("key came from dict.keys()")))).collect();
        if parts.is_empty() {
            "{ }".to_string()
        } else {
            format!("{{ {} }}", parts.join(" "))
        }
    }
    //#endregion Dictionary

    //#region Step
    fn indent(depth: usize) -> String {
        "  ".repeat(depth)
    }

    /// 🧱 Wraps already-rendered `items` in `{ }`, one per line indented at `depth + 1` when `pretty`,
    /// or space-joined on one line otherwise — mirrors `note_text::wrap_body`. Only ever called with a
    /// non-empty `items` (callers omit the section entirely when there is nothing to wrap).
    fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
        if pretty {
            let inner_pad = indent(depth + 1);
            let outer_pad = indent(depth);
            let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
            format!("{{\n{body}{outer_pad}}}")
        } else {
            format!("{{ {} }}", items.join(" "))
        }
    }

    fn parse_step(p: &mut Parser) -> Result<Step, vcs::TextError> {
        p.expect_keyword("step")?;
        let id = p.expect_kv_str("id")?;
        let kind = p.expect_kv_word("kind")?;
        let params = if p.peek_key().as_deref() == Some("params") { p.expect_kv_dict("params")? } else { Dictionary::new() };
        let bodies = if p.at_lbrace() {
            p.bump();
            let mut bodies = BTreeMap::new();
            while !p.at_rbrace() {
                p.expect_keyword("body")?;
                let slot = p.expect_word()?;
                p.expect_lbrace()?;
                let mut steps = Vec::new();
                while !p.at_rbrace() {
                    steps.push(parse_step(p)?);
                }
                p.expect_rbrace()?;
                bodies.insert(slot, Path { steps });
            }
            p.expect_rbrace()?;
            bodies
        } else {
            BTreeMap::new()
        };
        Ok(Step { id, kind, params, bodies })
    }

    fn print_step(step: &Step, depth: usize, pretty: bool) -> String {
        let mut out = format!("step id={} kind={}", quote(&step.id), step.kind);
        if !step.params.is_empty() {
            out.push_str(&format!(" params={}", print_dict(&step.params)));
        }
        if !step.bodies.is_empty() {
            let items: Vec<String> = step.bodies.iter().map(|(slot, path)| print_body(slot, path, depth + 1, pretty)).collect();
            out.push_str(&format!(" {}", wrap_body(&items, depth, pretty)));
        }
        out
    }

    fn print_body(slot: &str, path: &Path, depth: usize, pretty: bool) -> String {
        let items: Vec<String> = path.steps.iter().map(|step| print_step(step, depth + 1, pretty)).collect();
        format!("body {slot} {}", wrap_body(&items, depth, pretty))
    }
    //#endregion Step

    //#region Document
    /// 📥 Parses a full `.imperative` document: `imperative schema=...` (required), then an optional
    /// `seed={ ... }` dictionary and an optional `steps { ... }` list, in that order.
    pub(super) fn parse_document(text: &str) -> Result<ImperativeDocument, vcs::TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        p.expect_keyword("imperative")?;
        let schema = p.expect_kv_str("schema")?;
        let seed = if p.peek_key().as_deref() == Some("seed") { p.expect_kv_dict("seed")? } else { Dictionary::new() };
        let steps = if p.at_keyword("steps") {
            p.bump();
            p.expect_lbrace()?;
            let mut steps = Vec::new();
            while !p.at_rbrace() {
                steps.push(parse_step(&mut p)?);
            }
            p.expect_rbrace()?;
            steps
        } else {
            Vec::new()
        };
        Ok(ImperativeDocument { schema, path: Path { steps }, seed })
    }

    /// 📤 Renders `document` as `imperative schema=...` followed by `seed=`/`steps` sections that
    /// have content, joined by newlines when `pretty` or single spaces otherwise (mirrors
    /// `note_text::print_document`; see {@link parse_document} for the mirrored grammar).
    pub(super) fn print_document(document: &ImperativeDocument, pretty: bool) -> String {
        let mut parts = vec![format!("imperative schema={}", quote(&document.schema))];
        if !document.seed.is_empty() {
            parts.push(format!("seed={}", print_dict(&document.seed)));
        }
        if !document.path.steps.is_empty() {
            let items: Vec<String> = document.path.steps.iter().map(|step| print_step(step, 1, pretty)).collect();
            parts.push(format!("steps {}", wrap_body(&items, 0, pretty)));
        }
        parts.join(if pretty { "\n" } else { " " })
    }
    //#endregion Document

    //#region Operation
    /// ⚡ Parses one op-log line: `<add|remove|move|patch> owner=<id|-> slot=<id|-> ...`. `owner`/
    /// `slot` mirror `vcs`'s own `-` sentinel for an absent optional field.
    pub(super) fn parse_operation(line: &str) -> Result<ImperativeOperation, vcs::TextError> {
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;
        let owner = p.expect_kv_opt_word("owner")?;
        let slot = p.expect_kv_opt_word("slot")?;
        let path_ref = PathRef { owner, slot };
        let collection = match keyword.as_str() {
            "add" => {
                let index = p.expect_kv_usize("index")?;
                let item = parse_step(&mut p)?;
                CollectionOperation::Add { index, item }
            }
            "remove" => {
                let id = p.expect_kv_str("id")?;
                CollectionOperation::Remove { id }
            }
            "move" => {
                let id = p.expect_kv_str("id")?;
                let to_index = p.expect_kv_usize("to")?;
                CollectionOperation::Move { id, to_index }
            }
            "patch" => {
                let id = p.expect_kv_str("id")?;
                let patch = p.expect_kv_dict("patch")?;
                CollectionOperation::Patch { id, patch }
            }
            other => return Err(vcs::TextError::expected(format!("unknown operation '{other}'"), span, "add|remove|move|patch")),
        };
        Ok(ImperativeOperation { path_ref, collection })
    }

    /// 📤 Renders one `ImperativeOperation` as a single line (see {@link parse_operation}).
    pub(super) fn print_operation(operation: &ImperativeOperation) -> String {
        let path_ref = format!(
            "owner={} slot={}",
            operation.path_ref.owner.as_deref().unwrap_or("-"),
            operation.path_ref.slot.as_deref().unwrap_or("-"),
        );
        match &operation.collection {
            CollectionOperation::Add { index, item } => format!("add {path_ref} index={index} {}", print_step(item, 0, false)),
            CollectionOperation::Remove { id } => format!("remove {path_ref} id={}", quote(id)),
            CollectionOperation::Move { id, to_index } => format!("move {path_ref} id={} to={to_index}", quote(id)),
            CollectionOperation::Patch { id, patch } => format!("patch {path_ref} id={} patch={}", quote(id), print_dict(patch)),
        }
    }
    //#endregion Operation
}

impl vcs::DocumentDsl for ImperativeDocument {
    const EXTENSION: &'static str = "imperative";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        imperative_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        imperative_text::print_document(self, true)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for ImperativeOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        imperative_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        imperative_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region ⚠️ Errors
/// 🚨 Imperative core's fallible operations.
#[derive(Debug, thiserror::Error)]
pub enum ImperativeCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("missing owner")]
    MissingOwner,
    #[error("missing slot")]
    MissingSlot,
    #[error("unknown owner step: {0}")]
    UnknownOwnerStep(String),
    #[error("unknown step: {0}")]
    UnknownStep(String),
}
//#endregion ⚠️ Errors

/// 📄 The default `imperative` document, handcrafted in the `.imperative` DSL (see `🔖Dsl`) instead of
/// a hand-built Rust literal or a JSON fixture — {@link default_document} is the only way it should be
/// consumed.
const DEFAULT_IMPERATIVE_DOCUMENT_TEXT: &str = include_str!("../../example/default.imperative");

pub fn default_document() -> ImperativeDocument {
    <ImperativeDocument as vcs::DocumentDsl>::parse_dsl(DEFAULT_IMPERATIVE_DOCUMENT_TEXT)
        .expect("default.imperative is a static, hand-authored fixture that must always parse")
}
// #endregion 🔖Document

// #region 🔖Host
/// 🎛️ Native imperative path host.
pub struct ImperativeHost {
    pub document: ImperativeDocument,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_document(default_document())
    }
}

impl ImperativeHost {
    pub fn from_document(document: ImperativeDocument) -> Self {
        Self { document, registry: imperative_module_registry(), next_serial: 100 }
    }

    pub fn load_json(json: &str) -> Result<Self, ImperativeCoreError> {
        let document: ImperativeDocument = serde_json::from_str(json)?;
        if document.schema != "imperative.document" {
            return Err(ImperativeCoreError::UnsupportedSchema(document.schema));
        }
        Ok(Self::from_document(document))
    }

    pub fn to_json(&self) -> Result<String, ImperativeCoreError> {
        Ok(serde_json::to_string(&self.document)?)
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    fn resolve_path_mut<'a>(&'a mut self, path_ref: &PathRef) -> Result<&'a mut Path, ImperativeCoreError> {
        if path_ref.owner.is_none() && path_ref.slot.is_none() {
            return Ok(&mut self.document.path);
        }
        let owner = path_ref.owner.as_ref().ok_or(ImperativeCoreError::MissingOwner)?;
        let slot = path_ref.slot.as_ref().ok_or(ImperativeCoreError::MissingSlot)?;
        let owner_step = self.document.path.steps.iter_mut().find(|step| step.id == *owner).ok_or_else(|| ImperativeCoreError::UnknownOwnerStep(owner.clone()))?;
        Ok(owner_step.bodies.entry(slot.clone()).or_insert_with(Path::new))
    }

    pub fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.add_step_at(&PathRef::default(), kind, index).expect("root PathRef always resolves — resolve_path_mut only fails for a non-default owner/slot")
    }

    pub fn add_step_at(&mut self, path_ref: &PathRef, kind: &str, index: Option<usize>) -> Result<String, ImperativeCoreError> {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let path = self.resolve_path_mut(path_ref)?;
        let insert_at = index.unwrap_or(path.steps.len()).min(path.steps.len());
        path.steps.insert(insert_at, step);
        Ok(id)
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        self.remove_step_at(&PathRef::default(), id)
    }

    pub fn remove_step_at(&mut self, path_ref: &PathRef, id: &str) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let before = path.steps.len();
        path.steps.retain(|step| step.id != id);
        path.steps.len() != before
    }

    pub fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        self.move_step_at(&PathRef::default(), id, new_index)
    }

    pub fn move_step_at(&mut self, path_ref: &PathRef, id: &str, new_index: usize) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let Some(current) = path.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        let step = path.steps.remove(current);
        let insert_at = new_index.min(path.steps.len());
        path.steps.insert(insert_at, step);
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = serde_json::from_str(json)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        Ok(())
    }

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.document.path, &self.document.seed)
    }

    pub fn compile_text(&self) -> String {
        compile_to_text(&self.document.path)
    }
}
// #endregion 🔖Host

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    struct ImperativeSessionInner {
        host: ImperativeHost,
    }

    #[wasm_bindgen]
    pub struct ImperativeSession {
        state: Rc<RefCell<ImperativeSessionInner>>,
    }

    #[wasm_bindgen]
    impl ImperativeSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(ImperativeSessionInner { host: ImperativeHost::default() })) }
        }

        #[wasm_bindgen(js_name = loadPathJson)]
        pub fn load_path_json(&self, json: &str) -> Result<(), JsValue> {
            let host = ImperativeHost::load_json(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = pathJson)]
        pub fn path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, index: Option<usize>) -> String {
            self.state.borrow_mut().host.add_step(kind, index)
        }

        #[wasm_bindgen(js_name = addStepAt)]
        pub fn add_step_at(&self, path_ref_json: &str, kind: &str, index: Option<usize>) -> Result<String, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.add_step_at(&path_ref, kind, index).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = removeStepAt)]
        pub fn remove_step_at(&self, path_ref_json: &str, id: &str) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.remove_step_at(&path_ref, id))
        }

        #[wasm_bindgen(js_name = moveStep)]
        pub fn move_step(&self, id: &str, new_index: usize) -> bool {
            self.state.borrow_mut().host.move_step(id, new_index)
        }

        #[wasm_bindgen(js_name = moveStepAt)]
        pub fn move_step_at(&self, path_ref_json: &str, id: &str, new_index: usize) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.move_step_at(&path_ref, id, new_index))
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = setStepParamsAt)]
        pub fn set_step_params_at(&self, path_ref_json: &str, id: &str, json: &str) -> Result<(), JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.set_step_params_at(&path_ref, id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen]
        pub fn run(&self) -> Result<String, JsValue> {
            let result = self.state.borrow().host.run();
            serde_json::to_string(&result).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = compileText)]
        pub fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
        }
    }
}
// #endregion 🔖WasmSession

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_runs_default_document() {
        let host = ImperativeHost::default();
        let result = host.run();
        assert_eq!(result.effects.len(), 2);
        assert!(result.effects.iter().all(|entry| entry.error.is_none()));
    }

    #[test]
    fn host_adds_nested_step_in_control_body() {
        let mut host = ImperativeHost::default();
        let owner = host.add_step("control.if", None);
        let path_ref = PathRef { owner: Some(owner.clone()), slot: Some("then".into()) };
        let nested = host.add_step_at(&path_ref, "log.print", None).expect("add nested");
        assert_eq!(nested, "step-102");
        let owner_step = host.document.path.steps.iter().find(|step| step.id == owner).expect("owner");
        assert_eq!(owner_step.bodies.get("then").map(|path| path.steps.len()), Some(1));
    }

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn add_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-x", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn remove_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Remove { id: "step-1".into() } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn move_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Move { id: "step-1".into(), to_index: 1 } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn patch_step_params_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Patch { id: "step-1".into(), patch: Dictionary::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))) } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn add_step_into_nested_control_body_round_trips() {
        let mut document = default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        let operation = ImperativeOperation { path_ref: path_ref.clone(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-nested", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        let post = vcs::apply_operation(&document, &operation);
        let owner_step = post.path.steps.iter().find(|entry| entry.id == "step-if").expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn default_document_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&default_document());
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        let document = default_document();
        let envelope = vcs::create_document_vcs_envelope::<ImperativeDocument, ImperativeOperation>("imperative.document/v1", "test", document, None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-x", "log.print") } };
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![operation], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
}
//#endregion 🧪Tests
