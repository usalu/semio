#!/usr/bin/env python3
"""🖼️ S19 one-off codemod (set `norm-examples`): writes the production-printed example documents the census
(`s19-example-census.rs`) emitted for every example whose committed asset did not decode to its code-built constructor —
`<census>/<family>/<id>.dsl` → the example module's `include_str!` `🗣️.dsl.semio`, `<id>.pack` → the sibling
`🎒️.pack.semio` when that example keeps one. The constructors are what `setActiveExample` loaded before the roster route,
so the regenerated asset is exactly the document users saw. Only the families named on the command line are written.
Idempotent. usage: s19-norm-assets.py <root> <census-dir> <family …>"""
import os
import re
import sys

root, census, families = sys.argv[1], sys.argv[2], sys.argv[3:]
if not families or any(not re.fullmatch(r"(din|en|iso|vdi)\d+", family) for family in families):
    sys.exit(f"usage: s19-norm-assets.py <root> <census-dir> <family …> (got {families})")
ARTIFACTS = os.path.join(root, "✏️s/🔌️plugins/📕️norm/🗿️artifacts")


def example_modules(family):
    folder = next(name for name in os.listdir(ARTIFACTS) if name.endswith(family))
    examples = os.path.join(ARTIFACTS, folder, "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples")
    for name in sorted(os.listdir(examples)):
        source_path = os.path.join(examples, name, "🦀️.rs")
        if not os.path.exists(source_path):
            continue
        source = open(source_path, encoding="utf-8").read()
        example_id = re.search(r'pub const ID: &str = "([^"]+)";', source)
        primary = re.search(r'PRIMARY_TEXT: &str = include_str!\("([^"]+)"\);', source)
        if example_id and primary:
            yield example_id.group(1), os.path.normpath(os.path.join(os.path.dirname(source_path), primary.group(1)))


def write(path, data, binary):
    before = open(path, "rb").read() if os.path.exists(path) else None
    if before != data:
        open(path, "wb").write(data)
        print(f"wrote {os.path.relpath(path, root)}")


for family in families:
    directory = os.path.join(census, family)
    targets = dict(example_modules(family))
    for name in sorted(os.listdir(directory)):
        example_id, extension = os.path.splitext(name)
        if extension != ".dsl":
            continue
        asset = targets[example_id]
        write(asset, open(os.path.join(directory, name), "rb").read(), False)
        pack = os.path.join(os.path.dirname(os.path.dirname(asset)), "🎒️.pack.semio")
        if os.path.exists(pack):
            write(pack, open(os.path.join(directory, example_id + ".pack"), "rb").read(), True)
