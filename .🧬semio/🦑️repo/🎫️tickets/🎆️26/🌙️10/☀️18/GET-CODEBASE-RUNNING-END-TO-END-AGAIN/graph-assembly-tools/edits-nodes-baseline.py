import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
apply(L+"/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",[
 ('import { createMemoryGovernorV1, graphMemoryBudgetBytesV1 } from','import { createMemoryGovernorV1, graphMemoryBudgetBytesV1, measureMemoryBaselineV1 } from'),
 ('createMemoryGovernorV1({budgetBytes:graphMemoryBudgetBytesV1(process.env),workers:limit,','createMemoryGovernorV1({budgetBytes:graphMemoryBudgetBytesV1(process.env),baselineBytes:measureMemoryBaselineV1(),workers:limit,'),
])
