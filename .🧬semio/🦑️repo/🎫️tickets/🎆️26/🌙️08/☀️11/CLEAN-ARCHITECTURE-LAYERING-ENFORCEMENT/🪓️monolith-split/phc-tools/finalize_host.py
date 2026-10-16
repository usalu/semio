#!/usr/bin/env python3
"""🧾️ Finalizes the plugin-host carve: manifests with comment carry, project files, shared registries, conflict-checked flush."""
import os, sys, json, re, collections, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, regs, regs_host, hcmod
from plan_host import CRATES, FACADE, P
from carve import REPO

ORIG_MANIFEST = plan_host.P('📦️packages/🦀️rust/Cargo.toml')
ROOT_CARGO = REPO + '/Cargo.toml'
OWNERS = REPO + '/📋️build-owners.json'
LAUNCH = REPO + '/.vscode/launch.json'
SEED = REPO + '/.vscode/🧩️launch.seed.jsonc'


def dep_comments(text):
    """{dependency key: preceding comment block} of the original manifest, for both dependency tables."""
    out = {}
    lines = text.split('\n')
    table = None
    pending = []
    for ln in lines:
        s = ln.strip()
        m = re.match(r'^\[(.+)\]$', s)
        if m:
            table = m.group(1)
            pending = []
            continue
        if s.startswith('#'):
            pending.append(ln)
            continue
        km = re.match(r'^([A-Za-z0-9_\-]+)\s*=', s)
        if km and table in ('dependencies', 'dev-dependencies'):
            if pending:
                out[(table, km.group(1))] = '\n'.join(pending)
            pending = []
        elif s == '':
            pending = []
    return out


def carry_comments(manifest, comments):
    out = []
    table = None
    for ln in manifest.split('\n'):
        m = re.match(r'^\[(.+)\]$', ln.strip())
        if m:
            table = m.group(1)
        km = re.match(r'^([A-Za-z0-9_\-]+)\s*=', ln)
        if km and table in ('dependencies', 'dev-dependencies') and (table, km.group(1)) in comments:
            out.append(comments[(table, km.group(1))])
        out.append(ln)
    return '\n'.join(out)


def crate_features(model):
    feats = {}
    tested = {}
    for ident in CRATES:
        feats[ident] = regs.used_features(model, ident)
        tested[ident] = regs.has_tests(model, ident)
    return feats, tested


def scan_files(paths, names, ws=None):
    """Identifiers of `names` that start a path in the given files (bin and integration-test targets of the facade package)."""
    from rsitems import lex
    found = set()
    for p in paths:
        toks = [hcmod.Tok(t) for t in lex(ws.read(p) if ws is not None else open(p, encoding='utf-8').read())]
        for i, t in enumerate(toks):
            if t.k == 'id' and t.x in names and i + 2 < len(toks) and toks[i + 1].x == ':' and toks[i + 2].x == ':' and not (i > 0 and toks[i - 1].x == ':'):
                found.add(t.x)
    return found


def facade_extras(table_names, ws=None):
    bin_f = [P('🧵️shard/👶️child/🦀️.rs')]
    test_f = [P('🏗️instance/🧪️tests/🦀️.rs')]
    orig = regs_host.orig_cargo(ORIG_MANIFEST)
    names = {k.replace('-', '_') for k in list(orig['dependencies']) + list(orig['dev-dependencies'])} | set(table_names)
    return scan_files(bin_f, names, ws), scan_files(test_f, names, ws)


def manifests(model, extra=None, ws=None):
    regs_host.orig_cargo(ORIG_MANIFEST)
    extra = dict(extra or {})
    bin_names, test_names = facade_extras(CRATES.keys(), ws)
    fx = dict(extra.get(FACADE, {}))
    fx['extra_normal'] = tuple(fx.get('extra_normal', ())) + tuple(sorted(bin_names))
    fx['extra_dev'] = tuple(fx.get('extra_dev', ())) + tuple(sorted(test_names))
    extra[FACADE] = fx
    comments = dep_comments(open(ORIG_MANIFEST, encoding='utf-8').read())
    table = {k: dict(name=v['name'], root=v['root'], sfx=v['sfx'], desc=v['desc']) for k, v in CRATES.items()}
    feats, tested = crate_features(model)
    out = {}
    for ident, info in table.items():
        ov = (extra or {}).get(ident, {})
        text, usage = regs_host.build_cargo(model, ident, info, table, feats, tested, extra_normal=ov.get('extra_normal', ()), extra_dev=ov.get('extra_dev', ()), forward=ov.get('forward'))
        out[ident] = (carry_comments(text, comments), usage)
    return out, feats, tested


def facade_manifest(new_text):
    """Keeps the facade's package identity, lib, bin and test target; takes the computed dependency tables."""
    orig = open(ORIG_MANIFEST, encoding='utf-8').read()
    head = orig[:orig.index('[dependencies]')]
    bin_part = orig[orig.index('[[test]]'):] if '[[test]]' in orig else ''
    deps = new_text[new_text.index('[features]') if '[features]' in new_text else new_text.index('[dependencies]'):]
    return head + deps.rstrip('\n') + '\n\n' + bin_part


def stage_files(ws, C, extra=None):
    """Writes manifests and project files into the in-memory workspace; returns the list of new crates (name, package dir, project id)."""
    mans, feats, tested = manifests(C.new, extra, ws)
    new_crates = []
    for ident, (text, usage) in mans.items():
        v = CRATES[ident]
        pkg = regs_host.pkg_dir_of(v['root'])
        if ident == FACADE:
            ws.write(pkg + '/Cargo.toml', facade_manifest(text))
            continue
        assert not ws.exists(pkg + '/Cargo.toml'), pkg
        ws.write(pkg + '/Cargo.toml', text)
        ptext, pid = regs_host.project_json(v['name'])
        ws.write(pkg + '/📋️project.json', ptext)
        new_crates.append((v['name'], pkg, pid))
    return new_crates, mans, feats, tested


def registries(new_crates):
    """Read-modify-write texts of the shared registries (root workspace members, owner rows, launch routes)."""
    rels = [os.path.relpath(pkg, REPO) for _, pkg, _ in new_crates]
    texts = {p: open(p, encoding='utf-8').read() for p in (ROOT_CARGO, OWNERS, LAUNCH, SEED)}
    ct = regs_host.insert_members(texts[ROOT_CARGO], rels)
    ot = regs_host.insert_owner_rows(texts[OWNERS], [(pid, os.path.relpath(pkg, REPO), os.path.relpath(pkg, REPO) + '/📋️project.json') for _, pkg, pid in new_crates])
    pairs = [(n, pid) for n, _, pid in new_crates]
    orders = regs_host.order_bands([texts[LAUNCH], texts[SEED]], len(pairs))
    lt = regs_host.insert_routes(texts[LAUNCH], pairs, orders)
    st = regs_host.insert_routes(texts[SEED], pairs, orders)
    return texts, {ROOT_CARGO: ct, OWNERS: ot, LAUNCH: lt, SEED: st}


def write_registries(texts, out):
    for p, t in out.items():
        if open(p, encoding='utf-8').read() != texts[p]:
            raise RuntimeError('concurrent edit detected on ' + p)
    for p, t in out.items():
        open(p, 'w', encoding='utf-8').write(t)
