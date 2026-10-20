#!/usr/bin/env python3
# Applies the staged set to the live repo after the write pause: refuses on drift (live hash != recorded base) or while a dependency sync runs; --dry-run prints only.
import hashlib, json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
B = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
S = HERE + "/staged"
sys.path.insert(0, HERE)

MODIFIED = {
    "script.ts": BZ + "/📜️script.ts",
    "schema.json": BZ + "/🧬️schema/🔣️.json",
    "selection.ts": B + "/🧭️selection/🟦️.ts",
    "neutral.ts": B + "/📜️script.ts",
    "catalog.json": B + "/⚙️operations/🔣️catalog.json",
    "actions-fixture.json": BZ + "/🧪️tests/📥️actions/🔣️.json",
    "actions-schema.json": BZ + "/🧪️tests/📥️actions/🧬️schema.json",
    "actions-test.ts": BZ + "/🧪️tests/📥️actions/🟦️.ts",
    "arguments-test.ts": BZ + "/🧪️tests/📤️arguments/🟦️.ts",
    "selection-test.ts": B + "/🧭️selection/🧪️tests/🟦️.ts",
}
NEW = {
    "commands.ts": BZ + "/🧭️commands/🟦️.ts",
    "commands-schema.json": BZ + "/🧭️commands/🧬️schema/🔣️.json",
    "commands-test.ts": BZ + "/🧭️commands/🧪️tests/🟦️.ts",
    "commands-fixture.json": BZ + "/🧭️commands/🧪️tests/🧫️fixtures/🔣️.json",
    "affected.ts": BZ + "/🎯️affected/🟦️.ts",
    "affected-schema.json": BZ + "/🎯️affected/🧬️schema/🔣️.json",
    "affected-test.ts": BZ + "/🎯️affected/🧪️tests/🟦️.ts",
    "affected-fixture.json": BZ + "/🎯️affected/🧪️tests/🧫️fixtures/🔣️.json",
    "testrule-test.ts": BZ + "/🧪️tests/📥️test-actions/🟦️.ts",
}
ALIAS = {HERE + "/run/selection.ts": MODIFIED["selection.ts"], HERE + "/run/adapter.ts": MODIFIED["script.ts"]}
for k, v in NEW.items(): ALIAS[S + "/" + k] = v
SPEC = re.compile(r"""(from\s*|import\s*\(\s*)(["'])(\.{1,2}/[^"']+)\2""")

def sha(data): return hashlib.sha256(data).hexdigest()

def rewrite(text, staged_dir, dest):
    def fix(m):
        target = os.path.normpath(os.path.join(staged_dir, m.group(3)))
        target = ALIAS.get(target, target)
        rel = os.path.relpath(target, os.path.dirname(dest))
        if not rel.startswith("."): rel = "./" + rel
        return m.group(1) + m.group(2) + rel + m.group(2)
    return SPEC.sub(fix, text)

def main():
    dry = "--dry-run" in sys.argv
    bases = json.load(open(S + "/bases.json"))
    refused = []
    if not dry and subprocess.run(["pgrep", "-f", "dependencies (sync|lock)"], capture_output=True).returncode == 0:
        refused.append("a dependency sync/lock is running")
    for key, live in MODIFIED.items():
        current = sha(open(live, "rb").read())
        if current != bases[key]:
            refused.append("drift: %s (live %s != base %s); run build-staged.py again" % (live[len(ROOT) + 1:], current[:12], bases[key][:12]))
    for key, dest in NEW.items():
        if os.path.exists(dest):
            existing = open(dest, encoding="utf-8").read()
            wanted = rewrite(open(S + "/" + key, encoding="utf-8").read(), S, dest) if key.endswith(".ts") else open(S + "/" + key, encoding="utf-8").read()
            if existing != wanted: refused.append("new file exists with other content: " + dest[len(ROOT) + 1:])
    if refused:
        print("\n".join("REFUSED " + r for r in refused)); sys.exit(1)
    for key, live in MODIFIED.items():
        print(("would write " if dry else "write ") + live[len(ROOT) + 1:])
        if not dry:
            open(live, "w", encoding="utf-8").write(open(S + "/" + key, encoding="utf-8").read())
    for key, dest in NEW.items():
        print(("would write " if dry else "write ") + dest[len(ROOT) + 1:])
        if not dry:
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            text = open(S + "/" + key, encoding="utf-8").read()
            open(dest, "w", encoding="utf-8").write(rewrite(text, S, dest) if key.endswith(".ts") else text)

if __name__ == "__main__": main()
