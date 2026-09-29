"""🧬️ FH4 pass-2 P2-F3 edits on S20's p2 overlay (families P2-X wfc norm remodel draw cad raster fem forms space gis layout,
P2-Y stdio): every drift mutation-report site of `my-sites-0.json` becomes the frozen `MutationCode` that means the same
(§6 of `📓️fault-localization-api.md`), level kept, message dropped, target naming the element; then the hand edits
(`edits/*.py`, exact old → new spans) for producers, helpers, tests, fixtures and TS twins.
Idempotent: a converted call no longer carries a literal code; an edit whose `new` is present and `old` absent is applied.
Sites = `ledger-1.json` (`ledger.py 1`: S20's ledger re-run over my families after S20's apply codemod).
Usage: python3 fh4-p2.py [--dry-run] [--list] [family-substring]"""
from __future__ import annotations

import importlib.util
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).parent
sys.path.insert(0, str(HERE))
OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
PLUGINS = "✏️s/🔌️plugins/"
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh4-backup")
spec = importlib.util.spec_from_file_location("p2codemod", "/Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-p2/p2-codemod.py")
codemod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(codemod)

VARIANT_OF_CODE = {
    "mutation.missing": "TargetMissing", "mutation.missing-target": "TargetMissing", "mutation.missing-id": "TargetMissing",
    "mutation.apply.missing-target": "TargetMissing", "mutation.text-missing": "TargetMissing", "mutation.path-missing": "TargetMissing",
    "mutation.target-invalid": "TargetMissing", "forms.missing-response": "TargetMissing",
    "wfc3d.tile.missing": "TargetMissing", "wfc3d.slot.missing": "TargetMissing", "wfc3d.edge.missing": "TargetMissing",
    "wfc3d.rule.missing": "TargetMissing",
    "mutation.duplicate": "DuplicateId", "forms.duplicate-response": "DuplicateId", "wfc3d.tile.duplicate-id": "DuplicateId",
    "wfc3d.edge.duplicate-id": "DuplicateId", "wfc3d.rule.duplicate-id": "DuplicateId", "wfc3d.slot.duplicate-id": "DuplicateId",
    "wfc3d.tile.weight-unchanged": "NoOp", "wfc3d.seed.unchanged": "NoOp", "wfc3d.slot.pin-unchanged": "NoOp",
    "wfc3d.slot.pin-absent": "NoOp", "wfc3d.slot.extent-unchanged": "NoOp", "wfc3d.edge.already-connected": "NoOp",
    "wfc3d.tile.media-unchanged": "NoOp", "wfc3d.slot.position-unchanged": "NoOp",
    "wfc3d.slot.edges-cascaded": "Cascade", "wfc3d.tile.references-cascaded": "Cascade",
    "mutation.unknown-palette-color": "Invariant", "mutation.malformed-payload": "Invariant", "mutation.colour-in-use": "Invariant",
    "wfc3d.tile.non-positive-weight": "Invariant", "wfc3d.slot.degenerate-box": "Invariant", "wfc3d.edge.self-loop": "Invariant",
    "wfc3d.rule.unknown-tile": "Invariant", "wfc3d.slot.unknown-pinned-tile": "Invariant",
    "mutation.invalid-reconstruction-sparse": "Invariant", "mutation.invalid-reconstruction-mesh": "Invariant",
    "mutation.invalid-reconstruction-asset": "Invariant", "mutation.invalid-content-chunk": "Invariant",
    "mutation.content-gap": "Invariant", "mutation.content-kind-mismatch": "Invariant", "mutation.content-conflict": "Invariant", "mutation.content-capacity": "Invariant",
    "mutation.referenced": "Invariant", "mutation.incomplete-mesh": "Invariant", "mutation.invalid-asset-payload": "Invariant",
    "mutation.blend-mode-invalid": "Invariant", "mutation.invalid-text-size": "Invariant", "mutation.invalid-geometry": "Invariant",
    "mutation.child-identity": "Invariant", "mutation.id-mismatch": "Invariant", "mutation.target-referenced": "Invariant",
    "forms.invalid-response": "Invariant", "mutation.invalid-number": "Invariant", "s.home.local-studio-tombstone-refused": "Invariant",
    "invalid-remove-index": "Invariant", "invalid-modify-index": "Invariant", "invalid-add-index": "Invariant",
    "invalid-remove-target": "Invariant", "invalid-modify-target": "Invariant", "invalid-add-target": "Invariant",
    "invalid-upsert-target": "Invariant", "invalid-instance-order": "Invariant", "duplicate-base-target": "DuplicateId",
    "mutation.absorb.kind-mismatch": "Invariant", "mutation.inverse.kind-mismatch": "Invariant",
    **{f"stdio.pdf.{verb}.invalid-target": "Invariant" for verb in ("collapse-page-size", "set-page-size", "set-page-text", "clear-page-text",
                                                                     "replace-page-text", "resize-page", "insert-page", "move-page", "remove-page")},
}

