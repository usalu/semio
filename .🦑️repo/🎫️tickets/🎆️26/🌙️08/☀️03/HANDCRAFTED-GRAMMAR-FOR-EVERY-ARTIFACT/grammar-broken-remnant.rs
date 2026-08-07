//! @emoji 📖️ `dsl_grammar` — the self-hosted `.grammar` spec format: a hand-authorable,
//! EBNF-style description of one language's productions, used as the *normative* artifact every
//! handcrafted grammar in the repo ships alongside its parser/printer. This crate parses and
//! prints the format itself (this crate's own `📖️grammar/📖️grammar.grammar` is written in it and
//! parses cleanly under its own parser — see the `self_hosting` test), and provides a recognizer
//! that can check a target document's tokens against a compiled grammar for the subset of
//! productions this v1 supports (see `//#region 🔖️Recognizer
/// @emoji 🧭️ Recognizer with explicit terminal predicates, family fragment merge, and macro matchers.
pub struct MacroMatcher {
    pub name: &'static str,
    pub try_match: fn(&str) -> bool,
}

/// @emoji 🧩️ Named grammar fragments (family kits) merged into Recognizer::compile_with.
#[derive(Default, Clone)]
pub struct FragmentRegistry {
    fragments: std::collections::HashMap<String, GrammarFile>,
}

impl FragmentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: impl Into<String>, grammar: GrammarFile) {
        self.fragments.insert(name.into(), grammar);
    }

    pub fn get(&self, name: &str) -> Option<&GrammarFile> {
        self.fragments.get(name)
    }
}

pub struct Recognizer {
    grammar: GrammarFile,
    macros: Vec<MacroMatcher>,
}

impl Recognizer {
    pub fn compile(grammar: &GrammarFile) -> Self {
        Self::compile_with(grammar, &FragmentRegistry::new())
    }

    /// @emoji 🔗️ Compile grammar, merging productions from each use via registry.
    pub fn compile_with(grammar: &GrammarFile, registry: &FragmentRegistry) -> Self {
        let mut merged = grammar.clone();
        let mut seen = std::collections::HashSet::<String>::new();
        for p in &grammar.productions {
            seen.insert(p.name.clone());
        }
        for use_name in &grammar.uses {
            if let Some(frag) = registry.get(use_name) {
                for prod in &frag.productions {
                    if seen.insert(prod.name.clone()) {
                        merged.productions.push(prod.clone());
                    }
                }
            }
        }
        Self {
            grammar: merged,
            macros: default_macros(),
        }
    }

    fn find_production(&self, name: &str) -> Option<&Production> {
        self.grammar.productions.iter().find(|p| p.name == name)
    }

