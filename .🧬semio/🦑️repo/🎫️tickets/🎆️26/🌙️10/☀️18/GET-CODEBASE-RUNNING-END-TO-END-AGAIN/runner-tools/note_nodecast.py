"""📝 Adds the nodecast ready line to result-runner.md (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
anchor = "\n## Verdicts"
add = ("FOR coordinator: nodecast READY (1 file): `framework-os-dev:test` 🧩️contribution ('admits only selected-owner factory engine contributions across Bun, Node and Ajv') dies in Node 24 strip-types mode with "
       "`ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` on the angle-bracket cast `of<[string,string][]>[…]` in `📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts:64` (pairs()); "
       "fix = the same tuple list with `as [string,string][]`. Reproduced on rnfw: base 3/4 with exactly the live SyntaxError, fix 4/4; Node loads the module; tsc 0; scan 1/0. Apply: `python3 -I C/runner-tools/staged/nodecast/stage.py apply` ; "
       "verify: `sh C/runner-tools/staged/nodecast/verify.sh /Users/ueli/Documents/semio` ; revert: `… stage.py revert`. Product proof: `python3 -I C/prodmap-tools/group.py pm-osdev '@semio-tech/framework-os-dev' 'test'` "
       "(its 2 `🧹️config` bound reds remain, not diagnosed).\n")
open(p, "w", encoding="utf8").write(t.replace(anchor, add + anchor, 1))
