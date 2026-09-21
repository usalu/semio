#!/usr/bin/env python3
"""🇩🇪️ Rewrites every `MutationKind::label()` to return a bilingual `LocalizedLabel`.

Invariants, in the order they are checked — any failure aborts before a single file is written:
 1. 🎯️ Region anchor: a `fn label` is rewritten only when the nearest preceding `impl … for …`
    header names `MutationKind`, `CompositeMutationKind` or `SemanticMutation`. Inherent `label`
    methods on unrelated types are never touched.
 2. 🔁️ Span-keyed edits: each site is rewritten by its own byte span, descending, so no name-keyed
    text match can reach unrelated code (2026-09-03 codemod incident).
 3. 🗺️ Exhaustive table: every string literal inside a rewritten body must have an entry in
    `🐍️u3-de-glossary.json`. A miss aborts — there is no English pass-through.
 4. 🧮️ Path derivation: `LocalizedLabel` is qualified with the same module prefix the file already
    uses for the mutation trait, so no `use` line is edited.
 5. ♻️ Idempotence: a file already carrying `fn label(&self) -> LocalizedLabel` contributes no site,
    so a second run is a no-op.

`--retranslate` is the second pass, for AFTER the migration has landed: it re-derives the German
argument of every already-migrated site from the ENGLISH argument in the file and the CURRENT
glossary, and rewrites the German only where the two disagree. The glossary is therefore the single
source of truth for the German side — correcting a translation is a one-line edit there plus this
pass, never a hand edit in 2791 leaves. It obeys the same region anchor and span-keyed rules, and it
is idempotent for the same reason: a second run finds every German already equal to the table's.
"""
import json, os, re, sys, collections

HERE = os.path.dirname(os.path.abspath(__file__))
ROOTS = ["✏️s", "🧰️framework"]
TRAITS = ("MutationKind", "CompositeMutationKind", "SemanticMutation")
SIGNATURE = "fn label(&self) -> String"
MIGRATED = re.compile(r"fn label\(&self\) -> (?:[\w:]*::)?LocalizedLabel\b")
LITERAL = re.compile(r'"(?:[^"\\]|\\.)*"')


def find_repo(start):
    """🌳️ Walks up to the repository root (the directory owning `🧰️framework`)."""
    node = start
    while node != os.path.dirname(node):
        if os.path.isdir(os.path.join(node, "🧰️framework")):
            return node
        node = os.path.dirname(node)
    raise SystemExit("repository root not found")


def trait_prefix(source, header):
    """🧮️ The module path this file already spells the mutation trait with, e.g. `protocol::`."""
    match = re.search(r"impl(?:<.*?>)?\s+((?:[\w:]+::)?)(%s)\b" % "|".join(TRAITS), header)
    if match and match.group(1):
        # 🌐️ The kernel re-exports the carrier at its ROOT, not from the `os_spr` mutation module.
        return "crate::" if match.group(1) == "crate::os_spr::" else match.group(1)
    for used in re.findall(r"^\s*use\s+([^;]+);", source, re.M):
        if "MutationKind" in used:
            return used.split("::{")[0].split("::MutationKind")[0].strip() + "::"
    raise SystemExit(f"no module path for LocalizedLabel in header {header!r}")


def sites(source):
    """🎯️ `(start, end, prefix, body)` for every anchored `fn label` in one file."""
    lines = source.split("\n")
    offsets, cursor = [], 0
    for line in lines:
        offsets.append(cursor)
        cursor += len(line) + 1
    header = None
    found = []
    for index, line in enumerate(lines):
        if re.match(r"^\s*impl\b", line):
            header = line.strip()
        if SIGNATURE not in line:
            continue
        if header is None or " for " not in header or not any(t in header for t in TRAITS):
            continue
        prefix = trait_prefix(source, header)
        depth = line.count("{") - line.count("}")
        if depth == 0:
            body = line.split("{", 1)[1].rsplit("}", 1)[0].strip()
            found.append((offsets[index], offsets[index] + len(line), prefix, body, line[: len(line) - len(line.lstrip())]))
            continue
        collected, scan = [], index
        while depth > 0:
            scan += 1
            collected.append(lines[scan])
            depth += lines[scan].count("{") - lines[scan].count("}")
        body = "\n".join(collected[:-1])
        found.append((offsets[index], offsets[scan] + len(lines[scan]), prefix, body, line[: len(line) - len(line.lstrip())]))
    return found


