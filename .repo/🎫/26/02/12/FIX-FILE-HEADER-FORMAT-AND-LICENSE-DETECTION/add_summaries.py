#!/usr/bin/env python3
"""Add missing summary comments to file headers."""

import os
import re

SUMMARIES = {
    "compose/assets/icons.ts": "Re-exports Lucide React icons with domain-specific semantic aliases.",
    "compose/assets/index.ts": "Barrel export for all asset modules including icons, fonts, models and images.",
    "compose/assets/logo/logo.ts": "Generates animated SVG logo from static SVG input with keyframe sequences.",
    "compose/desktop/forge.config.ts": "Electron Forge configuration for building and packaging the desktop app.",
    "compose/desktop/forge.env.d.ts": "Type declarations for Electron Forge environment variables.",
    "compose/desktop/main.ts": "Entry point for the Electron main process managing windows and lifecycle.",
    "compose/desktop/postcss.config.ts": "PostCSS configuration for the desktop app with Tailwind and autoprefixer.",
    "compose/desktop/preload.ts": "Electron preload script exposing safe APIs to the renderer process.",
    "compose/desktop/renderer.tsx": "Entry point for the Electron renderer process mounting the React app.",
    "compose/desktop/tailwind.config.ts": "Tailwind CSS configuration for the desktop app styling.",
    "compose/desktop/vite.main.config.ts": "Vite build configuration for the Electron main process.",
    "compose/desktop/vite.preload.config.ts": "Vite build configuration for the Electron preload script.",
    "compose/desktop/vite.renderer.config.ts": "Vite build configuration for the Electron renderer process.",
    "compose/docs/index.tsx": "Entry point for the documentation site React app.",
    "compose/docs/postcss.config.ts": "PostCSS configuration for the docs site with Tailwind and autoprefixer.",
    "compose/docs/tailwind.config.ts": "Tailwind CSS configuration for the documentation site styling.",
    "compose/engine/build.ts": "Build script for the compose engine Python package.",
    "compose/engine/generate-schemas.ts": "Generates JSON schemas from the engine's Python models.",
    "compose/engine/post-build.ts": "Post-build script for engine artifact processing and packaging.",
    "compose/engine/sqliteschema.ts": "Exports the SQLite schema definition for the engine database.",
    "compose/gh/Compose.Grasshopper/build.ts": "Build script for the Grasshopper plugin assembly.",
    "compose/gh/Compose.Grasshopper/build-value-lists.ts": "Generates Grasshopper value list presets from domain data.",
    "compose/gh/Compose.Grasshopper/Compose.Grasshopper.cs": "Main Grasshopper plugin providing domain components for Rhino.",
    "compose/gh/Compose.Grasshopper/yak/build.ts": "Build script for Yak package distribution of the Grasshopper plugin.",
    "compose/gh/Compose.Grasshopper/yak/login.ts": "Authenticates with the Yak package server for plugin publishing.",
    "compose/gh/Compose.Grasshopper/yak/publish.ts": "Publishes the Grasshopper plugin package to the Yak server.",
    "compose/gh/Compose.Grasshopper/yak/test-push.ts": "Tests the Yak package push workflow for the Grasshopper plugin.",
    "compose/gh/Compose.Grasshopper/yak/test-search.ts": "Tests Yak package search functionality for the Grasshopper plugin.",
    "compose/gh/Compose.Grasshopper/yak/unyank.ts": "Restores a previously yanked version of the Grasshopper Yak package.",
    "compose/gh/Compose.Grasshopper/yak/yank.ts": "Yanks a specific version of the Grasshopper Yak package from the registry.",
    "compose/go/kit_sqlite.go": "SQLite-backed persistence layer for kit import and export operations.",
    "compose/go/compose.go": "Core domain library in Go implementing the compose data model and operations.",
    "compose/js/dev.ts": "Development server entry point for the JavaScript workspace.",
    "compose/js/eslint.config.ts": "ESLint configuration for the JavaScript workspace linting rules.",
    "compose/js/global.d.ts": "Global type declarations for the JavaScript workspace.",
    "compose/js/i18n.ts": "Internationalization setup and translation utilities for the UI.",
    "compose/js/index.ts": "Barrel export for the core JavaScript workspace modules.",
    "compose/jsonschema/build.ts": "Build script for generating and exporting JSON Schema definitions.",
    "compose/js/playwright.config.ts": "Playwright end-to-end test configuration for the JavaScript workspace.",
    "compose/js/postcss.config.ts": "PostCSS configuration for the JavaScript workspace with Tailwind.",
    "compose/js/compose.ts": "Core domain model types, schemas and utilities for the compose platform.",
    "compose/js/site.tsx": "Landing page and marketing site React component.",
    "compose/js/sketchpad/apps/index.ts": "Barrel export for all sketchpad app components.",
    "compose/js/sketchpad/Design.tsx": "Design app providing diagram and scene windows for editing designs.",
    "compose/js/sketchpad/Docs.tsx": "Documentation viewer app with workbench and detail panels.",
    "compose/js/sketchpad/elements.tsx": "Shared UI elements and primitive components for sketchpad apps.",
    "compose/js/sketchpad/Feedback.tsx": "Feedback collection app with rating hooks and submission forms.",
    "compose/js/sketchpad/Home.tsx": "Home screen app showing recent projects and getting started content.",
    "compose/js/sketchpad/kitSelectionHelper.ts": "Geometry and selection utilities for kit diagram interactions.",
    "compose/js/sketchpad/Kit.tsx": "Kit editor app for managing types, designs and qualities.",
    "compose/js/sketchpad/portColor.ts": "Color mapping utilities for port visualization in diagrams.",
    "compose/js/sketchpad/Quality.tsx": "Quality inspection app for viewing and editing quality attributes.",
    "compose/js/sketchpad/shared.ts": "Shared state management types, hooks and store factories for sketchpad.",
    "compose/js/sketchpad/Sketchpad.tsx": "Main sketchpad container managing app tabs, panels and window layout.",
    "compose/js/sketchpad/Tutorials.tsx": "Interactive tutorial system with step-by-step guided workflows.",
    "compose/js/sketchpad/Type.tsx": "Type editor app for defining and editing type properties and ports.",
    "compose/js/tailwind.config.ts": "Tailwind CSS configuration for the JavaScript workspace styling.",
    "compose/js/vite.config.ts": "Vite build and development configuration for the JavaScript workspace.",
    "compose/js/vite-env.d.ts": "Vite client type declarations for the JavaScript workspace.",
    "compose/net/Compose/build.ts": "Build script for the Compose .NET library assembly.",
    "compose/net/Compose/Compose.cs": "Core .NET library implementing the compose domain model and serialization.",
    "compose/play/index.tsx": "Entry point for the playground React app for interactive experimentation.",
    "compose/play/postcss.config.ts": "PostCSS configuration for the playground app with Tailwind.",
    "compose/play/tailwind.config.ts": "Tailwind CSS configuration for the playground app styling.",
    "compose/play/vite.config.ts": "Vite build and development configuration for the playground app.",
    "repo/cli/main.go": "Monorepo CLI tool for repository management, analysis and code generation.",
    "repo/server/main.go": "GraphQL server for the monorepo management API.",
    "repo/vscode/codegen.ts": "Code generation script for VS Code extension GraphQL types.",
    "repo/vscode/eslint.config.ts": "ESLint configuration for the VS Code extension linting rules.",
    "repo/vscode/extension.ts": "VS Code extension providing monorepo navigation, analysis and commands.",
    "repo/vscode/queries.ts": "GraphQL query document constants for the VS Code extension.",
    "repo/vscode/vite.config.ts": "Vite build configuration for the VS Code extension bundling.",
    "repo/vscode/vite.test.config.ts": "Vite test configuration for the VS Code extension test runner.",
    "compose/sketchpad/index.tsx": "Entry point for the standalone sketchpad web application.",
    "compose/sketchpad/postcss.config.ts": "PostCSS configuration for the sketchpad app with Tailwind.",
    "compose/sketchpad/tailwind.config.ts": "Tailwind CSS configuration for the sketchpad app styling.",
    "compose/sketchpad/vite.config.ts": "Vite build and development configuration for the sketchpad app.",
    "vitest.config.ts": "Root Vitest configuration for the monorepo test runner.",
}


