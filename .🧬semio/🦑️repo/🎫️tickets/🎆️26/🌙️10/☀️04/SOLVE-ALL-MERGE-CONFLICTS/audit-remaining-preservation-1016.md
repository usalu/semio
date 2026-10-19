# Remaining Preservation Routing Audit — 2026-10-16

Finite read-only source comparison. Base `f856dff081373e131ebf72618120cb73e303a912`; HEAD `1bae6115bc56bf5e1b01212774dd76877a5f64fe`; stash `ec7b3361858df86599da79e5b6dde70ccbbc3625`. No source or Git mutations, builds, tests, compilers or runtime execution. This report is the only write. Snapshot evidence can become stale while parallel executors work.

## OS: High Executor Routing

Compared stash AM paths to current bytes and both ancestor/stash blobs using read-only cat-file batches. These live paths exist at their HEAD owner and remain byte-identical to base despite productive stash changes, so they are stronger evidence than absent old locators. Do not overwrite other HEAD changes or automatically transplant an old architecture; merge corresponding authored stash semantics into current owner.

| Stash cluster under OS | Base-identical | Missing old locator | Other/merged |
| --- | ---: | ---: | ---: |
| config/schema/mutations | 61 | 1 | 0 |
| renderer/engine | 43 | 49 | 38 |
| SPR/history | 21 | 3 | 1 |
| workflow/artifacts | 20 | 6 | 22 |
| store/testing | 19 | 0 | 0 |
| store/tests | 19 | 0 | 3 |
| plugin/testing | 19 | 18 | 1 |
| SPR/command | 16 | 2 | 0 |
| plugin/host | 16 | 2 | 4 |
| plugin/reactor | 14 | 12 | 6 |
| store/presence | 11 | 1 | 1 |
| store/composition | 11 | 12 | 4 |

Exact live owner examples and reviewed base→stash semantics:

- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🦀️.rs`: stash separates diff/inverse into `ColdMutationKind`; current is base-identical. Route config mutation family with protocol cold-mutation preservation.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🦀️.rs`: stash imports cold traits and calls `apply_diff_cold`; current is base-identical. Route workflow/flow/artifact adapters with protocol owner.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🔁️fold/🦀️.rs`: stash uses controlled `vector`, controlled negative-clock refusal and operation identity copying; current is base-identical. Preserve original-control budgeting/refusal semantics with SPR history.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs`: stash uses `retirement_should_yield` and a depth-decremented `with_retained_loan` around child closing; current is base-identical. Route plugin retirement/context/original-turn families together with shared value/actor work.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: stash adds `next_demand` implementations for unsupported snapshot/factory and artifact-store opening, including depth overflow refusal; current is base-identical. Route store open/read/replay/paged/retirement custody as one owner cluster.

The OS changed-path envelope is 1090 stash AM paths. Missing locators are NOT proven losses: HEAD extracted/split some old owners. Renderer engine and plugin declaration/retained-command/instance locators especially need HEAD tree mapping before any transplant. Board original custody is already assigned; avoid concurrent edits there. These counts are routing evidence, not a whole-OS preservation verdict.

## Repo: Separate Non-Build Owner Follow-Up

Current base-identical live owners with productive stash deltas:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go`: stash adds analyze `--policy`, validates registered policy identifiers, uses `ToolAnalyze` instead of the older GraphQL path and introduces language/extension scope selection. Associated Go project/script/test owners are also base-identical. Preserve with statutes/workspace analysis owners.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json`: stash makes ordered interactions durable and adds a typed Interaction definition instead of never re-encoding them. Matching Go and Rust ticket codec owners/tests/fixtures remain base-identical. Route together; avoid losing durable interaction content.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts`: stash removes devcontainer editor-surface derivation and retains nextest/update-manifest ownership. Its source contracts/fixtures remain base-identical. Merge with agent/editor ownership changes rather than changing build architecture.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🤖️agent-clients/🟦️.ts`: base-identical live owner; authored stash changes remain unapplied. Command child changes only documentation; review main owner for behavior.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` and `📓️schema-catalog.md` remain base-identical. Review alongside restored schema owners; do not copy catalog wholesale over new HEAD entries.

Other bounded base-identical families: repo model Go/Rust/schema; statutes Go/schema; MCP Rust/schema/unit/entrypoint fixture; client CLI/MCP protocol/repository/component; workspace Go implementation/script/tests. Developer-tools six-file subtree and Nx-entrypoint schema/fixture are absent original locators, requiring relocation classification. Discovery/taxonomy/library main and Bun workspace files differ from both base/stash and therefore are NOT classified as missing content by this audit. Dependency TypeScript mjs reviewed delta adds JSDoc parameter types only; do not report it as a runtime loss. Dependency bootstrap owners also differ from both and need HEAD-aware semantic comparison.

## Smaller Product Follow-Up

Prior report `audit-teaching-preservation-1016.md` records pets 37 base-identical of 39 changed paths; server 7 of 8; print 5 of 6; hub compositions 5 of 8. Exact productive examples:

- `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts`: missing authored concealed-element selector/tree-walk/observed-attribute changes at current live owner.
- `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🦀️.rs`: missing authored cold-diff/prepared mutable application at current live owner; synchronize protocol family.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs` and unit child remain base-identical. Main server TS stash delta removes inline Vitest registration, so do not label it lost productive runtime functionality.
- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` and `🏷️type/🟦️.ts`: stash source-contract changes absent at these live owners.

No global source search for equivalent relocated semantics was performed. Base identity proves changes absent locally; it does not prove equivalent functionality exists nowhere. Root globals/launch, Teaching/course/shared modules and Domain ✏️/dashboard execution remain separately assigned.
