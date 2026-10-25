import os, subprocess, sys, time, signal
ws, shared, rounds, mode = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
cmdA, cmdB, targets = sys.argv[5], sys.argv[6], sys.argv[7]
cargo = "/Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo"
S = os.path.dirname(ws)
def run(env_extra, lane, wait=True, sub="build", tgt=None):
    env = dict(os.environ); env.update(env_extra); env["PATH"] = "/Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin" + ":" + env["PATH"]
    base = shared if lane is None else os.path.join(shared, lane)
    env["CARGO_TARGET_DIR"] = os.path.join(base, "t" + (tgt or "")); env["CARGO_BUILD_BUILD_DIR"] = os.path.join(base, "b")
    cmd = [cargo, sub, "--manifest-path", os.path.join(ws, "Cargo.toml"), "-p", "t"]
    if wait:
        p = subprocess.Popen(cmd, env=env, cwd=ws, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True); p.wait(); return p
    p = subprocess.Popen(["sh", "-c", 'read go; exec "$@"', "sh"] + cmd, env=env, cwd=ws, stdin=subprocess.PIPE, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    return p
for r in range(rounds):
    run({"XVAR": "0", "YVAR": "0", "TVAR": str(r)}, None)
    a = run({"XVAR": "1", "YVAR": "0", "TVAR": str(r)}, None if mode == "shared" else "laneA", False, cmdA, "A" if targets == "distinct" else None)
    b = run({"XVAR": "0", "YVAR": "1", "TVAR": str(r)}, None if mode == "shared" else "laneB", False, cmdB, "B" if targets == "distinct" else None)
    time.sleep(0.3)
    a.stdin.write(b"\n"); b.stdin.write(b"\n"); a.stdin.flush(); b.stdin.flush()
    t0 = time.time()
    while a.poll() is None or b.poll() is None:
        time.sleep(1)
        if time.time() - t0 > 25:
            pids = [p.pid for p in (a, b) if p.poll() is None]
            print(f"DEADLOCK round {r}: pids {pids} still alive after 25s (builds take ~5s)", flush=True)
            for pid in pids:
                for tool, cmd in (("lldb", ["lldb", "-p", str(pid), "--batch", "-o", "bt all", "-o", "detach"]), ("sample", ["sample", str(pid), "1", "-file", os.path.join(S, f"cycle-sample-{r}-{pid}.txt")])):
                    try:
                        out = subprocess.run(cmd, capture_output=True, text=True, timeout=40).stdout
                        open(os.path.join(S, f"cycle-{tool}-{r}-{pid}.txt"), "w").write(out)
                        print(tool, pid, "ok", len(out), flush=True)
                    except subprocess.TimeoutExpired:
                        print(tool, pid, "timed out", flush=True)
                print(subprocess.run(["lsof", "-p", str(pid)], capture_output=True, text=True).stdout.count("/.lock"), "unit lock files held", flush=True)
            for pid in pids:
                os.kill(pid, signal.SIGKILL)
            sys.exit(3)
    print(f"round {r} ok rc={a.returncode},{b.returncode} {time.time()-t0:.1f}s", flush=True)
print("no deadlock")
