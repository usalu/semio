"""🔍 Definition scan: members (methods, fields, associated fns) that are not `pub` in a crate and whose name is used as `.name` / `Type::name` by a crate that depends on it."""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import spec
import union

D = sys.argv[1]
roots = {key: os.path.join(D, *spec.CRATES[key][0], n.RS) for key in spec.CRATES}
roots["facade"] = os.path.join(D, n.PKG, n.RUST, n.RS)
files = {key: union.owned(root) for key, root in roots.items()}
deps = {key: set(spec.DEPS[key][0]) | {d.replace("+testkit", "") for d in spec.DEPS[key][1]} for key in spec.CRATES}
deps["facade"] = set(spec.FACADE_DEPS)

def text_of(key):
    return "\n".join(open(f, encoding="utf-8").read() for f in files[key])

private_members = {}
pub_members = {}
for key in files:
    txt = text_of(key)
    priv, pub = set(), set()
    for m in re.finditer(r"^( +)(pub(?:\([^)]*\))?\s+)?(?:async\s+|const\s+|unsafe\s+)*fn (\w+)", txt, re.M):
        (pub if m.group(2) and m.group(2).strip() == "pub" else priv).add(m.group(3))
    for m in re.finditer(r"^( +)(pub(?:\([^)]*\))?\s+)?(\w+): [^=;{]+,?$", txt, re.M):
        if m.group(3) in ("let", "return", "use", "mod", "fn", "type", "const", "static"):
            continue
        (pub if m.group(2) and m.group(2).strip() == "pub" else priv).add(m.group(3))
    private_members[key], pub_members[key] = priv, pub

used = {key: set(re.findall(r"(?<![\w])\.(\w+)", text_of(key))) | set(re.findall(r"\b[A-Z]\w+::(\w+)", text_of(key))) for key in files}
std_noise = set("new default clone len is_empty get push pop iter map unwrap expect as_str to_string into from insert remove contains extend sort join split trim parse next take drain clear first last min max filter find any all collect take_while flat_map filter_map and_then or_else ok err send recv lock read write flush close fmt eq ne cmp partial_cmp hash".split())
for key, ds in deps.items():
    for dep in sorted(ds):
        if dep not in private_members:
            continue
        shared = (used[key] & private_members[dep]) - pub_members[dep] - std_noise
        if shared:
            print(f"{key} -> {dep}: {sorted(shared)}")
