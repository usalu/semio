"""🧵 Author 65 harness: `python3 author-65-run.py <category/slug> [--force-mtime] [--dry]`.
Baseline product check, full fix, product check of the variant, accept when no measurement is worse; else accept scale by scale (cumulative), then split a rejected scale into restore-only and flag-only. Writes the accepted fix to the source, prints a summary and saves JSON under the generated folder."""
import glob, json, os, subprocess, sys, time

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "author-65")
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
os.makedirs(G, exist_ok=True)


def check(asset, source=None):
    cmd = [BUN, "volumes-check.ts", asset, "--product", "--json"] + (["--source", source] if source else [])
    tmp = os.path.join(G, "check.tmp.json")
    with open(tmp, "w") as f:
        subprocess.run(cmd, cwd=T, stdout=f, stderr=subprocess.DEVNULL)
    return json.load(open(tmp))["measurements"]


def key(m):
    return (m["rule"], m.get("scale"), m.get("list"), m["name"])


def margin(m):
    return m["value"] - m["limit"] if m["bound"] == "min" else m["limit"] - m["value"]


def worse(before, after):
    b = {key(m): m for m in before}
    out = []
    for m in after:
        if m["name"].startswith("info:") or m["name"].startswith("slenderRatio:") and m["ok"]: continue
        o = b.get(key(m))
        if o is None:
            if not m["ok"]: out.append((key(m), None, m["value"], m["limit"]))
            continue
        if m["ok"]: continue
        if o["ok"]: out.append((key(m), o["value"], m["value"], m["limit"])); continue
        tol = 0.005 * max(abs(m["limit"]), 1e-3)
        if margin(m) < margin(o) - tol: out.append((key(m), o["value"], m["value"], m["limit"]))
    return out


def fix(asset, extra, out):
    r = subprocess.run(["python3", "author-65-fix.py", asset, "--out", out] + extra, cwd=T, capture_output=True, text=True)
    return r.stdout


def main():
    asset = sys.argv[1]
    slug = asset.replace("/", "_")
    src = os.path.join(T, "volumes", asset + ".json")
    age = (time.time() - os.path.getmtime(src)) / 60
    if age < 20 and "--force-mtime" not in sys.argv:
        print(f"SKIP {asset}: source modified {age:.1f} min ago"); return
    base = check(asset)
    json.dump(base, open(os.path.join(G, slug + ".before.json"), "w"))
    V = os.path.join(G, slug + ".variant.json")
    summary = {"asset": asset, "plan": {}, "rejected": []}

    def trial(plan):
        fix(asset, ["--plan", ",".join(f"{k}:{v}" for k, v in plan.items())], V)
        a = check(asset, V)
        return worse(base, a)

    plan = {s: "all" for s in ["50", "100", "200", "500"]}
    w = trial(plan)
    if w:
        summary["rejected"].append({"all": [[list(k), o, v, l] for k, o, v, l in w][:10]})
        plan = {}
        for sc in ["50", "100", "200", "500"]:
            for mode in ("all", "thick", "flag"):
                w2 = trial({**plan, sc: mode})
                if not w2:
                    plan[sc] = mode
                    break
                summary["rejected"].append({f"{sc}:{mode}": [[list(k), o, v, l] for k, o, v, l in w2][:6]})
    summary["plan"] = plan
    if plan and "--dry" not in sys.argv:
        age = (time.time() - os.path.getmtime(src)) / 60
        if age < 20 and "--force-mtime" not in sys.argv:
            print("source changed meanwhile, not written"); return
        out = fix(asset, ["--plan", ",".join(f"{k}:{v}" for k, v in plan.items())], src)
        summary["log"] = os.path.join(G, "changes-" + slug + ".json")
    json.dump(summary, open(os.path.join(G, slug + ".run.json"), "w"), indent=1)
    print(json.dumps(summary, ensure_ascii=False)[:3500])


main()
