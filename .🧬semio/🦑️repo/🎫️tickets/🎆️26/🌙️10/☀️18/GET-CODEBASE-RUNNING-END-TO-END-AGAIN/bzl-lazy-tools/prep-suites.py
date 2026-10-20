#!/usr/bin/env python3
# Copies existing adapter test suites next to an adapter copy (live or staged) under /private/tmp so they run unchanged against it; specifiers leaving the adapter folder are re-pointed to the real library.
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
BZ = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
SUITES = sys.argv[2:] or ["📥️actions", "📥️fingerprint", "📥️sources", "📥️workspace", "📥️inventory"]
MODE = sys.argv[1] if len(sys.argv) > 1 else "staged"
OUT = "/private/tmp/bzlazy/suites-" + MODE + "/bz"
SPEC = re.compile(r"""(from\s*|import\s*\(\s*|new URL\(\s*)(["'])(\.{1,2}/[^"']+)\2""")
ALIAS = {}
if MODE == "staged":
    S = HERE + "/staged"
    ALIAS[BZ + "/🧬️schema/🔣️.json"] = OUT + "/🧬️schema/🔣️.json"
    ALIAS[BZ + "/🧭️commands/🟦️.ts"] = S + "/commands.ts"
    ALIAS[BZ + "/🎯️affected/🟦️.ts"] = S + "/affected.ts"
    ALIAS[ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️selection/🟦️.ts"] = HERE + "/run/selection.ts"

def rewrite(text, live, out):
    def fix(m):
        target = os.path.normpath(os.path.join(os.path.dirname(live), m.group(3)))
        inside = target.startswith(BZ + "/") and not target.startswith(BZ + "/🧪️tests/" + "\0")
        target = ALIAS.get(target, target)
        if target.startswith(BZ + "/🧪️tests/") or target == BZ + "/📜️script.ts":
            target = OUT + target[len(BZ):]
        rel = os.path.relpath(target, os.path.dirname(out))
        if not rel.startswith("."): rel = "./" + rel
        return m.group(1) + m.group(2) + rel + m.group(2)
    return SPEC.sub(fix, text)

def main():
    if os.path.exists(OUT): shutil.rmtree(OUT)
    os.makedirs(OUT + "/🧬️schema")
    live_script = BZ + "/📜️script.ts"
    src = (HERE + "/staged/script.ts") if MODE == "staged" else live_script
    open(OUT + "/📜️script.ts", "w", encoding="utf-8").write(rewrite(open(src, encoding="utf-8").read(), live_script, OUT + "/📜️script.ts"))
    shutil.copy(HERE + "/staged/schema.json" if MODE == "staged" else BZ + "/🧬️schema/🔣️.json", OUT + "/🧬️schema/🔣️.json")
    for suite in SUITES:
        src_dir = BZ + "/🧪️tests/" + suite
        dst_dir = OUT + "/🧪️tests/" + suite
        for dirpath, _dirs, files in os.walk(src_dir):
            rel = os.path.relpath(dirpath, src_dir)
            target_dir = os.path.normpath(os.path.join(dst_dir, rel))
            os.makedirs(target_dir, exist_ok=True)
            for f in files:
                s = os.path.join(dirpath, f); d = os.path.join(target_dir, f)
                swap = {BZ + "/🧪️tests/📥️actions/🔣️.json": "actions-fixture.json", BZ + "/🧪️tests/📥️actions/🧬️schema.json": "actions-schema.json"}
                if MODE == "staged" and s in swap:
                    shutil.copy(HERE + "/staged/" + swap[s], d); continue
                code = {BZ + "/🧪️tests/📥️actions/🟦️.ts": "actions-test.ts", BZ + "/🧪️tests/📤️arguments/🟦️.ts": "arguments-test.ts"}
                if MODE == "staged" and s in code:
                    open(d, "w", encoding="utf-8").write(rewrite(open(HERE + "/staged/" + code[s], encoding="utf-8").read(), s, d)); continue
                if f.endswith(".ts") or f.endswith(".mjs"):
                    open(d, "w", encoding="utf-8").write(rewrite(open(s, encoding="utf-8").read(), s, d))
                else:
                    shutil.copy(s, d)
    print(OUT)

if __name__ == "__main__": main()
