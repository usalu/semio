#!/usr/bin/env python3
"""🧾️ Finalizes the whole hub carve (lib, bin, bin tests, space): manifests, project files, shared registries, one conflict-checked flush and the post-flush checks."""
import os, sys, json, re, collections, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_lib, plan_space, plan_bintests, regs, finalize_lib, fmtcheck, hcmod, ws as wsm
from carve import REPO, Model

HUB = REPO + '/🌎️hub'
HUB_PKG = HUB + '/📦️packages/🦀️rust'
SPACE_PKG = HUB + '/🧩️compositions/🪐️space/📦️packages/🦀️rust'
NORMAL_SEAM = {'semio_hub_route_testkit'}
TOML_TOOL = 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))'

DESC_SPACE = {
    'semio_hub_space_foundation': 'Studio foundation: space configuration, presence, terminology and shared identifiers',
    'semio_hub_space_operations': 'Studio operations: headless compute over the kernel workflow snapshot',
    'semio_hub_space_commands_authoring': 'Studio authoring commands: node, instance, parameter and selection edits',
    'semio_hub_space_commands_media': 'Studio media and session commands: media graph edits, import and export, engagement and panel state',
    'semio_hub_space_app': 'Studio app: artifact app, manifest and dispatch, main window modes and panels',
    'semio_hub_space_laws': 'Studio laws: the per-command laws over the real studio app',
    'semio_hub_space': 'S Studio plugin: OS host shell bundling the Home launcher and Studio apps',
}


def load_cargo(path):
    out = subprocess.run(['bun', '-e', TOML_TOOL, path], capture_output=True, text=True, cwd=REPO).stdout
    return json.loads(out)


def crate_table(crates):
    return {k: dict(name=v['name'], root=v['root']) for k, v in crates.items()}


def manifests(model, crates, orig_path, orig_pkg, desc, overrides=None):
    regs.ORIG_CARGO = load_cargo(orig_path)
    regs.ORIG_PKG_DIR = orig_pkg
    table = crate_table(crates)
    feats = {k: regs.used_features(model, k) for k in table}
    tests = {k: regs.has_tests(model, k) for k in table}
    out = {}
    for ident, info in table.items():
        ov = (overrides or {}).get(ident, {})
        info = dict(info, description=ov.get('description', desc.get(ident, ident)))
        if 'pkg' in ov:
            info['pkg'] = ov['pkg']
        text, usage = regs.build_cargo(model, ident, info, table, feats, tests, extra_normal=ov.get('extra_normal', ()), extra_dev=ov.get('extra_dev', ()), features_extra=ov.get('features_extra', finalize_lib.FEATURE_EXTRA.get(ident)), optional_deps=ov.get('optional', finalize_lib.OPTIONAL.get(ident)), normal_seam=ident in NORMAL_SEAM)
        out[ident] = (text, usage, info)
    return out


def keep_package_identity(text):
    """The two surviving original packages keep the metadata they always had: no `id`, no explicit source-boundary table."""
    text = re.sub(r'\nid = "[^"]*"\n', '\n', text, count=1)
    return re.sub(r'\[package\.metadata\.semio\.rust-source-boundary\]\n(?:[a-zA-Z]+ = [0-9]+\n)+\n', '', text, count=1)


def os_hub_manifest(text):
    """The `semio-hub` package becomes bin-only: the `[lib]` section turns into the `os-hub` `[[bin]]` and the client-contract integration test keeps its target."""
    text = keep_package_identity(text)
    text = re.sub(r'\[lib\]\npath = "[^"]*"\n(name = "[^"]*"\n)?', '[[bin]]\nname = "os-hub"\npath = "../../🏗️bootstrap/🦀️.rs"\n', text, count=1)
    text = text.replace('[package.metadata.semio]\nrole = "hub"', '[package.metadata.semio]\nchannel-version-consumers = "🔖️channel-version/📇️consumers.json"\nrole = "hub"', 1)
    return text.rstrip('\n') + '\n\n[[test]]\nname = "inference_client_contract"\npath = "../../💡️inference/🧪️tests/🔬️client-contract/🦀️.rs"\n'


HUB_OVERRIDES = {
    'os_hub': dict(
        pkg=HUB_PKG,
        description='OS hub — the multi-tenant document/directory server: an axum WS shell over db::Database (documents/blobs) and HubDirectory (identity/tenancy), sqlite/postgres/neo4j backends selectable by feature',
        extra_dev=('semio_framework_os_mcp', 'semio_s_artifact_gis_gismap', 'semio_s_artifact_gis_gismap_mcp', 'semio_framework_async', 'directory', 'semio_framework_schema'),
    ),
}


def project_files(crates, skip=()):
    out = {}
    for ident, v in crates.items():
        if ident in skip:
            continue
        pkg = regs.pkg_dir_of(v['root'])
        text, pid = regs.project_json(v['name'])
        out[ident] = (pkg, text, pid)
    return out


SPACE_SERDE_NOTE = """# 🌱️ Test-only: every production/`#[cfg(test)]`-adjacent call site now runs on `ToValue`/`FromValue`
# (ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`). What remains is the
# `🧪️tests/<case>/🦀️.rs` per-mutation fixture-comparison suite, which deliberately keeps
# `serde_json` as the differential third-party oracle its own assertions are named after (CLAUDE.md:
# "You SHOULD use existing libraries as possible to test our implementation").
"""


