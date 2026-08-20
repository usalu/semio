#!/usr/bin/env python3
# 🚨🚨🚨 BROKEN — DO NOT RUN --apply AGAIN. Kept only as a documented cautionary artifact. 🚨🚨🚨
# It uses `span["byte_end"]` (a UTF-8 BYTE offset from `cargo --message-format=json`) directly as a
# Python STRING (Unicode codepoint) index. In a codebase this heavy with multi-byte emoji, byte
# offset != codepoint offset, so every insertion after the first multi-byte character drifts —
# landing `.await` inside doc comments, mid-identifier, mid-string-literal, or as an orphan token.
# Corrupted 372 sites across 16 files in one `--apply` run on 2026-08-20 (`alltargets-hard` packet;
# see 📓️terra-alltargets-hard-report.md, section "A severe bug in a tool I wrote"). Recovered via
# terra-hard-undo-corrupted-await.py + terra-hard-safe-await-fixer.py (the correct, byte-safe,
# per-line replacement — USE THAT ONE) + terra-hard-final-corruption-sweep.py +
# terra-hard-diagnostic-remove-bad-await.py. If you need this tool's FUNCTIONALITY (finding unawaited
# "must be used" Future warnings and inserting `.await`), use terra-hard-safe-await-fixer.py instead.
#
# 🩹 terra-hard-unawaited-future-fixer.py
#
# R10-compliant diagnostic-driven recovery tool (span-keyed, NOT name-keyed).
#
# Targets the "encoder writes almost nothing" bug class this ticket has hit before: a bulk
# asyncify codemod made a fn `async` but a caller kept using it as a plain statement, so the
# compiler emits `warning: unused implementer of \`Future\` that must be used` (not an error —
# `cargo check` still exits 0) while the call never actually runs.
#
# Reads `cargo check --message-format=json` compiler output, extracts every "unused implementer
# of `Future`" warning's primary span, and appends ".await" at the END of that exact span (right
# before the statement's trailing ";"), using the byte-accurate span the compiler itself reports —
# never a name or regex guess. Refuses (reports, does not guess) if a span's file has changed
# since the diagnostic was captured in a way that would make byte offsets unsafe — this tool
# expects to run immediately against a fresh `--message-format=json` capture, not a stale one.
import json
import sys
import argparse
from collections import defaultdict


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("json_path", help="path to a file containing cargo's --message-format=json output")
    ap.add_argument("--scope", help="only edit paths containing this substring (path-segment safety)")
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    edits_by_file = defaultdict(list)  # file -> list of (byte_start, byte_end)
    sites = 0

    with open(args.json_path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or not line.startswith("{"):
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            inner = msg.get("message", {})
            if inner.get("level") != "warning":
                continue
            text = inner.get("message", "")
            if "unused implementer of `Future` that must be used" not in text and "unused implementer of `dyn Future`" not in text:
                continue
            for span in inner.get("spans", []):
                if not span.get("is_primary"):
                    continue
                fname = span["file_name"]
                if args.scope and args.scope not in fname:
                    continue
                byte_end = span["byte_end"]
                edits_by_file[fname].append(byte_end)
                sites += 1

    print(f"found {sites} unawaited-Future warning sites across {len(edits_by_file)} files")
    for fname, ends in sorted(edits_by_file.items()):
        print(f"  {fname}: {len(ends)}")

    if not args.apply:
        print("dry-run only; pass --apply to write")
        return 0

    total_applied = 0
    for fname, ends in edits_by_file.items():
        with open(fname, encoding="utf-8") as f:
            text = f.read()
        # dedupe + sort descending so earlier byte offsets stay valid as we splice from the end
        unique_ends = sorted(set(ends), reverse=True)
        out = text
        applied = 0
        for pos in unique_ends:
            # sanity: only insert if this exact position isn't already followed by ".await"
            if out[pos:pos + 6] == ".await":
                continue
            out = out[:pos] + ".await" + out[pos:]
            applied += 1
        if out != text:
            with open(fname, "w", encoding="utf-8") as f:
                f.write(out)
            total_applied += applied
            print(f"applied {applied} edits to {fname}")
    print(f"total applied: {total_applied}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
