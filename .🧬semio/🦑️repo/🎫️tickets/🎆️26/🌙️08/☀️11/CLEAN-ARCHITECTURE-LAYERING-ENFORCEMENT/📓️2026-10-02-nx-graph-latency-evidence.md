# Nx Graph Latency Evidence

Read-only code/process snapshot; no reset, process interruption, graph/test invocation or alternate runner. No measured cause is claimed.

Observed attributes task chain52586→52587→52588 was alive for about5m34s with0% instantaneous CPU. Four isolated plugin workers were its children:52614 Nx JS,52624 Python,52657 Repo library and52658 test-cases. At the later snapshot library worker showed1%CPU while others0%. This establishes an active Nx process/plugin stage before a visible Bun law process, but instantaneous CPU and parentage do not identify a blocking operation or time attribution. Several unrelated native routes also retained their own plugin workers; their ages do not prove Root's route owns their delay.

Configured plugins come from nx.json:85 Repo library/🟨️.mjs and97 Repo test/🟨️.mjs. Tests createNodesV2 matches **/*.feature, yet testCaseProjects at183–202 ignores supplied configFiles for selection and calls discoverCaseDirs(workspaceRoot) on every invocation. That walker at318–340 recursively lists all admitted directories and re-lists test case child directories to locate feature files. Native input plans are then composed per owner with an invocation-local cache. This is a concrete whole-tree cost candidate; no timing here proves it dominates.

Generated ticket output scanning is not supported as a cause by these plugin walkers: test isExcluded applies taxonomy reserved subtree names, and current authority reserves .🧬semio. Repo walkCargoToml at80–92 explicitly skips .🧬semio, symlinks and generated directories. Native Cargo intermediate trees under the ticket are therefore excluded by those owned walks. Nx's underlying glob/file-map behavior may still differ, but requires its own evidence rather than blaming tests plugin.

Library implementationRevision at1539–1553 hashes taxonomy, policy and graph helper authority inputs. A real taxonomy edit changes its revision and reloads the plugin, which is required fresh authority. Tests implementationRevision hashes plugin/dependency module; testCaseProjects also imports library under its source hash and reads live taxonomy/policy. Root source edits may trigger graph processing through file map inputs, but no retained trace identifies which exact edit invalidated the current graph cache.

Potential owned improvement: derive feature candidate inventory from Nx supplied configFiles after exact taxonomy admission, or retain an authority-keyed filesystem inventory with explicit directory-change invalidation; do not omit physically added/removed cases. Keep canonicalCase/exclusion/native ownership plans and cancellation/progress. Shared measured stage events around case discovery, package/native plan composition, Cargo inventory, import-edge collection and plugin bootstrap would establish actual hot stages. This is a source-backed optimization proposal, not an authorization to use stale graphs or bypass registered routes.

Current cached import-edge implementation at1431–1441 already keys parse results by file hash and processes changed/full map according to projectFilesToProcess at1450. Replacing that with unconditional fresh parses would worsen work; preserve its owned authority boundaries.

No test timing/fullpass or causal latency diagnosis is asserted.
