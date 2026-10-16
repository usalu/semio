#!/usr/bin/env python3
"""📒️ Registration generator for new hub crates: Cargo.toml (computed dependencies), project.json, workspace member, workspace dependency, owner row and the six launch routes."""
import os, re, sys, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, cfgeval
from carve import REPO, inline_descendants

HUB = REPO + '/🌎️hub'
ORIG_CARGO = None


def load_orig_cargo(path=None):
    global ORIG_CARGO
    if ORIG_CARGO is None:
        p = path or '/tmp/hubw/semio-hub-cargo.json'
        if not os.path.exists(p):
            out = subprocess.run(['bun', '-e', 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))', HUB + '/📦️packages/🦀️rust/Cargo.toml'], capture_output=True, text=True, cwd=REPO).stdout
            open(p, 'w').write(out)
        ORIG_CARGO = json.load(open(p))
    return ORIG_CARGO


ORIG_PKG_DIR = HUB + '/📦️packages/🦀️rust'


def pkg_dir_of(root_file):
    return os.path.join(os.path.dirname(root_file), '📦️packages', '🦀️rust')


def mod_is_test_only(m):
    """True when a file module is only built under cfg(test)."""
    x = m
    while x is not None:
        for c in x.cfg:
            if not cfgeval.holds(c, test=False):
                return True
        x = x.parent
    return False


def item_ranges_test_only(fm):
    """Token ranges of top-level/inline items of this file module whose cfg is false in non-test builds."""
    out = []
    for m in [fm] + list(inline_descendants(fm)):
        for it in m.all_items:
            if any(not cfgeval.holds(c, test=False) for c in it.cfg):
                out.append(it.tok_range)
    return out


def scan_usage(model, ident, names):
    """Return (normal, test) sets of identifiers from `names` used by the crate (test is a superset of normal)."""
    normal, test = set(), set()
    root = model.roots[ident]
    for fm in hcmod.walk(root):
        if fm.inline or getattr(fm, 'missing', False):
            continue
        file_test_only = mod_is_test_only(fm)
        ranges = item_ranges_test_only(fm)
        toks = fm.toks
        n = len(toks)
        for i, t in enumerate(toks):
            hit = None
            if t.k == 'id' and t.x in names:
                nxt = toks[i + 1].x if i + 1 < n else ''
                nxt2 = toks[i + 2].x if i + 2 < n else ''
                prev = toks[i - 1].x if i > 0 else ''
                attr = prev == '[' and i > 1 and toks[i - 2].x == '#'
                if prev == '.' or (i > 0 and toks[i - 1].x == 'as'):
                    continue
                if attr:
                    hit = t.x
                elif nxt == ':' and nxt2 == ':':
                    if prev == ':' and i > 1 and toks[i - 2].x == ':':
                        if i > 2 and (toks[i - 3].k == 'id' or toks[i - 3].x in ('>',)):
                            continue
                    hit = t.x
            elif t.k == 'str' and t.x.startswith('"::'):
                m = re.match(r'"::([A-Za-z_][A-Za-z0-9_]*)', t.x)
                if m and m.group(1) in names:
                    hit = m.group(1)
            if hit is None:
                continue
            in_test = file_test_only or any(a <= i < b for a, b in ranges)
            if not in_test:
                normal.add(hit)
            test.add(hit)
    return normal, test


def has_tests(model, ident):
    root = model.roots[ident]
    for fm in hcmod.walk(root):
        if fm.inline or getattr(fm, 'missing', False):
            continue
        if mod_is_test_only(fm):
            return True
        for m in [fm] + list(inline_descendants(fm)):
            for it in m.all_items:
                if any(not cfgeval.holds(c, test=False) for c in it.cfg):
                    return True
    return False


def used_features(model, ident):
    feats = set()
    root = model.roots[ident]
    for fm in hcmod.walk(root):
        if fm.inline or getattr(fm, 'missing', False):
            continue
        for m in re.finditer(r'feature\s*=\s*"([A-Za-z0-9_-]+)"', fm.src):
            feats.add(m.group(1))
    return feats


def rel(from_dir, to_abs):
    return os.path.relpath(to_abs, from_dir)