    /// @emoji ✅️ Recognizes text against the grammar start production.
    pub fn recognize(&self, text: &str) -> Result<bool, TextError> {
        let raw = core_lex(text, &Limits::default(), false)?;
        let tokens: Vec<_> = raw
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof)
            .collect();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| {
            TextError::new(
                format!("start production `{}` not found", self.grammar.start),
                TextSpan::at(1, 1),
            )
        })?;
        match self.match_production(start, &tokens, 0) {
            Some(pos) => Ok(pos == tokens.len()),
            None => Ok(false),
        }
    }

    /// @emoji 📊️ Productions never reached while recognizing text.
    pub fn uncovered_productions(&self, text: &str) -> Result<Vec<String>, TextError> {
        let raw = core_lex(text, &Limits::default(), false)?;
        let tokens: Vec<_> = raw
            .into_iter()
            .filter(|t| !t.kind.is_trivia() && t.kind != CoreKind::Eof)
            .collect();
        let mut covered = std::collections::HashSet::<String>::new();
        let start = self.find_production(&self.grammar.start).ok_or_else(|| {
            TextError::new(
                format!("start production `{}` not found", self.grammar.start),
                TextSpan::at(1, 1),
            )
        })?;
        let _ = self.match_production_tracked(start, &tokens, 0, &mut covered);
        Ok(self
            .grammar
            .productions
            .iter()
            .map(|p| p.name.clone())
            .filter(|n| !covered.contains(n))
            .collect())
    }

    fn match_production(
        &self,
        production: &Production,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
    ) -> Option<usize> {
        let mut covered = std::collections::HashSet::new();
        self.match_production_tracked(production, tokens, pos, &mut covered)
    }

    fn match_production_tracked(
        &self,
        production: &Production,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        for alt in &production.alternatives {
            if let Some(next) = self.match_sequence_tracked(&alt.symbols, tokens, pos, covered) {
                covered.insert(production.name.clone());
                return Some(next);
            }
        }
        None
    }

    fn match_sequence_tracked(
        &self,
        symbols: &[Symbol],
        tokens: &[crate::os_dsl::core::SpannedToken],
        mut pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        for symbol in symbols {
            pos = self.match_symbol_tracked(symbol, tokens, pos, covered)?;
        }
        Some(pos)
    }

    fn match_symbol_tracked(
        &self,
        symbol: &Symbol,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
        covered: &mut std::collections::HashSet<String>,
    ) -> Option<usize> {
        match symbol {
            Symbol::Literal(text) => {
                let token = tokens.get(pos)?;
                (token.text.as_str().as_ref() == text.as_str()).then_some(pos + 1)
            }
            Symbol::Terminal(name) => {
                let token = tokens.get(pos)?;
                terminal_matches(name, token).then_some(pos + 1)
            }
            Symbol::Ref(name) => {
                if let Some(production) = self.find_production(name) {
                    self.match_production_tracked(production, tokens, pos, covered)
                } else if let Some(matcher) = self.macros.iter().find(|m| m.name == name) {
                    self.match_macro_span(matcher, tokens, pos)
                } else {
                    None
                }
            }
            Symbol::Macro(name, _args) => {
                let matcher = self.macros.iter().find(|m| &m.name == name)?;
                self.match_macro_span(matcher, tokens, pos)
            }
            Symbol::Group(alts) => alts
                .iter()
                .find_map(|alt| self.match_sequence_tracked(&alt.symbols, tokens, pos, covered)),
            Symbol::Optional(inner) => {
                Some(self.match_symbol_tracked(inner, tokens, pos, covered).unwrap_or(pos))
            }
            Symbol::Star(inner) => {
                let mut cur = pos;
                while let Some(next) = self.match_symbol_tracked(inner, tokens, cur, covered) {
                    if next == cur {
                        break;
                    }
                    cur = next;
                }
                Some(cur)
            }
            Symbol::Plus(inner) => {
                let first = self.match_symbol_tracked(inner, tokens, pos, covered)?;
                let mut cur = first;
                loop {
                    match self.match_symbol_tracked(inner, tokens, cur, covered) {
                        Some(next) if next != cur => cur = next,
                        _ => break,
                    }
                }
                Some(cur)
            }
        }
    }

    fn match_macro_span(
        &self,
        matcher: &MacroMatcher,
        tokens: &[crate::os_dsl::core::SpannedToken],
        pos: usize,
    ) -> Option<usize> {
        for end in pos + 1..=tokens.len() {
            let slice_text = slice_source_text(&tokens[pos..end]);
            if (matcher.try_match)(&slice_text) {
                return Some(end);
            }
        }
        None
    }
}

