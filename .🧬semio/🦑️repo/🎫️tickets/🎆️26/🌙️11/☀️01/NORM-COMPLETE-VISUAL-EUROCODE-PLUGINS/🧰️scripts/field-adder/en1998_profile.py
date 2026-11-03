"""EN 1998 profile of the field-adder: dispatches the spec fields of `specs/en1998-*.json` to the root (site, building) and row (bridge, assessment, foundation, tower, member) handlers, then completes every Rust struct literal of the snapshot types, idempotently."""
import os, re, sys

import en1998_root, en1998_rows

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
MODELS = {"site": "En1998Site", "building": "En1998Building", "bridge": "En1998Bridge", "assessment": "En1998Assessment", "foundation": "En1998Foundation", "tower": "En1998Tower", "tank": "En1998Tank", "member": "En1998Member", "retaining_wall": "En1998RetainingWall"}


def block_end(text, start):
    depth = 0
    for i in range(start, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    raise AssertionError("unbalanced")


def add_fields(text, open_index, close_index, additions):
    body = text[open_index + 1:close_index]
    kept = body.rstrip()
    tail = body[len(kept):]
    if "\n" in body:
        indent = re.match(r" *", kept.split("\n")[-1]).group(0)
        head = kept if kept.endswith(",") else kept + ","
        new = head + "".join("\n%s%s: %s," % (indent, n, v) for n, v in additions)
        return text[:open_index + 1] + new + tail + text[close_index:]
    head = kept if kept.endswith(",") or not kept else kept + ","
    new = head + "".join(" %s: %s," % (n, v) for n, v in additions)
    return text[:open_index + 1] + new.rstrip(",") + (tail or " ") + text[close_index:]


def complete_literals(fields, dry):
    model_text = open(ANY + "🧬️schema/📸️snapshot/🦀️.rs", encoding="utf-8").read()
    structs = {}
    for ent, model in MODELS.items():
        found = re.search(r"pub struct %s \{" % model, model_text)
        structs[model] = re.findall(r"^    pub (\w+):", model_text[found.end():block_end(model_text, found.end() - 1)], re.M)
    defaults = {f["field"]: float(f["default"]) for f in fields}
    changed = 0
    for root, dirs, files in os.walk(NORM):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(root, name)
            text = open(path, encoding="utf-8").read()
            original = text
            for model, names in structs.items():
                position = 0
                while True:
                    match = re.compile(r"(?<![\w])%s \{" % model).search(text, position)
                    if not match:
                        break
                    position = match.end()
                    before = text[max(0, match.start() - 14):match.start()]
                    if re.search(r"(struct|impl|for|type|use) $", before) or text[match.end():match.end() + 3] == " of":
                        continue
                    close = block_end(text, match.end() - 1)
                    body = text[match.end():close]
                    if re.search(r"\.\.\w", body) or not any(re.search(r"(?<![\w.])%s\s*:" % n, body) for n in names[:4]):
                        continue
                    missing = [n for n in names if not re.search(r"(?<![\w.])%s\b\s*[:,]|(?<![\w.])%s\s*$" % (n, n), body)]
                    additions = [(n, repr(defaults[n])) for n in missing if n in defaults]
                    if not additions:
                        continue
                    text = add_fields(text, match.end() - 1, close, additions)
                    position = match.end()
            if text != original:
                changed += 1
                print("  literals completed", path.replace(NORM, ""))
                if not dry:
                    open(path, "w", encoding="utf-8").write(text)
    print("  literal files:", changed)


def apply(specs, dry):
    en1998_root.DRY = dry
    en1998_rows.DRY = dry
    for f in specs["fields"]:
        ent = f["entity"]
        args = (ent, f["field"], f["default"], f["en"], f["de"])
        if ent in ("site", "building"):
            en1998_root.run(*args)
        else:
            en1998_rows.run(*args, f.get("unit"), f.get("ui"))
    complete_literals(specs["fields"], dry)
