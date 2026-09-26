"""N1 one-off codemod: moves every inline `#[cfg(test)] mod x { … }` the contract flags in 📕️norm into its canonical `🧪️tests/🔬️<name>/🦀️.rs` implementation, mounted by `#[path]`."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
MOVES = {
    "⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🏷️field-meta/🦀️.rs": {"field_meta_coverage": "field-meta-coverage"},
    "🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs": {"regen_assets": "regen-assets"},
    "📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs": {"tests": "unit"},
    "🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs": {"tests": "unit"},
    "🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs": {"unit_tests": "op-round-trip"},
    "🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs": {"tests": "unit"},
    "🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs": {"unit_tests": "op-round-trip"},
}
write = "--write" in sys.argv
for rel, modules in MOVES.items():
    path = ROOT + rel
    text = open(path, encoding="utf-8").read()
    for name, case in modules.items():
        head = re.search(rf"#\[cfg\(test\)\]\n(mod {name} \{{\n)", text)
        if not head:
            print("MISSING", rel, name); continue
        depth, i = 1, head.end()
        while depth:
            c = text[i]
            depth += c == "{"
            depth -= c == "}"
            i += 1
        body = text[head.end():i - 1]
        lines = [line[4:] if line.startswith("    ") else line for line in body.rstrip("\n").split("\n")]
        lines = [line for line in lines if "[DEBUG]" not in line]
        target_dir = os.path.join(os.path.dirname(path), "🧪️tests", f"🔬️{case}")
        target = os.path.join(target_dir, "🦀️.rs")
        if os.path.exists(target):
            print("EXISTS", target); continue
        doc = f"//! 🧪️ `{name}` — moved out of `{os.path.basename(os.path.dirname(path))}/🦀️.rs` into its canonical test implementation.\n"
        mount = f"#[cfg(test)]\n#[path = \"🧪️tests/🔬️{case}/🦀️.rs\"]\nmod {name};"
        text = text[:head.start()] + mount + text[i:]
        print(rel.split("/")[0], name, "->", f"🧪️tests/🔬️{case}/🦀️.rs", len(lines), "lines")
        if write:
            os.makedirs(target_dir, exist_ok=True)
            open(target, "w", encoding="utf-8").write(doc + "\n".join(lines) + "\n")
    if write: open(path, "w", encoding="utf-8").write(text)
