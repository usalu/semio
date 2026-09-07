#!/usr/bin/env python3
"""📇️ W13 — registers the 23 new fem2d vectors in all three places discovery is EXPLICIT.

Case discovery is not directory-driven in any of the three languages (W10 finding F9), so a new
bundle on disk is invisible until it is named:

1. **Rust** — one `#[cfg(test)] #[path = …] mod tests_…;` per case in the plugin's crate entry,
   inserted beside the kind's existing mounts.
2. **Oracle catalog** — one `{id, directoryName}` row under the kind's `vectors[].scenarios`, in the
   owning subset's `🔮️oracle/🔣️.json`. The coordinator's `mutationVectorRegistryBreaches` reports
   any physical directory that is not registered here, and any registered one whose bundle is not
   the closed shape.
3. **`🔣️taxonomy.json`** — one `members-of-tests.memberNames` entry per directory name.

All three patches are idempotent and ANCHORED: the taxonomy file in particular is shared with every
other concurrent worker, so this script inserts a contiguous block after one unique existing line
and never rewrites the file.

Usage:
    uv run python 🔨️w13-fem2d-register.py            # patch
    uv run python 🔨️w13-fem2d-register.py --check    # report, write nothing
"""

# region 🔖️Imports
import importlib.util
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Harness
HERE = os.path.dirname(os.path.abspath(__file__))


def load(name, stem):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, stem))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


cases = load("w13_cases", "🔨️w13-fem2d-cases.py")
REPO, SUBSETS, CRATE_ENTRY, TAXONOMY = cases.REPO, cases.SUBSETS, cases.CRATE_ENTRY, cases.TAXONOMY
TAXONOMY_ANCHOR = '        "🚫️rejects-a-missing-4271bc",\n'
"""⚓️ A W10 fem2d entry, unique in the file — this wave's block goes directly after it."""
# endregion 🔖️Harness


# region 🔖️Mounts
def mount_block(entry):
    """🔩️ One case's `#[cfg(test)]` mount, at the crate entry's own 36-space indentation."""
    pad = " " * 36
    path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/%s/🧬️schema/🧬️mutations/%s/🧪️tests/%s/🦀️.rs" % (cases.SUBSET_OF[entry["kind"]], cases.KIND_DIR[entry["kind"]], entry["directory"])
    return "%s#[cfg(test)]\n%s#[path = \"%s\"]\n%smod %s;\n" % (pad, pad, path, pad, entry["module"])


def patch_mounts(check):
    """🔩️ Inserts each mount after the LAST existing `🧪️tests` mount of its own fem2d kind."""
    text = open(CRATE_ENTRY, encoding="utf-8").read()
    added = 0
    for entry in cases.CASES:
        block = mount_block(entry)
        if block in text:
            continue
        needle = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/%s/🧬️schema/🧬️mutations/%s/🧪️tests/" % (cases.SUBSET_OF[entry["kind"]], cases.KIND_DIR[entry["kind"]])
        lines = text.split("\n")
        last = None
        for at, line in enumerate(lines):
            if needle in line and line.lstrip().startswith("#[path"):
                last = at + 1
        assert last is not None, entry["label"]
        assert lines[last].lstrip().startswith("mod tests_"), (entry["label"], lines[last])
        lines.insert(last + 1, block.rstrip("\n"))
        text = "\n".join(lines)
        added += 1
    if added and not check:
        with open(CRATE_ENTRY, "w", encoding="utf-8") as handle:
            handle.write(text)
    return added


# endregion 🔖️Mounts


# region 🔖️Catalog
def patch_catalogs(check):
    """📇️ Appends one `{id, directoryName}` scenario row per case to its kind's vector."""
    added = 0
    for subset in sorted({cases.SUBSET_OF[entry["kind"]] for entry in cases.CASES}):
        path = os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json")
        original = open(path, encoding="utf-8").read()
        catalog = json.loads(original)
        for entry in cases.CASES:
            if cases.SUBSET_OF[entry["kind"]] != subset:
                continue
            directory = cases.KIND_DIR[entry["kind"]]
            found = False
            for declaration in catalog["mutationCatalogs"]:
                for vector in declaration["vectors"]:
                    if vector["mutationDirectoryName"] != directory:
                        continue
                    found = True
                    if any(scenario["directoryName"] == entry["directory"] for scenario in vector["scenarios"]):
                        continue
                    vector["scenarios"].append({"id": entry["id"], "directoryName": entry["directory"]})
                    added += 1
            assert found, "%s: no catalog vector for %s" % (entry["label"], directory)
        text = json.dumps(catalog, indent=2, ensure_ascii=False) + "\n"
        if text != original and not check:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(text)
    return added


# endregion 🔖️Catalog


# region 🔖️Taxonomy
def patch_taxonomy(check):
    """🗂️ Inserts this wave's directory names as one contiguous block after a unique anchor line."""
    text = open(TAXONOMY, encoding="utf-8").read()
    assert text.count(TAXONOMY_ANCHOR) == 1, "the taxonomy anchor must be unique, found %d" % text.count(TAXONOMY_ANCHOR)
    missing = [entry["directory"] for entry in cases.CASES if ('        "%s",\n' % entry["directory"]) not in text]
    if not missing:
        return 0
    block = "".join('        "%s",\n' % name for name in missing)
    text = text.replace(TAXONOMY_ANCHOR, TAXONOMY_ANCHOR + block, 1)
    payload = json.loads(text)
    names = payload["semanticDirectoryMemberKinds"]["members-of-tests"]["memberNames"]
    assert len(names) == len(set(names)), "members-of-tests must stay duplicate-free"
    if not check:
        with open(TAXONOMY, "w", encoding="utf-8") as handle:
            handle.write(text)
    return len(missing)


# endregion 🔖️Taxonomy


# region 🔖️Main
def main():
    check = "--check" in sys.argv[1:]
    cases.prepare()
    print("crate-entry mounts added: %d" % patch_mounts(check))
    print("oracle catalog scenarios added: %d" % patch_catalogs(check))
    print("taxonomy members-of-tests names added: %d" % patch_taxonomy(check))
    return 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Main
