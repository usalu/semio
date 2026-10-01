"""🧪️ W2-W-norm-2 (design §14, option B): renames evidence case directories to the norm-wide outcome vocabulary and
moves over-long leaf directories to a shorter emoji. Writes ONLY inside the given norm artifact directory; the schema
catalog entries outside it are edited separately and by hand.

  python3 🧪️w2w-norm-2-short-cases.py cases <artifact-dir-name>                 e.g. 🪨️en1996
  python3 🧪️w2w-norm-2-short-cases.py leaf <artifact-dir-name> <old-leaf> <new-leaf>
  python3 🧪️w2w-norm-2-short-cases.py field <artifact-dir-name> <old-leaf> <new-leaf> <old_field> <new_field>
"""

import json
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
WITNESS = "🧾️wire-witness"
#: 📖️ The shared vocabulary: an outcome class → its case directory (Emoji_Presentation emoji, no U+FE0F).
VOCABULARY = {
    "applied": "✅apply",
    "mutation.no-op": "🟰noop",
    "mutation.target-missing": "❓gone",
    "mutation.target-referenced": "🔗used",
    "mutation.target-mismatch": "🔀other",
    "mutation.partial": "🧩part",
    "mutation.clamped": "📏clamp",
    "mutation.duplicate-id": "⛔dupe",
    "mutation.invariant": "🚫rule",
    "mutation.cascade": "🌊cascade",
}
TEXT = (".rs", ".json", ".py", ".feature", ".ts", ".md", ".semio", ".proto", ".graphql")


def subset(artifact):
    return f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def outcome_class(bundle):
    outcome = json.load(open(f"{bundle}/🎯️outcome/🔣️.json"))
    if outcome["status"] == "applied":
        codes = [message["code"] for message in outcome.get("messages", [])]
        return codes[0] if codes else "applied"
    return outcome["code"]


def text_files(artifact):
    root = f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/{artifact}"
    for directory, directories, names in os.walk(root):
        directories[:] = [name for name in directories if name not in ("dist", "node_modules", "target")]
        for name in names:
            if name.endswith(TEXT):
                yield os.path.join(directory, name)


def rewrite(artifact, replacements):
    changed = 0
    for path in text_files(artifact):
        try:
            text = open(path, encoding="utf-8").read()
        except UnicodeDecodeError:
            continue
        new = text
        for old, value in replacements:
            new = new.replace(old, value)
        if new != text:
            open(path, "w", encoding="utf-8").write(new)
            changed += 1
    return changed


def cases(artifact):
    fixtures = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
    sources = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    replacements, moves = [], []
    for leaf in sorted(os.listdir(fixtures)):
        if not os.path.isdir(f"{fixtures}/{leaf}"):
            continue
        taken = {}
        for old in sorted(name for name in os.listdir(f"{fixtures}/{leaf}") if name != WITNESS):
            base = VOCABULARY[outcome_class(f"{fixtures}/{leaf}/{old}")]
            taken[base] = taken.get(base, 0) + 1
            new = base if taken[base] == 1 else f"{base}{taken[base]}"
            if new == old:
                continue
            moves.append((f"{fixtures}/{leaf}/{old}", f"{fixtures}/{leaf}/{new}"))
            if os.path.isdir(f"{sources}/{leaf}/🧪️tests/{old}"):
                moves.append((f"{sources}/{leaf}/🧪️tests/{old}", f"{sources}/{leaf}/🧪️tests/{new}"))
            replacements += [(f"{leaf}/{old}{end}", f"{leaf}/{new}{end}") for end in ("/", " ", "`", "\n")]
            replacements += [(f"{leaf}/🧪️tests/{old}/", f"{leaf}/🧪️tests/{new}/"), (f'"{leaf}", "{old}"', f'"{leaf}", "{new}"')]
            replacements.append(("PENDING-LEAF-RELATIVE", (leaf, old, new)))
    for source, target in moves:
        assert not os.path.exists(target), target
        os.rename(source, target)
    plain = [pair for pair in replacements if pair[0] != "PENDING-LEAF-RELATIVE"]
    changed = rewrite(artifact, plain)
    relative = 0
    for _, (leaf, old, new) in (pair for pair in replacements if pair[0] == "PENDING-LEAF-RELATIVE"):
        path = f"{sources}/{leaf}/🦀️.rs"
        if os.path.exists(path):
            text = open(path, encoding="utf-8").read()
            new_text = text.replace(f'"🧪️tests/{old}/', f'"🧪️tests/{new}/')
            if new_text != text:
                open(path, "w", encoding="utf-8").write(new_text)
                relative += 1
    print(f"{artifact}: {len(moves)} directories moved, {changed} files rewritten, {relative} leaf mounts updated")