fn slice_source_text(tokens: &[crate::os_dsl::core::SpannedToken]) -> String {
    tokens
        .iter()
        .map(|t| t.text.as_str().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// @emoji 🏷️ Explicit terminal predicates — BOOL is Ident true|false.
fn terminal_matches(name: &str, token: &crate::os_dsl::core::SpannedToken) -> bool {
    let upper = name.to_uppercase();
    let text = token.text.as_str();
    match upper.as_str() {
        "BOOL" => matches!(token.kind, CoreKind::Ident) && (text == "true" || text == "false"),
        "IDENT" | "PLACEHOLDER" => matches!(token.kind, CoreKind::Ident | CoreKind::Placeholder),
        "INT" => matches!(token.kind, CoreKind::Int),
        "FLOAT" => matches!(token.kind, CoreKind::Float),
        "TEXT" | "STRING" => matches!(token.kind, CoreKind::Text),
        "STAR" => matches!(token.kind, CoreKind::Star),
        "PLUS" => matches!(token.kind, CoreKind::Plus),
        "EQUALS" | "EQ" => matches!(token.kind, CoreKind::Equals) || text == "=",
        "ARROW" => text == "->" || text == "→",
        "DASHARROW" => text == "-->" || text == "⟶",
        "BACKARROW" => text == "<-" || text == "←",
        "EDGEARROW" => text == "<->" || text == "<-->" || text == "↔",
        "QUANTITY" => matches!(token.kind, CoreKind::Float | CoreKind::Int),
        "VEC3" | "COLOR" | "POINT" | "UNIT" => {
            matches!(
                token.kind,
                CoreKind::Ident | CoreKind::Float | CoreKind::Int | CoreKind::Text
            )
        }
        other => format!("{:?}", token.kind).to_uppercase() == other,
    }
}

fn macro_table_ok(text: &str) -> bool {
    let t = text.trim();
    t.contains('|') || t.starts_with("table")
}

fn macro_quantity_ok(text: &str) -> bool {
    let parts: Vec<_> = text.split_whitespace().collect();
    !parts.is_empty()
        && parts[0]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit() || c == '-' || c == '.')
}

fn macro_props_ok(text: &str) -> bool {
    text.contains('=')
}

fn default_macros() -> Vec<MacroMatcher> {
    vec![
        MacroMatcher {
            name: "edge",
            try_match: |text| crate::os_dsl::notation::parse_edge_text(text).is_ok(),
        },
        MacroMatcher {
            name: "table",
            try_match: macro_table_ok,
        },
        MacroMatcher {
            name: "quantity",
            try_match: macro_quantity_ok,
        },
        MacroMatcher {
            name: "props",
            try_match: macro_props_ok,
        },
    ]
}
//#endregion 🔖️Recognizer



//#region 📡️ProtocolWalk
fn mismatch(offset: usize, message: impl Into<String>) -> ProtocolMismatch {
    ProtocolMismatch { offset, message: message.into() }
}

fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolMismatch> {
    let mut result = 0u64;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() {
            return Err(mismatch(*pos, "truncated varint"));
        }
        let byte = bytes[*pos];
        *pos += 1;
        result |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(mismatch(*pos, "varint overflow"));
        }
    }
}

fn need<'a>(bytes: &'a [u8], pos: usize, n: usize, what: &str) -> Result<&'a [u8], ProtocolMismatch> {
    if pos + n > bytes.len() {
        return Err(mismatch(pos, format!("truncated {what}: need {n} bytes, have {}", bytes.len().saturating_sub(pos))));
    }
    Ok(&bytes[pos..pos + n])
}

fn trailing_reserved(blocks: &[Block], from: usize) -> usize {
    let mut reserved = 0usize;
    for block in &blocks[from..] {
        match block {
            Block::Footer(n) => reserved += *n,
            Block::Chain(prim) => reserved += prim_fixed_width(prim).unwrap_or(0),
            Block::Struct { .. } | Block::Enum { .. } => {}
            Block::Header(_) | Block::Segment { .. } | Block::Record { .. } => break,
        }
    }
    reserved
}

fn resolve_count(count: &Count, env: &std::collections::HashMap<String, u64>, offset: usize) -> Result<usize, ProtocolMismatch> {
    match count {
        Count::Fixed(n) => Ok(*n),
        Count::Varint => Err(mismatch(offset, "Count::Varint must be read from the byte stream, not resolved from env")),
        Count::Field(name) => env.get(name).map(|v| *v as usize).ok_or_else(|| mismatch(offset, format!("unknown count field `{name}`"))),
    }
}

