import sys, os, re, json, subprocess, collections, shutil
sys.path.insert(0, "."); sys.path.insert(0, "../hostmcp-tools")
import sbx
from scaffold import *

SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
OS = "🧰️framework/🛍️products/💻️os/"
MCPREL = OS + "🔨️modules/🌉️mcp"
SNAP = SCR + "orig/🌉️mcp"

def generate(planmod, sandbox, with_tests=False):
    lay = Layout(planmod)
    em = Emitter(lay); em.sandbox = sandbox
    em.run()
    files = {}
    for o in lay.outs:
        files[o.path] = build_file(em, o)
    for ci in range(len(lay.names)):
        n = lay.names[ci]
        nd = node_dir(lay, ci)
        root = os.path.join(nd, "🦀️.rs") if nd else os.path.join(MCP, "📦️packages", "🦀️rust", "🦀️.rs")
        files[root] = lib_root(em, ci, sandbox)
    return lay, em, files

def union_root(lay):
    lines = []
    for ci, n in enumerate(lay.names):
        nd = node_dir(lay, ci)
        p = os.path.join(nd, "🦀️.rs") if nd else os.path.join(MCP, "📦️packages", "🦀️rust", "🦀️.rs")
        lines.append(f'#[path = "{os.path.relpath(p, MCP)}"]')
        lines.append(f"pub mod {crate_ident(lay, ci)};")
    for oo in lay.outs:
        if oo.kind == "integration":
            lines += ["#[cfg(test)]", f'#[path = "{os.path.relpath(oo.path, MCP)}"]', f"pub mod {oo.modname};"]
    ta = os.path.normpath(MCP + "/../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs")
    lines += ["#[cfg(test)]", f'#[path = "{os.path.relpath(ta, MCP)}"]', "pub(crate) mod test_allocation;", "#[cfg(test)]", "#[global_allocator]", "static TEST_ALLOCATION_OBSERVER: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;"]
    return "\n".join(lines) + "\n"

def build_split(lay, em, files):
    root = SCR + "sbx-split"
    sbx.mk(root, {MCPREL: SNAP})
    mcp = root + "/" + MCPREL
    for p, text in files.items():
        dst = mcp + p[len(MCP):]
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        open(dst, "w", encoding="utf-8").write(text)
    open(mcp + "/union_root.rs", "w", encoding="utf-8").write(union_root(lay))
    return root

def build_orig():
    root = SCR + "sbx-orig"
    sbx.mk(root, {MCPREL: SNAP})
    return root

CFGS = {"native": [], "test": ["--test"], "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"], "wasm": ["--target", "wasm32-unknown-unknown"], "p2": ["--target", "wasm32-wasip2"]}

def run(root, libpath, cfg):
    lib = root + "/" + MCPREL + "/" + libpath
    p = subprocess.run(["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "semio_framework_os_mcp", "--emit=metadata", "-o", "/dev/null", "--error-format=json", "--json=diagnostic-short", *CFGS[cfg], lib], capture_output=True, text=True, cwd=os.path.dirname(lib))
    return p.stderr

def norm(stderr):
    out = collections.Counter(); detail = collections.defaultdict(list)
    for ln in stderr.splitlines():
        if not ln.startswith("{"): continue
        try: d = json.loads(ln)
        except Exception: continue
        if "message" not in d or "level" not in d: continue
        lvl = d["level"]
        if lvl not in ("error", "warning"): continue
        spans = [s for s in d.get("spans", []) if s.get("is_primary")]
        txt = ""; lab = ""; f = ""; line = 0
        if spans:
            s = spans[0]
            txt = " ".join(" ".join(t["text"] for t in s.get("text", [])).split())
            lab = s.get("label") or ""; f = s["file_name"]; line = s["line_start"]
        code = (d.get("code") or {}).get("code") or ""
        msg = re.sub(r'\s+', ' ', d["message"])
        mm = re.search(r"unlinked crate `(\w+)`", lab) or re.search(r"unresolved import `(\w+)`$", msg)
        if mm and not mm.group(1).startswith("semio_framework_os_mcp") and mm.group(1) != "crate": continue
        if re.search(r"^cannot find .* in the crate root$", msg) or msg.startswith("aborting") or msg.startswith("couldn't find"): 
            if "semio_framework_os_mcp" not in msg: continue
        key = (lvl, code, msg, lab)
        out[key] += 1; detail[key].append((f, line, txt))
    return out, detail

def main(planmod, cfgs):
    lay, em, files = generate(planmod, True)
    for pair in [x for x in os.environ.get("NEG", "").split("||") if x]:
        old, new = pair.split("=>")
        for p in list(files):
            if old in files[p]:
                files[p] = files[p].replace(old, new, 1); print("NEG patched", os.path.relpath(p, MCP)[:80]); break
    ro = build_orig(); rs = build_split(lay, em, files)
    for r, rel in ((ro, ""), (rs, "")):
        fl = sbx.rsfiles(r + "/" + MCPREL); mods = sbx.modnames(fl); sbx.neutralize(fl, mods)
    bad = 0
    for cfg in cfgs:
        eo = run(ro, "📦️packages/🦀️rust/🦀️.rs", cfg); es = run(rs, "union_root.rs", cfg)
        open(SCR + f"o-{cfg}.json", "w").write(eo); open(SCR + f"s-{cfg}.json", "w").write(es)
        a, da = norm(eo); b, db = norm(es)
        ea = sum(v for k, v in a.items() if k[0] == "error"); eb = sum(v for k, v in b.items() if k[0] == "error")
        wa = sum(v for k, v in a.items() if k[0] == "warning"); wb = sum(v for k, v in b.items() if k[0] == "warning")
        only_o = a - b; only_s = b - a
        print(f"== {cfg}: orig errors={ea} warn={wa} | split errors={eb} warn={wb} | only-orig={sum(only_o.values())} only-split={sum(only_s.values())}")
        for k, v in sorted(only_o.items()): print("  -orig", v, k[0], k[1], k[2][:120], "|", os.path.basename(da[k][0][0]), da[k][0][1], da[k][0][2][:70])
        for k, v in sorted(only_s.items()): print("  +split", v, k[0], k[1], k[2][:120], "|", os.path.relpath(db[k][0][0], SCR + "sbx-split/" + MCPREL)[:70], db[k][0][1], db[k][0][2][:90])
        bad += sum(only_o.values()) + sum(only_s.values())
    print("TOTAL DIFF", bad)

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2:] or ["native"])
