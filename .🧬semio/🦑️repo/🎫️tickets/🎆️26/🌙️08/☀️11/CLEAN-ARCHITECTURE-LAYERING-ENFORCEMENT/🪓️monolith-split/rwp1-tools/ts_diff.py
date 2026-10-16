#!/usr/bin/env python3
# 🧪️ The renderer source-text readers (bun test and vitest) against the live tree (A) and the live tree with every WP1-owned directory restored from the pre-carve snapshot (B).
# usage: ts_diff.py <tag> [bun|vitest|both]
import json, os, re, subprocess, sys, shutil
from common import *
import uo

BUN = ['♻️frame-close', '🛑️scene-pointer-cancellation', '🧾️frame-action-ledger', '🖼️wgpu-raster-residency', '📨️browser-frame-transport', '🫀️settle-pump', '⏱️wgpu-worker-step-budget', '🔬️engine-contract', '🧲️scene-input-residency', '⏱️frame-latency', '⏱️wgpu-ui-turn-budget', '🧲️engine-surface-retention', '🖌️wgpu-document-owner-move', '🌳️wgpu-document-reconcile', '🫳️node-graph-gestures', '🪟️window-lifecycle-template-drag', '♻️hot-swap-session-handover', '🔗️hub-projection', '🚗️driver-editor', '🧪️node-graph-edit-rows']
VITEST = ['🧾️frame-action-ledger', '🫀️settle-pump', '🖱️wheel-application-point', '🔗️hub-projection', '🚗️driver-editor', '🌳️wgpu-document-reconcile', '🖼️wgpu-raster-residency', '🎮️wgpu-browser-input-wire', '🎮️wgpu-runtime-mailbox-admission', '🎯️presented-input-authority', '📮️wgpu-frame-deferred-ownership', '🔢️frame-generation-hold', '🛍️app-catalogue-attempt', '🪟️action-window-scope', '♻️frame-close', '🛑️scene-pointer-cancellation', '📨️browser-frame-transport', '🧲️scene-input-residency', '⏱️frame-latency']


def build_b(tag):
    B = RS + f"/ts-B-{tag}"
    uo.chain(B)
    uo.restore_owned(B)
    return B


def bun_run(base, name, log):
    p = base + "/" + ENG_REL + "/🧪️tests/" + name + "/🟦️.ts"
    if not os.path.exists(p):
        return None
    r = subprocess.run(["bun", "test", p], capture_output=True, text=True, cwd=os.path.dirname(p), timeout=900)
    out = r.stdout + r.stderr
    open(log, "a").write("=== " + name + "\n" + out + "\n")
    fails = sorted(set(re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+m?s\])?$", out, re.M)))
    m = re.search(r"(\d+) pass", out)
    n = re.search(r"(\d+) fail", out)
    return (int(m.group(1)) if m else 0, int(n.group(1)) if n else 0, fails)


def main():
    tag = sys.argv[1]
    what = sys.argv[2] if len(sys.argv) > 2 else "bun"
    B = build_b(tag)
    if what in ("bun", "both"):
        la, lb = f"{GEN}/rwp1-ts-{tag}-A.log", f"{GEN}/rwp1-ts-{tag}-B.log"
        open(la, "w").close()
        open(lb, "w").close()
        same = diff = 0
        for t in BUN:
            a, b = bun_run(REPO, t, la), bun_run(B, t, lb)
            if a == b:
                same += 1
            else:
                diff += 1
            print(("SAME " if a == b else "DIFF ") + t, "A", a and a[:2], "B", b and b[:2], ("newly failing: " + str(sorted(set(a[2]) - set(b[2]))[:6])) if a and b and a != b else "", flush=True)
        print("bun: same", same, "diff", diff)
    if what in ("vitest", "both"):
        cfg_rel = T_REL + "/🧪️tests/🎚️config/🟦️.ts"
        res = {}
        for label, base in (("A", REPO), ("B", B)):
            out = f"{GEN}/rwp1-vitest-{tag}-{label}.json"
            files = ["🧪️tests/" + f + "/🟦️.ts" for f in VITEST]
            r = subprocess.run([REPO + "/node_modules/.bin/vitest", "run", "--config", base + "/" + cfg_rel, "--reporter=json", "--outputFile=" + out, *files], capture_output=True, text=True, cwd=base + "/" + ENG_REL, timeout=3000)
            open(f"{GEN}/rwp1-vitest-{tag}-{label}.log", "w").write(r.stdout + r.stderr)
            try:
                d = json.load(open(out))
            except Exception as e:
                print(label, "vitest produced no report:", e)
                continue
            m = {}
            for f in d["testResults"]:
                nm = f["name"].split("/🧪️tests/")[-1]
                m[nm] = {t["fullName"]: t["status"] for t in f["assertionResults"]}
                if f.get("status") == "failed" and not f["assertionResults"]:
                    m[nm] = {"<file>": "failed: " + re.sub(r"[^ ']*🧰️[^ ']*", "<path>", (f.get("message") or ""))[:60]}
            res[label] = m
        if len(res) == 2:
            same = diff = 0
            for k in sorted(set(res["A"]) | set(res["B"])):
                if res["A"].get(k) == res["B"].get(k):
                    same += 1
                else:
                    diff += 1
                    a, b = res["A"].get(k) or {}, res["B"].get(k) or {}
                    print("DIFF", k, {x: (a.get(x), b.get(x)) for x in sorted(set(a) | set(b)) if a.get(x) != b.get(x)})
            print("vitest files:", same + diff, "same", same, "diff", diff)


if __name__ == "__main__":
    main()
