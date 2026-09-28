"""🪞️ AV2: mirrors the working tree into the AV scratch overlay (gitignored, inherited from AV1): every tracked file plus the
gitignored generated sources a build needs (`🤖️generated/`, `🔤️tokens/`, standalone `Cargo.lock`s), copying only files whose
size or mtime differ, and removing overlay files the tree no longer has (except the slice's own authored files listed in
`payload/manifest.json` "new" and anything under build/target dirs). The slice's EDITED files (manifest "edited") are never
overwritten; the sync lists the ones whose live copy drifted from the patch base (`BASE`) so they can be re-based.

Usage: python3 av2-overlay-sync.py [--dry-run]
"""
import json, os, shutil, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
DEST = os.path.join(ROOT, ".🧬semio/🌐hub/s13-av1-overlay")
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = os.path.join(ROOT, ".🧬semio/🌐hub/s14b-av2-base")
KEEP_IGNORED = ("🤖️generated/", "🔤️tokens/", "Cargo.lock")
SKIP_IGNORED = ("node_modules/", "/target/", "🧑‍💻dev/🔌️plugin-modules", "🧑‍💻dev/📤️distribution", "🧑‍💻dev/🧩️extension-modules", ".semio-", "/pkg/", "__pycache__", "/bin/", "/obj/", "/dist/", ".🧬semio/", ".tmp-ticket")
SKIP_WALK = {"target", "node_modules", ".git", ".nx", "dist", ".🧬semio", ".tmp-ticket"}


def copy(rel: str) -> int:
    src = os.path.join(ROOT, rel)
    dst = os.path.join(DEST, rel)
    try:
        st = os.lstat(src)
    except FileNotFoundError:
        return 0
    if os.path.isdir(src) and not os.path.islink(src):
        return 0
    try:
        dt = os.lstat(dst)
        if dt.st_size == st.st_size and int(dt.st_mtime) == int(st.st_mtime):
            return 0
    except FileNotFoundError:
        pass
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src):
        if os.path.lexists(dst):
            os.remove(dst)
        os.symlink(os.readlink(src), dst)
    else:
        shutil.copy2(src, dst)
    return 1


def ignored_sources() -> list:
    entries = subprocess.run(["git", "ls-files", "-z", "-o", "-i", "--exclude-standard", "--directory"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8").split("\0")
    kept = []
    for entry in entries:
        if not entry or any(s in entry for s in SKIP_IGNORED) or entry.startswith("."):
            continue
        if not entry.endswith("/"):
            if any(k in entry for k in KEEP_IGNORED):
                kept.append(entry)
            continue
        if not any(k in entry for k in KEEP_IGNORED[:2]):
            continue
        for directory, dirs, files in os.walk(os.path.join(ROOT, entry)):
            dirs[:] = [d for d in dirs if d not in SKIP_WALK]
            kept.extend(os.path.relpath(os.path.join(directory, name), ROOT) for name in files if name != ".DS_Store")
    return kept


def main() -> None:
    dry = "--dry-run" in sys.argv
    tracked = [f for f in subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8").split("\0") if f]
    ignored = ignored_sources()
    wanted = set(tracked) | set(ignored)
    manifest = os.path.join(HERE, "payload", "manifest.json")
    payload = json.load(open(manifest, encoding="utf-8")) if os.path.exists(manifest) else {"new": [], "edited": []}
    authored = set(payload["new"])
    edited = set(payload["edited"])
    drifted = [rel for rel in sorted(edited) if not os.path.exists(os.path.join(BASE, rel)) or open(os.path.join(ROOT, rel), "rb").read() != open(os.path.join(BASE, rel), "rb").read()]
    wanted -= edited
    stale = []
    for directory, dirs, files in os.walk(DEST):
        dirs[:] = [d for d in dirs if d not in SKIP_WALK]
        for name in files:
            rel = os.path.relpath(os.path.join(directory, name), DEST)
            if rel not in wanted and rel not in authored and rel not in edited and not os.path.exists(os.path.join(ROOT, rel)):
                stale.append(rel)
    if dry:
        print(f"tracked={len(tracked)} ignored_sources={len(ignored)} stale={len(stale)} (dry run)")
        for rel in stale[:40]:
            print(f"  stale {rel}")
        return
    with ThreadPoolExecutor(max_workers=8) as pool:
        copied = sum(pool.map(copy, sorted(wanted)))
    for rel in stale:
        os.remove(os.path.join(DEST, rel))
    print(f"tracked={len(tracked)} ignored_sources={len(ignored)} copied={copied} removed_stale={len(stale)} edited_kept={len(edited)} drifted={len(drifted)}")
    for rel in drifted:
        print(f"  drifted {rel}")


if __name__ == "__main__":
    main()
