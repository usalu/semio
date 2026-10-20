#!/usr/bin/env python3
"""Builds the documentation-law overlay: a mirror of the checkout made of symlinks, in which every directory on the way to a file that a law reads
through no-follow ancestry is a REAL directory and the file itself a real copy (APFS clone where available). Read only on the checkout.
Usage: overlay.py ROOT OVERLAY [extra relative path ...]"""
import json, os, shutil, subprocess, sys

root, overlay = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
extra = sys.argv[3:]
PRODUCT = "🧰️framework/🛍️products/🦑️repo"
LIB = PRODUCT + "/🔨️modules/📚️library"
POLICY = os.path.join(root, LIB, "🖼️assets/🏠️owner-projection-policy/🔣️.json")


def clone(source, target):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if os.path.lexists(target):
        os.remove(target)
    if subprocess.run(["cp", "-c", source, target], capture_output=True).returncode != 0:
        shutil.copy2(source, target)


def realize(path):
    """Makes every ancestor of `path` a real directory of symlinks (siblings stay symlinks) and returns the overlay path."""
    current_real, current_over = root, overlay
    parts = path.split("/")
    for part in parts[:-1]:
        current_real, current_over = os.path.join(current_real, part), os.path.join(current_over, part)
        if os.path.islink(current_over):
            os.remove(current_over)
            os.makedirs(current_over)
            for name in os.listdir(current_real):
                os.symlink(os.path.join(current_real, name), os.path.join(current_over, name))
        elif not os.path.exists(current_over):
            os.makedirs(current_over)
    return os.path.join(overlay, path)


shutil.rmtree(overlay, ignore_errors=True)
os.makedirs(overlay)
for name in os.listdir(root):
    if name == ".git":
        continue
    source = os.path.join(root, name)
    if os.path.isfile(source) and not os.path.islink(source):
        clone(source, os.path.join(overlay, name))
    else:
        os.symlink(source, os.path.join(overlay, name))
needed = [LIB]
entries = json.load(open(POLICY, encoding="utf8"))["entries"]
historical = json.load(open(os.path.join(root, LIB, "🧪️tests/🧫️fixtures/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json"), encoding="utf8"))["documents"]
readme_fixture = json.load(open(os.path.join(root, LIB, "🧪️tests/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json"), encoding="utf8"))
readme_vector = readme_fixture["documents"]
files = [row["target"] for row in readme_fixture["inlineCoordinates"] if row["target"]] + list(readme_vector.values()) + [e["sourcePath"] for e in entries] + [d["contract"]["path"] for d in historical] + ["✏️s/🧑‍💻dev/📋️project.json"] + extra
for path in files:
    if os.path.isfile(os.path.join(root, path)):
        target = realize(path)
        clone(os.path.join(root, path), target)
for extra_dir in ["✏️s/" + next(n for n in os.listdir(os.path.join(root, "✏️s")) if n.endswith("dev") and n != "🤖️generated")]:
    if os.path.isdir(os.path.join(root, extra_dir)):
        target = realize(extra_dir)
        if os.path.lexists(target):
            os.remove(target) if os.path.islink(target) else shutil.rmtree(target)
        subprocess.run(["cp", "-cR", os.path.join(root, extra_dir), target], check=True)
realize(LIB)
library_over = os.path.join(overlay, LIB)
if os.path.islink(library_over):
    os.remove(library_over)
elif os.path.isdir(library_over):
    shutil.rmtree(library_over)
subprocess.run(["cp", "-cR", os.path.join(root, LIB), library_over], check=True)
for name in os.listdir(os.path.join(overlay, PRODUCT)):
    source = os.path.join(root, PRODUCT, name)
    target = os.path.join(overlay, PRODUCT, name)
    if os.path.islink(target) and os.path.isfile(source) and not os.path.islink(source):
        os.remove(target)
        clone(source, target)
print("overlay ready:", overlay)
env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null", GIT_AUTHOR_NAME="overlay", GIT_AUTHOR_EMAIL="overlay@invalid", GIT_COMMITTER_NAME="overlay", GIT_COMMITTER_EMAIL="overlay@invalid")
for command in (["git", "init", "-q"], ["git", "add", "-A"], ["git", "commit", "-q", "-m", "overlay"]):
    subprocess.run(command, cwd=overlay, env=env, check=True, capture_output=True)
print("overlay has its own private git repository (the checkout's repository is never touched)")
