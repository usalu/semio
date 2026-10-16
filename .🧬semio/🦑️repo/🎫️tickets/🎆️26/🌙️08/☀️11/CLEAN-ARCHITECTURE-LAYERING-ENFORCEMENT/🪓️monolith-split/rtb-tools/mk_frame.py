"""🏗️ Creates the bin `frame-tests-1` (frame core and host laws of the renderer root). usage: mk_frame.py plan|apply"""
import sys
from build import *
import inventory as INV
import manifest
import mk_bins as MB

APPLY = "apply" in sys.argv
SUFFIX = "frame-tests-1"
FOLDER = T + "/🍇️frame-tests-1"
UE = lambda d: E + "/🧪️tests/" + d + "/🦀️.rs"
MEMBERS = [
    ("mod", UE("🧾️frame-action-ledger"), "frame_action_ledger_tests", ""),
    ("mod", UE("🖱️wheel-application-point"), "wheel_application_point_tests", ""),
    ("mod", UE("⏱️frame-latency-diagnostics"), "frame_latency_diagnostics_tests", ""),
    ("mod", UE("🧭️boot-axis-parity"), "boot_axis_parity_tests", ""),
    ("mod", R + "/🪶️sqlite/🧪️tests/🦀️.rs", "native_socket_sqlite_tests", '#[cfg(not(target_arch = "wasm32"))]'),
    ("include", UE("🐕️wgpu-present-stall-watch"), None, ""),
    ("include", UE("🧊️wgpu-renderer-standalone"), None, ""),
]
HOST_GLOBS = ["use semio_framework_os_renderer_wgpu_host_base::frame_latency::*;"]
DOC = "//! 🍇️ Companion test bin of the wgpu renderer: frame core and renderer laws (1).\n//!\n//! 🧭️ The tests of this bin were mounted by the renderer root and its hubs; they are mounted from their place in the renderer tree here, and the namespace they were written against is rebuilt by the imports below."


def plan():
    X.CURRENT[0] = SUFFIX
    inv = INV.inventory()
    hosts = sorted({inv[f][0]["host"] for k, f, n, c in MEMBERS if f in inv})
    set_hosts(hosts + [R + "/🦀️.rs"] + [R + "/📥️mounts/" + h + "/🦀️.rs" for h in os.listdir(R + "/📥️mounts") if os.path.isdir(R + "/📥️mounts/" + h)])
    texts = {}
    for k, f, n, c in MEMBERS:
        texts[f] = B.transform_test_probe(read(f))
    tmp = []
    tmpdir = SCRATCH + "/frame1"
    os.makedirs(tmpdir, exist_ok=True)
    for i, (f, t) in enumerate(texts.items()):
        pth = f"{tmpdir}/t{i}.rs"
        write(pth, t)
        tmp.append(pth)
    aliases, lines, unresolved = full_imports(SUFFIX, tmp)
    imports_ = merge_import_blocks(lines, simplify=False, own=lib_name(SUFFIX)) + HOST_GLOBS
    mounts = []
    for k, f, n, c in MEMBERS:
        rel_ = os.path.relpath(f, FOLDER)
        if k == "mod":
            mounts.append((c + "\n" if c else "") + f'#[path = "{rel_}"]\nmod {n};')
        else:
            mounts.append(f'include!("{rel_}");')
    root = DOC + "\n\n#![cfg(test)]\n\n" + "\n".join(aliases + imports_) + "\n\n" + "\n".join(mounts) + "\n"
    return root, texts, unresolved


if __name__ == "__main__":
    root, texts, unresolved = plan()
    print("unresolved", unresolved, "root lines", len(root.split("\n")))
    if not APPLY:
        print(root[:3500])
    else:
        for f, t in texts.items():
            edit(f, t)
        write(FOLDER + "/🦀️.rs", root)
        idents = used_idents(root) | set().union(*[used_idents(t) for t in texts.values()])
        mt, miss = manifest.build_manifest("frame-tests-1", "frame-tests-1", "Companion test bin of the wgpu renderer: frame core and renderer laws (1)", idents, dev_keys=("semio-framework-job",))
        write(FOLDER + "/📦️packages/🦀️rust/Cargo.toml", mt)
        write(FOLDER + "/📦️packages/🦀️rust/📋️project.json", '{\n  "name": "@semio-tech/os-renderer-wgpu-frame-tests-1-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n')
        print("created", FOLDER, "miss", sorted(m for m in miss if m[0].islower()))
