//! ✍️ Lightweight grammar tokenization for writer plugin scenes.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrammarToken {
    pub class: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
struct GrammarRule {
    pattern: regex::Regex,
    class: &'static str,
}

fn jack_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"(?i)\b(MATCH|WHERE|RETURN|CREATE|DELETE|SET|MERGE|AND|OR)\b").expect("jack keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("jack string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("jack number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"->|!=|[:=.,\[\]()-]").expect("jack operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").expect("jack ident"),
            class: "ident",
        },
    ]
}

fn wire_rules() -> Vec<GrammarRule> {
    vec![
        GrammarRule {
            pattern: regex::Regex::new(r"->").expect("wire keyword"),
            class: "keyword",
        },
        GrammarRule {
            pattern: regex::Regex::new(r#"'[^']*'|"[^"]*""#).expect("wire string"),
            class: "string",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b\d+(?:\.\d+)?\b").expect("wire number"),
            class: "number",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"[:@{}.,\[\]-]").expect("wire operator"),
            class: "operator",
        },
        GrammarRule {
            pattern: regex::Regex::new(r"\b[A-Za-z_][A-Za-z0-9_.-]*\b").expect("wire ident"),
            class: "ident",
        },
    ]
}

/** @emoji 🎨 Tokenizes source text for a supported writer language id. */
pub fn tokenize_language(text: &str, language_id: &str) -> Vec<GrammarToken> {
    let rules = match language_id {
        "jack" => jack_rules(),
        "wire" => wire_rules(),
        _ => return Vec::new(),
    };
    let mut occupied = vec![false; text.len()];
    let mut tokens = Vec::new();
    for rule in rules {
        for capture in rule.pattern.find_iter(text) {
            let start = capture.start();
            let end = capture.end();
            if occupied[start..end].iter().any(|filled| *filled) {
                continue;
            }
            for slot in &mut occupied[start..end] {
                *slot = true;
            }
            tokens.push(GrammarToken {
                class: rule.class.into(),
                start,
                end,
            });
        }
    }
    tokens.sort_by_key(|token| (token.start, std::cmp::Reverse(token.end)));
    tokens
}