def toml_inline(spec):
    if isinstance(spec, str):
        return json.dumps(spec, ensure_ascii=False)
    parts = []
    for k, v in spec.items():
        if isinstance(v, bool):
            parts.append(f'{k} = {"true" if v else "false"}')
        elif isinstance(v, list):
            parts.append(f'{k} = [{", ".join(json.dumps(x, ensure_ascii=False) for x in v)}]')
        else:
            parts.append(f'{k} = {json.dumps(v, ensure_ascii=False)}')
    return '{ ' + ', '.join(parts) + ' }'


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


def build_cargo(model, ident, info, crate_table, crate_features=None, tests_of=None, extra_normal=(), extra_dev=(), features_extra=None, optional_deps=None, normal_seam=False):
    """info: dict(name, root, description, id). crate_table: ident -> dict(name, root)."""
    orig = load_orig_cargo()
    pkg = info.get('pkg') or pkg_dir_of(info['root'])
    deps_orig = orig['dependencies']
    dev_orig = orig['dev-dependencies']
    key_by_ident = {}
    for k in list(deps_orig) + list(dev_orig):
        key_by_ident.setdefault(ident_of_key(k), k)
    names = set(key_by_ident) | set(crate_table)
    normal, test = scan_usage(model, ident, names)
    normal |= set(extra_normal)
    test |= set(extra_dev) | normal
    tested = tests_of.get(ident, False) if tests_of else has_tests(model, ident)
    hub_normal = sorted(i for i in normal if i in crate_table and i != ident)
    hub_dev = sorted(i for i in test if i in crate_table and i != ident and i not in normal)
    ext_normal = [i for i in normal if i not in crate_table]
    ext_dev = [i for i in test if i not in crate_table and i not in normal]
    optional_deps = optional_deps or {}
    desc = info.get('description', '').replace('"', "'")
    sid = info.get('id') or info['name'].replace('semio-', '', 1)
    feats = crate_features[ident] if crate_features else used_features(model, ident)
    own = sorted(feats - {'sqlite', 'postgres', 'neo4j'})
    rel_ws = rel(pkg, HUB)
    out = []
    out.append('[package]')
    out.append(f'workspace = "{rel_ws}"')
    out.append(f'name = "{info["name"]}"')
    out.append('version.workspace = true')
    out.append('edition.workspace = true')
    out.append('rust-version.workspace = true')
    out.append(f'description = "{desc}"')
    out.append('')
    out.append('[package.metadata.semio]')
    out.append('role = "hub"')
    out.append(f'id = "{sid}"')
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
    if info.get('lib_name'):
        out.append(f'name = "{info["lib_name"]}"')
    featmap = {}
    for f in own:
        fw = []
        for d in hub_normal + hub_dev:
            if crate_features and f in crate_features.get(d, ()):
                fw.append(f'{crate_table[d]["name"]}/{f}')
        featmap[f] = fw
    for f, v in (features_extra or {}).items():
        featmap[f] = list(featmap.get(f, [])) + list(v)
    for f, v in optional_deps.items():
        pass
    if featmap:
        out.append('')
        out.append('[features]')
        for f, v in featmap.items():
            out.append(f'{f} = [{", ".join(json.dumps(x, ensure_ascii=False) for x in v)}]')
    out.append('')
    out.append('[dependencies]')
    for i in hub_normal:
        tfn = [f for f in ('testkit', 'integration-fixtures') if normal_seam and crate_features and f in crate_features.get(i, ())]
        out.append(f'{crate_table[i]["name"]} = {toml_inline({"workspace": True, "features": tfn}) if tfn else "{ workspace = true }"}')
    for k in deps_orig:
        i = ident_of_key(k)
        if i in ext_normal and i not in optional_deps:
            out.append(f'{k} = {toml_inline(convert_spec(deps_orig[k], pkg))}')
    for i in ext_normal:
        k = key_by_ident[i]
        if k not in deps_orig and i not in optional_deps:
            out.append(f'{k} = {toml_inline(convert_spec(dev_orig[k], pkg))}')
    for i, spec in optional_deps.items():
        k = key_by_ident.get(i, i)
        base = convert_spec(dev_orig.get(k, deps_orig.get(k)), pkg)
        base = dict(base)
        base['optional'] = True
        for kk, vv in spec.items():
            base[kk] = vv
        out.append(f'{k} = {toml_inline(base)}')
    out.append('')
    out.append('[dev-dependencies]')
    seamy = ('testkit', 'integration-fixtures')
    for i in hub_dev + (hub_normal if tested else []):
        tf = [f for f in seamy if crate_features and f in crate_features.get(i, ())]
        if i in hub_dev:
            spec = {'workspace': True}
            if tf:
                spec['features'] = tf
            out.append(f'{crate_table[i]["name"]} = {toml_inline(spec)}')
        elif tf:
            out.append(f'{crate_table[i]["name"]} = {toml_inline({"workspace": True, "features": tf})}')
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
    return '\n'.join(out), dict(hub_normal=hub_normal, hub_dev=hub_dev, ext_normal=sorted(ext_normal), ext_dev=sorted(ext_dev), features=featmap)


