"""🧬️ S20 pass 2 (P2-F2) on the p2 overlay: every mutation-report site that names one of the 7 frozen codes (contract C2) —
`MutationOutcome::{error,fatal}(code, message, target)`, `MutationMessage::{info,warn,error,fatal}(code, message)` and the
outcome builders `.info/.warn(code, message)` — becomes the `MutationCode` variant with the message dropped; the file gains
`MutationCode` in the import that brings `MutationOutcome`/`MutationMessage` (or the site keeps its path qualifier).
Every other site (drift code, computed code, unparsable call) goes to the drift ledger for P2-F3. The definitions module is
skipped (P2-F1 edits it by hand). Idempotent: a converted site no longer matches.
Usage: python3 p2-codemod.py [--dry-run]   (ledger: .🧬semio/🌐hub/s14-s20-sets/p2/drift-sites.json)"""
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
LEDGER = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/p2/drift-sites.json")
ROOTS = ["✏️s/🔌️plugins", "✏️s/🔨️modules", "🧰️framework", "🌎️hub"]
SKIP_DIRS = {"node_modules", "target", "🤖️generated", ".git"}
DEFINITIONS = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"
FROZEN = {
    "mutation.target-missing": "TargetMissing", "mutation.no-op": "NoOp", "mutation.partial": "Partial",
    "mutation.clamped": "Clamped", "mutation.duplicate-id": "DuplicateId", "mutation.invariant": "Invariant",
    "mutation.cascade": "Cascade",
}
QUAL = r"(?P<qual>(?:(?:crate|super|self|[A-Za-z_]\w*)::)*)"
CALLS = [
    ("outcome", re.compile(QUAL + r"MutationOutcome(?:::<[^()]*?>)?::(?P<method>error|fatal)\s*\("), 3),
    ("message", re.compile(QUAL + r"MutationMessage::(?P<method>info|warn|error|fatal)\s*\("), 2),
    ("builder", re.compile(r"(?P<qual>)\.(?P<method>info|warn)\s*\("), 2),
    ("apply", re.compile(QUAL + r"MutationApplyError::(?P<method>new)\s*\("), 2),
]


def apply_variant(code: str) -> str:
    """🛡️ The frozen code an apply rejection (`mutation.apply.*`) means: a missing element → TargetMissing, a duplicate →
    DuplicateId, every other rejection of a diff its base cannot take → Invariant."""
    reason = code.removeprefix("mutation.apply.")
    return "TargetMissing" if reason.startswith("missing") else "DuplicateId" if reason.startswith("duplicate") else "Invariant"
CODE = re.compile(r'^(?:(?:[A-Za-z_]\w*::)*FaultCode::new\s*\(\s*)?"([^"\\]*)"\s*\)?$', re.S)
IMPORT = re.compile(r"^(?P<indent>[ \t]*)(?P<pub>pub(?:\([^)]*\))? )?use (?P<path>[\w:]+)::(?:\{(?P<group>[^;]*?)\}|(?P<single>Mutation(?:Outcome|Message|ApplyError)));", re.M)


def arguments(text: str, open_at: int) -> tuple[list[tuple[int, int]], int] | None:
    """✂️ The top-level argument spans of the call whose `(` is at `open_at` and the index after its `)`; `None` if the
    call cannot be parsed (a closure parameter list `|a, b|` or an unbalanced source)."""
    depth, start, spans, at = 0, open_at + 1, [], open_at
    while at < len(text):
        char = text[at]
        if char == '"' or (char == "r" and re.match(r'r#*"', text[at:at + 8]) and not re.match(r"\w", text[at - 1])):
            hashes = re.match(r'r(#*)"', text[at:at + 8]).group(1) if char == "r" else None
            if hashes is None:
                at += 1
                while text[at] != '"':
                    at += 2 if text[at] == "\\" else 1
            else:
                end = text.index('"' + hashes, at + len(hashes) + 2)
                at = end + len(hashes)
        elif char == "'" and re.match(r"'(?:\\.|[^'\\])'", text[at:at + 12]):
            at += len(re.match(r"'(?:\\.|[^'\\])+'", text[at:at + 12]).group(0)) - 1
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                spans.append((start, at))
                return [span for span in spans if text[span[0]:span[1]].strip()], at + 1
        elif char == "|" and depth == 1 and text[at + 1] != "|" and text[at - 1] != "|":
            return None
        elif char == "," and depth == 1:
            spans.append((start, at))
            start = at + 1
        at += 1
    return None


