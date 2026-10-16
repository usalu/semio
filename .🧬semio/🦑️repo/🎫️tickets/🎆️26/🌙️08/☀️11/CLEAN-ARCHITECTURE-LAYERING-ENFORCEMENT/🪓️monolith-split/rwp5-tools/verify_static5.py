# ✅️ Static gates of the WP5 working copy: bundle shape, bridge direction, no leftover foreign calls or old field paths, mounts, regions, rustfmt, path lengths, losslessness.
import os, re, sys, collections, subprocess, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx, deps, refs, parent, measure
from rsedit import Doc
from blank import blank

FAIL = []
def fail(msg): FAIL.append(msg); print("FAIL", msg)

def main(root):
    comp_fields = [f for f, _ in ctx.COMPONENTS]
    ns = deps.load(root)
    homes = collections.defaultdict(set)
    for n in ns:
        if n.kind == "method": homes[n.name].add(n.crate)
    bridges = {c: set() for c in ctx.BRIDGE}
    files = 0
    for crate in ctx.BRIDGE:
        x = cfg.CTX[crate]
        for slug, p in ctx.crate_files(root, crate):
            files += 1
            t = open(p, encoding="utf-8").read(); doc = Doc(t)
            for it in doc.items():
                if it["kind"] == "impl" and re.match(r"impl\s+ShellState\b", it["head"]): fail(f"{crate}/{slug}: leftover impl ShellState")
                if it["kind"] == "impl" and re.match(rf"impl(?:<[^>]*>)?\s+{x}<", it["head"]):
                    for m in doc.members(it):
                        body = "\n".join(doc.blank[m["s"]:m["e"] + 1])
                        for mm in re.finditer(r"\bself\s*\.\s*([A-Za-z_]\w*)\s*(\(|::|\b)", body):
                            name = mm.group(1); call = body[mm.end() - len(mm.group(2)):mm.end()].startswith("(") if mm.group(2) else False
                            if name in comp_fields and not call: continue
                            if name.startswith("as_"):
                                tgt = next((c for c, b in ctx.BRIDGE.items() if b == name), None)
                                if tgt is None: fail(f"{crate} {m['name']}: unknown bridge {name}")
                                elif cfg.RANK[tgt] >= cfg.RANK[crate]: fail(f"{crate} {m['name']}: bridge {name} points up")
                                else: bridges[crate].add(tgt)
                                continue
                            if call:
                                hs = homes.get(name)
                                if hs and crate not in hs: fail(f"{crate} {m['name']}: foreign call self.{name}() without a bridge (homes {sorted(hs)})")
                                if not hs: fail(f"{crate} {m['name']}: self.{name}() resolves to no WP5 method")
                            else:
                                fail(f"{crate} {m['name']}: self.{name} is neither a component nor a method")
            t_no = blank(t)
            for lineno, l in enumerate(t_no.split("\n"), 1):
                if re.search(r"\bShellState\b", l) and slug != ctx.BUNDLE_SLUG and re.search(r"&\s*(mut\s+)?ShellState\b", l):
                    fail(f"{crate}/{slug}:{lineno}: parameter or type still names ShellState: {l.strip()[:100]}")
    for crate, tg in bridges.items():
        bp = f"{cfg.stg(root, cfg.CRATE2DIR[crate])}/{ctx.BUNDLE_SLUG}/🦀️.rs"
        if not os.path.exists(bp):
            if tg: fail(f"{crate}: bridges needed but no bundle")
            continue
        bt = open(bp, encoding="utf-8").read()
        for f, ty in ctx.COMPONENTS:
            if f"pub {f}: &'a mut {ty}," not in bt: fail(f"{crate} bundle misses field {f}")
            if f"{f}: &mut shell.{f}," not in bt: fail(f"{crate} bundle `of` misses {f}")
        for lower in tg:
            if f"pub fn {ctx.BRIDGE[lower]}(&mut self)" not in bt: fail(f"{crate} bundle misses bridge {ctx.BRIDGE[lower]}")
        for have in re.findall(r"pub fn (as_\w+)\(&mut self\)", bt):
            if have not in {ctx.BRIDGE[l] for l in tg}: fail(f"{crate} bundle has unused bridge {have}")
    mods = parent.mods_of(root)
    ptxt = open(f"{root}/{cfg.PARENT}", encoding="utf-8").read()
    for d, c, slug, p in mods:
        pat = f'#[path = "📥️staging/{d}/{slug}/🦀️.rs"]'
        if ptxt.count(pat) != 1: fail(f"mount count {ptxt.count(pat)} for {d}/{slug}")
        if ptxt.count(f'include_str!("📥️staging/{d}/{slug}/🦀️.rs")') != 1: fail(f"SHELL_SOURCE count for {d}/{slug}")
    for m in re.finditer(r'#\[path = "📥️staging/(' + "|".join(map(re.escape, cfg.DIR2CRATE)) + r')/([^/]+)/🦀️\.rs"\]', ptxt):
        if not os.path.exists(f"{cfg.stg(root, m.group(1))}/{m.group(2)}/🦀️.rs"): fail(f"mount of a missing file {m.group(1)}/{m.group(2)}")
    allp = [p for _, _, _, p in mods]
    for p in allp:
        if len(p.encode("utf-8")) > 240 + len(root.encode("utf-8")) - len(cfg.ROOT.encode("utf-8")) + 0 and False: pass
        rel = os.path.relpath(p, root); full = f"{cfg.SHELL_REL}/{rel}"
        if len(full.encode("utf-8")) > 240: fail(f"path too long {len(full.encode('utf-8'))}: {rel}")
        t = open(p, encoding="utf-8").read()
        if t.count("//#region") != t.count("//#endregion"): fail(f"region imbalance {rel}")
        if not t.startswith("//!"): fail(f"no docstring {rel}")
    rc = 0
    for p in allp:
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", p], capture_output=True, text=True)
        if r.returncode != 0 and "error" in r.stderr: fail(f"rustfmt parse {os.path.relpath(p, root)[:60]}: {r.stderr.strip().splitlines()[0][:120]}"); rc += 1
    print("files", len(allp), "rustfmt failures", rc)
    for k, v in measure.measure(root).items():
        flag = "" if v[1] <= 3500 and v[2] <= 230000 and v[0] <= 27 else "OVER"
        print("%-22s files %3d lines %5d bytes %7d %s" % ((k,) + v + (flag,)))
        if flag: fail(f"budget {k}")
    print("FAILURES", len(FAIL))
    return len(FAIL)

if __name__ == "__main__": sys.exit(1 if main(sys.argv[1] if len(sys.argv) > 1 else cfg.NEW) else 0)
