"""Sequential per-project inference census: chunks of projects run by infer-one.mjs (faithful session); a chunk whose RSS or wall time passes a cap is killed (own pid only) and the project it was inside is recorded as heavy, the rest of the chunk continues."""
import json, os, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/infer-f"
BUN = ROOT + "/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
roots = [l for l in open(sys.argv[1], encoding="utf8").read().split("\n") if l]
chunk, rss_cap_kb, wall_cap = int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
out = open(sys.argv[5], "a", encoding="utf8")
env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}
env.update(INFER_F_FAITHFUL="1", BUN_JSC_forceRAMSize="3758096384")
def rss(pid):
    try: return int(subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True).stdout.strip() or 0)
    except Exception: return 0
pending = list(roots)
while pending:
    part, pending = pending[:chunk], pending[chunk:]
    listing = G + "/census-part.txt"; open(listing, "w", encoding="utf8").write("\n".join(part) + "\n")
    log = G + "/census-part.log"; open(log, "w").close()
    p = subprocess.Popen(["nice", "-n", "10", BUN, C + "/infer-f-tools/infer-one.mjs", "@list:" + listing], stdout=open(log, "ab"), stderr=subprocess.STDOUT, env=env, cwd=ROOT)
    start = time.time(); last = None; heavy = None
    while p.poll() is None:
        time.sleep(2)
        text = open(log, "rb").read().decode("utf8", "replace")
        starts = [l[len("[DEBUG] start "):] for l in text.split("\n") if l.startswith("[DEBUG] start ")]
        if starts and starts[-1] != last: last = starts[-1]; mark = time.time()
        if last is not None and (rss(p.pid) > rss_cap_kb or time.time() - mark > wall_cap):
            heavy = last; p.kill(); p.wait(); break
    text = open(log, "rb").read().decode("utf8", "replace")
    done = []
    for l in text.split("\n"):
        if l.startswith("[DEBUG] {"):
            row = json.loads(l[len("[DEBUG] "):]); out.write(json.dumps(row, ensure_ascii=False) + "\n"); done.append(row["root"])
    out.flush()
    if heavy is not None:
        out.write(json.dumps({"heavy": heavy, "rssKB": rss_cap_kb, "wallSeconds": wall_cap, "chunkSeconds": round(time.time() - start)}, ensure_ascii=False) + "\n"); out.flush()
        started = [l[len("[DEBUG] start "):] for l in text.split("\n") if l.startswith("[DEBUG] start ")]
        seen = set(started)
        pending = [r for r in part if r[-70:] not in {x[-70:] for x in seen}] + pending
    elif p.returncode not in (0, None):
        out.write(json.dumps({"crash": part[0][-80:], "code": p.returncode, "tail": text[-300:]}, ensure_ascii=False) + "\n"); out.flush()
        started = [l[len("[DEBUG] start "):] for l in text.split("\n") if l.startswith("[DEBUG] start ")]
        seen = set(started[:-1]) if started else set()
        pending = [r for r in part if r not in seen and not (started and r.endswith(started[-1]))] + pending
out.write(json.dumps({"finished": True}) + "\n"); out.close()
