#!/usr/bin/env python3
"""🔎️ Bisects which earlier test of the kept renderer wgpu lib binary poisons a later one in a serial run: runs `<prefix> + victim`
(`--exact`, one thread, libtest's alphabetical order) and halves the prefix until the single culprit whose presence turns the
victim red remains. Usage: renderer_bisect.py <victim test name>"""
import subprocess
import sys

BINARY = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-target/r6-live-renderer/semio_framework_os_renderer_wgpu-4b607c95eb950a60"
ENV = {"RUST_MIN_STACK": "67108864", "PATH": "/usr/bin:/bin"}


def names():
    out = subprocess.run([BINARY, "--list", "--format", "terse"], capture_output=True, text=True, env=ENV).stdout
    return sorted(line[: -len(": test")] for line in out.splitlines() if line.endswith(": test"))


def victim_red(prefix, victim):
    run = subprocess.run(["nice", "-n", "15", BINARY, "--exact", *prefix, victim, "--test-threads", "1"], capture_output=True, text=True, env=ENV, timeout=600)
    return f"test {victim} ... FAILED" in run.stdout or f"test {victim} ... ok" not in run.stdout


def main():
    victim = sys.argv[1]
    ordered = names()
    prefix = ordered[: ordered.index(victim)]
    print(f"prefix {len(prefix)} tests; victim red after full prefix: {victim_red(prefix, victim)}; alone: {victim_red([], victim)}", flush=True)
    low, high = 0, len(prefix)
    while high - low > 1:
        middle = (low + high) // 2
        if victim_red(prefix[:middle], victim):
            high = middle
        else:
            low = middle
        print(f"  window [{low}, {high})", flush=True)
    culprit = prefix[low]
    print(f"CULPRIT {culprit}; victim red after culprit alone: {victim_red([culprit], victim)}", flush=True)


if __name__ == "__main__":
    main()
