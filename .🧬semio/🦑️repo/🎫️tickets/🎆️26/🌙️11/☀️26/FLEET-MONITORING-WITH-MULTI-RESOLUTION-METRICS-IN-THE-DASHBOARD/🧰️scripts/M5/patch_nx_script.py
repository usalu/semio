import os, sys
P = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"
t = open(P, encoding="utf-8").read()
edits = [
 ('import {readBuildCountersV1,unitLedgerLinesV1,unitLedgerV1} from "../🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";',
  'import {readBuildCountersV1,unitLedgerLinesV1,unitLedgerV1} from "../🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";\nimport {createNxCacheCounterV1} from "./📊️nx-cache/🟦️.ts";'),
 ('output: createNxFailureTailV1() }; graphChanges.set(child, state);',
  'output: createNxFailureTailV1() }; graphChanges.set(child, state);\n        const nxCache = createNxCacheCounterV1(this.root);child.once("close", () => nxCache.flush());'),
 ('state.output.push(chunk, output === process.stderr ? "stderr" : "stdout");',
  'state.output.push(chunk, output === process.stderr ? "stderr" : "stdout"); if (output === process.stdout) nxCache.push(chunk);'),
]
for o, n in edits:
    c = t.count(o)
    if c != 1:
        print("ANCHOR", c, o[:90]); sys.exit(1)
    t = t.replace(o, n)
if "--check" in sys.argv: print("anchors ok"); sys.exit(0)
tmp = P + ".m5.tmp"; open(tmp, "w", encoding="utf-8").write(t); os.replace(tmp, P); print("ok")
