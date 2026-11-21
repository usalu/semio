"""🖼️ Stage the PDF 1.7 sampled image mutation producer without copying repository source."""
from pathlib import Path
import argparse
import difflib
import json
import os
import subprocess

TICKET = Path(__file__).resolve().parent
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
DOCUMENT = PDF / "✏️editor/📄️document"
MODULE = DOCUMENT / "🖼️image-authoring/🦀️.rs"
OUTPUT = TICKET / "🗑️generated" / "image-authoring"

SOURCE = r'''//! 🖼️ Places admitted local raster samples as ordinary PDF image XObjects.

use super::*;

/// 📍 Addresses an insertion or one existing image placement in PDF page space.
#[derive(Clone, Copy)]
pub enum ImagePlacement {
    Insert { page: usize, matrix: PdfMatrix },
    Replace { page: usize, operator: usize },
}

/// 🌈 Straight RGBA8 pixels in image row order.
pub struct SampledRgba<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [u8],
}

/// 📏 Caps the sampled representation at one bounded component array per image.
pub const MAX_IMAGE_IMPORT_PIXELS: usize = 4_096;

/// 🔍 Validates a page address before reading a picked file or applying its samples.
pub fn check_image_placement(snapshot: &PdfSnapshot, placement: ImagePlacement) -> Result<usize, Fault> {
    let (page, replacement) = match placement {
        ImagePlacement::Insert { page, matrix } => {
            if !matrix.iter().all(|value| value.is_finite()) || matrix[0] <= 0.0 || matrix[3] <= 0.0 {
                return Err(fault("image placement requires finite coordinates and positive dimensions"));
            }
            (page, None)
        }
        ImagePlacement::Replace { page, operator } => (page, Some(operator)),
    };
    let target = snapshot.pages.get(page).ok_or_else(|| fault("PDF image page no longer exists"))?;
    if let Some(operator) = replacement {
        match target.content.get(operator) {
            Some(PdfOp::InlineImage { .. }) => {}
            Some(PdfOp::PaintXObject { name }) if snapshot.image(name).is_some() => {}
            _ => return Err(fault("PDF image placement no longer exists")),
        }
    }
    Ok(page)
}

/// 🖼️ Installs a fresh image, its optional soft mask and the selected paint in one edit.
pub fn place_sampled_image(snapshot: &PdfSnapshot, placement: ImagePlacement, raster: SampledRgba<'_>) -> Result<Vec<PdfMutation>, Fault> {
    let page = check_image_placement(snapshot, placement)?;
    let pixels = (raster.width as usize).checked_mul(raster.height as usize)
        .filter(|pixels| (1..=MAX_IMAGE_IMPORT_PIXELS).contains(pixels))
        .filter(|pixels| raster.pixels.len() == pixels * 4)
        .ok_or_else(|| fault("image pixel dimensions or sample counts exceed the import limit"))?;
    let mut rgb = Vec::with_capacity(pixels * 3);
    let mut alpha = Vec::with_capacity(pixels);
    for sample in raster.pixels.chunks_exact(4) {
        rgb.extend_from_slice(&sample[..3]);
        alpha.push(sample[3]);
    }
    let taken: std::collections::HashSet<_> = snapshot.images.iter().map(|image| image.id.as_str())
        .chain(snapshot.forms.iter().map(|form| form.id.as_str())).collect();
    let mut names = (1..).map(|index| format!("SemioImage{index}")).filter(|name| !taken.contains(name.as_str()));
    let id = names.next().expect("available image XObject id");
    let mut image = PdfImage::rgb8(&id, raster.width, raster.height, rgb);
    let mut changes = Vec::with_capacity(3);
    if alpha.iter().any(|sample| *sample != 255) {
        let mask = names.next().expect("available soft mask XObject id");
        image.soft_mask = Some(mask.clone());
        changes.push(PdfMutation::SetImage(SetImage { image: PdfImage::gray8(mask, raster.width, raster.height, alpha), index: None }));
    }
    changes.push(PdfMutation::SetImage(SetImage { image, index: None }));
    let paint = PdfOp::PaintXObject { name: id };
    match placement {
        ImagePlacement::Insert { matrix, .. } => changes.push(PdfMutation::AppendPageContent(AppendPageContent {
            index: page,
            content: vec![PdfOp::Save, PdfOp::Transform { matrix }, paint, PdfOp::Restore],
        })),
        ImagePlacement::Replace { operator, .. } => changes.push(PdfMutation::ReplaceContent(ReplaceContent { index: page, at: operator, op: paint })),
    }
    Ok(changes)
}
'''