def rust_files() -> list[Path]:
    out = []
    for root in ROOTS:
        for current, dirs, names in os.walk(OVERLAY / root):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
            out += [Path(current) / name for name in names if name == "🦀️.rs"]
    return sorted(out)


def convert(rel: str, text: str, ledger: list[dict]) -> tuple[str, int, set[str]]:
    edits: list[tuple[int, int, str]] = []
    bare: set[str] = set()
    qualifiers = re.findall(r"(?<![\w:])((?:[A-Za-z_]\w*::)+)Mutation(?:Outcome|Message|ApplyError)\b", text)
    file_qual = max(set(qualifiers), key=qualifiers.count) if qualifiers else ""
    for kind, pattern, count in CALLS:
        for match in pattern.finditer(text):
            parsed = arguments(text, match.end() - 1)
            line = text.count("\n", 0, match.start()) + 1
            if parsed is None:
                ledger.append({"path": rel, "line": line, "kind": kind, "why": "unparsable call"})
                continue
            spans, end = parsed
            first = text[spans[0][0]:spans[0][1]].strip() if spans else ""
            if re.fullmatch(r"(?:[A-Za-z_]\w*::)*MutationCode::\w+", first):
                continue
            code = CODE.match(first)
            if kind == "builder" and (len(spans) != count or code is None):
                continue
            if len(spans) != count:
                ledger.append({"path": rel, "line": line, "kind": kind, "why": f"{len(spans)} arguments", "call": text[match.start():end][:300]})
                continue
            if kind == "apply" and code is not None and code.group(1).startswith("mutation.apply."):
                qual = match.group("qual") or file_qual
                if not qual:
                    bare.add(kind)
                edits.append((match.end(), end, f"{qual}MutationCode::{apply_variant(code.group(1))})"))
                continue
            if code is None or code.group(1) not in FROZEN:
                ledger.append({"path": rel, "line": line, "kind": kind, "method": match.group("method"), "code": code.group(1) if code else first, "message": text[spans[1][0]:spans[1][1]].strip()[:300]})
                continue
            qual = match.group("qual") or file_qual
            variant = f"{qual}MutationCode::{FROZEN[code.group(1)]}"
            if not qual:
                bare.add(kind)
            rest = [text[span[0]:span[1]].strip() for span in spans[2:]]
            edits.append((match.end(), end, ", ".join([variant, *rest]) + ")"))
    for start, end, replacement in sorted(edits, reverse=True):
        text = text[:start] + replacement + text[end:]
    return text, len(edits), bare


def add_import(text: str) -> tuple[str, bool]:
    """📥️ Inserts `MutationCode` before the first `MutationMessage`/`MutationOutcome` of every flat `use` group that imports
    one (a single-name `use` becomes a group); `False` when no import could carry it (glob, nested group, qualified use)."""
    changed = False

    def extend(match: re.Match) -> str:
        nonlocal changed
        group = match.group("group")
        head = f"{match.group('indent')}{match.group('pub') or ''}use {match.group('path')}::"
        if group is None:
            changed = True
            return f"{head}{{MutationCode, {match.group('single')}}};"
        names = [name.strip() for name in group.split(",")]
        if "{" in group or "MutationCode" in names or not any(name in ("MutationOutcome", "MutationMessage", "MutationApplyError") for name in names):
            changed = changed or "MutationCode" in names
            return match.group(0)
        changed = True
        at = min(group.index(name) for name in ("MutationApplyError", "MutationMessage", "MutationOutcome") if re.search(rf"\b{name}\b", group))
        return f"{head}{{{group[:at]}MutationCode, {group[at:]}}};"

    return IMPORT.sub(extend, text), changed


def main(dry_run: bool) -> None:
    ledger: list[dict] = []
    totals = {"files": 0, "sites": 0, "unimported": []}
    for path in rust_files():
        rel = str(path.relative_to(OVERLAY))
        if rel == DEFINITIONS:
            continue
        text = path.read_text()
        if "Mutation" not in text:
            continue
        converted, sites, bare = convert(rel, text, ledger)
        if not sites:
            continue
        if bare:
            converted, imported = add_import(converted)
            if not imported:
                totals["unimported"].append(rel)
        totals["files"] += 1
        totals["sites"] += sites
        if not dry_run:
            path.write_text(converted)
    LEDGER.parent.mkdir(parents=True, exist_ok=True)
    LEDGER.write_text(json.dumps(ledger, ensure_ascii=False, indent=1))
    print(f"files={totals['files']} sites={totals['sites']} drift/unparsed={len(ledger)} unimported={len(totals['unimported'])} → {LEDGER}")
    for rel in totals["unimported"][:40]:
        print("  unimported", rel)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
