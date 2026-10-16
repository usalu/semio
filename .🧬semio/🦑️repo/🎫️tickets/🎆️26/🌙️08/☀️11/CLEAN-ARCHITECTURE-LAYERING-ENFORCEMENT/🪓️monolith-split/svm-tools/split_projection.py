"""Row-table splitter for the schema projection module (svm): the `TYPES` array is cut at row boundaries into three child row tables
that the parent concatenates in a `const fn`. usage: split_projection.py [--install]; stages under 🗑️generated/svm-work/projection/stage."""
import hashlib, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from split import read, write, ROOT, F, WORK

REAL = "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
CHILDREN = [
    dict(folder="🧾️rows-a-h", mod="rows_a_h", doc="🧾️ Projection rows from `ActionAddress` to `HierarchyProvider`, in declaration order.", first=0, last=54),
    dict(folder="🎟️rows-h-t", mod="rows_h_t", doc="🎟️ Projection rows from `HoverSpec` to `TutorialDefinition`, in declaration order.", first=54, last=119),
    dict(folder="📖️rows-t-z", mod="rows_t_z", doc="📖️ Projection rows from `TutorialDocumentEvent` to the last row, in declaration order.", first=119, last=None),
]


def main():
    orig = read(F + "/svm-tools/orig/projection.rs").split("\n")
    if orig[-1] == "":
        orig.pop()
    head_end = next(i for i, l in enumerate(orig) if l == "    pub const TYPES: &[SchemaMetadata] = &[")
    rows = [i for i, l in enumerate(orig) if l == "        SchemaMetadata {"]
    tail = next(i for i in range(rows[-1], len(orig)) if orig[i] == "    ];")
    real_dir = os.path.dirname(ROOT + "/" + REAL)
    out = {}
    for c in CHILDREN:
        a = rows[c["first"]]
        b = rows[c["last"]] if c["last"] is not None else tail
        body = orig[a:b]
        assert body[0] == "        SchemaMetadata {" and body[-1] == "        },"
        text = [f"//! {c['doc']}", "", "    use super::SchemaMetadata;", "", "    pub(super) const ROWS: &[SchemaMetadata] = &["] + body + ["    ];"]
        out[f"{real_dir}/{c['folder']}/🦀️.rs"] = "\n".join(text) + "\n"
    mods = []
    for c in CHILDREN:
        mods += [f'    #[path = "{c["folder"]}/🦀️.rs"]', f'    mod {c["mod"]};']
    count = " + ".join(f'{c["mod"]}::ROWS.len()' for c in CHILDREN)
    parts = ", ".join(f'{c["mod"]}::ROWS' for c in CHILDREN)
    middle = (
        mods
        + ["", f"    const ROW_COUNT: usize = {count};", "",
           f"    pub const TYPES: &[SchemaMetadata] = &concat::<ROW_COUNT>(&[{parts}]);", "",
           "    /// 🧮️ Concatenates the child row tables into one table, preserving declaration order.",
           "    const fn concat<const N: usize>(parts: &[&[SchemaMetadata]]) -> [SchemaMetadata; N] {",
           '        let mut rows = [SchemaMetadata { name: "", version: 0, typescript: "" }; N];',
           "        let mut index = 0;",
           "        let mut part = 0;",
           "        while part < parts.len() {",
           "            let mut row = 0;",
           "            while row < parts[part].len() {",
           "                rows[index] = parts[part][row];",
           "                index += 1;",
           "                row += 1;",
           "            }",
           "            part += 1;",
           "        }",
           "        rows",
           "    }"]
    )
    parent = orig[:head_end] + middle + orig[tail + 1:]
    out[ROOT + "/" + REAL] = "\n".join(parent) + "\n"
    stage = f"{WORK}/projection/stage"
    if os.path.exists(stage):
        shutil.rmtree(stage)
    for p, t in out.items():
        write(f"{stage}/{os.path.relpath(p, ROOT)}", t)
        print(os.path.relpath(p, real_dir), t.count("\n"), "lines")
    if "--install" in sys.argv:
        real = ROOT + "/" + REAL
        assert hashlib.sha256(read(real).encode()).hexdigest() == hashlib.sha256(("\n".join(orig) + "\n").encode()).hexdigest(), "REAL FILE CHANGED SINCE SNAPSHOT"
        for p, t in out.items():
            if p != real:
                assert not os.path.exists(p)
                write(p, t)
        write(real, out[real])
        print("INSTALLED")


if __name__ == "__main__":
    main()
