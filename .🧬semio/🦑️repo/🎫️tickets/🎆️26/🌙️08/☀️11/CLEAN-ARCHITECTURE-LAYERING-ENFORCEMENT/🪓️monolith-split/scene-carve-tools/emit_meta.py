"""📦 Emits manifests and project files of the carved ui-scene crates into the staged tree and infers their dependencies from the sources."""
import os, re
from lib import *
from model import *
from synth import code_idents, mounts

FW = "🧰️framework"
MODS = FW + "/🔨️modules"
EXT = {
    "protocol": (MODS + "/📡️replication/📦️packages/🦀️rust", 'protocol = { path = "%s", package = "semio-framework-replication" }'),
    "semio_framework_geometry": (MODS + "/📐️geometry/📦️packages/🦀️rust", 'semio-framework-geometry = { path = "%s" }'),
    "semio_framework_mesh_engine": (MODS + "/🏗️mesh-engine/📦️packages/🦀️rust", 'semio-framework-mesh-engine = { path = "%s" }'),
    "semio_framework_ui_viewport": (MODS + "/🖱️ui/🪟️viewport/📦️packages/🦀️rust", 'semio-framework-ui-viewport = { path = "%s" }'),
    "semio_framework_pack_json": (MODS + "/🎒️pack/🔤️json/📦️packages/🦀️rust", 'semio-framework-pack-json = { path = "%s" }'),
    "ui_contract": (MODS + "/🖱️ui/🧬️contract/📦️packages/🦀️rust", 'ui_contract = { path = "%s", package = "semio-framework-ui-contract" }'),
}
FIXED = {
    "semio_framework_value": 'semio-framework-value = { workspace = true }',
    "serde": 'serde = { version = "1.0.219", features = ["derive"] }',
    "serde_json": 'serde_json = { version = "1.0.140" }',
    "fnv": 'fnv = "1.0.7"',
}
SIB = "semio_framework_ui_scene_"
CFG_TEST = re.compile(r'#\[cfg\((?:test|any\(test, feature = "testkit"\))\)\]\s*(?:#\[[^\n]*\]\s*)*$')


