# audit-agents-1020e: AGENTS.md audit of the applied set files

Read-only audit by `aud-agents-1020e` of the files written live by the applied sets. Repository root: `/Users/ueli/Documents/semio`. No builds, no `bun repo`, no git-modifying command, no sub-agents, no repo-wide grep. Only this file was written.

## Scope and method

- Entries per set (live file lists from the apply ledgers, backups, plans and manifests):

| Set (source) | Entries | Live state at audit time |
|---|---:|---|
| integrate2 (round 2 ledger, 25 groups, excluding the input-precision group) | 246 | applied 18:24-18:34; 33 files differ from the ledger after-hash (later writes) |
| input-precision (round 2 ledger group, same 11 files as its own set) | 11 | applied 18:29; 7 files differ from the applied hash |
| cli-zt (`cli-zt-tools/plan.json`: 18 writes + 271 edits) | 289 | applied live (`apply.py --apply`); 2 differ |
| graph-speed (`graph-speed-tools/staged/ops.json`: 10 files + 25 new) | 35 | applied; 38/38 replacements present |
| bzl-hermetic (`🗑️generated/bzl-hermetic/backup/ledger.json`) | 16 | applied; 16/16 hashes match |
| testfix-real (`staged-1` 149 ops + `staged` 35 ops) | 175 + 7 deletions | applied; 173/175 match (2 `Cargo.lock`, lock-sync pending); 7/7 deletions absent |
| conc-tests (`🗑️generated/conc-tests/backup/index.json`) | 29 | applied 20:41 (19 new files identical to staged, 23/23 edit pairs present); result header still says NOT applied |
| cargo-fix (`staged/manifest.json`, static set) | 77 | applied; 76/77 hashes match |

- Sample: 118 files (cap 120), stratified by set (integrate2 32, input-precision 8, cli-zt 22, graph-speed 10, bzl-hermetic 8, testfix-real 14, conc-tests 10, cargo-fix 14). One third of each quota was taken from files whose live content differs from the set's after-hash.
- Each sampled file was checked with a rule scan over its current content. Provenance: a finding is `pre-existing` when the same line is in `git show HEAD:<file>` (read-only), `introduced` when it is not, and new files count as introduced.
- Set-wide scan of all 885 live entries (regex level): `[DEBUG]` tags, `dbg!`, legacy/deprecation words, external imports outside test paths, AGENTS.md names, migration/adapter names, path lengths.
- Severity: High = explicit MUST NOT (runtime external library in production, legacy support). Medium = MUST/cleanup or language rule (temporary logs, comments inside definitions, English-only user text). Low = SHOULD, schema-pointer style, documentation drift.

## Allowed by AGENTS.md (not counted)

- Third-party packages (`ajv`, `typescript`, `esbuild`, `fast-glob`, `minimatch`, `graphlib`, `@iarna/toml`, `protobufjs`, `fast-json-patch`) imported only from test files (`bun:test`, `🧪️tests`, testing or fixture paths). This is the "validate with a third-party library" rule.
- Test-input Cargo manifests under `🎯️targets/...shell-tests-input-1` and test companion crates.
- Node/Bun built-ins (`node:*`, `bun:*`, `bun`), `@semio-tech/*` workspace imports and relative imports.
- `🛤️lane/🟦️.ts` (under `🎮️one-item`) imports `fast-json-patch`, `ajv`, `protobufjs/minimal`, but it is a `bun:test` file, so it is allowed.
- Rust `pub use <local_module>::` lines (local modules, not crates).
- `legacy` words in test names, law assertions and doc comments that describe removed shapes (no compatibility code).

## Set: integrate2 (sample 32 of 246)

Sample result: no docstring, comment-in-definition, external-import, or legacy violation in the 32 sampled files. Set-wide findings:

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🟦️.ts | 27, 35 (`[DEBUG]` lines) | temporary `[DEBUG]` log left in place (no removal) | Medium | pre-existing (HEAD); hub-bzl group |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts | 62 | temporary `[DEBUG]` log left in place | Medium | pre-existing (HEAD); sync-incr group |
| 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts | 38, 52, 117, 124 | user-facing text English-only (Error messages, no DE) | Medium | introduced (new file, mem-gov) |
| 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts | 34, 46, 47, 113 | user-facing text English-only (Error messages, no DE) | Medium | introduced (new file, mem-gov) |

Also noted: `host-capacity` gate and decision files are written in dense multi-statement one-liners (for example `gate/🟦️.ts:38`). Concise code is satisfied in form, but readability is weak (Low, review).

## Set: input-precision (sample 8 of 11)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json | 1 | schema-first: JSON data without `$schema` pointer (sibling schema `🧬️schema/🔣️.json` exists) | Low | introduced (new file) |

- 7 of 11 files differ from the applied hash (list in the appendix). The change is not identified by this audit; the sampled current content has no violation.
- Result header `result-input-precision.md` still says "staged, nothing applied", but the round 2 ledger shows the group applied at 18:29.