SITES = json.loads((HERE / "ledger-1.json").read_text())


FROZEN_CODE = {"TargetMissing": "mutation.target-missing", "NoOp": "mutation.no-op", "Partial": "mutation.partial", "Clamped": "mutation.clamped",
               "DuplicateId": "mutation.duplicate-id", "Invariant": "mutation.invariant", "Cascade": "mutation.cascade"}
RENAME_SUFFIXES = (".json", ".py", ".ts", ".feature", ".md")
RENAME_SKIP = {"target", "node_modules", "dist", "🤖️generated"}


def load_decisions() -> tuple[dict, list, list, list, list]:
    """📥️ Per-site overrides `{(path, line): {variant?, target?, at?, skip?}}`, exact edits `[(path, old, new)]` and code renames
    `[(scope dir, {drift code: variant})]` (tests, fixtures, references, twins — never non-test Rust) and whole-file transforms
    `[(path, fn(text) -> text)]`, and doc-comment renames `[(scope dir, {drift code: variant})]` (only `//` lines of
    non-test Rust) from `edits/*.py`."""
    overrides, edits, renames, transforms, docs = {}, [], [], [], []
    for module_path in sorted((HERE / "edits").glob("*.py")):
        scope: dict = {}
        exec(module_path.read_text(), scope)
        for (rel, line), decision in scope.get("OVERRIDES", {}).items():
            overrides[(PLUGINS + rel, line)] = decision
        edits += [(PLUGINS + rel, old, new) for rel, old, new in scope.get("EDITS", [])]
        renames += [(PLUGINS + rel, table) for rel, table in scope.get("RENAMES", [])]
        transforms += [(PLUGINS + rel, fn) for rel, fn in scope.get("TRANSFORMS", [])]
        docs += [(PLUGINS + rel, table) for rel, table in scope.get("DOC_RENAMES", [])]
    return overrides, edits, renames, transforms, docs


def rename_targets(scope: str) -> list[str]:
    """🗂️ Files a code rename may touch under `scope`: fixtures, references, twins, feature prose and Rust test files."""
    out = []
    for path in (OVERLAY / scope).rglob("*"):
        rel = str(path.relative_to(OVERLAY))
        if any(part in RENAME_SKIP for part in path.parts) or not path.is_file():
            continue
        if path.suffix in RENAME_SUFFIXES or (path.name == "🦀️.rs" and "🧪️tests" in path.parts):
            out.append(rel)
    return sorted(out)


def rename(text: str, table: dict) -> tuple[str, int]:
    """🔁️ Every exact-token occurrence of a drift code → its frozen code string; inside a Gherkin table cell the trailing
    padding absorbs the length change so the column stays aligned."""
    count = 0
    for code, variant in table.items():
        frozen = FROZEN_CODE[variant]

        def cell(match: re.Match) -> str:
            pad = match.group("pad")
            if pad is None:
                return frozen
            return frozen + " " * max(1, len(pad) + len(code) - len(frozen))

        text, hits = re.subn(r"(?<![A-Za-z0-9._-])" + re.escape(code) + r"(?![A-Za-z0-9_-])(?:(?P<pad> +)(?=\|))?", cell, text)
        count += hits
    return text, count


def call_at(text: str, line: int):
    """🔎️ The mutation-report call starting on `line` (kind, match, argument spans, end), or `None`."""
    start = sum(len(row) + 1 for row in text.split("\n")[: line - 1])
    stop = text.find("\n", start)
    for kind, pattern, _ in codemod.CALLS:
        for match in pattern.finditer(text, start, len(text)):
            if match.start() > stop:
                break
            parsed = codemod.arguments(text, match.end() - 1)
            if parsed is None:
                continue
            spans, end = parsed
            return kind, match, spans, end
    return None


