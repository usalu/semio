#!/usr/bin/env python3
"""🧵 Wraps bare-path `compose:`/`run:`/`sniff: Some(..)` values inside `ComposerEntry { .. }` and
`IoEntry { .. }` struct literals in the matching E4 thunk macro (`compose_thunk!`/`io_run_thunk!`/
`io_sniff_thunk!`, all defined in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` next to
`AsyncComposeFn`/`ComposeFuture`).

WHY THIS EXISTS
----------------
`io-async-signatures` turned the leaf functions plugins register into these vtable rows into
`async fn`. An `async fn` item's pointer type is unnameable, so `compose: some_async_fn` (or
`run: some_async_fn` / `sniff: Some(some_async_fn)`) can never coerce into the row's bare-`fn`-typed
field. The fix is NOT to touch the leaf function (it correctly stays `async fn` per the universal-
async decree) — it is to wrap the VALUE at the construction site in the matching thunk macro, which
generates a small sync `fn` that drives the async leaf to completion (`compose_thunk!` boxes a real
future for later polling; `io_run_thunk!`/`io_sniff_thunk!` resolve synchronously via
`resolve_ready`, matching `IoEntry.run`/`.sniff`'s genuinely non-future-returning field types).

WHAT COUNTS AS "BARE PATH"
---------------------------
A field value is rewritten only when it is a plain path expression: an identifier, optionally
`::`-qualified, optionally carrying one `::<...>` turbofish — e.g. `compose_hop1`,
`deserializer_sniff::<S, T>`, `my_mod::my_fn`. Anything else (already `..._thunk!(...)`, a closure,
`Box::pin(...)`, a call `foo(...)`, `None`) is left untouched — this is what makes the script
idempotent: a second `--scan`/`--apply` pass over already-rewritten source reports/changes nothing.

USAGE
-----
    python3 compose-thunk-rewrite.py --scan  [--root <path>]           # JSON report, no writes
    python3 compose-thunk-rewrite.py --apply --root <path>             # rewrites bare-path sites

`--root` restricts the walk to paths under it (relative to the repo root or absolute); omit for a
repo-wide walk. This ticket's own binding rule: `--scan` may run repo-wide, `--apply` may only be
given `--root '🧰️framework'` — the 163-site fleet-wide application across `✏️s/**` belongs to the
`fleet-codemods` packet, not this one.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"

#region 🔖️Discovery
STRUCT_NAMES = ("ComposerEntry", "IoEntry")
BARE_PATH_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:::<[^>()]*>)?(?:::[A-Za-z_][A-Za-z0-9_]*(?:::<[^>()]*>)?)*$")
FIELD_RE = re.compile(r"\b(compose|run|sniff)\s*:(?!:)\s*")  # (?!:) so "run::<S, T>" (a path
# appearing INSIDE an already-found value) is never mistaken for a second "run:" field key
ALREADY_WRAPPED_RE = re.compile(r"(compose_thunk|io_run_thunk|io_sniff_thunk)\s*!\s*\(")


def iter_rust_files(root: str):
    """📂 Yields every `.rs` file under `root`, skipping `🎯️target*` build directories."""
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if not d.startswith("🎯️target")]
        for name in filenames:
            if name.endswith(".rs"):
                yield os.path.join(dirpath, name)


def find_struct_literal_spans(text: str, struct_name: str):
    """🔎 Finds every `<struct_name> { .. }` literal span via balanced-brace scanning, skipping
    string/char literals so a `{`/`}` inside a `Dialect` field's string doesn't desync the count.
    Yields (open_brace_idx, close_brace_idx) covering the braces themselves.
    """
    pattern = re.compile(rf"\b{re.escape(struct_name)}\s*\{{")
    for m in pattern.finditer(text):
        # 🚧 skip the struct DEFINITION itself (`pub struct IoEntry { .. }`) and a function's
        # RETURN-TYPE-then-body-open (`fn f(..) -> IoEntry {`) — both use the exact same `Name {`
        # text a struct LITERAL does, and the latter would otherwise swallow the entire function
        # body (including any REAL `IoEntry { .. }` literal inside it) as one bogus outer span.
        preceding = text[:m.start()].rstrip()
        if preceding.endswith("struct") or preceding.endswith("->"):
            continue
        open_idx = m.end() - 1
        depth = 0
        i = open_idx
        in_string = False
        in_char = False
        escape = False
        while i < len(text):
            c = text[i]
            if in_string:
                if escape:
                    escape = False
                elif c == "\\":
                    escape = True
                elif c == '"':
                    in_string = False
            elif in_char:
                if escape:
                    escape = False
                elif c == "\\":
                    escape = True
                elif c == "'":
                    in_char = False
            elif c == '"':
                in_string = True
            elif c == "'" and i + 1 < len(text) and text[i + 1] != "\\" and i + 2 < len(text) and text[i + 2] == "'":
                # 🩹 heuristic: single-quoted char literal like 'a', not a lifetime like 'a>
                in_char = True
            elif c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    yield (open_idx, i)
                    break
            i += 1


def classify_field(value: str):
    """🏷️ Returns 'bare' (needs wrapping), 'wrapped' (already a thunk macro call), or 'other'
    (closure/call/None/anything else — left alone, reported separately)."""
    value = value.strip()
    if ALREADY_WRAPPED_RE.search(value):
        return "wrapped"
    if BARE_PATH_RE.match(value):
        return "bare"
    return "other"
#endregion 🔖️Discovery

#region 🔖️Extraction
def extract_sites(path: str, text: str):
    """🧾 Yields dicts describing every `compose:`/`run:`/`sniff: Some(..)` field found inside a
    `ComposerEntry`/`IoEntry` literal in `text`, each carrying enough to both report and rewrite."""
    sites = []
    for struct_name in STRUCT_NAMES:
        field_name = "compose" if struct_name == "ComposerEntry" else None
        for open_idx, close_idx in find_struct_literal_spans(text, struct_name):
            body = text[open_idx + 1:close_idx]
            body_offset = open_idx + 1
            for fm in FIELD_RE.finditer(body):
                field = fm.group(1)
                if struct_name == "ComposerEntry" and field != "compose":
                    continue
                if struct_name == "IoEntry" and field not in ("run", "sniff"):
                    continue
                value_start = fm.end()
                # 🔪 the field value runs to the next top-level ',' or the end of the struct body —
                # track paren/bracket/brace AND angle-bracket depth so a value like
                # `Some(deserializer_sniff::<S, T>)` or the bare `deserializer_sniff::<S, T>` (no
                # wrapping parens at all) doesn't split early on the turbofish's internal comma.
                # Angle brackets only ever appear here as `::<...>` turbofish (field values are
                # paths/calls/macro invocations, never comparison expressions), so naive `<`/`>`
                # counting is safe in this narrow context.
                depth = 0
                j = value_start
                while j < len(body):
                    c = body[j]
                    if c in "([{<":
                        depth += 1
                    elif c in ")]}>":
                        depth -= 1
                    elif c == "," and depth == 0:
                        break
                    j += 1
                raw_value = body[value_start:j]
                abs_start = body_offset + value_start
                abs_end = body_offset + j
                line = text.count("\n", 0, abs_start) + 1
                sites.append({
                    "path": path, "line": line, "struct": struct_name, "field": field,
                    "raw_value": raw_value.strip(), "abs_start": abs_start, "abs_end": abs_end,
                })
    return sites


def plan_rewrite(site: dict):
    """✍️ Returns (new_text, classification) for one site — `None` new_text means "not bare",
    i.e. nothing to rewrite (already wrapped, or not a plain path)."""
    field, raw = site["field"], site["raw_value"]
    if field == "sniff":
        m = re.match(r"^Some\s*\((.*)\)$", raw, re.DOTALL)
        if not m:
            return None, "other" if raw != "None" else "none"
        inner = m.group(1).strip()
        kind = classify_field(inner)
        if kind != "bare":
            return None, kind
        return f"Some(io_sniff_thunk!({inner}))", "bare"
    macro = {"compose": "compose_thunk", "run": "io_run_thunk"}[field]
    kind = classify_field(raw)
    if kind != "bare":
        return None, kind
    return f"{macro}!({raw})", "bare"
#endregion 🔖️Extraction

#region 🔖️Apply
def apply_file(path: str, text: str, sites: list[dict]) -> tuple[str, int]:
    """✏️ Applies every rewritable site in `sites` (already filtered to this file), descending by
    byte offset so earlier offsets stay valid."""
    edits = []
    for site in sites:
        new_value, kind = plan_rewrite(site)
        if kind == "bare" and new_value is not None:
            edits.append((site["abs_start"], site["abs_end"], new_value))
    edits.sort(key=lambda e: e[0], reverse=True)
    applied = 0
    for start, end, new_value in edits:
        text = text[:start] + new_value + text[end:]
        applied += 1
    return text, applied
#endregion 🔖️Apply

#region 🔖️Main
def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--scan", action="store_true")
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--root", default=None, help="restrict the walk to this path (relative to repo root, or absolute)")
    ap.add_argument("--report", default=None, help="write the --scan JSON report to this path too")
    args = ap.parse_args()

    if not args.scan and not args.apply:
        ap.error("choose --scan or --apply")

    root = args.root
    if root is None:
        root = REPO
    elif not os.path.isabs(root):
        root = os.path.join(REPO, root)

    totals = {"files_scanned": 0, "files_with_sites": 0, "bare": 0, "wrapped": 0, "other": 0, "none": 0}
    bare_sites = []
    other_sites = []

    for path in sorted(iter_rust_files(root)):
        with open(path, "r", encoding="utf-8") as fh:
            text = fh.read()
        if "ComposerEntry" not in text and "IoEntry" not in text:
            continue
        totals["files_scanned"] += 1
        sites = extract_sites(path, text)
        if not sites:
            continue
        totals["files_with_sites"] += 1

        classified = []
        for site in sites:
            new_value, kind = plan_rewrite(site)
            classified.append((site, new_value, kind))
            totals[kind] = totals.get(kind, 0) + 1
            rel = os.path.relpath(path, REPO)
            record = {"path": rel, "line": site["line"], "struct": site["struct"], "field": site["field"], "value": site["raw_value"]}
            if kind == "bare":
                bare_sites.append(record)
            elif kind == "other":
                other_sites.append(record)

        if args.apply:
            bare_only = [s for s, nv, k in classified if k == "bare"]
            if not bare_only:
                continue
            new_text, applied = apply_file(path, text, bare_only)
            if applied:
                with open(path, "w", encoding="utf-8") as fh:
                    fh.write(new_text)
                rel = os.path.relpath(path, REPO)
                print(f"  applied {applied} thunk-wrap edit(s) -> {rel}")

    report = {"root": os.path.relpath(root, REPO) if root.startswith(REPO) else root, "totals": totals, "bare_sites": bare_sites, "other_sites": other_sites}
    print(json.dumps(totals, indent=2))
    if args.report:
        with open(args.report, "w", encoding="utf-8") as fh:
            json.dump(report, fh, indent=2, ensure_ascii=False)
        print(f"report -> {args.report}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
#endregion 🔖️Main
