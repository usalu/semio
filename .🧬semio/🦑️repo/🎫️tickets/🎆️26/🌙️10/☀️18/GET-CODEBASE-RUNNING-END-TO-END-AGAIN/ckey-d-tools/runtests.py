"""python3 runtests.py <timeoutSeconds> <file>...  runs each file with bun test (nice 10) and prints one summary line."""
import subprocess, sys, re, time, os
limit = int(sys.argv[1])
for f in sys.argv[2:]:
    start = time.time()
    try:
        r = subprocess.run(["nice", "-n", "10", "bun", "test", "./" + f], capture_output=True, text=True, timeout=limit, stdin=subprocess.DEVNULL)
        out = r.stdout + r.stderr
        passed = re.findall(r"^\s*(\d+) pass", out, re.M); failed = re.findall(r"^\s*(\d+) fail", out, re.M)
        first = next((l for l in out.splitlines() if "error:" in l or "(fail)" in l), "")
        print(f"{round(time.time()-start):4d}s rc={r.returncode} pass={passed[-1] if passed else '?'} fail={failed[-1] if failed else '?'} {f.split('🏗️bazel/')[-1]} {first[:150]}", flush=True)
    except subprocess.TimeoutExpired:
        print(f"TIMEOUT {f}", flush=True)