WIRING = '''
#[path = "🖼️image-authoring/🦀️.rs"]
mod image_authoring;
pub use self::image_authoring::*;
'''


def single(source, before, after):
    """📍 Replace an exact source anchor after ruling out concurrent layout drift."""
    assert source.count(before) == 1, before
    return source.replace(before, after)


def picker_changes():
    """📂 Stage the existing shell picker callback and the controller's retained decoder."""
    editor = PDF / "✏️editor"
    page = editor / "🖼️page"
    picker = editor / "📥️local-image/🦀️.rs"
    assert not picker.exists()
    shell_module = (TICKET / "image-picker.rs").read_text(encoding="utf-8")
    parent = editor / "🦀️.rs"
    source = parent.read_text(encoding="utf-8")
    staged = single(source, "//#region 🔖️Dialect", '#[path = "📥️local-image/🦀️.rs"]\nmod image_import;\n\n//#region 🔖️Dialect')
    staged = single(staged, "    PageEdit { action: String, payload: String },", """    PageEdit { action: String, payload: String },
    #[dsl(key = "pick-image")]
    PickImage { page: u32, operator: u32, x: f64, y: f64, width: f64, height: f64, guard: String },
    #[dsl(key = "import-local-image")]
    ImportImage { page: u32, operator: u32, revision: String, x: f64, y: f64, width: f64, height: f64, payload: String },""")
    old = '"insert-image", "insert-page"'
    assert staged.count(old) == 2
    staged = staged.replace(old, '"insert-image", "replace-image", "import-local-image", "insert-page"', 1)
    staged = single(staged, "        bounded_native: true\n", """        bounded_native: true,
        native: {
            factory_name: "PdfLocalImageImport",
            factory_type: image_import::Factory,
            contract: image_import::CONTRACT,
            build: image_import::build,
            tools: ["import-local-image"]
        }
""")
    staged = single(staged, "            Pdf17EditorCommand::PageEdit { action, .. } => crate::editor::page::static_action(action),", """            Pdf17EditorCommand::PageEdit { action, .. } => crate::editor::page::static_action(action),
            _ => image_import::command_id(native).expect("PDF image editor command id"),""")
    staged = single(staged, "            other if crate::editor::page::is_page_action(other) => {", """            other if image_import::admits(other) => image_import::from_action(other, args),
            other if crate::editor::page::is_page_action(other) => {""")
    staged = single(staged, "            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),", """            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17EditorCommand::PickImage { .. } | Pdf17EditorCommand::ImportImage { .. }) =>
                Err(Fault::from("PNG import requires its retained interactive job")),
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),""")
    marker = '    tools: ["set-page"'
    start = staged.index(marker)
    stop = staged.index("\n", start)
    roster = staged[start:stop]
    staged = staged[:start] + single(roster, '"insert-image", ', "") + staged[stop:]
    staged = single(staged, "            _ => Err(Fault::from(\"stdio-pdf-native-edit-command-mismatch\")),\n        }\n    },\n}", """            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),
        }
    },
    host_tools: ["insert-image", "replace-image"],
    work: image_import::native_work,
}""")
    staged = single(staged, "    crate::editor::page::install_object_interaction(&mut definition);", """    for window in definition.window_kinds.iter_mut() {
        if window.id == main::WINDOW_KIND_ID { window.actions.retain(|action| action.id != "insert-image"); }
    }
    definition.actions.extend(image_import::actions());
    crate::editor::page::install_object_interaction(&mut definition);""")

    window = page / "🪟️windows/🦀️.rs"
    window_before = window.read_text(encoding="utf-8")
    needle = "    for field in inspector_fields(snapshot, &object) {"
    assert window_before.count(needle) == 1
    window_after = single(window_before, needle, """    if object.kind == ObjectKind::Image && controller_id == "s.stdio.pdf@1.7/*#editor" {
        let button = image_replacement_button(controller_id, object.page, object.end, publication_revision.0, locale == Locale::De)?;
        column = column.try_child(button).map_err(|_| inspector_error("pdf.object.image-action"))?;
    }
""" + needle)
    widget = DOCUMENT / "🖼️image-authoring/🎛️replace-button.rs"
    assert not widget.exists()
    code = [
        (DOCUMENT / "🖼️image-authoring/🧵️decode.rs", "image-decode.rs"),
        (DOCUMENT / "🖼️image-authoring/📥️admission.rs", "image-pick-admission.rs"),
    ]
    for file, _ in code:
        assert not file.exists(), file
    return [(parent, source, staged), (picker, "", shell_module), (window, window_before, window_after), (widget, "", BUTTON)] + [
        (file, "", (TICKET / authored).read_text(encoding="utf-8")) for file, authored in code
    ] + dependency_changes()


