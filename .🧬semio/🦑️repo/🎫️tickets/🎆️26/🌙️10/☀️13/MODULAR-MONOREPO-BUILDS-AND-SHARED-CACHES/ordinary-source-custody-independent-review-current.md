# Ordinary Source Custody Independent Review

Read-only Source inspection, 2026-10-15. No tests, Originals, production edits or diagnostic changes. Root reports SDK8d0 product Native BOTH passed 2/34, then ordinary prototype85834 cold compiled three units and failed selected Source physical-facts recheck. Those supplied outcomes were not rerun here. Nx owns exact-path/facts diagnosis; this audit does not attribute the failure to a coeditor or weaken physical guards.

All library-relative paths below are beneath `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Authored ordinary fixture ordering

`⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` 129–132 creates its fresh Cargo workspace, three manifests and three Rust Source files, then generates the initial Cargo.lock before the phase loop. At 133 phase mutations (workspace member addition/lock regeneration, lock comment, package description, Rust leaf change) and restore output deletion happen **before** `prepareSelectedRustBuildGraphV1` at 135 and before that phase's Native snapshot. Cold has none of those phase mutation branches.

The phase then awaits actual `runOwnedNxInvocationV1` at 135 before reading run records/receipts at 136–138. Those post-run operations read artifacts and mutate only in-memory comparison maps. The next phase cannot start its authored mutation before the current invocation resolves; the enclosing source-directory removal at 140 runs only after the loop unwinds or fails. No deliberate ordinary-fixture source mutation or fixture cleanup inside the cold awaited invocation was found.

This excludes those explicit same-test mutations as an explanation of a final snapshot recheck failure. It does not exclude writes by child/tool/generated-input owners, parallel tests, or another writer; naming such a writer requires the exact failing path and actual facts.

## Snapshot owner behavior

`🏗️build/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts` 36 copies the exact inventory to the graph image, holds its slot EX plus snapshot reader, awaits the child callback, then rereads **original workspace** paths before returning. Its finally at 37 removes only the graph-image slot after the awaited body and final checks. The child owner at 49 also checks originals and snapshot copies before and after its callback. Neither owner deliberately edits the original inventory.

The reported error at 26 is the initial per-file gate: actual regular-file type, no symlink, nlink exactly one, and size exactly the admitted bytes. It precedes content hashing and stable descriptor checks. It therefore does not by itself mean digest mismatch, deletion, transient ABA, or Source text edit. An ENOENT would instead come from lstat; exact path/facts determine which condition failed. The same helper serves original and projected roots, so stack/caller evidence is needed to identify which was being read.

## What 48 files / 521,836 bytes can include

`🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts` 30–31 derives a union of selected unit task inputs and actual Nx build-engine inputs, captures those paths, then reports the union count/byte sum. It is not a Rust-only Source-file census and not a package-directory walk.

Concrete possible categories are:

- Actual Rust source/generated inputs in each selected unit and its selected dependency closure.
- Published generated unit descriptor JSONs, explicitly inserted by producer `⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/🟦️.ts` 36–37.
- Sealed compiler program and receipt (`code.program`, `code.receipt`) designated defining configuration inputs at producer 32–34.
- Manifest/workspace lock/configuration/toolchain facts where included by the admitted task input construction; producer 33–34 records manifest/workspace Cargo.toml/Cargo.lock and applicable ancestor Cargo configuration/toolchain files, with graph preparation retaining produced inputs at `🏗️build/🕸️graph/🦀️rust/🟦️.ts` 46–48. The snapshot itself only receives paths actually present in selected task inputs plus engine inputs, so not every preparation fact is automatically a snapshot member.
- Actual build-engine Source/configuration inputs returned by `nxBuildEngineInputsV1` (selected Native owner 31), independent of the tiny Cargo fixture.

Final selection deduplicates paths; generated files can consequently be protected as defining inputs just like authored Source. The observed total alone cannot establish exact category counts or which generated-file owner may write it. Compiler .rlib outputs, output charge files, Nx run.json and output logs are not automatically part of this union merely because the cold child writes them; an explicit selected task/engine input declaration would be required.

## Actionable boundary

Keep the exact physical guard. First identify the actual failing member and whether original or projected root failed; classify it against the categories above and its real writer. If a declared generated input is legitimately changed by the child while held, fix its ownership/publication ordering or its defining-input classification rather than exempting generated paths wholesale. This Source audit found no authored cold fixture mutation inside custody and makes no cause attribution beyond that.
