#!/usr/bin/env python3
# 🐍️ SCRATCH (ticket-local): W2 packet P3 (norm) mechanical editor migration.
# Moves each of the 15 🎛️apps/<app> trees into their subset's ✏️editor/, rewrites the
# apps::<app>:: -> editor::<app>:: module path inside every moved file, and transforms each
# app's own root component.rs (ArtifactApp -> ArtifactEditor, APP_ID removed, DIALECT added,
# App::builder -> Editor::builder, testkit fallout) IN PLACE on its own real content — never a
# synthesized template — so every app's genuine per-file differences (e.g. en1990's `text`
# vs din4108's `#[dsl(block)] snapshot` ReplaceSnapshot payload) survive untouched.
import re, shutil, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/📕️norm"
APPS_DIR = PLUGIN / "🎛️apps"
ARTIFACTS_DIR = PLUGIN / "🗿️artifacts"

APPS = [
    dict(dir="📓️iso16757", variant="iso16757", pascal="Iso16757", label="ISO 16757", family="Iso16757"),
    dict(dir="📔️vdi3805", variant="vdi3805", pascal="Vdi3805", label="VDI 3805", family="Vdi3805"),
    dict(dir="📕️din4108", variant="din4108", pascal="Din4108", label="DIN 4108", family="Din4108"),
    dict(dir="📗️din16798", variant="din16798", pascal="Din16798", label="DIN EN 16798", family="DinEn16798"),
    dict(dir="📘️en1990", variant="en1990", pascal="En1990", label="EN 1990", family="En1990"),
    dict(dir="📘️en1991", variant="en1991", pascal="En1991", label="EN 1991", family="En1991"),
    dict(dir="📘️en1992", variant="en1992", pascal="En1992", label="EN 1992", family="En1992"),
    dict(dir="📘️en1993", variant="en1993", pascal="En1993", label="EN 1993", family="En1993"),
    dict(dir="📘️en1994", variant="en1994", pascal="En1994", label="EN 1994", family="En1994"),
    dict(dir="📘️en1995", variant="en1995", pascal="En1995", label="EN 1995", family="En1995"),
    dict(dir="📘️en1996", variant="en1996", pascal="En1996", label="EN 1996", family="En1996"),
    dict(dir="📘️en1997", variant="en1997", pascal="En1997", label="EN 1997", family="En1997"),
    dict(dir="📘️en1998", variant="en1998", pascal="En1998", label="EN 1998", family="En1998"),
    dict(dir="📘️en1999", variant="en1999", pascal="En1999", label="EN 1999", family="En1999"),
    dict(dir="📙️din18599", variant="din18599", pascal="Din18599", label="DIN V 18599", family="DinV18599"),
]
for a in APPS:
    a["dialect_const"] = f"{a['pascal'].upper()}_DIALECT"
    a["doc_schema_const"] = f"{a['pascal'].upper()}_DOCUMENT_SCHEMA"

EDITOR_SUBTREE = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"


def subset_dir(app):
    return ARTIFACTS_DIR / app["dir"] / EDITOR_SUBTREE


def sed_module_path(text, app):
    old = f"apps::{app['variant']}::"
    new = f"editor::{app['variant']}::"
    return text.replace(old, new)


def move_tree(src: Path, dst: Path):
    assert src.is_dir(), f"missing source dir {src}"
    dst.parent.mkdir(parents=True, exist_ok=True)
    if dst.exists():
        shutil.rmtree(dst)
    shutil.move(str(src), str(dst))


def rewrite_rs_files_in_place(base: Path, app):
    for p in base.rglob("*.rs"):
        text = p.read_text(encoding="utf-8")
        new_text = sed_module_path(text, app)
        if new_text != text:
            p.write_text(new_text, encoding="utf-8")


