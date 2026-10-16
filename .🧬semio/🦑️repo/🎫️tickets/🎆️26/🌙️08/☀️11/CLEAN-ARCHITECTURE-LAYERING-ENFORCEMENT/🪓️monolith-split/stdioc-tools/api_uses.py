#!/usr/bin/env python3
# 🔎️ Collects every distinct public path the dependents (outside the contract tree) use from `semio_s_artifact_stdio_contract`, groups expanded.
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
OUT = sys.argv[1]
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "grep", "-l", "semio_s_artifact_stdio_contract", "--", "*.rs", ":!.🧬semio", ":!✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
files = [f for f in files if f]
NAME = "semio_s_artifact_stdio_contract"
paths = set()


def expand(prefix, body):
    out = []
    depth, current, parts = 0, "", []
    for ch in body:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(current)
            current = ""
        else:
            current += ch
    if current.strip():
        parts.append(current)
    for part in parts:
        part = part.strip()
        if not part:
            continue
        part = re.sub(r"\s+", " ", part)
        if "{" in part:
            head, rest = part.split("{", 1)
            head = head.replace(" ", "")
            out.extend(expand(prefix + head, rest.rsplit("}", 1)[0]))
        else:
            name = re.split(r" as ", part)[0].replace(" ", "")
            out.append(prefix + name)
    return out


for rel in files:
    text = open(os.path.join(ROOT, rel), encoding="utf8", errors="replace").read()
    text = re.sub(r"(?m)^\s*//.*$", "", text)
    for match in re.finditer(NAME + r"::", text):
        i = match.end()
        path = ""
        while i < len(text):
            ch = text[i]
            if ch == "{" and not path.endswith("::"):
                break
            if ch == "{":
                depth, j = 1, i + 1
                while depth and j < len(text):
                    depth += text[j] == "{"
                    depth -= text[j] == "}"
                    j += 1
                paths.update(expand(path, text[i + 1:j - 1]))
                path = None
                break
            if text[i:i + 2] == "::":
                path += "::"
                i += 2
                continue
            if re.match(r"[A-Za-z0-9_]", ch):
                path += ch
                i += 1
                continue
            break
        if path is not None:
            paths.add(path.rstrip(":"))
cleaned = sorted(p for p in paths if p and not p.endswith("self") and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z_][A-Za-z0-9_]*)*", p))
open(OUT, "w", encoding="utf8").write("\n".join(cleaned) + "\n")
print(len(files), "files", len(cleaned), "distinct paths")
