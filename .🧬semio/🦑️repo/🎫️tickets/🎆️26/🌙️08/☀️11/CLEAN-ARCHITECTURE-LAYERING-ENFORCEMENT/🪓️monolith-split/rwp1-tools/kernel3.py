#!/usr/bin/env python3
# 🧠️ Kernel group carve, root step: the four crate roots with explicit imports computed from the old namespace.
import json, os, sys
from common import *
from carve import *
import needs, provide
from rs import uses
from kernel1 import K, FILES, PROT, POOLC

L1 = PROT + "/📥️lifted/📬️kernel-replay-client/🦀️.rs"
L2 = PROT + "/📥️lifted/🏊️kernel-pool/🦀️.rs"
L3 = PROT + "/📥️lifted/📦️kernel-runtime/🦀️.rs"
L4 = POOLC + "/📥️lifted/🗃️kernel-retained-state/🦀️.rs"
info = json.load(open(RS + "/kernel2.json"))
parent_text = read(FILES["parent"]) if os.path.exists(FILES["parent"]) else None
parent_entries = [e for e in uses(parent_text) if e[1] == "" and not e[4]] if parent_text else []
PAR = json.load(open(RS + "/kernel-parent-entries.json")) if not parent_entries else None
if parent_entries:
    json.dump(parent_entries, open(RS + "/kernel-parent-entries.json", "w"), ensure_ascii=False)
else:
    parent_entries = [tuple(x) for x in PAR]


def make_root(suffix, doc, mounts, files, extra_head="", tail=""):
    glob_files = [f for f in files if "use super::*" in read(f)]
    u, e, p = needs.wanted(glob_files, extra_entries=parent_entries, self_lib=lib_name(suffix))
    imports = needs.render(u, e, p)
    write_crate_root(suffix, doc, mounts, imports=(imports + "\n" + extra_head).strip("\n"), tail=tail)
    mark_carved(suffix)
    provide.build()


make_root("kernel-protocol", """//! 📡️ Kernel protocol: sequence authority, job-progress presentation, turn budget, extension index, requests and close submissions, the mounted product replay subsystem, response slots and exchange outcomes, the request queue, the parallel runtime and component page reading.
//!
//! 🧭️ Native only: the bottom of the actor-kernel group. The module files stay in the renderer tree; this crate root mounts them and re-exports their surface.""",
    [("kernel_core", FILES["core"], "mod", None), ("replay_vocabulary", L1, "mod", None), ("request_queue", L2, "mod", None), ("native_component", L3, "mod", None), ("parallel_runtime", T + "/🎠️runtime/🦀️.rs", "pub mod", None)],
    [FILES["core"], L1, L2, L3, T + "/🎠️runtime/🦀️.rs"])
make_root("kernel-retained", """//! 🗃️ Kernel retained state: retained document exchange and surfaces, pending rejections, mounted replay routes and command document retirement.
//!
//! 🧭️ Native only. The module file stays in the renderer tree; this crate root mounts it and re-exports its surface.""",
    [("kernel_retained_state", FILES["retained"], "mod", None)], [FILES["retained"]])
make_root("kernel-pool", """//! 🏊️ Kernel pool: the kernel pool state and its implementation, the pool runner and the kernel client.
//!
//! 🧭️ Native only. The module files stay in the renderer tree; this crate root mounts them and re-exports their surface.""",
    [("kernel_pool", FILES["pool"], "mod", None), ("kernel_client", FILES["replay"], "mod", None), ("pool_state", L4, "mod", None)], [FILES["pool"], FILES["replay"], L4])

# ---- facade crate root
E_ = lambda s: s
facade_doc = info["parent_doc"].replace("//#region 🎠️KernelRuntime\n", "").rstrip("\n") + "\n//!\n//! 🧭️ Native only. The facade of the four kernel crates plus the scale-bench scenario driver; it re-exports exactly the surface the renderer consumers name."
pub_uses = """pub use semio_framework_os_renderer_wgpu_host_base::KernelPoolFuture;
pub use semio_framework_os_renderer_wgpu_kernel_pool::KernelClient;
pub use semio_framework_os_renderer_wgpu_kernel_protocol::{read_native_component, take_job_progress_presentation, ExchangeOutcome, JobProgressPresentationLease, KernelCloseHandle, KernelCloseStatus, MountedProductReplayAdmission};
#[cfg(test)]
pub use semio_framework_os_renderer_wgpu_kernel_protocol::CANCELLED_KERNEL_TURNS;"""
test_globs = """#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_pool::*;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_protocol::*;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_retained::*;"""
scale = R + "/📊️scale-bench/🦀️.rs"
sem = info["sem"]
pub_names = {"KernelPoolFuture", "KernelClient", "read_native_component", "take_job_progress_presentation", "ExchangeOutcome", "JobProgressPresentationLease", "KernelCloseHandle", "KernelCloseStatus", "MountedProductReplayAdmission", "CANCELLED_KERNEL_TURNS"}
u, e, p = needs.wanted([scale], extra_entries=parent_entries, self_lib=lib_name("kernel-runtime"), skip=pub_names)
imports = needs.render(u, e, p)
ut, et, pt = needs.wanted([sem], extra_entries=parent_entries, self_lib=lib_name("kernel-runtime"), skip=pub_names | {n for s in u.values() for n in s} | {n for s in p.values() for n in s})
test_imports = "\n".join("#[cfg(test)]\n" + block.replace("\n#[cfg", "\n#[cfg") for block in needs.render(ut, et, pt).split("\n") if block.strip() and not block.startswith("#[cfg"))
folder = T + "/" + CRATES["kernel-runtime"]
text = facade_doc + "\n\n" + pub_uses + "\n\n" + imports + "\n\n" + test_imports + "\n\n" + test_globs + "\n\n"
text += f'#[path = "{relpath(folder, scale)}"]\npub mod scale_bench;\n\n'
text += f'#[cfg(test)]\ninclude!("{relpath(folder, sem)}");\n\n'
text += '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
write(folder + "/🦀️.rs", text)
mark_carved("kernel-runtime")
provide.build()
