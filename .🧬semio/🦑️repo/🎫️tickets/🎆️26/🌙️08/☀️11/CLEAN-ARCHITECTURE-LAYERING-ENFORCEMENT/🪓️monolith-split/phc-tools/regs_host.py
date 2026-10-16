#!/usr/bin/env python3
"""📒️ Registration generator for the plugin-host crates: Cargo.toml (computed dependencies), project.json, workspace member, owner row and the six launch routes."""
import os, re, sys, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, cfgeval, regs
from carve import REPO, inline_descendants

ORIG_PKG_DIR = REPO + '/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust'
ORIG = None
TOML_TOOL = 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))'


def orig_cargo(text_path=None):
    global ORIG
    if ORIG is None:
        out = subprocess.run(['bun', '-e', TOML_TOOL, text_path or ORIG_PKG_DIR + '/Cargo.toml'], capture_output=True, text=True, cwd=REPO).stdout
        ORIG = json.loads(out)
    return ORIG


def pkg_dir_of(root_file):
    d = os.path.dirname(root_file)
    if root_file.endswith('/📦️packages/🦀️rust/🦀️.rs'):
        return d
    return os.path.join(d, '📦️packages', '🦀️rust')


def rel(from_dir, to_abs):
    return os.path.relpath(to_abs, from_dir)


def convert_spec(spec, pkg_dir):
    if isinstance(spec, str):
        return spec
    spec = dict(spec)
    if 'path' in spec:
        absp = os.path.normpath(os.path.join(ORIG_PKG_DIR, spec['path']))
        spec['path'] = rel(pkg_dir, absp)
    return spec


def ident_of_key(k):
    return k.replace('-', '_')


def toml_inline(spec):
    return regs.toml_inline(spec)


def build_cargo(model, ident, info, table, features_of, tested_of, extra_normal=(), extra_dev=(), forward=None):
    """table: ident -> dict(name, root, sfx); features_of: ident -> set of features the crate defines; tested_of: ident -> bool."""
    orig = orig_cargo()
    pkg = pkg_dir_of(info['root'])
    deps_orig = orig['dependencies']
    dev_orig = orig['dev-dependencies']
    key_by_ident = {}
    for k in list(deps_orig) + list(dev_orig):
        key_by_ident.setdefault(ident_of_key(k), k)
    names = set(key_by_ident) | set(table)
    normal, test = regs.scan_usage(model, ident, names)
    normal |= set(extra_normal)
    test |= set(extra_dev) | normal
    tested = tested_of.get(ident, False)
    fam_normal = sorted(i for i in normal if i in table and i != ident)
    fam_dev = sorted(i for i in test if i in table and i != ident and i not in normal)
    ext_normal = [i for i in normal if i not in table]
    ext_dev = [i for i in test if i not in table and i not in normal]
    sid = info['sfx']
    pid = 'os-plugin-host' + ('-' + sid if sid else '')
    desc = info['desc'].replace('"', "'")
    own = sorted(features_of[ident])
    out = []
    out.append('[package]')
    out.append(f'workspace = "{rel(pkg, REPO)}"')
    out.append(f'name = "{info["name"]}"')
    out.append('version = "0.1.0"')
    out.append('edition = "2021"')
    out.append('rust-version.workspace = true')
    out.append(f'description = "{desc}"')
    out.append('')
    out.append('[package.metadata.semio]')
    out.append('role = "product"')
    out.append(f'id = "{pid}"')
    out.append('')
    out.append('[package.metadata.semio.rust-source-boundary]')
    out.append('maxSourceBytes = 262144')
    out.append('maxSourceFiles = 32')
    out.append('maxActiveLines = 4000')
    out.append('')
    out.append('[lints]')
    out.append('workspace = true')
    out.append('')
    out.append('[lib]')
    out.append(f'path = "{rel(pkg, info["root"])}"')
    out.append('doctest = false')
    featmap = {}
    for f in own:
        fw = []
        for d in fam_normal:
            if f in features_of.get(d, ()):
                fw.append(f'{table[d]["name"]}/{f}')
        featmap[f] = fw
    for f, v in (forward or {}).items():
        featmap[f] = list(featmap.get(f, [])) + list(v)
    if featmap:
        out.append('')
        out.append('[features]')
        for f, v in featmap.items():
            out.append(f'{f} = [{", ".join(json.dumps(x, ensure_ascii=False) for x in v)}]')
    out.append('')
    out.append('[dependencies]')
    for k in deps_orig:
        i = ident_of_key(k)
        if i in ext_normal:
            out.append(f'{k} = {toml_inline(convert_spec(deps_orig[k], pkg))}')
    for i in ext_normal:
        k = key_by_ident[i]
        if k not in deps_orig:
            out.append(f'{k} = {toml_inline(convert_spec(dev_orig[k], pkg))}')
    for i in fam_normal:
        out.append(f'{table[i]["name"]} = {{ path = "{rel(pkg, pkg_dir_of(table[i]["root"]))}" }}')
    out.append('')
    out.append('[dev-dependencies]')
    for i in fam_dev + (fam_normal if tested else []):
        tf = ['testkit'] if 'testkit' in features_of.get(i, ()) else []
        if i in fam_dev:
            spec = f'{{ path = "{rel(pkg, pkg_dir_of(table[i]["root"]))}"' + (f', features = {json.dumps(tf)}' if tf else '') + ' }'
            out.append(f'{table[i]["name"]} = {spec}')
        elif tf:
            out.append(f'{table[i]["name"]} = {{ path = "{rel(pkg, pkg_dir_of(table[i]["root"]))}", features = {json.dumps(tf)} }}')
    for k in dev_orig:
        i = ident_of_key(k)
        if i in ext_dev:
            out.append(f'{k} = {toml_inline(convert_spec(dev_orig[k], pkg))}')
        elif tested and i in ext_normal and i in test and k in deps_orig and dev_orig[k] != deps_orig[k]:
            out.append(f'{k} = {toml_inline(convert_spec(dev_orig[k], pkg))}')
    for i in ext_dev:
        k = key_by_ident[i]
        if k not in dev_orig:
            out.append(f'{k} = {toml_inline(convert_spec(deps_orig[k], pkg))}')
    out.append('')
    return '\n'.join(out), dict(fam_normal=fam_normal, fam_dev=fam_dev, ext_normal=sorted(ext_normal), ext_dev=sorted(ext_dev), features=featmap)


