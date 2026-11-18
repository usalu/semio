import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage1d");SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
s.edit(SI,[
('const real = [{ name: "closure:entries (real)", ...spread(closureEntriesV1, entry => ({ recipe: entry.recipe, value: entry.value, rows: entry.rows, digests: entry.digests, sidecar: entry.sidecar, answers: [...entry.answers ?? []], layers: entry.layers })) },',
 'const part = key => ({ name: "closure:entries." + key, ...spread(closureEntriesV1, entry => key === "answers" ? [...entry.answers ?? []] : entry[key]) }), real = [part("value"), part("rows"), part("digests"), part("sidecar"), part("answers"), part("recipe"), part("layers"),',1),
])
s.finish([B+"🧠️memory/📋️slice-report/🧪️tests/🟦️.ts"],"- Step 1d: the closure entry cache is measured by component (value, rows, digests, sidecar, answers, recipe, layers).")
