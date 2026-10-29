# Tracked Path Budget Audit

The cross-platform bootstrap fixture reserves a46 UTF-16-unit clone root under stock Windows MAX_PATH. Relative file paths may use213 units (260−1−46); relative directory paths may use201 (260−12−1−46). The original202 directory arithmetic in this ticket was corrected. Native JavaScript UTF-16 length and independent iconv-lite UTF-16LE byte counts agree in the suite.

A fresh read-only tracked-index snapshot now finds 0 over-budget files and 0 over-budget directories; maximum lengths are 213 and 201. Source paths/index state changed concurrently during the task, so the initial historical inventory is superseded by this measurement. The index can retain entries absent from the current working tree; product-path-audit.md is authoritative for the applied semantic product fixture refactor and current-worktree proof.

## Over-Budget Files

None in this snapshot.

## Over-Budget Directories

None in this snapshot.

No Source copies, moves, deletions or git index modifications were performed by this audit.

## Strict256 Absolute Path Clarification

Current fixture pathBudget declares windowsMaxPath260,cloneRootMax46,componentMax255. Cross-platform test163 computes relative fileBudget260−1−46=213 and does not construct an absolute clone root. This proves stock MAX_PATH sizing, not the users stricter256-character visible path invariant. If46 denotes the full prefix including trailing separator, the permitted visible maximum is259 (three above256; the APIconstant differs byfour because of NUL). If46 denotes root spelling without trailingseparator, actual visiblemaximum260 isfour above256. The fixture currently leaves that separator meaning implicit.

A fresh readonly repo tools Git ls-files cached NUL inventory measured 216469 indexed paths, 216465 physically present, 4 absentindexentries. Current root /home/deck/semio has16UTF16units plusseparator. Presentpaths maxrelative213/currentabsolute230; no currentabsolute path exceeds256. Nonticketproducts count129930, maxrelative210/currentabsolute227; productdirectories max201. Current Linux Source is within the strict invariant.

At a46-unit fullprefix includingseparator, strictrelativefilebudget is210: 11 present tracked files exceed256, allhistoricalticketpaths, zero products. At a46-unit root plus one separator, strictrelativefilebudget209: 30 presentfiles exceed256, including 12 products. Generated strict-path-budget.json retains both exact lists. Neither list implies source copies or removable artifacts; preserve audits/reports.

Recommended policy correction is an explicit maximumAbsolutePathUnits256 and unambiguous clonePrefixUnits includingseparator, then filebudget=min(256−prefix, stockWindows260−NUL−prefix). Keep existing201directorylimit where needed for stockWindows directorycreation reserve; no need to weaken it. Do not claim relative213 plus46 root meets strict256. No Source/schema edit, move, deletion or Git index mutation by this audit.

### Present Ticket Files Above256 at Full Prefix46

- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️current-plugin-oct8-extended-zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz-actor-observation-implementation-prepared.json (relative 211, prefixed 257)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️current-plugin-oct8-extended-zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz-producer-neutral-green-repair-before.json (relative 212, prefixed 258)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️current-plugin-oct8-extended-zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz-hub-exact-custody-snapshot-before.json (relative 211, prefixed 257)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️current-plugin-oct8-extended-zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz-existing-hub-storage-owner-before.json (relative 212, prefixed 258)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📥️current-plugin-oct8-extended-zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz-fresh-current-package-owner-tests.json (relative 213, prefixed 259)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/infer-f-tools/staged/files/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/⏱️phases/🧪️tests/🐍️model.py (relative 212, prefixed 258)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/quietfc-g-tools/new/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🌤️followup/🧫️fixtures/📼️recorded.json (relative 213, prefixed 259)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/repo-mcp-build-tools/originals/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧪️tests/🧫️fixtures/🚪️entrypoint-contract.json (relative 213, prefixed 259)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/setup-g-tools/staged.pre-rebase/base/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🔣️policy.json (relative 211, prefixed 257)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity (relative 211, prefixed 257)
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/xplat3-g-tools/staged/files/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🧪️tests/🟦️.ts.pre-backlog (relative 213, prefixed 259)
