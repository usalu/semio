"""🧪️ W2-W-norm-3 follow-up: renames mutation kinds of one norm artifact everywhere at once (coordinator-approved
renames only) — leaf and fixture directories, kind, variant, camelCase wire tag, module, binary tag constant, descriptor
entity, plus the exact label/display replacements given. No alias remains.

`python3 🧪️w2-w-norm-3-rename.py <artifact dir> <old-kind>=<new-kind> … [--text '<old>'='<new>' …]`
"""
import os
import sys

NORM = "✏️s/🔌️plugins/📕️norm/🗿️artifacts"


def pascal(kind):
    return "".join(part.capitalize() for part in kind.split("-"))


def main(argv):
    artifact = f"{NORM}/{argv[0]}"
    subset = f"{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    kinds, texts, mode = [], [], "kinds"
    for arg in argv[1:]:
        if arg == "--text":
            mode = "text"
            continue
        old, new = arg.split("=", 1)
        (kinds if mode == "kinds" else texts).append((old, new))
    dirs = {name[len(name) - len(old):]: name for name in os.listdir(f"{subset}/🧬️schema/🧬️mutations") for old, _ in kinds if name.endswith(old) and name[: len(name) - len(old)] and not name[: len(name) - len(old)][-1].isalnum()}
    pairs = []
    for old, new in kinds:
        old_dir = dirs[old]
        pairs += [(old_dir, old_dir[: len(old_dir) - len(old)] + new), (old, new), (pascal(old), pascal(new)), (pascal(old)[0].lower() + pascal(old)[1:], pascal(new)[0].lower() + pascal(new)[1:]), (old.replace("-", "_"), new.replace("-", "_")), ("TAG_" + old.replace("-", "_").upper(), "TAG_" + new.replace("-", "_").upper()), (f'entity: "{old.split("-", 1)[1]}"', f'entity: "{new.split("-", 1)[1]}"')]
    changed = 0
    for root, folders, files in os.walk(artifact):
        folders[:] = [name for name in folders if name not in ("dist", "target", "node_modules")]
        for name in files:
            if not name.endswith((".rs", ".ts", ".json", ".py", ".feature", ".semio", ".graphql", ".proto", ".md", ".g4", ".ebnf")):
                continue
            path = os.path.join(root, name)
            try:
                text = open(path, encoding="utf-8").read()
            except UnicodeDecodeError:
                continue
            new_text = text
            for old, new in pairs + texts:
                new_text = new_text.replace(old, new)
            if new_text != text:
                open(path, "w", encoding="utf-8").write(new_text)
                changed += 1
    for old, new in kinds:
        for parent in (f"{subset}/🧬️schema/🧬️mutations", f"{subset}/🧫️fixtures/🧬️mutations"):
            if os.path.isdir(f"{parent}/{dirs[old]}"):
                os.rename(f"{parent}/{dirs[old]}", f"{parent}/{dirs[old][: len(dirs[old]) - len(old)] + new}")
    print("rewrote", changed, "files; renamed", len(kinds), "kinds")


if __name__ == "__main__":
    main(sys.argv[1:])
