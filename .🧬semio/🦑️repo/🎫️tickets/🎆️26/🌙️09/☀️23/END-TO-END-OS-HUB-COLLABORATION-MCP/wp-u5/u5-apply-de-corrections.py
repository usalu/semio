#!/usr/bin/env python3
"""✍️ U5 one-off codemod: writes the handcrafted German (and, where the English was a field-name dump, English)
mutation-label literals from `u5-de-corrections.txt` into the leaves, and turns kebab-case English op ids into
sentence-case English. Region-anchored exactly like `u5-label-audit.py` (trait `impl … for` + `fn label -> …LocalizedLabel`),
span-keyed, descending, literal-only. Every rewritten literal must keep its placeholder sequence and be a valid Rust
string body; any correction that matches no site aborts the run (a typo is never silently dropped).

Usage: python3 u5-apply-de-corrections.py <repoRoot> <corrections.txt> <reportJson> [--apply]
"""
import json
import os
import re
import sys

ROOT, CORRECTIONS, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
APPLY = "--apply" in sys.argv
SCAN = ["✏️s", "🧰️framework"]
SKIP_DIRS = {"node_modules", "dist", "target", ".git", "🗑️generated"}
IMPL_RE = re.compile(r"impl\b[^{;]*?\b(MutationKind|CompositeMutationKind|SemanticMutation)\b[^{;]*?\bfor\b")
FN_RE = re.compile(r"fn\s+label\s*\(\s*&\s*self\s*\)\s*->\s*[\w:]*LocalizedLabel\s*\{")
STR_RE = re.compile(r'"(?:\\.|[^"\\])*"')
PLACEHOLDER = re.compile(r"\{[^{}]*\}")
KEBAB = re.compile(r"[a-z0-9]+(?:[- ][a-z0-9]+)*")
ACRONYMS = {"vlr": "VLR", "id3v1": "ID3v1", "id3v2": "ID3v2", "jfif": "JFIF", "sof": "SOF", "vml": "VML", "icc": "ICC", "dst": "DST", "srgb": "sRGB", "ifd": "IFD", "bcf": "BCF", "dims": "dimensions", "re": "re-", "comments1": "comments 1", "comments2": "comments 2"}


def strip(text: str) -> str:
    return PLACEHOLDER.sub("", text).strip()


def valid_body(text: str) -> bool:
    index = 0
    while index < len(text):
        if text[index] == "\\":
            if index + 1 >= len(text) or text[index + 1] not in '"\\nt':
                return False
            index += 2
            continue
        if text[index] == '"':
            return False
        index += 1
    return True


def parse_corrections(path: str) -> dict:
    table: dict = {}
    crate = None
    for number, raw in enumerate(open(path, encoding="utf-8"), 1):
        line = raw.rstrip("\n")
        if line.startswith("## "):
            crate = line[3:].strip()
            continue
        if not line.strip() or line.startswith("#"):
            continue
        if "  →  " not in line or crate is None:
            raise SystemExit(f"corrections:{number}: malformed line: {line}")
        en, rest = line.split("  →  ", 1)
        new_en = None
        if "  ⟸  " in rest:
            rest, new_en = rest.split("  ⟸  ", 1)
        key = (crate, tuple(part for part in en.split(" ‖ ")))
        if key in table:
            raise SystemExit(f"corrections:{number}: duplicate key {key}")
        table[key] = {"de": rest.split(" ‖ "), "en": new_en.split(" ‖ ") if new_en else None, "line": number, "hits": 0}
    return table


def block_end(text: str, open_index: int) -> int:
    depth, index, in_str = 0, open_index, False
    while index < len(text):
        char = text[index]
        if in_str:
            if char == "\\":
                index += 2
                continue
            if char == '"':
                in_str = False
        elif char == '"':
            in_str = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return len(text) - 1


def native_args(body: str, base: int) -> list[tuple[tuple[int, int], tuple[int, int]]]:
    calls = []
    for match in re.finditer(r"LocalizedLabel::native\s*\(", body):
        index, depth, in_str, start, comma = match.end(), 1, False, match.end(), None
        while index < len(body) and depth > 0:
            char = body[index]
            if in_str:
                if char == "\\":
                    index += 2
                    continue
                if char == '"':
                    in_str = False
            elif char == '"':
                in_str = True
            elif char in "([{":
                depth += 1
            elif char in ")]}":
                depth -= 1
            elif char == "," and depth == 1 and comma is None:
                comma = index
            index += 1
        if comma is not None:
            calls.append(((base + start, base + comma), (base + comma + 1, base + index - 1)))
    return calls


def literal_spans(text: str, span: tuple[int, int]) -> list[tuple[int, int, str]]:
    out = []
    for match in STR_RE.finditer(text, span[0], span[1]):
        body = match.group(0)[1:-1]
        if strip(body):
            out.append((match.start() + 1, match.end() - 1, body))
    return out


