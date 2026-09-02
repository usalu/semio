#!/usr/bin/env python3
"""🔨️ Restores mutation-leaf physical ownership to the aggregate that wraps each leaf.

Shard A6/A7 of ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS moved leaf
directories into satellite subset folders and repointed the base aggregate's module mounts at them
with `#[path = "../../../<satellite>/.../<leaf>/<file>"]`. `validate_mutation_leaf_source` requires
every wrapped leaf to be an immediate child of the aggregate's own mutation root, so that structure
cannot compile (7 x E0080). Subset ownership is declared in the mutation MANIFEST
(`owningSubsetOf` = `mutation.subset ?? manifest.subset`), never by file location, so moving the
directories back keeps the `unsplit-artifact-subset` gate satisfied.

Moves each cross-mounted leaf dir back next to its aggregate, rewrites the leaf descriptor's
`owner`, and rewrites the mount to a local path. Idempotent: a mount that is already local is left
alone.
"""
import json
import os
import re
import shutil
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
MOUNT = re.compile(r'#\[path = "((?:\.\./)+)([^"]+?)/([^"/]+)/([^"/]+)"\]')

AGGREGATES = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️hdrl/🧬️schema/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🧬️schema/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/🧬️schema/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️header/🧬️schema/🧬️mutations/🦀️.rs",
]


def relative(path):
    return os.path.relpath(path, ROOT).replace(os.sep, "/")


def restore(aggregate_rel, apply):
    aggregate = os.path.join(ROOT, aggregate_rel)
    mutations_dir = os.path.dirname(aggregate)
    source = open(aggregate, encoding="utf-8").read()
    moved = []

    def rewrite(match):
        ups, middle, leaf, filename = match.groups()
        origin = os.path.normpath(os.path.join(mutations_dir, ups + middle, leaf))
        target = os.path.join(mutations_dir, leaf)
        if not os.path.isdir(origin):
            print(f"  !! missing leaf dir {relative(origin)}", file=sys.stderr)
            return match.group(0)
        if os.path.exists(target):
            print(f"  !! target already exists {relative(target)}", file=sys.stderr)
            return match.group(0)
        moved.append((origin, target))
        return f'#[path = "{leaf}/{filename}"]'

    rewritten = MOUNT.sub(rewrite, source)
    print(f"{relative(aggregate)}: {len(moved)} leaf dir(s)")
    if not apply:
        for origin, target in moved:
            print(f"  {relative(origin)} -> {relative(target)}")
        return len(moved)
    for origin, target in moved:
        shutil.move(origin, target)
        descriptor = os.path.join(target, "🔣️.json")
        if os.path.isfile(descriptor):
            payload = json.load(open(descriptor, encoding="utf-8"))
            payload["owner"] = relative(target)
            with open(descriptor, "w", encoding="utf-8") as handle:
                json.dump(payload, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
        else:
            print(f"  !! no descriptor in {relative(target)}", file=sys.stderr)
        parent = os.path.dirname(origin)
        while parent.startswith(ROOT) and os.path.isdir(parent) and not os.listdir(parent):
            os.rmdir(parent)
            parent = os.path.dirname(parent)
    with open(aggregate, "w", encoding="utf-8") as handle:
        handle.write(rewritten)
    return len(moved)


if __name__ == "__main__":
    apply = "--apply" in sys.argv
    total = sum(restore(path, apply) for path in AGGREGATES)
    print(f"{'moved' if apply else 'would move'} {total} leaf dir(s)")
