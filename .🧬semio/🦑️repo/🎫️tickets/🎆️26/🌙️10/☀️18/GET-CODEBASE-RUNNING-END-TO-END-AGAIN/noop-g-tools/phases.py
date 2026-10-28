#!/usr/bin/env python3
"""phases.py <run.ts>...: one line per run with the phases of the hot path in seconds (from SEMIO_REPO_PROFILE tables)."""
import re, sys
def tables(path):
    cur = None; out = []
    for l in open(path, encoding="utf8", errors="replace"):
        m = re.match(r"\s*[\d.]+ (#repo-profile.*|#total.*|\d+\t\d+\t\S+\t.*)$", l.rstrip("\n"))
        if not m: continue
        t = m.group(1)
        if t.startswith("#repo-profile"): cur = {}; out.append(cur)
        elif t.startswith("#total"): cur["_total"] = float(t.split("\t")[1])
        elif cur is not None:
            p = t.split("\t"); cur.setdefault(p[2], []).append((float(p[3]), float(p[4]), float(p[5]), p[6] if len(p) > 6 else ""))
    return out
def dur(tab, name): return sum(r[1] for r in tab.get(name, [])) / 1000
def start(tab, name): return tab[name][0][0] / 1000 if name in tab else None
rows = [("capacity wait", "door", "door.capacity", "self"), ("source capture", "door", "source.capture", "dur")]
for path in sys.argv[1:]:
    ts = tables(path)
    door = next((t for t in ts if "door.capacity" in t), {}); kernel = next((t for t in ts if "tool.bazel" in t or "graph.load" in t), {}); planchild = next((t for t in ts if "graph.plan" in t and "graph.load" not in t), {}); planner = next((t for t in ts if "bazel.source-snapshot" in t), {})
    served = door.get("source.served", [(0, 0, 0, "-")])[0][3]
    cap = door.get("door.capacity", [(0, 0, 0, "")])[0][2] / 1000
    pin = (start(kernel, "graph.pinned") or 0) - (start(kernel, "graph.rust-ready") or 0) if kernel else 0
    line = dict(file=path.split("/")[-1], wait=cap, capture=dur(door, "source.capture"), served=served, boot=(start(kernel, "boot.loaded") or 0), ensure=dur(kernel, "dependencies.ensure") + dur(kernel, "dependencies.locks"), plan_child=planchild.get("_total", 0) / 1000, graph_load=dur(kernel, "graph.load"), pin=pin, bazel_prep=dur(kernel, "bazel.prepare") + dur(kernel, "bazel.modules"), publish=dur(kernel, "bazel.publish"), planner=dur(kernel, "bazel.planner"), info=dur(kernel, "bazel.info"), invoke=dur(kernel, "bazel.invoke"), total=door.get("_total", 0) / 1000)
    line["fixed_before_bazel_invoke"] = line["capture"] + line["boot"] + line["ensure"] + line["graph_load"] + line["pin"] + line["bazel_prep"] + line["publish"] + line["info"]
    print("  ".join("%s=%s" % (k, ("%.1f" % v) if isinstance(v, float) else v) for k, v in line.items()))
