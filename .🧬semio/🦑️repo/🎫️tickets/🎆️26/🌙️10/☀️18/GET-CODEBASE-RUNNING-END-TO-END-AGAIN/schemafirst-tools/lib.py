#!/usr/bin/env python3
"""🏭️ Builds staged schema-first batches (lane schemafirst): each batches/<name>.py describes its edits with this library; running it reads every file from the source root (SF_SRC, default the live root; the verified base is the live tree plus the compliance batches), applies counted edits, adds schema definitions and law rows, and writes staged/<name>/{manifest.json,files,base,stage.py,verify.sh}. Re-run it to rebase."""
import hashlib, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here:
        here = os.path.dirname(here)
    return here


LIVE = root_of(C)
SRC = os.path.abspath(os.environ.get("SF_SRC", LIVE))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PROC = "🧰️framework/🔨️modules/🏃️process/"
BUILD = LIB + "🏗️build/"
LAW = BUILD + "🧪️tests/🧬️policy-literals/"
LAW_SRC = os.path.join(HERE, "law-src")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def compliance_index():
    index = {}
    base = os.path.join(C, "compliance-tools", "staged")
    for batch in sorted(os.listdir(base)):
        manifest = os.path.join(base, batch, "manifest.json")
        if os.path.exists(manifest):
            for rel, row in json.load(open(manifest, encoding="utf8")).items():
                if row["new"]:
                    index[row["new"]] = "compliance/" + batch
    return index


POOL = ["📏️", "🚧️", "🎚️", "🧮️", "⚖️", "🔩️", "⛓️", "📐️", "🗜️", "🧯️", "🪫", "🧱️", "🪢️", "🪤", "🛟", "🧲️", "🪝", "🧰️", "🔖️", "🏷️", "⏱️", "⏳️", "🔒️", "🧷️", "🔧️"]
MARK = "‹E›"
KEEP = object()


def fresh(text, new):
    """Replaces each ‹E› of `new` by an emoji no docstring of `text` (or of `new`) opens with."""
    while MARK in new:
        used = set(m.group(1).replace("\uFE0F", "") for m in re.finditer(r"/\*\*\s*\n?\s*\*?\s*(\S)", text + new))
        pick = next(e for e in POOL if e.replace("\uFE0F", "") not in used)
        new = new.replace(MARK, pick, 1)
    return new


ROW_EMOJI = {"revision": "🔏️", "revision-observation": "🔁️", "revision-graph": "🔂️", "revision-arguments": "🔃️", "observation": "👁️", "graph": "🕸️", "memo": "🧠️", "generations": "🕰️", "generation-claims": "🧩️", "generation-chunks": "🧱️", "cache-gc": "🧹️", "host-capacity": "🌡️", "bazel-sessions": "🧵️", "bazel-script": "📜️", "read-oracle": "🔭️", "precision": "🎯️", "inputs": "📥️", "command-runtime": "⌨️", "inference": "🔎️", "records": "🗃️"}


class Def:
    """One policy definition: values in order, (en, de) per key, and an English summary with its German twin."""

    def __init__(self, name, summary_en, summary_de, rows):
        self.name, self.summary = name, "EN: %s DE: %s" % (summary_en, summary_de)
        self.values = {key: value for key, value, _en, _de in rows}
        self.en = {key: en for key, _value, en, _de in rows}
        self.de = {key: de for key, _value, _en, de in rows}


def indent_of(text):
    match = re.search(r"\n( +)\"", text)
    return len(match.group(1)) if match else 2


def dump(value, unit, depth):
    body = json.dumps(value, indent=unit, ensure_ascii=False)
    return body.replace("\n", "\n" + " " * (unit * depth))


def close_of(text, key, frm=0):
    """Index of the closing brace of the first object named `key` at or after `frm`."""
    start = text.index('"%s"' % key, frm)
    at = text.index("{", start)
    depth, inside, escaped = 0, False, False
    for index in range(at, len(text)):
        char = text[index]
        if inside:
            escaped = not escaped and char == "\\"
            if char == '"' and not escaped:
                inside = False
            continue
        if char == '"':
            inside = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
    raise SystemExit("unbalanced " + key)


