# 🧯 Build Hazards Audit 2026-11-25

Read-only audit of the Rust sources in `🧰️framework`, `✏️s/🔌️plugins/📕️norm` and `🌎️hub/🧩️compositions/📕️norm`. No cargo, no builds, no git writes, no files touched except this report. The tree is live; counts reflect the scan at about 10:30 local time.

Scope: 22,313 `.rs` files (framework 10,418, norm plugin 11,854, hub composition 40 at scan time). Method: a Rust lexer pass that blanks comments and string or char literal contents, then resolution of `#[path]`, `mod x;`, `include!`, `include_str!`, `include_bytes!`, and `concat!(env!("CARGO_MANIFEST_DIR"), ...)` targets against the filesystem. Scratch scripts and raw output live in `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-gBqsL2/b312cdbe-9b14-4aed-8d2a-5322451ba23d/scratchpad/` (`pathcheck3.txt`, `envcheck.txt`, `braces.py`, `debugprints.txt`).

## Summary counts

| Check | Result |
|---|---|
| `#[path]` attributes scanned | 19,227 (315 on inline modules, resolved through the inline-module chain) |
| `mod x;` declarations | 18,924 |
| `include!` / `include_str!` / `include_bytes!` invocations | 12,153 (51 non-literal, mostly macro templates) |
| Absolute `#[path]` / include targets | 5 (all missing, see H1) |
| Missing `#[path]` or `mod` targets, plain | 4 (test-only, see M1) |
| Missing `include` targets, literal | 2 (test-only, see M1) |
| Missing `CARGO_MANIFEST_DIR` include targets (unresolved under any ancestor root) | 29 (see M2) |
| `CARGO_MANIFEST_DIR` includes resolved via an ancestor root | 12 |
| Plain `mod` with no file | 2, both in a template file (see H2) |
| Template placeholders (`@NAME@`) in Rust sources | 8 lines in one file (see H2) |
| Files modified in the last 3 hours | 179 |
| Brace/paren/bracket imbalance in those files | 0 (approximate, lexer-based) |
| Conflict markers in Rust sources | 0 |
| `console.error` / `console_error` in Rust code | 0 |
| `[DEBUG]` print macros in the three roots | 15 (7 production framework, 8 test) |
| `[DEBUG]` production prints in norm plugin or hub | 0 |

## High

### H1 Absolute `#[path]` to a non-existent debug mount (build-breaking)

Five `mod dbg;` declarations point at an absolute path inside a ticket folder whose `🗑️generated/` directory is gone. The target `debug-ledger.rs` does not exist. These files are in the framework core (`🌱️value` decode/record machinery) that norm guests compile, so the build fails with E0583 (file not found for module `dbg`).

Target: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs` (missing; the ticket folder exists).

| File | Line | Modified |
|---|---|---|
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🏗️record-machine/🔀️enum/🦀️.rs` | 3 | 2026-11-25 10:24:53 |
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🏗️record-machine/🦀️.rs` | 10 | 2026-11-25 10:25:23 |
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs` | 2 | 2026-11-25 10:24:53 |
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs` | 6 | 2026-11-25 10:25:16 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs` | 2 | 2026-11-25 10:25:16 |

Fix: remove the five `#[path=...] #[allow(dead_code)] ... mod dbg;` lines, together with the debug module they feed, or point them at a file that is checked in.

### H2 Template placeholders and missing modules in a test fixture

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🔬️isolated/🦀️.rs` is an unrendered template. It is not valid Rust:

- Lines 35, 37: `#[path = @MODULE@]`, `#[path = @LINK_MODULE@]`
- Lines 36, 38: `mod composition_group;` and `mod composition_links;` have no files on disk
- Lines 41, 64, 92, 101, 103, 104: `@COLD_CLOSE@`, `@PRIVATE_GROUP_GRANT@`, `@SIBLINGS@`, `@INITIAL...@`, `@EMPTY_ROW@`, `@LINKS@` placeholders in expression position

Modified 2026-11-23 01:15:31. A sibling file `🟦️.ts` sits in the same folder. This breaks the test target that includes it. I did not verify which test target reaches this file, so the severity is High only if that target builds. Fix: render the template or delete it.

## Medium

### M1 Test-only missing targets (literal paths)

| File | Line | Reference | Target |
|---|---|---|---|
| `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/📦️original/🦀️.rs` | 93 | `#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;` | `📦️original/🧪️tests/` absent |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️receive/🧪️tests/🦀️.rs` | 5 | `#[path="../../🫴️receiving/🎟️work/🧪️tests/📥️bank/🦀️.rs"] mod funding;` | absent |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🗄️owned-receiving-pre733/intrinsic.rs` | 2 | `#[path="../../../🎟️work/🧪️tests/📥️bank/🦀️.rs"] mod funding;` | absent |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🗄️owned-receiving-pre733/work.rs` | 2 | same as above | absent |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel-part-2/🗑️retirement-custody-and-transaction-rows/🦀️.rs` | 195 | `include_str!("🧫️fixtures/🔣️.json")` | fixtures folder absent |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪜️tests-mounted-grant/🦀️.rs` | 18 | `include!("../🧪️tests/🔬️mounted-grant-archive-publication/🦀️.rs")` | resolves under `🧷️companions/`, which has no `🧪️tests/` |

The `pre733` folder name suggests a legacy snapshot; it is still compiled if its parent is a test target.

### M2 `CARGO_MANIFEST_DIR` includes that resolve under no ancestor directory

