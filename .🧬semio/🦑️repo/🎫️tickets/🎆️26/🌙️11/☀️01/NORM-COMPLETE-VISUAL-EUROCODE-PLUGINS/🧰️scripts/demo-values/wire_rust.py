"""Wires cases/🌱️demo into `catalogue_demo_cases` of each code (idempotent). Run with python3 -I wire_rust.py."""
import re, sys
sys.path.insert(0, "."); from dv_lib import *
CASES = {"en1990": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1991": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases", "en1992": "🧬️schema/💡️inferences/⚖️checks/🗄️registry/🗂️cases", "en1995": "🧬️schema/⚖️kernel/📇️catalogue/🗂️cases", "en1996": "🧬️schema/📇️masonry-catalogue/🗂️cases", "en1997": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue/🗂️cases"}
DIRS["en1992"] = "🏛️en1992"
for code, rel in CASES.items():
    p = art(code) / rel / "🦀️.rs"
    t = p.read_text(encoding="utf-8")
    if "demo::demo_values" in t:
        print(code, "already wired"); continue
    pat = re.compile(r'(\w+Cases)::entries\(\)\s*\.iter\(\)\s*\.map\(\|entry\| ([\w:]+)::from\(&design_case::seed::<\1>\(entry, &(?:demo_case_id\(entry\.id\)|format!\("case-\{\}", entry\.id\.to_lowercase\(\)\))\)\)\)\s*\.collect\(\)')
    m = pat.search(t)
    assert m, code
    new = f'{m[1]}::entries()\n        .iter()\n        .map(|entry| {{\n            let mut case = design_case::seed::<{m[1]}>(entry, &demo_case_id(entry.id));\n            case.values = demo::demo_values(entry.id, case.values);\n            {m[2]}::from(&case)\n        }})\n        .collect()'
    t = t[:m.start()] + new + t[m.end():]
    decl = '\n#[path = "🌱️demo/🦀️.rs"]\nmod demo;\n'
    if "#[cfg(test)]" in t:
        t = t.replace("\n#[cfg(test)]", decl + "\n#[cfg(test)]", 1)
    else:
        t = t.rstrip("\n") + "\n" + decl
    p.write_text(t, encoding="utf-8")
    print(code, "wired")