BUTTON = r'''//! 🎛️ Revision-pinned replacement control for a selected PDF image.

use semio_plugin::{
    ActionId, Buildable, BuiltNode, HasBase, PluginAssemblyError, UiAssemblyResult, UiMapBuilder, UiText, UiValue,
    plugin_app_close_prelude as ui,
};

/// 📂 Builds a localized picker action for one selected image placement.
pub fn image_replacement_button(controller: &str, page: usize, at: usize, revision: u64, german: bool) -> UiAssemblyResult<BuiltNode> {
    let fail = || PluginAssemblyError::new("pdf.object.image-action", "PDF image picker control admission failed");
    let guard = UiText::try_from_str(&format!("{revision:016x}")).ok_or_else(fail)?;
    let mut args = UiMapBuilder::try_new().ok_or_else(fail)?;
    for (key, value) in [("page", UiValue::Number(page as f64)), ("operator", UiValue::Number(at as f64)), ("text", UiValue::Text(guard))] {
        args.try_insert(key.into(), value).map_err(|_| fail())?;
    }
    let action = ActionId::try_v1(controller, "replace-image").ok_or_else(fail)?;
    let label = if german { "Bild durch PNG ersetzen" } else { "Replace Image with PNG" };
    ui::button(ui::Label(UiText::clipped(label))).try_id("pdf-replace-image").map_err(|_| fail())?
        .try_on_with(ui::Trigger::Activate, action, UiValue::Map(args.finish())).map_err(|_| fail())?
        .try_build().map_err(|_| fail())
}
'''


BUTTON_WIRING = r'''
#[path = "🎛️replace-button.rs"]
mod replace_button;
pub use replace_button::image_replacement_button;

#[path = "🧵️decode.rs"]
mod decode;
pub use decode::*;

#[path = "📥️admission.rs"]
mod admission;
pub use admission::*;
'''


LOCK_ADDITIONS = {
    "semio-s-artifact-stdio-pdf-editor-document": ("semio-framework-pixels", "semio-s-artifact-stdio-contract"),
    "semio-s-artifact-stdio-pdf-schema-5": ("semio-s-artifact-stdio-pdf-editor-document",),
}


def read_stage_source(file):
    """📖 Read lock newline sequences literally so unchanged rows remain byte-identical."""
    if file.name == "Cargo.lock":
        with file.open(encoding="utf-8", newline="") as source:
            return source.read()
    return file.read_text(encoding="utf-8")


def lock_sections(locked):
    """📑 Address complete package records without reserializing the lockfile."""
    records = locked.split("[[package]]")
    packages = {}
    for index, record in enumerate(records):
        lines = record.splitlines()
        if len(lines) > 1 and lines[1].startswith('name = "') and lines[1].endswith('"'):
            name = lines[1][8:-1]
            packages.setdefault(name, []).append(index)
    return records, packages


