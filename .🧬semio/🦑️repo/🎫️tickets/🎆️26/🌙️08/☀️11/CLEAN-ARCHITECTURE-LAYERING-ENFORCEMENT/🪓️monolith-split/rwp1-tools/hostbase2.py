#!/usr/bin/env python3
# 🏛️ Step 2 of the host-base carve (applies to the repo): lifts renderer-io and KernelPoolFuture, writes the host-base crate root, slims the renderer root.
import json, os, re, sys
from common import *
from carve import *
import items
import needs
from rs import blank

DRY = "--dry" in sys.argv
HB = T + "/" + CRATES["host-base"]
ROOT = R + "/🦀️.rs"
DECODE = R + "/🔁️asset-decode-jobs/🦀️.rs"
POOL = R + "/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs"
KRT = R + "/📦️kernel-runtime/🦀️.rs"


def rebase(text, old_dir, new_dir):
    def sub(m):
        lit = m.group(2)
        if os.path.isabs(lit):
            return m.group(0)
        return m.group(1) + os.path.relpath(os.path.normpath(os.path.join(old_dir, lit)), new_dir) + '"'
    return re.sub(r'(#\[\s*path\s*=\s*")([^"]+)"', lambda m: sub(m), text)


def lines_between(text, start_marker, end_marker):
    L = text.split("\n")
    s = [i for i, l in enumerate(L) if l.startswith(start_marker)]
    e = [i for i, l in enumerate(L) if l.startswith(end_marker)]
    assert len(s) == 1 and len(e) == 1 and s[0] < e[0], (start_marker, s, e)
    return s[0], e[0]


# ---- 1. renderer-io region out of the decode jobs
dtext = read(DECODE)
s, e = lines_between(dtext, "//#region 🧵️RendererWorkerPool", "//#endregion 🧵️RendererWorkerPool")
DL = dtext.split("\n")
io_region = "\n".join(DL[s:e + 1]) + "\n"
d_new = "\n".join(DL[:s] + DL[e + 1:])
d_new = re.sub(r"\n{3,}$", "\n", d_new)
io_dir = HB + "/📥️lifted/🔁️asset-decode-jobs"
io_text = "//! 🧵️ The renderer worker pool and its bounded native I/O slots: one `RendererIoHandle` per submitted request, the slot ledger and the pump the winit thread and the kernel workers drive.\n//!\n//! 🧭️ Lifted out of the asset decode jobs into host-base: the decode jobs are one client of this transport, the kernel and the frame loop are others.\nuse super::*;\n\n" + rebase(io_region, os.path.dirname(DECODE), io_dir)
io_text = io_text.replace("pub(crate)", "pub").replace("pub(super)", "pub")
io_text = re.sub(r"(?m)^((?:async |const |unsafe )*(?:fn|struct|enum|const|static|type|trait) )", r"pub \1", io_text)

# ---- 2. KernelPoolFuture out of the kernel pool
ptext = read(POOL)
tb = items.table(ptext)
kf = [x for x in tb if x["name"] == "KernelPoolFuture" and x["kind"] in ("struct",)]
assert len(kf) == 1
ks = kf[0]["start"]
wk = [x for x in tb if x["kind"] == "impl" and x["name"].startswith("impl std::task::Wake for KernelPoolFuture")]
assert len(wk) == 1
ke = wk[0]["end"]
PL = ptext.split("\n")
kpf_block = "\n".join(PL[ks - 1:ke]) + "\n"
p_new = "\n".join(PL[:ks - 1] + PL[ke:])
p_new = p_new.replace("\n\n\n", "\n\n")
kpf_dir = HB + "/📥️lifted/🏊️kernel-pool"
kpf_text0 = "//! 🏊️ One kernel-pool task: a boxed future driven turn by turn on a worker-pool lane, rescheduled by its waker.\n//!\n//! 🧭️ Lifted out of the kernel pool into host-base: the renderer root spawns application tasks with it before any kernel crate exists.\nuse std::future::Future;\nuse std::pin::Pin;\nuse std::sync::{Arc, Mutex};\nuse std::task::{Context, Waker};\n\n" + kpf_block.replace("pub(crate)", "pub").replace("pub(super)", "pub")
kpf_text = re.sub(r"(?m)^((?:async |const |unsafe )*(?:fn|struct|enum|const|static|type|trait) )", r"pub \1", kpf_text0)

# ---- 3. renderer root split
moved = read(RS + "/hostbase-moved.rs")
wrappers = read(RS + "/hostbase-wrappers.rs")
rtext = read(ROOT)
RL = rtext.split("\n")
rs_i = [i for i, l in enumerate(RL) if l.startswith("//#region 🔖️ActorBudgetBridge")][0]
be_i = [i for i, l in enumerate(RL) if l.startswith("//#endregion 🧭️BootDescriptor")][0]
check_moved = "\n".join(RL[rs_i:be_i + 1]) + "\n"
assert len(check_moved.split("\n")) > len(moved.split("\n")), "moved text must be a subset"

