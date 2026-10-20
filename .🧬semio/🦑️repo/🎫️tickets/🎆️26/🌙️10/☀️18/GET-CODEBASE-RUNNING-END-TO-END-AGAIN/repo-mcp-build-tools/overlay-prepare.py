"""Copies the Go module closure of the repo MCP module into an overlay root (default /private/tmp/repo-mcp-build/root), keeping the repository-relative layout so relative `replace` directives keep working.
Standard library only. Usage: python3 overlay-prepare.py [--root OVERLAY] [--repo REPO] [--fresh]
Sources of the staged set (C/repo-mcp-build-tools/staged/<repo-relative path>) are layered over the copy by overlay-stage.py."""
import argparse, os, re, shutil, sys
parser = argparse.ArgumentParser()
parser.add_argument("--repo", default=os.getcwd())
parser.add_argument("--root", default="/private/tmp/repo-mcp-build/root")
parser.add_argument("--fresh", action="store_true")
args = parser.parse_args()
repo, root = os.path.abspath(args.repo), os.path.abspath(args.root)
start = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp"
if args.fresh and os.path.isdir(root):
    shutil.rmtree(root)
seen, queue = set(), [start]
while queue:
    rel = queue.pop()
    if rel in seen:
        continue
    seen.add(rel)
    gomod = os.path.join(repo, rel, "go.mod")
    if not os.path.isfile(gomod):
        continue
    for line in open(gomod, encoding="utf-8"):
        match = re.search(r"=>\s*(\S+)", line)
        if match and match.group(1).startswith("."):
            queue.append(os.path.normpath(os.path.join(rel, match.group(1))))
ignore = shutil.ignore_patterns("node_modules", "target", "dist", ".git")
# nested modules are copied through their parents, so keep only the outermost
outer = [rel for rel in sorted(seen) if not any(rel != other and rel.startswith(other + os.sep) for other in seen)]
for rel in sorted(seen):
    destination = os.path.join(root, rel)
    if os.path.isdir(destination):
        continue
    shutil.copytree(os.path.join(repo, rel), destination, ignore=ignore, symlinks=True, dirs_exist_ok=True)
# sibling neutral module that holds fixtures read by tests (OriginPath("..", "..", "🔌️mcp", ...))
neutral = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp"
if not os.path.isdir(os.path.join(root, neutral)):
    shutil.copytree(os.path.join(repo, neutral), os.path.join(root, neutral), ignore=ignore, symlinks=True)
with open(os.path.join(root, "go.work"), "w", encoding="utf-8") as handle:
    handle.write("go 1.25\n\nuse (\n" + "".join("\t./" + rel + "\n" for rel in sorted(seen)) + ")\n")
for relative in ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json"]:
    os.makedirs(os.path.dirname(os.path.join(root, relative)), exist_ok=True)
    shutil.copy2(os.path.join(repo, relative), os.path.join(root, relative))
print("modules", len(seen), "root", root)
for rel in sorted(seen):
    print(" ", rel)
