#!/usr/bin/env python3
"""Apply package rename map across the monorepo (ticket-only orchestrator)."""
from __future__ import annotations

import json
import os
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
TICKET = Path(__file__).resolve().parent
SKIP_DIRS = {
    "node_modules",
    "target",
    ".git",
    "dist",
    ".venv",
    "storybook-static",
    ".nx",
}
SKIP_FILE_SUFFIX = {".png", ".jpg", ".jpeg", ".gif", ".webp", ".woff", ".woff2", ".ico", ".pdf", ".zip", ".wasm"}

RUST_RENAMES: dict[str, str] = {
    "animate-plugin": "semio-s-plugin-animate",
    "animate_core": "semio-s-plugin-animate-core",
    "animate_video": "semio-s-plugin-animate-video",
    "architect-spine": "semio-s-plugin-architect-spine",
    "architect_spine": "semio-s-plugin-architect-spine-module",
    "block-plugin": "semio-s-plugin-block",
    "cad-plugin": "semio-s-plugin-cad",
    "compose": "semio-compose-rs",
    "compose-gql": "semio-compose-gql",
    "compose-hub": "semio-compose-hub",
    "compose_query": "semio-compose-query",
    "dag-plugin": "semio-s-plugin-dag",
    "draw-plugin": "semio-s-plugin-draw",
    "energy_engine": "semio-s-plugin-energy-engine",
    "fem_core": "semio-s-plugin-fem-core",
    "flow-plugin": "semio-s-plugin-flow",
    "forms-plugin": "semio-s-plugin-forms",
    "framework_editor": "semio-framework-editor",
    "framework_surface_node_graph": "semio-framework-os-kernel-surface-node-graph",
    "framework_surface_paint": "semio-framework-os-kernel-surface-paint",
    "framework_surface_terrain": "semio-framework-os-kernel-surface-terrain",
    "framework_surface_tiled_map": "semio-framework-os-kernel-surface-tiled-map",
    "fsm": "semio-s-plugin-draw-fsm",
    "fsm_macros": "semio-s-plugin-draw-fsm-macros",
    "hub": "semio-hub",
    "hub-directory": "semio-hub-directory",
    "hub-directory-neo4j": "semio-hub-directory-neo4j",
    "hub-directory-postgres": "semio-hub-directory-postgres",
    "hub-directory-sqlite": "semio-hub-directory-sqlite",
    "imperative-plugin": "semio-s-plugin-imperative",
    "imperative_engine": "semio-s-kernel-imperative",
    "imperative_module_control": "semio-s-plugin-imperative-control",
    "imperative_module_core": "semio-s-plugin-imperative-core",
    "imperative_module_logic": "semio-s-plugin-imperative-logic",
    "imperative_module_math": "semio-s-plugin-imperative-math",
    "imperative_module_text": "semio-s-plugin-imperative-text",
    "lowpoly-plugin": "semio-s-plugin-lowpoly",
    "mathematical-plugin": "semio-s-plugin-mathematical",
    "norm-plugin": "semio-s-plugin-norm",
    "norm_core": "semio-s-plugin-norm-core",
    "playbook-module-procedural": "semio-s-plugin-playbook-procedural",
    "playbook-plugin": "semio-s-plugin-playbook",
    "procedural-plugin": "semio-s-plugin-procedural",
    "process-plugin": "semio-s-plugin-process",
    "puzzle-plugin": "semio-s-plugin-puzzle",
    "raster-plugin": "semio-s-plugin-raster",
    "reasoning-mindmap-plugin": "semio-s-plugin-reasoning-mindmap",
    "reasoning_mindmap": "semio-s-kernel-reasoning-mindmap",
    "remodel-plugin": "semio-s-plugin-remodel",
    "remodel_camera": "semio-s-plugin-remodel-camera",
    "remodel_dense": "semio-s-plugin-remodel-dense",
    "remodel_engine": "semio-s-plugin-remodel-engine",
    "remodel_feature": "semio-s-plugin-remodel-feature",
    "remodel_geo": "semio-s-plugin-remodel-geo",
    "remodel_image": "semio-s-plugin-remodel-image",
    "remodel_mesh": "semio-s-plugin-remodel-mesh",
    "remodel_motion": "semio-s-plugin-remodel-motion",
    "remodel_sfm": "semio-s-plugin-remodel-sfm",
    "remodel_video": "semio-s-plugin-remodel-video",
    "repo_cli": "semio-framework-repo-cli",
    "s-plugin": "semio-s-plugin-space",
    "sequence-plugin": "semio-s-plugin-sequence",
    "shooting-plugin": "semio-s-plugin-shooting",
    "sourcing-module-beams": "semio-s-plugin-sourcing-beams",
    "sourcing-module-slabs": "semio-s-plugin-sourcing-slabs",
    "sourcing-module-windows": "semio-s-plugin-sourcing-windows",
    "sourcing-plugin": "semio-s-plugin-sourcing",
    "trinity-plugin": "semio-s-plugin-trinity",
    "trinity_jack": "semio-s-plugin-trinity-jack",
    "trinity_jack_lsp": "semio-s-plugin-trinity-jack-lsp",
    "trinity_jack_shell": "semio-s-plugin-trinity-jack-shell",
    "trinity_ram": "semio-s-plugin-trinity-ram",
    "ui_styling": "semio-framework-ui-styling",
    "ui_tui": "semio-framework-ui-tui",
    "ui_wgpu": "semio-framework-ui-wgpu",
    "writer-plugin": "semio-s-plugin-writer",
}

