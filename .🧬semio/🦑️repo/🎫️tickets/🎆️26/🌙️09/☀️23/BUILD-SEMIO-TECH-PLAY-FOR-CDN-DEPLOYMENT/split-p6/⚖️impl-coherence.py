#!/usr/bin/env python3
"""⚖️ Lists impl blocks a crate cut made illegal: inherent impls of a type another family crate defines (E0116) and
trait impls where neither the trait nor the self type (nor any type parameter of the trait) is local (E0117).

Types and traits are attributed to the family crate whose lib closure defines them at top level. A trait whose name is
not defined by any family crate counts as foreign. Generic blanket impls (`impl<T> Tr for T`) are skipped.

usage: ⚖️impl-coherence.py <workspace dir> <family package prefix>
"""
import json, os, re, subprocess, sys, importlib.util, collections

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py"))
relocate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(relocate)
workspace_dir, prefix = os.path.join(REPO, sys.argv[1]), sys.argv[2]


def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text())))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def closure(root):
    out = subprocess.run(["bun", os.path.join(HERE, "📏️unit-files.ts"), root, "test", "--files"], capture_output=True, text=True, cwd=HERE)
    return [os.path.join(REPO, line.split("\t")[4]) for line in out.stdout.splitlines() if line.startswith("F\t") and line.split("\t")[3] == "source"]


workspace = toml_load(os.path.join(workspace_dir, "Cargo.toml"))
crates = {}
for member in workspace["workspace"]["members"]:
    manifest = toml_load(os.path.join(workspace_dir, member, "Cargo.toml"))
    name = manifest["package"]["name"]
    if name.startswith(prefix):
        crates[name] = closure(os.path.normpath(os.path.join(workspace_dir, member, manifest.get("lib", {}).get("path", "src/lib.rs"))))
types, traits = collections.defaultdict(set), collections.defaultdict(set)
for crate, files in crates.items():
    for path in files:
        text = relocate.strip(open(path, encoding="utf-8").read())
        for m in re.finditer(r"(?m)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?(struct|enum|union|type|trait)\s+(\w+)", text):
            (traits if m.group(1) == "trait" else types)[m.group(2)].add(crate)
        for m in re.finditer(r"\b(?:struct|enum|trait|type|fn)\s+(\w+)", "\n".join(re.findall(r"\w+!\s*[\(\{\[]([^;]*?)[\)\}\]]\s*;?", text))):
            types[m.group(1)].add(crate)
        for m in re.finditer(r"(?m)^[ \t]*(?:[\w:]+::)?\w+!\s*\(\s*(\w+)\s*,", text):
            types[m.group(1)].add(crate)
IMPL = re.compile(r"(?m)^[ \t]*(?:unsafe\s+)?impl\b\s*(<(?:[^<>]|<(?:[^<>]|<[^<>]*>)*>)*>)?\s*([^{;]*?)\s*(?:where\b[^{]*)?\{")
bad = []
for crate, files in crates.items():
    for path in files:
        raw = open(path, encoding="utf-8").read()
        text = relocate.strip(raw)
        for m in IMPL.finditer(text):
            generics, head = m.group(1) or "", m.group(2)
            params = set(re.findall(r"\b([A-Z]\w*)\b(?=\s*[:,>])", generics))
            if " for " in f" {head} ":
                trait, ty = re.split(r"\s+for\s+", head, maxsplit=1)
            else:
                trait, ty = None, head
            core = re.sub(r"^[&\s]*(?:mut\s+)?(?:dyn\s+)?", "", ty.strip())
            tname = re.findall(r"\w+", re.sub(r"<.*", "", core))
            tname = tname[-1] if tname else "?"
            if tname in params:
                continue
            owners = types.get(tname, set())
            line = raw.count("\n", 0, m.start()) + 1
            where = f"{os.path.relpath(path, REPO).split('/🪆️subsets/')[-1]}:{line}"
            if trait is None:
                if owners and crate not in owners:
                    bad.append(("E0116", crate, f"impl {tname}", sorted(owners), where))
                continue
            trname = re.findall(r"\w+", re.sub(r"<.*", "", trait.strip()))
            trname = trname[-1] if trname else "?"
            trait_local = crate in traits.get(trname, set())
            args = set(re.findall(r"\b(\w+)\b", trait[trait.find("<"):] if "<" in trait else ""))
            arg_local = any(crate in types.get(a, set()) for a in args)
            ty_local = crate in owners or (not owners and tname not in ("Vec", "Option", "Box", "String"))
            if not (trait_local or ty_local or arg_local):
                bad.append(("E0117", crate, f"impl {trname} for {tname}", sorted(owners), where))
for code, crate, what, owners, where in sorted(bad):
    print(f"{code} {crate[len(prefix):] or '<agg>'}: {what}  (type in {[o[len(prefix):] or '<agg>' for o in owners]})  {where}")
print(f"violations: {len(bad)}")
