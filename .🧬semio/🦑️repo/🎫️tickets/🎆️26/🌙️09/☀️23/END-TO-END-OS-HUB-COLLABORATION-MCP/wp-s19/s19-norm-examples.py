#!/usr/bin/env python3
"""🎨️ S19 one-off codemod (set `norm-examples`): every norm family whose `setActiveExample` kept a hand-written id match
(en1992, en1994, en1997, en1998, iso16757, din4108; session 14c: din16798 — its match missed `residential-method3`, the
roster law's red — and the same hand-kept tables of en1991, en1993, en1996, en1999, vdi3805 and en1995's private parse)
resolves the example through its editor's OWN picker roster via the shared `app_surface::roster_example_snapshot`; en1997
and din4108 rosters gain the example modules they had on disk but hid from the picker; the norm surface suite gains the
roster law (every roster example loads, roster == `📚️examples/*`). en1990 and din18599 keep their text-based
`setSnapshot` route (complete tables, law-guarded).
Idempotent. usage: s19-norm-examples.py <root>
"""
import os
import re
import sys

root = sys.argv[1]
A = "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
H = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
E = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
FAMILIES = {"🏛️en1992": ("en1992", "En1992"), "🧩️en1994": ("en1994", "En1994"), "🌍️en1997": ("en1997", "En1997"), "🫨️en1998": ("en1998", "En1998"), "📇️iso16757": ("iso16757", "Iso16757"), "🧱️din4108": ("din4108", "Din4108"), "🌬️din16798": ("din16798", "Din16798"), "🏋️en1991": ("en1991", "En1991"), "🔩️en1993": ("en1993", "En1993"), "🪨️en1996": ("en1996", "En1996"), "🪶️en1999": ("en1999", "En1999"), "🏭️vdi3805": ("vdi3805", "Vdi3805"), "🪵️en1995": ("en1995", "En1995")}
ROSTER_DOC = "/// 🎨️ Replaces the live document with the named roster example, or clears it when the id is empty.\n"


def edit(rel, change):
    path = os.path.join(root, rel)
    before = open(path, encoding="utf-8").read()
    after = change(before)
    if after != before:
        open(path, "w", encoding="utf-8").write(after)
        print(f"edited {rel}")


HELPER = '''//#endregion 🔖️ValuePath

//#region 🔖️Examples
/// 🎨️ Resolves `setActiveExample` through the editor's OWN example roster — the list its navbar picker
/// offers — so every example the picker shows loads, and nothing outside it does. An empty id is the
/// empty document, an id outside the roster is `None` (the no-op a picker's stale selection needs),
/// and an example body that does not parse is a named fault, never a silent fallback.
pub fn roster_example_snapshot<D: store::ArtifactDsl + Default>(examples: Vec<ExampleSource>, example_id: &str) -> Result<Option<D>, Fault> {
    let id = example_id.trim();
    if id.is_empty() {
        return Ok(Some(D::default()));
    }
    let Some(example) = examples.into_iter().find(|example| example.id() == id) else {
        return Ok(None);
    };
    D::parse_dsl(&example.document())
        .map(Some)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' does not parse: {error:?}")))
}
//#endregion 🔖️Examples

//#region 🔖️Render
'''


def helper(text):
    if "fn roster_example_snapshot" in text:
        return text
    anchor = "//#endregion 🔖️ValuePath\n\n//#region 🔖️Render\n"
    assert text.count(anchor) == 1, "app-surface anchor"
    return text.replace(anchor, HELPER)


edit("✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs", helper)


def handler(module, ty):
    def change(text):
        if "roster_example_snapshot" in text:
            return text
        start = text.index("pub fn handle(")
        doc_start = text.rfind("\n", 0, start - 1) + 1
        has_doc = text[doc_start:start].startswith("///")
        end = text.index("\n}\n", start) + 3
        body = f'''pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, {ty}Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<{ty}Mutation, NoConfigMutation>, Fault> {{
    let Some(snapshot) = crate::app_surface::roster_example_snapshot(<crate::editor::{module}::{ty}PlayApp as ArtifactEditor>::examples(), &payload.example_id)? else {{
        return Ok(Emit::default());
    }};
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot {{ snapshot }}, doc, cfg)
}}
'''
        if has_doc:
            start = doc_start
        body = ROSTER_DOC + body
        text = text[:start] + body + text[end:]
        text = re.sub(r"fn example_primary_text\(id: &str\) -> Option<&'static str> \{\n.*?\n}\n\n", "", text, flags=re.S)
        text = text.replace("use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};", "use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};")
        text = text.replace("use crate::standards::v1::subsets::any::schema::snapshot::{decode_en1997_dsl, compliant_demo, noncompliant_demo};\n", "")
        text = re.sub(r"\nfn from_primary\(.*?\n}\n\n", "\n", text, flags=re.S)
        return text
    return change


for folder, (module, ty) in FAMILIES.items():
    edit(f"{A}/{folder}/{H}", handler(module, ty))


def roster(old, new):
    def change(text):
        if new in text:
            return text
        assert text.count(old) == 1, old
        return text.replace(old, new)
    return change


