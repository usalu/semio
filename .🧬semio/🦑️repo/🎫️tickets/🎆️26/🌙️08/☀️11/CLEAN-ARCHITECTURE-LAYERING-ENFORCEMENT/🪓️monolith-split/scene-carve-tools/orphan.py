"""⚖️ Orphan and inherent-impl scan: every impl of a carved crate must either be inherent on a type the crate defines or implement a trait or a type the crate defines."""
import re, sys
from lib import *
from model import *
from union import owned_files
from rsitems import split_items

STD = {"Vec", "Option", "String", "Box", "Result", "u8", "u16", "u32", "u64", "i32", "i64", "f32", "f64", "bool", "str", "usize", "isize", "char", "HashMap", "BTreeMap", "BTreeSet", "Mutex", "Arc", "Rc"}


def collect_defs(files, tree):
    defs = set()
    for f in files:
        for it in split_items(rd(tree + "/" + f)):
            if it.kind in ("struct", "enum", "union", "trait", "type"):
                defs.add(it.name)
            if it.kind == "other":
                for m in re.finditer(r"string_enum!\s*\{(?:\s*///[^\n]*\n|\s*#\[[^\n]*\]\n)*\s*(\w+)\s*\{", it.text):
                    defs.add(m.group(1))
                for m in re.finditer(r"scene_pack_wire!\(\s*(\w+)", it.text):
                    defs.add(m.group(1))
            if it.kind == "other" and it.name.startswith("scene_pack_wire"):
                m = re.match(r"scene_pack_wire!\(\s*(\w+)", it.text.strip())
                if m:
                    defs.add(m.group(1))
    return defs


def impls(files, tree):
    out = []
    for f in files:
        text = rd(tree + "/" + f)
        for it in split_items(text):
            if it.kind == "impl":
                head = re.sub(r"\s+", " ", it.text.split("{")[0] if "{" in it.text else it.text)
                head = re.sub(r"^(?:\s*(?:///[^\n]*|#\[[^\]]*\]))*", "", head).strip()
                out.append((f, head))
        for m in re.finditer(r"^\s*(?:unsafe\s+)?impl(?:<[^>]*>)?\s+([^{;]+?)\s*(?:where[^{]*)?\{", text, re.M):
            pass
    return out


def check(tree, keys):
    bad = []
    alldefs = {}
    for k in keys:
        files = sorted(set(owned_files(tree, CRATES[k]["node"] + "/🦀️.rs"))) if k != "facade" else sorted(set(owned_files(tree, "📦️packages/🦀️rust/🦀️.rs")))
        alldefs[k] = (files, collect_defs(files, tree))
    for k in keys:
        files, defs = alldefs[k]
        for f, head in impls(files, tree):
            m = re.match(r"(?:unsafe\s+)?impl(?:<[^>]*>)?\s+(.*)$", head)
            if not m:
                continue
            rest = m.group(1)
            if " for " in rest:
                trait, ty = rest.split(" for ", 1)
                tname = re.sub(r"<.*", "", trait.strip()).split("::")[-1]
                tbase = re.sub(r"^[&\[\(]+(?:mut\s+)?", "", ty.strip())
                tbase = re.sub(r"<.*", "", tbase).split("::")[-1].split(" ")[0]
                if tname not in defs and tbase not in defs:
                    bad.append((k, f, head))
            else:
                tbase = re.sub(r"<.*", "", rest.strip()).split("::")[-1].split(" ")[0]
                if tbase not in defs:
                    bad.append((k, f, head))
    return bad


if __name__ == "__main__":
    tree = sys.argv[1]
    keys = sys.argv[2:]
    for b in check(tree, keys):
        print(*b)
    print("done")
