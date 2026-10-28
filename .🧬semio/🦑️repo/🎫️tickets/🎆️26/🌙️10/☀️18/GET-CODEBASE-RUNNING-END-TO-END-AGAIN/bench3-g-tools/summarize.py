"""📋 Summarises measure.py records of one stage directory: one markdown table row per record (against the bench2-g baseline) and one `ROW:` line for coordination.md. Streams the logs line by line.

`summarize.py <dir> <stage> [<label-regex>]` prints rows; `--row` prints only the coordination line."""
import glob, json, os, re, statistics, sys

BASE = {"course.noop": (60, 77, 83), "course.edit": (506,)}


def scan(path):
    reserve = plan_reused = plan_stale = None
    sized = None
    try:
        with open(path, encoding="utf8", errors="replace") as handle:
            for line in handle:
                if len(line) > 4096:
                    continue
                m = re.search(r"need (\d+) MB", line)
                if m and reserve is None:
                    reserve = int(m.group(1))
                m = re.search(r"sized at (\d+)%", line)
                if m and sized is None:
                    sized = m.group(0)
                if "[bazel] plan reused" in line:
                    plan_reused = (plan_reused or 0) + 1
                if "[bazel] this plan cannot be reused" in line:
                    plan_stale = (plan_stale or 0) + 1
    except OSError:
        pass
    return reserve, sized, plan_reused or 0, plan_stale or 0


def gib(v):
    return "-" if not v else "%.2f GiB" % (v / 1073741824)


def row(r):
    ph = r.get("phases", {})
    a = (r.get("actions") or {}).get("bazel")
    acts = "-" if a is None else "%d executed, %d cached" % (a["executed"], a["cached"])
    reserve, sized, reused, stale = scan(r.get("log", ""))
    ph_s = ", ".join("%s %d" % (k, round(v / 1000)) for k, v in ph.items())
    g = r.get("graph") or {}
    return {"id": r["id"], "exit": r["exit"], "wall": round(r["wallMs"] / 1000), "phases": ph_s,
            "rss": gib(r.get("peakRssBytes")), "foot": gib(r.get("peakFootprintBytes")), "jvm": gib(r.get("bazelServersPeakRssBytes")),
            "acts": acts, "graph": "gen %s, %s inferred" % (g.get("generation"), g.get("inferredProjects", 0)),
            "env": "load %s, swap max %s MB, reserve %s" % (None if r.get("load1Start") is None else round(r["load1Start"]), None if r.get("swapUsedMbMax") is None else round(r["swapUsedMbMax"]), ("not asked/queued" if sized is None else sized) if reserve is None else "%d MB (%s)" % (reserve, sized)),
            "plan": "plan reused x%d, not reusable x%d" % (reused, stale), "attempts": r.get("attempts"), "executed": None if a is None else a["executed"]}


def main():
    args = [x for x in sys.argv[1:] if not x.startswith("--")]
    d, stage = args[0], args[1]
    rx = re.compile(args[2]) if len(args) > 2 else re.compile(".")
    rows = []
    for p in sorted(glob.glob(os.path.join(d, "*.json"))):
        if p.endswith(".all.json") or p.endswith("report.json"):
            continue
        try:
            r = json.load(open(p, encoding="utf8"))
        except ValueError:
            continue
        if "wallMs" in r and rx.search(r["id"]):
            rows.append(row(r))
    if "--row" in sys.argv:
        samples = [x for x in rows if not re.search(r"\.(prime|r\d+)$", x["id"])]
        walls = [x["wall"] for x in samples]
        print("%s: %s | walls %s s | executed %s | footprint %s | %s | %s" % (stage, ",".join(sorted({x["id"].split(".")[0] for x in rows})), "/".join(map(str, walls)) or "-", "/".join(str(x["executed"]) for x in samples) or "-", "/".join(x["foot"] for x in samples) or "-", samples[0]["env"] if samples else "-", samples[0]["plan"] if samples else "-"))
        return
    print("| run | exit | wall (s) | phases (s) | memory | actions | graph | load, swap, capacity reserve | plan |")
    print("|---|---|---|---|---|---|---|---|---|")
    for x in rows:
        print("| %s | %s | %s | %s | RSS %s, footprint %s, bazel JVM %s | %s | %s | %s | %s |" % (x["id"], x["exit"], x["wall"], x["phases"], x["rss"], x["foot"], x["jvm"], x["acts"], x["graph"], x["env"], x["plan"]))


main()
