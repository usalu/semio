"""🧑‍🤝‍🧑 T9 turnkey: the real three-agent acceptance run on the declared 16 GB machine, one command, one verdict table, one `verdict.json`.

`run.sh [--prime] [--wait-quiet] [--force] [--tag T]`   real run: preflight (selftest, readiness, quiet window, follow-up window, simulation), optional warm-up, `agents3.py run`, verdict
`run.sh --dry-run`                                      the same pipeline against a fake checkout and replayed gate logs (no build, no repo process, nothing live edited), plus a starvation negative control
`run.sh --preflight`                                    only the checks, no run, no edit

Output: `C/🗑️generated/agents3/verdict.json` (real) or `verdict.dry-run.json` (dry run), the run directory `C/🗑️generated/agents3/run-<tag>` (logs, timeline, summary.md).
Exit: 0 PASS (also PASS with an unexercised progress rule), 1 FAIL or a failed gate, 2 INCONCLUSIVE or WARN-level evidence missing.
The machine is declared, not emulated: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=16384` and `SEMIO_HOST_CAPACITY_CORES=8` shrink what the capacity governor sees (available memory = min(real free, declared)), the real host keeps its 32 GB.
"""
import argparse, glob, json, os, re, shlex, shutil, subprocess, sys, time
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.abspath(os.path.join(HERE, ".."))
ROOT = os.path.abspath(os.path.join(C, *([".."] * 7)))
sys.path.insert(0, os.path.join(C, "harness3-tools"))
sys.path.insert(0, os.path.join(C, "agents3-g-tools"))
import hostgate  # noqa: E402
import render  # noqa: E402

GEN = os.path.join(C, "🗑️generated", "agents3")
HARNESS = os.path.join(C, "agents3-g-tools", "agents3.py")
CRITERIA = render.CRITERIA["agents3"]
PRODUCTS = json.load(open(os.path.join(C, "agents3-g-tools", "products.json"), encoding="utf8"))
TARGET_FILES = [p["editFile"] for p in PRODUCTS["products"]] + [p["file"] for p in PRODUCTS["peers"] if p["id"] == "unrelated-ts"]
EDITED_FILES = [p["editFile"] for p in PRODUCTS["products"] if p["id"] in ("course", "print")] + [p["file"] for p in PRODUCTS["peers"] if p["id"] == "unrelated-ts"]
CLASSES = {"course": "run:build:@teaching/architecture-course:build", "print": "run:test:@semio-tech/print-rs:test", "dashboard": "run:build:@semio-tech/repo-dashboard-rs:build"}
HARNESS_SCENARIOS = ["hot", "hot-firstedit", "hot-late", "hot-third", "late-third", "mixed", "mixed-long"]
MB = 1048576.0
STATE = {"gates": [], "notes": []}


def say(text):
    print("[%s] %s" % (time.strftime("%H:%M:%S"), text), flush=True)


def gate(name, ok, detail, blocking=True):
    STATE["gates"].append({"name": name, "ok": bool(ok), "blocking": blocking, "detail": detail})
    say("%-34s %s  %s" % (name, "ok  " if ok else ("FAIL" if blocking else "WARN"), detail))
    return ok


def git_dirty(files):
    out = subprocess.run(["git", "status", "--porcelain", "--"] + files, cwd=ROOT, capture_output=True, text=True).stdout
    return [line for line in out.splitlines() if line.strip()]


def capacity_directory():
    home = os.path.expanduser("~")
    if sys.platform == "darwin":
        base = os.path.join(home, "Library", "Caches")
    elif sys.platform == "win32":
        base = os.environ.get("LOCALAPPDATA") or os.path.join(home, "AppData", "Local")
    else:
        base = os.environ.get("XDG_CACHE_HOME") if os.path.isabs(os.environ.get("XDG_CACHE_HOME", "")) else os.path.join(home, ".cache")
    return os.path.join(base, "semio", "host-capacity")


