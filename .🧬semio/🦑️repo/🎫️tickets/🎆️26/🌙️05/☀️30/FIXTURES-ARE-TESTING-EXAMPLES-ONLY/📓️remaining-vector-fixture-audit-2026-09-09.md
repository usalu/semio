# Remaining Vector Fixture Audit — 2026-09-09

## Result

The bounded semantic census found live test corpora outside `🧫️fixtures` in Framework and `✏️s`; no such Hub corpus was found. It did not repeat the root-owned physical layout or URI-resolver scans, and it excludes the Stdio eleven contract vectors, the original eleven plugin moves, canonical TypeScript fixture-path work, and library-lane ownership work.

The coordinator has already moved the Flow, CAD, Writer, Sourcing, and seven Store/Presence findings identified here. The remaining Framework queue below has exact reader proof and should move atomically with those readers.

## Live Corpus Moves Still Needed

| Current corpus | Reader evidence | Destination |
| --- | --- | --- |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧫️cases.json` | `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:272` iterates four HTTP failure vectors. | The bundle owner's `🧫️fixtures/🧫️cases.json`. |
| `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🚨️fault.fixture.json` | Actor Vitest reader `🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-publication-fixture-preserves-watchdog-and-te/🟦️.ts:14`; seven OS dispatch Rust-test includes at `🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:44-207`. | `🚪️lifetime/🧫️fixtures/🚨️fault/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🧪️group-history.json` | `🌿️vcs/🧪️tests/🔬️group-history-visibility/🦀️.rs:9`. | `🌿️vcs/🧫️fixtures/👥️group-history/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🧪️installation.json` | Extension Rust unit `:5` and plugin Store installation identity test `🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts:29,56`. | `🧩️extension/🧫️fixtures/🧩️installation/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🩺️runtime-fault-vectors.json` | Plugin Rust test `🧪️tests/🔬️plugin-runtime-runtime-cleanup-fault-vector/🦀️.rs:4`; renderer test imports at `📺️renderer/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts:9`. Production sources only name it in comments. | `🔌️plugin/🧫️fixtures/🩺️runtime-fault-vectors.json`. |
| `🔌️plugin/🥇️tool-latest-wins.json` and `🔌️plugin/🔗️tool-latest-wins-integration.json` | Rust test paths include `app-typed-command-full-operation:250,451,598,715,744,769,827`, `plugin-runtime-plugin-builder-contract:1489`, `Store unit:3124`; TS reader `tool-job-latest-wins:9,29`. | `🔌️plugin/🧫️fixtures/`, retaining filenames. |
| `🔌️plugin/🔬️tool-factory-proof.json` | Test-only Rust helper at `🔌️plugin/🦀️.rs:11634` is `#[cfg(test)]`; TS reader `🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts:9`. | `🔌️plugin/🧫️fixtures/🔬️tool-factory-proof.json`. |
| `🔌️plugin/🚪️lifetime/🛂️aggregate-admission.json` | `🚪️lifetime/🧪️tests/🧪️aggregate-admission/🦀️.rs:6`. | `🚪️lifetime/🧫️fixtures/🛂️aggregate-admission.json`. |
| `🔌️plugin/📇️registry/📦️deployment/{🔒️fixed-parent-cases.json,🧪️cases.json}` | Fixed-parent reader `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts:146`; deployment readers `📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts:46,62,80` and Store test `:57`. | `📇️registry/🧫️fixtures/📦️deployment/`, retaining filenames. |
| Twelve ShellHost vectors under `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost`: `🌱️artifact-creation/{🔣,🪪️catalog-authority/🔣,🚪️ready-opening/🔣}`, `👥️presence-scope/🔣`, `🪪️host-bootstrap/🔣`, `📇️directory-bootstrap/🔣`, and `🗨️dialog-origin/{🔣,🛂️admission/🔣,🛂️admission/📄️document/🔣,🎥️tutorial/{🔣,⏩️seek/🔣,🧵️serial/🔣}}`. | Only renderer tests import them: `🔬️artifact-creation-ready-opening:6,7,9`, `👥️scoped-presence:21`, `⚡️quick:14`, `📇️directory-home-bootstrap:16`, `🔬️engine-contract:32,34-37,42,44-45`, and `🔬️document-opening:4`. | The ShellHost owner's `🧫️fixtures/`, preserving each current relative subpath. |

