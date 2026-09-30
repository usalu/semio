"""🌿️ Adds `alternatives: Vec::new()` to every full `HistoryView` struct literal (struct-update literals are left alone)."""
import pathlib, re, subprocess, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
files = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "HistoryView {", "🧰️framework", "✏️s"], cwd=ROOT, capture_output=True, text=True).stdout.split()
dry = "--dry" in sys.argv
for rel in files:
    path = ROOT / rel
    text = path.read_text()
    out, i, changed = [], 0, 0
    for match in re.finditer(r"HistoryView \{", text):
        start = match.start()
        before = text[max(0, start - 40):start]
        if before.rstrip().endswith("struct") or before.rstrip().endswith("impl") or before.rstrip().endswith("for") or before.rstrip().endswith("->") or before.rstrip().endswith("::Artifact"):
            continue
        if text[max(0, start - 8):start].endswith("Artifact") or re.search(r"(->|struct|impl|for)\s*(\w+::)*$", before):
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
        if ".." in body.replace("..=", "") or "alternatives:" in body or "active_alternative_id" not in body:
            continue
        field = re.search(r"active_alternative_id: [^,]+,", body)
        insert_at = match.end() + field.end()
        out.append(insert_at)
    for insert_at in reversed(out):
        text = text[:insert_at] + " alternatives: Vec::new()," + text[insert_at:]
        changed += 1
    if changed:
        print(rel, changed)
        if not dry:
            path.write_text(text)