def update_lock(locked):
    """🔗 Insert the staged normal dependency edges into existing local PDF package rows."""
    records, packages = lock_sections(locked)
    if "semio-s-artifact-stdio-pdf-editor-document" not in packages:
        return locked
    for owner, add in LOCK_ADDITIONS.items():
        indexes = packages.get(owner, [])
        if not indexes:
            continue
        assert len(indexes) == 1, owner
        at, = indexes
        block = records[at]
        newline = "\r\n" if block.startswith("\r\n") else "\n"
        start = block.index("dependencies = [" + newline) + len("dependencies = [" + newline)
        end = block.index("]" + newline, start)
        entries = block[start:end].splitlines(keepends=True)
        for dependency in add:
            assert len(packages.get(dependency, [])) == 1, (owner, dependency)
            entry = f' "{dependency}",' + newline
            assert entries.count(entry) <= 1, (owner, dependency)
            if entry not in entries:
                ordinal = next((i for i, current in enumerate(entries) if current > entry), len(entries))
                entries.insert(ordinal, entry)
        records[at] = block[:start] + "".join(entries) + block[end:]
    return "[[package]]".join(records)


def matching_locks():
    """📚 Discover tracked locks that already resolve the PDF document editor."""
    query = ["repo", "tools", "exec", "--light", "git", "--", "ls-files", "-z", "--", "Cargo.lock", ":(glob)**/Cargo.lock"]
    files = [Path(os.fsdecode(path)) for path in subprocess.check_output(query).split(b"\0") if path]
    matches = []
    for file in sorted(set(files)):
        if not file.is_file():
            continue
        locked = read_stage_source(file)
        _, packages = lock_sections(locked)
        if "semio-s-artifact-stdio-pdf-editor-document" in packages:
            updated = update_lock(locked)
            assert update_lock(updated) == updated, file
            matches.append((file, locked, updated))
    return matches


def ensure_normal_dependency(manifest, dependency, definition):
    """📦 Add a normal Cargo edge only when it is absent."""
    header = "[dependencies]\n"
    start = manifest.index(header) + len(header)
    next_section = manifest.find("\n[", start)
    end = len(manifest) if next_section == -1 else next_section
    rows = manifest[start:end].splitlines()
    existing = [row for row in rows if row.split("=", 1)[0].strip() == dependency]
    if existing:
        assert existing == [definition], dependency
        return manifest
    return manifest[:start] + definition + "\n" + manifest[start:]


def dependency_changes():
    """📦 Stage existing internal crate edges in their manifests and matching checked-in locks."""
    package = Path("📦️packages/🦀️rust")
    document = DOCUMENT / package
    controller = PDF.parent / "🧾️vt/🧬️schema" / package
    pixels = Path("🧰️framework/🔨️modules/🔲️pixels") / package
    relative = lambda target, owner: Path(os.path.relpath(target, owner)).as_posix()
    doc_toml = document / "Cargo.toml"
    ctrl_toml = controller / "Cargo.toml"
    doc_before = doc_toml.read_text(encoding="utf-8")
    ctrl_before = ctrl_toml.read_text(encoding="utf-8")
    doc_after = ensure_normal_dependency(doc_before, "semio-framework-pixels", f'semio-framework-pixels = {{ path = "{relative(pixels, document)}" }}')
    doc_after = ensure_normal_dependency(doc_after, "semio-s-artifact-stdio-contract", "semio-s-artifact-stdio-contract = { workspace = true }")
    ctrl_after = ensure_normal_dependency(ctrl_before, "semio-s-artifact-stdio-pdf-editor-document", f'semio-s-artifact-stdio-pdf-editor-document = {{ path = "{relative(document, controller)}" }}')
    return [(doc_toml, doc_before, doc_after), (ctrl_toml, ctrl_before, ctrl_after), *matching_locks()]


