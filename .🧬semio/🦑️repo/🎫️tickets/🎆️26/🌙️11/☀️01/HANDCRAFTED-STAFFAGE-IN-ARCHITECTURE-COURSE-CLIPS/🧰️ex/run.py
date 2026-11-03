import json, subprocess, sys, time, re
from pathlib import Path

ROOT = Path("C:/git/semio")
T = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
S = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage"
V = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals"
EXP = S / "🔨️modules/📤️export/🎭️cast/🟦️.ts"
MAN = V / "🧍️staffage/🔣️.json"
OUT = T / "🗑️generated/ex"
RES = T / "🧰️ex/results.json"
BATCH = int(sys.argv[1])
SKIP_DONE = len(sys.argv) > 2 and sys.argv[2] == "resume"

ids = sorted(json.loads(MAN.read_text(encoding="utf8"))["casts"])
results = json.loads(RES.read_text(encoding="utf8")) if SKIP_DONE and RES.exists() else {}
ids = [i for i in ids if not results.get(i, {}).get("ok")]
OUT.mkdir(parents=True, exist_ok=True)

for i in range(0, len(ids), BATCH):
    batch = ids[i:i + BATCH]
    t0 = time.time()
    p = subprocess.Popen(["repo", "tools", "exec", "bun", "--", str(EXP), "--manifest", str(MAN), "--ids", ",".join(batch), "--out", str(OUT)],
                         cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf8", errors="replace")
    marks, err = [], []
    for line in p.stderr:
        err.append(line)
        m = re.match(r"\[cast (\d+)/(\d+)\] (\S+)", line)
        if m:
            marks.append((m.group(3), time.time()))
    p.wait()
    end = time.time()
    peak = next((l.split()[1] for l in err if l.startswith("[peak]")), None)
    for k, (cid, ts) in enumerate(marks):
        nxt = marks[k + 1][1] if k + 1 < len(marks) else end
        idx = OUT / cid / "🔣️.json"
        r = {"ok": idx.exists(), "wall": round(nxt - ts, 1), "batchPeakMB": peak, "batchExit": p.returncode}
        if r["ok"]:
            j = json.loads(idx.read_text(encoding="utf8"))
            r.update(frames=len(j["frames"]), fps=j["fps"], durationS=j["durationS"])
        results[cid] = r
    for cid in batch:
        if cid not in results or not (OUT / cid / "🔣️.json").exists():
            results.setdefault(cid, {"ok": False})["ok"] = False
            results[cid]["err"] = "".join(err)[-1200:]
    print("batch", i, "exit", p.returncode, "wall", round(end - t0), "peak", peak, flush=True)
    for cid in batch:
        r = results[cid]
        print(" ", cid, {k: v for k, v in r.items() if k != "err"}, flush=True)
        if not r["ok"]:
            print("   ERR", r.get("err", "")[-500:], flush=True)
    RES.write_text(json.dumps(results, indent=1, ensure_ascii=False), encoding="utf8")
