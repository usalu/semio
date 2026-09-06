#!/usr/bin/env python3
"""🧯️ De-async a plugin crate back to the framework's synchronous trait convention.

Two idempotent passes over `*.rs` under a plugin root:

* ``strip`` — rewrites ``async fn NAME(`` to ``fn NAME(`` for every declaration EXCEPT
  (a) ``#[semio_framework_async_macros::async_test]`` test bodies (the macro rejects a sync fn) and
  (b) the genuinely suspending framework surface: ``serialize``/``deserialize``/``sniff`` inside an
  ``impl Serializer<_>``/``impl Deserializer<_>`` (``🧰️framework/🔨️modules/🚪️io/🦀️.rs`` declares
  them ``-> impl Future``), ``infer_cached`` inside an ``impl ArtifactInferrer`` and ``decompose``
  inside an ``impl ArtifactDecomposer`` (both ``async fn`` in
  ``🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs``) — a sync declaration found in one of
  those blocks is restored to ``async``.
* ``unawait`` — deletes the ``.await`` postfix at the byte spans rustc reported as
  ``E0728 await is only allowed inside async functions`` or ``E0277 … is not a future``, i.e.
  exactly the call sites of the now-synchronous callees.

Both passes are re-runnable: ``strip`` skips already-sync declarations, ``unawait`` skips spans that
no longer carry an ``.await``.
"""

import json
import os
import re
import sys

KEEP_TRAITS = ("Serializer", "Deserializer", "ArtifactInferrer", "ArtifactDecomposer")
KEEP_METHODS = {"serialize", "deserialize", "sniff", "infer_cached", "decompose"}
DECL = re.compile(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)async fn (\w+)")
SYNC_DECL = re.compile(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)fn (\w+)")
IMPL = re.compile(r"^\s*impl\b[^{]*\b(" + "|".join(KEEP_TRAITS) + r")\b[^{]*\bfor\b")


def keep_regions(lines):
    """🛡️ Line indices covered by an `impl Serializer/Deserializer … for … { … }` block."""
    spans = []
    for i, line in enumerate(lines):
        if not IMPL.match(line):
            continue
        depth, started, j = 0, False, i
        while j < len(lines):
            for ch in lines[j]:
                if ch == "{":
                    depth += 1
                    started = True
                elif ch == "}":
                    depth -= 1
            if started and depth <= 0:
                break
            j += 1
        spans.append((i, j))
    return spans


def is_async_test(lines, i):
    j = i - 1
    while j >= 0:
        s = lines[j].strip()
        if s.startswith("#[") or s.startswith("///") or s.startswith("//"):
            if "async_test" in s:
                return True
            j -= 1
            continue
        return False
    return False


def strip(root):
    changed, touched = 0, 0
    for dirpath, _, names in os.walk(root):
        for name in names:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            text = open(path, encoding="utf-8").read()
            lines = text.split("\n")
            spans = keep_regions(lines)
            hits = 0
            for i, line in enumerate(lines):
                inside = any(a <= i <= b for a, b in spans)
                s = SYNC_DECL.match(line)
                if s and inside and s.group(3) in KEEP_METHODS:
                    lines[i] = f"{s.group(1)}{s.group(2)}async fn {s.group(3)}" + line[s.end():]
                    hits += 1
                    continue
                m = DECL.match(line)
                if not m:
                    continue
                if is_async_test(lines, i):
                    continue
                if m.group(3) in KEEP_METHODS and inside:
                    continue
                lines[i] = f"{m.group(1)}{m.group(2)}fn {m.group(3)}" + line[m.end():]
                hits += 1
            if hits:
                open(path, "w", encoding="utf-8").write("\n".join(lines))
                changed += hits
                touched += 1
    print(f"strip: {changed} declarations in {touched} files")


def unawait(spans_path):
    """✂️ `spans_path` holds `cargo --message-format=json` diagnostics; drop each reported `.await`."""
    edits = {}
    for line in open(spans_path, encoding="utf-8"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        diag = msg.get("message")
        if not isinstance(diag, dict):
            continue
        code = (diag.get("code") or {}).get("code")
        text = diag.get("message") or ""
        if code == "E0728":
            kind = "keyword"
        elif code == "E0277" and "is not a future" in text:
            kind = "expression"
        else:
            continue
        for span in diag.get("spans", []):
            if not span.get("is_primary"):
                continue
            edits.setdefault(os.path.abspath(span["file_name"]), set()).add(
                (span["line_start"], span["column_start"], span["line_end"], span["column_end"], kind)
            )
    total = 0
    for path, spans in sorted(edits.items()):
        lines = open(path, encoding="utf-8").read().split("\n")
        for ls, cs, le, ce, kind in sorted(spans, reverse=True):
            if ls != le:
                continue
            row = lines[ls - 1]
            cut = cs - 2 if kind == "keyword" else cs - 1 + row[cs - 1 : ce - 1].rfind(".await")
            if cut < 0 or row[cut : cut + len(".await")] != ".await":
                continue
            lines[ls - 1] = row[:cut] + row[cut + len(".await") :]
            total += 1
        open(path, "w", encoding="utf-8").write("\n".join(lines))
    print(f"unawait: {total} `.await` removed across {len(edits)} files")


if __name__ == "__main__":
    if sys.argv[1] == "strip":
        strip(sys.argv[2])
    elif sys.argv[1] == "unawait":
        unawait(sys.argv[2])
    else:
        raise SystemExit("usage: 🔨️lane-o-de-async.py strip <plugin-root> | unawait <diagnostics.jsonl>")
