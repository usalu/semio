//#region 🔖️Recognizer
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