fn walk_prim(
    prim: &Prim,
    bytes: &[u8],
    pos: &mut usize,
    env: &mut std::collections::HashMap<String, u64>,
    reserved_tail: usize,
) -> Result<(), ProtocolMismatch> {
    match prim {
        Prim::U8 => {
            need(bytes, *pos, 1, "u8")?;
            *pos += 1;
        }
        Prim::U16 => {
            need(bytes, *pos, 2, "u16")?;
            *pos += 2;
        }
        Prim::U32 | Prim::I32 | Prim::F32 => {
            need(bytes, *pos, 4, "u32/i32/f32")?;
            *pos += 4;
        }
        Prim::U64 | Prim::I64 | Prim::F64 => {
            need(bytes, *pos, 8, "u64/i64/f64")?;
            *pos += 8;
        }
        Prim::Fixed(n) => {
            need(bytes, *pos, *n, "fixed")?;
            *pos += *n;
        }
        Prim::Varint | Prim::Tag | Prim::Zigzag => {
            let _ = read_varint_u64(bytes, pos)?;
        }
        Prim::Bytes | Prim::Utf8 => {
            let end = bytes.len().saturating_sub(reserved_tail);
            if *pos > end {
                return Err(mismatch(*pos, "bytes field overlaps trailing reserved region"));
            }
            *pos = end;
        }
        Prim::Array(inner, count) => {
            let n = match count {
                Count::Varint => read_varint_u64(bytes, pos)? as usize,
                other => resolve_count(other, env, *pos)?,
            };
            if matches!(inner.as_ref(), Prim::U8) {
                need(bytes, *pos, n, "byte array")?;
                *pos += n;
            } else {
                for _ in 0..n {
                    walk_prim(inner, bytes, pos, env, reserved_tail)?;
                }
            }
        }
        Prim::Ref(name) => return Err(mismatch(*pos, format!("unresolved protocol Ref({name}) during walk"))),
    }
    Ok(())
}

fn walk_fields(fields: &[Field], bytes: &[u8], pos: &mut usize, reserved_tail: usize) -> Result<(), ProtocolMismatch> {
    let mut env = std::collections::HashMap::new();
    for (index, field) in fields.iter().enumerate() {
        let field_reserved = if index + 1 == fields.len() {
            reserved_tail
        } else {
            reserved_tail
                + fields[index + 1..]
                    .iter()
                    .map(|f| prim_fixed_width(&f.ty).unwrap_or(0))
                    .sum::<usize>()
        };
        match &field.ty {
            Prim::U8 => {
                let slice = need(bytes, *pos, 1, &field.name)?;
                env.insert(field.name.clone(), u64::from(slice[0]));
                *pos += 1;
            }
            Prim::U16 => {
                let slice = need(bytes, *pos, 2, &field.name)?;
                env.insert(field.name.clone(), u64::from(u16::from_le_bytes([slice[0], slice[1]])));
                *pos += 2;
            }
            Prim::U32 => {
                let slice = need(bytes, *pos, 4, &field.name)?;
                env.insert(field.name.clone(), u64::from(u32::from_le_bytes(slice.try_into().unwrap())));
                *pos += 4;
            }
            Prim::U64 => {
                let slice = need(bytes, *pos, 8, &field.name)?;
                env.insert(field.name.clone(), u64::from_le_bytes(slice.try_into().unwrap()));
                *pos += 8;
            }
            Prim::Varint | Prim::Tag | Prim::Zigzag => {
                let value = read_varint_u64(bytes, pos)?;
                env.insert(field.name.clone(), value);
            }
            other => walk_prim(other, bytes, pos, &mut env, field_reserved)?,
        }
    }
    Ok(())
}

fn definitions_only(block: &Block) -> bool {
    matches!(block, Block::Struct { .. } | Block::Enum { .. })
}

