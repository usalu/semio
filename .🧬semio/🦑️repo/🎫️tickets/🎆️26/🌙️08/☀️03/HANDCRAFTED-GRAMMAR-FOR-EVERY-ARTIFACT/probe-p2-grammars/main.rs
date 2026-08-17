use std::fs;
use std::path::PathBuf;

fn family_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family")
}

fn main() {
    let families = [
        ("graph", "🕸️graph/📖️family-graph.grammar.semio", "family-graph"),
        ("scene", "🎬️scene/📖️family-scene.grammar.semio", "family-scene"),
        ("sheet", "📊️sheet/📖️family-sheet.grammar.semio", "family-sheet"),
        ("catalog", "🗂️catalog/📖️family-catalog.grammar.semio", "family-catalog"),
        ("recipe", "🧑‍🍳️recipe/📖️family-recipe.grammar.semio", "family-recipe"),
        ("geo", "🌍️geo/📖️family-geo.grammar.semio", "family-geo"),
        ("embed", "📎️embed/📖️family-embed.grammar.semio", "family-embed"),
    ];
    let mut failed = false;
    for (label, rel, id) in families {
        let path = family_dir().join(rel);
        let source = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {label}: {e}"));
        match probe_p2_grammars::os_dsl::grammar::parse_grammar(&source) {
            Ok(grammar) => {
                if grammar.id != id {
                    eprintln!("id mismatch {label}: expected {id}, got {}", grammar.id);
                    failed = true;
                } else {
                    println!("ok {label} id={} productions={}", grammar.id, grammar.productions.len());
                }
            }
            Err(e) => {
                eprintln!("parse failed {label}: {}", e.message);
                failed = true;
            }
        }
    }
    if failed {
        std::process::exit(1);
    }
}
