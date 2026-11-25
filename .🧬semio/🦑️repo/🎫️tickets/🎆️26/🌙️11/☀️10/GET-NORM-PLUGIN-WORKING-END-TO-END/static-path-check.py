"""🔎️ Lists the first segment of every crate:: / super:: path and the glob-imported names of the test files a laws crate mounts, and reports the ones the crate under test does not declare publicly in its own closure.
usage: static-path-check.py <laws-crate> <crate-under-test>"""
import json, os, re, subprocess, sys
laws, under = sys.argv[1:3]
here = os.path.dirname(os.path.abspath(__file__))
def closure(name):
    jobs = subprocess.run(["python3", here + "/real-closure-jobs.py", name], capture_output=True, text=True, check=True).stdout
    open(here + "/🗑️generated/spc-jobs.json", "w").write(jobs)
    out = subprocess.run(["node", "--no-warnings", here + "/real-closure-probe.mjs", here + "/🗑️generated/spc-jobs.json"], capture_output=True, text=True, env={**os.environ, "DUMP": "1"}, check=True).stdout
    return [r for r in json.loads(out) if r["unit"] == "lib[cfg(test)]"][0]["inputs"]
lf = [p for p, *_ in closure(laws)]
uf = [p for p, *_ in closure(under)]
used = {}
for p in lf:
    t = open(p, encoding="utf8").read()
    for m in re.finditer(r"\b(?:crate|super)::(\w+)", t):
        used.setdefault(m.group(1), set()).add(os.path.basename(os.path.dirname(p)))
    for m in re.finditer(r"use (?:crate|super)::\{([^}]*)\}", t):
        for n in re.findall(r"\b(\w+)\b", m.group(1)):
            used.setdefault(n, set()).add("brace")
src = "\n".join(open(p, encoding="utf8").read() for p in uf)
declared = set(re.findall(r"\bpub\s+(?:async\s+)?(?:mod|fn|struct|enum|const|static|type|trait|union)\s+(\w+)", src))
declared |= set(re.findall(r"\bpub use [^;]*?\bas (\w+);", src))
declared |= set(re.findall(r"\bpub use [^;]*?::(\w+);", src))
declared |= set(re.findall(r"\bpub extern crate [^;]*? as (\w+);", src))
declared |= set(re.findall(r"macro_rules!\s+(\w+)", src))
local = set()
for p in lf:
    t = open(p, encoding="utf8").read()
    local |= set(re.findall(r"\bmod\s+(\w+)\s*[;{]", t)) | set(re.findall(r"\bfn\s+(\w+)", t))
for n in sorted(used):
    where = ",".join(sorted(used[n]))
    flag = "pub" if n in declared else ("local" if n in local else "NOT-PUB?")
    print(f"{flag:9}{n:40}{where}")

print("--- glob-provided names (used unqualified, not imported or defined in the laws crate): pub in crate under test? ---")
allsrc = "\n".join(open(p, encoding="utf8").read() for p in lf)
imported = set()
for m in re.finditer(r"\buse\s+([^;]+);", allsrc):
    imported |= set(re.findall(r"\b(\w+)\b", m.group(1)))
defined = set(re.findall(r"\b(?:fn|struct|enum|const|static|type|trait|mod|macro_rules!)\s+(\w+)", allsrc)) | set(re.findall(r"\blet\s+(?:mut\s+)?(\w+)", allsrc)) | set(re.findall(r"[|(,]\s*(?:mut\s+)?(\w+)\s*(?::|,|\)|\|)", allsrc))
code = re.sub(r'"(?:\\.|[^"\\])*"', '""', allsrc)
code = re.sub(r"//[^\n]*", "", code)
std = set("assert assert_eq assert_ne panic format vec matches println Some None Ok Err Vec String Option Result Box self Self true false str f64 u32 u64 usize i32 i64 bool u8 char iter map filter collect expect unwrap len iter into from clone push contains get insert sort join starts_with any all new default is_empty first last min max abs sqrt powi to_string to_owned as_str as_ref cloned copied zip enumerate rev take skip chain flat_map filter_map find position windows chunks keys values entry or_default extend retain dedup sort_unstable sort_by partial_cmp total_cmp cmp unwrap_or unwrap_or_else map_or ok_or_else ok and_then is_some is_none is_finite round floor ceil exp ln log10 sin cos tan atan2 hypot to_ascii_lowercase trim split lines parse as_array as_object as_f64 as_u64 as_bool matches_str drop_in_place async_test test cfg allow".split())
cands = set(re.findall(r"(?<![\w.:])([a-z_][a-z0-9_]*)\s*\(", code)) | set(re.findall(r"(?<![\w.:])([A-Z][A-Za-z0-9]+)\b", code)) | set(re.findall(r"(?<![\w.:])([a-z_][a-z0-9_]*)::", code))
for n in sorted(cands - imported - defined - std):
    if n in declared: continue
    if n in {"crate","super","std","core","alloc","serde_json","semio_s_plugin_norm_compliance"} or n.startswith("semio_"): continue
    print("NOT-PUB?  ", n, "(declared in under-test closure non-pub)" if re.search(r"\b(?:fn|struct|enum|const|mod|type|trait|static)\s+" + n + r"\b", src) else "(not found in closure: via dependency glob or std)")