def get_comment_prefix(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "#"
    if ext in (".cs",):
        return "//"
    if ext in (".sql",):
        return "--"
    return "//"


def get_header_region_start(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "# region Header"
    if ext in (".cs",):
        return "#region \U0001f516Header"
    return "// #region \U0001f516Header"


def get_header_region_end(filepath):
    ext = os.path.splitext(filepath)[1]
    if ext in (".py",):
        return "# endregion Header"
    if ext in (".cs",):
        return "#endregion \U0001f516Header"
    return "// #endregion \U0001f516Header"


def add_summary(root_dir, filepath, summary):
    abs_path = os.path.join(root_dir, filepath)
    if not os.path.exists(abs_path):
        print(f"  SKIP (not found): {filepath}")
        return False
    with open(abs_path, "r", encoding="utf-8") as f:
        content = f.read()
    lines = content.split("\n")
    cp = get_comment_prefix(filepath)
    header_end_marker = get_header_region_end(filepath)
    header_end_idx = -1
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == header_end_marker or stripped == header_end_marker.strip():
            header_end_idx = i
            break
    if header_end_idx == -1:
        print(f"  SKIP (no header end): {filepath}")
        return False
    summary_line = f"{cp} {summary}"
    if lines[header_end_idx - 1].strip() == "":
        lines.insert(header_end_idx, summary_line)
        lines.insert(header_end_idx + 1, "")
    else:
        lines.insert(header_end_idx, "")
        lines.insert(header_end_idx + 1, summary_line)
        lines.insert(header_end_idx + 2, "")
    new_content = "\n".join(lines)
    with open(abs_path, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"  FIXED: {filepath}")
    return True


def main():
    root_dir = "/workspaces/semio"
    fixed = 0
    skipped = 0
    for filepath, summary in SUMMARIES.items():
        result = add_summary(root_dir, filepath, summary)
        if result:
            fixed += 1
        else:
            skipped += 1
    print(f"\nDone. Fixed: {fixed}, Skipped: {skipped}")


if __name__ == "__main__":
    main()
