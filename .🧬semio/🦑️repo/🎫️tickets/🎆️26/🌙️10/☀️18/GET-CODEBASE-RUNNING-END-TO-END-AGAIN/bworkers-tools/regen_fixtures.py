# 🧪 Renders the golden fixtures that depend on the staged code (the generated BUILD file and the help snapshots) by running the staged code in the overlay, then stages the results.
import json, os, subprocess, sys
import lib

OVERLAY = os.environ.get("BW_OVERLAY", "/private/tmp/bworkers/r")


def run(script):
    target = os.path.join(OVERLAY, os.path.basename(script))
    with open(os.path.join(lib.TOOLS, "regen", script), encoding="utf8") as handle, open(target, "w", encoding="utf8") as out:
        out.write(handle.read())
    env = dict(os.environ, BUN_RUNTIME_TRANSPILER_CACHE_PATH="0")
    result = subprocess.run(["bun", target], cwd=OVERLAY, capture_output=True, text=True, env=env)
    os.remove(target)
    if result.returncode != 0:
        sys.exit(result.stderr[-2000:])
    return result.stdout


# BUILD golden: only the mnemonic and multiplex attributes of task rules may differ from the authored golden
lines = json.loads(run("build-lines.ts"))
path = lib.BAZEL + "/" + lib.TESTS + "/\U0001F4E5️actions/" + lib.JSON_NAME
raw = lib.read(path)
start = raw.index('  "build": [')
end = raw.index("\n  ]", start) + 4
new = raw[:start] + '  "build": [\n' + ",\n".join("    " + json.dumps(line, ensure_ascii=False) for line in lines) + "\n  ]" + raw[end:]
json.loads(new)
lib.write_staged("tests-actions-fixture.json", new)
impl = lib.read(lib.BAZEL + "/" + lib.TESTS + "/\U0001F50F️implementation/" + lib.JSON_NAME)
assert impl.count('"kind": "birth-action"') == 1
lib.write_staged("tests-implementation-fixture.json", impl.replace('"kind": "birth-action"', '"kind": "work-action"'))

# help snapshots
snapshots = json.loads(run("help-snapshots.ts"))
lib.write_staged("help-fixture.json", json.dumps(snapshots, indent=2, ensure_ascii=False))
print("regenerated: BUILD golden", len(lines), "lines; help snapshots", sum(len(v) for v in snapshots["snapshots"].values()))
