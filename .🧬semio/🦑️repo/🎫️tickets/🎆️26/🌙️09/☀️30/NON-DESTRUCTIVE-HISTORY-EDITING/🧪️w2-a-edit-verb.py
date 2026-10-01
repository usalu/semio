"""🏷️ `verb` pass: adds the authoring-verb field to every `Edit`, `MutationEnvelope` and `HistoryEdit` struct literal.
A literal copying `description` (`transaction` for envelopes) from a value that carries a verb copies the verb; every
other literal gets `None`. Sites whose codec owns a local `verb` binding are left to the hand-written codec edits
(`MANUAL`). Dry run with `--dry`."""
import pathlib, re, subprocess, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
DRY = "--dry" in sys.argv
CARRIERS = {"source", "header", "history_edit", "oracle", "edit", "envelope"}
KINDS = [("Edit", ["forwards", "sequence_number"], "description"), ("HistoryEdit", ["ops", "started_at"], "description"), ("MutationEnvelope", ["mutation_id", "document_id"], "transaction")]
MANUAL = {
    ("📡️replication/🎮️mutation/🦀️.rs", "Edit"),
    ("📡️replication/🔗️causal/🦀️.rs", "MutationEnvelope"),
    ("📡️spr/📜️history/🦀️.rs", "HistoryEdit"),
}


def fields_of(body):
    fields, depth, start, i, quote = [], 0, 0, 0, False
    while i < len(body):
        c = body[i]
        if quote:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                quote = False
        elif c == '"':
            quote = True
        elif c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            fields.append((body[start:i].strip(), i + 1))
            start = i + 1
        i += 1
    if body[start:].strip():
        fields.append((body[start:].strip(), len(body)))
    return fields


def literals(text, name, required):
    pattern = re.compile(r"(?<![A-Za-z_])" + name + r"(::<[^{}]*?>)?\s*\{")
    for match in pattern.finditer(text):
        before = text[max(0, match.start() - 40):match.start()]
        if re.search(r"(struct|enum|impl|for|fn|->|let|\|)\s*(\(\s*)?$", before):
            continue
        depth, j = 0, match.end() - 1
        while True:
            c = text[j]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        body = text[match.end():j]
        fields = fields_of(body)
        names = [field.split(":", 1)[0].strip() for field, _ in fields]
        if any(field not in names for field in required) or "verb" in names or any(n.startswith("..") for n in names):
            continue
        yield match, body, fields


def copied(expr):
    m = re.match(r"^&?([a-z_]+)\.(?:description|transaction)(\.clone\(\))?$", expr.strip())
    if m and m.group(1) in CARRIERS:
        return f"{m.group(1)}.verb{m.group(2) or ''}"
    return None


def manual(rel, kind, expr):
    if any(rel.endswith(path) and kind == k for path, k in MANUAL) and expr in ("description", "transaction"):
        return True
    return rel.endswith("🏪️store/🦀️.rs") and kind == "Edit" and expr in ("description", "self.strings[2].take()")


files = subprocess.run(["/usr/bin/grep", "-rlE", "--include=🦀️.rs", r"(Edit|MutationEnvelope|HistoryEdit)(::<[^>]*>)?\s*\{", "🧰️framework", "✏️s", "🌎️hub"], cwd=ROOT, capture_output=True, text=True).stdout.split()
total, notes = 0, []
for rel in files:
    path = ROOT / rel
    text = path.read_text()
    inserts = []
    for kind, required, field in KINDS:
        for match, body, fields in literals(text, kind, required):
            line = text.count("\n", 0, match.start()) + 1
            found = next(((entry, end) for entry, end in fields if entry.split(":", 1)[0].strip() == field), None)
            if found is None:
                notes.append(f"SKIP {rel}:{line} {kind} without {field}")
                continue
            entry, end = found
            expr = entry.split(":", 1)[1].strip() if ":" in entry else field
            if manual(rel, kind, expr):
                notes.append(f"MANUAL {rel}:{line} {kind} ({expr})")
                continue
            verb = copied(expr) or "None"
            needs_comma = not body[:end].rstrip().endswith(",")
            inserts.append((match.end() + end, ("," if needs_comma else "") + f" verb: {verb},", line, kind, expr))
            if verb != "None":
                notes.append(f"COPY {rel}:{line} {kind} {expr} -> {verb}")
    for at, snippet, line, kind, expr in sorted(inserts, reverse=True):
        text = text[:at] + snippet + text[at:]
        total += 1
    if inserts and not DRY:
        path.write_text(text)
print("literals", total)
for note in notes:
    print(note)
