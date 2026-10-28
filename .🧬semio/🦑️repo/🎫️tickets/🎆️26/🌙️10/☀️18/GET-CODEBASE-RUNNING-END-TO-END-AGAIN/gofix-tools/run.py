import argparse, json, os, resource, subprocess, sys, threading, time
from concurrent.futures import ThreadPoolExecutor

ap = argparse.ArgumentParser()
ap.add_argument("--mirror", required=True)
ap.add_argument("--plan", required=True)
ap.add_argument("--out", required=True)
ap.add_argument("--cache", required=True)
ap.add_argument("--steps", default="vet,test")
ap.add_argument("--only", default="")
ap.add_argument("--ids", default="")
ap.add_argument("--jobs", type=int, default=2)
ap.add_argument("--scratch", choices=["inside", "outside"], default="inside")
ap.add_argument("--label", default="run")
ap.add_argument("--run", default="")
ap.add_argument("--skip", default="")
ap.add_argument("--timeout", default="15m")
a = ap.parse_args()

mirror = os.path.realpath(a.mirror)
goroot = os.environ["GOFIX_GOROOT"]
os.makedirs(a.out, exist_ok=True)
os.makedirs(a.cache, exist_ok=True)
rows = json.load(open(os.path.join(a.plan, "modules.json")))
only = [s for s in a.only.split(",") if s]
rows = [r for r in rows if not only or any(s in r["module"] for s in only)]
if a.ids:
    lo, _, hi = a.ids.partition("-")
    rows = [r for r in rows if int(lo) <= r.get("id", -1) <= int(hi or lo)]
lock = threading.Lock()


def rss_guard():
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1024 * 1024:
        print("[DEBUG] runner RSS above 500 MB, aborting", file=sys.stderr)
        os._exit(9)


def environment(row, step):
    scratch_root = os.path.join(mirror, ".cache", "gofix", f"{a.label}-{row['id']}") if a.scratch == "inside" else os.path.join(os.path.realpath(os.environ.get("TMPDIR", "/tmp")), f"gofix-{a.label}-{row['id']}")
    tmp = os.path.join(scratch_root, "tmp")
    os.makedirs(tmp, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("GO", "SEMIO_")) or k.startswith("GOFIX_")}
    env.update({
        "PATH": os.path.join(goroot, "bin") + os.pathsep + env.get("PATH", ""),
        "GOROOT": goroot,
        "GOTOOLCHAIN": "local",
        "GOFLAGS": "-mod=readonly",
        "GOPROXY": "off",
        "GOCACHE": a.cache,
        "GOMODCACHE": os.path.join(a.cache, "..", "gomod"),
        "GOWORK": os.path.join(mirror, "go.work") if row["inWork"] else "off",
        "TMPDIR": tmp, "TMP": tmp, "TEMP": tmp,
        "SEMIO_TEST_ARTIFACT_DIR": scratch_root,
    })
    for key, value in os.environ.items():
        if key.startswith("GOFIX_ENV_"):
            env[key[len("GOFIX_ENV_"):]] = value
    return env, scratch_root


def stream(cmd, cwd, env, log):
    t0 = time.time()
    failing, order = {}, []
    counts = dict(pass_=0, fail=0, skip=0, top_pass=0, top_fail=0, top_skip=0, pkg_ok=0, pkg_fail=0, pkg_none=0)
    buf = {}
    with open(log, "wb") as out:
        proc = subprocess.Popen(["nice", "-n", "10"] + cmd, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        for raw in proc.stdout:
            out.write(raw)
            line = raw.decode("utf8", "replace")
            if not line.startswith("{"):
                continue
            try:
                e = json.loads(line)
            except ValueError:
                continue
            key = (e.get("Package", ""), e.get("Test", ""))
            act = e.get("Action")
            if act == "output" and key[1]:
                b = buf.setdefault(key, [])
                if len(b) < 120:
                    b.append(e.get("Output", ""))
            elif act in ("pass", "fail", "skip"):
                if key[1]:
                    top = "/" not in key[1]
                    counts[{"pass": "pass_", "fail": "fail", "skip": "skip"}[act]] += 1
                    if top:
                        counts["top_" + {"pass": "pass", "fail": "fail", "skip": "skip"}[act]] += 1
                    if act == "fail":
                        failing[key] = "".join(buf.get(key, []))[-4000:]
                        order.append(key)
                    buf.pop(key, None)
                else:
                    counts[{"pass": "pkg_ok", "fail": "pkg_fail", "skip": "pkg_none"}[act]] += 1
        proc.wait()
    rss_guard()
    return proc.returncode, round(time.time() - t0, 1), counts, [{"package": k[0], "test": k[1], "output": failing[k]} for k in order if "/" not in k[1] or True]


def tail(log, n=60):
    try:
        with open(log, "rb") as f:
            f.seek(0, 2)
            size = f.tell()
            f.seek(max(0, size - 12000))
            return f.read().decode("utf8", "replace").splitlines()[-n:]
    except OSError:
        return []


def module(row):
    res = {"module": row["module"], "id": row.get("id"), "steps": []}
    if "error" in row:
        res["plan_error"] = row["error"]
        return res
    cwd = os.path.join(mirror, row["module"])
    overlay = os.path.join(a.plan, f"{row['id']}.overlay.json")
    for step in a.steps.split(","):
        env, scratch = environment(row, step)
        log = os.path.join(a.out, f"{a.label}-{row['id']}-{step}.log")
        pk = row["packages"]
        if step == "vet":
            cmd = ["go", "vet", f"-overlay={overlay}"] + pk
        elif step == "build":
            cmd = ["go", "build", f"-overlay={overlay}", "-o", os.devnull] + pk
        else:
            cmd = ["go", "test", f"-overlay={overlay}", "-count=1", "-json", "-timeout", a.timeout] + (["-run", a.run] if a.run else []) + (["-skip", a.skip] if a.skip else []) + pk
        code, secs, counts, failing = stream(cmd, cwd, env, log)
        entry = {"step": step, "exit": code, "seconds": secs, "log": os.path.basename(log)}
        if step == "test":
            entry["counts"] = counts
            entry["failing"] = failing
        else:
            entry["tail"] = tail(log, 25) if code else []
        res["steps"].append(entry)
        with lock:
            print(f"[gofix] {row['module'].split('/')[-3:]} {step} exit={code} {secs}s " + (f"top {counts['top_pass']}/{counts['top_fail']}/{counts['top_skip']} pkg {counts['pkg_ok']}/{counts['pkg_fail']}" if step == "test" else ""), flush=True)
    return res


with ThreadPoolExecutor(max_workers=a.jobs) as pool:
    results = list(pool.map(module, rows))
json.dump({"label": a.label, "scratch": a.scratch, "results": results}, open(os.path.join(a.out, f"summary-{a.label}.json"), "w"), indent=1)
print("[gofix] done", len(results))
