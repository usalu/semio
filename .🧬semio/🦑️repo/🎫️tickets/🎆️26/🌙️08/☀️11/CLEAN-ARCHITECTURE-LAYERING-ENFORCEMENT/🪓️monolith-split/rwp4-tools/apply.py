# 🚀️ WP4 apply (re-appliable): copies the verified Shell tree into the repo and rewrites the external ShellState consumers.
# state in <scratch>/apply-state: orig-shell (the pre-WP4 Shell element), applied-shell (what the last apply wrote), ext-orig / ext-applied (external files before / after the last apply),
# hub-orig (the WP4 hub before the carve), root-orig (shell-vocab skeleton root). A re-apply starts from the originals and refuses when a live file differs from what the last apply wrote.
# usage: apply.py <t1 tree> [--dry] [--carve <overlay>]
import sys, os, shutil, subprocess, json, filecmp
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
SH = SH_LIVE
from ext_overlay import candidate_files
from t1ext import rewrite_external
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
ST = SCR + '/apply-state'
E = ENG
SHELL_EL = R + '/' + E + '/🧱️elements/🐚️Shell'
HUB = E + '/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs'
VOC = E + '/🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs'
OWN = E + '/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts'
CFG = E + '/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
OWN_OLD = '      const end = shell.indexOf("\\n    fn ", start + 1);\n      const body = shell.slice(start, end < 0 ? undefined : end);'
OWN_NEW = '      const boundary = /\\n(?:    (?:pub(?:\\([^)]*\\))?\\s+)?(?:async\\s+)?fn |\\})/gu;\n      boundary.lastIndex = start + 1;\n      const end = boundary.exec(shell)?.index ?? -1;\n      const body = shell.slice(start, end < 0 ? undefined : end);'
RET = E + '/🧪️tests/🧲️engine-surface-retention/🟦️.ts'
RET_OLD = '    const shell = readFileSync(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"), "utf8");\n    expect(shell).toContain("fn mirror_engine_surface_states(");'
RET_NEW = '    const shell = readModuleTreeSource(join(engineRoot, "🧱️elements", "🐚️Shell", "🎯️targets", "🧊️wgpu", "🦀️.rs"));\n    expect(shell).toContain("fn mirror_engine_surface_states(");'
CFG_ADD = ', "🧪️tests/🧬️shell-component-decomposition/🟦️.ts"'

def rd(p): return open(p, encoding='utf-8').read()
def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True); open(p, 'w', encoding='utf-8').write(t)

def tree_equal(a, b):
    d = subprocess.run(['diff', '-rq', '--exclude=node_modules', a, b], capture_output=True, text=True)
    return d.stdout.strip() == '', d.stdout[:1500]

def main(tree, dry, ovc):
    first = not os.path.exists(ST + '/orig-shell')
    if first:
        ok, why = tree_equal(SCR + '/orig-shell', SHELL_EL); assert ok, 'live Shell element differs from the pre-WP4 snapshot:\n' + why
        if not dry:
            shutil.copytree(SCR + '/orig-shell', ST + '/orig-shell')
            shutil.copyfile(SCR + '/orig-hub-wp4.rs', ST + '/hub-orig.rs')
            wr(ST + '/root-orig.rs', rd(R + '/' + VOC))
    else:
        ok, why = tree_equal(ST + '/applied-shell', SHELL_EL); assert ok, 'live Shell element differs from the last applied tree (peer edit?):\n' + why
        shutil.rmtree(SH + '/📥️staging', ignore_errors=True) if not dry else None
    tree = os.path.realpath(tree)
    n = 0
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            src = os.path.join(dp, f); rel = os.path.relpath(src, tree)
            dst = os.path.join(SH, rel)
            assert os.path.realpath(os.path.dirname(dst)).startswith(os.path.realpath(SH)), dst
            if os.path.exists(dst) and open(dst, 'rb').read() == open(src, 'rb').read(): continue
            n += 1
            if not dry:
                os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(src, dst)
    print('shell tree files written:', n)
    changed = []; report = []
    for rel in sorted(set(candidate_files()) | (set(json.load(open(ST + '/ext-files.json'))) if os.path.exists(ST + '/ext-files.json') else set())):
        p = os.path.join(R, rel)
        if not os.path.exists(p): continue
        live = rd(p)
        so, sa = ST + '/ext-orig/' + rel, ST + '/ext-applied/' + rel
        if os.path.exists(so):
            assert live == rd(sa), 'external file differs from the last applied text: ' + rel
            base = rd(so)
        else:
            base = live
        new = rewrite_external(base, report, rel)
        if new != base or os.path.exists(so):
            changed.append(rel)
            if not dry:
                if not os.path.exists(so): wr(so, base)
                wr(sa, new)
                assert rd(p) == live, 'file changed under us: ' + rel
                wr(p, new)
    print('external files rewritten:', len(changed), 'reports', len(report))
    for r in report[:20]: print(r)
    if ovc:
        hub = rd(R + '/' + HUB)
        if first: assert hub == rd(SCR + '/orig-hub-wp4.rs'), 'hub changed'
        else: assert hub == rd(ST + '/hub-applied.rs'), 'hub differs from the last applied'
        sk = rd(R + '/' + VOC)
        assert ('WP4 skeleton' in sk) if first else (sk == rd(ST + '/root-applied.rs')), 'vocab root differs'
        if not dry:
            for rel, key in ((HUB, 'hub'), (VOC, 'root')):
                shutil.copyfile(os.path.join(ovc, rel), R + '/' + rel); wr(ST + f'/{key}-applied.rs', rd(os.path.join(ovc, rel)))
        print('carve files written')
    for rel, old, new in ((OWN, OWN_OLD, OWN_NEW), (RET, RET_OLD, RET_NEW)):
        raw = rd(R + '/' + rel)
        if old in raw:
            assert raw.count(old) == 1
            if not dry: wr(R + '/' + rel, raw.replace(old, new))
            print('edited:', rel[-50:])
    raw = rd(R + '/' + CFG)
    if 'shell-component-decomposition' not in raw:
        i = raw.index('include: ['); j = raw.index(']', i)
        if not dry: wr(R + '/' + CFG, raw[:j] + CFG_ADD + raw[j:])
        print('edited: vitest include')
    if not dry:
        shutil.rmtree(ST + '/applied-shell', ignore_errors=True); shutil.copytree(SHELL_EL, ST + '/applied-shell', ignore=shutil.ignore_patterns('node_modules'))
        json.dump(changed, open(ST + '/ext-files.json', 'w'), ensure_ascii=False)
        json.dump(changed, open(GEN + '/rwp4-ext-applied.json', 'w'), ensure_ascii=False, indent=0)

if __name__ == '__main__':
    ovc = sys.argv[sys.argv.index('--carve') + 1] if '--carve' in sys.argv else None
    main(sys.argv[1], '--dry' in sys.argv, ovc)
