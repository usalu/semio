"""🧱️ Structure of the four energy quiz content files against HEAD: only the `{en, de}` texts may differ.

- Everything outside the `en` / `de` string literals is identical character for character (ids, numbers, `draw`, layout).
- The parsed files have the same JSON paths in the same order and equal non-text values of the same type.
- Every text has a non-empty English and German string.

--changes lists every changed text as Markdown: the hand-made changes (wording, thousands separators) as table rows
with the changed fragment, and the typography rules as counts with examples. No-break spaces are shown as ⍽.

Usage: python ui_content_structure_check.py [--changes]
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from collections import Counter
from os.path import commonprefix
from pathlib import Path

sys.dont_write_bytecode = True

from ui_content_typography_check import LANGUAGES, NBSP, OPERATORS, ROOT, TEXT, quiz_files, rules, texts, unit_symbols

GROUPED = {"en": re.compile(r"(?<![0-9.,])[0-9]{1,3}(?:,[0-9]{3})+"), "de": re.compile(r"(?<![0-9.,])[0-9]{1,3}(?:\.[0-9]{3})+(?![0-9.])")}
SEPARATOR = re.compile(rf"(?<=[0-9])[,. {NBSP}](?=[0-9]{{3}}(?![0-9]))")
EXAMPLES = 3


def head(path: Path) -> str:
    """🕰️ The committed version of a file, read through `git show` (nothing is modified)."""
    shown = subprocess.run(["git", "show", f"HEAD:{path.relative_to(ROOT).as_posix()}"], cwd=ROOT, capture_output=True, check=True)
    return shown.stdout.decode("utf-8")


def leaves(node: object, path: str = "") -> list[tuple[str, object]]:
    """🍃️ Every scalar of a JSON document with its path, in document order."""
    if isinstance(node, dict):
        return [leaf for key, value in node.items() for leaf in leaves(value, f"{path}.{key}" if path else key)]
    if isinstance(node, list):
        return [leaf for index, value in enumerate(node) for leaf in leaves(value, f"{path}[{index}]")]
    return [(path, node)]


def skeleton(raw: str) -> str:
    """🦴️ A file without the contents of its `en` / `de` string literals and with one newline style."""
    return TEXT.sub(r"\1\4", raw.replace("\r\n", "\n"))


def names(quiz: dict, path: str) -> str:
    """🏷️ The ids of the list elements a JSON path walks through, e.g. `energies/world-primary-energy`."""
    node: object = quiz
    found: list[str] = []
    for key, index in re.findall(r"([A-Za-z]+)(?:\[([0-9]+)\])?", path):
        node = node[key]
        if index:
            node = node[int(index)]
            found.append(node["id"])
    return "/".join(found)


def structure_findings(old_raw: str, new_raw: str) -> list[str]:
    """🚨️ Everything that differs outside the texts, and every empty text."""
    found: list[str] = []
    if skeleton(old_raw) != skeleton(new_raw):
        pairs = zip(skeleton(old_raw).split("\n"), skeleton(new_raw).split("\n"))
        found += [f"line {number} differs outside a text: {new}" for number, (old, new) in enumerate(pairs, 1) if old != new] or ["line count differs"]
    old, new = json.loads(old_raw), json.loads(new_raw)
    old_leaves, new_leaves = leaves(old), leaves(new)
    if [path for path, _ in old_leaves] != [path for path, _ in new_leaves]:
        found.append("JSON paths differ: " + ", ".join(sorted({path for path, _ in old_leaves} ^ {path for path, _ in new_leaves})))
    text_paths = {f"{path}.{language}" for path, _ in texts(new) for language in LANGUAGES}
    if text_paths != {f"{path}.{language}" for path, _ in texts(old) for language in LANGUAGES}:
        found.append("the set of texts differs")
    before = dict(old_leaves)
    for path, value in new_leaves:
        if path in text_paths:
            if not isinstance(value, str) or not value.strip():
                found.append(f"{path}: empty text")
        elif path in before and (type(value) is not type(before[path]) or value != before[path]):
            found.append(f"{path}: {before[path]!r} became {value!r}")
    return found


def typeset(text: str, language: str, symbols: tuple[str, ...]) -> str:
    """🔤️ A text after the mechanical rules of the typography script."""
    for _, pattern, replacement in rules(symbols, language):
        text = pattern.sub(replacement, text)
    return text


def fragment(old: str, new: str) -> tuple[str, str]:
    """✂️ The differing middle of two texts, widened to whole words."""
    start = len(commonprefix([old, new]))
    end = len(commonprefix([old[start:][::-1], new[start:][::-1]]))
    start = max(old.rfind(" ", 0, start), old.rfind(NBSP, 0, start)) + 1
    old_end, new_end = len(old) - end, len(new) - end
    while old_end < len(old) and old[old_end] not in f" {NBSP}":
        old_end, new_end = old_end + 1, new_end + 1
    return old[start:old_end], new[start:new_end]


def shown(text: str) -> str:
    """👁️ A text for a Markdown table cell, no-break spaces visible."""
    return text.replace(NBSP, "⍽").replace("|", "\\|")


def changes(topic: str, old: dict, new: dict, symbols: tuple[str, ...]) -> list[str]:
    """📝️ The Markdown section of one file: hand-made changes and counts of the typography rules."""
    rows: list[str] = []
    counts: Counter[str] = Counter()
    examples: dict[str, list[str]] = {}

    def count(rule: str, amount: int, example: str | None = None) -> None:
        counts[rule] += amount
        if amount > 0 and example is not None and len(examples.setdefault(rule, [])) < EXAMPLES and example not in examples[rule]:
            examples[rule].append(example)

    changed = 0
    for (path, before), (_, after) in zip(texts(old), texts(new)):
        for language in LANGUAGES:
            was, now = before[language], after[language]
            if was == now:
                continue
            changed += 1
            fixed = typeset(was, language, symbols)
            if fixed != now:
                kind = "thousands" if SEPARATOR.sub("", fixed) == SEPARATOR.sub("", now) else "wording"
                rows.append(f"| `{path}.{language}` ({names(new, path)}) | {language} | {kind} | " + " | ".join(map(shown, fragment(fixed, now))) + " |")
            for match in re.finditer(NBSP, now):
                around = shown(now[max(0, match.start() - 12) : match.end() + 12])
                if now[match.end()] == "%":
                    count(f"{language} percent", 1, around)
                elif now[match.start() - 1] in OPERATORS or now[match.end()] in OPERATORS:
                    count(f"{language} no-break space at operator", 1, around)
                else:
                    count(f"{language} no-break space before unit", 1, around)
            if language == "en":
                percent = re.search(r"[0-9]%", now)
                count("en percent", len(re.findall(r"[0-9]%", now)) - len(re.findall(r"[0-9]%", was)), shown(now[max(0, percent.start() - 12) : percent.end() + 12]) if percent else None)
            grouped = [match[0] for match in GROUPED[language].finditer(now) if match[0] not in was]
            for number in grouped:
                count(f"{language} thousands separator", 1, number)
            dash = was.find(" — ")
            count(f"{language} em dash to en dash", was.count("—") - now.count("—"), shown(was[max(0, dash - 12) : dash + 15]) if dash >= 0 else None)
            count(f"{language} minus sign", now.count("−") - was.count("−"))
    lines = [f"### {topic}", "", f"{changed} of {2 * len(texts(new))} strings changed.", ""]
    lines += ["| JSON path | language | kind | old | new |", "|---|---|---|---|---|", *rows, ""]
    lines += ["| rule | count | examples (new text) |", "|---|---|---|"]
    lines += [f"| {rule} | {amount} | {' · '.join(f'`{example}`' for example in examples.get(rule, []))} |" for rule, amount in sorted(counts.items()) if amount]
    return [*lines, ""]


def main() -> int:
    """🚦️ Compares the four files with HEAD; exit code 1 when anything but a text differs."""
    sys.stdout.reconfigure(encoding="utf-8")
    files = quiz_files()
    symbols = unit_symbols([json.loads(path.read_text(encoding="utf-8")) for path in files])
    total = 0
    for path in files:
        topic = path.parents[1].name
        old_raw, new_raw = head(path), path.read_text(encoding="utf-8", newline="")
        found = structure_findings(old_raw, new_raw)
        total += len(found)
        if "--changes" in sys.argv[1:]:
            print("\n".join(changes(topic, json.loads(old_raw), json.loads(new_raw), symbols)))
        newlines = "CRLF" if new_raw.count("\r\n") == new_raw.count("\n") else "LF" if "\r" not in new_raw else "mixed"
        print(f"{topic}: {len(leaves(json.loads(new_raw)))} JSON paths, {len(texts(json.loads(new_raw)))} texts, newlines {newlines}, {len(found)} structure findings")
        for line in found:
            print(f"  FINDING {line}")
    print("OK" if total == 0 else f"FAILED: {total} findings")
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
