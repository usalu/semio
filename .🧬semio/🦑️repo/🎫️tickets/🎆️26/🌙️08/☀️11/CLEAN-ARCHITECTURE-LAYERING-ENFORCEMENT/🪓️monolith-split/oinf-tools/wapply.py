"""Atomic switch-over of the world carve into the repository (after the stage tree is verified)."""
import sys, os, re, json, shutil, hashlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wreg import *
from wgen import CRATES, crate_pkg

SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
LIVE = R + "/" + INFREL
OLD_DIRS = ['🧹️lifecycle', '🧵️interaction', '🧱️scene', '🖌️overlay', '🌉️bridge', '🎛️frame', '🖱️pointer', '📡️assets', '⏯️tool-run-trace']
TEST_DIRS_OLD = ['🔬️unit', '📜️journal-sequences', '📇️surface-verbs', '🖱️pointer-gestures', '🧲️scene-input-residency', '🌉️bridge-empty-build']

def tree_md5(root):
    h = hashlib.md5()
    for dp, dn, fn in sorted(os.walk(root)):
        dn[:] = sorted(d for d in dn if d != 'node_modules')
        for f in sorted(fn):
            p = os.path.join(dp, f)
            h.update(os.path.relpath(p, root).encode()); h.update(open(p, 'rb').read())
    return h.hexdigest()

def check_unchanged(snapshot):
    a = tree_md5(snapshot + '/🌍️world'); b = tree_md5(LIVE + '/🌍️world')
    return a == b, a, b

def copy_world(stage, crates):
    w = LIVE + '/🌍️world'
    for d in OLD_DIRS:
        shutil.rmtree(f"{w}/{d}", ignore_errors=True)
    for d in TEST_DIRS_OLD:
        p = f"{w}/🧪️tests/{d}/🦀️.rs"
        if os.path.exists(p): os.remove(p)
    for c in crates:
        shutil.rmtree(f"{w}/{c.folder}", ignore_errors=True)
        shutil.copytree(f"{stage}/🌍️world/{c.folder}", f"{w}/{c.folder}", symlinks=True)
    shutil.copyfile(f"{stage}/🌍️world/🦀️.rs", f"{w}/🦀️.rs")

def patch_facade_manifest(crates):
    p = LIVE + '/📦️packages/🦀️rust/Cargo.toml'; t = open(p, encoding='utf-8').read()
    anchor = '[target.\'cfg(not(all(target_arch = "wasm32", target_env = "p2")))\'.dependencies]\n'
    assert anchor in t
    add = ''.join(f'{crate_pkg(c.id)} = {{ path = "../../🌍️world/{c.folder}/📦️packages/🦀️rust" }}\n' for c in crates if f'{crate_pkg(c.id)} =' not in t)
    comment = "# 🌍️ The world family lives in producer crates (components, cursors, scene, bridge, interaction, host over the model); `src/🌍️world/🦀️.rs` is only their facade.\n"
    if add:
        t = t.replace(anchor, anchor + comment + add, 1)
        open(p, 'w', encoding='utf-8').write(t)

def edit_file(path, old, new, count=1):
    t = open(R + '/' + path, encoding='utf-8').read()
    assert t.count(old) == count, (path, old[:60], t.count(old))
    open(R + '/' + path, 'w', encoding='utf-8').write(t.replace(old, new))


E = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
CONSUMER_EDITS = [
 (f"{E}/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs", "if state.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {", "if state.session.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {", 1),
 (f"{E}/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs", "state.tool_run_trace_window_id", "state.trace.tool_run_trace_window_id", 3),
 (f"{E}/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs", "state.orbit.to_camera()", "state.camera.orbit.to_camera()", 4),
 (f"{E}/🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs", "state.bounds = Rect::new(PANE.0, PANE.1, PANE.2, PANE.3);", "state.session.bounds = Rect::new(PANE.0, PANE.1, PANE.2, PANE.3);", 1),
 (f"{E}/🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs", "state.bounds = if id == \"world\"", "state.session.bounds = if id == \"world\"", 1),
]