def migrate_app_tree(app):
    old_app = APPS_DIR / app["dir"]
    editor = subset_dir(app)

    # 1) delete scaffold placeholders we are about to overwrite
    scaffold_mode = editor / "🎭️modes/✏️edit"
    if scaffold_mode.exists():
        shutil.rmtree(scaffold_mode)
    scaffold_root_ts = editor / "🟦️component.ts"
    scaffold_root_rs = editor / "🦀️component.rs"

    # 2) move the four content trees verbatim
    move_tree(old_app / "🎮️commands", editor / "🎮️commands")
    move_tree(old_app / "🎭️modes" / "✏️edit", editor / "🎭️modes" / "✏️edit")
    move_tree(old_app / "📌️panels", editor / "📌️panels")
    move_tree(old_app / "📚️examples", editor / "📚️examples")

    # 3) rewrite apps::<app>:: -> editor::<app>:: inside every moved .rs file
    for sub in ["🎮️commands", "🎭️modes", "📌️panels", "📚️examples"]:
        rewrite_rs_files_in_place(editor / sub, app)

    # 4) transform the root component.rs and move it into place
    root_text = (old_app / "🦀️component.rs").read_text(encoding="utf-8")
    root_text = transform_root_component(root_text, app)
    scaffold_root_rs.write_text(root_text, encoding="utf-8")

    # 5) remove the scaffold root component.ts (real content written by a later pass)
    if scaffold_root_ts.exists():
        scaffold_root_ts.unlink()

    return old_app


