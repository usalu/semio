#!/usr/bin/env python3
"""Sweep remaining non-crate siblings using STRUCTURAL-only emoji translation.
Renames only the well-known structural segments; leaves plugin/module/app names,
data segment names, and language dirs (rs/js/py/go/net/rb) as words for now."""
import os, re, shutil, sys

RULES = [
    (re.compile(r'^framework/product/os/module/renderer/wgpu\b'), '🧰/🛍️/💻/🔨/renderer/⚡️/🦀/🧑‍🎨/🧊'),
    (re.compile(r'^framework/product/os/module/renderer/js/react\b'), '🧰/🛍️/💻/🔨/renderer/⚡️/🟦/🧑‍🎨/⚛️'),
    (re.compile(r'^framework/product/os/module/renderer\b'), '🧰/🛍️/💻/🔨/renderer'),
    (re.compile(r'^framework/product/os/module/plugin\b'), '🧰/🛍️/💻/🔨/plugin'),
    (re.compile(r'^framework/product/os/module/([^/]+)\b'), r'🧰/🛍️/💻/🔨/\1'),
    (re.compile(r'^framework/product/os\b'), '🧰/🛍️/💻'),
    (re.compile(r'^framework/product/print\b'), '🧰/🛍️/📓'),
    (re.compile(r'^framework/product\b'), '🧰/🛍️'),
    (re.compile(r'^framework/module/ui/js/react\b'), '🧰/🔨/ui/⚡️/🟦/react'),
    (re.compile(r'^framework/module/ui/asset\b'), '🧰/🔨/ui/asset'),
    (re.compile(r'^framework/module/ui/(styling|tui|wgpu)\b'), r'🧰/🔨/ui/⚡️/🦀/\1'),
    (re.compile(r'^framework/module/surface/([^/]+)\b'), r'🧰/🔨/surface/⚡️/🦀/\1'),
    (re.compile(r'^framework/module/math/([^/]+)\b'), r'🧰/🔨/math/⚡️/🦀/\1'),
    (re.compile(r'^framework/module/([^/]+)\b'), r'🧰/🔨/\1'),
    (re.compile(r'^framework/os\b'), '🧰/os'),  # stray leftover stub dir
    (re.compile(r'^framework\b'), '🧰'),
    (re.compile(r'^s/plugin/([^/]+)/manifest/artifact\b'), r'✏️/🔌/\1/🛂/🗿'),
    (re.compile(r'^s/plugin/([^/]+)/manifest\b'), r'✏️/🔌/\1/🛂'),
    (re.compile(r'^s/plugin/([^/]+)/extension/([^/]+)\b'), r'✏️/🔌/\1/🧩/\2'),
    (re.compile(r'^s/plugin/([^/]+)/module/([^/]+)/([^/]+)\b'), r'✏️/🔌/\1/🔨/\2/⚡️/🦀/\3'),
    (re.compile(r'^s/plugin/([^/]+)/module/([^/]+)\b'), r'✏️/🔌/\1/🔨/\2'),
    (re.compile(r'^s/plugin/draw/module/fsm/macros\b'), '✏️/🔌/draw/🔨/fsm/⚡️/🦀/macros'),
    (re.compile(r'^s/plugin/([^/]+)/app/([^/]+)/(engine|dsl|op|pack|protocol|ui)\b'), r'✏️/🔌/\1/🎛️/\2/🔨/\3'),
    (re.compile(r'^s/plugin/([^/]+)/app/([^/]+)\b'), r'✏️/🔌/\1/🎛️/\2'),
    (re.compile(r'^s/plugin/([^/]+)\b'), r'✏️/🔌/\1'),
    (re.compile(r'^s/module/([^/]+)\b'), r'✏️/🔨/\1'),
    (re.compile(r'^s\b'), '✏️'),
    (re.compile(r'^hub/directory\b'), '🌎/🔨/directory'),
    (re.compile(r'^hub\b'), '🌎'),
    (re.compile(r'^mit-bestand\b'), '♻️'),
    (re.compile(r'^repo/cli\b'), '🧰/🛍️/🦑/🔨/cli'),
    (re.compile(r'^repo/lib\b'), '🧰/🛍️/🦑/🔨/lib'),
    (re.compile(r'^repo/client\b'), '🧰/🛍️/🦑/🔨/client'),
    (re.compile(r'^repo/server\b'), '🧰/🛍️/🦑/🔨/server'),
    (re.compile(r'^repo/native\b'), '🧰/🛍️/🦑/🔨/native'),
    (re.compile(r'^repo/asset\b'), '🧰/🛍️/🦑/asset'),
    (re.compile(r'^repo/hooks\b'), '🧰/🛍️/🦑/hooks'),
    (re.compile(r'^repo/hook\b'), '🧰/🛍️/🦑/hook'),
    (re.compile(r'^repo\b'), '🧰/🛍️/🦑'),
]

def translate(path):
    for pattern, repl in RULES:
        new_path, n = pattern.subn(repl, path, count=1)
        if n:
            return new_path
    return None

def merge_move(old, new):
    if not os.path.exists(old):
        return
    if not os.path.lexists(new):
        os.makedirs(os.path.dirname(new), exist_ok=True)
        shutil.move(old, new)
        return
    if os.path.isdir(old) and os.path.isdir(new):
        for entry in os.listdir(old):
            merge_move(os.path.join(old, entry), os.path.join(new, entry))
        if not os.listdir(old):
            os.rmdir(old)
    else:
        print(f"COLLISION (file vs file/dir): {old} -> {new}")

roots = sys.argv[1:] if len(sys.argv) > 1 else ['framework', 's', 'hub', 'repo', 'mit-bestand']

moved = 0
skipped = []
for root in roots:
    if not os.path.isdir(root):
        continue
    # collect all top-level-ish paths first (bottom-up doesn't matter since merge_move handles both)
    for dirpath, dirnames, filenames in os.walk(root, topdown=True):
        # skip already-emoji'd paths (shouldn't occur under these roots but just in case)
        for name in filenames + dirnames:
            full = os.path.join(dirpath, name)
        # only process at each level once: try translating dirpath itself before descending
    # Simpler: walk bottom-up over all paths, try translating each existing top-level child chain
    for dirpath, dirnames, filenames in os.walk(root, topdown=False):
        for name in filenames:
            full = os.path.join(dirpath, name)
            new = translate(full)
            if new is None:
                skipped.append(full)
                continue
            merge_move(full, new)
            moved += 1
        # after files handled, try to remove now-empty dirs, or move whole dir if translatable and now fully empty of untranslatable content
    # cleanup empty dirs bottom-up
    for dirpath, dirnames, filenames in os.walk(root, topdown=False):
        try:
            if not os.listdir(dirpath):
                os.rmdir(dirpath)
        except FileNotFoundError:
            pass

print(f"moved: {moved}")
print(f"skipped (no rule matched): {len(skipped)}")
for s in skipped[:60]:
    print(" ", s)
