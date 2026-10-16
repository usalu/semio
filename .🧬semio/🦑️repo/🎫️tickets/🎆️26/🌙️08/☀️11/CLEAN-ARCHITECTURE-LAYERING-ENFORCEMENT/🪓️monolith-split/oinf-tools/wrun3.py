import sys; sys.path.insert(0,'.')
from wtgen import *
import wapi
from wext import external_world_names
SCRO = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
OUT = SCRO + '/stage-w3'

def main():
    import wgen
    pool, byfile = load_pool()
    extra = test_used_sets(pool)
    P = plan()
    crates, ic, imports = build_crates(P, extra)
    defcfg = wgen.LAST_DEFCFG
    wdef = {}
    for c in crates:
        for nm in c.defs: wdef[nm] = c.idx
    T = build_tests(P, crates, imports, defcfg)
    host = crates[-1]
    host.body.append('#[cfg(any(test, feature = "testkit"))]\n#[path = "🧰️testkit/🦀️.rs"]\nmod testkit;')
    shutil.rmtree(OUT, ignore_errors=True)
    shutil.copytree(SCRO + '/stage-w1', OUT, symlinks=True)
    E = set(external_world_names())
    write_crates(crates, OUT)
    facade = wapi.build_facade(crates, wdef, defcfg, E, OUT)
    open(OUT + '/🌍️world/🦀️.rs', 'w', encoding='utf-8').write(facade)
    # remove old test sources (keep fixtures and json)
    for d in TEST_DIRS:
        p = f"{OUT}/🌍️world/🧪️tests/{d}/🦀️.rs"
        if os.path.exists(p): os.remove(p)
    files = []
    for tg in T['targets']:
        path, text = gen_target(T, tg, crates, imports, defcfg, wdef, OUT)
        files.append((CRATES[tg['crate']][0], path, text.count('\n')))
    # testkit (host): with_world_step_context closure + Drop impl
    pool_ = T['pool']
    wsc = [k for k, t in pool_.items() if t.it.name == 'with_world_step_context'][0]
    tg = dict(crate=len(CRATES) - 1, dir='testkit', part=0, tests=[wsc], extra=T['special_drop'], keep_special=True,
              newdir=f"{OUT}/🌍️world/{host.folder}/🧰️testkit", head="//! 🧰️ Test-only host surface: the step context fixture and the close-driving `Drop` for `World3dState` (feature `testkit`).\n\n")
    path, text = gen_target(T, tg, crates, imports, defcfg, wdef, OUT)
    files.append(('host', path, text.count('\n')))
    return crates, T, files, P

if __name__ == '__main__':
    crates, T, files, P = main()
    for f in files: print(f[0], f[1][len(OUT):], f[2])
    print('unreached', [(T['pool'][k].dir, T['pool'][k].it.name) for k in T['unreached']])
