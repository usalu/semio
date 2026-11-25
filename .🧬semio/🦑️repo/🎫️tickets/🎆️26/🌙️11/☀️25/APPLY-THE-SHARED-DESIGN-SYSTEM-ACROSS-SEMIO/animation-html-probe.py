"""🧪 Build the production Animate HTML and CSS emitters with the production HTML writer in isolation."""

from pathlib import Path
import re
import subprocess
import sys

ROOT = Path.cwd()
TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "animation"
PRESENTATION = ROOT / "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"
HTML = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any"


def between(source, start, after):
    """📎 Extract a complete production definition block between stable Rust declarations."""
    begin = source.index(start)
    return source[begin:source.index(after, begin)]


def main():
    """🎬 Compile the two emitters and serialize one authored bilingual deck label."""
    model = (HTML / "🧬️schema/📸️snapshot/🦀️.rs").read_text()
    types = between(model, "pub struct HtmlAttr", "//#endregion 🔖️Model")
    types = re.sub(r"^\s*#\[[^\n]*\n", "\n", types, flags=re.MULTILINE)
    constants = "\n".join(line for line in model.splitlines() if line.startswith(("pub const STDIO_HTML_DOCUMENT_SCHEMA", "pub(crate) const VOID_ELEMENTS")))
    codec = (HTML / "🚪️io/📝️text/📸️snapshot/🦀️.rs").read_text()
    helpers = "\n".join(between(codec, start, "\n}\n") + "\n}" for start in ("pub fn is_void_element", "pub(crate) fn encode_text", "pub(crate) fn encode_attr_value"))
    writer = between(codec, "pub fn write_html_document", "impl pack::snapshot::ArtifactDsl for HtmlSnapshot")
    emitters = between(PRESENTATION.read_text(), "    fn index_html_snapshot", "    fn player_boot_js")
    emitters = re.sub(r'include_str!\("([^"]+)"\)', lambda m: 'include_str!(r#"' + str((PRESENTATION.parent / m[1]).resolve()) + '"#)', emitters)
    code = f'''#![allow(dead_code)]
extern crate self as semio_s_artifact_stdio_html_snapshot;
pub mod standards {{ pub mod v5 {{ pub mod subsets {{ pub mod any {{ pub mod schema {{ pub mod snapshot {{
{constants}
{types}
{helpers}
{writer}
}} }} }} }} }} }}
{emitters}
fn main() {{
    use standards::v5::subsets::any::schema::snapshot::write_html_document;
    let html = index_html_snapshot(r#"{{"title":"A & B < C · Wärme"}}"#);
    std::fs::write("index.html", write_html_document(&html)).unwrap();
    std::fs::write("styles.css", styles_css()).unwrap();
}}
'''
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "html_probe.rs").write_text(code)
    subprocess.run([sys.argv[1], "--edition", "2021", "html_probe.rs", "-o", "html_probe"], cwd=OUT, check=True)
    subprocess.run([str(OUT / "html_probe")], cwd=OUT, check=True)


if __name__ == "__main__":
    main()