def split_owned(tree, rel, test=False):
    text = rd(tree + "/" + rel)
    out = [(rel, test)]
    d = os.path.dirname(rel)
    for m in re.finditer(r'((?:#\[[^\n]*\]\s*)*)#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
        attrs = m.group(1)
        child_test = test or ("cfg(test)" in attrs or 'cfg(any(test, feature = "testkit"))' in attrs)
        out += split_owned(tree, os.path.normpath(os.path.join(d, m.group(2))), child_test)
    return out


def roots_used(text):
    ids = code_idents(text)
    ids2 = set(re.findall(r"\b(protocol|ui_contract|semio_framework_\w+|serde_json|serde|fnv)\s*::", re.sub(r"//[^\n]*", "", text)))
    used = set()
    for n in ids2 | ids:
        if n in EXT or n in FIXED:
            used.add(n)
    if {"Serialize", "Deserialize"} & ids:
        used.add("serde")
    return used


def key_of_ident(i):
    return next(k for k in CRATES if CRATES[k]["ident"] == i)


def infer(tree, k, root_rel):
    lib, tst = set(), set()
    sib_lib, sib_tst = set(), set()
    for f, is_test in split_owned(tree, root_rel):
        t = rd(tree + "/" + f)
        u = roots_used(t)
        sibs = {key_of_ident(m) for m in re.findall(r"\b(" + SIB + r"\w+)\b", t) if re.fullmatch(SIB + r"\w+", m) and m in {c["ident"] for c in CRATES.values()}}
        (tst if is_test else lib).update(u)
        (sib_tst if is_test else sib_lib).update(sibs)
    return lib, tst - lib, sib_lib, sib_tst - sib_lib


def testkit_dev(tree, k, root_rel):
    return None


def pkg_dir(k):
    return CRATES[k]["node"] + "/📦️packages/🦀️rust"


def rel_dep(k, target_rel):
    return os.path.relpath(FW + "/" + "🔨️modules/🖱️ui/🎬️scene/" + pkg_dir(k), FW).replace("\\", "/") and os.path.relpath(target_rel, MODS + "/🖱️ui/🎬️scene/" + pkg_dir(k))


KITS = {"mesh", "camera"}
DESC = {
    "algebra": "Synchronous Vec3/Mat4 algebra shared by the scene math crates — wasm-safe, depends only on the geometry types",
    "camera": "Orbit and parallel cameras, projection matrices, framing, gumball and ray math and the level-of-detail grid — wasm-safe scene camera math",
    "mesh": "Paged mesh authority, native resource custody and instance records for the scene math — wasm-safe, uses the replication retirement protocol",
    "pass": "Scene draw descriptors, materials, lighting, shadow pass, line and grid draws and frustum culling — wasm-safe",
    "picking": "Ray, marquee and component screen picking over leased meshes — wasm-safe",
    "pack": "Self-contained binary serde codec for SceneDoc payloads — wasm-safe, depends only on serde",
    "doc": "The SceneDoc contract, lane carriers and references, pack wire macro and value-codec primitives shared by every product scene — wasm-safe",
    "modelling": "Domain-neutral world-3d modelling primitives: annotations, scalar fields, pick filter, section plane and highlight tokens — wasm-safe",
    "world3d": "The world-3d scene payload with its presentation, typed lanes, modelling builders and world snapshot lease — wasm-safe",
    "node-graph": "The node-graph scene payload with its port, node, edge, operator and hover records — wasm-safe",
    "plane": "The canvas, paint, board, tiled-map and ink-canvas scene payloads with the canvas framing and snapshot lease — wasm-safe",
    "views": "The text-editor, table, block-list, diff-view, event-feed, graph-timeline, virtual-file-system and icon-render scene payloads with the draft text splice — wasm-safe",
    "surface": "The encode/decode bridge between a typed SceneDoc payload and ui_contract's SurfaceProps.doc — wasm-safe",
}


def manifest(tree, k):
    root_rel = CRATES[k]["node"] + "/🦀️.rs"
    lib, tst, sib_lib, sib_tst = infer(tree, k, root_rel)
    pkgd = MODS + "/🖱️ui/🎬️scene/" + pkg_dir(k)
    ups = len(pkgd.split("/")) - 1
    deps, devs = [], []
    def ext_line(n):
        if n in EXT:
            return n, EXT[n][1] % os.path.relpath(EXT[n][0], pkgd)
        return n, FIXED[n]
    for n in sorted(lib):
        deps.append(ext_line(n))
    for sk in sorted(sib_lib):
        if sk == k:
            continue
        deps.append((CRATES[sk]["pkg"], '%s = { path = "%s" }' % (CRATES[sk]["pkg"], os.path.relpath(MODS + "/🖱️ui/🎬️scene/" + pkg_dir(sk), pkgd))))
    for n in sorted(tst):
        devs.append(ext_line(n))
    kit_users = set()
    for f, is_test in split_owned(tree, root_rel):
        for m in re.finditer(r"(" + SIB + r"\w+)::testkit", rd(tree + "/" + f)):
            kit_users.add(key_of_ident(m.group(1)))
    for sk in sorted(sib_tst | kit_users):
        if sk == k:
            continue
        feat = ', features = ["testkit"]' if sk in KITS else ""
        devs.append((CRATES[sk]["pkg"], '%s = { path = "%s"%s }' % (CRATES[sk]["pkg"], os.path.relpath(MODS + "/🖱️ui/🎬️scene/" + pkg_dir(sk), pkgd), feat)))
    ups = len(pkgd.split("/")) - 1
    workspace = "/".join([".."] * ups)
    deps.sort(key=lambda x: x[1].split(' ')[0].lower())
    devs.sort(key=lambda x: x[1].split(' ')[0].lower())
    t = "[package]\nworkspace = \"%s\"\nname = \"%s\"\nversion = \"0.1.0\"\nedition = \"2021\"\nrust-version.workspace = true\ndescription = \"%s\"\n\n" % (workspace, CRATES[k]["pkg"], DESC[k])
    t += "[package.metadata.semio]\nrole = \"framework\"\nid = \"ui-scene-%s\"\n\n[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true\n\n[lib]\npath = \"../../🦀️.rs\"\n\n" % k
    if k in KITS:
        t += "[features]\ntestkit = []\n\n"
    t += "[dependencies]\n" + "".join(l + "\n" for _, l in deps) + "\n[dev-dependencies]\n" + "".join(l + "\n" for _, l in devs)
    return t, deps, devs, workspace


PROJECT = '{\n  "name": "@semio-tech/ui-scene-%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n'


def write_meta(tree):
    summary = {}
    for k in CRATES:
        t, deps, devs, ws = manifest(tree, k)
        wr(tree, pkg_dir(k) + "/Cargo.toml", t)
        wr(tree, pkg_dir(k) + "/📋️project.json", PROJECT % k)
        summary[k] = (ws, [d for d, _ in deps], [d for d, _ in devs])
    return summary


if __name__ == "__main__":
    s = write_meta(NEW)
    for k, v in s.items():
        print(k, v[0].count(".."), "deps", v[1], "dev", v[2])


def write_facade(tree):
    pk = "📦️packages/🦀️rust"
    lines = []
    for k in sorted(CRATES, key=lambda x: CRATES[x]["pkg"]):
        lines.append('%s = { path = "../../%s/📦️packages/🦀️rust" }' % (CRATES[k]["pkg"], CRATES[k]["node"]))
    t = '''[package]
workspace = "../../../../.."
name = "semio-framework-ui-scene"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Thin facade over the ui-scene producer crates: product scene payloads (World3dScene/TableScene/...), their pack-encoded SurfaceDoc bridge and the generic 3D scene math, re-exported name for name — wasm-safe"

[package.metadata.semio]
role = "framework"
id = "ui-scene"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
''' + "\n".join(lines) + '''

[dev-dependencies]
serde_json = { version = "1.0.140" }
'''
    wr(tree, pk + "/Cargo.toml", t)
    s = rd(tree + "/" + pk + "/📜️script.ts")
    old = '''    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--all-features", ...rest] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation);'''
    assert old in s
    s = s.replace(old, '''    await runRepositoryCargoTests(SCENE_PACKAGES, this.repoRoot, this.invocation, ["--all-features", ...rest]);''')
    s = s.replace('import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";\n', 'import { runRepositoryCargoTests } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";\n')
    s = s.replace('import { readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";\n', '')
    pk_names = ["semio-framework-ui-scene-algebra", "semio-framework-ui-scene-camera", "semio-framework-ui-scene-mesh", "semio-framework-ui-scene-pass", "semio-framework-ui-scene-picking", "semio-framework-ui-scene-pack", "semio-framework-ui-scene-doc", "semio-framework-ui-scene-modelling", "semio-framework-ui-scene-world3d", "semio-framework-ui-scene-node-graph", "semio-framework-ui-scene-plane", "semio-framework-ui-scene-views", "semio-framework-ui-scene-surface", "semio-framework-ui-scene"]
    marker = "//#region 🔖️test\n"
    s = s.replace(marker, "/** 🎬️ The scene crates, bottom up: the math producers, the pack codec, the doc contract, the product scenes, the surface bridge, then the facade. */\nconst SCENE_PACKAGES = [" + ", ".join('"%s"' % n for n in pk_names) + "];\n\n" + marker)
    s = s.replace("/** ⚙️ Runs the `semio-framework-ui-scene` test suite and the guest-target compile gates.", "/** ⚙️ Runs the test suites of `semio-framework-ui-scene` and the producer crates carved from it, and the guest-target compile gates.")
    wr(tree, pk + "/📜️script.ts", s)


if __name__ == "__main__" and False:
    pass
