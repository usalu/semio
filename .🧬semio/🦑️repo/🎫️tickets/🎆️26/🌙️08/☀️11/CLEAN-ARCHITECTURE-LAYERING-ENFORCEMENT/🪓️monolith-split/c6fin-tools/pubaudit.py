"""🔏 C6PLUG2 python port of the puzzle publication-authority ownerOracle (📜️script.ts) for the 2d and 5d owners, run over the original and the split source concatenations. usage: pubaudit.py"""
import json, os, re, sys
R = '/Users/ueli/Documents/semio/'
T = os.path.dirname(os.path.abspath(__file__))
ft = dict(l.rstrip('\n').split('\t') for l in open(T + '/files.tsv', encoding='utf-8'))
J = R + '✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json'
fixture = json.load(open(J, encoding='utf-8'))
PUZ = R + '✏️s/🔌️plugins/🧩️puzzle/'
def variant(v): return ''.join(p[0].upper() + p[1:] for p in v.split('-'))
def quoted(s): return re.findall(r'"([^"]+)"', s)
def exact(a, b): return sorted(a) == sorted(b) and len(set(a)) == len(a) and len(set(b)) == len(b)
reserved = {"copy", "cut", "paste", "import-media"}
def pairs(s):
    return dict(re.findall(r'\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)', s))
def retained_ids(s, owner):
    dim = owner[6:8].upper()
    m = re.search(r'PUZZLE%s_RETAINED_TOOL_IDS: &\[&str\] = &\[([\s\S]*?)\];' % dim, s)
    if not m: return None
    return quoted(m.group(1))
def contracts(s):
    out = {}
    for m in re.finditer(r'ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]', s):
        out[m.group(1)] = re.findall(r'ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|WindowConfig|WindowTransient|Child|Interaction|HostOnly)', m.group(2))
    return out
