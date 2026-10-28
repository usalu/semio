"""🪞 Builds a graph-free mirror of the repository path to one directory: every ancestor level is a real directory whose other entries are symlinks to the real tree, the target directory is a real copy, so one staged file can be tried without touching the library.
usage: mirror.py <mirror-root> <repo-relative directory to copy for real> [<repo-relative directory to copy for real> ...]"""
import os, shutil, sys

ROOT = os.path.realpath(os.path.join(os.path.dirname(__file__), *[".."] * 8))
mirror, targets = sys.argv[1], sys.argv[2:]
shutil.rmtree(mirror, ignore_errors=True)


def build(relative: str, real: bool) -> None:
    parts = relative.split("/")
    for depth in range(len(parts)):
        here, keep = "/".join(parts[:depth]), parts[depth]
        directory = os.path.join(mirror, here) if here else mirror
        os.makedirs(directory, exist_ok=True)
        for name in os.listdir(os.path.join(ROOT, here) if here else ROOT):
            if name == keep:
                continue
            link = os.path.join(directory, name)
            if not os.path.lexists(link) and not os.path.isdir(link):
                os.symlink(os.path.join(ROOT, here, name), link)
    destination = os.path.join(mirror, relative)
    if os.path.lexists(destination) and not os.path.isdir(destination):
        os.remove(destination)
    if not os.path.isdir(destination):
        shutil.copytree(os.path.join(ROOT, relative), destination, symlinks=True)


for target in targets:
    build(target, True)
print(mirror)