def with_serde_note(text):
    head, sep, dev = text.partition('[dev-dependencies]\n')
    if not sep:
        return text
    lines = dev.split('\n')
    for i, l in enumerate(lines):
        if l.startswith('serde = ') or l.startswith('serde_json = '):
            lines.insert(i, SPACE_SERDE_NOTE.rstrip('\n'))
            break
    return head + sep + '\n'.join(lines)


def space_crates():
    return collections.OrderedDict((k, dict(root=v, name=k.replace('_', '-'))) for k, v in plan_space.ROOTS.items())


def stage(apply_registries=False):
    """Builds every file change in memory: returns (ws, sws, new_crates) where new_crates lists (name, pkg dir, project id) of every crate that gets registered."""
    ws, C, roots, base_ws = plan_lib.prepare()
    sws, SC, sroots, sbase = plan_space.prepare()
    new_crates = []
    mans = manifests(C.new, plan_lib.CRATES, HUB_PKG + '/Cargo.toml', HUB_PKG, finalize_lib.DESC, HUB_OVERRIDES)
    for ident, (text, usage, info) in mans.items():
        v = plan_lib.CRATES[ident]
        if ident == 'os_hub':
            ws.write(HUB_PKG + '/Cargo.toml', os_hub_manifest(text))
            continue
        pkg = regs.pkg_dir_of(v['root'])
        assert not ws.exists(pkg + '/Cargo.toml'), pkg
        ws.write(pkg + '/Cargo.toml', text)
        ptext, pid = regs.project_json(v['name'])
        ws.write(pkg + '/📋️project.json', ptext)
        new_crates.append((v['name'], pkg, pid))
    ws.delete(HUB_PKG + '/🦀️.rs')
    scr = space_crates()
    smans = manifests(SC.new, scr, SPACE_PKG + '/Cargo.toml', SPACE_PKG, DESC_SPACE, {'semio_hub_space': dict(pkg=SPACE_PKG, description=DESC_SPACE['semio_hub_space'])})
    for ident, (text, usage, info) in smans.items():
        v = scr[ident]
        if ident == 'semio_hub_space':
            sws.write(SPACE_PKG + '/Cargo.toml', with_serde_note(keep_package_identity(text)))
            continue
        pkg = regs.pkg_dir_of(v['root'])
        assert not sws.exists(pkg + '/Cargo.toml'), pkg
        sws.write(pkg + '/Cargo.toml', with_serde_note(text))
        ptext, pid = regs.project_json(v['name'])
        sws.write(pkg + '/📋️project.json', ptext)
        new_crates.append((v['name'], pkg, pid))
    return ws, sws, new_crates, mans, smans


def registries(new_crates, dry=True):
    """Read-modify-write of the shared registries (hub members and workspace dependencies, owner rows, launch routes)."""
    rels = [os.path.relpath(pkg, HUB) for _, pkg, _ in new_crates]
    cargo = HUB + '/Cargo.toml'
    owners = REPO + '/📋️build-owners.json'
    launch = REPO + '/.vscode/launch.json'
    seed = REPO + '/.vscode/🧩️launch.seed.jsonc'
    texts = {p: open(p).read() for p in (cargo, owners, launch, seed)}
    ct = regs.insert_hub_members(texts[cargo], rels)
    ct = regs.insert_workspace_deps(ct, [(n, os.path.relpath(p, HUB)) for n, p, _ in new_crates])
    ot = regs.insert_owner_rows(texts[owners], [(pid, os.path.relpath(pkg, REPO), os.path.relpath(pkg, REPO) + '/📋️project.json') for _, pkg, pid in new_crates])
    pairs = [(n, pid.replace('@semio-tech/', '@semio-tech/')) for n, _, pid in new_crates]
    orders = regs.order_bands([texts[launch], texts[seed]], len(pairs))
    lt = regs.insert_routes(texts[launch], pairs, orders, False)
    st = regs.insert_routes(texts[seed], pairs, orders, True)
    out = {cargo: ct, owners: ot, launch: lt, seed: st}
    if not dry:
        for p, t in out.items():
            if open(p).read() != texts[p]:
                raise RuntimeError('concurrent edit detected on ' + p)
            open(p, 'w').write(t)
    return texts, out


if __name__ == '__main__':
    apply = '--apply' in sys.argv
    ws, sws, new_crates, mans, smans = stage()
    print('changed lib/bin files', len(ws.changed()), 'space files', len(sws.changed()), 'new crates', len(new_crates))
    texts, out = registries(new_crates, dry=True)
    for p in out:
        print('registry', os.path.relpath(p, REPO), len(texts[p]), '->', len(out[p]))
    longest = max((len(os.path.relpath(p, REPO).encode()) for p in ws.changed() + sws.changed()), default=0)
    print('longest changed path bytes', longest)
    import defscan
    from helpers import P
    base_roots = [('semio_hub', P('📦️packages/🦀️rust/🦀️.rs')), ('os_hub', P('🏗️bootstrap/🦀️.rs'))]
    lost, gained, un = defscan.scan('hub', base_roots, ws, list(plan_lib.CRATES.items()) if False else [(k, v['root']) for k, v in plan_lib.CRATES.items()], defscan.explained)
    slost, sgained, sun = defscan.scan('space', [('semio_hub_space', plan_space.ROOT)], sws, list(plan_space.ROOTS.items()), defscan.explained)
    open('/tmp/hubw/changed.txt', 'w').write('\n'.join(ws.changed() + sws.changed() + [p for p in out]) + '\n')
    if apply:
        a = ws.flush()
        b = sws.flush()
        registries(new_crates, dry=False)
        print('flushed', len(a), len(b))
