#!/usr/bin/env python3
from __future__ import annotations
import json, os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
SKIP_DIR_NAMES = {".git", ".repo", "node_modules", "target", "dist", "build", ".next", "coverage", "__pycache__", ".turbo", "out", ".venv", ".claude", ".vscode-test", "Visual Studio Code.app"}
TEXT_SUFFIXES = {".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".md", ".toml", ".wit", ".cs", ".go", ".py", ".yml", ".yaml", ".css", ".sh", ".ps1", ".s", ".dag", ".plan.md", ".spec.ts"}

REPLACEMENTS = [
    ("plugin_runtime", "program_runtime"),
    ("plugin_app_labels", "program_app_labels"),
    ("/studios/", "/spaces/"),
    ("parseStudioShellPath", "parseSpaceShellPath"),
    ("studioId", "spaceId"),
    ("StudioShell", "SpaceShell"),
    ("studioShell", "spaceShell"),
    ("studioE2e", "spaceE2e"),
    ("studio_uri", "space_uri"),
    ("studio_path", "space_path"),
    ("//#region 🔖️Studio", "//#region 🔖️Space"),
    ("//#endregion 🔖️Studio", "//#endregion 🔖️Space"),
    ("//#region StudioHistoryDocument", "//#region SpaceHistoryDocument"),
    ("//#endregion StudioHistoryDocument", "//#endregion SpaceHistoryDocument"),
    ("//#region 🏛️StudioTests", "//#region 🏛️SpaceTests"),
    ("fn studio_", "fn space_"),
    (" studio host", " space host"),
    (" studio checkpoint", " space checkpoint"),
    (" studio member", " space member"),
    (" studio-level", " space-level"),
    ("`(studio, document)`", "`(space, document)`"),
    ("about studio", "about space"),
    ("plugin-modules", "program-modules"),
    ("PLUGIN_BUILD_TARGETS", "PROGRAM_BUILD_TARGETS"),
    ("PLUGIN_TARGETS", "PROGRAM_TARGETS"),
    ("PluginBuildTarget", "ProgramBuildTarget"),
    ("pluginModuleUrl", "programModuleUrl"),
    ("pluginId", "programId"),
    ("plugin registry", "program registry"),
    ("semio plugin registry", "semio program registry"),
    ("plugin crate", "program crate"),
    ("plugin crates", "program crates"),
    ("plugin bundle", "program bundle"),
    ("plugin SDK", "program SDK"),
    ("plugin-init", "program-init"),
    ("plugin worker", "program worker"),
    ("plugin host", "program host"),
    ("plugin registration", "program registration"),
    ("plugin dispatch", "program dispatch"),
    ("plugin runtime", "program runtime"),
    ("WASM plugin", "WASM program"),
    ("wasm plugin", "wasm program"),
    ("hot-swappable WASM component", "hot-swappable WASM program"),
    (" — plugin", " — program"),
    (" plugin ", " program "),
    (" plugin.", " program."),
    (" plugin,", " program,"),
    (" plugin\n", " program\n"),
    (" plugin)", " program)"),
    ("(plugin ", "(program "),
    ("`plugin`", "`program`"),
    ("os-programs", "os-programs"),
    ("plugins.stories", "programs.stories"),
    ("os-plugins.spec", "os-programs.spec"),
    ("Declarative app plugin SDK", "Declarative app program SDK"),
    ("studio member resource", "space member artifact"),
    ("`Studio`/the", "`Space`/the"),
    ("blobHash? }` — `mediaType` mirrors", "blobHash? }` — `mediaType` mirrors"),
    ("studio parameter", "space parameter"),
    ("demo_studio", "demo_space"),
    ("parse_demo_studio", "parse_demo_space"),
    ("demo_studio_projection", "demo_space_projection"),
    ("DEMO_STUDIO_JSON", "DEMO_SPACE_JSON"),
    ("ensure_studio_fixtures", "ensure_space_fixtures"),
    ("StudioApp", "SpaceApp"),
    ("app_studio", "app_space"),
    ("SStudioApp", "SSpaceApp"),
    ("studio.os.json", "space.os.json"),
    ("studio-text-test", "space-text-test"),
]

def iter_files():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dp = Path(dirpath)
        if set(dp.parts) & SKIP_DIR_NAMES: 
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        for name in filenames:
            p = dp / name
            if p.suffix in TEXT_SUFFIXES or name in {".gitignore", "Cargo.lock"}:
                yield p

def rename_story_files():
    renames = [
        (ROOT / ".storybook/stories/framework/os/plugins.stories.tsx", ROOT / ".storybook/stories/framework/os/programs.stories.tsx"),
        (ROOT / ".storybook/os-plugins.spec.ts", ROOT / ".storybook/os-programs.spec.ts"),
    ]
    log = []
    for src, dst in renames:
        if src.is_file() and not dst.exists():
            src.rename(dst)
            log.append(f"{src.name} -> {dst.name}")
    return log

def main():
    file_renames = rename_story_files()
    touched = []
    for path in iter_files():
        if "vite" in path.name and path.suffix == ".ts":
            continue
        try:
            original = path.read_text(encoding="utf-8")
        except Exception:
            continue
        updated = original
        for old, new in REPLACEMENTS:
            updated = updated.replace(old, new)
        if updated != original:
            path.write_text(updated, encoding="utf-8")
            touched.append(str(path.relative_to(ROOT)))
    report = {"file_renames": file_renames, "touched": len(touched)}
    (TICKET / "rename-pass2-report.json").write_text(json.dumps(report, indent=2))
    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    main()