def project_json(name):
    pid = '@semio-tech/' + name.replace('semio-', '', 1) + '-rs'
    return json.dumps({"name": pid, "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + '\n', pid


# ------------------------------------------------------------------ shared registries (read-modify-write at flush time)
def insert_members(text, rels):
    """Inserts each member before the first existing member that is byte-greater, leaving the order of all other lines alone."""
    m = re.search(r'members = \[\n(.*?)\n\]', text, re.S)
    lines = m.group(1).split('\n')
    assert all(l.strip().startswith('"') for l in lines), [l for l in lines if not l.strip().startswith('"')][:3]
    existing = [l.strip().strip(',').strip('"') for l in lines]
    for r in sorted(set(rels) - set(existing), key=lambda x: x.encode('utf-8')):
        pos = next((i for i, e in enumerate(existing) if e.encode('utf-8') > r.encode('utf-8')), len(existing))
        existing.insert(pos, r)
        lines.insert(pos, f'    "{r}",')
    return text[:m.start(1)] + '\n'.join(lines) + text[m.end(1):]


def insert_owner_rows(text, rows):
    """rows: (name, root_rel, manifest_rel); the owners array is sorted by byte order of the name."""
    for name, root_rel, manifest_rel in rows:
        if f'"name": "{name}"' in text:
            continue
        block = f'''    {{
      "name": "{name}",
      "root": "{root_rel}",
      "manifest": "{manifest_rel}",
      "packages": []
    }},
'''
        pos = None
        for m in re.finditer(r'    \{\n      "name": "([^"]+)"', text):
            if m.group(1).encode('utf-8') > name.encode('utf-8'):
                pos = m.start()
                break
        assert pos is not None, name
        text = text[:pos] + block + text[pos:]
    return text


SEED_ENV = '\n      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'
ROUTE_ANCHOR = '"name": "🦀️ semio-framework-os-mcp-http-transport 🧪️test 🚀️release"'


def route_blocks(crates, orders):
    out = []
    for idx, (cname, pid) in enumerate(crates):
        for (target, label, group, key) in (('build', '🏗️build', '4_build', 'build'), ('check', '🔎️check', '4_gate', 'check'), ('test', '🧪️test', '4_test', 'test')):
            for ci, (conf, clabel) in enumerate((('prototype', '🛠️prototype'), ('release', '🚀️release'))):
                order = orders[key][idx * 2 + ci]
                out.append(f'''    {{
      "name": "🦀️ {cname} {label} {clabel}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {pid}:{target} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }}{"," + SEED_ENV}
    }},
''')
    return ''.join(out)


def insert_routes(text, crates, orders, anchor=ROUTE_ANCHOR):
    crates = [(c, p) for c, p in crates if f'"name": "🦀️ {c} 🏗️build 🛠️prototype"' not in text]
    if not crates:
        return text
    i = text.index(anchor)
    end = text.index('\n    },\n', i) + len('\n    },\n')
    return text[:end] + route_blocks(crates, orders) + text[end:]


def order_bands(texts, count):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    out = {}
    for kind, prefix in (('build', '900.33'), ('check', '922.33'), ('test', '903.33')):
        vals, n = [], 1
        while len(vals) < 2 * count:
            v = f'{prefix}{n:03d}'
            n += 1
            if v not in used:
                vals.append(v)
        out[kind] = vals
    return out