# visibility widening inside the moved root text
def widen_fns(text):
    out = []
    for l in text.split("\n"):
        m = re.match(r"^((?:async |const )*)fn ", l)
        if m and not l.startswith("pub"):
            l = "pub " + l
        l = l.replace("pub(crate) ", "pub ", 1) if l.startswith("pub(crate) ") else l
        out.append(l)
    return "\n".join(out)

moved_pub = widen_fns(moved)
assert moved_pub.count("kernel_runtime::KernelPoolFuture::spawn") == 1
moved_pub = moved_pub.replace("kernel_runtime::KernelPoolFuture::spawn", "KernelPoolFuture::spawn")

# ---- names exported by host-base
def exports(text, extra_attrs=()):
    res = {}
    for it in items.table(text):
        if it["kind"] in ("impl", "macro_rules!") or it.get("thread_local"):
            continue
        if it["name"] in ("compiled_host_platform",):
            continue
        res.setdefault(it["name"], []).append(it)
    return res

all_exports = {}
for label, text in (("root", moved_pub), ("io", io_text), ("kpf", kpf_text)):
    for name, its in exports(text).items():
        all_exports.setdefault(name, []).extend(its)

# which names does the rest of the tree mention?
mention = {}
others = []
for base, dirs, files in os.walk(E):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target")]
    for f in files:
        if f.endswith(".rs"):
            others.append(os.path.join(base, f))
import rs as rsmod
tok_cache = {}
def toks(p):
    if p not in tok_cache:
        tok_cache[p] = needs.tokens(read(p))
    return tok_cache[p]

wrapper_toks = needs.tokens(wrappers)
external = {}
for name in all_exports:
    users = [p for p in others if not p.startswith(HB) and p != ROOT + ".never" and (name in toks(p))]
    if name in wrapper_toks:
        users.append("<root wrappers>")
    external[name] = users
json.dump({k: [u.replace(E + "/", "") for u in v][:6] for k, v in external.items()}, open(RS + "/hostbase-external.json", "w"), ensure_ascii=False, indent=1)

def cfg_line(its):
    sets = [tuple(a for a in x["attrs"] if a.startswith("#[cfg(")) for x in its]
    if all(s == sets[0] for s in sets):
        return "\n".join(sets[0])
    return ""

pub_names, crate_names = [], []
for name, its in sorted(all_exports.items()):
    if not external[name]:
        continue
    vis_pub = any(x["vis"] == "pub" or x["vis"] == "" and False for x in its)
    # originally pub (not pub(crate)): check the moved text original
    orig_pub = any(re.search(r"^pub\s+(?:async\s+|const\s+)*(?:fn|struct|enum|const|static|type|trait)\s+" + re.escape(name) + r"\b", check_moved, re.M) for _ in [0])
    (pub_names if orig_pub else crate_names).append((name, cfg_line(its)))

def render_use(names, vis):
    out = []
    by_cfg = {}
    for n, c in names:
        by_cfg.setdefault(c, []).append(n)
    for c, ns in sorted(by_cfg.items()):
        body = "{" + ", ".join(sorted(ns)) + "}"
        out.append((c + "\n" if c else "") + f"{vis} use {lib_name('host-base')}::{body};")
    return "\n".join(out)

reexport = render_use(pub_names, "pub") + "\n" + render_use(crate_names, "pub(crate)") + "\n"
mod_reexport = "pub use semio_framework_os_renderer_wgpu_host_base::{action_args_json, frame_latency, mesh_assets, plugin_catalog, runtime_assets};\n"

# ---- slimmed root
head = "\n".join(RL[:rs_i])
tail = "\n".join(RL[be_i + 1:])
# wrapper regions
wrap_by_region = {
    "semioWgpuSetHostAppearance": "🌓️HostAppearance",
    "semioWgpuSetHostIntroductionSuppressed": "🎓️HostIntroduction",
    "semioWgpuSetHostPlatform": "⌨️HostPlatform",
    "semioWgpuSetHostLocale": "🗣️HostLocale",
}
wtext = ""
for key, region in wrap_by_region.items():
    m = re.search(r"((?:///[^\n]*\n)*)#\[cfg\(target_arch = \"wasm32\"\)\]\n#\[wasm_bindgen\(js_name = " + key + r"\)\]\n[^\n]*\{\n(?:[^\n]*\n)*?\}\n", wrappers)
    assert m, key
    wtext += f"//#region {region}\n{m.group(0)}//#endregion {region}\n\n"