def german(body, table, missing):
    """🇩🇪️ The same body with every string literal replaced by its checked-in translation."""
    def swap(match):
        inner = match.group(0)[1:-1]
        if inner not in table:
            missing[inner] += 1
            return match.group(0)
        return '"' + table[inner] + '"'
    return LITERAL.sub(swap, body)


LITERAL_CONVERSION = re.compile(r'^("(?:[^"\\]|\\.)*")\.(?:to_string|into|to_owned)\(\)$|^String::from\(\s*("(?:[^"\\]|\\.)*")\s*\)$')


def literal_argument(expression):
    """🏷️ `"x".to_string()` / `"x".into()` / `String::from("x")` collapse to the bare `&str` the
    constructor already takes; everything else is borrowed as `&String`."""
    match = LITERAL_CONVERSION.match(expression)
    return (match.group(1) or match.group(2)) if match else "&" + expression


def rewrite(body, prefix, indent):
    """🧵️ `LocalizedLabel::native(&<english>, &<german>)`, block-wrapped when the body is not one expression."""
    english, deutsch = body
    if "\n" not in english and ";" not in english:
        english, deutsch = literal_argument(english.strip()), literal_argument(deutsch.strip())
        return (
            f"{indent}fn label(&self) -> {prefix}LocalizedLabel {{\n"
            f"{indent}    {prefix}LocalizedLabel::native({english}, {deutsch})\n"
            f"{indent}}}"
        )
    else:
        english = "{\n" + english + "\n" + indent + "    }"
        deutsch = "{\n" + deutsch + "\n" + indent + "    }"
    return (
        f"{indent}fn label(&self) -> {prefix}LocalizedLabel {{\n"
        f"{indent}    {prefix}LocalizedLabel::native(&{english}, &{deutsch})\n"
        f"{indent}}}"
    )


def split_native_arguments(text):
    """✂️ The two top-level arguments of a `LocalizedLabel::native(…, …)` call, by depth and string
    state — a comma inside a `format!` argument list or inside a literal is not the separator."""
    opening = text.find("(")
    depth, index, inside, escaped, comma = 0, opening, False, False, None
    while index < len(text):
        character = text[index]
        if inside:
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == '"':
                inside = False
        elif character == '"':
            inside = True
        elif character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
            if depth == 0:
                return text[opening + 1 : comma].strip(), text[comma + 1 : index].strip(), opening + 1, index
        elif character == "," and depth == 1 and comma is None:
            comma = index
        index += 1
    raise SystemExit(f"unbalanced LocalizedLabel::native call in {text[:120]!r}")


def migrated_sites(source):
    """🎯️ `(start, end)` of every already-migrated, trait-anchored `fn label` body in one file."""
    lines = source.split("\n")
    offsets, cursor = [], 0
    for line in lines:
        offsets.append(cursor)
        cursor += len(line) + 1
    header, found = None, []
    for index, line in enumerate(lines):
        if re.match(r"^\s*impl\b", line):
            header = line.strip()
        if not MIGRATED.search(line):
            continue
        if header is None or " for " not in header or not any(t in header for t in TRAITS):
            continue
        depth = line.count("{") - line.count("}")
        scan = index
        while depth > 0:
            scan += 1
            depth += lines[scan].count("{") - lines[scan].count("}")
        found.append((offsets[index], offsets[scan] + len(lines[scan])))
    return found


def retranslated(english, deutsch, table, missing, mismatched, path):
    """🇩🇪️ The German argument with every string literal replaced by the current translation of the
    English literal in the SAME position. Only the literals move: the expressions around them stay
    exactly as authored, so a German side that reaches for its own helper (a translated enum word,
    a different argument order) survives a glossary correction instead of being flattened back into
    a copy of the English side."""
    sources = LITERAL.findall(english)
    targets = list(LITERAL.finditer(deutsch))
    if len(sources) != len(targets):
        mismatched.append((path, english[:80]))
        return None
    out, cursor = [], 0
    for literal, target in zip(sources, targets):
        inner = literal[1:-1]
        if inner not in table:
            missing[inner] += 1
            return None
        out.append(deutsch[cursor : target.start()])
        out.append('"' + table[inner] + '"')
        cursor = target.end()
    out.append(deutsch[cursor:])
    return "".join(out)