## Moves Executed Or In Progress By The Coordinator

- Flow `🎬️action-cohort/🔣️.json` is a route/law test corpus read by the Flow TypeScript audit and Flow Rust unit case. Its schema remains in `🎬️action-cohort/🧬️schema`; the corpus moves to the Flow plugin's `🧫️fixtures/🎬️action-cohort/`.
- CAD presence `🧪️retirement.json`, Writer's migration JSON formerly in `📚️examples/🎬️demo-session`, and Sourcing's `📦️expected-stock.json` plus presence retirement fixture are test-only vectors and have been handed to the coordinator with their Rust/TypeScript readers.
- Store/Presence's `🧹️retirement.json`, `📌️peer-commit.json`, `🛂️peer-admission.json`, `📢️member-publication.json`, `🎯️group-cursor.json`, `📖️group-read.json`, and `🌱️runtime-seed.json` are test-only cases. The coordinator reports these seven and their five reader files are already moved/updated.

## Classified Non-Moves

- The semantic-marker pass considered 270 authored JSON files outside fixture, asset, and schema roots. 194 live under explicit `🔮️oracle`, `🧪️oracle`, or `⚖️oracle` declaration roots, and 36 are Stdio `📜️artifact-definition.json` declarations. These are oracle/contract declarations, not example fixtures.
- UI contract `📚️examples/🧪️conformance` JSON is protocol conformance data beneath `🧬️contract`; it remains a declaration root. Likewise testkit mutation JSON is owned descriptor/protocol declaration data, not a fixture relocation target.
- `🌉️mcp/🎚️config/🧱️binary-gate.json` is read by the MCP Rust package script and listed as an Nx target input; it is build configuration. Its separately canonical test fixture is already under `🌉️mcp/🧪️tests/.../🧫️fixtures`.
- No consumer was found for Energy's two `🪨️tests/p7c2-energy-retained-wire-*.json` files, CAD's generated `📚️examples/⚙️machine/🔣️.json`, Layout's `📚️examples/🛂️manifest.json`, or Dev activation's `🧫️cases.json`. They are not actionable fixture moves without an owner decision; this audit does not infer one from their names.
- No Hub authored JSON with test-vector markers remained outside its canonical fixture roots. Hub TypeScript fixture reads inspected in the integration suite resolve only into `🌎️hub/**/🧫️fixtures` or Framework canonical fixtures.

## Limits

This is a read-only source/semantic audit. It used path and JSON-key markers plus exact reader searches; it did not rerun the root-wide layout scan, resolver scan, builds, or test suites. No test result is claimed. Temporary generated inventories were removed after their counts and reader evidence were recorded here.


## Framework corpus execution follow-up

**Status:** Complete for the assigned Framework queue. Nineteen example corpora moved to semantic `🧫️fixtures` owners without compatibility aliases: three plugin-root tool corpora, one lifetime admission corpus, two registry deployment corpora, the shared Preview2 import-rewrite corpus, and twelve ShellHost corpora. Schema and production implementation files stayed in place.

The Preview2 rewrite vectors moved out of the TypeScript package to `🔌️plugin/🧫️fixtures/🕸️imports/🧫️cases.json` because the independent repository library oracle is a cross-language consumer. ShellHost vectors retain their previous semantic subpaths below `ShellHost/🧫️fixtures`. Every direct Rust/TypeScript reader and two computed ShellHost runner paths now resolve the new owners.

### Byte and path evidence

The pre-move and post-move manifests are in [pre-move.json](🗑️generated/framework-final-corpora/pre-move.json) and [moves.json](🗑️generated/framework-final-corpora/moves.json). All 19 old paths are absent, all 19 new paths exist and parse as JSON, and every SHA-256 hash and byte count matches. [validation.json](🗑️generated/framework-final-corpora/validation.json) retains the focused results.

