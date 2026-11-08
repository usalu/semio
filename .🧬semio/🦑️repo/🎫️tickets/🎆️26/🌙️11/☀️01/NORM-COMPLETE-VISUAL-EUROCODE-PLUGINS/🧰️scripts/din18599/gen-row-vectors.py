"""Adds value bounds and the rejected / no-op / duplicate vectors to the per-row DIN V 18599 leaves, registers them (oracle manifest, Python rows, feature table, Rust harness) and runs the Python oracle over every new row.

Run after `gen-row-leaves.py` (which it reads for the leaf table): `python3 gen-row-vectors.py`.
"""
import importlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
src = open(f'{HERE}/gen-row-leaves.py', encoding='utf-8').read()
src = src[:src.rindex('\nmain()')]
ns = {}
exec(compile(src, 'gen', 'exec'), ns)
globals().update({k: ns[k] for k in ('LEAVES', 'MUT', 'FIX', 'SUB', 'BASE', 'BOUNDS', 'REJECTED', 'TARGET', 'NEW_VALUES', 'EMOJI', 'dump', 'wr', 'rd', 'schema_json', 'diff_rs', 'meta_json', 'vector')})

NOOP, GONE, RULE, DUPE = '🟰noop', '❓gone', '🚫rule', '⛔dupe'


def outcome(status, code=None, level=None):
    if status == 'applied':
        return {'status': 'applied'}
    return {'status': status, 'code': code, 'messages': [{'level': level, 'code': code}]}


def base_for(l):
    doc = json.loads(json.dumps(BASE))
    if l.verb == 'change' and l.entity == 'element' and l.field[0] == 'zone-id':
        doc['zones'].append({**doc['zones'][0], 'id': 'zone-annex', 'labelEn': 'Annex', 'labelDe': 'Anbau'})
    return doc


def write(l, scenario, before, mutation, after, out):
    base = f'{FIX}/{l.dir}/{scenario}'
    for role, value in (('before', before), ('mutation', mutation), ('after', after), ('outcome', out)):
        wr(f'{base}/🔣️.{role}.json', dump(value))


def scenarios_of(l):
    out = [('apply', '✅apply')]
    if l.verb == 'change':
        out.append(('noop', NOOP))
        out.append(('gone', GONE))
        if (l.entity, l.field[2]) in BOUNDS:
            out.append(('rule', RULE))
    elif l.verb == 'insert':
        out.append(('dupe', DUPE))
    else:
        out.append(('gone', GONE))
    return out


def extra_vectors(l):
    doc = base_for(l)
    coll = l.e['coll']
    if l.verb == 'change':
        f = l.field
        idk, nk = f'{l.entity}Id', 'new' + f[2][0].upper() + f[2][1:]
        target = TARGET[l.entity]
        row = next(r for r in doc[coll] if r['id'] == target)
        write(l, NOOP, doc, {'mutation': l.camel, idk: target, nk: row[f[2]]}, doc, outcome('no-op', 'mutation.no-op', 'warning'))
        write(l, GONE, doc, {'mutation': l.camel, idk: 'ghost', nk: NEW_VALUES[(l.entity, f[2])]}, doc, outcome('rejected', 'mutation.target-missing', 'error'))
        if (l.entity, f[2]) in BOUNDS:
            write(l, RULE, doc, {'mutation': l.camel, idk: target, nk: REJECTED[(l.entity, f[2])]}, doc, outcome('rejected', 'mutation.invariant', 'fatal'))
    elif l.verb == 'insert':
        existing = doc[coll][0]
        write(l, DUPE, doc, {'mutation': l.camel, 'index': 0, l.entity: existing}, doc, outcome('rejected', 'mutation.duplicate-id', 'fatal'))
    else:
        write(l, GONE, doc, {'mutation': l.camel, 'id': 'ghost'}, doc, outcome('rejected', 'mutation.target-missing', 'error'))


def tag_of(l):
    return json.loads(rd(f'{MUT}/{l.dir}/🔣️.json'))['binaryTag']