edit(f"{A}/🌍️en1997/{E}", roster("vec![crate::examples::demo::source()]", "vec![crate::examples::demo::source(), crate::examples::compliant::source(), crate::examples::noncompliant::source()]"))
edit(f"{A}/🧱️din4108/{E}", roster("vec![crate::examples::demo::source()]", "vec![crate::examples::demo::source(), crate::examples::failing_thin_insulation::source()]"))

LAW = '''
//#region 🗂️RosterExamples
/// 🗂️ Asserts every example one family editor offers in its navbar picker loads through `setActiveExample`,
/// and that the picker roster is exactly the family's `📚️examples/*` example modules — an example on disk
/// the picker hides, or a picker entry `setActiveExample` cannot load, is the drift this law exists for
/// (en1992 offered five examples and loaded two).
macro_rules! roster_examples_case {
    ($name:ident, $editor:ty, $set_active:path, $snapshot:ty, $family_dir:literal) => {
        #[test]
        fn $name() {
            use semio_framework_plugin::ArtifactEditor;
            use $set_active as set_active_example;
            let roster = <$editor as ArtifactEditor>::examples();
            let boot = <$snapshot>::default();
            let config = semio_framework_plugin::NoConfig::default();
            for example in &roster {
                let emit = set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: example.id().to_string() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .unwrap_or_else(|fault| panic!("{}: example '{}' must load: {fault:?}", stringify!($name), example.id()));
                assert!(
                    !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                    "{}: example '{}' loaded nothing",
                    stringify!($name),
                    example.id()
                );
            }
            let mut rostered: Vec<String> = roster.iter().map(|example| example.id().to_string()).collect();
            rostered.sort();
            let mut on_disk = example_ids_on_disk($family_dir);
            on_disk.sort();
            assert_eq!(rostered, on_disk, "{}: the picker roster must be exactly the family's 📚️examples/* modules", stringify!($name));
        }
    };
}

/// 📚️ The `pub const ID` of every `📚️examples/*/🦀️.rs` example module of one norm family.
fn example_ids_on_disk(family_dir: &str) -> Vec<String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts").join(family_dir).join("🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("{}: {error}", root.display()))
        .filter_map(|entry| std::fs::read_to_string(entry.ok()?.path().join("🦀️.rs")).ok())
        .filter_map(|source| source.lines().find_map(|line| line.trim().strip_prefix("pub const ID: &str = \\"")?.strip_suffix("\\";").map(str::to_string)))
        .collect()
}

'''


def law(text):
    if "roster_examples_case!" in text:
        return text
    families = {d.encode("ascii", "ignore").decode(): d for d in os.listdir(os.path.join(root, A)) if d != "🦀️.rs"}
    cases = re.findall(r"set_active_example_case!\(\s*(\w+),\s*([\w:]+),\s*([\w:]+),\s*([\w:]+),\s*([\w:]+),\s*([\w:]+),\s*([\w:]+)\s*\);", text)
    assert len(cases) == 15, len(cases)
    rows = []
    for name, create, _command, set_active, _id, _primary, snapshot in cases:
        family = name.split("_")[0]
        editor = create.rsplit("::", 1)[0] + "::" + snapshot.rsplit("::", 1)[1].replace("Snapshot", "PlayApp")
        rows.append(f'roster_examples_case!({family}_roster_examples_all_load, {editor}, {set_active}, {snapshot}, "{families[family]}");')
    return text.rstrip("\n") + "\n" + LAW + "\n".join(rows) + "\n//#endregion 🗂️RosterExamples\n"


edit("✏️s/🔌️plugins/📕️norm/🧪️tests/🔬️surface/🦀️.rs", law)

EN1998_LAW = '''
/// 🖼️ Every en1998 example asset (what the picker, the descriptor and `setActiveExample` load) decodes to exactly its
/// code-built snapshot (what the compliance gate evaluates) — the August flat-format assets stopped parsing when the
/// snapshot moved to `layout = "lines"`, and the old hand-kept `setActiveExample` arms hid it by loading the constructors.
#[semio_framework_async_macros::async_test]
async fn every_example_asset_decodes_to_its_code_built_snapshot() {
    for (id, text, snapshot) in [
        (crate::seismic_rc_frame::ID, crate::seismic_rc_frame::PRIMARY_TEXT, crate::seismic_rc_frame::snapshot()),
        (crate::seismic_rc_frame_fail::ID, crate::seismic_rc_frame_fail::PRIMARY_TEXT, crate::seismic_rc_frame_fail::snapshot()),
        (crate::seismic_multipart::ID, crate::seismic_multipart::PRIMARY_TEXT, crate::seismic_multipart::snapshot()),
        (crate::seismic_multipart_fail::ID, crate::seismic_multipart_fail::PRIMARY_TEXT, crate::seismic_multipart_fail::snapshot()),
    ] {
        let decoded = crate::standards::v1::subsets::any::schema::snapshot::decode_en1998_dsl(text).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert!(decoded == snapshot, "{id}: the committed asset must be the code-built snapshot");
    }
}
'''


def en1998_law(text):
    if "fn every_example_asset_decodes_to_its_code_built_snapshot" in text:
        return text
    return text.rstrip("\n") + "\n" + EN1998_LAW


edit(f"{A}/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🦀️.rs", en1998_law)
