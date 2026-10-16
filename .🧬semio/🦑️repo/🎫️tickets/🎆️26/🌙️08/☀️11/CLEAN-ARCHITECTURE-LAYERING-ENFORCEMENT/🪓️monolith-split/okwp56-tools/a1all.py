# 🧱 A1 driver: builds the overlay {path: new text} of the whole decomposition (store root, durable group, member group, codec glue) without writing the repository.
import json, os, sys, builtins, io, hashlib
import em, rs, a1gen, a1ext, a1c1
from a1ext import hop
S = em.S
DG = S + '/🧩️composition/🗄️durable-group/🦀️.rs'
MG = S + '/🧩️composition/📬️publication/🤝️group/🦀️.rs'

PRE = {}
def load_pre():
    R = '/Users/ueli/Documents/semio'
    HERE = em.HERE
    for line in open(HERE + '/pre-a1/INDEX.txt', encoding='utf8').read().split('\n'):
        if not line.strip(): continue
        f, rel = line.split('\t')
        PRE[os.path.join(R, rel)] = open(os.path.join(HERE, os.path.relpath(f, HERE) if os.path.isabs(f) else f), encoding='utf8').read()
    PRE[em.ROOT] = open(HERE + '/pre-a1/root-pre-a1.rs', encoding='utf8').read()
    _open = builtins.open
    def popen(file, mode='r', *a, **k):
        if 'r' in mode and 'b' not in mode and str(file) in PRE: return io.StringIO(PRE[str(file)])
        return _open(file, mode, *a, **k)
    builtins.open = popen

def build_all():
    t = em.read()
    ext = json.load(open(em.HERE + '/../🗑️generated/okwp56-ext-method-use.json'))
    new, ms, stats = a1gen.build_a1(t, ext)
    new = a1gen.post_construct(new)
    new = a1c1.patch_c1(new)
    new = new.replace('store.envelope.active_alternative_id = saved_line.clone();', 'store.log.envelope.active_alternative_id = saved_line.clone();').replace('store.envelope.viewer_checkpoint_id = saved_checkpoint.clone();', 'store.log.envelope.viewer_checkpoint_id = saved_checkpoint.clone();')
    its = rs.outline(new)
    L = new.split('\n')
    for name in ('close_codec_reduction_store', 'replay_envelopes_onto_pair', 'bare_artifact_codec'):
        it = [i for i in its if i['kind'] == 'fn' and i['name'] == name]
        assert len(it) == 1, name
        it = it[0]
        body = '\n'.join(L[it['s'] - 1: it['e']])
        nb, n = hop(body, {'store'})
        L[it['s'] - 1: it['e']] = nb.split('\n') if n else body.split('\n')
        its = rs.outline('\n'.join(L)); 
    out = {em.ROOT: '\n'.join(L)}
    dg = open(DG, encoding='utf8').read()
    out[DG] = a1ext.convert_durable_group(dg)
    mg = open(MG, encoding='utf8').read()
    out[MG] = a1ext.convert_member_group(mg)
    import a1tests
    tout, trep = a1tests.convert_tests([S + '/' + f for f in a1tests.TEST_FILES])
    out.update(tout)
    stats['tests'] = trep
    return out, ms, stats

builtins_open = builtins.open
if __name__ == '__main__':
    if '--pre' in sys.argv: load_pre()
    out, ms, stats = build_all()
    import hashlib
    hashes = {p: hashlib.sha256(builtins_open(p, 'rb').read()).hexdigest() for p in out}
    json.dump({'out': out, 'hashes': hashes}, open(em.HERE + '/../🗑️generated/okwp56-a1-overlay.json', 'w'), ensure_ascii=False)
    for p, t in out.items(): print(len(t.split('\n')), os.path.relpath(p, S))
    print('delegates', stats['delegates'])
