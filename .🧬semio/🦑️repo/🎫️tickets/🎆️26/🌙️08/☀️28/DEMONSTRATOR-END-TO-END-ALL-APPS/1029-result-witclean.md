# 1029 result: witclean

STATUS: GREEN for the slice (3d wasm workspace + TS + JCO descriptor test green). The hub batch is RED only from a foreign edit (see Verification), so hub is not yet confirmed.

## Removed (P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`)
- `P/🧬️schema/📜️reactor.wit`: exports `reserve-poll-input`, `write-poll-input`, `close-poll-input` and the records `actor-poll-input-call`, `poll-input-page`. The "at most one event per poll" LAW doc on `poll` is kept.
- `P/📣️macros/⚛️wp4-reactor/🦀️.rs`: the three guest export impls.
- `P/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`: `poll_input_call`, `reserve/write/close_poll_input_wit`.
- Whole dead resident-source machinery: folder `P/⚛️reactor/🎟️context/🌉️call/📥️input/` (OriginalActorPollInput, `*_poll_input_step`, its Rust test, TS source test, fixtures, schema) deleted; `call/🦀️.rs` loses the `mod`, the `poll_input` field, its `terminal_is_empty` term and the `has_original_poll_input` branch in `close_step`.
- `P/🧬️guest-contract/📦️packages/🦀️rust/Cargo.toml`: `[[test]] original_actor_poll_input`. (dev-dependency `wasmtime` there is now unused; left because dropping it touches Cargo.lock, which only the coordinator may sync.)
- `P/🧠️interpreter/📦️packages/🦀️rust/📋️project.json` + `📜️script.ts` + `P/🧠️interpreter/🧪️tests/🔬️rust-package-commands/🟦️.ts`: targets/classes/runner `test-poll-input-source`, `test-poll-input-native`.
- `P/🌐️browser-bundle/🛂️descriptor/🧪️tests/🧫️fixtures/🔣️.json`: the three names. That test compares the fixture with independent JCO-generated types of the WIT.
- No hits remain in ✏️s, 🌎️hub, other TS bindings (rg of all spellings, excluding tickets). `owned_abi::PollInput` / `poll_owned` is a different, live path and untouched.

## Note
I ran `git rm --cached` on the deleted folder by mistake (index only); the follow-up `git reset` was refused by a peer's index.lock, so the five deletions show as staged. Working tree is as intended.

## Verification
- wasm32-wasip2 check of `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml --workspace`: rc 0, 0 errors, `Finished` (`🗑️generated/1029/witclean-3d.{txt,rc}`). It compiles the full `semio-framework-plugin*` chain including guest-contract, reactor and the actor-export macro, so the WIT/guest removal is proven to compile on wasm.
- Hub batch (13 packages of `matrix/run.sh chk hub`): rc 1 (`witclean-hub.txt`). Only failure: `semio-framework-value-retirement` E0277 x3 at `value/♻️retirement/🎮️controlled/🦀️.rs:59` (`ControlledRetirement<T>` / `dyn RetirementCursor` not `Sync`). Cause is a foreign uncommitted edit (mtime 17:26) in `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs` that changed `trait ErasedSnapshotRetirement: Send` to `Send + Sync`, which contradicts the `RetirementCursor: Send` docstring ("shared access belongs to an explicit owner gate"). The check aborted there, before any plugin crate. I did not touch it (design call: revert to `Send`, or propagate `Sync` to `RetireOwned`/`RetirementCursor`). Re-run `1029-tools/witclean/chk.sh` hub step after the owner settles it; no poll-input symbol appears anywhere in the log.
- tsc (`1029-tools/tsfront/tsconfig.json`): only the 2 known `@semio-tech/puzzle-wasm` not-found errors (`witclean-tsc.txt`).
- `bun test` of `browser-bundle/🛂️descriptor/🧪️tests/🟦️.ts`: 3 pass, 0 fail. It compares the fixture and `ACTOR_COMPONENT_EXPORTS` against exports derived by JCO `types()` from the edited WIT (independent oracle).
- No bridge/materialization TS referenced the removed exports, so none changed (the bridge's own tests were not re-run).