def project_json(name):
    pid = '@semio-tech/' + name.replace('semio-', '', 1) + '-rs'
    return json.dumps({"name": pid, "tags": ["scope:hub", "language:rust"]}, indent=2, ensure_ascii=False) + '\n', pid


# ------------------------------------------------------------------ shared registries (applied at flush time, read-modify-write)
def insert_hub_members(text, rels):
    """rels: hub-relative package dirs; insert into `members = [ ... ]` keeping byte order."""
    m = re.search(r'members = \[\n(.*?)\n\]', text, re.S)
    lines = m.group(1).split('\n')
    existing = [l.strip().strip(',').strip('"') for l in lines]
    allm = sorted(set(existing) | set(rels), key=lambda s: s.encode('utf-8'))
    body = '\n'.join(f'    "{x}",' for x in allm)
    return text[:m.start(1)] + body + text[m.end(1):]


def insert_workspace_deps(text, entries):
    """entries: list of (name, hub-relative pkg dir) -> `name = { path = "../🌎️hub/<rel>" }` after the last semio-hub-* workspace dependency."""
    out = text
    for name, rel_dir in entries:
        line = f'{name} = {{ path = "../🌎️hub/{rel_dir}" }}\n'
        if f'\n{name} = ' in out:
            continue
        anchor = out.rindex('\nsemio-hub-inference-binding = ')
        end = out.index('\n', anchor + 1) + 1
        out = out[:end] + line + out[end:]
    return out


def insert_owner_rows(text, rows):
    """rows: list of (name, root_rel, manifest_rel). Inserts after the nearest preceding `@semio-tech/hub-*` row in name order."""
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
        # find first hub owner row with greater name
        pos = None
        for m in re.finditer(r'    \{\n      "name": "(@semio-tech/hub-[^"]+)"', text):
            if m.group(1) > name:
                pos = m.start()
                break
        if pos is None:
            ms = list(re.finditer(r'    \{\n      "name": "(@semio-tech/hub-[^"]+)"', text))
            end = text.index('    },\n', ms[-1].start()) + len('    },\n')
            pos = end
        text = text[:pos] + block + text[pos:]
    return text


def fresh_band(texts, prefix, count, step=1e-5):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    for k in range(0, 1000):
        base = f'{prefix}{k}'
        if not any(u.startswith(base) for u in used):
            return base
    raise RuntimeError('no free order band')


ROUTE_ANCHOR = '"name": "🦀️ semio-hub-inference-binding 🧪️test 🚀️release"'


SEED_ENV = '\n      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'


def route_blocks(crates, orders, seed=False):
    """crates: list of (crate name, project id). orders: dict kind -> list of order strings (len = crates*2)."""
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
      }}{"," + SEED_ENV if seed else ""}
    }},
''')
    return ''.join(out)


def insert_routes(text, crates, orders, seed=False):
    crates = [(c, p) for c, p in crates if f'"name": "🦀️ {c} 🏗️build 🛠️prototype"' not in text]
    if not crates:
        return text
    i = text.index(ROUTE_ANCHOR)
    end = text.index('\n    },\n', i) + len('\n    },\n')
    return text[:end] + route_blocks(crates, orders, seed) + text[end:]


def order_bands(texts, count):
    """Free `order` values for `count` crates: {kind: [2*count strings]} in the existing build/check/test bands."""
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    out = {}
    for kind, prefix in (('build', '900.9'), ('check', '922.9'), ('test', '903.9')):
        vals, n = [], 1
        while len(vals) < 2 * count:
            v = f'{prefix}{n:04d}'
            n += 1
            if v not in used:
                vals.append(v)
        out[kind] = vals
    return out
