#!/usr/bin/env python3
"""🪝 Repeated-`.await` hoister v2 for gate-services — BYTES ONLY, no str/bytes offset mixing.

v1 (deleted) decoded the file to `str` for regex matching (Python `re` on `str` counts CODE
POINTS), then spliced using those same integer offsets into the raw `bytes` object (which is
indexed in BYTES) — this file is full of multi-byte emoji, so every position downstream of the
first emoji was already wrong, and it corrupted a function boundary. Root-caused after the fact by
reproducing it: byte len vs char len diverge by thousands of positions in this file.

This version does everything in `bytes` throughout: patterns are `rb"..."` byte-strings, matches
are `bytes` objects, splicing is on the same `bytes` buffer regex operated on. No decode except for
printing.

WHAT THIS FIXES
----------------
`let x = some_async_fn(..);` followed by `x.await` used MORE THAN ONCE in the same function is
R10's residue shape 2 ("awaiting one future repeatedly") — `x` is `impl Future<Output=T>`, not
`Copy`, so the first `.await` moves it and every later `x.await` is E0382. Fix: hoist the `.await`
onto the LET binding, then every later occurrence becomes plain `x` (no `.await`).

SCOPING (still diagnostic-adjacent, not blind name substitution)
------------------------------------------------------------------
Operates function-by-function: for each `#[semio_framework_async_macros::async_test]` /
`#[test]` block (found by scanning for the attribute then locating its `async fn ... {` and the
MATCHING closing brace via byte-level brace counting), collect every identifier that appears as
`IDENT.await` two or more times WITHIN THAT SAME FUNCTION BODY, and only then hoist it — a name
used once, or names in different functions, are left untouched, so this cannot cross a function
boundary the way v1's byte-count bug did.

Usage:
  python3 terra-gateservices-hoist-repeated-await-v2.py --apply
  python3 terra-gateservices-hoist-repeated-await-v2.py --dry-run
"""
import argparse, re, sys, os

REPO = "/Users/ueli/Documents/semio"
FILE = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs".encode("utf-8")
FILE = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs"

FN_HEAD_RE = re.compile(rb"async\s+fn\s+[A-Za-z0-9_]+\s*\([^)]*\)[^{;]*\{")


def iter_function_bodies(data: bytes):
    """Yield (body_start, body_end) byte offsets for every `async fn ... { ... }` top-level match
    in the test module, via brace counting from each `{` found by FN_HEAD_RE."""
    for m in FN_HEAD_RE.finditer(data):
        body_start = m.end() - 1  # the `{` itself
        depth = 0
        i = body_start
        n = len(data)
        while i < n:
            b = data[i:i+1]
            if b == b"{":
                depth += 1
            elif b == b"}":
                depth -= 1
                if depth == 0:
                    yield (body_start, i + 1)
                    break
            i += 1


def hoist_in_body(body: bytes) -> bytes:
    """Find identifiers awaited 2+ times in this body and hoist each to its `let` binding."""
    await_re = re.compile(rb"\b([A-Za-z_][A-Za-z0-9_]*)\.await\b")
    counts = {}
    for m in await_re.finditer(body):
        name = m.group(1)
        counts[name] = counts.get(name, 0) + 1
    repeated = [name for name, c in counts.items() if c >= 2]
    if not repeated:
        return body

    changed = body
    for name in repeated:
        let_re = re.compile(rb"\blet\s+(mut\s+)?" + re.escape(name) + rb"\b\s*(:\s*[^=;]+?)?\s*=([^;]*);")
        let_m = let_re.search(changed)
        if not let_m:
            print(f"  !! no `let {name.decode()} = ...;` found in this body — skipping", file=sys.stderr)
            continue
        rhs = let_m.group(3)
        if rhs.rstrip().endswith(b".await"):
            new_binding = let_m.group(0)  # already hoisted
        else:
            type_ann = b" " + let_m.group(2) if let_m.group(2) else b""
            new_binding = b"let " + (let_m.group(1) or b"") + name + type_ann + b" =" + rhs.rstrip() + b".await;"
        # Replace the let-binding first (single occurrence), then strip `.await` from every
        # `name.await` OUTSIDE that binding's own span (the binding's RHS itself must keep its
        # single real `.await`, already handled above).
        before = changed[: let_m.start()]
        after = changed[let_m.end():]
        after = re.sub(rb"\b" + re.escape(name) + rb"\.await\b", name, after)
        before = re.sub(rb"\b" + re.escape(name) + rb"\.await\b", name, before)
        changed = before + new_binding + after
    return changed


def process(data: bytes):
    edits = 0
    out = bytearray(data)
    # Collect spans first (on the ORIGINAL data), then apply from the END backward so earlier
    # offsets stay valid — never re-scan `out` positions against spans computed on `data`.
    spans = list(iter_function_bodies(data))
    spans.sort(key=lambda s: s[0], reverse=True)
    for body_start, body_end in spans:
        body = bytes(out[body_start:body_end])
        new_body = hoist_in_body(body)
        if new_body != body:
            edits += 1
            out[body_start:body_end] = new_body
    return bytes(out), edits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    with open(FILE, "rb") as fh:
        data = fh.read()
    new_data, edits = process(data)
    print(f"functions with hoisted repeats: {edits}")
    if args.apply and new_data != data:
        with open(FILE, "wb") as fh:
            fh.write(new_data)
        print("written")
    elif args.dry_run:
        print("(dry-run, not written)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