def stage(apply=False, picker=False):
    """🧷 Produce a reproducible patch against the current source or apply it after review."""
    parent = DOCUMENT / "🦀️.rs"
    before = parent.read_text(encoding="utf-8")
    assert not MODULE.exists() and 'mod image_authoring;' not in before
    after = before.rstrip() + "\n" + WIRING
    changes = [(parent, before, after), (MODULE, "", SOURCE)]
    if picker:
        changes[1] = (MODULE, "", SOURCE + BUTTON_WIRING)
        changes.extend(picker_changes())
    if apply:
        changes = [change for change in changes if change[1] != change[2]]
        for file, prior, _ in changes:
            if prior:
                assert read_stage_source(file) == prior, file
            else:
                assert not file.exists(), file
        for file, prior, result in sorted(changes, key=lambda change: bool(change[1])):
            if prior:
                assert read_stage_source(file) == prior, file
            else:
                assert not file.exists(), file
                file.parent.mkdir(parents=True, exist_ok=True)
            with file.open("w", encoding="utf-8", newline="\n") as stream:
                stream.write(result)
    else:
        OUTPUT.mkdir(parents=True, exist_ok=True)
        patch = []
        for file, prior, result in changes:
            old_name = "a/" + file.as_posix() if prior else "/dev/null"
            new_name = "b/" + file.as_posix()
            patch.extend(difflib.unified_diff(prior.splitlines(keepends=True), result.splitlines(keepends=True), old_name, new_name))
        (OUTPUT / ("image-picker.patch" if picker else "image-authoring.patch")).write_text("".join(patch), encoding="utf-8")



def verify_closure(neighbors=False, live=False):
    """📏 Measure the staged or live paths through the normal Rust crate closure reader."""
    if live:
        locks = matching_locks()
        assert all(before == after for _, before, after in locks), "live PDF dependency rows differ"
        files = [(str(file), after) for file, _, after in locks]
    else:
        read_current = (DOCUMENT / "🦀️.rs").read_text(encoding="utf-8")
        files = [(str(DOCUMENT / "🦀️.rs"), read_current.rstrip() + "\n" + WIRING),
                 (str(MODULE), SOURCE + BUTTON_WIRING)]
        files += [(str(file), result) for file, _, result in picker_changes()]
    if neighbors and not live:
        boundary = OUTPUT.parent / "boundary-draft"
        mappings = json.loads((boundary / "overlay.json").read_text(encoding="utf-8"))
        if any(str(file) in mappings for file, _ in files):
            raise ValueError("Boundary and image overlays share source paths")
        files += [(file, (boundary / name).read_text(encoding="utf-8")) for file, name in mappings.items()]
        font = OUTPUT.parent / "text-font-edit"
        mappings = {
            "✏️editor/🖼️page/🔤️metrics/🦀️.rs": "metrics.rs",
            "✏️editor/🖼️page/🧮️content-ops/🦀️.rs": "content-ops.rs",
            "✏️editor/🖼️page/🖱️object-edits/🦀️.rs": "object-edits.rs",
            "✏️editor/🧪️tests/🔬️unit/🦀️.rs": "editor-tests.rs",
        }
        if any(str(PDF / file) in dict(files) for file in mappings):
            raise ValueError("Font and image overlays share source paths")
        files += [(str(PDF / file), (font / name).read_text(encoding="utf-8")) for file, name in mappings.items()]
    command = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--print", "cfg"]
    lines = subprocess.check_output(command, text=True).splitlines()
    cfg = [line.replace('"', "") for line in lines if line and not line.startswith("[")] + ["debug_assertions"]
    script = ["repo", "tools", "exec", "--light", "bun", "--", str(TICKET / "image-authoring-closures.ts")]
    result = subprocess.run(script, input=json.dumps({"files": files, "cfg": cfg}), text=True)
    if result.returncode:
        raise SystemExit(result.returncode)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--picker", action="store_true")
    parser.add_argument("--closure", action="store_true")
    parser.add_argument("--neighbors", action="store_true")
    parser.add_argument("--live", action="store_true")
    options = parser.parse_args()
    if options.closure:
        verify_closure(options.neighbors, options.live)
    else:
        stage(options.apply, options.picker)
