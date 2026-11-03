"""🧹 Restore HEAD line endings for ticket-touched files under teaching/architecture (OI-71).

Usage: python eol_fix.py [--apply]
Without --apply it only classifies. Read-only git queries; writes via temp file and os.replace.
"""
import os
import subprocess
import sys
import tempfile

PREFIX = "\U0001F393️teaching/\U0001F3DB️architecture/"


def git(*args):
    return subprocess.run(["git", "-c", "core.quotepath=false", *args], check=True, capture_output=True).stdout


def main():
    apply = "--apply" in sys.argv
    root = git("rev-parse", "--show-toplevel").decode().strip()
    os.chdir(root)
    entries = git("status", "--porcelain", "-z").split(b"\0")
    paths = []
    i = 0
    while i < len(entries):
        e = entries[i].decode("utf-8")
        i += 1
        if len(e) < 4:
            continue
        code, path = e[:2], e[3:]
        if "R" in code or "C" in code:
            i += 1
        if path.startswith(PREFIX) and code.strip() in ("M", "MM", "AM", "A", "M "):
            paths.append(path)
    counts = {"changed": 0, "same": 0, "skipped_locked": 0, "mixed_head": 0}
    for p in sorted(paths):
        work = open(p, "rb").read()
        try:
            head = git("show", "HEAD:" + p)
        except subprocess.CalledProcessError:
            print("NOHEAD", p)
            continue
        h_crlf = head.count(b"\r\n")
        h_lf = head.count(b"\n")
        w_crlf = work.count(b"\r\n")
        if h_crlf not in (0, h_lf):
            counts["mixed_head"] += 1
            print("MIXED_HEAD", h_crlf, h_lf, p)
            continue
        kind = "head_crlf" if h_crlf else "head_lf"
        new = work.replace(b"\r\n", b"\n")
        if h_crlf:
            new = new.replace(b"\n", b"\r\n")
        if new == work:
            counts["same"] += 1
            continue
        counts["changed"] += 1
        if not apply:
            print("WOULD", kind, "work_crlf", w_crlf, "work_lf", work.count(b"\n"), p)
            continue
        d = os.path.dirname(os.path.abspath(p))
        fd, tmp = tempfile.mkstemp(dir=d, prefix=".eolfix-", suffix=".tmp")
        try:
            with os.fdopen(fd, "wb") as f:
                f.write(new)
            os.replace(tmp, p)
            print("FIXED", kind, p)
        except PermissionError:
            counts["skipped_locked"] += 1
            print("LOCKED", p)
            os.unlink(tmp)
    print("candidates", len(paths), counts)


if __name__ == "__main__":
    main()
