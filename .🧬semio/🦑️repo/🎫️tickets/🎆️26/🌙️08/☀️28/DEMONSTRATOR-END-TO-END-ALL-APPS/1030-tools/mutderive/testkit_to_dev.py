#!/usr/bin/env python3
import os, re, subprocess, sys, tempfile

TICKETS = ".🧬semio/"
KEEP = ("🧰️test-support/",)
DEP = re.compile(r'^(\s*semio-framework-value\s*=\s*\{\s*path\s*=\s*"[^"]*")\s*,\s*features\s*=\s*\["testkit"\]\s*\}(\s*)$')
SEC = re.compile(r'^\s*\[([^\]]+)\]\s*$')


def transform(text):
    lines = text.split("\n")
    out, section, moved, dev_has, dev_end = [], None, None, False, None
    for line in lines:
        m = SEC.match(line)
        if m:
            section = m.group(1)
        d = DEP.match(line) if section == "dependencies" else None
        if d:
            moved = d.group(1)
            out.append(d.group(1) + " }" + d.group(2))
            continue
        out.append(line)
    if moved is None:
        return text
    dev_line = moved + ', features = ["testkit"] }'
    section, idx = None, None
    for i, line in enumerate(out):
        m = SEC.match(line)
        if m:
            section = m.group(1)
            if section == "dev-dependencies":
                idx = i
        if section == "dev-dependencies" and re.match(r'^\s*semio-framework-value\s*=', line):
            return "\n".join(out)
    if idx is None:
        body = "\n".join(out).rstrip("\n")
        return body + "\n\n[dev-dependencies]\n" + dev_line + "\n"
    out.insert(idx + 1, dev_line)
    return "\n".join(out)


def main():
    root = os.getcwd()
    apply = "--apply" in sys.argv
    files = subprocess.check_output(["git", "grep", "-l", "-z", "semio-framework-value.*testkit", "--", "*Cargo.toml"]).decode().split("\0")
    n = 0
    for rel in sorted(f for f in files if f and not f.startswith(TICKETS) and not any(k in f for k in KEEP)):
        with open(rel, encoding="utf-8") as h:
            text = h.read()
        new = transform(text)
        if new == text:
            continue
        n += 1
        if apply:
            fd, tmp = tempfile.mkstemp(dir=os.path.dirname(rel))
            with os.fdopen(fd, "w", encoding="utf-8") as o:
                o.write(new)
            os.chmod(tmp, os.stat(rel).st_mode)
            os.replace(tmp, rel)
    print(("APPLIED" if apply else "DRY"), n)


main()
