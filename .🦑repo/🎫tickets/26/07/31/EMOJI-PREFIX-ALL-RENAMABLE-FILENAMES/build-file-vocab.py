#!/usr/bin/env python3
"""Build file-rename-map.json: emoji-prefix every renamable basename under the four product roots."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

ROOTS = ["🧰framework", "✏️s", "🌎hub", "♻️mit-bestand"]
SKIP_DIRNAMES = {"node_modules", "target", ".git", ".repo", ".nx", "pkg", ".DS_Store", "partial_movie_files", "_vendor", "interfaces"}

EXCLUDE_PATH_PREFIXES = [
    "♻️mit-bestand/recherche/",
    "♻️mit-bestand/präsentation/33.projektetage/⚡️/🟦/slide/",
    "🧰framework/🛍️product/🦑repo/🔨module/🖥️server/🎛️coordinator/⚡️implementation/🟦typescript/app",
    "🧰framework/🔨module/🖱️ui/⚡️implementation/🔷dotnet/🎨styling/Elements.Styling",
]
EXCLUDE_SUBTREE_NAMES = {"partial_movie_files", "_vendor", "interfaces", "osm-tiles"}

NEXT_LOCKED_IN_APP = frozenset(
    {
        "page.tsx", "page.ts", "page.jsx", "page.js",
        "layout.tsx", "layout.ts", "layout.jsx", "layout.js",
        "route.ts", "route.js",
        "loading.tsx", "error.tsx", "not-found.tsx", "template.tsx", "default.tsx",
        "middleware.ts", "middleware.js",
        "globals.css",
    }
)

NEVER_RENAME_FILES = frozenset(
    {
        "Cargo.toml", "Cargo.lock", "package.json", "bun.lock", "bunfig.toml",
        "go.mod", "go.sum", "go.work", "go.work.sum", "pyproject.toml", "uv.lock",
        "project.json", "nx.json", "tsconfig.json",
        "script.ts", "AGENTS.md", "CLAUDE.md",
        "README.md", "LICENSE.md", "CHANGELOG.md", "CITATION.cff",
        "rust-toolchain.toml", "rustfmt.toml", "nextest.toml",
        "eslint.config.mjs", ".prettierrc.json", ".prettierignore",
        "CMakeLists.txt", "CMakePresets.json", "Trunk.toml", "conftest.py",
        "Monorepo.sln",
        "post-commit", "prepare-commit-msg", "post-checkout", "post-rewrite", "post-merge",
        "pre-commit", "commit-msg", "pre-push",
        "main.go", "main_test.go",
        "client", "client_bin", "server",
        ".nojekyll",
    }
)

FILE_VOCAB = {
    "build.rs": "📦build.rs",
    "vitest.config.ts": "🧪vitest.config.ts",
    "vite.config.ts": "⚙️vite.config.ts",
    "postcss.config.ts": "🎨postcss.config.ts",
    "tailwind.config.ts": "🎨tailwind.config.ts",
    "index.html": "🌐index.html",
    "index.test.ts": "🧪index.test.ts",
    "generated.rs": "🤖generated.rs",
    "generated.py": "🤖generated.py",
    "schema.graphql": "🔗schema.graphql",
    "schema.sql": "🛢️schema.sql",
    "go.work.sum": "🐹go.work.sum",
    "CNAME": "🌐CNAME",
    "Dockerfile": "🐳Dockerfile",
    "Caddyfile": "🌐Caddyfile",
    "__init__.py": "🐍__init__.py",
}

EXT_EMOJI = {
    ".svg": "🔣",
    ".png": "🖼️",
    ".jpg": "🖼️",
    ".jpeg": "🖼️",
    ".gif": "🖼️",
    ".webp": "🖼️",
    ".ico": "🖼️",
    ".glb": "🧊",
    ".gltf": "🧊",
    ".3dm": "📐",
    ".zip": "📦",
    ".bin": "📦",
    ".ttf": "🔤",
    ".woff": "🔤",
    ".woff2": "🔤",
    ".tex": "🖋️",
    ".bib": "📚",
    ".dsl": "🗣️",
    ".spk": "📦",
    ".shields": "🛡️",
    ".css": "🎨",
    ".html": "🌐",
    ".tsx": "⚛️",
    ".jsx": "⚛️",
    ".js": "🟨",
    ".ts": "🟦",
    ".go": "🐹",
    ".rs": "🦀",
    ".json": "🔣",
    ".md": "📄",
    ".mdx": "📄",
    ".sql": "🛢️",
    ".graphql": "🔗",
    ".sty": "🖋️",
    ".cypher": "🔗",
    ".toml": "📋",
    ".yaml": "📋",
    ".yml": "📋",
    ".wit": "📜",
    ".curate": "🗂️",
    ".forms": "📋",
    ".writer": "✒️",
    ".trinity": "🔱",
    ".sequence": "🎬",
    ".draw": "🖍️",
    ".note": "🗒️",
    ".layout": "📏",
    ".raster": "🖨️",
    ".imperative": "📜",
    ".manifest": "🛂",
    ".manifest.json": "🛂manifest.json",
    ".pdf": "📄",
    ".mp4": "🎥",
    ".stp": "📐",
    ".gh": "🦗",
    ".rhl": "📐",
    ".procedural2d": "🌀",
    ".procedural3d": "🌀",
    ".gismap": "🌍",
    ".gisterrain": "🌍",
    ".shooting": "🎥",
    ".fem2d": "🏗️",
    ".fem3d": "🏗️",
    ".process3d": "🏭",
    ".lowpoly": "💠",
    ".wires": "🔌",
    ".puzzle2d": "🧩",
    ".puzzle5d": "🧩",
    ".puzzle3d": "🧩",
    ".block2d": "🧱",
    ".block5d": "🧱",
    ".block3d": "🧱",
    ".s": "✏️",
    ".flow": "🌊",
    ".ops": "🔧",
    ".wasm": "🕸️",
    ".cls": "🖋️",
    ".log": "📋",
    ".vsix": "🧩",
    ".txt": "📝",
    ".mjs": "🟨",
    ".ps1": "⌨️",
    ".sh": "⌨️",
    ".cs": "🔷",
    ".py": "🐍",
    ".csv": "📋",
    ".hdr": "🖼️",
    ".dag": "🕸️",
}


def ext_key_for_name(name: str) -> str:
    lower = name.lower()
    for ext in sorted(EXT_EMOJI, key=len, reverse=True):
        if lower.endswith(ext):
            return ext
    return os.path.splitext(name)[1].lower()


def has_emoji_prefix(name: str) -> bool:
    if not name:
        return False
    return ord(name[0]) > 127


def is_excluded(relpath: str) -> bool:
    posix = relpath.replace(os.sep, "/")
    for prefix in EXCLUDE_PATH_PREFIXES:
        if posix == prefix.rstrip("/") or posix.startswith(prefix):
            return True
    parts = posix.split("/")
    if any(p in EXCLUDE_SUBTREE_NAMES for p in parts):
        return True
    return False


def next_app_locked(relpath: str, name: str) -> bool:
    posix = relpath.replace(os.sep, "/")
    if "/app/" in posix or posix.endswith("/app"):
        return name in NEXT_LOCKED_IN_APP
    return False


def csproj_sln(name: str) -> bool:
    return name.endswith(".csproj") or name.endswith(".sln")


def proposed_new_name(relpath: str, name: str):
    if name in NEVER_RENAME_FILES or csproj_sln(name):
        return None
    if name.startswith(".") or has_emoji_prefix(name):
        return None
    if next_app_locked(relpath, name):
        return None
    if name in FILE_VOCAB:
        return FILE_VOCAB[name]
    lower = name.lower()
    if lower in FILE_VOCAB:
        return FILE_VOCAB[lower]
    ext = os.path.splitext(name)[1]
    if ext:
        ext_lower = ext_key_for_name(name)
        if ext_lower in EXT_EMOJI:
            return EXT_EMOJI[ext_lower] + name
        if name.endswith(".manifest.json"):
            return "🛂manifest.json" if name == "manifest.json" else "🛂" + name
    if "." in name:
        return "📎" + name
    return "📎" + name


def collect_files():
    entries = []
    missing = []
    for root in ROOTS:
        if not os.path.isdir(root):
            continue
        for dirpath, dirs, files in os.walk(root):
            if is_excluded(dirpath):
                dirs[:] = []
                continue
            dirs[:] = [d for d in dirs if d not in SKIP_DIRNAMES and not d.startswith(".")]
            for f in files:
                if f.startswith("."):
                    continue
                full = os.path.join(dirpath, f)
                rel = full.replace(os.sep, "/")
                if is_excluded(rel):
                    continue
                new_name = proposed_new_name(rel, f)
                if new_name is None:
                    if f in NEVER_RENAME_FILES or has_emoji_prefix(f) or next_app_locked(rel, f):
                        continue
                    if csproj_sln(f):
                        continue
                    missing.append(rel)
                    continue
                if new_name == f:
                    continue
                new_rel = os.path.join(dirpath, new_name).replace(os.sep, "/")
                entries.append({"old": rel, "new": new_rel})
    return entries, missing


def main():
    entries, missing = collect_files()
    out = os.path.join(HERE, "file-rename-map.json")
    with open(out, "w", encoding="utf-8") as f:
        json.dump(entries, f, ensure_ascii=False, indent=2)
    print(f"wrote {len(entries)} renames to {out}")
    if missing:
        print(f"MISSING vocab ({len(missing)}):")
        for m in sorted(missing)[:80]:
            print(" ", m)
        if len(missing) > 80:
            print(f"  ... and {len(missing) - 80} more")
        sys.exit(1)


if __name__ == "__main__":
    main()
