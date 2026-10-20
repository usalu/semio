#!/usr/bin/env python3
# Measures one generated Bazel workspace: analysis (cold/warm server), execution (cold/warm/after clean/after server+output-base loss) with Bazel's own BEP metrics.
import json, os, shutil, subprocess, sys, time, re
ROOT = "/Users/ueli/Documents/semio"
BZ = ROOT + "/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"
TMP = "/private/tmp/bzlazy"
ENV = dict(os.environ, LANG="en_US.UTF-8", LC_ALL="en_US.UTF-8")

def phases(path):
    import gzip
    try:
        d = json.load(gzip.open(path))
    except Exception:
        return {}
    mk = {e["name"]: e["ts"] / 1000 for e in d["traceEvents"] if e.get("cat") == "build phase marker"}
    r = {}
    if "Evaluate target patterns" in mk and "Initialize command" in mk: r["initMs"] = int(mk["Evaluate target patterns"] - mk["Initialize command"])
    if "Load and analyze dependencies" in mk and "Evaluate target patterns" in mk: r["loadMs"] = int(mk["Load and analyze dependencies"] - mk["Evaluate target patterns"])
    keys = [k for k in mk if k in ("Complete build", "Prepare for build", "Execute actions")]
    if "Load and analyze dependencies" in mk:
        nxt = [mk[k] for k in mk if mk[k] > mk["Load and analyze dependencies"]]
        if nxt: r["analyzeMs"] = int(min(nxt) - mk["Load and analyze dependencies"])
    return r

def stub(ws):
    p = ws + "/tasks.bzl"
    t = open(p).read()
    if "run_shell" in t: return
    a = 'ctx.actions.run(executable = ctx.attr.bun, arguments = args, inputs'
    assert a in t, "task rule shape changed"
    t = t.replace(a, 'ctx.actions.run_shell(command = "mkdir -p \\"$1\\" && echo ok > \\"$1/snapshot.json\\"", arguments = [output.path], inputs')
    t = t.replace('outputs = [output], mnemonic = "RepoTask"', 'outputs = [output], mnemonic = "RepoTask"')
    open(p, "w").write(t)

class Ws:
    def __init__(self, name, ws):
        self.name, self.ws = name, ws
        self.ob = f"{TMP}/ob-{name}"
        self.dc = f"{TMP}/dc-{name}"
        self.bep = f"{TMP}/{name}.bep"
        self.prof = f"{TMP}/{name}.prof.gz"
    def startup(self):
        return [f"--output_base={self.ob}", f"--install_base={TMP}/install", "--max_idle_secs=900", "--host_jvm_args=-Xmx" + os.environ.get("XMX", "2048m"), "--nosystem_rc", "--nohome_rc", f"--bazelrc={ROOT}/.bazelrc"]
    def opts(self):
        return ["--lockfile_mode=off", "--incompatible_strict_action_env", "--incompatible_autoload_externally=", f"--disk_cache={self.dc}", "--jobs=4", "--keep_going", f"--build_event_json_file={self.bep}", f"--profile={self.prof}"]
    def run(self, verb, targets, extra=()):
        if os.path.exists(self.bep): os.remove(self.bep)
        if os.path.exists(self.prof): os.remove(self.prof)
        t = time.time()
        p = subprocess.run([BZ, *self.startup(), verb, *self.opts(), *extra, *targets], cwd=self.ws, env=ENV, capture_output=True, text=True)
        wall = int((time.time() - t) * 1000)
        out = p.stdout + p.stderr
        m = {"rc": p.returncode, "wallMs": wall}
        bm = None
        if os.path.exists(self.bep):
            for l in open(self.bep):
                try: d = json.loads(l)
                except Exception: continue
                if "buildMetrics" in d: bm = d["buildMetrics"]
        if bm:
            tm = bm.get("timingMetrics", {})
            m["analysisMs"] = int(tm.get("analysisPhaseTimeInMs", 0))
            m["executionMs"] = int(tm.get("executionPhaseTimeInMs", 0))
            m["cpuMs"] = int(tm.get("cpuTimeInMs", 0))
            m["packages"] = int(bm.get("packageMetrics", {}).get("packagesLoaded", 0))
            m["configured"] = int(bm.get("targetMetrics", {}).get("targetsConfigured", 0))
            a = bm.get("actionSummary", {})
            m["actionsCreated"] = int(a.get("actionsCreated", 0))
            rc = {r["name"]: int(r.get("count", 0)) for r in a.get("runnerCount", [])}
            m["actions"] = rc.get("total", 0)
            m["hits"] = sum(v for k, v in rc.items() if k.endswith("cache hit"))
            m["internal"] = rc.get("internal", 0)
            m["executed"] = sum(v for k, v in rc.items() if k != "total" and k != "internal" and not k.endswith("cache hit"))
        m.update(phases(self.prof))
        m["uploadSkipped"] = len(re.findall(r"Skipping uploading", out))
        mm = re.search(r"(\d+) processes?: (.*)", out)
        m["summary"] = mm.group(0) if mm else ""
        m["tail"] = out.strip().splitlines()[-1][:200] if out.strip() else ""
        return m
    def kill(self):
        subprocess.run([BZ, *self.startup(), "shutdown"], cwd=self.ws, env=ENV, capture_output=True)
    def wipe_ob(self):
        self.kill(); subprocess.run(["chmod", "-R", "u+w", self.ob], capture_output=True); shutil.rmtree(self.ob, ignore_errors=True)
    def wipe_dc(self):
        shutil.rmtree(self.dc, ignore_errors=True)

def main():
    name, ws, label = sys.argv[1], sys.argv[2], sys.argv[3]
    mode = sys.argv[4] if len(sys.argv) > 4 else "all"
    w = Ws(name, ws)
    if os.environ.get("STUB", "1") == "1": stub(ws)
    res = {"name": name, "label": label}
    w.wipe_ob(); w.wipe_dc()
    res["analysis_cold"] = w.run("build", [label], ["--nobuild"])
    res["analysis_warm"] = w.run("build", [label], ["--nobuild"])
    if mode != "analysis":
        w.wipe_ob(); w.wipe_dc()
        res["exec_cold"] = w.run("build", [label])
        res["exec_warm"] = w.run("build", [label])
        subprocess.run([BZ, *w.startup(), "clean"], cwd=ws, env=ENV, capture_output=True)
        res["after_clean"] = w.run("build", [label])
        w.wipe_ob()
        res["after_server_loss"] = w.run("build", [label])
        res["aquery"] = aq(w, label)
        res["closure"] = closure(w, label)
    w.kill()
    print(json.dumps(res))

def closure(w, label):
    p = subprocess.run([BZ, *w.startup(), "cquery", "--lockfile_mode=off", "--incompatible_autoload_externally=", f'kind("repo_", deps({label}))', "--output=label"], cwd=w.ws, env=ENV, capture_output=True, text=True)
    labels = sorted(set(re.findall(r"//:(t_[0-9a-f]{16})", p.stdout)))
    return labels

def aq(w, label):
    p = subprocess.run([BZ, *w.startup(), "aquery", "--lockfile_mode=off", "--incompatible_autoload_externally=", f"deps({label})", "--output=jsonproto"], cwd=w.ws, env=ENV, capture_output=True, text=True)
    try:
        d = json.loads(p.stdout)
    except Exception:
        return {"error": p.stderr[-200:]}
    acts = d.get("actions", [])
    from collections import Counter
    c = Counter(a.get("mnemonic") for a in acts)
    return {"actions": len(acts), "mnemonics": dict(c)}

if __name__ == "__main__": main()