Crate root is not determinable without `cargo metadata`, which I did not run. I tested every ancestor directory of each include; these 29 resolve under none. All are test code.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:558`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🏷️catalog-staging-command-refusal-and-direct-fixture-laws/🦀️.rs:74`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🏷️catalog-staging-command-refusal-and-direct-fixture-laws/🦀️.rs:104`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🥇️one-item-publication-fixture-and-snapshot-read-lease-laws/🦀️.rs:3`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🥇️one-item-publication-fixture-and-snapshot-read-lease-laws/🦀️.rs:5`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws-unit/🦀️.rs:484`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs:47`
- `…/🔬️wgpu-block-list/🦀️.rs:51`
- `…/🔬️wgpu-event-feed/🦀️.rs:172`
- `…/🔬️wgpu-event-feed/🦀️.rs:420`
- `…/🔬️wgpu-graph-timeline/🦀️.rs:75`
- `…/🔬️wgpu-table/🦀️.rs:234`, `:309`, `:343`, `:376`, `:493`, `:513`
- `…/🔬️wgpu-virtual-file-system/🦀️.rs:110`, `:141`, `:212`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs:171`
- `…/🔌️plugin/🧪️tests/🔬️app-app-builder/🎛️descriptor-owner-layouts-terminology-and-history/🦀️.rs:6`
- `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-native-aggregate-backing/🦀️.rs:2`
- `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/📨️envelope-decode-ladder/🦀️.rs:220`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🔧️replacement-fixtures-and-recursive-documents/🦀️.rs:108`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🧵️recursive-wire-open-and-producer/🦀️.rs:319`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-3/🔚️close-turn-wire-input-and-no-state-disposers/🦀️.rs:25`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-4/🎞️host-events-media-cancel-and-download-seam/🦀️.rs:203`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-4/🔀️selected-ids-merge-vocabulary-and-checkpoint/🦀️.rs:110`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-6-x/🦀️.rs:242`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-7/🆔️request-context-serializer-and-shared-job/🦀️.rs:19`
- `…/🔬️plugin-runtime-plugin-builder-contract-part-7/📜️mounted-history-and-reserved-job-currency/🦀️.rs:5`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🏗️workspace/🦀️.rs:18`

Prefixes are abbreviated with `…`; the full path is the framework prefix `🧰️framework/🛍️products/💻️os/🔨️modules/` (or `🔌️plugin/`) plus the tail shown. Use the file and line to locate each.

## Low

### L1 Unconditional `[DEBUG]` prints in production framework code

These compile into norm guests and fire on every call. They do not break the build.

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/📣️dispatch-and-overlay-api/🦀️.rs:220`, `eprintln!("[DEBUG] ptr window=...")` in the pointer dispatch block, modified 2026-11-24 17:29:59. Hot path (every pointer event).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs:132`, `eprintln!("[DEBUG] ephemeral advance phase ...")` at the top of `advance`, modified 2026-11-25 01:53:10. Hot path.

### L2 Conditional `[DEBUG]` prints in production framework code

- `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs:21`, guarded by `if !SEEN.swap(...)` (fires once), modified 2026-11-24 03:31:09
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/📦️output/🦀️.rs:57`, guarded by a payload-extent error check, modified 2026-11-25 09:23:01
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs:165`, guarded by `if let Some(pending)`, modified 2026-11-25 03:53:04
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/👷️work/🦀️.rs:61`, guarded by `if let Ok(d)`, modified 2026-11-25 09:59:51. This folder is `🧪️testing`, not a test folder; its module is gated by `cfg(any(test, feature = "artifact-app-testing"))`, so it is feature-gated support code.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs:135`, inside the `match self.phase` arm 1, modified 2026-11-25 01:53:10

### L3 Temporary thread-local instrumentation in framework core

`🧰️framework/🔨️modules/🌱️value/🦀️.rs` (modified 2026-11-25 03:53:58) ships `DEBUG_UNITS`, `debug_unit`, `debug_unit_add`, `debug_units_report` as production items. Their doc comments say "remove it with its call site". 24 references across four other framework files call them (the largest is `🏃️mounted-run-and-pump/🦀️.rs` with 15). Not a build break, but it is a [DEBUG] leftover in production code.

### L4 Other [DEBUG] occurrences

- Test-only prints: norm `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/🧪️tests/🔬️unit/🦀️.rs` lines 937, 939, 942, 946, 952 (modified 2026-11-25 09:29:25); plus 3 framework test prints.
- Doc-comment and assert-message mentions: 14 lines, not compiled as prints.
- Norm plugin and hub composition: no production `[DEBUG]` prints.

## Not hazards

- All 179 files modified in the last 3 hours are brace-, paren- and bracket-balanced by the lexer. This is approximate: it does not run rustc, so it cannot see macro-level or type-level errors.
- `#[path = "./\u{1f4b0}\u{FE0F}backing/..."]` at `🛍️products/💻️os/🔨️modules/🔁️workflow/.../📸️snapshot/🧪️tests/🦀️.rs:92` resolves after Rust string-escape decoding.
- Include macros inside `macro_rules!` templates (for example `include_str!(#dependencies)` in `🗣️dsl/✨️derive/`) are templates, not literal targets.

## Limits

- Reachability was not checked: I did not verify which test targets include `🔬️isolated/🦀️.rs` or the M1 files. Severity for H2 and M1 assumes they are compiled.
- Crate roots for `CARGO_MANIFEST_DIR` were not resolved with `cargo metadata`; M2 is a best-effort list.
- The tree is live. Re-run the scan before acting on the list.
