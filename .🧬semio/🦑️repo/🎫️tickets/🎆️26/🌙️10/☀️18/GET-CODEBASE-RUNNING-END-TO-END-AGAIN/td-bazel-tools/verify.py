"""⚖️ [td-bazel] runs the laws of one staged batch on a root (a mirror with the batch applied, or its base for the red run): `plain` = bare `bun test ./file`, `env` = the graph-free official build envelope (brun.ts), `bazel` = env with SEMIO_TEST_BAZEL_BINARY (the managed live Bazel unless the variable is set). Prints one line per law file with pass/fail and failing names; exit 1 when any law file fails. usage: python3 -I verify.py <batch> <root>"""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/"
S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/"
LAWS = {
 "starlark": [("bazel", B + "🧪️tests/📥️platform/🟦️.ts"), ("plain", B + "🧪️tests/📥️coverage/🟦️.ts"), ("bazel", B + "🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts"), ("plain", B + "👷️workers/🧪️tests/🟦️.ts")],
 "scopes": [("plain", B + "🎬️scopes/🧪️tests/🟦️.ts"), ("plain", S + "🟦️.ts"), ("plain", S + "🧵️unit-cache-lanes/🟦️.ts")],
 "stale": [("env", B + "🧪️tests/📥️control/🟦️.ts"), ("plain", B + "🧪️tests/📥️purpose/🧪️tests/🟦️.ts"), ("env", B + "🧪️tests/📥️purpose/🧪️tests/🟦️.ts"), ("env", B + "🧪️tests/🔏️implementation/🟦️.ts"), ("bazel", B + "🧪️tests/📥️paths/🟦️.ts"), ("bazel", B + "💾️storage/🧪️tests/🟦️.ts"), ("bazel", B + "📸️snapshot/🧪️tests/🔒️planned/🟦️.ts")],
}
def main():
    batch, root = sys.argv[1], os.path.realpath(sys.argv[2])
    bazel = os.environ.get("SEMIO_TEST_BAZEL_BINARY") or os.path.join(LIVE, ".cache/b/t/m/bazel/bin", sorted(os.listdir(os.path.join(LIVE, ".cache/b/t/m/bazel/bin")))[0], "bazel")
    bad = 0
    for mode, law in LAWS[batch]:
        env = {k: v for k, v in os.environ.items() if k not in ("SEMIO_TEST_ARTIFACT_DIR", "SEMIO_SCRIPT_PROCESS_INVOCATION")}
        if mode == "bazel": env["SEMIO_TEST_BAZEL_BINARY"] = bazel
        cmd = ["nice", "-n", "10", "bun", "test", "--timeout", "300000", "./" + law] if mode == "plain" else ["bun", os.path.join(HERE, "brun.ts"), "file", law]
        env["BR_ROOT"] = root; env["BR_WALL"] = "900"
        out = subprocess.run(cmd, cwd=root, env=env, capture_output=True, text=True, timeout=1200)
        text = out.stdout + out.stderr
        nums = dict((k, int(v)) for v, k in re.findall(r"^ *(\d+) (pass|fail|skip)$", text, re.M))
        fails = sorted(set(l[7:].split(" [")[0][:110] for l in text.splitlines() if l.startswith("(fail) ")))
        bad += bool(nums.get("fail") or not nums.get("pass"))
        print(f"{mode:5} {nums.get('pass', 0):3}p {nums.get('fail', 0):2}f {nums.get('skip', 0):2}s {law[len(B) if law.startswith(B) else 0:]} {' | '.join(fails)}", flush=True)
    sys.exit(1 if bad else 0)
main()
