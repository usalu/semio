import subprocess, sys, time, os
name, outpath, cmd = sys.argv[1], sys.argv[2], sys.argv[3]
t0 = time.time()
p = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica", env={**os.environ, "SEMIO_WORKSPACE_ROOT": "/Users/Shared/semio"})
dt = time.time() - t0
with open(outpath, "w") as f:
    f.write(p.stdout)
    if p.stderr:
        f.write("\n---STDERR---\n" + p.stderr)
print(f"{name} wall_s={dt:.2f} exit={p.returncode} stdout_bytes={len(p.stdout)} stderr_bytes={len(p.stderr)}")