/// @emoji 🧭️ Spec-driven byte walker — consumes every declared wire slot and must finish at
/// exactly `bytes.len()`, else returns [`ProtocolMismatch`] with the failing offset.
pub fn walk_protocol(spec: &ProtocolFile, bytes: &[u8]) -> Result<ProtocolTrace, ProtocolMismatch> {
    let mut pos = 0usize;
    match &spec.framing {
        Framing::Magic(magic) => {
            let got = need(bytes, 0, 8, "magic")?;
            if got != magic {
                return Err(mismatch(0, format!("magic mismatch: expected {magic:?}, got {got:?}")));
            }
            pos = 8;
        }
        Framing::Record | Framing::Chunked => {}
    }

    let skip_records = matches!(spec.framing, Framing::Magic(_) | Framing::Chunked);

    for (index, block) in spec.blocks.iter().enumerate() {
        if definitions_only(block) {
            continue;
        }
        if skip_records && matches!(block, Block::Record { .. }) {
            continue;
        }
        let reserved = trailing_reserved(&spec.blocks, index + 1);
        match block {
            Block::Header(fields) => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Segment { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Record { fields, .. } => walk_fields(fields, bytes, &mut pos, reserved)?,
            Block::Footer(size) => {
                need(bytes, pos, *size, "footer")?;
                pos += *size;
            }
            Block::Chain(prim) => {
                let mut env = std::collections::HashMap::new();
                walk_prim(prim, bytes, &mut pos, &mut env, 0)?;
            }
            Block::Struct { .. } | Block::Enum { .. } => {}
        }
    }

    if pos != bytes.len() {
        return Err(mismatch(pos, format!("trailing {} bytes after protocol walk", bytes.len() - pos)));
    }
    Ok(ProtocolTrace { consumed: pos })
}

/// @emoji 📡️ Byte-level protocol conformance via [`walk_protocol`].
pub fn verify_protocol_bytes(spec: &ProtocolFile, bytes: &[u8]) -> Result<(), String> {
    walk_protocol(spec, bytes).map(|_| ()).map_err(|e| format!("offset {}: {}", e.offset, e.message))
}

/// @emoji 📡️ Parses handcrafted `.protocol.semio` source then verifies bytes (M5 pack/spr law).
pub fn verify_protocol_source(source: &str, bytes: &[u8]) -> Result<(), String> {
    let spec = parse_protocol(source).map_err(|error| error.message)?;
    verify_protocol_bytes(&spec, bytes)
}

/// @emoji 🛟 Shallow envelope check — any 0x89 magic (pack) or non-empty (spr).
pub fn verify_protocol_envelope(framing_hint: &str, bytes: &[u8]) -> Result<(), String> {
    if framing_hint.contains("record") || framing_hint.ends_with(".spr") {
        if bytes.is_empty() {
            return Err("spr envelope rejects empty bytes".into());
        }
        return Ok(());
    }
    if bytes.len() < 8 || bytes[0] != 0x89 {
        return Err(format!("pack envelope requires 0x89 magic prefix, got {} bytes", bytes.len()));
    }
    Ok(())
}
//#endregion 📡️ProtocolWalk

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_grammar_header() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = \"hello\"\n").expect("parse_grammar");
        assert_eq!(g.id, "demo");
        assert_eq!(g.start, "doc");
        assert_eq!(g.productions.len(), 1);
        assert_eq!(g.productions[0].alternatives[0].symbols, vec![Symbol::Literal("hello".to_string())]);
    }

    #[test]
    fn parses_extension_and_uses() {
        let g = parse_grammar("grammar fem2d\nextension fem2d\nuse core\nuse family-sheet\nstart document\ndocument = header\nheader = \"fem2d\" TEXT\n")
            .expect("parse_grammar");
        assert_eq!(g.extension, Some("fem2d".to_string()));
        assert_eq!(g.uses, vec!["core".to_string(), "family-sheet".to_string()]);
        assert_eq!(g.productions.len(), 2);
    }

    #[test]
    fn parses_terminal_vs_ref_vs_macro() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = TEXT node table(\"rows\", row)\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        assert_eq!(symbols[0], Symbol::Terminal("TEXT".to_string()));
        assert_eq!(symbols[1], Symbol::Ref("node".to_string()));
        assert_eq!(symbols[2], Symbol::Macro("table".to_string(), vec![MacroArg::Literal("rows".to_string()), MacroArg::Ident("row".to_string())]));
    }

    #[test]
    fn parses_alternation_group_and_quantifiers() {
        let g = parse_grammar("grammar demo\nstart doc\ndoc = {\"a\" | \"b\"}? node* row+\nnode = IDENT\nrow = IDENT\n").expect("parse_grammar");
        let symbols = &g.productions[0].alternatives[0].symbols;
        match &symbols[0] {
            Symbol::Optional(inner) => match inner.as_ref() {
                Symbol::Group(alts) => {
                    assert_eq!(alts.len(), 2);
                    assert_eq!(alts[0].symbols, vec![Symbol::Literal("a".to_string())]);
                    assert_eq!(alts[1].symbols, vec![Symbol::Literal("b".to_string())]);
                }
                other => panic!("expected Group, got {other:?}"),
            },
            other => panic!("expected Optional, got {other:?}"),
        }
        assert!(matches!(&symbols[1], Symbol::Star(_)));
        assert!(matches!(&symbols[2], Symbol::Plus(_)));
    }

    #[test]
    fn round_trip_matrix_over_representative_grammars() {
        let sources = vec![
            "grammar demo\nstart doc\ndoc = \"hello\"\n",
            "grammar fem2d\nextension fem2d\nuse core\nstart document\ndocument = header body\nheader = \"fem2d\" TEXT\nbody = row*\nrow = IDENT FLOAT?\n",
            "grammar demo\nstart doc\ndoc = {\"a\" | \"b\"} node+\nnode = IDENT\n",
        ];
        for source in sources {
            let parsed = parse_grammar(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_grammar(&parsed);
            let reparsed = parse_grammar(&printed).unwrap_or_else(|e| panic!("reparse of canonical {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, parsed, "round trip mismatch for {source:?} -> {printed:?}");
            let canonical_twice = canonicalize(&printed).expect("canonicalize");
            assert_eq!(canonical_twice, printed, "canonicalize is not idempotent for {printed:?}");
        }
    }

    #[test]
    fn missing_start_directive_is_an_error() {
        let err = parse_grammar("grammar demo\ndoc = \"hello\"\n").unwrap_err();
        assert!(err.message.contains("start"), "unexpected message: {}", err.message);
    }

    /// @emoji 🪞️ This crate's own format description parses under the parser it defines — the
    /// self-hosting proof the architecture plan calls for.
    #[test]
    fn self_hosting_grammar_grammar_parses_and_round_trips() {
        let source = include_str!("📖️grammar.grammar.semio");
        let parsed = parse_grammar(source).expect("dsl_grammar's own grammar.grammar must parse under its own parser");
        assert_eq!(parsed.id, "grammar");
        let printed = print_grammar(&parsed);
        let reparsed = parse_grammar(&printed).expect("canonical print of grammar.grammar must reparse");
        assert_eq!(reparsed, parsed);
    }

    #[test]
    fn recognizer_matches_plain_arrow_via_registered_edge_macro() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = edge\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("a->b").expect("recognize"));
        assert!(recognizer.recognize("a -[e1:Connection]->b").expect("recognize"));
        assert!(!recognizer.recognize("a-> ->").expect("recognize"));
    }

    #[test]
    fn recognizer_matches_literals_terminals_and_quantifiers() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = \"beam\" IDENT node*\nnode = IDENT\n").expect("parse_grammar");
        let recognizer = Recognizer::compile(&grammar);
        assert!(recognizer.recognize("beam e3 n1 n2").expect("recognize"));
        assert!(recognizer.recognize("beam e3").expect("recognize"));
        assert!(!recognizer.recognize("beam").expect("recognize"));
    }

    #[test]
    fn parse_grammar_sets_dialect_grammar_vs_protocol() {
        let g = parse_grammar("dialect grammar\ngrammar demo\nstart doc\ndoc = \"x\"\n").expect("grammar");
        assert_eq!(g.dialect, SemioDialect::Grammar);
        let p = parse_grammar(
            "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo\nstart frame\nframing magic 0x8953504B0D0A1A0A\nheader fixed 4\nfield flags u32\n",
        )
        .expect("protocol");
        assert_eq!(p.dialect, SemioDialect::Protocol);
        assert_eq!(p.start, "frame");
        assert_eq!(p.id, "demo.pack");
    }

    #[test]
    fn protocol_parse_print_round_trip_retains_body() {
        let source = r#"dialect protocol
protocol flow.pack
version 1
schema flow
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
record field id u16 type tag
field tag varint
field body bytes
footer fixed 84
"#;
        let parsed = parse_protocol(source).expect("parse_protocol");
        assert_eq!(parsed.id, "flow.pack");
        assert!(matches!(parsed.framing, Framing::Magic(_)));
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Header(_))));
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Segment { .. })));
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Footer(84))));
        let printed = print_protocol(&parsed);
        let reparsed = parse_protocol(&printed).expect("reparse print_protocol");
        assert_eq!(reparsed, parsed);
        let once = canonicalize(source).expect("canonicalize");
        let twice = canonicalize(&once).expect("canonicalize twice");
        assert_eq!(once, twice);
    }

    #[test]
    fn protocol_parses_rich_struct_enum_segment_forms() {
        let source = r#"dialect protocol
protocol demo.pack
version 1
schema demo
start frame
framing magic 0x8953504B0D0A1A0A
struct Vertex { x f32 y f32 z f32 }
enum Op { ObjectsAdd=1 ObjectsRemove=2 }
segment Objects kind=1 { count varint items Array(Ref(Object), Field(count)) }
footer fixed 84
"#;
        let parsed = parse_protocol(source).expect("parse rich protocol");
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Struct { name, .. } if name == "Vertex")));
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Enum { name, .. } if name == "Op")));
        assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Segment { name, kind: Some(1), .. } if name == "Objects")));
        let printed = print_protocol(&parsed);
        assert_eq!(parse_protocol(&printed).expect("reparse"), parsed);
    }

    #[test]
    fn walk_protocol_shape_a_spk_like_buffer() {
        let source = r#"dialect protocol
protocol demo.pack
version 1
schema demo
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 12
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
footer fixed 84
"#;
        let spec = parse_protocol(source).expect("parse");
        let mut bytes = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&0u16.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.push(1);
        bytes.push(0);
        bytes.push(0);
        bytes.extend(std::iter::repeat(0u8).take(84));
        let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
        assert_eq!(trace.consumed, bytes.len());
        verify_protocol_bytes(&spec, &bytes).expect("verify");
        let mut bad = bytes.clone();
        bad[0] = 0x00;
        assert!(walk_protocol(&spec, &bad).is_err());
    }

    #[test]
    fn walk_protocol_minimal_op_binary_record() {
        let source = r#"dialect protocol
protocol demo.spr
version 1
schema demo.operation
start record
framing record
field format u8
field ordinal varint
field body bytes
"#;
        let spec = parse_protocol(source).expect("parse spr");
        let bytes = vec![1u8, 0x00, 0xAA, 0xBB];
        let trace = walk_protocol(&spec, &bytes).expect("walk OpBinary");
        assert_eq!(trace.consumed, 4);
        assert!(walk_protocol(&spec, &[]).is_err());
    }

    #[test]
    fn self_hosting_protocol_grammar_semio_parses_as_grammar() {
        let source = include_str!("📖️protocol.grammar.semio");
        let parsed = parse_grammar(source).expect("protocol.grammar.semio must parse as dialect grammar");
        assert_eq!(parsed.dialect, SemioDialect::Grammar);
        assert_eq!(parsed.id, "protocol");
        let printed = print_grammar(&parsed);
        let reparsed = parse_grammar(&printed).expect("canonical protocol grammar reparses");
        assert_eq!(reparsed, parsed);
    }

}

    #[test]
    fn parse_protocol_roundtrips_magic_pack() {
        let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
        let parsed = parse_protocol(source).expect("parse_protocol");
        let printed = print_protocol(&parsed);
        let reparsed = parse_protocol(&printed).expect("reparse");
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn walk_protocol_consumes_magic_and_header() {
        let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
        let spec = parse_protocol(source).expect("parse");
        let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&7u32.to_le_bytes());
        walk_protocol(&spec, &bytes).expect("walk");
        assert!(walk_protocol(&spec, &bytes[..8]).is_err());
    }

    #[test]
    fn walk_protocol_spr_record_body_as_rest() {
        let source = "dialect protocol\nprotocol demo.spr\nversion 1\nschema demo.op\nstart record\nframing record\nfield format u8\nfield body bytes\n";
        let spec = parse_protocol(source).expect("parse");
        walk_protocol(&spec, &[1u8, 9, 9, 9]).expect("spr walk");
    }

    #[test]
    fn recognizer_matches_bool_terminal() {
        let grammar = parse_grammar("grammar demo\nstart doc\ndoc = BOOL\n").expect("grammar");
        let rec = Recognizer::compile(&grammar);
        assert_eq!(rec.recognize("true").unwrap(), true);
        assert_eq!(rec.recognize("false").unwrap(), true);
        assert_eq!(rec.recognize("maybe").unwrap(), false);
    }

    #[test]
    fn verify_protocol_source_ok() {
        let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
        let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
        bytes.extend_from_slice(&0u32.to_le_bytes());
        verify_protocol_source(source, &bytes).expect("verify_protocol_source");
    }

//#endregion 🔖️Tests
