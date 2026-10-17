"""🔗 C4P5 consumer re-points of the Rust source-text readers: the editor root is now a split root, so the test targets that read `../../🦀️.rs` read the root plus its mounted children through one helper.
Stages (never installs) the edited copies under the stage dir and records the sha256 of every base text, so the installer can refuse when a peer changed the file meanwhile.
usage: consumers.py"""
import hashlib, json, os, re

REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
STAGE = F + "/🗑️generated/c4p5-stage"
E = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
READ = '&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).expect("committed test data reads")'
HELPER = '''
/// 📚️ The editor lib root and every module mounted below it, read as the one text the unsplit root held: the children first, the root last.
pub fn editor_source() -> String {
    let editor = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    ["🧰️retained-commands", "📬️store-preparation", "✂️clipboard-job", "🧩️play-app", "📋️manifest"]
        .iter()
        .map(|child| format!("{editor}/{child}/🦀️.rs"))
        .chain([format!("{editor}/🦀️.rs")])
        .map(|path| std::fs::read_to_string(path).expect("committed test data reads"))
        .collect::<Vec<_>>()
        .join("\\n")
}
'''
out = {}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def stage(rel, new_text, base_bytes):
    p = os.path.join(STAGE, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(new_text)
    out[rel] = dict(base_sha256=sha(base_bytes))


ctx = f"{E}/🧪️tests/🧰️context/🦀️.rs"
raw = open(f"{REPO}/{ctx}", "rb").read()
text = raw.decode("utf-8")
assert "fn editor_source" not in text
assert text.endswith("}\n")
stage(ctx, text + HELPER, raw)

unit = f"{E}/🧪️tests/🔬️unit/🦀️.rs"
raw = open(f"{REPO}/{unit}", "rb").read()
text = raw.decode("utf-8")
n = text.count(READ)
assert n == 11, n
stage(unit, text.replace(READ, "&editor_source()"), raw)

wiring = f"{E}/🧪️tests/🧵️retained-wiring/🦀️.rs"
raw = open(f"{REPO}/{wiring}", "rb").read()
text = raw.decode("utf-8")
assert text.count(READ) == 1
stage(wiring, text.replace(READ, "&crate::context::editor_source()"), raw)

json.dump(out, open(F + "/🗑️generated/c4p5-consumers.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
print("staged", len(out), "consumer files")
