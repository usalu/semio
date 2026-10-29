# OS History Command Syntax Repair — 2026-10-29

## Cause

Canonical Part21 retry58811 cleared root dependency inheritance and admitted101/382 units, then stopped before compiler at kernel unit102: unbalanced Rust source delimiter. A bounded canonical source-reader probe located `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🦀️.rs`.

The exact current file had mtime2026-10-28T13:26:20.541Z,56002bytes and SHA256 `3ed2f062c9ee102b91018b6a0c0aef2e6a4878ff13f45ef0260061c49e7ce751`. Canonical token pairing located unmatched opening brace at line125/offset23672. Independent registered pinned Rustc confirmed the same real unclosed implementation:

`SEMIO_LANGUAGE=en repo tools exec --light rustup -- run nightly-2026-07-20 rustc -Z parse-crate-root-only --crate-type=lib <actual source path>`

Rustc exited1 and explicitly pointed to the line125 helper implementation. This was not a selector false positive. Rustfmt was unavailable in the pinned minimal toolchain; nothing was installed. The Rustc parse-only interface required no linker or Cargo build and emitted no artifact.

## Repair and preservation

Inserted exactly one closing brace immediately before the sibling inherent implementation formerly at145, after `native_vcs_replay_turn` closes. Existing command/funding/ownership bodies were untouched. The mutation checked the entire current original hash and unique boundary anchor before writing, preserving all existing peer bytes. No Git restoration, source copy, broad formatting, dependency migration or guard weakening occurred.

Current hash after the two inserted bytes (`}\n`): `90ecc627952b57a60200586cda73b37d8a3239b7d165ac2cff51b774dd459a01`.

## Schema-first regression and independent oracle

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/📐️syntax` contains a closed neutral schema, three synthetic boundary cases (unclosed helper, correctly closed siblings, nested body), and focused test. Existing selector acceptance is compared against the pinned real Rustc parse-only implementation through the registered repo/rustup tool interface. The actual history file is also parsed by both implementations. No runtime library dependency or production test API was added.

- Red:1 passed,1 failed; actual source rejected as unbalanced.
- Green:**2 passed,0 failed,15 assertions**, authoritative exit0.
- Command:`SEMIO_LANGUAGE=en repo tools exec --light bun -- test './🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/📐️syntax/🟦️.ts'`.
- Logs:`🗑️generated/kernel-syntax-red.log`, `🗑️generated/kernel-syntax-green.log`.
- Existing bounded source closure location probe `kernel-source-location.ts` now passes15reads. Its approximate host cfg is diagnostic evidence only, not a compiler receipt.

## Changed files and next actual gate

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🦀️.rs`: one implementation closing brace.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/📐️syntax/🔣️.json`: neutral cases.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/📐️syntax/🧬️schema/🔣️.json`: closed schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/📐️syntax/🟦️.ts`: selector/Rustc oracle and actual-source regression.
- This report; existing workspace report retains58811 causal result.

Source is ready/frozen. No Cargo process, service, or duplicate job is active here. The next coordinated canonical Part21 attempt must still prove full admission, compilation, and actual named law execution. Parse-only success is not semantic compile or runtime success.

## Canonical retry99185 after syntax repair

Parent authorized the next sole canonical exact Part21 law. Fresh capacity was2950MB headroom/no waiters; ordinary governor retained admission authority. **Authoritative terminal1**, no compiler or law execution. Source admission passed the repaired kernel and completed129/382 units. Unit130 `semio-framework-os-store-engine` then exceeded its unchanged production Source byte cap: close-host-drop-and-pair-merge source18130bytes with only13708bytes remaining. Log `🗑️generated/part21-kernel-fixed.log`.

This is a new owning crate boundary defect. The offending file is not itself larger than the total cap; preceding mounted files already consumed the budget. No isolated-file limit or tokenizer failure is inferred.

A bounded read-only diagnostic (`store-engine-source-extent.ts`, max96files/2MiB, approximate hostcfg) observes the complete production closure: **30files,607408bytes,4423active lines**, compared with declared32files/262144bytes/4000lines. Its full path/hash inventory is `🗑️generated/store-engine-source-extent.json`. This diagnostic makes no compiler admission claim and publishes no product. The largest inputs include command56KB, lifted root49KB, delegates39KB, replay publication37KB and branch transitions36KB. Merely partitioning test targets cannot repair this production closure; existing production ownership needs coherent decomposition. Coordinator was notified to align Linux/OS owner scope. No source cap/guard bypass, broad move, retry, or peer mutation was performed. No current live process remains here.
