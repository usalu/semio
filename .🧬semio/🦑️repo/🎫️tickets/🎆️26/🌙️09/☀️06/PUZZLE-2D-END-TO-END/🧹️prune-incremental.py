#!/usr/bin/env python3
"""🧹️ Frees disk under a running fleet: deletes cargo incremental SESSION dirs whose `s-*.lock` no
process holds (flock probe) and that are older than --minutes (default 10). Live peer builds keep their
lock, so they are never touched. Usage: python3 🧹️prune-incremental.py [--minutes 10] [--dry-run]."""
import argparse, fcntl, os, shutil, sys, time

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
BASE = os.path.join(ROOT, ".🧬semio/🦑️repo/⚡️cache/cargo/build")
ROOTS = ["debug/incremental", "wasm32-wasip2/wasm-dev/incremental", "wasm32-wasip2/debug/incremental", "wasm32-unknown-unknown/debug/incremental", "release/incremental"]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--minutes", type=float, default=10.0)
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    now = time.time()
    freed = locked = kept = 0
    for relative in ROOTS:
        root = os.path.join(BASE, relative)
        if not os.path.isdir(root):
            continue
        for crate in os.listdir(root):
            cdir = os.path.join(root, crate)
            if not os.path.isdir(cdir):
                continue
            for entry in os.listdir(cdir):
                if not entry.endswith(".lock"):
                    continue
                stem = entry[:-5]
                dirs = [os.path.join(cdir, d) for d in os.listdir(cdir) if d.startswith(stem) and os.path.isdir(os.path.join(cdir, d))]
                if not dirs:
                    continue
                lock = os.path.join(cdir, entry)
                try:
                    fd = os.open(lock, os.O_RDWR)
                except OSError:
                    continue
                try:
                    fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except OSError:
                    locked += 1
                    os.close(fd)
                    continue
                age = now - max(os.path.getmtime(d) for d in dirs)
                if age < args.minutes * 60:
                    kept += 1
                else:
                    for d in dirs:
                        size = sum(os.path.getsize(os.path.join(dp, f)) for dp, _, fn in os.walk(d) for f in fn if os.path.exists(os.path.join(dp, f)))
                        if not args.dry_run:
                            shutil.rmtree(d, ignore_errors=True)
                        freed += size
                    if not args.dry_run:
                        try:
                            os.remove(lock)
                        except OSError:
                            pass
                fcntl.flock(fd, fcntl.LOCK_UN)
                os.close(fd)
    print(f"{'would free' if args.dry_run else 'freed'} {freed / 2**30:.1f} GB · locked (live, skipped) {locked} · recent (kept) {kept}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