def followup_ages():
    """Minutes since the newest sample of each product class in the live estimates store (the quiet follow-up forecast holds for 30 minutes); None when unknown."""
    path = os.path.join(capacity_directory(), "estimates.json")
    try:
        classes = json.load(open(path, encoding="utf8"))["classes"]
    except (OSError, ValueError, KeyError):
        return {name: None for name in CLASSES}
    now = time.time() * 1000
    ages = {}
    for name, key in CLASSES.items():
        samples = classes.get(key, {}).get("samples", [])
        ages[name] = round((now - max(s["atMs"] for s in samples)) / 60000, 1) if samples else None
    return ages


def host_sample():
    try:
        return json.load(open(os.path.join(capacity_directory(), "sample.json"), encoding="utf8")).get("host", {})
    except (OSError, ValueError):
        return {}


MEMORY_NEEDED_MB = CRITERIA["declaredTotalMegabytes"] - 2048 + 0


def available_mb():
    """Memory the kernel can hand out now without compressing or swapping (the capacity governor's definition: free + inactive + speculative, capped by free + file-backed + purgeable on macOS; MemAvailable on Linux); None when unknown."""
    try:
        if sys.platform == "darwin":
            text = subprocess.run(["vm_stat"], capture_output=True, text=True, timeout=10).stdout
            page = int(re.search(r"page size of (\d+)", text).group(1))
            pages = lambda name: int(re.search(r"^%s:\s+(\d+)\." % name, text, re.M).group(1)) if re.search(r"^%s:\s+(\d+)\." % name, text, re.M) else 0
            settled = pages("Pages free") + pages("Pages inactive") + pages("Pages speculative")
            reclaimable = pages("Pages free") + pages("File-backed pages") + pages("Pages purgeable") if re.search(r"^File-backed pages:", text, re.M) else settled
            return int(min(settled, reclaimable) * page / MB)
        if sys.platform.startswith("linux"):
            return int(int(re.search(r"MemAvailable:\s+(\d+) kB", open("/proc/meminfo").read()).group(1)) / 1024)
    except Exception:
        pass
    return host_sample().get("availableMegabytes")


def foreign_grants():
    return len(glob.glob(os.path.join(capacity_directory(), "grants", "*.json")))


def swap_used_mb():
    try:
        text = subprocess.run(["sysctl", "-n", "vm.swapusage"], capture_output=True, text=True, timeout=10).stdout
        return float(re.search(r"used = ([\d.]+)M", text).group(1))
    except Exception:
        return None


def free_percent():
    try:
        return int(re.search(r"free percentage: (\d+)%", subprocess.run(["memory_pressure", "-Q"], capture_output=True, text=True, timeout=10).stdout).group(1))
    except Exception:
        return None


def tool(args, **kw):
    return subprocess.run(args, capture_output=True, text=True, **kw)


def step_selftest(args):
    if args.skip_selftest:
        return gate("harness selftest", True, "skipped", False)
    result = tool([sys.executable, "-I", os.path.join(C, "harness3-tools", "selftest.py")], timeout=600)
    last = (result.stdout.strip().splitlines() or ["no output"])[-1]
    return gate("harness selftest", result.returncode == 0 and "SELFTEST PASS" in last, last[:160])


def step_readiness(args):
    result = tool([sys.executable, "-I", HARNESS, "preflight"], timeout=600)
    lines = [x for x in result.stdout.splitlines() if x.strip()]
    for line in lines:
        say("    " + line[:200])
    bad = [x for x in lines if "MISSING" in x or "looks edited" in x or "declaration knobs live (TOTAL_MEGABYTES, CORES): False" in x]
    return gate("harness readiness (agents3.py)", not bad and result.returncode == 0, "edit targets clean, declaration knobs live, probes present" if not bad else "; ".join(bad)[:200])


