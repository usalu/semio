"""🔮️ EN2: projects the prepared draw verb descriptions onto a copy of the descriptor roots, so the description law
(`semio-os-mcp audit`) and its AJV twin (`wp-d1/d1-oracle.ts`) can judge them before the draw plugin is re-described.

Usage: en2-draw-projection.py <editor source with the patch> <out-base> <out-patched>

Both outputs start from LB's landed-source projection (`.🧬semio/🌐hub/s13-lb-d1-projected`) with the draw descriptor
replaced by the LIVE `✏️s/🔌️plugins/🖍️draw/🔣️.json`; `<out-patched>` additionally carries, for every draw action whose
builder in <editor source> declares `.describe(LocalizedLabel::native(en, de))` / `.use_when([...])` and whose
descriptor entry has no description yet, exactly those texts (native = reuse, as the describe step writes them).
Honest by construction: only literal `bounded_catalog("<id>", …)` + `.describe(` + `.use_when(` chains are read.
"""
import json
import re
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SOURCE_ROOT = ROOT / ".🧬semio/🌐hub/s13-lb-d1-projected"
DRAW = "✏️s/🔌️plugins/🖍️draw/🔣️.json"
STR = r'"((?:[^"\\]|\\.)*)"'
START = re.compile(r'bounded_catalog\(\s*' + STR)
DESCRIBE = re.compile(r'\.describe\(LocalizedLabel::native\(\s*' + STR + r',\s*' + STR + r',?\s*\)\)')
USE_WHEN = re.compile(r'\.use_when\(\[(.*?)\]\)', re.S)


def declared(source):
    found = {}
    starts = list(START.finditer(source))
    for index, start in enumerate(starts):
        end = starts[index + 1].start() if index + 1 < len(starts) else len(source)
        chain = source[start.end() : end]
        describe, use_when = DESCRIBE.search(chain), USE_WHEN.search(chain)
        if describe is None or use_when is None:
            continue
        english, german = describe.groups()
        found[start.group(1)] = (json.loads('"%s"' % english), json.loads('"%s"' % german), [json.loads('"%s"' % phrase) for phrase in re.findall(STR, use_when.group(1))])
    return found


def actions(node):
    if isinstance(node, dict):
        if isinstance(node.get("id"), str) and isinstance(node.get("semantics"), dict):
            yield node
        for value in node.values():
            yield from actions(value)
    elif isinstance(node, list):
        for value in node:
            yield from actions(value)


def build(out, projected):
    shutil.rmtree(out, ignore_errors=True)
    shutil.copytree(SOURCE_ROOT, out)
    descriptor = json.loads((ROOT / DRAW).read_text(encoding="utf-8"))
    touched = []
    for action in actions(descriptor):
        if action["id"] in projected and "description" not in action["semantics"]:
            english, german, phrases = projected[action["id"]]
            action["semantics"]["description"] = {"native": {"en": english, "de": german}, "reuse": {"en": english, "de": german}}
            action["semantics"]["useWhen"] = phrases
            touched.append(action["id"])
    (out / DRAW).write_text(json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return touched


source = Path(sys.argv[1]).read_text(encoding="utf-8")
projected = {verb: texts for verb, texts in declared(source).items() if verb in ("editSelection", "editPath", "editFill")}
print("declared in source:", sorted(projected))
print("base:", build(Path(sys.argv[2]), {}))
print("patched:", build(Path(sys.argv[3]), projected))
