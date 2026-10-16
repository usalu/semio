"""Cargo manifests, project.json files and registrations for the board crates."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bcrates import *
from oinf_rs import lex
import wman

SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
MAN = json.load(open(SC + '/infinite-manifest-board.json', encoding='utf-8'))
REL_INF = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
R = "/Users/ueli/Documents/semio"
used_first_segments = wman.used_first_segments
key_ident = wman.key_ident

def depth_below_inf(folder): return len(folder.split('/')) + 2   # folder levels + 📦️packages + 🦀️rust

def dep_value(v, up):
    if isinstance(v, str): return f'"{v}"'
    parts = []
    for k in ('workspace', 'path', 'package', 'version', 'default-features', 'features', 'optional'):
        if k not in v: continue
        x = v[k]
        if k == 'path': x = up + x
        if isinstance(x, bool): parts.append(f"{k} = {'true' if x else 'false'}")
        elif isinstance(x, list): parts.append(f"{k} = [" + ', '.join(f'"{y}"' for y in x) + "]")
        else: parts.append(f'{k} = "{x}"')
    return '{ ' + ', '.join(parts) + ' }'

def pkg_dir(folder): return f"{folder}/📦️packages/🦀️rust"

def build_manifest(c, libtext, testtexts, siblings, targets):
    d = depth_below_inf(c.folder)
    up = '../' * (d - 2)
    ws = '/'.join(['..'] * (5 + d))
    idl = used_first_segments(libtext)
    idt = set()
    for t in testtexts: idt |= used_first_segments(t)
    lines = ["[package]", f'workspace = "{ws}"', f'name = "{crate_pkg(c.id)}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true',
             f'description = "{TITLES[c.id][1].rstrip(".")}"', "",
             "[package.metadata.semio]", 'role = "product"', f'id = "os-infinite-board-{c.id}"', "",
             "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
             "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', ""]
    lower = [crate_pkg(CRATES[k][0]) for k in sorted(siblings)]
    lines += ["[features]", "testkit = [" + ', '.join(f'"{p}/testkit"' for p in lower) + "]", ""]
    deps = []; dev = []; tdeps = collections.defaultdict(list)
    for k, v in MAN['dependencies'].items():
        if k.startswith('semio-framework-os-infinite-world-'): continue
        if key_ident(k) in idl: deps.append(f"{k} = {dep_value(v, up)}")
    for k in sorted(siblings):
        sib = CRATES[k]
        rel = os.path.relpath(f"{REL_INF}/{sib[2]}/📦️packages/🦀️rust", f"{REL_INF}/{pkg_dir(c.folder)}")
        deps.append(f'{crate_pkg(sib[0])} = {{ path = "{rel}" }}')
    for tgt, tv in MAN.get('target', {}).items():
        for k, v in tv.get('dependencies', {}).items():
            if k.startswith('semio-framework-os-infinite-world-'): continue
            if key_ident(k) in idl: tdeps[tgt].append(f"{k} = {dep_value(v, up)}")
    for k, v in MAN['dev-dependencies'].items():
        if key_ident(k) in idt | idl: dev.append(f"{k} = {dep_value(v, up)}")
    for k, v in MAN['dependencies'].items():
        if k.startswith('semio-framework-os-infinite-world-'): continue
        if key_ident(k) in idt and key_ident(k) not in idl and not any(l.startswith(k + ' = ') for l in dev): dev.append(f"{k} = {dep_value(v, up)}")
    for tgt, tv in MAN.get('target', {}).items():
        for k, v in tv.get('dependencies', {}).items():
            if k.startswith('semio-framework-os-infinite-world-'): continue
            if key_ident(k) in idt and key_ident(k) not in idl: tdeps[tgt].append(f"{k} = {dep_value(v, up)}")
    lines += ["[dependencies]"] + deps + [""]
    for tgt, ls in tdeps.items(): lines += [f"[target.'{tgt}'.dependencies]"] + ls + [""]
    lines += ["[dev-dependencies]", f'{crate_pkg(c.id)} = {{ path = ".", features = ["testkit"] }}'] + dev + [""]
    for name, path in targets: lines += ["[[test]]", f'name = "{name}"', f'path = "{path}"', ""]
    return '\n'.join(lines)

def project_name(cid): return f"@semio-tech/os-infinite-board-{cid}-rs"

def project_json(cid):
    return json.dumps({"name": project_name(cid), "tags": ["language:rust", "role:product", "family:infinite"]}, indent=2, ensure_ascii=False) + "\n"

def slug(s):
    s = re.sub(r'[^\x00-\x7f]', '', s).strip('-')
    return re.sub(r'[^a-z0-9]+', '_', s.lower()).strip('_')

def write_manifests(crates, outroot, siblings, libtexts, testtexts, targets):
    for c in crates:
        pkg = f"{outroot}/{pkg_dir(c.folder)}"
        os.makedirs(pkg, exist_ok=True)
        open(pkg + '/Cargo.toml', 'w', encoding='utf-8').write(build_manifest(c, libtexts[c.id], testtexts[c.id], siblings[c.idx], targets[c.id]))
        open(pkg + '/📋️project.json', 'w', encoding='utf-8').write(project_json(c.id))
