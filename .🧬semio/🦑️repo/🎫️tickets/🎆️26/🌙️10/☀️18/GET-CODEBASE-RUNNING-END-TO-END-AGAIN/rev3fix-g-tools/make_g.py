from make_lib import *

# ---- G: the farm closes every pool
farm = B + "/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs"
edit(farm, "      for (const pool of pools) { await pool.farm.close(); pool.release(); rmSync(pool.base, { recursive: true, force: true }); }\n",
"""      const failures = [];
      for (const pool of pools) { try { await pool.farm.close(); } catch (error) { failures.push(error); } finally { try { pool.release(); } finally { rmSync(pool.base, { recursive: true, force: true }); } } }
""")
edit(farm, "      if (environment[policy.phasesEnvironment] === \"1\") write(\"[graph-farm] \" + JSON.stringify({ pools: stats, ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));\n",
"      if (environment[policy.phasesEnvironment] === \"1\") write(\"[graph-farm] \" + JSON.stringify({ pools: stats, ...counts, adoptMilliseconds: Math.round(counts.adoptMilliseconds), projects, reason: plan.reason }));\n      if (failures.length === 1) throw failures[0];\n      if (failures.length > 1) throw new AggregateError(failures, \"Closing the graph farm failed for \" + failures.length + \" pools\");\n")
newfile(B + "/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🔒️farm-close/🟦️.ts", "farm-close.ts")
