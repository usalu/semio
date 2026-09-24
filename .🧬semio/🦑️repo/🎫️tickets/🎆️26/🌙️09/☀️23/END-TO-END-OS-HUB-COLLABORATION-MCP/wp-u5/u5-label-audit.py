#!/usr/bin/env python3
"""🔎️ U5 audit: does every `MutationKind`/`SemanticMutation`/`CompositeMutationKind` label carry real German?

Walks every `🦀️.rs` under `✏️s` and `🧰️framework`, finds each trait-anchored `fn label(&self) -> …LocalizedLabel`,
splits every `LocalizedLabel::native(en, de)` call at its top-level comma and classifies the German side:

- `identical`   — the German literal text equals the English literal text (an untranslated copy);
- `english`     — the German literals contain English function words or verbs (a transliteration/leak);
- `kebab`       — the English side is a kebab-case op id rather than prose (reported, en quality);
- `data`        — `LocalizedLabel::data(…)` inside a mutation label (locale-invariant by declaration);
- `other`       — the body resolves the label some other way (delegation, helper) and is listed for review.

Usage: python3 u5-label-audit.py <repoRoot> <outJson>
"""
import json
import os
import re
import sys

ROOT = sys.argv[1]
OUT = sys.argv[2]
SCAN = ["✏️s", "🧰️framework"]
SKIP_DIRS = {"node_modules", "dist", "target", ".git", "🗑️generated"}
IMPL_RE = re.compile(r"impl\b[^{;]*?\b(MutationKind|CompositeMutationKind|SemanticMutation)\b[^{;]*?\bfor\b")
FN_RE = re.compile(r"fn\s+label\s*\(\s*&\s*self\s*\)\s*->\s*[\w:]*LocalizedLabel\s*\{")
STR_RE = re.compile(r'r#*"(?:[^"]|"(?!#))*"#*|"(?:\\.|[^"\\])*"')
ENGLISH = re.compile(r"\b(Set|Add|Remove|Delete|Update|Change|Move|Rename|Create|Clear|Toggle|Insert|Replace|Select|Apply|Reset|Edit|Import|Export|Enable|Disable|Show|Hide|Assign|Unassign|Attach|Detach|Connect|Disconnect|Split|Merge|Link|Unlink|Resize|Rotate|Scale|Translate|Duplicate|Paste|Copy|Cut|Group|Ungroup|Lock|Unlock|the|to|of|and|with|for|from|into|on|at|by|new|all|selected|value|name|node|layer|point|row|column)\b")
CAMEL = re.compile(r"[a-zäöüß][A-Z]")
PLURAL_HACK = re.compile(r"\(s\)|\(n\)|\(en\)")


def literals(text: str) -> list[str]:
    out = []
    for match in STR_RE.finditer(text):
        raw = match.group(0)
        if raw.startswith("r"):
            body = raw[raw.index('"') + 1 : raw.rindex('"')]
        else:
            body = raw[1:-1]
        out.append(body)
    return out


def block_end(text: str, open_index: int) -> int:
    depth = 0
    index = open_index
    in_str = False
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


def split_native_calls(body: str) -> list[tuple[str, str]]:
    calls = []
    for match in re.finditer(r"LocalizedLabel::native\s*\(", body):
        index = match.end()
        depth = 1
        in_str = False
        start = index
        comma = None
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
        if comma is None:
            continue
        calls.append((body[start:comma], body[comma + 1 : index - 1]))
    return calls


def strip_placeholders(text: str) -> str:
    return re.sub(r"\{[^{}]*\}", "", text).strip()


def classify(body: str) -> tuple[str, list[str], list[str]]:
    calls = split_native_calls(body)
    if not calls:
        return ("data" if "LocalizedLabel::data" in body else "other", [], [])
    en_all: list[str] = []
    de_all: list[str] = []
    verdict = "ok"
    for en, de in calls:
        en_lits = [value for value in literals(en) if strip_placeholders(value)]
        de_lits = [value for value in literals(de) if strip_placeholders(value)]
        en_all += en_lits
        de_all += de_lits
        if en_lits and en_lits == de_lits:
            verdict = "identical"
        elif verdict == "ok" and any(ENGLISH.search(strip_placeholders(value)) for value in de_lits):
            verdict = "english"
        elif verdict == "ok" and any(CAMEL.search(strip_placeholders(value)) for value in de_lits):
            verdict = "camel"
        elif verdict == "ok" and any(PLURAL_HACK.search(value) for value in de_lits):
            verdict = "plural"
        elif verdict == "ok" and any("Stütze" in value for value in de_lits) and any(re.search(r"\b(table|row|cell|sheet|csv|grid)\b", value, re.I) for value in en_lits):
            verdict = "column"
    if verdict == "ok" and en_all and all(re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)+", strip_placeholders(value).split(" ")[0] or "") for value in en_all):
        verdict = "kebab"
    return verdict, en_all, de_all


OWNERS: list[tuple[str, str]] = []
for scan in SCAN:
    for current, dirs, files in os.walk(os.path.join(ROOT, scan)):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "Cargo.toml" in files and current.endswith(os.path.join("📦️packages", "🦀️rust")):
            with open(os.path.join(current, "Cargo.toml"), encoding="utf-8") as handle:
                text = handle.read()
            name = re.search(r'^name\s*=\s*"([^"]+)"', text, re.M)
            if "[package]" in text and name:
                OWNERS.append((os.path.dirname(os.path.dirname(current)) + os.sep, name.group(1)))
OWNERS.sort(key=lambda owner: -len(owner[0]))


def crate_of(path: str) -> str:
    for prefix, name in OWNERS:
        if path.startswith(prefix):
            return name
    return "?"


rows = []
for scan in SCAN:
    for current, dirs, files in os.walk(os.path.join(ROOT, scan)):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        if "🦀️.rs" not in files:
            continue
        path = os.path.join(current, "🦀️.rs")
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        if "fn label" not in text:
            continue
        for match in FN_RE.finditer(text):
            header = None
            for impl in IMPL_RE.finditer(text, 0, match.start()):
                header = impl
            if header is None:
                continue
            open_index = match.end() - 1
            end = block_end(text, open_index)
            body = text[open_index : end + 1]
            verdict, en, de = classify(body)
            line = text.count("\n", 0, match.start()) + 1
            rows.append({"file": os.path.relpath(path, ROOT), "line": line, "trait": header.group(1), "verdict": verdict, "en": en, "de": de, "body": body if verdict in ("other", "data") else ""})

crates: dict[str, dict[str, int]] = {}
for row in rows:
    row["crate"] = crate_of(os.path.join(ROOT, row["file"]))
    crates.setdefault(row["crate"], {}).setdefault(row["verdict"], 0)
    crates[row["crate"]][row["verdict"]] += 1
totals: dict[str, int] = {}
for row in rows:
    totals[row["verdict"]] = totals.get(row["verdict"], 0) + 1
with open(OUT, "w", encoding="utf-8") as handle:
    json.dump({"totals": totals, "sites": len(rows), "crates": crates, "rows": rows}, handle, ensure_ascii=False, indent=1)
print(json.dumps({"sites": len(rows), "totals": totals}, ensure_ascii=False))
