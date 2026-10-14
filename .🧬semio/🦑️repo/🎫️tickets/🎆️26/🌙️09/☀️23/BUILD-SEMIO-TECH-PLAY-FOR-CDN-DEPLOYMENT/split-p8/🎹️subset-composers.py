#!/usr/bin/env python3
"""🎹️ Lifts the extending subsets' composers out of an owning subset's `io_registry` into the format root's
`io_registry` (`subset_entries()`), so the owning subset no longer names the subsets that extend it; the root
declaration and the root registry serve them.

usage: 🎹️subset-composers.py <artifact dir (repo-rel)> <owning subset io root (artifact-rel)> [--apply]
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
A = os.path.join(REPO, sys.argv[1]); io_file = os.path.join(A, sys.argv[2]); apply = "--apply" in sys.argv
root = os.path.join(A, "🦀️.rs")
io = open(io_file, encoding="utf-8").read()
reg = re.search(r"(?ms)^pub mod io_registry \{.*?^\}\n", io)
block = reg.group(0)
own = re.search(r"use (crate::[\w:]+)::io::\w+ as \w+;", block)
own_prefix = own.group(1) if own else None
lifted = [m for m in re.finditer(r"(?m)^    use (crate::[\w:]+::io::(\w+));\n", block) if not own_prefix or not m.group(1).startswith(own_prefix + "::")]
if not lifted:
    raise SystemExit("nothing to lift")
new_block = block
for m in lifted:
    new_block = new_block.replace(m.group(0), "")
    new_block = re.sub(r",\s*composer_entry_of::<" + m.group(2) + r">\(\)", "", new_block)
io = io.replace(block, new_block)
r = open(root, encoding="utf-8").read()
rreg = re.search(r"(?ms)^pub mod io_registry \{.*?^\}\n", r)
rb = rreg.group(0)
uses = "".join(f"    use {m.group(1)};\n" for m in lifted) + ("    use semio_framework_plugin::composer_entry_of;\n" if "composer_entry_of" not in rb else "")
rb2 = rb.replace("    use std::sync::OnceLock;\n", uses + "    use std::sync::OnceLock;\n", 1)
entries = ", ".join(f"composer_entry_of::<{m.group(2)}>()" for m in lifted)
fn = f"""    static SUBSETS: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    /// 🧩️ The composers of the subsets that extend the owning subset; they register here, above it.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn subset_entries() -> &'static [ComposerEntry] {{
        SUBSETS.get_or_init(|| vec![{entries}]).as_slice()
    }}

"""
rb2 = re.sub(r"(    static ENTRIES: [^\n]*\n\n)", lambda m: m.group(1) + fn, rb2, count=1)
rb2 = re.sub(r"(ENTRIES\.get_or_init\(\|\| [^\n]*?)\.collect\(\)", lambda m: m.group(1) + ".chain(subset_entries().iter()).collect()", rb2, count=1)
rb2 = re.sub(r"(?m)^(        register_composer_entries\([^\n]*\n)(?!        register_composer_entries)", lambda m: m.group(1) + '        register_composer_entries(subset_entries()).expect("static Stdio registration must be available and conflict-free");\n', rb2, count=1)
r = r.replace(rb, rb2)
r = re.sub(r"(?m)^(        \.composers\([\w:]+::io::io_registry::entries\(\)\)\n)(?!        \.composers\(io_registry::subset_entries)", lambda m: m.group(1) + "        .composers(io_registry::subset_entries())\n", r, count=1)
print(rb2)
if apply:
    open(io_file, "w", encoding="utf-8").write(io); open(root, "w", encoding="utf-8").write(r)