def step_quiet(args, strict):
    runs = hostgate.official_runs()
    queued = hostgate.waiting(ROOT, None)
    count, _pids = hostgate.bazel_servers()
    available = available_mb()
    swap, free, load = swap_used_mb(), free_percent(), os.getloadavg()[0] if hasattr(os, "getloadavg") else None
    gate("quiet: foreign product runs", not runs, "none" if not runs else "%d: %s" % (len(runs), [c[:90] for _p, c in runs][:2]), strict)
    gate("quiet: lease waiters", queued is not None and queued < 4, "%s waiting (limit 4)" % queued, strict)
    gate("quiet: capacity grants held", foreign_grants() == 0, "%d live grant file(s)" % foreign_grants(), False)
    gate("quiet: Bazel servers alive", count <= 1, "%d server(s); they idle out after 10 minutes and count toward the footprint" % count, False)
    gate("host: memory available", available is None or available >= MEMORY_NEEDED_MB, "%s MB available (need >= %d: the declared 16 GB minus the 2 GB floor; below it the live estimates make a peer wait for another's whole run), load %s, free %s %%, swap used %s MB" % (available, MEMORY_NEEDED_MB, load and round(load, 1), free, swap), strict)
    return {"foreignRuns": len(runs), "waiters": queued, "bazelServers": count, "availableMb": available, "load1": load, "freePct": free, "swapUsedMb": swap}


def step_followup():
    ages = followup_ages()
    stale = [name for name, age in ages.items() if age is None or age > 30]
    gate("follow-up window (30 min)", not stale, "minutes since the last run per class: %s%s" % (ages, "" if not stale else "; stale %s: expect the hot-late shape (full learned reservations), run with --prime to hold the quiet forecast" % stale), False)
    return ages


def step_sim(args, label):
    out = os.path.join(GEN, "sim")
    os.makedirs(out, exist_ok=True)
    env = dict(os.environ, QUIET16_OUT=out)
    result = subprocess.run([sys.executable, "-I", os.path.join(HERE, "sim", "table.py"), ROOT, label, "--law", "--named", "--quiet", "--lastrun", "--busy"], capture_output=True, text=True, env=env, timeout=900)
    law = [x for x in result.stdout.splitlines() if x.startswith("LAW")]
    rows = []
    try:
        for row in json.load(open(os.path.join(out, "table-%s.json" % label), encoding="utf8")):
            if row["scenario"] in HARNESS_SCENARIOS:
                runs = row["runs"]
                rows.append({"scenario": row["scenario"], "host": row["host"], "makespanS": row["makespan"], "maxQueueS": max([r["door"] + r["upgrade"] for r in runs] or [0]), "starved": row["queueOver2x"] + ([row["starvedRows"]] if row["starvedRows"] else []),
                             "runs": [{"agent": r["agent"], "run": r["run"], "wallS": round(r["wall"]), "aloneS": round(r["alone"]), "queueS": round(r["door"] + r["upgrade"]), "jobs": r["jobs"]} for r in runs]})
    except (OSError, ValueError, KeyError) as error:
        say("simulation table unreadable: %s" % error)
    flags = None
    try:
        flags = json.load(open(os.path.join(out, "sim-%s-hot-h32d-quiet.json" % label), encoding="utf8")).get("live")
    except (OSError, ValueError):
        pass
    ok = result.returncode == 0 and any("PASS" in x for x in law)
    gate("simulation of the live tree (law)", ok, "; ".join(law)[:120] or (result.stderr.strip().splitlines() or ["no output"])[-1][:160])
    stored = None
    try:
        store = os.path.join(out, "estimates-live.json")
        shutil.copyfile(os.path.join(capacity_directory(), "estimates.json"), store)
        again = subprocess.run([sys.executable, "-I", os.path.join(HERE, "sim", "table.py"), ROOT, label + "-store", "--law", "--named", "--quiet", "--lastrun", "--busy"], capture_output=True, text=True, env=dict(env, AGENTS3_STORE=store), timeout=900)
        stored = [x for x in again.stdout.splitlines() if x.startswith("LAW")]
        gate("simulation with the live learned estimates", again.returncode == 0, "; ".join(stored)[:300] + ("" if again.returncode == 0 else " (a real 16 GB laptop; the declared 16 GB Mac is judged by the prediction for this host)"), False)
    except OSError:
        pass
    for row in rows:
        if row["host"] in ("h32d-quiet", "h32d-fleet") and row["scenario"] in ("hot", "hot-firstedit", "hot-late"):
            say("    predicted %-13s %-10s makespan %4d s, longest queue %3d s, starved %s" % (row["scenario"], row["host"], row["makespanS"], row["maxQueueS"], row["starved"] or "none"))
    return {"law": "PASS" if ok else "FAIL", "liveStoreLaw": stored, "liveFlags": flags, "scenarios": rows}


