"""🔀️ AV2: re-bases the slice's edited overlay files onto the live tree after peers changed them. For every manifest "edited"
file whose live copy differs from the patch base (`s14b-av2-base`): 3-way merge (`git merge-file -p`, read-only on the repo)
of overlay (AV2) ⟵ base ⟶ live into the overlay, then the base becomes the live copy. Conflicts stay marked in the overlay
and are listed; resolve them before `av2-patch.py make`.

Usage: python3 av2-rebase.py
"""
import json, os, shutil, subprocess

ROOT = "/Users/ueli/Documents/semio"
HUB = os.path.join(ROOT, ".🧬semio/🌐hub")
OVERLAY = os.path.join(HUB, "s13-av1-overlay")
BASE = os.path.join(HUB, "s14b-av2-base")
HERE = os.path.dirname(os.path.abspath(__file__))


def main() -> int:
    manifest = json.load(open(os.path.join(HERE, "payload", "manifest.json"), encoding="utf-8"))
    conflicts = 0
    for rel in manifest["edited"]:
        live, base, overlay = os.path.join(ROOT, rel), os.path.join(BASE, rel), os.path.join(OVERLAY, rel)
        if open(live, "rb").read() == open(base, "rb").read():
            continue
        merged = subprocess.run(["git", "merge-file", "-p", "-L", "av2", "-L", "base", "-L", "live", overlay, base, live], capture_output=True)
        open(overlay, "wb").write(merged.stdout)
        shutil.copy2(live, base)
        conflicts += max(merged.returncode, 0)
        print(f"[av2-rebase] {merged.returncode:3d} conflict(s) {rel}")
    print(f"[av2-rebase] conflicts={conflicts}")
    return 1 if conflicts else 0


if __name__ == "__main__":
    raise SystemExit(main())
