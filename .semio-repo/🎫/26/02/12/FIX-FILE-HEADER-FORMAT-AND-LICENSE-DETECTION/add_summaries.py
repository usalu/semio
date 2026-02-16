#!/usr/bin/env python3
"""Add missing summary comments to file headers."""
import os
import re

SUMMARIES = {
    "semio/assets/icons.ts": "Re-exports Lucide React icons with domain-specific semantic aliases.",
    "semio/assets/index.ts": "Barrel export for all asset modules including icons, fonts, models and images.",
    "semio/assets/logo/logo.ts": "Generates animated SVG logo from static SVG input with keyframe sequences.",
    "semio/desktop/forge.config.ts": "Electron Forge configuration for building and packaging the desktop app.",
    "semio/desktop/forge.env.d.ts": "Type declarations for Electron Forge environment variables.",
    "semio/desktop/main.ts": "Entry point for the Electron main process managing windows and lifecycle.",
    "semio/desktop/postcss.config.ts": "PostCSS configuration for the desktop app with Tailwind and autoprefixer.",
    "semio/desktop/preload.ts": "Electron preload script exposing safe APIs to the renderer process.",
    "semio/desktop/renderer.tsx": "Entry point for the Electron renderer process mounting the React app.",
    "semio/desktop/tailwind.config.ts": "Tailwind CSS configuration for the desktop app styling.",
    "semio/desktop/vite.main.config.ts": "Vite build configuration for the Electron main process.",
    "semio/desktop/vite.preload.config.ts": "Vite build configuration for the Electron preload script.",
    "semio/desktop/vite.renderer.config.ts": "Vite build configuration for the Electron renderer process.",
    "semio/docs/index.tsx": "Entry point for the documentation site React app.",
    "semio/docs/postcss.config.ts": "PostCSS configuration for the docs site with Tailwind and autoprefixer.",
    "semio/docs/tailwind.config.ts": "Tailwind CSS configuration for the documentation site styling.",
    "semio/engine/build.ts": "Build script for the semio engine Python package.",
    "semio/engine/generate-schemas.ts": "Generates JSON schemas from the engine's Python models.",
    "semio/engine/post-build.ts": "Post-build script for engine artifact processing and packaging.",
    "semio/engine/sqliteschema.ts": "Exports the SQLite schema definition for the engine database.",
    "semio/gh/Semio.Grasshopper/build.ts": "Build script for the Grasshopper plugin assembly.",
    "semio/gh/Semio.Grasshopper/build-value-lists.ts": "Generates Grasshopper value list presets from domain data.",
    "semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs": "Main Grasshopper plugin providing domain components for Rhino.",
    "semio/gh/Semio.Grasshopper/yak/build.ts": "Build script for Yak package distribution of the Grasshopper plugin.",
    "semio/gh/Semio.Grasshopper/yak/login.ts": "Authenticates with the Yak package server for plugin publishing.",
    "semio/gh/Semio.Grasshopper/yak/publish.ts": "Publishes the Grasshopper plugin package to the Yak server.",
    "semio/gh/Semio.Grasshopper/yak/test-push.ts": "Tests the Yak package push workflow for the Grasshopper plugin.",
    "semio/gh/Semio.Grasshopper/yak/test-search.ts": "Tests Yak package search functionality for the Grasshopper plugin.",
    "semio/gh/Semio.Grasshopper/yak/unyank.ts": "Restores a previously yanked version of the Grasshopper Yak package.",
    "semio/gh/Semio.Grasshopper/yak/yank.ts": "Yanks a specific version of the Grasshopper Yak package from the registry.",
    "semio/go/kit_sqlite.go": "SQLite-backed persistence layer for kit import and export operations.",
    "semio/go/semio.go": "Core domain library in Go implementing the semio data model and operations.",
    "semio/js/dev.ts": "Development server entry point for the JavaScript workspace.",
    "semio/js/eslint.config.ts": "ESLint configuration for the JavaScript workspace linting rules.",
    "semio/js/global.d.ts": "Global type declarations for the JavaScript workspace.",
    "semio/js/i18n.ts": "Internationalization setup and translation utilities for the UI.",
    "semio/js/index.ts": "Barrel export for the core JavaScript workspace modules.",
    "semio/jsonschema/build.ts": "Build script for generating and exporting JSON Schema definitions.",
    "semio/js/playwright.config.ts": "Playwright end-to-end test configuration for the JavaScript workspace.",
    "semio/js/postcss.config.ts": "PostCSS configuration for the JavaScript workspace with Tailwind.",
    "semio/js/semio.ts": "Core domain model types, schemas and utilities for the semio platform.",
    "semio/js/site.tsx": "Landing page and marketing site React component.",
    "semio/js/sketchpad/apps/index.ts": "Barrel export for all sketchpad app components.",
    "semio/js/sketchpad/Design.tsx": "Design app providing diagram and scene windows for editing designs.",
    "semio/js/sketchpad/Docs.tsx": "Documentation viewer app with workbench and detail panels.",
    "semio/js/sketchpad/elements.tsx": "Shared UI elements and primitive components for sketchpad apps.",
    "semio/js/sketchpad/Feedback.tsx": "Feedback collection app with rating hooks and submission forms.",
    "semio/js/sketchpad/Home.tsx": "Home screen app showing recent projects and getting started content.",
    "semio/js/sketchpad/kitSelectionHelper.ts": "Geometry and selection utilities for kit diagram interactions.",
    "semio/js/sketchpad/Kit.tsx": "Kit editor app for managing types, designs and qualities.",
    "semio/js/sketchpad/portColor.ts": "Color mapping utilities for port visualization in diagrams.",
    "semio/js/sketchpad/Quality.tsx": "Quality inspection app for viewing and editing quality attributes.",
    "semio/js/sketchpad/shared.ts": "Shared state management types, hooks and store factories for sketchpad.",
    "semio/js/sketchpad/Sketchpad.tsx": "Main sketchpad container managing app tabs, panels and window layout.",
    "semio/js/sketchpad/Tutorials.tsx": "Interactive tutorial system with step-by-step guided workflows.",
    "semio/js/sketchpad/Type.tsx": "Type editor app for defining and editing type properties and ports.",
    "semio/js/tailwind.config.ts": "Tailwind CSS configuration for the JavaScript workspace styling.",
    "semio/js/vite.config.ts": "Vite build and development configuration for the JavaScript workspace.",
    "semio/js/vite-env.d.ts": "Vite client type declarations for the JavaScript workspace.",
    "semio/net/Semio/build.ts": "Build script for the Semio .NET library assembly.",
    "semio/net/Semio/Semio.cs": "Core .NET library implementing the semio domain model and serialization.",
    "semio/play/index.tsx": "Entry point for the playground React app for interactive experimentation.",
    "semio/play/postcss.config.ts": "PostCSS configuration for the playground app with Tailwind.",
    "semio/play/tailwind.config.ts": "Tailwind CSS configuration for the playground app styling.",
    "semio/play/vite.config.ts": "Vite build and development configuration for the playground app.",
    "semio-repo/cli/main.go": "Monorepo CLI tool for repository management, analysis and code generation.",
    "semio-repo/server/main.go": "GraphQL server for the monorepo management API.",
    "semio-repo/vscode/codegen.ts": "Code generation script for VS Code extension GraphQL types.",
    "semio-repo/vscode/eslint.config.ts": "ESLint configuration for the VS Code extension linting rules.",
    "semio-repo/vscode/extension.ts": "VS Code extension providing monorepo navigation, analysis and commands.",
    "semio-repo/vscode/queries.ts": "GraphQL query document constants for the VS Code extension.",
    "semio-repo/vscode/vite.config.ts": "Vite build configuration for the VS Code extension bundling.",
    "semio-repo/vscode/vite.test.config.ts": "Vite test configuration for the VS Code extension test runner.",
    "semio/sketchpad/index.tsx": "Entry point for the standalone sketchpad web application.",
    "semio/sketchpad/postcss.config.ts": "PostCSS configuration for the sketchpad app with Tailwind.",
    "semio/sketchpad/tailwind.config.ts": "Tailwind CSS configuration for the sketchpad app styling.",
    "semio/sketchpad/vite.config.ts": "Vite build and development configuration for the sketchpad app.",
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
