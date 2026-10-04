"""🧯️ S4-TOOLS-B — moves the forms, gis, energy, playbook and procedural plugin trees onto the `semio-framework-pack-error` API.

`store::PackError::Schema(<detail>)` becomes `store::PackError::Refusal(store::PackRefusal::Malformed { kind: InvalidValue,
what: "pack", offset: 0, detail: (<detail>).into() })` (a bare `map_err(store::PackError::Schema)` gets the closure form), and
`store::PackError::ValueRefusal` becomes `store::PackError::from` (the crate's `From<ValueError>`). Dry run by default (lists the
files), `--write` applies; idempotent.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
TREES = [ROOT / "✏️s/🔌️plugins" / name for name in ("📋️forms", "🌍️gis", "🔋️energy", "📖️playbook", "🌀️procedural")]
SCHEMA = "store::PackError::Schema"
REFUSAL = "store::PackError::Refusal(store::PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: \"pack\", offset: 0, detail: "


def balanced(text, start):
    depth, index = 0, start
    while True:
        char = text[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        elif char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        index += 1


def migrate(text):
    out, at = [], 0
    while True:
        found = text.find(SCHEMA, at)
        if found < 0:
            out.append(text[at:])
            break
        out.append(text[at:found])
        after = found + len(SCHEMA)
        if text[after] == "(":
            end = balanced(text, after)
            out.append(f"{REFUSAL}({text[after + 1:end]}).into() }})")
            at = end + 1
        else:
            out.append(f"|detail| {REFUSAL}detail }})")
            at = after
    migrated = "".join(out).replace("store::PackError::ValueRefusal(", "store::PackError::from(").replace("store::PackError::ValueRefusal", "store::PackError::from")
    migrated = re.sub(r'detail: \(("(?:[^"\\]|\\.)*")\.into\(\)\)\.into\(\) \}', r'detail: \1.into() }', migrated)
    migrated = re.sub(r'detail: \(([A-Za-z_][A-Za-z0-9_.]*\.to_string\(\))\)\.into\(\) \}', r'detail: \1 }', migrated)
    return re.sub(r'detail: \((format!\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\))\)\.into\(\) \}', r'detail: \1 }', migrated)


def main():
    write = "--write" in sys.argv[1:]
    if any(arg != "--write" for arg in sys.argv[1:]):
        sys.exit(f"unknown arguments {sys.argv[1:]}")
    changed = 0
    for tree in TREES:
        for path in tree.rglob("🦀️.rs"):
            if any(part in ("dist", "node_modules", "target") for part in path.parts):
                continue
            text = path.read_text(encoding="utf-8")
            after = migrate(text)
            if after != text:
                changed += 1
                print(f"{'migrated' if write else 'pending'} {path.relative_to(ROOT)}")
                if write:
                    path.write_text(after, encoding="utf-8")
    print(f"{changed} files {'migrated' if write else 'pending'}")


main()