## Set: cli-zt (sample 22 of 289)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts | 100, 108, 113, 114 | legacy support: `dialectFromLegacyExampleAppId`, drops legacy `appId` (MUST NOT implement legacy support) | High | pre-existing (HEAD); not removed by cli-zt |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts | 104, 120 | temporary `[DEBUG]` log left in place | Medium | pre-existing (HEAD) |
| 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source/🟦️.ts | 79 | temporary `[DEBUG]` log left in place | Medium | pre-existing (HEAD) |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts | 52, 53 | comments inside a definition | Medium | pre-existing (HEAD); sampled |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔀️rust-non-repo-join-bases/🟦️.ts | 13-16, 98-102, 155-156, 314-316, 339-341 | comments inside definitions (17 lines in this file) | Medium | pre-existing (HEAD) |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json | `reason` of all 25 commands | English-only text; the registry has EN+DE `summary` (ok), `reason` is EN only (review whether it is shown to users) | Low | introduced (new registry file) |

- Registry (`🔣️.json`): `languages` = `[en, de]`, every `summary` has `en` and `de`, no default-language key. Compliant.
- Two files changed after apply (drift, list in appendix); both are audited at current content.

## Set: graph-speed (sample 10 of 35)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs | 74, 75 | comments inside a definition (`// 🛡️ Nx's native walker...`) | Medium | pre-existing (HEAD); sampled |

- All 38 anchored replacements are live. The 25 new files in the sample have no violation.

## Set: bzl-hermetic (sample 8 of 16)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔣️.json | 1 | schema-first: JSON data without `$schema` pointer (schema lives in the sibling `🧬️schema/`) | Low | introduced (new file) |

- Sampled TypeScript: docstrings start with an emoji, no comments inside definitions, externals only in tests. Compliant.

## Set: testfix-real (sample 14 of 175; 7 deletions verified)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎬️MediaTransportHost/🟦️.tsx | 2 | runtime external library in production code (`react`, UI component; MUST NOT create runtime dependencies on external libraries) | High | pre-existing (HEAD, same line) |
| set-wide: 48 test files (`.rs` and `.ts`) | 95 lines in total (full list in the appendix) | temporary `[DEBUG]` logs left in test sources | Medium | pre-existing (HEAD); none added by the set |

- `Cargo.lock` (2 files) differs from the set's hash because lock-sync is pending; expected drift, not a violation.
- The sampled Rust files have no docstring or comment-in-definition violation.

## Set: conc-tests (sample 10 of 29)

No rule violation in the 10 sampled files (docstrings start with an emoji; external imports only in test files).

- Documentation drift (Low): `result-conc-tests.md` says "NOT applied", but the live tree holds all 29 changes (applied 20:41).

## Set: cargo-fix (sample 14 of 77)

| File (repo-relative, under `/Users/ueli/Documents/semio/`) | Line(s) | Rule | Severity | Provenance |
|---|---|---|---|---|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/📦️packages/🦀️rust/Cargo.toml | 51, 52, 53, 54 | runtime external crates in production (`wasmtime`, `wasmtime-wasi`, `serde`, `serde_json`; MUST NOT create runtime dependencies) | High | pre-existing (HEAD); not added by the set |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/⚙️plugin-runtime/📦️packages/🦀️rust/Cargo.toml | 38, 39 | runtime external crates in production (`serde_json`, `tokio`) | High | pre-existing (HEAD); not added by the set |

- Sampled `.rs` files: docstrings start with emoji and no comment-in-definition violation. Doc comments in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, lines 123 and 130, mention a retired legacy enum (documentation only, no compatibility code).
- One file differs from the set's hash (drift, not a violation).

## Cross-cutting (set-wide scan of 885 entries)

