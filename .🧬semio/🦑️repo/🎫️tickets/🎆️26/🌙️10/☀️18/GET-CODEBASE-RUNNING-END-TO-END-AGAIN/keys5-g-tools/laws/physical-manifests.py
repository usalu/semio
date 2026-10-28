# keys5-g: independent Python reader of the physical package scan (🗂️workspaces `walk`): every directory below the root that carries a package.json, outside dot directories and the skipped build/vendor/scratch names.
# usage: python3 -I physical-manifests.py <repo root>   prints one repository-relative package.json path per line
import os, sys
SKIP = {"🗑️generated", "node_modules", "target", "dist", "build", "🤖️generated", "storybook-static", "temp", "coverage", "🔌️plugin-modules", ".🧬semio", "compose"}
root = sys.argv[1]
out = []
def walk(directory):
    try: entries = sorted(os.scandir(directory), key=lambda e: e.name.encode("utf-8"))
    except OSError: return
    for e in entries:
        if not e.is_dir(follow_symlinks=False) or e.name.startswith(".") or e.name in SKIP: continue
        if os.path.exists(os.path.join(e.path, "package.json")): out.append(os.path.relpath(os.path.join(e.path, "package.json"), root))
        walk(e.path)
walk(root)
sys.stdout.write("\n".join(sorted(out)) + "\n")