bm = wrappers[wrappers.index("/// 🌐️ wasm boot hook"):]
wtext += f"//#region 🧭️BootDescriptor\n{bm}//#endregion 🧭️BootDescriptor\n"
new_root = head + "\n" + mod_reexport + reexport + "\n" + wtext + "\n" + tail
if not DRY:
    pass

src_anchor = '    include_str!("🔁️asset-decode-jobs/🦀️.rs"),\n'
assert new_root.count(src_anchor) == 1
new_root = new_root.replace(src_anchor, src_anchor + '    include_str!("../🏛️host-base/🦀️.rs"),\n    include_str!("../🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs"),\n    include_str!("../🏛️host-base/📥️lifted/🏊️kernel-pool/🦀️.rs"),\n')

# ---- host-base root file
mount_modules = '''
#[path = "../⏱️frame-latency/🦀️.rs"]
pub mod frame_latency;

#[path = "../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🦀️.rs"]
pub mod mesh_assets;

#[path = "../🔤️runtime-assets/🦀️.rs"]
pub mod runtime_assets;

#[path = "../📇️catalog/🦀️.rs"]
pub mod plugin_catalog;
pub use plugin_catalog::{install_plugin_catalog_json_v1, install_plugin_catalog_v1};

#[cfg(not(target_os = "wasi"))]
#[path = "../🧊️renderer/📇️registry/🦀️.rs"]
mod registration;

#[path = "📥️lifted/🔁️asset-decode-jobs/🦀️.rs"]
mod renderer_io;
pub use renderer_io::*;

#[path = "📥️lifted/🏊️kernel-pool/🦀️.rs"]
mod kernel_pool_future;
pub use kernel_pool_future::KernelPoolFuture;
'''
imp_files_text = moved_pub
tmp = RS + "/hostbase-imports-input.rs"
write(tmp, moved_pub + "\n" + io_text + "\n" + kpf_text)
ul, el, pl = needs.wanted([tmp], skip={"Theme"} if False else ())
imports = needs.render(ul, el)
doc = "//! 🏛️ Host statics, boot descriptor, logging, plugin catalog, runtime assets, mesh assets, frame latency and renderer I/O of the wgpu renderer.\n//!\n//! 🧭️ Native and wasm32. The dissolved root of the renderer crate (host appearance, platform and locale, boot descriptor, logging, application task spawning) plus the leaf modules every other crate of the partition stands on.\n"
hb_root = doc + "\n" + imports + "\n#[cfg(target_arch = \"wasm32\")]\nuse wasm_bindgen::prelude::*;\n\n" + moved_pub + "\n" + mount_modules + '\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
write(RS + "/hostbase-root.rs", hb_root)
write(RS + "/hostbase-newroot.rs", new_root)
write(RS + "/hostbase-io.rs", io_text)
write(RS + "/hostbase-kpf.rs", kpf_text)
write(RS + "/hostbase-decode-new.rs", d_new)
write(RS + "/hostbase-pool-new.rs", p_new)
print("pub", [n for n, _ in pub_names])
print("crate", [n for n, _ in crate_names])
if DRY:
    sys.exit(0)
# ---- apply
snapshot_check = {ROOT: rtext, DECODE: dtext, POOL: ptext}
for p, t0 in snapshot_check.items():
    assert read(p) == t0, ("changed under me", p)
write(io_dir + "/🦀️.rs", io_text)
write(kpf_dir + "/🦀️.rs", kpf_text)
write(HB + "/🦀️.rs", hb_root)
write(DECODE, d_new)
write(POOL, p_new)
write(ROOT, new_root)

# ---- kernel runtime parent, hub, library root
edit(KRT, '#[path = "🧮️kernel-core/🦀️.rs"]\nmod kernel_core;', 'pub(crate) use semio_framework_os_renderer_wgpu_host_base::KernelPoolFuture;\n\n#[path = "🧮️kernel-core/🦀️.rs"]\nmod kernel_core;')
edit(HUB, '#[path = "../../../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🦀️.rs"]\npub mod mesh_assets;\n', "")
edit(HUB, '#[path = "../../../🔤️runtime-assets/🦀️.rs"]\npub mod runtime_assets;\n', "")
edit(HUB, '#[path="../../../📇️catalog/🦀️.rs"]\npub mod plugin_catalog;\n', "")
edit(HUB, '#[path = "../../../⏱️frame-latency/🦀️.rs"]\npub(crate) mod frame_latency;\n', "")
LIB = T + "/📦️packages/🦀️rust/📚️library/🦀️.rs"
edit(LIB, '#[cfg(not(target_os = "wasi"))]\n#[path = "../../../🧊️renderer/📇️registry/🦀️.rs"]\nmod renderer_registration;\n\n', "")
mark_carved("host-base")
