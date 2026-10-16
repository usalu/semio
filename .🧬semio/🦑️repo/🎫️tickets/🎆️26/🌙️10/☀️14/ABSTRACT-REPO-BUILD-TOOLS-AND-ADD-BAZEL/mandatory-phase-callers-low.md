# Mandatory Phase Caller Audit

Read-only rg plus TypeScript AST parsing of 86 permanent TS/MJS files mentioning the six scoped APIs; 178 direct or imported-alias calls inspected. No tests, native execution, stores or Source edits. This is syntax/arity evidence, not whole-program type or runtime proof.

Found 104 calls below mandatory arity across 58 files. Cargo imports aliased as runCargoTestsV1/runCargoLintV1 are included; a literal-name-only census misses these. All listed Cargo aliases resolve through explicit named imports to the repository admission owner.

- `./🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts:37`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/📜️script.ts:52`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust/📜️script.ts:22`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:22`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:27`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:72`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:88`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts:101`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/📜️script.ts:13`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust/📜️script.ts:16`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust/📜️script.ts:20`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/📜️script.ts:25`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/📦️packages/🦀️rust/📜️script.ts:38`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/📦️packages/🦀️rust/📜️script.ts:196`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/📜️script.ts:23`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:42`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:60`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:78`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:79`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:87`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:95`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:112`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:120`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:128`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts:172`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts:40`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/🌱️value/📦️packages/🦀️rust/📜️script.ts:37`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/🌱️value/📦️packages/🦀️rust/📜️script.ts:38`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📜️script.ts:63`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts:67`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/📜️script.ts:22`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📜️script.ts:35`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📜️script.ts:59`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📜️script.ts:18`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust/📜️script.ts:25`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧮️expression/📦️packages/🦀️rust/📜️script.ts:18`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/📦️packages/🦀️rust/📜️script.ts:12`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts:33`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts:32`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/📜️script.ts:25`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust/📜️script.ts:27`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts:27`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts:37`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts:68`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts:80`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/📜️script.ts:23`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts:69`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts:77`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/📜️script.ts:39`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📜️script.ts:25`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📜️script.ts:37`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust/📜️script.ts:23`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:65`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:82`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust/📜️script.ts:28`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust/📜️script.ts:23`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts:43`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts:60`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/📜️script.ts:11`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📜️script.ts:24`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📜️script.ts:31`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:35`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:67`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:413`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts:225`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:39`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:49`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:59`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:66`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:73`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:80`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:87`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:94`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts:101`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts:26`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts:38`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts:64`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts:69`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:19`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts:19`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/📜️script.ts:16`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts:17`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts:24`: `runCargoLintV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts:54`: `runOwnedBuildChildInvocationV1` has 3 arguments; requires 4.
- `./🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:1782`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:2871`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:2884`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:2897`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:2908`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/📜️script.ts:15`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📜️script.ts:24`: `runCargoTestsV1` has 4 arguments; requires 5.
- `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/⏱️camera/📦️packages/🦀️rust/📜️script.ts:18`: `runCargoTestsV1` has 4 arguments; requires 5.

Package Script entry methods may issue the first genuine phase once and carry it. Helpers beneath existing admitted scopes must receive the actual phase, not mint sibling root controls. WATCH direct/adapter calls and current projector calls have required arity in this bounded census. Generator production call passes actual context as argument six; its selected permanent law checks phaseControl. Dynamic/eval/model function calls not identified through these names remain outside this syntactic census.

## Lexical Binding Refinement

TypeScript lexical symbol resolution of the retained findings identifies 1 shadow/non-import calls; the initial 104 count is raw syntax, not 104 proven canonical bindings.

- `./🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts:77`: [{"kind":"BindingElement","line":76,"text":"runCargoLintV1"}]

Exact framework helper: `🧰️framework/📦️packages/🦀️rust/📜️script.ts:36` testCargo(original,root,manifest,packages,args), canonical call38. Earlier chat label runFramework was descriptive and inexact.

## Dashboard Correction Review

Current dashboard BuildScript forwarding branch passes first genuine phase0 as argument four, issued once at the Script entry. Direct actual build branch is unchanged. New permanent method-AST law authenticates exact Original/private-issued phase, once-only issuance and literal forwarded arguments, with four route cases independently checked through Ajv/SQLite. Child/install ports are explicitly modeled; this is Source authority proof, not native dashboard runtime. Review is clean; executor terminal/strict evidence remains separately owned.