| Rule | Result | Severity |
|---|---|---|
| AGENTS.md edited | none | - |
| Migration/adapter/shim file names | one name hit: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts` (channel adapter, domain term, review) | Low |
| `dbg!` macros | none | - |
| `[DEBUG]` tags left | 111 lines in 58 files across integrate2, cli-zt, testfix-real, all pre-existing (HEAD) | Medium |
| Default language | none found (EN and DE everywhere checked) | - |
| Path length | longest absolute path 181 code points (limit 256) | - |
| External runtime libraries in production | `react` (testfix-real), `wasmtime`, `serde`, `serde_json`, `tokio` (cargo-fix); all pre-existing | High |
| Legacy support | `dialectFromLegacyExampleAppId` (cli-zt) | High |

## Appendix A: `[DEBUG]` lines in the audited sets (file:line)

- `cli-zt`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source/🟦️.ts`: lines 79
- `cli-zt`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts`: lines 104, 120
- `integrate2`: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏱️timers/🟦️.ts`: lines 36
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts`: lines 87, 101, 138, 139
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts`: lines 26, 64
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧪️tests/🟦️.ts`: lines 62
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/📥️birth/🟦️.ts`: lines 91
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️fingerprint/🟦️.ts`: lines 27, 35
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts`: lines 34
- `integrate2`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts`: lines 105
- `testfix-real`: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: lines 441
- `testfix-real`: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: lines 416
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire/🧪️tests/📤️transfer/🟦️.ts`: lines 89
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧪️tests/🟦️.ts`: lines 213
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🟰️change/🎚️record/🧪️tests/🦀️.rs`: lines 20, 21
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🟰️change/📋️indexed/🧪️tests/🦀️.rs`: lines 12, 13
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🟰️change/🧪️tests/🦀️.rs`: lines 13, 16, 18, 25, 30
- `testfix-real`: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🧪️tests/🦀️.rs`: lines 62, 107
- `testfix-real`: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧪️source-and-native-tests/🟦️.ts`: lines 318, 330, 380, 381, 392
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/🧪️tests/🟦️.ts`: lines 13, 27, 34
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/🧪️tests/🦀️.rs`: lines 28, 37, 47, 69
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🧪️tests/🦀️.rs`: lines 48, 65
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🧪️tests/🟦️.ts`: lines 24
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🧪️tests/🦀️.rs`: lines 32
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📥️ingress/📚️original/🧪️tests/🟦️.ts`: lines 10
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/📜️pair/🧪️tests/🟦️.ts`: lines 14, 23
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🔗️binding/🧪️tests/🦀️.rs`: lines 24, 29, 34
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs`: lines 123, 134, 142, 214
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs`: lines 375, 426, 477, 498, 531, 619, 658, 690, 702, 725
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🦀️.rs`: lines 24
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🟦️.ts`: lines 10
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🦀️.rs`: lines 17, 26
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🟦️.ts`: lines 15
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs`: lines 57
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/🌱️birth/🧪️tests/🦀️.rs`: lines 21
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-facets/🧪️tests/🟦️.ts`: lines 9
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-facets/🧪️tests/🦀️.rs`: lines 46, 53
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/⚖️law/🧪️tests/🦀️.rs`: lines 17
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🪪️source-identity/🧪️tests/🟦️.ts`: lines 11
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🪪️source-identity/🧪️tests/🦀️.rs`: lines 17, 36, 45
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🟦️.ts`: lines 30
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🦀️.rs`: lines 97
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🧪️tests/🟦️.ts`: lines 14, 24, 33
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts`: lines 28
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🦀️.rs`: lines 25
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📸️preparation/🧪️tests/🚪️public/🦀️.rs`: lines 54, 73, 82, 86
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📸️preparation/🧪️tests/🟦️.ts`: lines 27, 35
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📸️preparation/🧪️tests/🦀️.rs`: lines 17
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧪️tests/🪪️render-revision/🦀️.rs`: lines 29
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🏗️snapshot-builder/🧪️tests/🟦️.ts`: lines 19
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🏗️snapshot-builder/🧪️tests/🦀️.rs`: lines 76
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🎞️host-events-media-cancel-and-download-seam/🦀️.rs`: lines 272
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🏠️catalog/🧪️tests/🟦️.ts`: lines 3
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🏠️catalog/🧪️tests/🦀️.rs`: lines 7
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/🧪️tests/🟦️.ts`: lines 6
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/🧪️tests/🦀️.rs`: lines 19
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🪢️retained-sync-hello-laws/🦀️.rs`: lines 480
- `testfix-real`: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️unit/🦀️.rs`: lines 15, 46, 102, 194, 245, 281

## Appendix B: live content differs from the set's applied hash (audited at current content)

- `integrate2` (33):
  - `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🧬️schema/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🟦️.ts`
  - `📋️project.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧬️schema/🔣️.json`
  - `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
  - `🧰️framework/🛍️products/🦑️repo/README.md`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🧫️fixtures/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts`
  - `🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/🏛️crate-partition/🔣️.json`
  - `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️child-group-owner.json`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔧️build-surfaces/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/📥️birth/🟦️.ts`
- `input-precision` (7):
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧪️tests/🧬️schema/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`
- `cli-zt` (2):
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts`
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json`
- `testfix-real` (2):
  - `Cargo.lock`
  - `🧰️framework/Cargo.lock`
- `cargo-fix` (1):
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🍎️shell-tests-input-1/📦️packages/🦀️rust/Cargo.toml`

## Limits

- Rule checks are regex-level on the current content, not a full parse. Docstrings are checked only where a line starts with `/**`, `//!` or `///`. Comments inside definitions are detected only on indented full-line `//` or `/*` comments; trailing comments are not covered.
- English-only detection is limited to registry-style `en`/`de` objects and to visible user-text patterns. Concise-code is not measured, only noted.
- No file was built, run or formatted. Staged-only tool outputs were not audited.
