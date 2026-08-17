#!/usr/bin/env python3
"""Splits the old energy 📦️lib.rs's 50 top-level `mod X { ... }` blocks into standalone files."""
import re, sys, os

SRC = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚙️engine/⚡️implementations/🦀️rust/📦️lib.rs"
OUT_DIR = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/⚙️engine"

with open(SRC, "r", encoding="utf-8") as f:
    text = f.read()

lines = text.split("\n")

def find_blocks(lines):
    blocks = []  # (name, start_line_idx, end_line_idx) inclusive of `mod X {` and closing `}`
    i = 0
    n = len(lines)
    mod_re = re.compile(r"^mod ([a-z_0-9]+) \{\s*$")
    while i < n:
        m = mod_re.match(lines[i])
        if m:
            name = m.group(1)
            start = i
            depth = 0
            j = i
            while j < n:
                depth += brace_delta(lines[j])
                if depth == 0 and j > start:
                    break
                if depth == 0 and j == start:
                    # single line unlikely, but guard
                    pass
                j += 1
            end = j
            blocks.append((name, start, end))
            i = end + 1
        else:
            i += 1
    return blocks

def brace_delta(line):
    """Count net {/} on this line, ignoring braces inside line comments and string literals."""
    delta = 0
    in_str = False
    escape = False
    k = 0
    L = len(line)
    while k < L:
        c = line[k]
        if in_str:
            if escape:
                escape = False
            elif c == "\\":
                escape = True
            elif c == '"':
                in_str = False
        else:
            if c == "/" and k + 1 < L and line[k+1] == "/":
                break  # rest of line is a comment
            if c == '"':
                in_str = True
            elif c == "{":
                delta += 1
            elif c == "}":
                delta -= 1
        k += 1
    return delta

blocks = find_blocks(lines)
print(f"found {len(blocks)} top-level mod blocks", file=sys.stderr)

os.makedirs(OUT_DIR, exist_ok=True)

manifest = []
for name, start, end in blocks:
    # body is lines[start+1 .. end-1] inclusive (excluding `mod X {` line and closing `}` line)
    body_lines = lines[start+1:end]
    # dedent by 4 spaces where present
    dedented = []
    for l in body_lines:
        if l.startswith("    "):
            dedented.append(l[4:])
        elif l.strip() == "":
            dedented.append("")
        else:
            dedented.append(l)
    body = "\n".join(dedented).strip("\n") + "\n"
    out_path = os.path.join(OUT_DIR, f"🦀️{name}.rs")
    with open(out_path, "w", encoding="utf-8") as f:
        f.write(body)
    manifest.append(name)

print("\n".join(manifest))

# sanity: verify total lines accounted for matches expectation (mod header lines + bodies + closing braces == region covered)
covered = sum(end - start + 1 for _, start, end in blocks)
print(f"covered {covered} of {len(lines)} lines (mod blocks incl wrapper lines)", file=sys.stderr)
first_start = blocks[0][1]
last_end = blocks[-1][2]
print(f"first block starts at line {first_start+1}, last block ends at line {last_end+1}", file=sys.stderr)
print(f"total file lines: {len(lines)}", file=sys.stderr)
