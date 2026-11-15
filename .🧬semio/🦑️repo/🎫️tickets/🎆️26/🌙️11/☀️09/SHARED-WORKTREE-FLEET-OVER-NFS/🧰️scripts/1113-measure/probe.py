import os, sys, time, json, subprocess, glob

ROOT = "/Users/Shared/semio"
TICKET = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
GEN = TICKET + "/🗑️generated"
SEG = ROOT + "/../semio-store/v1/origins/g1-server/segments"
SEG = "/Users/Shared/semio-store/v1/origins/g1-server/segments"
HINTS = ROOT + "/.cache/f/hints"
SCRATCH = TICKET + "/🧰️scripts/1113-measure"
POLL = 0.05
LIMIT = 120.0

def newest_segment():
    files = glob.glob(SEG + "/*")
    if not files:
        return None
    return max(files, key=lambda f: (os.path.getmtime(f), f))

def size_of(path):
    try:
        return os.path.getsize(path)
    except OSError:
        return -1

def contains_since(path, offset, needle):
    try:
        with open(path, "rb") as f:
            f.seek(offset)
            data = f.read()
        return needle in data, len(data)
    except OSError:
        return False, 0

def wait_publish(needle, start_seg_path, start_seg_size, t0, hint_path=None):
    grew_at = None
    seen_at = None
    hint_gone_at = None
    seg = start_seg_path
    off = start_seg_size
    deadline = t0 + LIMIT
    while True:
        now = time.time()
        cur = newest_segment()
        if cur != seg:
            # segment rolled over: scan the old one from its offset, then switch
            if seg and os.path.exists(seg):
                hit, _ = contains_since(seg, off, needle.encode())
                if hit and seen_at is None:
                    seen_at = now
            seg, off = cur, 0
        size = size_of(seg) if seg else -1
        if size > off and grew_at is None:
            grew_at = now
        if seen_at is None and seg:
            hit, n = contains_since(seg, off, needle.encode())
            if hit:
                seen_at = now
        if hint_path and hint_gone_at is None and not os.path.exists(hint_path):
            hint_gone_at = now
        if seen_at is not None and (hint_path is None or hint_gone_at is not None):
            break
        if now > deadline:
            break
        time.sleep(POLL)
    return {
        "grew_ms": None if grew_at is None else round((grew_at - t0) * 1000, 1),
        "contains_ms": None if seen_at is None else round((seen_at - t0) * 1000, 1),
        "timeout": seen_at is None,
        "hint_consumed_ms": None if hint_gone_at is None else round((hint_gone_at - t0) * 1000, 1),
        "segment": os.path.basename(seg) if seg else None,
    }

def run_probe(n, mode, run_id):
    d = os.environ.get("PROBE_DIR", GEN)
    os.makedirs(d, exist_ok=True)
    name = f"probe-{n}.txt"
    path = os.path.join(d, name)
    token = f"semio-probe {run_id} {mode} {n}"
    seg = newest_segment()
    start_size = size_of(seg) if seg else 0
    hint_path = None
    t0 = time.time()
    with open(path, "w", encoding="utf-8") as f:
        f.write(token + "\n")
    if mode == "hint":
        payload = json.dumps({"tool_name": "Write", "tool_input": {"file_path": path}}).encode()
        env = {**os.environ, "CLAUDE_PROJECT_DIR": ROOT}
        cmd = 'd="$CLAUDE_PROJECT_DIR/.cache/f/hints";[ -d "$d" ]&&cat >"$d/.h$$"&&mv "$d/.h$$" "$d/h$$-$(date +%s).hook";exit 0'
        before = set(os.listdir(HINTS))
        subprocess.run(["bash", "-c", cmd], input=payload, env=env, check=False)
        created = sorted(set(os.listdir(HINTS)) - before)
        hint_path = os.path.join(HINTS, created[0]) if created else None
        hint_created = bool(created)
    else:
        hint_created = None
    res = wait_publish(name, seg, start_size, t0, hint_path)
    res.update({"n": n, "mode": mode, "file": name, "hint_created": hint_created})
    return res

if __name__ == "__main__":
    mode = sys.argv[1]
    ids = [int(x) for x in sys.argv[2].split(",")]
    run_id = sys.argv[3]
    os.makedirs(SCRATCH, exist_ok=True)
    out = open(os.path.join(SCRATCH, f"probe-{mode}-{run_id}.jsonl"), "a")
    for n in ids:
        r = run_probe(n, mode, run_id)
        out.write(json.dumps(r) + "\n")
        out.flush()
        print(json.dumps(r))
        time.sleep(1.0)
    out.close()