def leaf(artifact, old, new):
    for root in (f"{subset(artifact)}/🧬️schema/🧬️mutations", f"{subset(artifact)}/🧫️fixtures/🧬️mutations"):
        if os.path.isdir(f"{root}/{old}"):
            assert not os.path.exists(f"{root}/{new}")
            os.rename(f"{root}/{old}", f"{root}/{new}")
    old_emoji, new_emoji = re.match(r"^\D+?(?=[a-z])", old).group(0), re.match(r"^\D+?(?=[a-z])", new).group(0)
    changed = rewrite(artifact, [(f"/{old}/", f"/{new}/"), (f"/{old}\"", f"/{new}\""), (f'"{old}"', f'"{new}"')])
    descriptor = f"{subset(artifact)}/🧬️schema/🧬️mutations/{new}/🔣️.json"
    text = open(descriptor, encoding="utf-8").read()
    open(descriptor, "w", encoding="utf-8").write(text.replace(f'"emoji": "{old_emoji}"', f'"emoji": "{new_emoji}"'))
    source = f"{subset(artifact)}/🧬️schema/🧬️mutations/{new}/🦀️.rs"
    text = open(source, encoding="utf-8").read()
    open(source, "w", encoding="utf-8").write(text.replace(f"//! {old_emoji} ", f"//! {new_emoji} ", 1))
    print(f"{artifact}: {old} -> {new}; {changed} files rewritten")


def field(artifact, old_leaf, new_leaf, old_snake, new_snake):
    """🔤 Renames one scalar field and the `change-<field>` kind derived from it, in every spelling (snake, camel,
    Pascal, kebab), and moves the kind's schema leaf and fixture leaf to `new_leaf`."""
    pascal = lambda snake: "".join(part.title() for part in snake.split("_"))
    camel = lambda snake: pascal(snake)[:1].lower() + pascal(snake)[1:]
    kebab = lambda snake: snake.replace("_", "-")
    for root in (f"{subset(artifact)}/🧬️schema/🧬️mutations", f"{subset(artifact)}/🧫️fixtures/🧬️mutations"):
        if os.path.isdir(f"{root}/{old_leaf}"):
            assert not os.path.exists(f"{root}/{new_leaf}")
            os.rename(f"{root}/{old_leaf}", f"{root}/{new_leaf}")
    spellings = [(old_leaf, new_leaf)] + [(spell(old_snake), spell(new_snake)) for spell in (pascal, camel, kebab, lambda snake: snake)]
    changed = rewrite(artifact, spellings)
    print(f"{artifact}: {old_snake} -> {new_snake}, {old_leaf} -> {new_leaf}; {changed} files rewritten")


def repair(artifact, renamed_leaves):
    """🩹 Re-applies the reference rewrite after directories were already moved, from the catalog's previous names."""
    manifest = json.load(open(f"{subset(artifact)}/🔮️oracles/🔣️.json"))
    fixtures = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
    sources = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    replacements, mounts = [], []
    for vector in manifest["mutationCatalogs"][0]["vectors"]:
        old_leaf = vector["sourceMutationDirectoryName"]
        leaf = renamed_leaves.get(old_leaf, old_leaf)
        current = sorted(name for name in os.listdir(f"{fixtures}/{leaf}") if name != WITNESS)
        for scenario, new in zip(vector["scenarios"], current):
            old = scenario["directoryName"]
            if old == new:
                continue
            replacements += [(f"{old_leaf}/{old}/", f"{old_leaf}/{new}/"), (f"{old_leaf}/🧪️tests/{old}/", f"{old_leaf}/🧪️tests/{new}/"), (f'"{old_leaf}", "{old}"', f'"{old_leaf}", "{new}"')]
            mounts.append((leaf, old, new))
    for old_leaf, leaf in renamed_leaves.items():
        replacements += [(f"/{old_leaf}/", f"/{leaf}/"), (f"/{old_leaf}\"", f"/{leaf}\""), (f'"{old_leaf}"', f'"{leaf}"')]
    changed = rewrite(artifact, replacements)
    relative = 0
    for leaf, old, new in mounts:
        path = f"{sources}/{leaf}/🦀️.rs"
        text = open(path, encoding="utf-8").read()
        new_text = text.replace(f'"🧪️tests/{old}/', f'"🧪️tests/{new}/')
        if new_text != text:
            open(path, "w", encoding="utf-8").write(new_text)
            relative += 1
    print(f"{artifact}: repaired {changed} files, {relative} leaf mounts")


if __name__ == "__main__":
    {"cases": lambda: cases(sys.argv[2]), "leaf": lambda: leaf(sys.argv[2], sys.argv[3], sys.argv[4]), "field": lambda: field(*sys.argv[2:7]), "repair": lambda: repair(sys.argv[2], dict(pair.split("=>") for pair in sys.argv[3:]))}[sys.argv[1]]()