class Batch:
    def __init__(self, name, title):
        self.name, self.title = name, title
        self.files, self.bases, self.notes = {}, {}, {}

    def text(self, rel):
        if rel in self.files:
            return self.files[rel].decode("utf8")
        path = os.path.join(SRC, rel)
        data = open(path, "rb").read()
        self.bases.setdefault(rel, data)
        return data.decode("utf8")

    def put(self, rel, text):
        self.text(rel) if os.path.exists(os.path.join(SRC, rel)) else self.bases.setdefault(rel, None)
        self.files[rel] = text.encode("utf8")

    def sub(self, rel, old, new, count=1):
        text = self.text(rel)
        found = text.count(old)
        if count == "all" and found > 0:
            count = found
        if found != count:
            raise SystemExit("anchor count %d != %d in %s: %s" % (found, count, rel[-60:], old[:80]))
        self.put(rel, text.replace(old, fresh(text, new) if MARK in new else new))

    def cut(self, rel, start, end, new):
        """Replaces the unique span from `start` through `end` (inclusive)."""
        text = self.text(rel)
        if text.count(start) != 1:
            raise SystemExit("start anchor count %d in %s: %s" % (text.count(start), rel[-60:], start[:60]))
        at = text.index(start)
        stop = text.index(end, at) + len(end)
        self.put(rel, text[:at] + fresh(text, new) + text[stop:])

    def subs(self, rel, pairs):
        for pair in pairs:
            self.sub(rel, *pair)

    def add_defs(self, rel, defs, key="definitions"):
        """Adds each definition (or the missing keys of an existing one: the existing values must agree) and its Descriptions sibling."""
        unit = indent_of(self.text(rel))
        if key not in json.loads(self.text(rel)):
            text = self.text(rel).rstrip()
            self.put(rel, text[:-1].rstrip() + ',\n' + ' ' * unit + '"%s": {\n%s}\n}\n' % (key, ' ' * unit))
        for definition in defs:
            text = self.text(rel)
            document = json.loads(text)[key]
            if definition.name in document:
                held = document[definition.name]["const"]
                for name, value in held.items():
                    if name in definition.values and definition.values[name] is not KEEP and definition.values[name] != value:
                        raise SystemExit("%s.%s holds %r, not %r" % (definition.name, name, value, definition.values[name]))
                missing = {name: value for name, value in definition.values.items() if name not in held and value is not KEEP}
                if missing:
                    end = close_of(text, "const", text.index('"%s"' % definition.name, text.index('"%s"' % key)))
                    pad = " " * (unit * 4)
                    items = ",\n".join("%s%s: %s" % (pad, json.dumps(name), dump(value, unit, 4)) for name, value in missing.items())
                    text = text[:end].rstrip() + ",\n" + items + "\n" + " " * (unit * 3) + text[end:]
                    self.put(rel, text)
                text = self.text(rel)
                document = json.loads(text)[key]
                if set(document[definition.name]["const"]) != set(definition.values):
                    raise SystemExit("%s keys differ from the schema: %s" % (definition.name, set(document[definition.name]["const"]) ^ set(definition.values)))
                piece = '"%sDescriptions": %s' % (definition.name, dump({"const": {"en": definition.en, "de": definition.de}}, unit, 2))
            else:
                body = {"description": definition.summary, "const": definition.values}
                piece = '"%s": %s,\n%s"%sDescriptions": %s' % (definition.name, dump(body, unit, 2), " " * (unit * 2), definition.name, dump({"const": {"en": definition.en, "de": definition.de}}, unit, 2))
            text = self.text(rel)
            end = close_of(text, key)
            tail_indent = text[text.rfind("\n", 0, end) + 1 : end]
            head = text[:end].rstrip()
            self.put(rel, head + ("\n" if head.endswith("{") else ",\n") + " " * (unit * 2) + piece + "\n" + tail_indent + text[end:])
            json.loads(self.files[rel].decode("utf8"))

    def edit_json(self, rel, change):
        """Applies `change(document)` to a JSON file that round-trips exactly through json.dumps(indent=2) and puts the result."""
        text = self.text(rel)
        document = json.loads(text)
        if json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "") != text:
            raise SystemExit("not a round-trip JSON file: " + rel[-60:])
        change(document)
        self.put(rel, json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""))

    def new_schema(self, rel, title, defs):
        unit = 2
        pieces = []
        for definition in defs:
            body = {"description": definition.summary, "const": definition.values}
            pieces.append('"%s": %s' % (definition.name, dump(body, unit, 2)))
            pieces.append('"%sDescriptions": %s' % (definition.name, dump({"const": {"en": definition.en, "de": definition.de}}, unit, 2)))
        text = '{\n  "$schema": "http://json-schema.org/draft-07/schema#",\n  "title": %s,\n  "definitions": {\n    %s\n  }\n}\n' % (json.dumps(title, ensure_ascii=False), ",\n    ".join(pieces))
        json.loads(text)
        self.put(rel, text)

    def row(self, slug, root, schema, definitions, sources, exports=(), allow=(), minimum_number=None, container=None, minimum_text=None, document=None, pointer=None):
        body = {"version": 1, "root": root, "schema": schema, "definitions": list(definitions), "sources": list(sources)}
        if container:
            body["container"] = container
        if document:
            body["document"], body["pointer"] = document, list(pointer or [])
        if exports:
            body["exports"] = [{"source": s, "name": n, "definition": d} for s, n, d in exports]
        if minimum_number:
            body["minimumNumber"] = minimum_number
        if minimum_text:
            body["minimumText"] = minimum_text
        if allow:
            body["allow"] = [{"source": s, "value": v, "reason": r} for s, v, r in allow]
        self.put(LAW + ROW_EMOJI[slug] + slug + "/🔣️.json", json.dumps(body, indent=1, ensure_ascii=False) + "\n")

    def write(self, tests=(), also=()):
        out = os.path.join(HERE, "staged", self.name)
        shutil.rmtree(out, ignore_errors=True)
        for sub in ("files", "base"):
            os.makedirs(os.path.join(out, sub))
        index = compliance_index()
        manifest = {}
        for number, (rel, data) in enumerate(self.files.items()):
            name = "%02d%s" % (number, os.path.splitext(rel)[1])
            base = self.bases.get(rel)
            live = os.path.join(LIVE, rel)
            live_hash = sha(open(live, "rb").read()) if os.path.exists(live) else None
            row = {"file": name, "base": sha(base) if base is not None else None, "new": sha(data)}
            if row["base"] != live_hash:
                row["requires"] = index.get(row["base"], "a batch that edits it first") if row["base"] else "the live file is absent"
            manifest[rel] = row
            open(os.path.join(out, "files", name), "wb").write(data)
            if base is not None:
                open(os.path.join(out, "base", name), "wb").write(base)
        json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
        template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
        lines = template.split("\n")
        lines[1] = '"""🧬️ Staged batch `%s` (lane schemafirst): %s' % (self.name, self.title) + ("" if self.title.endswith(".") else ".")
        stage = "\n".join(lines).replace('print("DRIFTED", rel[-90:])', 'print("DRIFTED", rel[-90:], "requires", _row.get("requires", "-"))').replace('print(kind, rel[-90:])', 'print(kind, rel[-90:], _row.get("requires", "") if kind == "drifted" else "")')
        open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(stage)
        os.chmod(os.path.join(out, "stage.py"), 0o755)
        law = [test for test in tests if test.endswith("🧬️policy-literals/🟦️.ts")]
        module = [test for test in tests if test not in law]
        commands = "\n".join('(cd "$root" && bun test "%s")' % test for test in law)
        if module:
            commands += "\necho 'module tests, each file through the envelope runner (C/red-g-tools/mrun.sh <mirror> env <file>), failure sets equal to the base: %s'" % " ".join(module)
        open(os.path.join(out, "verify.sh"), "w", encoding="utf8").write(VERIFY % {"name": self.name, "tests": commands})
        os.chmod(os.path.join(out, "verify.sh"), 0o755)
        print("%s: %d files (%d new), requires: %s" % (self.name, len(manifest), sum(1 for r in manifest.values() if r["base"] is None), sorted({r["requires"] for r in manifest.values() if "requires" in r}) or "-"))


VERIFY = """#!/bin/sh
# 🧬️ Verifies the staged batch `%(name)s` (lane schemafirst): stage check, Bun parse-check + docstring/emoji/[DEBUG] laws of the changed files, then the policy literal law over the whole repository (the module tests are listed for the envelope runner).
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default: the root above this batch)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=${VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}
tools=$(cd "$here/../.." && pwd)
c=$(cd "$tools/.." && pwd)
cd "$root"
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$tools/docheck.py" "$here" "$root"
%(tests)s
"""
