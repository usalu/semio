#!/usr/bin/env python3
"""👁️ C13 prepared set P2 (row 3.4, guest-linked → L1 train with P1): a viewer's manifest declares no verb its guard rejects.

Measured 2026-09-28 21:2x on hub 7800 (catalog p24, `two-human --journey viewer`, all 4 kinds): a Spectator's viewer window offers
Undo/Redo/Commit Checkpoint/Create Alternative/Cut/Paste in its action pane and binds mod+z/mod+x/mod+v, because
`try_build_definition` appends every framework history + clipboard verb to every app whatever its role; the guest's
`ViewerGuard` then refuses them at dispatch (`viewer.read-only`), which the browser actor reports only as
`action-guest-refused` — the viewer gets live edit controls that silently do nothing. The manifest is the contract every host
derives its action pane, palette and keybindings from, so the builder must not declare on a viewer what the viewer refuses:
the read cursor (`checkoutCheckpoint`/`switchAlternative`) and `copy` stay, `VIEWER_REJECTED_ACTION_IDS` go. The guard stays
the backstop for a hand-written dispatch.

Touches: the app builder (`🔌️plugin/🦀️.rs`: role parsed once, before the framework verbs are appended), the guard's doc, a
builder law (`🧪️tests/🔬️app-app-builder/🦀️.rs`), and the TS mirror's doc (`🛂️manifest/🟦️.ts`). Idempotent and all-or-nothing.

usage: p2-viewer-manifest-no-rejected-verbs.py [--root <repo or overlay>] (--dry-run | --write | --revert)
  --write   byte-backs up every touched file to `w3-backup/c13-p2/before/` (+ the written bytes to `after/`) before writing
  --revert  restores `before/` for every file whose live bytes still equal `after/`; a file edited since is listed and kept"""
import json
import pathlib
import sys

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1]) if "--root" in args else pathlib.Path("/Users/ueli/Documents/semio")
modes = [mode for mode in ("--dry-run", "--write", "--revert") if mode in args]
if len(modes) != 1:
    sys.exit("usage: p2-viewer-manifest-no-rejected-verbs.py [--root <dir>] (--dry-run | --write | --revert)")
mode = modes[0]
backup = pathlib.Path(__file__).resolve().parent / "w3-backup" / "c13-p2"

PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER_LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-app-builder/🦀️.rs"
MANIFEST_TS = "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"

PARSE = """            let (dialect, role) = semio_framework::parse_surface_app_id(&self.id).map_err(|error| PluginAssemblyError::new("app-definition.invalid", format!("app id {} must be a canonical surface id: {error}", self.id)))?;
"""

