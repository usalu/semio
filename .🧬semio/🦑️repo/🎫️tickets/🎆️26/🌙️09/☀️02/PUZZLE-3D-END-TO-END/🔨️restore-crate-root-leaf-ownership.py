#!/usr/bin/env python3
"""🔨️ Second pass of the leaf-ownership restore, for artifacts whose leaf modules are mounted from
the stdio crate root instead of from the aggregate file (gltf, and any sibling the a6 shard split
the same way).

Inside each generated `pub mod mutations { … }` block the aggregate itself is mounted as
`mod component;`. Its subset is the aggregate's owner; every sibling `pub mod <leaf>;` mounted from a
DIFFERENT subset violates `validate_mutation_leaf_source`'s immediate-child requirement. Moves those
leaf directories next to the aggregate, rewrites their descriptor `owner`, and repoints the mount.
"""
import json
import os
import re
import shutil
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
CRATE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs")
BLOCK = re.compile(r"pub mod mutations \{(.*?)\n(\s*)\}", re.DOTALL)
LEAF = re.compile(r'#\[path = "((?:\.\./)+)(🗿️artifacts/[^"]*?/🪆️subsets/)([^"/]+)(/🧬️schema/🧬️mutations)/([^"/]+)/([^"/]+)"\]')
COMPONENT = re.compile(r'#\[path = "((?:\.\./)+)(🗿️artifacts/[^"]*?/🪆️subsets/)([^"/]+)(/🧬️schema/🧬️mutations)/([^"/]+)"\]\s*\n\s*mod component;')


def relative(path):
    return os.path.relpath(path, ROOT).replace(os.sep, "/")


def run(apply):
    source = open(CRATE, encoding="utf-8").read()
    crate_dir = os.path.dirname(CRATE)
    moves = []

    def rewrite_block(block_match):
        body = block_match.group(1)
        owner = COMPONENT.search(body)
        if owner is None:
            return block_match.group(0)
        ups, prefix, owner_subset, tail, _ = owner.groups()
        aggregate_dir = os.path.normpath(os.path.join(crate_dir, ups + prefix + owner_subset + tail))

        def rewrite_leaf(leaf_match):
            leaf_ups, leaf_prefix, subset, leaf_tail, leaf, filename = leaf_match.groups()
            if leaf_prefix != prefix or subset == owner_subset:
                return leaf_match.group(0)
            origin = os.path.normpath(os.path.join(crate_dir, leaf_ups + leaf_prefix + subset + leaf_tail, leaf))
            target = os.path.join(aggregate_dir, leaf)
            if not os.path.isdir(origin):
                print(f"  !! missing {relative(origin)}", file=sys.stderr)
                return leaf_match.group(0)
            if os.path.exists(target):
                print(f"  !! exists {relative(target)}", file=sys.stderr)
                return leaf_match.group(0)
            moves.append((origin, target))
            return f'#[path = "{leaf_ups}{leaf_prefix}{owner_subset}{leaf_tail}/{leaf}/{filename}"]'

        return block_match.group(0)[: len("pub mod mutations {")] + LEAF.sub(rewrite_leaf, body) + "\n" + block_match.group(2) + "}"

    rewritten = BLOCK.sub(rewrite_block, source)
    for origin, target in moves:
        print(f"{relative(origin)} -> {relative(target)}")
    print(f"{'moved' if apply else 'would move'} {len(moves)} leaf dir(s)")
    if not apply:
        return
    for origin, target in moves:
        shutil.move(origin, target)
        descriptor = os.path.join(target, "🔣️.json")
        if os.path.isfile(descriptor):
            payload = json.load(open(descriptor, encoding="utf-8"))
            payload["owner"] = relative(target)
            with open(descriptor, "w", encoding="utf-8") as handle:
                json.dump(payload, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
        parent = os.path.dirname(origin)
        while parent.startswith(ROOT) and os.path.isdir(parent) and not os.listdir(parent):
            os.rmdir(parent)
            parent = os.path.dirname(parent)
    with open(CRATE, "w", encoding="utf-8") as handle:
        handle.write(rewritten)


if __name__ == "__main__":
    run("--apply" in sys.argv)