def english_from_kebab(literal: str) -> str:
    words = [ACRONYMS.get(word, word) for word in re.split(r"[- ]", literal)]
    sentence = " ".join(words).replace("re- ", "re-")
    return sentence[:1].upper() + sentence[1:]


OWNERS: list[tuple[str, str]] = []
for scan in SCAN:
    for current, dirs, files in os.walk(os.path.join(ROOT, scan)):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "Cargo.toml" in files and current.endswith(os.path.join("📦️packages", "🦀️rust")):
            text = open(os.path.join(current, "Cargo.toml"), encoding="utf-8").read()
            name = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M)
            if "[package]" in text and name:
                OWNERS.append((os.path.dirname(os.path.dirname(current)) + os.sep, name.group(1)))
OWNERS.sort(key=lambda owner: -len(owner[0]))


def crate_of(path: str) -> str:
    return next((name for prefix, name in OWNERS if path.startswith(prefix)), "?")


table = parse_corrections(CORRECTIONS)
edits_by_file: dict[str, list[tuple[int, int, str]]] = {}
report = {"sites": 0, "corrected": 0, "kebabEnglish": 0, "crates": {}, "errors": []}
for scan in SCAN:
    for current, dirs, files in os.walk(os.path.join(ROOT, scan)):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "🦀️.rs" not in files:
            continue
        path = os.path.join(current, "🦀️.rs")
        text = open(path, encoding="utf-8").read()
        if "fn label" not in text:
            continue
        for match in FN_RE.finditer(text):
            if not any(True for _ in IMPL_RE.finditer(text, 0, match.start())):
                continue
            open_index = match.end() - 1
            end = block_end(text, open_index)
            calls = native_args(text[open_index : end + 1], open_index)
            if not calls:
                continue
            report["sites"] += 1
            en_spans = [span for call in calls for span in literal_spans(text, call[0])]
            de_spans = [span for call in calls for span in literal_spans(text, call[1])]
            en_lits = tuple(span[2] for span in en_spans)
            crate = crate_of(path)
            entry = table.get((crate, en_lits))
            site = f"{os.path.relpath(path, ROOT)}:{text.count(chr(10), 0, match.start()) + 1}"
            edits = edits_by_file.setdefault(path, [])
            if entry is not None:
                entry["hits"] += 1
                if len(entry["de"]) != len(de_spans):
                    report["errors"].append(f"{site}: correction line {entry['line']} carries {len(entry['de'])} German literals, site has {len(de_spans)}")
                    continue
                for (start, stop, old), new in zip(de_spans, entry["de"]):
                    if PLACEHOLDER.findall(old) != PLACEHOLDER.findall(new) or not valid_body(new):
                        report["errors"].append(f"{site}: line {entry['line']} German `{new}` breaks placeholders/escaping of `{old}`")
                    elif old != new:
                        edits.append((start, stop, new))
                if entry["en"] is not None:
                    if len(entry["en"]) != len(en_spans):
                        report["errors"].append(f"{site}: line {entry['line']} English literal count mismatch")
                        continue
                    for (start, stop, old), new in zip(en_spans, entry["en"]):
                        if PLACEHOLDER.findall(old) != PLACEHOLDER.findall(new) or not valid_body(new):
                            report["errors"].append(f"{site}: line {entry['line']} English `{new}` breaks placeholders/escaping of `{old}`")
                        elif old != new:
                            edits.append((start, stop, new))
                report["corrected"] += 1
                report["crates"].setdefault(crate, 0)
                report["crates"][crate] += 1
            if (entry is None or entry["en"] is None) and en_spans and all(KEBAB.fullmatch(span[2]) for span in en_spans):
                for start, stop, old in en_spans:
                    edits.append((start, stop, english_from_kebab(old)))
                report["kebabEnglish"] += 1
                report["crates"].setdefault(crate, 0)
                report["crates"][crate] += 1

for (crate, en), entry in table.items():
    if entry["hits"] == 0:
        report["errors"].append(f"correction line {entry['line']} ({crate}: {' ‖ '.join(en)}) matched no site")

changed_files = 0
for path, edits in edits_by_file.items():
    if not edits:
        continue
    text = open(path, encoding="utf-8").read()
    for start, stop, new in sorted(edits, key=lambda edit: -edit[0]):
        text = text[:start] + new + text[stop:]
    changed_files += 1
    if APPLY and not report["errors"]:
        open(path, "w", encoding="utf-8").write(text)
report["changedFiles"] = changed_files
report["applied"] = APPLY and not report["errors"]
report["files"] = sorted(os.path.relpath(path, ROOT) for path, edits in edits_by_file.items() if edits)
json.dump(report, open(REPORT, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({key: report[key] for key in ("sites", "corrected", "kebabEnglish", "changedFiles", "applied")}, ensure_ascii=False), f"errors={len(report['errors'])}")
for error in report["errors"][:60]:
    print(" ", error)
