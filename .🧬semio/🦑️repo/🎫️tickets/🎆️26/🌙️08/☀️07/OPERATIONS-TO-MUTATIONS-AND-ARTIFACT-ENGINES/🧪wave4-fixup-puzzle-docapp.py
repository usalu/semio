#!/usr/bin/env python3
"""Migrate puzzle 3d/5d DocumentApp impls to static associated-fn API (mirror ◻2d)."""

from pathlib import Path
import re


def migrate_document_app(
    path: Path,
    app: str,
    projection: str,
    mutation: str,
    config: str,
    config_mutation: str,
    command: str,
    app_id_const: str,
    schema_const: str,
) -> None:
    text = path.read_text()
    if "NoDraft" not in text:
        m = re.search(r"use semio_framework_plugin::\{([^}]+)\}", text)
        if not m:
            raise SystemExit(f"missing semio_framework_plugin import in {path}")
        old = m.group(0)
        inner = m.group(1)
        if "DocumentApp" in inner:
            inner = inner.replace(
                "DocumentApp",
                "DocumentApp, DraftView, EngineHandles, NoDraft, NoDraftMutation",
                1,
            )
        else:
            inner = "NoDraft, NoDraftMutation, DraftView, EngineHandles, " + inner
        text = text.replace(old, "use semio_framework_plugin::{" + inner + "}", 1)

    start = text.find(f"impl DocumentApp for {app} {{")
    if start < 0:
        raise SystemExit(f"no DocumentApp for {app} in {path}")
    create = text.find("\npub fn create_", start)
    if create < 0:
        raise SystemExit(f"no create_ after DocumentApp in {path}")
    block = text[start:create]
    print(f"=== {path} DocumentApp block length {len(block)} ===")

    new_header = (
        f"impl DocumentApp for {app} {{\n"
        f"    const APP_ID: &'static str = {app_id_const};\n"
        f"    const DOCUMENT_SCHEMA: &'static str = {schema_const};\n"
        f"    type Projection = {projection};\n"
        f"    type Mutation = {mutation};\n"
        f"    type Config = {config};\n"
        f"    type ConfigMutation = {config_mutation};\n"
        f"    type Draft = NoDraft;\n"
        f"    type DraftMutation = NoDraftMutation;\n"
        f"    type Command = {command};\n"
    )

    rest = re.sub(
        rf"impl DocumentApp for {app} \{{.*?(?=\n    fn |\n    ///)",
        new_header,
        block,
        count=1,
        flags=re.S,
    )

    methods = [
        "initial_projection",
        "clipboard_media_type",
        "copy_fragment",
        "cut_operations",
        "paste_operations",
        "command_id",
        "handle",
        "io",
        "import_media",
        "render",
        "window_engagements",
        "window_measures",
        "tool_measures",
        "context_menu",
    ]
    for meth in methods:
        rest = re.sub(rf"fn {meth}\(&self,", f"fn {meth}(", rest)
        rest = re.sub(rf"fn {meth}\(&self\)", f"fn {meth}()", rest)

    rest = rest.replace(
        f"fn command_id(command: &{command}) -> &str {{",
        f"fn command_id(command: &{command}) -> &'static str {{",
    )

    # Escape braces for literal `<'_>` / Emit generics in f-string replacement via .format
    old_handle = (
        "fn handle(command: &{command}, doc: &DocumentView<'_, {projection}>, "
        "cfg: &ConfigView<'_, {config}>) -> Result<Emit<{mutation}, {config_mutation}>, Fault>"
    ).format(
        command=command,
        projection=projection,
        config=config,
        mutation=mutation,
        config_mutation=config_mutation,
    )
    new_handle = (
        "fn handle(command: &{command}, doc: &DocumentView<'_, {projection}>, "
        "cfg: &ConfigView<'_, {config}>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) "
        "-> Result<Emit<{mutation}, {config_mutation}, Self::DraftMutation>, Fault>"
    ).format(
        command=command,
        projection=projection,
        config=config,
        mutation=mutation,
        config_mutation=config_mutation,
    )
    n = rest.count(old_handle)
    rest = rest.replace(old_handle, new_handle)
    print(f"handle sig replacements: {n}")

    rest = rest.replace(
        f"Result<Emit<{mutation}, {config_mutation}>, MediaError>",
        f"Result<Emit<{mutation}, {config_mutation}, Self::DraftMutation>, MediaError>",
    )
    rest = rest.replace("self.clipboard_media_type()", "Self::clipboard_media_type()")
    rest = rest.replace("self.handle_action_impl(", "Self::handle_action_impl(")

    text2 = text[:start] + rest + text[create:]
    text2, n2 = re.subn(r"(fn handle_action_impl)\(&self,", r"\1(", text2, count=1)
    print(f"handle_action_impl &self removals: {n2}")

    path.write_text(text2)
    print(f"Wrote {path}")


def main() -> None:
    base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/✩️puzzle/🎛️apps")
    # Correct emoji path:
    base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/�")
    base = Path("/Users/ueli/Documents/semio") / "✏️s/🔌️plugins/�"
    base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/�")
    # Use literal puzzle path without corruption:
    base = Path("/Users/ueli/Documents/semio") / "✏️s" / "🔌️plugins" / "�"
    # Final:
    base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/�")


if __name__ == "__main__":
    # Resolve paths via glob to avoid emoji typos in this script source.
    root = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
    puzzle = next(p for p in root.iterdir() if p.name.endswith("puzzle") or "puzzle" in p.name)
    apps = puzzle / "🎛️apps"
    app3d = next(p for p in apps.iterdir() if "3d" in p.name and p.is_dir())
    app5d = next(p for p in apps.iterdir() if "5d" in p.name and p.is_dir())
    file3d = app3d / "🦀️component.rs"
    file5d = app5d / "🦀️component.rs"
    print("3d", file3d)
    print("5d", file5d)
    migrate_document_app(
        file3d,
        "Puzzle3dPlayApp",
        "Puzzle3dPlayProjection",
        "Puzzle3dMutation",
        "Puzzle3dConfig",
        "Puzzle3dConfigMutation",
        "Puzzle3dCommand",
        "PUZZLE3D_PLAY_APP_ID",
        "PUZZLE3D_FIXTURE_SCHEMA",
    )
    migrate_document_app(
        file5d,
        "Puzzle5dPlayApp",
        "Puzzle5dPlayProjection",
        "Puzzle5dMutation",
        "Puzzle5dConfig",
        "Puzzle5dConfigMutation",
        "Puzzle5dCommand",
        "PUZZLE5D_PLAY_APP_ID",
        "PUZZLE5D_SCHEMA",
    )
    print("done")