### Focused verification

- The plugin factory-proof exported oracle passed 28 assertions.
- The shared import-rewrite oracle passed all four vectors against independent `es-module-lexer` import spans.
- The fixed-parent Bun case passed one test and 60 expectations.
- The renderer React quick Nx target passed five tests. A focused long-budget Nx run of the five affected consumer files passed 478 tests across five files.
- The actual registry deployment functions and JSON Schema passed 14 route cases, three module URL cases, three valid-directory cases, and twelve invalid-directory cases.
- Twelve changed Rust `include_str!` edges resolve and parse at their new fixture owners.
- The scoped-presence runner oracle reached `checks=29 clean` before its subsequently duplicated Vitest phase was stopped; the same consumer file had already passed in the focused 478-test Nx run.

### Bounded limitations

- The latest-wins exported oracle loaded and completed the moved latest-wins schema/equality sections, then failed at its later retained `ChildEmit` live-source predicate because concurrent plugin implementation work changed that source contract. The relocation itself did not cause a file, JSON, schema, or equality failure.
- The registry Vitest file initially failed before test collection because `📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts:31` still resolved `../🧬️catalog-complete` instead of the canonical `../../🧫️fixtures/🧬️catalog-complete`. The coordinator owns that separate stale-reader correction and final focused rerun.
- The directory-home-bootstrap runner command remained queued behind shared Nx graph construction and was stopped. Its computed fixture path exists, its source consumer compiled, and its focused test file passed in the 478-test run.
- No broad native build or repository-wide layout scan was run.

### Exact flat authored-path manifest

`authored-paths.json` contains the same 62 unique entries.

```json
[
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🥇️tool-latest-wins.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🥇️tool-latest-wins.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔗️tool-latest-wins-integration.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔗️tool-latest-wins-integration.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔬️tool-factory-proof.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🔬️tool-factory-proof.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🛂️aggregate-admission.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🧫️fixtures/🛂️aggregate-admission.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🔒️fixed-parent-cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/📦️deployment/🔒️fixed-parent-cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🧪️cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/📦️deployment/🧪️cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🕸️imports/🧫️cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🕸️imports/🧫️cases.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🪪️catalog-authority/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🚪️ready-opening/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/👥️presence-scope/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🪪️host-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/📇️directory-bootstrap/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/📄️document/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial/⏩️seek/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/⏩️seek/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial/🧵️serial/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🧵️serial/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🧪️tests/🧪️aggregate-admission/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️artifact-creation-ready-opening/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️remaining-vector-fixture-audit-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🗑️generated/framework-final-corpora/pre-move.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🗑️generated/framework-final-corpora/moves.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🗑️generated/framework-final-corpora/validation.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🗑️generated/framework-final-corpora/authored-paths.json"
]
```


## DSL boxed-fields final follow-up

**Status:** Complete. The 98-byte boxed-field example moved from the DSL test-case tree to the DSL semantic fixture owner. Its sole live reader now uses `include_str!("../../🧫️fixtures/📦️boxed-fields/🔣️.json")`; no compatibility file remains.

[The retained move evidence](🗑️generated/framework-final-corpora/dsl-boxed-fields-move.json) records identical pre/post SHA-256 `65837d1d8190ca69eb3017e0795a7ec72008b65e138d625ff92acf4dbc2b5df1`, matching byte counts, valid JSON, the exact reader path, old-path absence, and new-path presence. A focused source scan found no other boxed-fields reader and no remaining `🧪️tests/🧫️fixtures` path in the DSL owner.

The exact Nx consumer command was attempted. Its first invocation rejected a misplaced optional `--nocapture` argument before compilation. The corrected invocation waited for the shared project graph and then began compiling Cargo dependencies; it was stopped without a test result so the coordinator could run the final repository scan. No test pass is claimed.

### Exact flat authored paths

`dsl-boxed-fields-authored-paths.json` retains the same array.

```json
[
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧫️fixtures/📦️boxed-fields/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧫️fixtures/📦️boxed-fields/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/📦️boxed-fields/🦀️.rs"
]
```
