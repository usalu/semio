import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage1c");SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
s.edit(SI,[
('return alive === 0 ? [] : [{ name: "closure:layerVerdicts (" + alive + " rounds)", entries, bytes: entries * 96 }]; }',
 '''const spread = (cache, map) => { const keys = cache.keys(), step = Math.max(1, Math.floor(keys.length / 16)); return { entries: keys.length, sample: () => keys.filter((_key, at) => at % step === 0).slice(0, 16).map(key => map(cache.peek(key))) }; };
  const real = [{ name: "closure:entries (real)", ...spread(closureEntriesV1, entry => ({ recipe: entry.recipe, value: entry.value, rows: entry.rows, digests: entry.digests, sidecar: entry.sidecar, answers: [...entry.answers ?? []], layers: entry.layers })) }, { name: "closure:walkFacts (real)", ...spread(walkFactsByDigestV1, fact => fact) }, { name: "closure:walkMembers (real)", ...spread(walkMembersByDigestV1, set => [...set]) }, { name: "closure:recipes (real)", ...spread(knownClosureRecipesV1, recipe => recipe) }];
  return [...real, ...alive === 0 ? [] : [{ name: "closure:layerVerdicts (" + alive + " rounds)", entries, bytes: entries * 96 }]]; }''',1),
])
s.finish([B+"🧠️memory/📋️slice-report/🧪️tests/🟦️.ts"],"- Step 1c: the report measures the closure entry cache, the walk fact and member tables and the recipe table by sampled real size (the bounded caches charge a declared constant, not what they hold).")