def retranslate(repo, table):
    """🇩🇪️ Rewrites the GERMAN argument of every migrated site to what the current glossary says."""
    missing, mismatched, changed, files, total = collections.Counter(), [], [], 0, 0
    for root in ROOTS:
        for dirpath, _, names in os.walk(os.path.join(repo, root)):
            for name in names:
                if name != "🦀️.rs":
                    continue
                path = os.path.join(dirpath, name)
                try:
                    source = open(path, encoding="utf-8").read()
                except (OSError, UnicodeDecodeError):
                    continue
                if "LocalizedLabel::native" not in source:
                    continue
                found = migrated_sites(source)
                if not found:
                    continue
                total += len(found)
                updated, edits = source, 0
                for start, end in reversed(found):
                    block = source[start:end]
                    call = block.find("LocalizedLabel::native")
                    if call < 0:
                        continue
                    english, deutsch, first, last = split_native_arguments(block[call:])
                    expected = retranslated(english, deutsch, table, missing, mismatched, path)
                    if expected is None or expected == deutsch:
                        continue
                    replaced = block[: call + first] + english + ", " + expected + block[call + last :]
                    updated = updated[:start] + replaced + updated[end:]
                    edits += 1
                if edits:
                    changed.append((path, updated, edits))
                    files += 1
    if missing:
        print(f"untranslated literals: {len(missing)}", file=sys.stderr)
        for literal, count in missing.most_common(30):
            print(f"  {count}× {literal!r}", file=sys.stderr)
        raise SystemExit("aborted before writing: the glossary is not exhaustive")
    if mismatched:
        print(f"skipped {len(mismatched)} sites whose German carries a different number of literals than its English:", file=sys.stderr)
        for path, head in mismatched[:20]:
            print(f"  {os.path.relpath(path, repo)}: {head}", file=sys.stderr)
    if "--dry-run" in sys.argv:
        print(f"would retranslate {sum(count for _, _, count in changed)} of {total} migrated sites in {files} files")
        return 0
    for path, updated, _ in changed:
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(updated)
    for path, _, count in sorted(changed, key=lambda row: -row[2])[:20]:
        print(f"{count:5d}  {os.path.relpath(path, repo)}")
    print(f"retranslated {sum(count for _, _, count in changed)} of {total} migrated sites in {files} files")
    return 0


def main():
    repo = find_repo(HERE)
    table = json.load(open(os.path.join(HERE, "🐍️u3-de-glossary.json"), encoding="utf-8"))
    if "--retranslate" in sys.argv:
        return retranslate(repo, table)
    missing, per_file, total = collections.Counter(), {}, 0
    planned = []
    for root in ROOTS:
        for dirpath, _, names in os.walk(os.path.join(repo, root)):
            for name in names:
                if name != "🦀️.rs":
                    continue
                path = os.path.join(dirpath, name)
                try:
                    source = open(path, encoding="utf-8").read()
                except (OSError, UnicodeDecodeError):
                    continue
                if SIGNATURE not in source:
                    continue
                found = sites(source)
                if not found:
                    continue
                spans = [(start, end) for start, end, _, _, _ in found]
                if len(set(spans)) != len(spans):
                    raise SystemExit(f"overlapping spans in {path}")
                updated = source
                for start, end, prefix, body, indent in reversed(found):
                    pair = (body, german(body, table, missing))
                    updated = updated[:start] + rewrite(pair, prefix, indent) + updated[end:]
                planned.append((path, updated, len(found)))
                per_file[os.path.relpath(path, repo)] = len(found)
                total += len(found)
    if missing:
        print(f"untranslated literals: {len(missing)}", file=sys.stderr)
        for literal, count in missing.most_common(30):
            print(f"  {count}× {literal!r}", file=sys.stderr)
        raise SystemExit("aborted before writing: the glossary is not exhaustive")
    if "--dry-run" in sys.argv:
        print(f"would rewrite {total} sites in {len(planned)} files")
        return 0
    for path, updated, _ in planned:
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(updated)
    for path, _, count in sorted(planned, key=lambda row: -row[2])[:20]:
        print(f"{count:5d}  {os.path.relpath(path, repo)}")
    print(f"rewrote {total} sites in {len(planned)} files")
    with open(os.path.join(HERE, "🗑️generated", "u3-codemod-per-file.txt"), "w", encoding="utf-8") as handle:
        for name, count in sorted(per_file.items()):
            handle.write(f"{count}\t{name}\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
