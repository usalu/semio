#!/usr/bin/env python3
"""S-4: mechanically add a 4th `mutations: FacetLeaves` block to every
`*_artifact_schema_descriptor()` constructor, mirroring the existing `diff` block
but pointing at `🧬️mutations/*` leaves instead of `🔺️diff/*`."""
import re, subprocess, sys

ROOT = "."

def ctor_files():
    out = subprocess.run(
        ["grep", "-rln", "fn.*_artifact_schema_descriptor()", "--include=*.rs", ROOT],
        capture_output=True, text=True
    ).stdout.strip().split("\n")
    return [f for f in out if f]

DIFF_BLOCK_RE = re.compile(
    r'(?P<indent>[ \t]*)diff:\s*(?P<qual>[A-Za-z_][A-Za-z0-9_]*::)?FacetLeaves\s*\{\n'
    r'(?P<inner>[ \t]*)rust:\s*include_str!\("🔺️diff/🦀️component\.rs"\),\n'
    r'[ \t]*typescript:\s*include_str!\("🔺️diff/🟦️component\.ts"\),\n'
    r'[ \t]*graphql:\s*include_str!\("🔺️diff/🔗️component\.graphql"\),\n'
    r'[ \t]*json_schema:\s*include_str!\("🔺️diff/🔣️component\.json"\),\n'
    r'[ \t]*proto:\s*include_str!\("🔺️diff/🛰️component\.proto"\),\n'
    r'(?P=indent)\},\n'
)

def build_mutations_block(indent: str, inner: str, qual: str) -> str:
    q = qual or ""
    return (
        f'{indent}mutations: {q}FacetLeaves {{\n'
        f'{inner}rust: include_str!("🧬️mutations/🦀️component.rs"),\n'
        f'{inner}typescript: include_str!("🧬️mutations/🟦️component.ts"),\n'
        f'{inner}graphql: include_str!("🧬️mutations/🔗️component.graphql"),\n'
        f'{inner}json_schema: include_str!("🧬️mutations/🔣️component.json"),\n'
        f'{inner}proto: include_str!("🧬️mutations/🛰️component.proto"),\n'
        f'{indent}}},\n'
    )

def main():
    files = ctor_files()
    changed, skipped_already, failed = [], [], []
    for f in files:
        with open(f, encoding="utf-8") as fh:
            content = fh.read()
        if "mutations: " in content and "🧬️mutations/🦀️component.rs" in content:
            skipped_already.append(f)
            continue
        m = DIFF_BLOCK_RE.search(content)
        if not m:
            failed.append(f)
            continue
        insertion = build_mutations_block(m.group("indent"), m.group("inner"), m.group("qual"))
        new_content = content[: m.end()] + insertion + content[m.end():]
        with open(f, "w", encoding="utf-8") as fh:
            fh.write(new_content)
        changed.append(f)
    print(f"changed: {len(changed)}")
    print(f"already had mutations block: {len(skipped_already)}")
    print(f"FAILED (no diff-block match): {len(failed)}")
    for x in failed:
        print("  FAILED:", x)
    return 0 if not failed else 1

if __name__ == "__main__":
    sys.exit(main())
