"""Cargo manifests, project.json files and registrations for the world crates."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wgen import CRATES, crate_ident, crate_pkg, TITLES
from oinf_rs import lex

SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
MAN = json.load(open(SCR + '/infinite-manifest.json', encoding='utf-8'))
REL_WORLD = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"

def used_first_segments(text):
    toks = [t for t in lex(text) if t.k != 'comment']
    out = set()
    for i in range(len(toks) - 2):
        if toks[i].k == 'id' and toks[i+1].s == ':' and toks[i+2].s == ':' and not (i > 0 and toks[i-1].s == ':'):
            out.add(toks[i].s)
    for i, t in enumerate(toks):
        if t.s == 'use' and t.k == 'id' and i + 1 < len(toks) and toks[i+1].k == 'id': out.add(toks[i+1].s)
    return out

def dep_value(v, extra_up='../../'):
    if isinstance(v, str): return f'"{v}"'
    parts = []
    for k in ('workspace', 'path', 'package', 'version', 'default-features', 'features', 'optional'):
        if k not in v: continue
        x = v[k]
        if k == 'path': x = extra_up + x
        if isinstance(x, bool): parts.append(f"{k} = {'true' if x else 'false'}")
        elif isinstance(x, list): parts.append(f"{k} = [" + ', '.join(f'"{y}"' for y in x) + "]")
        else: parts.append(f'{k} = "{x}"')
    return '{ ' + ', '.join(parts) + ' }'

def key_ident(k): return k.replace('-', '_')

def build_manifest(c, ci, libtext, testtexts, siblings_used):
    """c: crate; returns Cargo.toml text."""
    idents_lib = used_first_segments(libtext)
    idents_test = set()
    for t in testtexts: idents_test |= used_first_segments(t)
    lines = ["[package]", 'workspace = "../../../../../../../../.."', f'name = "{crate_pkg(c.id)}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true',
             f'description = "{TITLES[c.id][1].rstrip(".")}"', "",
             "[package.metadata.semio]", 'role = "product"', f'id = "os-infinite-world-{c.id}"', "",
             "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
             "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', ""]
    # features
    lower = [crate_pkg(CRATES[k][0]) for k in sorted(siblings_used)]
    feats = ["testkit = [" + ', '.join(f'"{p}/testkit"' for p in lower) + "]"]
    lines += ["[features]"] + feats + [""]
    deps = []; devdeps = []; tdeps = collections.defaultdict(list)
    for k, v in MAN['dependencies'].items():
        if key_ident(k) in idents_lib: deps.append(f"{k} = {dep_value(v)}")
    for k in sorted(siblings_used):
        cid = CRATES[k][0]
        deps.append(f'{crate_pkg(cid)} = {{ path = "../../../{CRATES[k][2]}/📦️packages/🦀️rust" }}')
    for tgt, tv in MAN.get('target', {}).items():
        for k, v in tv.get('dependencies', {}).items():
            if key_ident(k) in idents_lib: tdeps[tgt].append(f"{k} = {dep_value(v)}")
    for k, v in MAN['dev-dependencies'].items():
        if key_ident(k) in idents_test | idents_lib:
            devdeps.append(f"{k} = {dep_value(v)}")
    # test-only normal deps that are not dev in the facade but needed
    for k, v in MAN['dependencies'].items():
        if key_ident(k) in idents_test and key_ident(k) not in idents_lib and f"{k} = " not in '\n'.join(devdeps):
            devdeps.append(f"{k} = {dep_value(v)}")
    for tgt, tv in MAN.get('target', {}).items():
        for k, v in tv.get('dependencies', {}).items():
            if key_ident(k) in idents_test and key_ident(k) not in idents_lib:
                tdeps[tgt].append(f"{k} = {dep_value(v)}")
    lines += ["[dependencies]"] + deps + [""]
    for tgt, ls in tdeps.items():
        lines += [f"[target.'{tgt}'.dependencies]"] + ls + [""]
    selfdev = f'{crate_pkg(c.id)} = {{ path = ".", features = ["testkit"] }}'
    lines += ["[dev-dependencies]", selfdev] + devdeps + [""]
    for name, path in testtargets_for(c):
        lines += ["[[test]]", f'name = "{name}"', f'path = "{path}"', ""]
    return '\n'.join(lines)

_TT = {}
def testtargets_for(c): return _TT.get(c.id, [])
def set_test_targets(mapping): _TT.update(mapping)