def transform_root_component(text: str, app: dict) -> str:
    pascal = app["pascal"]
    variant = app["variant"]
    dialect_const = app["dialect_const"]

    # -- apps::<app>:: -> editor::<app>:: everywhere in this file too (imports of sibling nodes)
    text = sed_module_path(text, app)

    # -- import line: ArtifactApp -> ArtifactEditor, App -> (drop, unused after builder rewrite)
    old_import = (
        "use semio_framework_plugin::{App, AppIo, ArtifactApp, ArtifactView, ConfigView, DraftView, Emit, Fault, "
        "LocalizedLabel, Media, MediaError, NoDraft, NoDraftMutation, UiNode};"
    )
    new_import = (
        "use semio_framework_plugin::{AppIo, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, "
        "LocalizedLabel, Media, MediaError, NoDraft, NoDraftMutation, UiNode};\n"
        "// 🚧️ SDK GAP: `Dialect` is not in `semio_framework_plugin`'s curated crate-root re-export list\n"
        "// (only `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` are,\n"
        "// per ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W0-F gap 1) — only reachable through `app`.\n"
        "use semio_framework_plugin::app::Dialect;"
    )
    assert old_import in text, f"[{variant}] import line not found verbatim"
    text = text.replace(old_import, new_import)

    # -- drop the top-level pub const APP_ID (dead after Editor::builder no longer takes an id)
    text = re.sub(r'pub const APP_ID: &str = "[^"]*";\n', "", text, count=1)

    # -- trait impl header
    text = text.replace(f"impl ArtifactApp for {pascal}PlayApp {{", f"impl ArtifactEditor for {pascal}PlayApp {{", 1)

    # -- const APP_ID (trait) -> const DIALECT
    text = re.sub(
        r'    const APP_ID: &\'static str = "[^"]*";\n',
        f"    const DIALECT: Dialect = crate::artifacts::{variant}::{dialect_const};\n",
        text,
        count=1,
    )

    # -- manifest builder rewrite
    old_head = f"pub fn create_{variant}_app() -> App {{\n    App::from_builder(\n        App::builder(APP_ID, LocalizedLabel::data(LABEL))\n"
    new_head = f"pub fn create_{variant}_app() -> semio_framework_plugin::AppDefinition {{\n    Editor::builder(crate::artifacts::{variant}::{dialect_const})\n"
    assert old_head in text, f"[{variant}] manifest head not found verbatim"
    text = text.replace(old_head, new_head, 1)

    # -- close out the builder chain: replace the trailing
    #      ,\n    )\n    .example(...)\n    .workflow(...)\n}
    #    with a plain .build_definition() and drop the examples/workflow calls (SDK gap, same as cad).
    m = re.search(
        r"\.keybinding\(\"mod\+shift\+z\", \"redo\"\),\n    \)\n    \.example\([^\n]*\n    \.workflow\([^\n]*\n\}",
        text,
    )
    assert m, f"[{variant}] manifest tail not found"
    replacement = (
        '.keybinding("mod+shift+z", "redo")\n'
        "            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder` takes a bare `AppDefinition` — there is no\n"
        "            // `.example(...)`/`.workflow(...)` on this builder (see the pilot's w2-cad-report.md \"SDK\n"
        "            // gaps\" #4), so the old app-level example/workflow registration is dropped here, not\n"
        "            // silently: the subset's own `📚️examples/🎬️demo-session` facet (real content, moved\n"
        "            // verbatim below) is the modern role-agnostic replacement surface for this.\n"
        "            .build_definition()\n}"
    )
    text = text[: m.start()] + replacement + text[m.end() :]

    # -- testkit module fallout
    text = text.replace(
        f"pub type NormApp = VcsArtifactApp<{pascal}PlayApp>;",
        f"pub type NormApp = VcsArtifactApp<EditorApp<{pascal}PlayApp>>;",
        1,
    )
    text = text.replace(
        f"        sdk_new_app::<{pascal}PlayApp>()",
        f"        sdk_new_app::<EditorApp<{pascal}PlayApp>>()",
        1,
    )
    old_registry = f"        new_app_with_registry::<{pascal}PlayApp>(create_{variant}_app)"
    new_registry = (
        f"        new_app_with_registry::<EditorApp<{pascal}PlayApp>>({variant}_manifest_for_testkit)"
    )
    assert old_registry in text, f"[{variant}] registry testkit line not found"
    text = text.replace(old_registry, new_registry, 1)

    # -- add EditorApp import + the manifest_for_testkit wrapper (contract §2.4 App{definition,examples} adapter)
    old_testkit_use = "use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};\n    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};"
    new_testkit_use = (
        "use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};\n"
        "    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};\n\n"
        f"    /// ✏️ Adapts `create_{variant}_app`'s `AppDefinition` (contract §2.4) into the `App {{ definition,\n"
        f"    /// examples }}` shape `testkit::new_app_with_registry` still expects (framework testkit gap,\n"
        f"    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).\n"
        f"    pub fn {variant}_manifest_for_testkit() -> semio_framework_plugin::App {{\n"
        f"        semio_framework_plugin::App {{ definition: create_{variant}_app(), examples: Vec::new() }}\n"
        f"    }}"
    )
    assert old_testkit_use in text, f"[{variant}] testkit use line not found"
    text = text.replace(old_testkit_use, new_testkit_use, 1)

    # -- the manifest test asserted the old hand-written id; the id is now derived (surface_app_id),
    #    already proven by the plugin-root `surface_tests` (assert_editor_and_viewer_share_dialect) —
    #    drop the stale assertion rather than pull in a new `semio-framework` Cargo dependency just to
    #    re-derive the expected string here (cad's own migrated editor test does not re-check `.id` either).
    old_id_assert = "        assert_eq!(definition.id, APP_ID);\n"
    assert old_id_assert in text, f"[{variant}] definition.id assertion not found"
    text = text.replace(old_id_assert, "", 1)

    return text


def delete_old_app_dir(old_app: Path):
    shutil.rmtree(old_app)


def main():
    for app in APPS:
        print("migrating", app["variant"])
        old_app = migrate_app_tree(app)
        delete_old_app_dir(old_app)
    if APPS_DIR.exists() and not any(APPS_DIR.iterdir()):
        APPS_DIR.rmdir()
        print("removed empty", APPS_DIR)
    elif APPS_DIR.exists():
        print("WARNING: apps dir not empty:", list(APPS_DIR.iterdir()))


if __name__ == "__main__":
    main()
