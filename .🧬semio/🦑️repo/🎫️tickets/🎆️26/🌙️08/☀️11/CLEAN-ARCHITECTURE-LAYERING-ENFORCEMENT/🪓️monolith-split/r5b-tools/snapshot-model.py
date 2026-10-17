#!/usr/bin/env python3
"""Splits the 1.7 base schema snapshot: the typed document model moves to the child `📐️model`, the root keeps PdfSnapshot and its codecs."""
import os, sys
B = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot"
root = f"{B}/🦀️.rs"
model = f"{B}/📐️model/🦀️.rs"
lines = open(root).read().split("\n")
def idx(prefix, start=0):
    for i in range(start, len(lines)):
        if lines[i].startswith(prefix): return i
    raise SystemExit("missing " + prefix)
a = idx("//#region 🔖️ObjectModel")
b = idx("//#region 🔖️Snapshot")
head = lines[:a]
body = lines[a:b]
tail = lines[b:]
head_text = "\n".join(head)
assert '#[path="🌱️value/🧵️canonical/🦀️.rs"]\nmod canonical;' in head_text
new_head = head_text.replace('#[path="🌱️value/🧵️canonical/🦀️.rs"]\nmod canonical;\n', '#[path = "📐️model/🦀️.rs"]\npub mod model;\npub use model::*;\n')
model_text = "//! 📐️ The typed PDF 1.7 document model: object graph, content, colour, fonts, XObjects, navigation, annotations, forms and the page model.\n\nuse std::fmt;\n\n#[path=\"../🌱️value/🧵️canonical/🦀️.rs\"]\nmod canonical;\n\n" + "\n".join(body).rstrip() + "\n"
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    o = os.environ["F"] + "/r5b-tools/orig/snapshot-root.rs"
    if not os.path.exists(o): open(o, "w").write(open(root).read())
    os.makedirs(os.path.dirname(model), exist_ok=True)
    assert not os.path.exists(model)
    open(model, "w").write(model_text)
    open(root, "w").write(new_head + "\n" + "\n".join(tail))
    print("applied")
else:
    print(len(head), len(body), len(tail))
