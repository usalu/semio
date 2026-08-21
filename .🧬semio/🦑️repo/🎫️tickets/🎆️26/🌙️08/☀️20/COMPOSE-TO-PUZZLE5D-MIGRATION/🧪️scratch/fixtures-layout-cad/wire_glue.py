#!/usr/bin/env python3
"""Mount every new 🧪️tests case from the plugin's 📦️glue.rs, immediately after that triad's
`pub mod inverse;` line and at the same indentation."""
import os, pathlib, re, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
INVERSE_DIR = "↩️inverse"
TESTS_DIR = "\U0001f9ea️tests"
RS = "\U0001f980️component.rs"


def wire(glue_path, mutations_root):
    leaves = {}
    for entry in sorted(os.listdir(mutations_root)):
        if (mutations_root / entry).is_dir():
            leaves[re.sub(r"^[^a-z]*", "", entry)] = entry

    cases = []  # (leaf_dir, case_dir_name, mod_name)
    for slug, entry in leaves.items():
        tests = mutations_root / entry / TESTS_DIR
        if not tests.is_dir():
            continue
        for case in sorted(os.listdir(tests)):
            if not (tests / case).is_dir():
                continue
            mod = "tests_" + re.sub(r"[^a-z0-9]+", "_", case)
            cases.append((entry, case, mod))

    text = glue_path.read_text(encoding="utf-8")
    lines = text.split("\n")
    out = []
    mounted = []
    i = 0
    while i < len(lines):
        line = lines[i]
        out.append(line)
        m = re.match(r'^(\s*)#\[path = "(.*/' + re.escape(INVERSE_DIR) + r'/' + re.escape(RS) + r')"\]$', line)
        if m and i + 1 < len(lines) and lines[i + 1].strip() == "pub mod inverse;":
            indent, path = m.group(1), m.group(2)
            out.append(lines[i + 1])
            i += 2
            prefix = path[: -len("/" + INVERSE_DIR + "/" + RS)]
            leaf_dir = prefix.rsplit("/", 1)[1]
            for (entry, case, mod) in cases:
                if entry != leaf_dir:
                    continue
                if f"/{TESTS_DIR}/{case}/{RS}" in text:
                    continue  # already mounted
                out.append(f'{indent}#[cfg(test)]')
                out.append(f'{indent}#[path = "{prefix}/{TESTS_DIR}/{case}/{RS}"]')
                out.append(f'{indent}mod {mod};')
                mounted.append((leaf_dir, case, mod))
            continue
        i += 1

    glue_path.write_text("\n".join(out), encoding="utf-8")
    return mounted


for plugin, artifact in [("\U0001f4cf️layout", "\U0001f4cf️layout"), ("\U0001f4d0️cad", "\U0001f4d0️cad")]:
    glue = ROOT / f"✏️s/\U0001f50c️plugins/{plugin}/\U0001f4e6️packages/\U0001f980️rust/\U0001f4e6️glue.rs"
    mutations = ROOT / f"✏️s/\U0001f50c️plugins/{plugin}/\U0001f5ff️artifacts/{artifact}/\U0001f3c5️standards/\U0001f516️1/\U0001fa86️subsets/✳️any/\U0001f9ec️schema/\U0001f9ec️mutations"
    assert glue.is_file(), glue
    assert mutations.is_dir(), mutations
    mounted = wire(glue, mutations)
    print(plugin, "mounted", len(mounted))
    for row in mounted:
        print("   ", row[0], row[1], row[2])
