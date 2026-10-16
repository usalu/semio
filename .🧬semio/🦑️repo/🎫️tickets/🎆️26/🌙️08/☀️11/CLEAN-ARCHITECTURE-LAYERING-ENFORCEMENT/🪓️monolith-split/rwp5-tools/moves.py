# 🚛️ The WP5 size spill as an executable plan: tutorial group up to window-actions, keyboard and sync groups of actions up to pointer, free items of actions down to window-actions, paint helpers of view-chrome down to view-paint.
import os, re, sys, shutil, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, mover, deps, slack, rsedit
from mover import Plan, read, write, path_of
from rsedit import Doc

D = cfg.CRATE2DIR

def merge_slug(root, src_crate, slug, dst_crate):
    a = path_of(root, D[src_crate], slug); b = path_of(root, D[dst_crate], slug)
    src = read(a).split("\n")
    k = next(i for i, l in enumerate(src) if l.startswith("use super::*;"))
    body = "\n".join(src[k + 1:]).strip("\n")
    if os.path.exists(b):
        write(b, read(b).rstrip("\n") + "\n\n" + body + "\n")
    else:
        write(b, mover.new_file(mover.EMOJI[dst_crate], dst_crate, slug) + "\n" + body + "\n")
    os.remove(a); os.rmdir(os.path.dirname(a))

def nodes_of(root, crate, slug=None):
    ns = deps.load(root); deps.link(ns); iv = slack.intervals(ns)
    return [(n, iv[id(n)]) for n in ns if n.crate == crate and (slug is None or n.slug == slug)], ns

def run(root, log):
    plan = Plan(root)
    log.append("A: session tutorial-runtime -> window-actions (whole slug)")
    mover.move_slug(root, D["shell-session"], "🎓️tutorial-runtime", D["shell-window-actions"])
    cand, ns = nodes_of(root, "shell-actions")
    up = [n for n, (lo, hi) in cand if n.kind == "method" and hi >= 3]
    down = [n for n, (lo, hi) in cand if n.kind == "free" and lo <= 1 and hi >= 1]
    for n in up:
        plan.take_method(D["shell-actions"], n.slug, n.name, D["shell-pointer"]); log.append(f"B: method {n.name} {n.slug} -> pointer")
    plan.flush()
    for n in down:
        plan.take_item(D["shell-actions"], n.slug, n.name, D["shell-window-actions"]); log.append(f"C: free {n.name} {n.slug} -> window-actions")
    plan.flush()
    cand, ns = nodes_of(root, "shell-view-chrome")
    paint = [n for n, (lo, hi) in cand if n.kind == "free" and lo <= cfg.RANK["shell-view-paint"] and hi >= cfg.RANK["shell-view-paint"]]
    for n in paint:
        plan.take_item(D["shell-view-chrome"], n.slug, n.name, D["shell-view-paint"]); log.append(f"D: free {n.name} {n.slug} -> view-paint")
    plan.flush()
    return plan

if __name__ == "__main__":
    root = cfg.NEW; log = []
    run(root, log); print("\n".join(log))
