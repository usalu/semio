import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
B = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model"
ANY = B + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
PREFIXES = ["✏️s/🔌️plugins/🏙️bim", "🌎️hub/🧩️compositions/🏙️bim"]
def git(*args):
    return subprocess.run(["git", "-c", "core.quotepath=false", *args], capture_output=True, check=True).stdout
def restore():
    names = [n for n in git("diff", "--name-only", "-z", "HEAD", "--", *PREFIXES).decode("utf8").split("\0") if n]
    for n in names:
        open(n, "wb").write(git("show", "HEAD:" + n))
    print("restored", len(names))
def pre():
    subprocess.run([sys.executable, os.path.join(HERE, "macro_imports.py"), "--apply"], check=True, stdout=subprocess.DEVNULL)
    by = json.load(open(os.path.join(HERE, "sqlite-by.json"), encoding="utf8"))
    vals = sorted(by["🧬️schema/📸️snapshot/💠️values/🦀️.rs"])
    snap = sorted(by["🧬️schema/📸️snapshot/🧱️entities/🦀️.rs"] + by["🧬️schema/📸️snapshot/🦴️structure/🦀️.rs"] + by["🧬️schema/📸️snapshot/🦀️.rs"])
    f = ANY + "/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"
    s = open(f, encoding="utf8").read()
    if "use crate::*;\n" in s:
        s = s.replace("use crate::*;\n", "use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{%s};\nuse semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{%s};\n" % (",".join(vals), ",".join(snap)))
        open(f, "w", encoding="utf8").write(s)
    for rel in ["/✏️editor/🧵️gestures/🧪️tests/🔬️unit/🦀️.rs", "/✏️editor/🎮️commands/🔗️attach-walls/🧪️tests/🔬️unit/🦀️.rs", "/✏️editor/🎮️commands/🔑️gesture-keys/🧪️tests/🔬️unit/🦀️.rs"]:
        path = ANY + rel
        text = open(path, encoding="utf8").read()
        open(path, "w", encoding="utf8").write(text.replace("crate::editor::bim::BIM_TOOL_IDS", "semio_s_artifact_bim_model_bim::editor::bim::BIM_TOOL_IDS"))
    cr = ANY + "/🧬️schema/🧬️mutations/🔙️cascade-restore/🦀️.rs"
    text = open(cr, encoding="utf8").read()
    macro_path = "crate::standards::v1::subsets::any::schema::mutations::$module::$variant"
    if macro_path in text:
        modules = sorted(set(re.findall(r"\((create_[a-z_]+),", text)))
        text = text.replace(macro_path, "$module::$variant")
        marker = "macro_rules! restore {"
        text = text.replace(marker, "use crate::standards::v1::subsets::any::schema::mutations::{%s};\n\n%s" % (", ".join(modules), marker), 1)
        open(cr, "w", encoding="utf8").write(text)
    g = ANY + "/🧪️tests/🏗️export-bim-1-ifc/🦀️.rs"
    s = open(g, encoding="utf8").read()
    a = "        use semio_s_artifact_bim_model::standards::v1::subsets::any::io::export::ifc::ifc4::{library_report, library_to_part21};\n"
    if a in s:
        s = s.replace(a, "        use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::Schema;\n        use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::{ifc4::library_report, model_to_part21};\n").replace("library_report(&library_to_part21(&snapshot).0)", "library_report(&model_to_part21(Schema::Ifc4, &snapshot)?.0)")
        open(g, "w", encoding="utf8").write(s)
def post():
    open(B + "/🦀️.rs", "w", encoding="utf8").write('''//! 🏢️ BIM model artifact: building information as a parametric, event-sourced document.
//!
//! `ModelSnapshot` stores authored parameters only; every derived value (elevations, wall heights, quantities, solids) is a field of
//! `ModelInference`. Mutations are sparse typed diffs with concrete inverses (`ModelMutation`, `ModelDiff`). The artifact is carved into
//! component crates; this crate only exports the artifact identity and the declaration tree, and every consumer imports the crate that owns what it uses.

pub use semio_s_artifact_bim_model_v1::{BimApplication, artifact, definition};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
    p = B + "/🧪️tests/🔬️unit/🦀️.rs"
    t = open(p, encoding="utf8").read()
    if t.startswith("use super::*;\n"):
        body = t.replace("use super::*;\n", "", 1)
        body = body.replace("semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Phase::", "Phase::")
        v = [n for n in ["OpeningKind", "Phase", "Point2", "Profile", "PropertySet", "PropertyValue", "RoofShape", "Slope", "SpaceBoundary", "StairFlight", "TopConstraint", "Turn", "Vertex"] if re.search(rf"(?<![\w:]){n}\b", body)]
        e = [n for n in ["Column", "ColumnType", "ModelSnapshot", "Opening", "Roof", "Slab", "Space", "Stair"] if re.search(rf"(?<![\w:]){n}\b", body)]
        head = "use super::definition;\nuse semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::{artifact_kind, snapshot::values::{%s}, BIM_MODEL_DOCUMENT_SCHEMA};\nuse semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{%s};\n" % (", ".join(v), ", ".join(e))
        body = body.replace("semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::binary::snapshot::", "pack::").replace("semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::", "text::")
        if "as pack;" not in body: head += "use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::{binary::snapshot as pack, text::snapshot as text};\n"
        open(p, "w", encoding="utf8").write(head + body)
    p = "🌎️hub/🧩️compositions/🏙️bim/📦️packages/🦀️rust/🦀️.rs"
    s = open(p, encoding="utf8").read()
    a = "//#region ✏️Editor\n\n//#endregion ✏️Editor\n\n//#region 👁️Viewer\n\n//#endregion 👁️Viewer\n\n"
    if a in s: open(p, "w", encoding="utf8").write(s.replace(a, ""))
    p = "🌎️hub/🧩️compositions/🏙️bim/🧪️tests/🔬️surface/🦀️.rs"
    s = open(p, encoding="utf8").read()
    a = "crate::artifacts::model::BIM_MODEL_DOCUMENT_SCHEMA"
    if a in s: open(p, "w", encoding="utf8").write(s.replace(a, "semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DOCUMENT_SCHEMA"))
    print("post-edited")
if __name__ == "__main__":
    {"restore": restore, "pre": pre, "post": post}[sys.argv[1]]()
