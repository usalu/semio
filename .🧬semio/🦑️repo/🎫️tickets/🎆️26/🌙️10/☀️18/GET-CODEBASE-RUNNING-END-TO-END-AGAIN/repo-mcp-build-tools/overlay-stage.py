"""Layers staged/ over the overlay copy (default /private/tmp/repo-mcp-build/root): copies every staged file to its repository path inside the overlay. Standard library only."""
import argparse, os, shutil
parser = argparse.ArgumentParser()
parser.add_argument("--root", default="/private/tmp/repo-mcp-build/root")
args = parser.parse_args()
staged = os.path.join(os.path.dirname(os.path.abspath(__file__)), "staged")
count = 0
for folder, _, names in os.walk(staged):
    for name in names:
        source = os.path.join(folder, name)
        destination = os.path.join(args.root, os.path.relpath(source, staged))
        os.makedirs(os.path.dirname(destination), exist_ok=True)
        shutil.copy2(source, destination)
        count += 1
print("layered", count, "files over", args.root)