def apply_consumers():
    done = []
    for path, old, new, count in CONSUMER_EDITS:
        t = open(R + '/' + path, encoding='utf-8').read()
        if t.count(old) != count:
            done.append((path.split('/')[-2], 'SKIPPED', t.count(old), count)); continue
        open(R + '/' + path, 'w', encoding='utf-8').write(t.replace(old, new)); done.append((path.split('/')[-2], 'ok', count, count))
    return done

WORLD_ROOTS = [f"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/{c[2]}/🦀️.rs" for c in CRATES]

def apply_readers():
    out = []
    # fem policy script
    p = "✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts"
    old = 'const frameworkWorld = policyReadModuleTreeSource(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs");'
    arr = ', '.join(f'"{x}"' for x in WORLD_ROOTS)
    new = f'const frameworkWorld = [{arr}].map(root => policyReadModuleTreeSource(this.repoRoot, root)).join("\\n");'
    t = open(R + '/' + p, encoding='utf-8').read()
    if t.count(old) == 1: open(R + '/' + p, 'w', encoding='utf-8').write(t.replace(old, new)); out.append(('fem script', 'ok'))
    else: out.append(('fem script', 'SKIPPED'))
    # root script
    p = "📜️script.ts"
    t = open(R + '/' + p, encoding='utf-8').read()
    old1 = 'const INTERACTIVITY_AUDIT_WORLD3D_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs";'
    new1 = f'const INTERACTIVITY_AUDIT_WORLD3D_FILES = [{arr}];'
    old2 = 'const mountedFrameWorld3d = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_WORLD3D_FILE);'
    new2 = 'const mountedFrameWorld3d = INTERACTIVITY_AUDIT_WORLD3D_FILES.map(file => policyReadRustPolicySource(repoRoot, file)).join("\\n");'
    old3 = '  INTERACTIVITY_AUDIT_WORLD3D_FILE,\n'
    new3 = '  INTERACTIVITY_AUDIT_WORLD3D_FILES,\n'
    if t.count(old1) == 1 and t.count(old2) == 1 and t.count(old3) == 1:
        t = t.replace(old1, new1).replace(old2, new2).replace(old3, new3)
        open(R + '/' + p, 'w', encoding='utf-8').write(t); out.append(('root script', 'ok'))
    else: out.append(('root script', f'SKIPPED {t.count(old1)} {t.count(old2)} {t.count(old3)}'))
    # source access roots
    p = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts"
    t = open(R + '/' + p, encoding='utf-8').read()
    old = '  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",\n'
    new = ''.join(f'  "{x}",\n' for x in WORLD_ROOTS)
    if t.count(old) == 1: open(R + '/' + p, 'w', encoding='utf-8').write(t.replace(old, new)); out.append(('source-access', 'ok'))
    else: out.append(('source-access', 'SKIPPED'))
    return out

def rewrite_routes(testname_to_crate):
    """launch routes that filtered world tests of the monolithic crate"""
    pat = re.compile(r'bun repo run semio-framework-os-infinite:test( --skip-cache)?( --exclude-dependencies)? -- --offline --lib ([^ ]+)((?: -- [^"]*)?)')
    res = []
    for f in LAUNCH:
        t = open(f, encoding='utf-8').read()
        def sub(m):
            flt = m.group(3)
            cs = collections.Counter(c for n, c in testname_to_crate.items() if n.startswith(flt.rstrip(':')) or (flt == 'world::'))
            if flt == 'world::': cid = 'host'
            elif cs: cid = cs.most_common(1)[0][0]
            else:
                res.append((f.split('/')[-1], flt, 'UNMAPPED')); return m.group(0)
            res.append((f.split('/')[-1], flt, cid))
            tail = m.group(4)
            return f"bun repo run {project_name(cid)}:test{m.group(1) or ''}{m.group(2) or ''} -- --offline --tests{'' if flt == 'world::' else ' ' + flt}{tail}"
        t2 = pat.sub(sub, t)
        open(f, 'w', encoding='utf-8').write(t2)
    return res