def impl_for(p, tr, ty): return re.search(r'impl (?:[A-Za-z_][A-Za-z0-9_]*::)*%s for %s\b' % (tr, ty), p) is not None
def oracle(owner, source):
    name = owner['owner']
    production = source.split('//#region 🧪️UnitTests')[0]
    pr = pairs(production)
    groups = [dict(g, routes=[r for r in g['routes'] if not (name != 'Puzzle5dPlayApp' or r not in reserved) or False] if False else [r for r in g['routes'] if (name != 'Puzzle5dPlayApp' or r not in reserved) and (name != 'Puzzle2dPlayApp' or r not in reserved)]) for g in owner['groups']]
    app_routes = [r for g in groups for r in g['routes']]
    migrated = [r for g in groups if g['status'] == 'migrated' for r in g['routes']]
    expected = {r: variant(g['status']) for g in groups for r in g['routes']}
    if not exact(list(pr), app_routes): return 'pairs differ: %s | %s' % (sorted(set(pr) ^ set(app_routes))[:5], len(pr))
    if not all(pr.get(r) == expected.get(r) for r in app_routes): return 'pair status'
    ri = retained_ids(production, name)
    if ri is not None and not exact(ri, migrated): return 'retained ids'
    factory = name[:8] + 'RetainedCommandJobFactory'
    parts = re.split(r'impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for %s\b' % factory, production)
    fb = parts[1].split('//#endregion 🧵️RetainedCommands')[0] if len(parts) > 1 else ''
    c = contracts(fb)
    pp = production.split('semio_framework_plugin::bounded_first_step_tool_proofs!')
    proof_block = pp[1].split('fn register_tool_job_factories')[0] if len(pp) > 1 else ''
    pm = re.search(r'tools:\s*\[([^\]]*)\]', proof_block)
    proof_ids = quoted(pm.group(1)) if pm else []
    ok = (impl_for(production, 'ToolJobFactory', factory) and impl_for(production, 'ArtifactOwnedToolJobFactory', factory)
          and ('factory: "%s"' % factory) in proof_block and ('factory_type: %s' % factory) in proof_block
          and re.search(r'type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<%s>;' % name, production)
          and (('registry.register(%s::new(&controller_id))' % factory) if name == 'Puzzle5dPlayApp' else ('registry.register(%s::new(&controller))' % factory)) in production
          and 'fn build_artifact_store_one_item_preparation_factory()' in production and 'fn build_config_store_one_item_preparation_factory()' in production
          and 'build_draft_store_one_item_preparation_factory' not in production and 'build_presence_store_one_item_preparation_factory' not in production and 'build_transient_store_one_item_preparation_factory' not in production
          and not re.search(r'"setLocale"|"setTerminology"|ConfigMutation::Set(?:Locale|Terminology)', production))
    if not ok: return 'factory block: impl=%s proof=%s' % (impl_for(production, 'ArtifactOwnedToolJobFactory', factory), bool(proof_block))
    mig = [g for g in groups if g['status'] == 'migrated']
    exp = {r: [variant(l) for l in g['lanes']] for g in mig for r in g['routes']}
    if not (exact(list(c), list(exp)) and all(exact(c.get(r, []), l) for r, l in exp.items())): return 'contracts: %s' % sorted(set(c) ^ set(exp))[:6]
    if not exact(proof_ids, migrated): return 'proof ids %s' % sorted(set(proof_ids) ^ set(migrated))[:5]
    if name == 'Puzzle2dPlayApp':
        needles = ['struct Puzzle2dConfigStorePreparationFactory', 'Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))', 'struct Puzzle2dArtifactStorePreparationFactory', 'Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))', 'PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536', 'request.operation != request.authority.operation()', 'ArtifactStoreOneItemPreparationStep::Progress', 'fn cancel(&mut self)', 'fn begin_close(&mut self)', 'base.return_to_registry()', 'fn terminal_is_empty(&self)', 'fn puzzle2d_dispatch_emit(', 'PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024', 'if addressed > PUZZLE2D_SELECTION_BATCH_LIMIT {\n        return None;\n    }', 'Some(addressed.saturating_add(connects).max(1))']
        miss = [n for n in needles if n not in production]
        return 'OK' if not miss else 'missing %s' % miss
    guard = production.find('if !["copy", "cut", "paste", "import-media"].contains(&request.tool_id.as_str())')
    decode = production.find('puzzle5d_preflight_reserved_wire', guard)
    needles = ['type Owner = EditorApp<Puzzle5dPlayApp>;', 'fn build_artifact_store_one_item_preparation_factory()', 'Some(std::sync::Arc::new(Puzzle5dStorePreparationFactory))', 'fn build_config_store_one_item_preparation_factory()', 'Some(std::sync::Arc::new(Puzzle5dConfigStorePreparationFactory))']
    miss = [n for n in needles if n not in production]
    bad = [n for n in ('Puzzle5dCopyJobFactory', 'puzzle5d_reserved_factory!') if n in production]
    return 'OK' if not miss and not bad and guard >= 0 and decode > guard else 'missing %s %s guard=%s decode=%s' % (miss, bad, guard, decode)
def source_of(owner, orig_overrides):
    root = PUZ + owner['source']
    d = os.path.dirname(root)
    texts = []
    for m in owner.get('mounted', []):
        p = os.path.normpath(os.path.join(d, m))
        texts.append(orig_overrides.get(p) or open(p, encoding='utf-8').read())
    rootp = os.path.normpath(root)
    texts.append(orig_overrides.get(rootp) or open(root, encoding='utf-8').read())
    return '\n'.join(texts)
if __name__ == '__main__':
    ov = {os.path.normpath(R + ft[t]): open('%s/orig/%s.rs' % (T, t), encoding='utf-8').read() for t in ('pe', 'pr', 'p5')}
    for o in fixture['owners']:
        if o['owner'] == 'Puzzle3dPlayApp': continue
        old = dict(o, mounted=[m for m in o['mounted'] if '/' not in m.replace('/🦀️.rs', '')])
        print(o['owner'], 'ORIG', oracle(old, source_of(old, ov)), '| NEW', oracle(o, source_of(o, {})))
