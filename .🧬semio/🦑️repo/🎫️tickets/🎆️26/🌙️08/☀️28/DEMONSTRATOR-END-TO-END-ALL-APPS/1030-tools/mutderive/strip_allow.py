#!/usr/bin/env python3
import os, subprocess, sys, tempfile

LINE = "#![allow(long_running_const_eval)]"
TS = "\\n#![allow(long_running_const_eval)]"
TICKETS = ".🧬semio/🦑️repo/🎫️tickets/"


def files(root):
    tracked = subprocess.check_output(["git", "-C", root, "ls-files", "-z"]).decode().split("\0")
    fresh = subprocess.check_output(["git", "-C", root, "ls-files", "-z", "-o", "--exclude-standard"]).decode().split("\0")
    return sorted({f for f in tracked + fresh if f and not f.startswith(TICKETS)})


def strip_rust(text):
    kept = []
    drop_blank = False
    changed = False
    for line in text.split("\n"):
        if line.strip() == LINE:
            drop_blank = bool(kept) and kept[-1].strip() == ""
            changed = True
            continue
        if drop_blank and line.strip() == "":
            drop_blank = False
            continue
        drop_blank = False
        kept.append(line)
    return "\n".join(kept) if changed else text


def rewrite(path, text):
    if path.endswith(".rs"):
        return strip_rust(text)
    if path.endswith(".ts"):
        return text.replace(TS, "")
    return text


def main():
    root = os.getcwd()
    apply = "--apply" in sys.argv
    changed = []
    for rel in files(root):
        if not rel.endswith((".rs", ".ts")):
            continue
        try:
            with open(rel, encoding="utf-8") as handle:
                text = handle.read()
        except (OSError, UnicodeDecodeError):
            continue
        if "long_running_const_eval" not in text:
            continue
        updated = rewrite(rel, text)
        if updated == text:
            print("UNHANDLED", rel)
            continue
        changed.append(rel)
        if apply:
            fd, tmp = tempfile.mkstemp(dir=os.path.dirname(rel) or ".")
            with os.fdopen(fd, "w", encoding="utf-8") as out:
                out.write(updated)
            os.chmod(tmp, os.stat(rel).st_mode)
            os.replace(tmp, rel)
    print(("APPLIED" if apply else "DRY"), len(changed))


main()
