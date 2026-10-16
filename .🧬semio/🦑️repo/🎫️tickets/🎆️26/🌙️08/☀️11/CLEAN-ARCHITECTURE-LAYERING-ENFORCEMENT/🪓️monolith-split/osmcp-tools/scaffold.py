import sys, os, re, collections, json
sys.path.insert(0, ".")
from emit import *

# original dependency specs (key -> spec text after '= ') from the facade manifest, paths relative to the OLD package dir
OLD_UPS = "../../../../../../"
DEPS = {
 "semio-framework-pack": '{ path = "%s🔨️modules/🎒️pack/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-artifact-reference": '{ workspace = true }',
 "semio-framework-dsl-record": '{ path = "%s🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-pack-json": '{ path = "%s🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-diagnostic": '{ path = "%s🔨️modules/⚠️diagnostic/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-dsl": '{ path = "%s🔨️modules/🗣️dsl/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-ui-locale": '{ path = "%s🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-schema-registry": '{ path = "%s🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-schema-composition": '{ path = "%s🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-value": '{ path = "%s🔨️modules/🌱️value/📦️packages/🦀️rust" }' % OLD_UPS,
 "semio-framework-dispatch-macros": '{ path = "%s🔨️modules/🔀️dispatch/📦️packages/🦀️rust", package = "semio-framework-dispatch-macros" }' % OLD_UPS,
 "semio-framework": '{ workspace = true }',
 "semio-framework-async": '{ workspace = true }',
 "semio-framework-schema": '{ path = "%s🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }' % OLD_UPS,
 "semio-framework-ui": '{ workspace = true, features = ["wgpu"] }',
 "schemars": '"0.8.22"',
 "serde": '{ workspace = true }',
 "serde_json": '{ workspace = true }',
 "axum": '{ version = "0.8", features = ["ws"] }',
 "tokio": '{ workspace = true, features = ["full"] }',
 "futures": '"0.3"',
 "semio-framework-os-kernel": '{ path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["sync", "ureq"] }',
 "semio-framework-os-services": '{ workspace = true }',
 "semio-framework-actor": '{ path = "%s🔨️modules/🎭️actor/📦️packages/🦀️rust", package = "semio-framework-actor" }' % OLD_UPS,
 "framework_hash": '{ path = "%s🔨️modules/🔏️hash/📦️packages/🦀️rust", package = "semio-framework-hash" }' % OLD_UPS,
}
TARGET_DEPS = {
 "semio-framework-plugin-host": '{ path = "../../../🔌️plugin/🖥️host/📦️packages/🦀️rust", package = "semio-framework-plugin-host" }',
 "semio-framework-plugin-guest-contract": '{ workspace = true }',
 "semio-framework-plugin": '{ path = "../../../🔌️plugin/📦️packages/🦀️rust", package = "semio-framework-plugin" }',
}
DEV_DEPS = {
 "semio-framework-trace": '{ workspace = true }',
 "semio-framework-os-kernel": '{ path = "../../../../📦️packages/🦀️rust", package = "semio-framework-os-kernel", features = ["mutation-testing"] }',
 "tokio-tungstenite": '"0.26"',
}

def ident_to_key(ident):
    for k in list(DEPS) + list(TARGET_DEPS) + list(DEV_DEPS):
        if k.replace("-", "_") == ident: return k
    return None

def deepen(spec, extra="../"):
    return re.sub(r'path = "(\.\./[^"]*)"', lambda m: 'path = "' + extra + m.group(1) + '"', spec)

def node_dir(lay, ci):
    n = lay.names[ci]
    if n == "runtime": return None
    return os.path.join(MCP, lay.plan.NODES[n][0])

def pkg_dir(lay, ci):
    n = lay.names[ci]
    if n == "runtime": return os.path.join(MCP, "📦️packages", "🦀️rust")
    return os.path.join(node_dir(lay, ci), "📦️packages", "🦀️rust")

def rel(from_dir, to_path):
    return os.path.relpath(to_path, from_dir)

def mount_cfgs(o):
    return re.findall(r'#\[cfg\(.*?\)\]', o.mount_cfg or "")

def crate_files(em, ci):
    return [o for o in em.lay.outs if o.crate == ci]

def lib_root(em, ci, sandbox=False):
    lay = em.lay
    nd = node_dir(lay, ci)
    outs = sorted([o for o in crate_files(em, ci) if o.kind != "integration"], key=lambda o: o.modname)
    n = lay.names[ci]
    lines = []
    desc = lay.plan.NODES[n][1]
    emoji = (lay.plan.NODES[n][0] or "🌉️")[0:1]
    lines.append(f"//! {emoji}️ {desc}")
    lines.append("")
    needs_async = any(re.search(r'#\[dyn_enum\]', u.it.text) for o in outs for u in o.units)
    if needs_async: lines.append("#![allow(async_fn_in_trait)]\n")
    uses_store = any(re.search(r'(?<![\w:])store::', u.it.text) for o in outs for u in o.units)
    if uses_store: lines.append("extern crate semio_framework_os_kernel as store;\n")
    base = nd if nd else os.path.join(MCP, "📦️packages", "🦀️rust")
    ta_users = em.alloc_users
    if not sandbox and n == "foundation" and ta_users:
        ta = os.path.normpath(MCP + "/../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs")
        lines.append('#[cfg(any(test, feature = "testkit"))]')
        lines.append(f'#[path = "{rel(base, ta)}"]')
        lines.append("pub mod test_allocation;")
        lines.append("")
    elif not sandbox and ci in ta_users:
        lines.append("#[cfg(test)]")
        lines.append("#[global_allocator]")
        lines.append("static TEST_ALLOCATION_OBSERVER: semio_framework_os_mcp_foundation::test_allocation::RequestedAllocator = semio_framework_os_mcp_foundation::test_allocation::RequestedAllocator;")
        lines.append("")
    for o in outs:
        for c in mount_cfgs(o): lines.append(c)
        lines.append(f'#[path = "{rel(base, o.path)}"]')
        lines.append(f"pub mod {o.modname};" if (n != "runtime" or sandbox or o.modname == "entrypoint") else f"mod {o.modname};")
        lines.append("")
    if n == "runtime" and not sandbox:
        lines += facade_lines(em, False)
    return "\n".join(lines).rstrip() + "\n"

def manifest(em, ci):
    lay = em.lay
    n = lay.names[ci]
    pkg = crate_pkg(lay, ci)
    deps = {}; tdeps = {}; ddeps = {}
    for r in sorted(em.ext_roots[ci]):
        k = "semio-framework-os-kernel" if r == "store" else ident_to_key(r)
        if k is None: continue
        if k in TARGET_DEPS: tdeps[k] = TARGET_DEPS[k]
        elif k in DEPS: deps[k] = DEPS[k]
    for r in sorted(em.ext_roots_test[ci]):
        k = "semio-framework-os-kernel" if r == "store" else ident_to_key(r)
        if k is None or k in deps or k in tdeps: continue
        if k in TARGET_DEPS: tdeps[k] = TARGET_DEPS[k]
        elif k in DEPS: ddeps[k] = DEPS[k]
        elif k in DEV_DEPS: ddeps[k] = DEV_DEPS[k]
    if "semio-framework-os-kernel" in deps or "semio-framework-os-kernel" in ddeps or True:
        pass
    inter = sorted(em.crate_deps[ci]); inter_dev = sorted(em.crate_deps_dev[ci] - em.crate_deps[ci])
    feats = {}
    if ci in em.testkit_users or em.testkit_forward.get(ci) or any(ci in v for v in []):
        pass
    ta_f = (n == "foundation" and bool(em.alloc_users))
    has_tk = ta_f or any(id(o) in em.testkit_outs for o in crate_files(em, ci)) or any(id(u.atom) in em.testkit_atoms for o in crate_files(em, ci) for u in o.units)
    fw = sorted(em.testkit_forward.get(ci, ()))
    lines = ["[package]", f'workspace = "{"/".join([".."] * 8)}"', f'name = "{pkg}"', 'version = "0.1.0"', 'edition = "2021"', 'rust-version.workspace = true',
             f'description = "{lay.plan.NODES[n][1]}"', "", "[package.metadata.semio]", 'role = "product"', f'id = "os-mcp-{n}"', "",
             "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
             "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "doctest = false"]
    if has_tk or fw:
        tk = [f'"{crate_pkg(lay, j)}/testkit"' for j in fw] + (['"dep:semio-framework-trace"'] if ta_f else [])
        lines += ["", "[features]", "testkit = [" + ", ".join(tk) + "]"]
    lines += ["", "[dependencies]"]
    for k, v in sorted(deps.items()): lines.append(f"{k} = {deepen(v)}")
    if ta_f: lines.append("semio-framework-trace = { workspace = true, optional = true }")
    for c in inter:
        lines.append(f'{crate_pkg(lay, c)} = {{ path = "../../../{lay.plan.NODES[lay.names[c]][0]}/📦️packages/🦀️rust" }}')
    if tdeps:
        lines += ["", "[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]"]
        for k, v in sorted(tdeps.items()): lines.append(f"{k} = {deepen(v)}")
    dev = []
    if n == "foundation" and ta_f and "semio-framework-trace" not in ddeps:
        ddeps["semio-framework-trace"] = DEV_DEPS["semio-framework-trace"]
    for k, v in sorted(ddeps.items()): dev.append(f"{k} = {deepen(v)}")
    for c in sorted(set(inter_dev) | set(x for x in inter if lay.names[x] and ci in em.testkit_users.get(x, set()) and False)):
        tk = ci in em.testkit_users.get(c, set())
        feat = ', features = ["testkit"]' if tk else ""
        dev.append(f'{crate_pkg(lay, c)} = {{ path = "../../../{lay.plan.NODES[lay.names[c]][0]}/📦️packages/🦀️rust"{feat} }}')
    for c in inter:
        if ci in em.testkit_users.get(c, set()):
            dev.append(f'{crate_pkg(lay, c)} = {{ path = "../../../{lay.plan.NODES[lay.names[c]][0]}/📦️packages/🦀️rust", features = ["testkit"] }}')
    if dev:
        lines += ["", "[dev-dependencies]"] + dev
    return "\n".join(lines) + "\n"

def project_json(em, ci):
    lay = em.lay
    n = lay.names[ci]
    return json.dumps({"name": f"@semio-tech/framework-os-mcp-{n}-rs", "tags": ["scope:framework", "language:rust"]}, indent=2, ensure_ascii=False) + "\n"

def find_pub_atom(em, name, hint=""):
    cands = [a for a in em.w.defs.get(name, []) if id(a) in em.lay.crate_of and em.out_of_atom(a) is not None and not em.lay.flags[id(a)]]
    cands = [a for a in cands if a.item.vis.startswith("pub")]
    if not cands: raise SystemExit(f"facade: no public atom {name}")
    pref = [a for a in cands if hint and hint in a.file.modpath]
    return (pref or cands)[0]

def facade_lines(em, sandbox=False):
    lay = em.lay
    out = []
    def path(a):
        to = em.out_of_atom(a)
        if sandbox: return f"crate::{crate_ident(lay, to.crate)}::{to.modname}"
        if lay.names[to.crate] == "runtime": return f"crate::{to.modname}"
        return f"{crate_ident(lay, to.crate)}::{to.modname}"
    tree = collections.OrderedDict()
    for key, names in lay.plan.FACADE.items():
        hint = key.split("::")[0]
        groups = collections.OrderedDict()
        for n in names:
            a = find_pub_atom(em, n, hint)
            groups.setdefault(path(a), []).append(n)
        tree[key] = groups
    for key, groups in tree.items():
        if key == "":
            for mp, names in groups.items(): out.append(f"pub use {mp}::{{{', '.join(sorted(set(names)))}}};")
            continue
        parts = key.split("::")
        if len(parts) == 1:
            out.append(f"pub mod {parts[0]} {{")
            for mp, names in groups.items(): out.append(f"    pub use {mp}::{{{', '.join(sorted(set(names)))}}};")
            out.append("}")
    if "workspace::remote" in tree:
        # nested module: emit inside workspace module by textual insertion
        idx = [i for i, l in enumerate(out) if l == "pub mod workspace {"][0]
        end = idx + 1
        while out[end] != "}": end += 1
        inner = ["    pub mod remote {"] + [f"        pub use {mp}::{{{', '.join(sorted(set(names)))}}};" for mp, names in tree["workspace::remote"].items()] + ["    }"]
        out[end:end] = inner
    flat = collections.defaultdict(list)
    for a in em.facade_flat.values():
        flat[path(a)].append(a.name)
    for mp, names in flat.items(): out.append(f"pub use {mp}::{{{', '.join(sorted(set(names)))}}};")
    out.append("pub use crate::entrypoint::run_mcp_entrypoint;" if not sandbox else "pub use crate::semio_framework_os_mcp::entrypoint::run_mcp_entrypoint;")
    return out
