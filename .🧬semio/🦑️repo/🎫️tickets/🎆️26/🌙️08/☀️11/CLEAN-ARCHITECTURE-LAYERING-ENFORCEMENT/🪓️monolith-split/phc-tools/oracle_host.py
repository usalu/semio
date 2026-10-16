#!/usr/bin/env python3
"""🔬️ Full-resolution differential for the plugin-host carve: the pristine monolith against the carved crates, stitched into one analysis-only rustc run per configuration."""
import os, sys, re, json, pickle, collections, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ws as wsm, hcmod, union_host as U
import plan_host
from carve import REPO

SCOPE = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
BASE_ROOT = plan_host.P('📦️packages/🦀️rust/🦀️.rs')
BASE_IDENT = 'semio_framework_plugin_host'
CACHE = '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/phc-oracle'
FEAT = ('feature="testkit"',)


def load_external():
    out = subprocess.run(['bun', '-e', 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))', plan_host.P('📦️packages/🦀️rust/Cargo.toml')], capture_output=True, text=True, cwd=REPO).stdout
    t = json.loads(out)
    ids = set()
    for tbl in ('dependencies', 'dev-dependencies'):
        for k, v in t.get(tbl, {}).items():
            ids.add(k.replace('-', '_'))
            if isinstance(v, dict) and 'package' in v:
                ids.add(v['package'].replace('-', '_'))
    ids |= {'std', 'core', 'alloc', 'dsl', 'protocol', 'store', 'wasmtime', 'wasmtime_wasi', 'serde', 'serde_json', 'tokio', 'semio_framework_hash'}
    return ids


U.EXTERNAL = load_external()


def baseline(label, under, cfgs=()):
    os.makedirs(CACHE, exist_ok=True)
    pk = f'{CACHE}/base-{label}.pkl'
    if os.path.exists(pk):
        return pickle.load(open(pk, 'rb'))
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    u = U.Union(base_ws, collections.OrderedDict([(BASE_IDENT, BASE_ROOT)]), SCOPE, tag='base-' + label)
    diags, err = u.run(BASE_IDENT if under else None, tuple(cfgs) + FEAT)
    c = U.multiset(diags)
    pickle.dump((c, [d for d in diags if U.key_of(d)]), open(pk, 'wb'))
    return c, [d for d in diags if U.key_of(d)]


def split(label, ws, roots, under, cfgs=()):
    import time
    last = None
    for attempt in range(8):
        try:
            u = U.Union(ws, roots, SCOPE, tag='split-' + label.replace(':', '_'))
            diags, err = u.run(under, tuple(cfgs) + FEAT)
            return U.multiset(diags), [d for d in diags if U.key_of(d)]
        except OSError as e:
            last = e
            time.sleep(20 * (attempt + 1))
    raise last


def clean(c, crates):
    return collections.Counter({k: n for k, n in c.items() if not U.is_extern_noise(k[0], k[1], crates)})


def compare(label, b, s, crates, limit=80, show=True):
    bc, sc = clean(b[0], crates), clean(s[0], crates)
    flat = lambda c: collections.Counter({k: sum(n for (a, m, f), n in c.items() if (a, m) == k) for k in {(a, m) for (a, m, f) in c}})
    fb, fs = flat(bc), flat(sc)
    only_s, only_b = U.diff(fs, fb)
    print(f'[{label}] base {sum(bc.values())} / split {sum(sc.values())}; only-split {sum(only_s.values())} only-base {sum(only_b.values())}')
    if show:
        files = collections.defaultdict(list)
        for (code, msg, f), n in sc.items():
            files[(code, msg)].append(f[-72:])
        for (code, msg), n in sorted(only_s.items(), key=lambda x: (x[0][1], x[0][0]))[:limit]:
            print(f'   SPLIT+ {n}x {code} {msg[:170]}   @ {sorted(set(files[(code, msg)]))[:2]}')
    return only_s, only_b


def run_native(ws, roots, show=True):
    b = baseline('native', None)
    s = split('native', ws, roots, None)
    return compare('native', b, s, set(roots), show=show)


def run_test(ws, roots, crate, show=True):
    b = baseline('test', True)
    s = split('test_' + crate, ws, roots, crate)
    return compare('test:' + crate, b, s, set(roots), show=show)


if __name__ == '__main__':
    ws, C, roots, base_ws = plan_host.prepare()
    mode = sys.argv[1] if len(sys.argv) > 1 else 'native'
    if mode == 'native':
        run_native(ws, roots)
    else:
        for c in (sys.argv[2:] or list(roots)):
            run_test(ws, roots, c)


def repair(ws, C, roots, rounds=4, under=None):
    import autovis
    total = 0
    for r in range(rounds):
        s = split('repair-' + (under or 'native'), ws, roots, under)
        C.build_new(roots)
        patches, why = autovis.plan(C, s[1], roots)
        n = autovis.apply(ws, patches)
        autovis.save_decisions(autovis.load_decisions() + why)
        print(f'repair round {r}: {n} widenings', collections.Counter((w['code'], w['kind']) for w in why))
        total += n
        if n == 0:
            break
        C.build_new(roots)
    return total
