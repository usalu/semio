import os, subprocess, sys, time, random, json, signal
ws, shared = sys.argv[1], sys.argv[2]
rounds = int(sys.argv[3]); nproc = int(sys.argv[4]); stall = int(sys.argv[5]); mode = sys.argv[6] if len(sys.argv) > 6 else "shared"
cargo = "/Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/cargo"
tops = [f"t{i}" for i in range(8)]
log = open(os.path.join(ws, "..", "repro.log"), "a")
def say(m): log.write(time.strftime("%H:%M:%S ") + m + "\n"); log.flush(); print(m, flush=True)
def children(pid):
    out = subprocess.run(["pgrep", "-P", str(pid)], capture_output=True, text=True).stdout.split()
    return [int(x) for x in out]
def tree_cpu(pid):
    total = 0.0; stack = [pid]; n = 0
    while stack:
        p = stack.pop(); n += 1
        r = subprocess.run(["ps", "-o", "%cpu=", "-p", str(p)], capture_output=True, text=True).stdout.strip()
        total += float(r or 0); stack += children(p)
    return total, n
for rnd in range(rounds):
    src = os.path.join(ws, "l0", "src", "lib.rs")
    open(src, "a").write("// round %d\n" % rnd)
    procs = []
    for i in range(nproc):
        env = dict(os.environ)
        env["PATH"] = "/Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin" + ":" + env["PATH"]
        env["LOCKC_VARIANT"] = str(rnd % 2 + i % 2)
        if mode == "shared":
            env["CARGO_TARGET_DIR"] = os.path.join(shared, "t"); env["CARGO_BUILD_BUILD_DIR"] = os.path.join(shared, "b")
        else:
            env["CARGO_TARGET_DIR"] = os.path.join(shared, f"lane{i}", "t"); env["CARGO_BUILD_BUILD_DIR"] = os.path.join(shared, f"lane{i}", "b")
        pkgs = random.sample(tops, 3)
        args = [cargo, "build", "-j2", "--manifest-path", os.path.join(ws, "Cargo.toml")] + sum([["-p", p] for p in pkgs], [])
        if i % 2: args += ["--features", "f1"] if False else []
        procs.append((subprocess.Popen(args, env=env, cwd=ws, stdout=subprocess.DEVNULL, stderr=open(os.path.join(ws, "..", f"err-{rnd}-{i}.log"), "w"), start_new_session=True), pkgs))
        time.sleep(random.random() * 0.8)
    idle = {}
    while any(p.poll() is None for p, _ in procs):
        time.sleep(3)
        alive = [(p, k) for p, k in procs if p.poll() is None]
        stuck = []
        for p, k in alive:
            cpu, n = tree_cpu(p.pid)
            if cpu < 0.5 and n == 1:
                idle[p.pid] = idle.get(p.pid, 0) + 3
            else:
                idle[p.pid] = 0
            if idle[p.pid] >= stall: stuck.append(p.pid)
        if stuck and len(stuck) == len(alive):
            say(f"DEADLOCK round {rnd}: all {len(alive)} alive cargos idle >= {stall}s with no child: {stuck}")
            for pid in stuck:
                subprocess.run(["sample", str(pid), "2", "-file", os.path.join(ws, "..", f"sample-{rnd}-{pid}.txt")], capture_output=True)
            for pid in stuck:
                try: os.kill(pid, signal.SIGKILL)
                except Exception: pass
            sys.exit(3)
    rcs = [p.returncode for p, _ in procs]
    say(f"round {rnd} done rcs={rcs}")
say("no deadlock")