def classes_of(l):
    ids = [s[0] for s in scenarios_of(l)]
    return ['applied'] + (['no-op'] if 'noop' in ids else []) + (['rejected'] if {'gone', 'rule', 'dupe'} & set(ids) else [])


def main():
    for l in LEAVES:
        extra_vectors(l)
        d = f'{MUT}/{l.dir}'
        if l.verb == 'change':
            wr(f'{d}/🧬️schema/🔣️.json', schema_json(l))
            wr(f'{d}/🔺️diff/🦀️.rs', diff_rs(l))
        meta = json.loads(rd(f'{d}/🔣️.json'))
        meta['outcomeClasses'] = classes_of(l)
        wr(f'{d}/🔣️.json', dump(meta))
    # oracle manifest
    p = f'{SUB}/🔮️oracles/🔣️.json'
    m = json.loads(rd(p))
    leaves = {l.kind: l for l in LEAVES}
    for catalog in m['mutationCatalogs']:
        for vector_entry in catalog['vectors']:
            l = leaves.get(vector_entry['mutationId'])
            if l:
                vector_entry['scenarios'] = [{'id': 'apply', 'directoryName': '✅apply'}] + [{'id': f'{l.kind}-{sid}', 'directoryName': dirname} for sid, dirname in scenarios_of(l)[1:]]
    for manifest in m['mutationManifests']:
        for entry in manifest['mutations']:
            l = leaves.get(entry['id'])
            if l:
                entry['outcomes'] = classes_of(l)
    wr(p, dump(m))
    # python rows
    p = f'{SUB}/🧪️tests/⚡️mutate-din18599-1/🐍️.py'
    t = rd(p)
    marker = '    "change-delta-u-wb-rule": ("change-delta-u-wb", "🌉change-delta-u-wb", "🚫rule"),\n'
    assert marker in t
    rows = ''.join(f'    "{l.kind}-{sid}": ("{l.kind}", "{l.dir}", "{dirname}"),\n' for l in LEAVES for sid, dirname in scenarios_of(l)[1:])
    if f'"{LEAVES[0].kind}-noop"' not in t:
        t = t.replace(marker, marker + rows, 1)
    wr(p, t)
    # feature rows
    p = f'{SUB}/🧪️tests/⚡️mutate-din18599-1/🥒️.feature'
    t = rd(p)
    marker = '      | change-delta-u-wb-rule        | 🌉change-delta-u-wb         | 🚫rule   |\n'
    assert marker in t
    frows = ''.join(f'      | {l.kind + "-" + sid:<29} | {l.dir} | {dirname}  |\n' for l in LEAVES for sid, dirname in scenarios_of(l)[1:])
    if f'{LEAVES[0].kind}-noop' not in t:
        t = t.replace(marker, marker + frows, 1)
        t = t.replace('  Four `<kind>-rule` rows ask', '''  Every per-row leaf also has refusal rows: `<kind>-gone` addresses an id the document does not hold
  (`mutation.target-missing`), `<kind>-rule` writes a value outside the physical domain its leaf payload schema states
  (`mutation.invariant`), `<kind>-dupe` inserts an id the list already holds (`mutation.duplicate-id`) and
  `<kind>-noop` re-applies the current value (`mutation.no-op`); each leaves the document bit-identical.

  Four `<kind>-rule` rows ask''', 1)
    wr(p, t)
    # rust harness
    p = f'{SUB}/🧪️tests/⚡️mutate-din18599-1/🦀️.rs'
    t = rd(p)
    if 'pub fn rows()' not in t:
        a = t.index('    pub fn mutate(kind: &\'static str)')
        b = t.index('    /// ↩️ The metamorphic inverse law')
        t = t[:a] + '''    pub fn mutate(row: String) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let [before, mutation, after, outcome] = vector(ctx)?;
            let (base, expected, mutation) = (snapshot_of(&before, "before")?, snapshot_of(&after, "after")?, mutation_of(&mutation)?);
            let outcome = parse_json(&outcome)?;
            let status = outcome.str("status");
            if !["applied", "no-op", "rejected"].contains(&status.as_str()) {
                return Err(format!("mutate-{row}: unknown committed outcome status {status:?}"));
            }
            let spelled = |message: &Json| {
                let level = message.str("level");
                format!("{}{}:{}", level.get(..1).unwrap_or_default().to_uppercase(), level.get(1..).unwrap_or_default(), message.str("code"))
            };
            let promised: Vec<String> = outcome.array("messages").iter().map(spelled).collect();
            let (current, messages) = apply_din18599_mutation(&base, &mutation).map_err(|error| format!("mutate-{row}: production dispatch failed: {error}"))?;
            if messages != promised {
                return Err(format!("mutate-{row}: production dispatch raised {messages:?}, the committed outcome {promised:?}"));
            }
            if current != expected {
                return Err(disagreement(&format!("mutate-{row}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(&row, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{row}: a {status} mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// 🧫️ The refusal, no-op and duplicate rows the subset's committed catalog registers beside each kind's canonical vector.
    pub fn rows() -> Vec<String> {
        let manifest = parse_json(include_str!("../../🔮️oracles/🔣️.json")).expect("the committed oracle manifest is JSON");
        manifest.array("mutationCatalogs").iter().flat_map(|catalog| catalog.array("vectors")).flat_map(|vector| vector.array("scenarios").into_iter().skip(1)).map(|scenario| scenario.str("id")).collect()
    }

''' + t[b:]
        t = t.replace('''            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }''', '''            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind.to_string())).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }
        for row in subject::rows() {
            built = built.subject(&format!("mutate-{row}"), subject::mutate(row));
        }''', 1)
        t = t.replace("/// 🧭️ Registration by FULL expanded scenario id, one `mutate-`/`inverse-` pair per kind of the production vocabulary\n/// plus the carrier identity.", "/// 🧭️ Registration by FULL expanded scenario id, one `mutate-`/`inverse-` pair per kind of the production vocabulary\n/// plus the carrier identity and one `mutate-` row per further catalog vector.")
        wr(p, t)
    validate()