def step_sim_now(label):
    """Predicts the harness-shaped scenarios for the host as it is right now (live free memory and load from the capacity sample, the live learned estimates store), so a loaded host is not mistaken for a governor defect."""
    sample = host_sample()
    total, available = sample.get("totalMegabytes"), available_mb()
    if not total or available is None:
        return gate("prediction for this host (now)", True, "no capacity sample yet (nothing predicted)", False) and None
    out = os.path.join(GEN, "sim-now")
    os.makedirs(out, exist_ok=True)
    store = os.path.join(out, "estimates.json")
    try:
        shutil.copyfile(os.path.join(capacity_directory(), "estimates.json"), store)
    except OSError:
        store = None
    host = {"realMegabytes": total, "declaredMegabytes": CRITERIA["declaredTotalMegabytes"], "declaredCores": CRITERIA["declaredCores"], "foreignMegabytes": max(0, total - available), "foreignLoad": sample.get("load") or 0}
    rows, bad = [], []
    for scenario in ("hot", "hot-firstedit", "hot-late"):
        command = ["bun", os.path.join(HERE, "sim", "sim.ts"), "--root", ROOT, "--scenario", scenario, "--host", "h32d-quiet", "--host-json", json.dumps(host), "--json", os.path.join(out, "%s-%s.json" % (label, scenario))] + (["--store", store] if store else [])
        result = subprocess.run(command, capture_output=True, text=True, cwd=ROOT, timeout=600)
        try:
            d = json.loads(result.stdout.strip().splitlines()[-1])
        except (ValueError, IndexError):
            bad.append("%s: simulation failed" % scenario)
            continue
        queue = max([r["door"] + r["upgrade"] for r in d["runs"]] or [0])
        rows.append({"scenario": scenario, "makespanS": d["makespan"], "maxQueueS": round(queue), "queueOver2x": d["queueOver2x"], "starvedRows": d["starvedRows"], "minFreePercent": d["minFree"]["freePercent"]})
        if d["queueOver2x"] or d["starvedRows"]:
            bad.append("%s: queue %d s (%s)" % (scenario, queue, d["queueOver2x"] or "%d starved rows" % d["starvedRows"]))
    gate("prediction for this host (now)", not bad, "%d MB available of %d; %s" % (available, total, "; ".join(bad) or "no queue above twice a run's own time in hot, hot-firstedit, hot-late"), False)
    return {"host": host, "scenarios": rows}


def prime_runs(args):
    log = os.path.join(GEN, "prime-%s.log" % args.tag)
    say("warming the products (official builds, one at a time): %s" % log)
    with open(log, "wb") as handle:
        code = subprocess.run(["sh", os.path.join(C, "harness3-tools", "prime.sh"), "h3-" + args.tag], stdout=handle, stderr=subprocess.STDOUT, cwd=ROOT).returncode
    bad = []
    for path in sorted(glob.glob(os.path.join(C, "gate-g-tools", "run", "h3-%s-*.json" % args.tag))):
        record = json.load(open(path, encoding="utf8"))
        if record.get("exit") != 0:
            bad.append(os.path.basename(path))
    return gate("prime (course, print, dashboard, hot x2)", code == 0 and not bad, "every record exit 0" if not bad else "failed: %s" % bad)


def make_fake_root(tag):
    root = os.path.join(GEN, "dry", tag, "root")
    shutil.rmtree(root, ignore_errors=True)
    for rel in TARGET_FILES:
        target = os.path.join(root, rel)
        assert len(target) < 250, "path too long: %d" % len(target)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(os.path.join(ROOT, rel), target)
    return root


