# 🏭️ RENDERER-WP5 build: frozen snapshot of the Shell element -> spill moves -> bundle conversion -> reference rewrites -> parent mounts -> consumers (tests in the element, overlay for other packages' files).
import os, sys, shutil, subprocess, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, moves, ctx, refs, parent, special, ext, measure, deps, cleanup
from mover import read, write

def main():
    report = []
    subprocess.check_call(["rsync", "-a", "--delete", cfg.ORIG + "/", cfg.NEW + "/"])
    shutil.rmtree(cfg.GEN + "/new-ext", ignore_errors=True)
    log = []; moves.run(cfg.NEW, log); report.append(f"moves: {len(log)}")
    cleanup.run(cfg.NEW, report)
    sl, man = ctx.convert(cfg.NEW, report)
    refs.rewrite_selfless(cfg.NEW, set(sl), report)
    refs.widen(cfg.NEW, report)
    tab = ext.method_table(cfg.NEW)
    cand = ext.candidates(tab); SHEL = cfg.SHELL_REL + "/"; hazards = []; changed_ext = []; n_in = n_out = 0
    for rel in cand:
        if rel.startswith(SHEL + cfg.TGT + "/"): continue
        inside = rel.startswith(SHEL)
        p = os.path.join(cfg.NEW, rel[len(SHEL):]) if inside else os.path.join(cfg.ROOT, rel)
        raw = read(p)
        out, n = ext.rewrite(raw, rel, tab, hazards, not (inside or ("/🧱️elements/" in rel and "/🧪️tests/" in rel)))
        if n:
            if inside: write(p, out); n_in += n
            else:
                q = os.path.join(cfg.GEN, "new-ext", rel); write(q, out); changed_ext.append(rel); n_out += n
    report.append(f"consumer call sites rewritten: {n_in} inside the element tests, {n_out} in {len(changed_ext)} other files")
    special.run(cfg.NEW, report)
    special.tests_late(cfg.NEW, report)
    special.boot_load(cfg.NEW, report)
    special.tests_hazards(cfg.NEW, report)
    special.borrow_fixes(cfg.NEW, report)
    special.outside(report)
    parent.update(cfg.NEW)
    json.dump(dict(report=report, hazards=hazards, ext=changed_ext, manual=man), open(cfg.GEN + "/build-report.json", "w"), ensure_ascii=False, indent=1)
    print("\n".join(report)); print("hazards:", len(hazards)); [print("  ", h) for h in hazards]
    for k, v in measure.measure(cfg.NEW).items(): print("%-22s files %3d lines %5d bytes %7d" % ((k,) + v))

if __name__ == "__main__": main()
