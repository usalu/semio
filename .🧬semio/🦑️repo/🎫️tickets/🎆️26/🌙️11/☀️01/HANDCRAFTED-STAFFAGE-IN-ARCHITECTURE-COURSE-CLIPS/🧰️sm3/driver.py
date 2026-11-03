import json, os, subprocess, sys, time
from pathlib import Path
import psutil
R = Path("C:/git/semio")
T = next((R / ".🧬semio/🦑️repo/🎫️tickets").glob("*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"))
G = T / "🗑️generated/sm3"
G.mkdir(parents=True, exist_ok=True)
res = G / "results.jsonl"
CACHE = R / ".🧬semio/🎓️teaching/⚡️cache"
PROJECT = R / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
LIMIT = 5 * 1024 ** 3
scene_timeout, files = sys.argv[1], sys.argv[2].split("|")
only = sys.argv[3] if len(sys.argv) > 3 else ""
def log(m):
    with (G / "progress.log").open("a", encoding="utf-8") as p:
        p.write(time.strftime("%H:%M:%S ") + m + "\n")
def rows():
    if not res.exists():
        return []
    return [json.loads(l) for l in res.read_text(encoding="utf-8").splitlines()]
def key(d):
    return Path(d["file"]).parent.name + ":" + d["scene"]
def tree_rss(proc):
    try:
        ps = [proc] + proc.children(recursive=True)
        return sum(p.memory_info().rss for p in ps if p.is_running())
    except psutil.Error:
        return 0
def kill_tree(proc):
    for p in proc.children(recursive=True) + [proc]:
        try:
            p.kill()
        except psutil.Error:
            pass
env = dict(os.environ, SM_MEDIA=str(G / "media"), PYTHONUTF8="1", PYTHONIOENCODING="utf-8", PYTHONDONTWRITEBYTECODE="1", UV_PROJECT_ENVIRONMENT=str(CACHE / "python"),
           SEMIO_TEACHING_LANGUAGE=os.environ.get("SEMIO_TEACHING_LANGUAGE", "en"), SEMIO_TEACHING_AUDIENCE="speaker", SEMIO_TEACHING_OUTPUT_ROOT=str(CACHE / "tests"),
           SEMIO_TEACHING_CACHE_DIR=str(CACHE / "fonts"), SM_SCENE_TIMEOUT=scene_timeout)
base = len(rows())
for attempt in range(40):
    done = {key(d) for d in rows()[base:] if d["status"] in ("pass", "timeout", "memkill", "fail") and d["file"].replace("\\", "/").split("architecture/")[-1] in {f.split("architecture/")[-1] for f in files}}
    cmd = ["uv", "run", "--project", str(PROJECT), "--python", "3.12", "python", str(T / "🧰️sm3/worker.py"), "|".join(str(R / f) for f in files), str(res), only, ",".join(sorted(done))]
    log(f"attempt {attempt} files={len(files)} skip={sorted(done)}")
    with (G / "last.log").open("ab") as out:
        proc = psutil.Popen(cmd, cwd=str(R), env=env, stdin=subprocess.DEVNULL, stdout=out, stderr=subprocess.STDOUT)
        peak = 0
        killed = False
        while proc.poll() is None:
            time.sleep(3)
            rss = tree_rss(proc)
            peak = max(peak, rss)
            if rss > LIMIT:
                kill_tree(proc)
                killed = True
                break
    log(f"exit rc={proc.poll()} peak={peak >> 20}MB killed={killed}")
    rs = rows()
    last = rs[-1] if rs else {}
    if killed:
        with res.open("a", encoding="utf-8") as f:
            f.write(json.dumps({"scene": last.get("scene", "?"), "status": "memkill", "file": last.get("file", ""), "peak_mb": peak >> 20}, ensure_ascii=False) + "\n")
        continue
    if last.get("status") == "timeout":
        continue
    break
log("DONE")