def run_harness(args, tag, outdir, env_extra):
    os.makedirs(outdir, exist_ok=True)
    env = dict(os.environ, **env_extra)
    command = [sys.executable, "-I", HARNESS, "run", "--tag", tag, "--out", outdir, "--timeout", str(args.timeout), "--max-footprint-mb", str(args.max_footprint_mb), "--max-swap-growth-mb", str(args.max_swap_growth_mb), "--stagger", str(args.stagger)]
    log = os.path.join(GEN, "run-%s.chain.log" % tag)
    say("harness: %s (log %s)" % (" ".join(command[2:6]), log))
    with open(log, "wb") as handle:
        process = subprocess.Popen(command, stdout=handle, stderr=subprocess.STDOUT, cwd=ROOT, env=env, start_new_session=(os.name != "nt"))
        try:
            code = process.wait()
        except KeyboardInterrupt:
            process.terminate()
            code = process.wait()
    tail = open(log, encoding="utf8", errors="replace").read().strip().splitlines()[-3:]
    for line in tail:
        say("    " + line[:200])
    return code


def lines_for(record, language):
    """The EN or DE progress rows a run printed, as recorded by the harness (first wait row, admission, growth)."""
    wanted = ("waiting", "wartet", "reservation grown", "Reservierung nach", "admitted", "zugelassen", "grown from", "gewachsen", "forecast quiet", "ruhig prognostiziert")
    rows = [x["line"] for x in record.get("capacity", []) if any(w in x["line"] for w in wanted)]
    return rows[:4]


def build_verdict(directory, mode, tag, extras):
    meta = json.load(open(os.path.join(directory, "meta.json"), encoding="utf8"))
    results = json.load(open(os.path.join(directory, "results.json"), encoding="utf8"))
    text, overall, verdicts = render.agents3_markdown(directory)
    stats = render.timeline_stats(os.path.join(directory, "timeline.jsonl"))
    peaks = meta.get("peaks", {})
    page = meta.get("pageBytes") or 16384
    swapouts = (stats["last"]["swapouts"] - meta["vm0"]["swapouts"]) * page if stats["last"] else 0
    agents = []
    for name in "ABC":
        runs = []
        for record in results.get(name, []):
            sig = record.get("signals") or {}
            cap = sig.get("capacity") or {}
            bz = (record.get("actions") or {}).get("bazel") or {}
            runs.append({"label": record["label"], "attempt": record["id"].rsplit(".", 1)[-1], "exit": record["exit"], "wallS": round(record["wallMs"] / 1000, 1), "queueS": round(record["queueMs"] / 1000, 1), "runS": round(record["runMs"] / 1000, 1), "startedClock": record.get("startedClock"),
                         "executed": bz.get("executed"), "cached": bz.get("cached"), "graph": (sig.get("graph") or {}).get("outcome"), "plan": (sig.get("plan") or {}).get("outcome"),
                         "capacity": {"quiet": cap.get("quiet"), "admitted": cap.get("admitted"), "grown": cap.get("grown"), "jobsGrown": cap.get("jobsGrown"), "waitRows": cap.get("waitRows"), "maxWaitS": cap.get("maxWaitS"), "maxGrowWaitS": cap.get("maxGrowWaitS"), "maxWaitRowGapMs": cap.get("maxWaitRowGapMs"), "starvedRows": cap.get("starvedRows"), "switchedOff": cap.get("switchedOff")},
                         "maxSilenceS": round((sig.get("maxSilenceMs") or 0) / 1000, 1), "languageRows": sig.get("languages"), "wrongLanguageRows": sig.get("wrongLanguageRows"), "restarts": (sig.get("restarts") or {}).get("count"), "progressLines": lines_for(record, record["language"])})
        peak = peaks.get(name, {})
        agents.append({"agent": name, "language": runs and next(r["language"] for r in results[name]) or None, "wallS": round(sum(r["wallS"] for r in runs), 1), "queueS": round(sum(r["queueS"] for r in runs), 1), "peakFootprintMiB": round(peak.get("foot", 0) / MB), "peakRssMiB": round(peak.get("rss", 0) / MB), "runs": runs})
    names = [v[0] for v in verdicts]
    status = {name: v for name, v, _val, _rule in verdicts}
    progress_row = next((val for name, v, val, _rule in verdicts if name == "progress visible to waiters"), "")
    unexercised = status.get("progress visible to waiters") == "INCONCLUSIVE" and " 0 queued run(s), 0 run(s) that printed wait rows" in str(progress_row)
    rest = [v for name, v in status.items() if not (unexercised and name == "progress visible to waiters")]
    final = render.worst(rest)
    label = final if not (final == "PASS" and unexercised) else "PASS (progress rule not exercised: nothing waited)"
    starved = next((val for name, v, val, _rule in verdicts if name == "no starvation"), None)
    total = float(meta["env"].get("SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES", CRITERIA["declaredTotalMegabytes"]))
    verdict = {
        "version": 1, "mode": mode, "tag": tag, "overall": label, "overallStatus": final, "progressRuleExercised": not unexercised, "startedAt": meta["startedAt"], "endedAt": datetime.now(timezone.utc).isoformat(), "makespanS": round(meta.get("makespanMs", 0) / 1000, 1),
        "declared": meta["env"], "aborted": meta.get("aborted"), "restoredExact": meta.get("restoredExact"),
        "host": {"swapStartMb": meta.get("swapStartMb"), "swapEndMb": meta.get("swapEndMb"), "swapGrowthMiB": round(max(0.0, stats["swapMax"] - (stats["swapFirst"] or 0.0))), "swapoutsMiB": round(swapouts / MB), "compressorMaxMiB": round(stats["compressorMax"]), "freePctMin": stats["freeMin"], "load1Max": round(stats["loadMax"], 1),
                 "footprintPeakMiB": round(stats["footSumMax"] / MB), "footprintAt": stats["footSumAt"], "declaredTotalMiB": total, "bazelServersPeakMiB": round(peaks.get("servers", {}).get("foot", 0) / MB)},
        "agents": agents,
        "starvation": {"verdict": "STARVED" if status.get("no starvation") == "FAIL" else "none", "detail": starved},
        "swapStorm": {"verdict": "FAIL" if any(status.get(n) == "FAIL" for n in names if n.startswith("no swap storm")) else "none"},
        "languages": {"en": sum(a_run["languageRows"]["en"] for a in agents for a_run in a["runs"] if a["language"] == "en" and a_run.get("languageRows")), "de": sum(a_run["languageRows"]["de"] for a in agents for a_run in a["runs"] if a["language"] == "de" and a_run.get("languageRows"))},
        "criteria": [{"name": n, "verdict": v, "value": str(val), "rule": rule} for n, v, val, rule in verdicts],
        "gates": STATE["gates"], **extras,
    }
    return verdict, text