HUNKS: dict[str, list[tuple[str, str]]] = {
    PLUGIN: [
        (
            """            let app_declared_actions = !self.actions.is_empty();
            let mut actions = self.actions;
            for history_action in history_action_definitions() {
                if declared_action_ids.insert(history_action.id.clone()) {
                    actions.push(history_action);
                }
            }
            for clipboard_action in clipboard_action_definitions() {
                if declared_action_ids.insert(clipboard_action.id.clone()) {
                    actions.push(clipboard_action);
                }
            }
""",
            PARSE
            + """            let app_declared_actions = !self.actions.is_empty();
            let mut actions = self.actions;
            for framework_action in history_action_definitions().into_iter().chain(clipboard_action_definitions()).filter(|action| role != AppRole::Viewer || !VIEWER_REJECTED_ACTION_IDS.contains(&action.id.as_str())) {
                if declared_action_ids.insert(framework_action.id.clone()) {
                    actions.push(framework_action);
                }
            }
""",
        ),
        (
            PARSE + """            let mut definition = AppDefinition {
""",
            """            let mut definition = AppDefinition {
""",
        ),
        (
            """    /// `switchAlternative` are deliberately absent: they move the read cursor across ALREADY-EXISTING
    /// history and never create new content, so a viewer may still browse checkpoints/alternatives.
    const VIEWER_REJECTED_ACTION_IDS""",
            """    /// `switchAlternative` are deliberately absent: they move the read cursor across ALREADY-EXISTING
    /// history and never create new content, so a viewer may still browse checkpoints/alternatives.
    /// `try_build_definition` never declares these on a viewer app, so no host offers or binds them to a
    /// Spectator; this guard is the backstop for a dispatch no manifest offered.
    const VIEWER_REJECTED_ACTION_IDS""",
        ),
    ],
    BUILDER_LAWS: [
        (
            """    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {""",
            """    /// 👁️ One app of `slug`'s test dialect in `role`, otherwise [`minimal_app`].
    async fn surface_app(slug: &str, role: AppRole) -> AppDefinition {
        App::builder(surface_app_id(&ArtifactDialect { artifact_kind: format!("s.test.app-builder.{slug}"), standard: "1".into(), subset: "*".into() }, role), LocalizedLabel::data("App"))
            .await
            .document(["semio", slug])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), format!("{slug}.main"), SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .build_definition()
    }

    /// 👁️ Row 3.4 (C13, measured on hub 7800 p24): a viewer's manifest declares — and binds — no verb its `ViewerGuard`
    /// rejects, so no host offers a Spectator an edit control; the read cursor and `copy` stay, and the editor of the same
    /// dialect keeps every framework verb.
    #[semio_framework_async_macros::async_test]
    async fn build_definition_offers_a_viewer_no_verb_its_guard_rejects() {
        let viewer = surface_app("viewer-guard", AppRole::Viewer).await;
        let editor = surface_app("viewer-guard", AppRole::Editor).await;
        let viewer_ids: HashSet<&str> = declared_actions(&viewer).map(|action| action.id.as_str()).collect();
        let editor_ids: HashSet<&str> = declared_actions(&editor).map(|action| action.id.as_str()).collect();
        for verb in VIEWER_REJECTED_ACTION_IDS {
            assert!(!viewer_ids.contains(verb), "a viewer declares {verb}");
            assert!(viewer.keybindings.iter().all(|binding| binding.action.action != verb), "a viewer binds {verb}");
        }
        for verb in ["switchAlternative", "checkoutCheckpoint", "copy"] {
            assert!(viewer_ids.contains(verb), "a viewer still browses history and copies: {verb}");
        }
        for verb in ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint", "copy", "cut", "paste"] {
            assert!(editor_ids.contains(verb), "the editor keeps {verb}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_does_not_duplicate_manually_declared_history_keybinding() {""",
        ),
    ],
    MANIFEST_TS: [
        (
            """/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every app receives, used by the shell to render the same set without a wasm round trip. */""",
            """/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every editor receives, used by the shell to render the same set without a wasm round trip. A viewer
 * receives only the read cursor (`switchAlternative`, `checkoutCheckpoint`): its manifest never declares a verb its guard
 * rejects. */""",
        ),
    ],
}

if mode == "--revert":
    manifest = json.loads((backup / "manifest.json").read_text(encoding="utf-8"))
    kept = 0
    for relative in manifest["files"]:
        live = root / relative
        if live.read_bytes() != (backup / "after" / relative).read_bytes():
            kept += 1
            print(f"{relative}: KEPT (edited since the write)")
            continue
        live.write_bytes((backup / "before" / relative).read_bytes())
        print(f"{relative}: reverted")
    print(f"REVERTED: {kept} file(s) kept")
    sys.exit(1 if kept else 0)

problems = 0
writes: dict[pathlib.Path, str] = {}
for relative, hunks in HUNKS.items():
    path = root / relative
    original = path.read_text(encoding="utf-8")
    text = original
    for old, new in hunks:
        if old in new:
            state = "already applied" if text.count(new) == 1 else "applies" if text.count(old) == 1 else f"MISSING (old {text.count(old)}, new {text.count(new)})"
        else:
            state = "applies" if text.count(old) == 1 else "already applied" if text.count(new) == 1 else f"MISSING (old {text.count(old)}, new {text.count(new)})"
        if state == "applies":
            text = text.replace(old, new)
        if state.startswith("MISSING"):
            problems += 1
        print(f"{relative}: {state}: {old.strip().splitlines()[0][:90]}")
    if text != original:
        writes[path] = text
write = mode == "--write" and problems == 0
if write:
    files = [str(path.relative_to(root)) for path in writes]
    for path, text in writes.items():
        relative = path.relative_to(root)
        for side, data in (("before", path.read_bytes()), ("after", text.encode("utf-8"))):
            target = backup / side / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    (backup / "manifest.json").write_text(json.dumps({"root": str(root), "files": files}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for path, text in writes.items():
        path.write_text(text, encoding="utf-8")
print(f"{'WRITTEN' if write else 'DRY RUN'}: {problems} problem(s), {len(writes)} file(s)")
sys.exit(1 if problems else 0)
