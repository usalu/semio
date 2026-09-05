//! 🏭️ Writes independently-authored sequence JSON carrier fixture pairs.

use std::fs;
use std::path::PathBuf;

use sequence_json::{apply, build_seed, project, KINDS};

fn destination(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "change-step-collapsed" => ("🪜️step", "🗂️change-step-collapsed"),
        "connect-steps" => ("🔗️dependency", "🔗️connect-steps"),
        "disconnect-steps" => ("🔗️dependency", "✂️disconnect-steps"),
        "move-step" => ("🪜️step", "📍️move-step"),
        _ => unreachable!("unknown JSON fixture kind"),
    }
}

fn main() {
    let out_root = std::env::args().nth(1).unwrap_or_else(|| "../../../".into());
    let seed = build_seed();
    for kind in KINDS {
        let before_bytes = format!("{}\n", serde_json::to_string_pretty(&seed).expect("seed serializes"));
        let after = apply(kind, &seed).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let after_bytes = format!("{}\n", serde_json::to_string_pretty(&after).expect("mutation serializes"));
        assert_ne!(project(before_bytes.as_bytes()).expect("before projects"), project(after_bytes.as_bytes()).expect("after projects"), "{kind}: every pair must be observable");
        let (subset, directory) = destination(kind);
        let dir = PathBuf::from(&out_root).join(subset).join("🧫️fixtures").join(directory);
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("⬅️before.json"), before_bytes).expect("write before.json");
        fs::write(dir.join("➡️after.json"), after_bytes).expect("write after.json");
    }
    println!("[generate] wrote {} sequence JSON carrier pair(s)", KINDS.len());
}