def table(verdict):
    rows = ["", "| agent | lang | run | exit | wall s | queue s | run s | executed | waits (max s / rows) | peak footprint MiB | peak RSS MiB | EN / DE rows |", "|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for a in verdict["agents"]:
        for r in a["runs"]:
            cap = r["capacity"]
            langs = r.get("languageRows") or {}
            rows.append("| %s | %s | %s/%s | %s | %.1f | %.1f | %.1f | %s | %s s / %s | %s | %s | %s / %s |" % (a["agent"], a["language"], r["label"], r["attempt"], r["exit"], r["wallS"], r["queueS"], r["runS"], r["executed"], max(cap.get("maxWaitS") or 0, cap.get("maxGrowWaitS") or 0), cap.get("waitRows"), a["peakFootprintMiB"], a["peakRssMiB"], langs.get("en"), langs.get("de")))
    host = verdict["host"]
    rows += ["", "host: swap growth %s MiB (limit %d), swapouts %s MiB (limit %d), free memory min %s %% (limit %d), footprint peak %s MiB (limit: declared %.0f MiB minus the capacity floor), load1 max %s, makespan %s s" % (host["swapGrowthMiB"], CRITERIA["swapGrowthMegabytesMax"], host["swapoutsMiB"], CRITERIA["swapoutBytesMax"] // 1048576, host["freePctMin"], CRITERIA["freePercentMin"], host["footprintPeakMiB"], host["declaredTotalMiB"], host["load1Max"], verdict["makespanS"]),
             "starvation: %s %s | swap storm: %s | languages seen: %s" % (verdict["starvation"]["verdict"], verdict["starvation"]["detail"] if verdict["starvation"]["verdict"] != "none" else "", verdict["swapStorm"]["verdict"], verdict["languages"]), ""]
    rows += ["| criterion | verdict | value |", "|---|---|---|"] + ["| %s | **%s** | %s |" % (c["name"], c["verdict"], c["value"].replace("|", "/")[:160]) for c in verdict["criteria"]]
    for a in verdict["agents"]:
        for r in a["runs"]:
            for line in r["progressLines"][:2]:
                rows.append("- %s %s [%s]: %s" % (a["agent"], r["label"], a["language"], line[:200]))
    rows += ["", "VERDICT %s: %s%s" % (verdict["mode"], verdict["overall"], " (aborted: %s)" % verdict["aborted"] if verdict["aborted"] else "")]
    return "\n".join(rows)


def dry_run(args):
    tag = args.tag or "dry"
    live_before = git_dirty(TARGET_FILES)
    results = {}
    for name, negative in (("pos", False), ("neg", True)):
        root = make_fake_root(tag + name)
        out = os.path.join(GEN, "dry", tag + name, "run")
        env = {"BENCH_LIVE": root, "AGENTS3_FAKE_CMD": "%s -I %s {agent} {label} {target} {language}" % (shlex.quote(sys.executable), shlex.quote(os.path.join(HERE, "fakeruns.py"))), "AGENTS3_SCRATCH": os.path.join(GEN, "dry", tag + name, "backup"), "HARNESS_NOGATE": "1"}
        if negative:
            env["AGENTS3_FAKE_NEGATIVE"] = "1"
        code = run_harness(args, tag + name, out, env)
        verdict, text = build_verdict(out, "dry-run" + ("-negative" if negative else ""), tag + name, {"fakeRoot": root, "harnessExit": code})
        results[name] = verdict
        json.dump(verdict, open(os.path.join(out, "verdict.full.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
        say("---- %s control ----" % ("negative" if negative else "positive"))
        print(table(verdict), flush=True)
    pos, neg = results["pos"], results["neg"]
    live_after = git_dirty(TARGET_FILES)
    checks = [
        ("positive run completed, every agent ran", all(a["runs"] and all(r["exit"] == 0 for r in a["runs"]) for a in pos["agents"])),
        ("EN and DE rows both parsed", pos["languages"]["en"] > 0 and pos["languages"]["de"] > 0),
        ("no wrong-language rows in the positive run", all(not r.get("wrongLanguageRows") for a in pos["agents"] for r in a["runs"])),
        ("fake-root edits restored byte-exact", bool(pos["restoredExact"]) and all(pos["restoredExact"].values())),
        ("negative control turns `no starvation` FAIL", next(c["verdict"] for c in neg["criteria"] if c["name"] == "no starvation") == "FAIL" and neg["starvation"]["verdict"] == "STARVED"),
        ("starved criterion PASS in the positive run", pos["starvation"]["verdict"] == "none"),
        ("live edit targets untouched", live_before == live_after == []),
        ("selftest and simulation gates ran", {g["name"] for g in STATE["gates"]} >= {"harness selftest", "simulation of the live tree (law)"}),
    ]
    out = {"version": 1, "mode": "dry-run", "wiring": [{"check": n, "ok": bool(ok)} for n, ok in checks], "positive": pos, "negative": neg, "gates": STATE["gates"]}
    path = os.path.join(GEN, "verdict.dry-run.json")
    json.dump(out, open(path, "w", encoding="utf8"), indent=1, ensure_ascii=False)
    say("dry-run wiring:")
    for name, ok in checks:
        say("    %-52s %s" % (name, "ok" if ok else "FAIL"))
    wired = all(ok for _, ok in checks)
    say("DRY-RUN WIRING %s -> %s" % ("OK" if wired else "FAILED", path))
    return 0 if wired else 1


def real_run(args):
    tag = args.tag or time.strftime("%m%d-%H%M")
    out = os.path.join(GEN, "run-%s" % tag)
    if os.path.exists(os.path.join(out, "results.json")):
        say("run directory %s already holds a result; pick another --tag" % out)
        return 1
    if args.wait_quiet:
        say("waiting for a quiet host (no foreign product run, fewer than 4 lease waiters twice in a row, %d MB available)" % MEMORY_NEEDED_MB)
        hostgate.wait_quiet(ROOT, say=say)
        for poll in range(240):
            now = available_mb()
            if now is None or now >= MEMORY_NEEDED_MB:
                break
            if poll % 4 == 0:
                say("memory: %s MB available, waiting for %d" % (now, MEMORY_NEEDED_MB))
            time.sleep(30)
    if args.prime and not prime_runs(args):
        return 1
    if args.prime:
        step_followup()
    code = run_harness(args, tag, out, {"HARNESS_NOGATE": "1"})
    if not os.path.exists(os.path.join(out, "results.json")):
        say("the harness left no results.json (exit %s); restore with: python3 -I %s restore --dir %s" % (code, HARNESS, out))
        return 1
    dirty = git_dirty(EDITED_FILES)
    gate("edit targets back to git state", not dirty, "clean" if not dirty else "DIRTY %s; restore with: python3 -I %s restore --dir %s" % (dirty, HARNESS, out))
    verdict, text = build_verdict(out, "real", tag, {"harnessExit": code, "runDirectory": out, "editTargetsDirty": dirty, "prediction": STATE.get("prediction"), "preflight": STATE.get("preflight")})
    for path in (os.path.join(GEN, "verdict.json"), os.path.join(out, "verdict.full.json")):
        json.dump(verdict, open(path, "w", encoding="utf8"), indent=1, ensure_ascii=False)
    print(table(verdict), flush=True)
    say("verdict written to %s" % os.path.join(GEN, "verdict.json"))
    if dirty or verdict["overallStatus"] == "FAIL":
        return 1
    return 2 if verdict["overallStatus"] in ("INCONCLUSIVE", "WARN") else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--preflight", action="store_true")
    parser.add_argument("--prime", action="store_true", help="warm course, print and dashboard (official builds) right before the run so every class is inside the 30-minute follow-up window")
    parser.add_argument("--wait-quiet", action="store_true", help="block until no foreign product run is executing and fewer than 4 lease waiters")
    parser.add_argument("--force", action="store_true", help="run although a blocking gate failed (the verdict then says so)")
    parser.add_argument("--tag", default=None)
    parser.add_argument("--timeout", type=float, default=3000.0)
    parser.add_argument("--max-footprint-mb", type=int, default=26000)
    parser.add_argument("--max-swap-growth-mb", type=int, default=4096)
    parser.add_argument("--stagger", type=float, default=3.0)
    parser.add_argument("--skip-selftest", action="store_true")
    args = parser.parse_args()
    os.makedirs(GEN, exist_ok=True)
    strict = not args.dry_run
    say("agents3 T9 %s on %s (declared %s MB / %s cores)" % ("dry run" if args.dry_run else "preflight" if args.preflight else "real run", ROOT, CRITERIA["declaredTotalMegabytes"], CRITERIA["declaredCores"]))
    step_selftest(args)
    step_readiness(args)
    quiet = step_quiet(args, strict)
    ages = step_followup()
    prediction = step_sim(args, "dry" if args.dry_run else "live")
    prediction["now"] = step_sim_now("dry" if args.dry_run else "live")
    STATE["prediction"], STATE["preflight"] = prediction, {"quiet": quiet, "followupMinutes": ages}
    blocked = [g for g in STATE["gates"] if g["blocking"] and not g["ok"]]
    if args.dry_run:
        return dry_run(args)
    if blocked and not args.force and not args.wait_quiet:
        say("BLOCKED by %d gate(s): %s" % (len(blocked), [g["name"] for g in blocked]))
        say("fix them, pass --wait-quiet to wait for a quiet host, or --force to run anyway")
        return 1
    if args.preflight:
        say("PREFLIGHT %s" % ("OK" if not blocked else "BLOCKED"))
        return 0 if not blocked else 1
    return real_run(args)


if __name__ == "__main__":
    sys.exit(main())
