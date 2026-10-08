use std::fs;
use std::path::{Path, PathBuf};

/// 📥️ Records actual build-producer filesystem inputs independently of rustc dep-info.
#[derive(Default)]
pub struct ResourceObservationsV1 {
    rows: Vec<String>,
}

fn quoted(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            character if character < '\u{20}' => output.push_str(&format!("\\u{:04x}", character as u32)),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn path_text(path: &Path) -> String {
    quoted(path.to_str().expect("UTF-8 build resource path"))
}

impl ResourceObservationsV1 {
    pub fn directory(&mut self, path: &Path, entries: &[PathBuf]) {
        let mut entries = entries.iter().map(|path| path_text(path)).collect::<Vec<_>>();
        entries.sort();
        self.rows.push(format!("{{\"kind\":\"directory\",\"path\":{},\"entries\":[{}]}}", path_text(path), entries.join(",")));
    }

    pub fn copied(&mut self, source: &Path, output: &Path) {
        self.rows.push(format!("{{\"kind\":\"copy\",\"path\":{},\"output\":{}}}", path_text(source), path_text(output)));
    }

    pub fn read(&mut self, source: &Path, bytes: &[u8], out_dir: &Path) {
        let output = out_dir.join(format!("semio-runtime-resource-read-{}.bin", self.rows.len()));
        fs::write(&output, bytes).expect("record actual build resource read");
        self.rows.push(format!("{{\"kind\":\"read\",\"path\":{},\"output\":{}}}", path_text(source), path_text(&output)));
    }

    pub fn finish(&self, out_dir: &Path) {
        fs::write(out_dir.join("semio-runtime-resource-inputs.jsonl"), self.rows.join("\n") + "\n").expect("record build resource observations");
    }
}