NPM_RENAMES: dict[str, str] = {
    '"name": "compose"': '"name": "workspace"',
    '"name": "compose-vscode"': '"name": "@semio-tech/compose-vscode"',
    '"name": "repo-vscode"': '"name": "@semio-tech/repo-vscode"',
    "@semio-tech/infinite-canvas-react-renderer": "@semio-tech/infinite-canvas-react-renderer",
    "@semio-tech/semio-asset": "@semio-tech/asset",
    "@semio-tech/semio-icon": "@semio-tech/icon",
    "@semio-tech/semio-image": "@semio-tech/image",
    "@semio-tech/semio-logo": "@semio-tech/logo",
    "@semio-tech/trinity-jack-lsp-worker": "@semio-tech/trinity-jack-lsp",
    "@semio-tech/trinity-ram-rs": "@semio-tech/trinity-ram",
}

PYTHON_RENAMES: dict[str, str] = {
    'name = "compose"': 'name = "semio-compose"',
    'name = "ui-styling"': 'name = "semio-framework-ui-styling"',
}


def rust_ident(pkg: str) -> str:
    return pkg.replace("-", "_")


def should_walk_dir(name: str) -> bool:
    return name not in SKIP_DIRS and not name.startswith(".") or name == ".vscode"


def iter_files() -> list[Path]:
    out: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if should_walk_dir(d) or d == ".🦑️repo"]
        if any(p in dirpath for p in ("/node_modules/", "/target/", "/.venv/")):
            continue
        for fn in filenames:
            p = Path(dirpath) / fn
            if p.suffix in SKIP_FILE_SUFFIX:
                continue
            if fn == "apply_package_renames.py":
                continue
            out.append(p)
    return out


def apply_rust_renames(content: str, path: Path) -> str:
    for old, new in sorted(RUST_RENAMES.items(), key=lambda x: -len(x[0])):
        content = content.replace(f'name = "{old}"', f'name = "{new}"')
        content = content.replace(f'package = "{old}"', f'package = "{new}"')
        if path.suffix == ".rs":
            old_id = rust_ident(old)
            new_id = rust_ident(new)
            if old_id != new_id:
                content = re.sub(rf"\b{re.escape(old_id)}\b", new_id, content)
    return content


def apply_npm_renames(content: str) -> str:
    for old, new in sorted(NPM_RENAMES.items(), key=lambda x: -len(x[0])):
        content = content.replace(old, new)
    return content


def main() -> None:
    rename_map = {
        "rust": RUST_RENAMES,
        "npm": NPM_RENAMES,
        "python": PYTHON_RENAMES,
    }
    (TICKET / "rename-map.json").write_text(
        json.dumps(rename_map, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    exts = {
        ".toml",
        ".rs",
        ".ts",
        ".tsx",
        ".js",
        ".jsx",
        ".mjs",
        ".cjs",
        ".json",
        ".md",
        ".sln",
        ".csproj",
        ".cs",
        ".go",
        ".py",
        ".lock",
    }
    changed = 0
    for path in iter_files():
        if path.suffix not in exts and path.name not in ("Cargo.lock", "bun.lock", "go.work", "go.work.sum"):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        orig = text
        if path.name in ("Cargo.toml", "Cargo.lock") or path.suffix in (".toml", ".rs"):
            text = apply_rust_renames(text, path)
        if path.name in ("package.json", "project.json", "bun.lock") or path.suffix == ".json":
            text = apply_npm_renames(text)
        if path.name == "pyproject.toml":
            for old, new in PYTHON_RENAMES.items():
                text = text.replace(old, new)
        if path.suffix in (".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".md", ".json"):
            text = apply_npm_renames(text)
            text = apply_rust_renames(text, path)
        if text != orig:
            path.write_text(text, encoding="utf-8")
            changed += 1
    print(f"[apply_package_renames] updated {changed} files")
    print(f"[apply_package_renames] map written to {TICKET / 'rename-map.json'}")


if __name__ == "__main__":
    main()