class Ctx:
    def input_bytes(self, uri):
        assert uri.startswith('shared://🧬️mutations/'), uri
        return open(f'{FIX}/' + uri[len('shared://🧬️mutations/'):], 'rb').read()


def validate():
    import types
    stub = types.ModuleType('semio_repo_test')

    class Adapter:
        def __init__(self, *_):
            pass

        def oracle(self, *_):
            return self

        def subject(self, *_):
            return self

    class Outcome:
        def __init__(self, *args, **kwargs):
            pass

    import hashlib
    stub.Adapter, stub.Outcome = Adapter, Outcome
    stub.digest = lambda data: hashlib.sha256(data).hexdigest()
    stub.__getattr__ = lambda name: (lambda *a, **k: None)
    sys.modules['semio_repo_test'] = stub
    sys.path.insert(0, '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution')
    for name in list(sys.modules):
        if name == '🐍️':
            del sys.modules[name]
    vocabulary = importlib.import_module('🐍️')
    spec = importlib.util.spec_from_file_location('din_mutate', f'{SUB}/🧪️tests/⚡️mutate-din18599-1/🐍️.py')
    module = importlib.util.module_from_spec(spec)
    sys.path.insert(0, f'{SUB}/🧪️tests/⚡️mutate-din18599-1')
    spec.loader.exec_module(module)
    adapter = module.adapter()
    subset = module.Subset('DIN V 18599', module.KINDS, module.VECTORS, module.DSL_ASSET, module.ENVELOPE, schemas=module.SCHEMAS, documents=module.DOCUMENTS)
    vocabulary.declare_edit_rules(subset)
    ctx = Ctx()
    bad = 0
    rows = [l.kind for l in LEAVES] + [f'{l.kind}-{sid}' for l in LEAVES for sid, _ in scenarios_of(l)[1:]]
    for row in rows:
        try:
            vocabulary.mutate_handler(subset, row)(ctx)
            if row in [l.kind for l in LEAVES]:
                vocabulary.inverse_handler(subset, row)(ctx)
        except Exception as error:
            bad += 1
            print('FAIL', row, str(error)[:300])
    print('python oracle rows checked', len(rows), 'failures', bad)


if __name__ == '__main__':
    main()
