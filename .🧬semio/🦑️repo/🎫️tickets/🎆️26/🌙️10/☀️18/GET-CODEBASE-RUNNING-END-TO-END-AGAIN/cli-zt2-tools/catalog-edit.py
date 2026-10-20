"""Edits the operations catalogue text of an overlay: localized notes, the notes label, the dependencies synopsis, per-subcommand flags and examples. Usage: catalog-edit.py <catalog.json> <notes.json>"""
import json, sys
path, notes_path = sys.argv[1:3]
text = open(path, encoding="utf-8", newline="").read()
notes = json.load(open(notes_path, encoding="utf-8"))

def block_end(source, start):
    depth, index, quoted = 0, start, False
    while index < len(source):
        char = source[index]
        if quoted:
            if char == "\\": index += 1
            elif char == '"': quoted = False
        elif char == '"': quoted = True
        elif char == "[": depth += 1
        elif char == "]":
            depth -= 1
            if depth == 0: return index
        index += 1
    sys.exit("unbalanced notes block")

catalog = json.loads(text)
cursor = 0
for operation in catalog["operations"]:
    if "notes" not in operation: continue
    rows = notes[operation["name"]]
    if [row[0] for row in rows] != [row for row in operation["notes"]] and len(rows) != len(operation["notes"]): sys.exit(f"{operation['name']}: note count {len(operation['notes'])} != {len(rows)}")
    start = text.index('"notes": [', cursor)
    open_index = start + len('"notes": ')
    end = block_end(text, open_index)
    indent = text[text.rfind("\n", 0, start) + 1:start]
    body = ",\n".join(f"{indent}  {json.dumps({'en': en, 'de': de}, ensure_ascii=False)}" for en, de in rows)
    text = text[:open_index] + "[\n" + body + "\n" + indent + "]" + text[end + 1:]
    cursor = open_index + len(body)

def sub(old, new, count=1):
    global text
    if text.count(old) != count: sys.exit(f"anchor count {text.count(old)} != {count}: {old[:90]!r}")
    text = text.replace(old, new)

sub('"usedBy": { "en": "used by", "de": "verwendet von" }\n', '"usedBy": { "en": "used by", "de": "verwendet von" },\n    "notes": { "en": "Notes", "de": "Hinweise" }\n')
sub('"synopsis": "dependencies sync|lock --all|--scope <owner>",', '"synopsis": "dependencies sync|lock --all|--scope <owner> | ensure",')

def example(command, en, de): return {"command": command, "description": {"en": en, "de": de}}
scoped = {
    "dependencies sync --all|--scope <owner>": {"flags": ["--all", "--scope"]},
    "dependencies lock --all|--scope <owner>": {"flags": ["--all", "--scope"]},
    "dependencies ensure": {"flags": []},
    "cache status": {"flags": []},
    "cache clear": {"flags": []},
    "cache remote status": {"flags": []},
    "cache serve": {"flags": ["--port", "--host", "--directory", "--max-bytes"]},
    "capacity status [--json]": {"flags": ["--json"]},
    "capacity run [--operation=<name>|--memory=<megabytes> [--cpu=<cores>]] [--jobs=<n>] -- <command> <args>": {"flags": ["--operation", "--memory", "--cpu", "--jobs"]},
    "incremental list": {"flags": []},
    "incremental plan [--scenario=<ids|all>]": {"flags": ["--scenario", "--implementations", "--root", "--json"], "examples": [example("repo incremental plan --scenario=fw-ts-comment", "Print the tasks one scenario must and may re-execute.", "Gibt die Tasks aus, die ein Szenario neu ausführen muss und darf.")]},
    "incremental prepare [--fresh]": {"flags": ["--root", "--fresh"], "examples": [example("repo incremental prepare --fresh", "Clone the checkout into the disposable root and bootstrap it.", "Klont den Checkout in das Wegwerf-Verzeichnis und richtet es ein.")]},
    "incremental run [--scenario=<ids|all>]": {"flags": ["--scenario", "--implementations", "--root", "--phases", "--file", "--json"]},
    "incremental discard": {"flags": ["--root"], "examples": [example("repo incremental discard", "Remove the disposable root.", "Entfernt das Wegwerf-Verzeichnis.")]},
    "concurrency list": {"flags": []},
    "concurrency plan [--workloads=<ids|all>] [--sessions=<n>]": {"flags": ["--workloads", "--sessions", "--implementations", "--root", "--json"]},
    "show targets": {"examples": [example("repo show targets --json", "Print every target of the neutral graph as JSON.", "Gibt jedes Target des neutralen Graphen als JSON aus.")]},
}
for synopsis, extra in scoped.items():
    pieces = " ".join(f'"{key}": {json.dumps(value, ensure_ascii=False)},' for key, value in extra.items())
    anchor = f'"synopsis": {json.dumps(synopsis, ensure_ascii=False)},'
    if text.count(anchor) != 1:
        if text.count(anchor) == 0: sys.exit(f"no subcommand anchor: {synopsis}")
        sys.exit(f"ambiguous anchor ({text.count(anchor)}): {synopsis}")
    text = text.replace(anchor, anchor + " " + pieces)
json.loads(text)
open(path, "w", encoding="utf-8", newline="").write(text)
print("catalogue edited")