def rewrite(rel: str, text: str, entry: dict, decision: dict) -> tuple[str, str]:
    """✏️ One ledger site → the frozen-code call; answers (text, status)."""
    if decision.get("skip"):
        return text, "hand"
    found = call_at(text, entry["line"])
    if found is None:
        return text, "applied?"
    kind, match, spans, end = found
    first = text[spans[0][0]:spans[0][1]].strip()
    if re.fullmatch(r"(?:[A-Za-z_]\w*::)*MutationCode::\w+", first):
        return text, "applied"
    variant = decision.get("variant") or VARIANT_OF_CODE.get(entry["code"])
    if variant is None:
        return text, "undecided"
    qualifiers = re.findall(r"(?<![\w:])((?:[A-Za-z_]\w*::)+)Mutation(?:Outcome|Message|ApplyError)\b", text)
    qual = match.group("qual") or (max(set(qualifiers), key=qualifiers.count) if qualifiers else "")
    code = f"{qual}MutationCode::{variant}"
    method = match.group("method")
    if kind == "outcome":
        target = decision.get("target") or text[spans[2][0]:spans[2][1]].strip()
        replacement = (match.start(), end, f"{match.group(0)}{code}, {target})")
    elif kind in ("message", "apply"):
        replacement = (match.start(), end, f"{match.group(0)}{code})")
    elif decision.get("at"):
        replacement = (match.start(), end, f".absorb_messages([{qual}MutationMessage::{method}({code}).at(vec![{decision['at']}])])")
    else:
        replacement = (match.start(), end, f".{method}({code})")
    start, stop, new = replacement
    text = text[:start] + new + text[stop:]
    if not qual:
        text, imported = codemod.add_import(text)
        if not imported:
            return text, "unimported"
    return text, "converted"


def main(dry_run: bool, needle: str) -> None:
    overrides, edits, renames, transforms, docs = load_decisions()
    by_file: dict[str, list[dict]] = {}
    for entry in SITES:
        if needle in entry["path"]:
            by_file.setdefault(entry["path"], []).append(entry)
    tally: dict[str, int] = {}
    problems = []
    touched: dict[str, str] = {}
    for rel, entries in by_file.items():
        text = touched.get(rel) or (OVERLAY / rel).read_text()
        for entry in sorted(entries, key=lambda e: -e["line"]):
            text, status = rewrite(rel, text, entry, overrides.get((rel, entry["line"]), {}))
            tally[status] = tally.get(status, 0) + 1
            if status in ("applied?", "undecided", "unimported"):
                problems.append(f"{status} {rel}:{entry['line']} {entry['code']}")
        touched[rel] = text
    for rel, old, new in edits:
        if needle not in rel:
            continue
        text = touched.get(rel) or (OVERLAY / rel).read_text()
        if old in text and not (old in new and new in text):
            text = text.replace(old, new)
            tally["edit"] = tally.get("edit", 0) + 1
        elif not new or new in text:
            tally["edit-applied"] = tally.get("edit-applied", 0) + 1
        else:
            problems.append(f"edit-missing {rel}: {old[:90]!r}")
        touched[rel] = text
    for rel, transform in transforms:
        if needle not in rel:
            continue
        text = touched.get(rel) or (OVERLAY / rel).read_text()
        new = transform(text)
        tally["transform" if new != text else "transform-applied"] = tally.get("transform" if new != text else "transform-applied", 0) + 1
        touched[rel] = new
    for scope, table in renames:
        if needle not in scope:
            continue
        for rel in rename_targets(scope):
            text = touched.get(rel) or (OVERLAY / rel).read_text()
            text, hits = rename(text, table)
            if hits:
                tally["renamed"] = tally.get("renamed", 0) + hits
                touched[rel] = text
    for scope, table in docs:
        if needle not in scope:
            continue
        for path in sorted((OVERLAY / scope).rglob("🦀️.rs")):
            rel = str(path.relative_to(OVERLAY))
            if any(part in RENAME_SKIP for part in path.parts) or "🧪️tests" in path.parts:
                continue
            text = touched.get(rel) or path.read_text()
            lines, hits = text.split("\n"), 0
            for at, line in enumerate(lines):
                if line.lstrip().startswith("//"):
                    lines[at], count = rename(line, table)
                    hits += count
            if hits:
                tally["doc-renamed"] = tally.get("doc-renamed", 0) + hits
                touched[rel] = "\n".join(lines)
    changed = 0
    for rel, text in touched.items():
        if text != (OVERLAY / rel).read_text():
            changed += 1
            if dry_run and "--list" in sys.argv:
                print("  would write", rel)
            if not dry_run:
                keep = BACKUP / rel
                if not keep.exists():
                    keep.parent.mkdir(parents=True, exist_ok=True)
                    keep.write_bytes((OVERLAY / rel).read_bytes())
                (OVERLAY / rel).write_text(text)
    print(f"{'dry-run' if dry_run else 'written'}: files changed={changed} {tally}")
    for problem in problems:
        print("  ", problem)


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    main("--dry-run" in sys.argv[1:], args[0] if args else "")
