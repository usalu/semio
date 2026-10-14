#!/usr/bin/env python3
"""🔪️ Splits one oversized unit-test file into a shared context file plus several chunk files that each fit a unit.

The file is `pub mod context { … }` followed by top-level tests. The context body becomes its own file; helpers named by
more than one chunk move into it as `pub` items; every top-level `use` of the test body is flattened to leaves and each
chunk keeps only the leaves its own text names; each chunk opens with its doc line and `use context::*;`.

usage: 🔪️split-test-file.py <spec.json> plan|apply
spec: {"file": "<repo-rel test file>", "context": {"path", "doc"},
       "chunks": [{"path", "doc", "from": "<first line text of the chunk, exact>"}]}   (first chunk path may be the file)
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
census = {}
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
strip = census["strip"]
spec, mode = json.load(open(sys.argv[1], encoding="utf-8")), sys.argv[2]
absolute = lambda p: os.path.normpath(os.path.join(REPO, p))
source = open(absolute(spec["file"]), encoding="utf-8").read()
stripped = strip(source)


def block_end(s, open_at):
    depth = 0
    for i in range(open_at, len(s)):
        if s[i] == "{":
            depth += 1
        elif s[i] == "}":
            depth -= 1
            if depth == 0:
                return i + 1
    raise SystemExit("unbalanced")


head = re.match(r'pub mod context \{', stripped)
if not head:
    raise SystemExit("file must open with `pub mod context {`")
context_end = block_end(stripped, head.end() - 1)
context_body = source[head.end():context_end - 1]
context_body = "\n".join(line[4:] if line.startswith("    ") else line.strip() for line in context_body.strip("\n").split("\n"))
context_body = context_body.replace("use super::super::*;", "use super::*;")
body, sbody = source[context_end:], stripped[context_end:]


def expand(path, tree, out):
    tree = tree.strip()
    if tree.startswith("{"):
        depth, part, parts = 0, "", []
        for ch in tree[1:-1]:
            if ch == "," and depth == 0:
                parts.append(part); part = ""
                continue
            depth += ch == "{"
            depth -= ch == "}"
            part += ch
        parts.append(part)
        for p in parts:
            if p.strip():
                expand(path, p, out)
        return
    m = re.match(r'^((?:[\w]+\s*::\s*)*)(\{.*\}|[\w*]+(?:\s+as\s+\w+)?)$', tree, re.S)
    prefix = re.sub(r'\s+', '', m.group(1))
    if m.group(2).startswith("{"):
        expand(path + prefix, m.group(2), out)
    else:
        leaf = re.sub(r'\s+', ' ', m.group(2))
        name = leaf.split(" as ")[-1].strip()
        out.append((path + prefix + leaf, name))


uses, spans = [], []
depth = 0
for m in re.finditer(r'[{}]|(?m)^((?:#\[[^\]]*\]\s*)*)use\s+', sbody):
    if m.group(0) == "{":
        depth += 1
    elif m.group(0) == "}":
        depth -= 1
    elif depth == 0 and not m.group(1):
        end = sbody.index(";", m.end())
        tree = source[context_end + m.end():context_end + end]
        if tree.strip() in ("context::*", "super::*"):
            spans.append((m.start(), end + 1))
            continue
        expand("", tree, uses)
        spans.append((m.start(), end + 1))
lines = list(body)
for start, end in sorted(spans, reverse=True):
    while True:
        previous = body.rfind("\n", 0, max(start - 1, 0)) + 1
        if start > 0 and body[previous:start].lstrip().startswith("///"):
            start = previous
        else:
            break
    seg_end = end + 1 if body[end:end + 1] == "\n" else end
    lines[start:seg_end] = []
body = "".join(lines)
sbody = strip(body)
starts = []
for chunk in spec["chunks"]:
    at = body.find("\n" + chunk["from"] + "\n") + 1 if chunk.get("from") else 0
    if chunk.get("from") and at == 0:
        raise SystemExit("chunk start not found: " + chunk["from"])
    starts.append(at)
starts.append(len(body))
texts = [body[starts[i]:starts[i + 1]] for i in range(len(spec["chunks"]))]
helpers = []
for m in re.finditer(r'(?m)^((?:///[^\n]*\n|#\[[^\n]*\]\n)*)(?:pub )?(async )?fn (\w+)', sbody):
    if re.search(r'#\[(?:test|[\w:]*async_test|should_panic)', m.group(1)):
        continue
    name = m.group(3)
    owner = next(i for i in range(len(texts)) if starts[i] <= m.start() < starts[i + 1])
    users = [i for i, t in enumerate(texts) if re.search(r'\b' + name + r'\b', strip(t))]
    if users != [owner]:
        end = block_end(sbody, sbody.index("{", m.end()))
        helpers.append((m.start(), end, name, owner))
for start, end, name, owner in sorted(helpers, reverse=True):
    text = body[start:end]
    text = re.sub(r'(?m)^(?:pub )?((?:async )?fn ' + name + r')', r'pub \1', text, count=1)
    context_body += "\n\n" + text
    rel_start = start - starts[owner]
    t = texts[owner]
    seg = t[rel_start:rel_start + (end - start)]
    assert seg == body[start:end]
    texts[owner] = t[:rel_start] + t[rel_start + (end - start):]
out = {}
out[absolute(spec["context"]["path"])] = f"//! {spec['context']['doc']}\n\n" + context_body.strip("\n") + "\n"
for chunk, text in zip(spec["chunks"], texts):
    s = strip(text)
    keep = [leaf for leaf, name in uses if name == "*" or re.search(r'\b' + re.escape(name) + r'\b', s)]
    seen, kept = set(), []
    for leaf in keep:
        if leaf not in seen:
            seen.add(leaf); kept.append(f"use {leaf};")
    out[absolute(chunk["path"])] = f"//! {chunk['doc']}\n\nuse context::*;\n" + "\n".join(kept) + "\n\n" + re.sub(r'\n{3,}', "\n\n", text.strip("\n")) + "\n"
for path, text in out.items():
    print(f"{os.path.relpath(path, REPO)}: {text.count(chr(10))} lines, {len(text.encode())} bytes")
print("helpers moved:", [h[2] for h in helpers])
if mode == "apply":
    for path, text in out.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)
