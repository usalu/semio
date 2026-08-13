#!/usr/bin/env python3
"""One-off wave-A0 scaffold: every window gains the 🎚️config + 🫧️transient lanes, every mode gains
its four declared children, each marked with the repo's tracked `📌️empty.md` when genuinely empty.
Lives inside the ticket folder (never a permanent script) per CLAUDE.md."""
import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
TAXONOMY = json.load(open(os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), encoding="utf-8"))

WINDOWS_DIR = TAXONOMY["windowsDirName"]
MODES_DIR = TAXONOMY["modesDirName"]
APPS_DIR = TAXONOMY["appsDirName"]
MARKER = TAXONOMY["windowEmptyFacetFilename"]
WINDOW_REQUIRED = TAXONOMY["windowRequiredChildDirs"]
MODE_REQUIRED = TAXONOMY["modeChildDirs"]
LEAF_NAMES = set(TAXONOMY["taxonomyLeafFilenames"].values())

WINDOW_MARKER = "# Empty Window Facet\n\nThis facet currently declares no specific items.\n"
MODE_MARKER = "# Empty Mode Facet\n\nThis facet currently declares no specific items.\n"

SKIP_DIRS = {".git", "node_modules", "target", "dist", ".nx", ".🦑️repo"}


def app_roots():
    for area in ("✏️s", "🧰️framework"):
        base = os.path.join(ROOT, area)
        for cur, dirs, _ in os.walk(base):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
            if os.path.basename(cur) != APPS_DIR:
                continue
            for name in sorted(os.listdir(cur)):
                path = os.path.join(cur, name)
                if os.path.isdir(path):
                    yield path


def subdirs(path):
    if not os.path.isdir(path):
        return []
    return sorted(p for p in (os.path.join(path, n) for n in os.listdir(path)) if os.path.isdir(p))


def ensure(facet_dir, marker_text):
    """Create `facet_dir` when absent and drop the tracked marker when it holds no specific items."""
    created_dir = not os.path.isdir(facet_dir)
    os.makedirs(facet_dir, exist_ok=True)
    marker = os.path.join(facet_dir, MARKER)
    has_members = any(os.path.isdir(p) for p in (os.path.join(facet_dir, n) for n in os.listdir(facet_dir)))
    created_marker = False
    if not has_members and not os.path.exists(marker):
        with open(marker, "w", encoding="utf-8") as fh:
            fh.write(marker_text)
        created_marker = True
    return created_dir, created_marker


def all_window_dirs(app_root):
    """Windows live at `<app>/🪟️windows/<w>` and at `<app>/🎭️modes/<m>/🪟️windows/<w>`."""
    for cur, dirs, _ in os.walk(app_root):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if os.path.basename(cur) == WINDOWS_DIR:
            for window in subdirs(cur):
                yield window


stats = {"window_dirs": 0, "window_markers": 0, "mode_dirs": 0, "mode_markers": 0, "windows": 0, "modes": 0}
for app in app_roots():
    for window in all_window_dirs(app):
        stats["windows"] += 1
        for required in WINDOW_REQUIRED:
            made_dir, made_marker = ensure(os.path.join(window, required), WINDOW_MARKER)
            stats["window_dirs"] += made_dir
            stats["window_markers"] += made_marker
    for mode in subdirs(os.path.join(app, MODES_DIR)):
        stats["modes"] += 1
        for required in MODE_REQUIRED:
            made_dir, made_marker = ensure(os.path.join(mode, required), MODE_MARKER)
            stats["mode_dirs"] += made_dir
            stats["mode_markers"] += made_marker

for key, value in stats.items():
    print(f"{key}: {value}")
