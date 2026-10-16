# 🧪️ Runs the renderer source-text readers with `bun test` against the live engine tree (A) and a baseline copy with the original renderer root (B).
import os, re, subprocess, sys, json
import oracle as o
tests = [
 'E/🧪️tests/♻️frame-close', 'E/🧪️tests/🛑️scene-pointer-cancellation', 'E/🧪️tests/🧾️frame-action-ledger', 'E/🧪️tests/🖼️wgpu-raster-residency',
 'E/🧪️tests/📨️browser-frame-transport', 'E/🧪️tests/🫀️settle-pump', 'E/🧪️tests/⏱️wgpu-worker-step-budget', 'E/🧪️tests/🔬️engine-contract',
 'E/🧪️tests/🧲️scene-input-residency', 'E/🧪️tests/⏱️frame-latency', 'E/🧪️tests/⏱️wgpu-ui-turn-budget', 'E/🧪️tests/🧲️engine-surface-retention',
 'E/🧪️tests/🖌️wgpu-document-owner-move', 'E/🧪️tests/🌳️wgpu-document-reconcile', 'E/🧪️tests/🫳️node-graph-gestures', 'E/🧪️tests/🪟️window-lifecycle-template-drag',
 'E/🧪️tests/♻️hot-swap-session-handover', 'E/🧪️tests/🔗️hub-projection', 'E/🧪️tests/🚗️driver-editor', 'E/🧪️tests/🧪️node-graph-edit-rows',
]
def path(base, t): return base + '/' + o.ENG + '/' + t[len('E/'):] + '/🟦️.ts'
def run(base, t, log):
    p = path(base, t)
    if not os.path.exists(p): return None
    r = subprocess.run(['bun', 'test', p], capture_output=True, text=True, cwd=os.path.dirname(p), timeout=900)
    out = r.stdout + r.stderr
    open(log, 'a').write('=== ' + t + '\n' + out + '\n')
    fails = sorted(set(re.findall(r'^\(fail\) (.*?)(?: \[[\d.]+m?s\])?$', out, re.M)))
    m = re.search(r'(\d+) pass', out); n = re.search(r'(\d+) fail', out)
    return (int(m.group(1)) if m else 0, int(n.group(1)) if n else 0, fails)
if __name__ == '__main__':
    tag = sys.argv[1]; mode = sys.argv[2] if len(sys.argv) > 2 else 'snapshot'
    GEN = o.GEN
    A = o.ROOT
    B = o.SCR + '/rwp0-ts-B'
    o.chain(B); o.baseline(B, mode)
    la, lb = f'{GEN}/rwp0-ts-{tag}-A.log', f'{GEN}/rwp0-ts-{tag}-B.log'
    for l in (la, lb):
        open(l, 'w').close()
    res = {}
    for t in tests:
        a = run(A, t, la); b = run(B, t, lb)
        res[t] = (a, b)
        print(('SAME ' if a == b else 'DIFF ') + t, 'A', a, 'B', b, flush=True)
    json.dump({k: v for k, v in res.items()}, open(f'{GEN}/rwp0-ts-{tag}.json', 'w'), ensure_ascii=False, indent=1)
