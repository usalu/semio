#!/usr/bin/env python3
"""Builds a verification overlay: a symlink mirror of the checkout whose chain down to the Bazel adapter is made of real
directories, so new files land there without touching the checkout. Copies only `.github`, `package.json` and the Bazel
operations directory (the one an edit changes).

  python3 mkoverlay.py [--root <checkout>] [--out /private/tmp/ci-bazel/ov]
"""
import argparse, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
VS16 = "️"


def find(path, name):
    hits = [n for n in os.listdir(path) if n.replace(VS16, "") == name.replace(VS16, "")]
    if len(hits) != 1:
        raise SystemExit(f"cannot resolve {name!r} in {path}")
    return hits[0]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=os.path.abspath(os.path.join(HERE, *[".."] * 8)))
    parser.add_argument("--out", default="/private/tmp/ci-bazel/ov")
    args = parser.parse_args()
    if os.path.exists(args.out):
        shutil.rmtree(args.out)
    chain = ["🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏗️build", "🔧️tools", "🏗️bazel"]
    os.makedirs(args.out)
    real, mirror = args.root, args.out
    for entry in os.listdir(real):
        if entry in (".github", "package.json"):
            continue
        if entry.replace(VS16, "") == chain[0].replace(VS16, ""):
            continue
        os.symlink(os.path.join(real, entry), os.path.join(mirror, entry))
    shutil.copytree(os.path.join(real, ".github"), os.path.join(mirror, ".github"))
    shutil.copy2(os.path.join(real, "package.json"), os.path.join(mirror, "package.json"))
    for index, name in enumerate(chain):
        actual = find(real, name)
        real, mirror = os.path.join(real, actual), os.path.join(mirror, actual)
        os.makedirs(mirror)
        nxt = chain[index + 1] if index + 1 < len(chain) else None
        for entry in os.listdir(real):
            if nxt is not None and entry.replace(VS16, "") == nxt.replace(VS16, ""):
                continue
            if nxt is None and entry.replace(VS16, "") == "⚙️operations".replace(VS16, ""):
                shutil.copytree(os.path.join(real, entry), os.path.join(mirror, entry))
                continue
            os.symlink(os.path.join(real, entry), os.path.join(mirror, entry))
    print("overlay", args.out)


if __name__ == "__main__":
    sys.exit(main())
