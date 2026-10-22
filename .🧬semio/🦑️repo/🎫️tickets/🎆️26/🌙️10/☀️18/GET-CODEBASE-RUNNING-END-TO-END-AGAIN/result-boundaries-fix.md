# Result boundaries-fix (wave 1022a)

Tools: `C/boundaries-fix-tools/` (`scan_facade.py` use-tree scan of `semio_framework_value::<name>` importers, `size_today.ts` gate counter over lib/test x mac/linux/windows/wasm + all features, `registered.ts` registration check).
Corpora: db `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts` (+ fixture `🔣️.json`), os `🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` (+ fixtures `🔣️.json`, `🧬️cuts.json`). Both are hand-authored (no generator): rows are validated against the manifests / source closures, so stale rows were updated to the measured truth.

## Outcome
- `bun test ./<db>/🧪️tests/📦️crate-boundaries/🟦️.ts`: 45 pass / 0 fail (was 3 red).
- `bun test ./<os>/🧪️tests/📦️crate-boundaries/🟦️.ts`: 131 pass / 0 fail (was 3 red, then 4 more cut rows surfaced behind the first failing assertion).

## Per row verdict
| row | verdict | action |
|--|--|--|
| db ids | STALE: `semio-framework-value` is only a `[dev-dependencies]` entry, used only by `ids/🧪️tests` (rg: no production use) | removed `semio-framework-value` from the row |
| db engine-base | STALE: `use pack::ContentHash;` in the lib root, manifest has `semio-framework-pack` | added `semio-framework-pack` (not forbidden, no cycle) |
| db io-core | STALE: production code in `🚦️executor-driver-and-faults` uses `protocol::` (= semio-framework-replication) and `semio_framework_value::retained_clone` | added `semio-framework-replication`, `semio-framework-value` (corpus test also green for the earlier async-sync/async-testkit rows from rust-carve) |
| os durable-group-wire | STALE ceiling: a peer carve (staged) moved the 236-line `three-member-decision-record-and-codecs` into wire (closure 1,388 -> 1,624 active lines; gate 4,000, bytes 123 KB) | `maximumActiveLines` 1500 -> 1700 |
| os space-member | CODE VIOLATION: the closure mounts `🪪️member-facets/🧊️reference/🦀️.rs`, whose `#[macro_export] macro_rules! reference_space_members` still spelled `$crate::os_store::...` / `$crate::os_vcs::...` (the pre-split facade, which space-member must not name) and had ZERO invokers in the whole repo | deleted the dead macro (no re-export/adapter possible: space-member cannot reach those types through `$crate`); closure 1,961 -> 1,895 lines, no forbidden token left |
| os cut `retirement_queue` | facade re-export `pub use semio_framework_value_retirement_queue as retirement_queue;` was removed from the value facade (committed b7b9f105a82; greenfield: no re-exports) | cut consumer re-pointed to a real consumer (`⏪️time-travel/🎮️decision/📨️command/🦀️.rs`, canonical = `use semio_framework_value_retirement_queue::RetirementQueue;`); code fix at the root: the only remaining importer of the removed path, `🔌️plugin/🪟️window/📬️mutation/🧪️tests/🦀️.rs:23`, now says `semio_framework_value_retirement_queue::RetirementQueue::default()` |
| os cuts `original-aliases`, `paged-text`, `ordered` | same cause (re-exports removed from value facade in b7b9f105a82); canonical strings re-pointed to real consumers (plugin tool-machine, pack value encode expression, value companion tests) | `🧬️cuts.json` |
| os cut `durable-group` | owner moved: `DurableOwnedThreeMemberDecisionV1` now authored in `semio-framework-os-store-durable-group-wire` (peer carve), exported by the store root via `pub use ..._durable_group_wire::lifted_durable_group::*;` | producer + canonical updated |

## Sizing laws for today's crates
`size_today.ts` (lib and test cfg, mac/linux/windows/wasm, all features) over 17 manifests: ALL WITHIN GATE (262,144 B / 32 files / 4,000 lines). Max per crate: async-timer 3 %, async-sync 3 %, async-slots 6 %, async-testkit 4 %, pack-record-encode 43 %, pack-record-encode-cursor 66 %, os-store-engine 85 % (3,003 L / 15 f), engine-close 6 %, engine-member-group 9 %, presence 66 %, replay-operation 54 %, replay 50 %, ephemeral 35 %, read 42 %, space-member 45 %, durable-group-wire 47 %, durable-group 59 %. `registered.ts`: all 9 created crates have rust-source-boundary metadata, project.json, a build-owners row and a workspace member entry.

## Changed files
- `🛢️db/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (3 rows)
- `os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (durable-group-wire ceiling), `os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🧬️cuts.json` (5 cuts)
- `os/🔨️modules/🏪️store/🧩️composition/🪪️member-facets/🧊️reference/🦀️.rs` (dead macro removed)
- `os/🔨️modules/🔌️plugin/🪟️window/📬️mutation/🧪️tests/🦀️.rs` (import path)

## Open (not touched, not corpus rows)
Importers of the value-facade names removed in b7b9f105a82 are still dangling (scan: `boundaries-fix-tools/scan_facade.py ordered,paged_text,OriginalAliasBatch`):
- `semio_framework_value::ordered::...` / `::paged_text::...` in `os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/{⚖️laws,🛫️encoder,🫙️owner}/🦀️.rs` (compiled by companion `🧷️companions/🍎️tests-01`, which has `semio-framework-value-ordered` but no `semio-framework-value-paged-text`; a peer has the `⚖️laws` Cargo.toml staged right now, so I left them)
- `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🧬️generation/🪶️sqlite/🚦️native/🌱️value/🦀️.rs:5` `semio_framework_value::ordered::RetirementStep` (needs `semio-framework-value-ordered` in the crate + workspace dep + lock).
- Not verified by cargo (tests-only change, cargo/lock work per workspace avoided to not collide with peers).
No processes started; nothing left running.
