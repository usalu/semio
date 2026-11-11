# 🧭️ Wave 1108 Lane Brief (2026-11-08 evening, weekly quota reset)

Goal: Entwerfen-mit-Bestand demonstrator end to end (`activate-prototype` -> serve 6029 -> e2e 50 + `1105b-tools/e2eready/run.sh` PASS). Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` (folder `C` below). State: `1107-HANDOVER.md` (FINAL-7 is the latest), rulings at the end of `1107-brief.md`, claims log `1105b-claims.md` (tail).

## Paths
```
cd /Users/ueli/Documents/semio
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
```

## Compile
- Foreground only (background children die with your turn): `"$C/1105b-tools/chk.sh" W-<lane>-<n> <manifest> <native|wasm> <cargo check args>`; then read `$G/W-<lane>-<n>.rc` and `.txt`. Tests: `"$C/1105b-tools/runtime/tst.sh"` (read its header). Labels start with `W-` and are unique (increase `<n>`).
- Error list: `awk '/^error/{e=1;print;next} e&&/^ *--> /{print;e=0} /^warning/{e=0}' "$G/W-....txt"`.
- Never run cargo, nx, lock, sync, activate or serve directly. Never raise any limit (source budget, slots, fuel, tables) to make something pass.

## Rules
- AGENTS.md applies (concise code, no comments inside definitions, docstrings start with a unique emoji, no legacy shims, no compatibility layers, clean long-term fix, no CRUD).
- No git writes of any kind (no commit, stash, checkout, restore). No worktrees. No sub-agents.
- Do not open, close or reopen tickets or goals. Do not delete `🗑️generated` or anything in it except your own new logs.
- Peers edit the same tree. Claim before editing: append `HH:MM (11/08) <lane> CLAIMED <crate/files>` to `$C/1105b-claims.md`; append `RELEASED` / `FIXED-verified <label>` when done. Edits stay compile-atomic per file (no half-ported file left behind when you stop).
- Cargo.toml edits: allowed, but append `LOCK NEEDED <manifest> (<why>)` to the claims log; the coordinator runs lock and sync.
- Wallet ruling: no wallet inside a turn. Work is charged to the calling turn through the effect (`cx.with_original_effect(...)`), never through an `AuthoringWallet` shortcut or a grant built by hand.
- Write your result file `$C/1108-result-<lane>.md` after every step (state, labels, rerun command, open items, debts). Last line: `STATUS: FIXED-verified <label>` or `STATUS: BLOCKED <why>`.
- `[DEBUG] ` prefix on any temporary log; remove them before you finish.

## Reference ports of the new `plugin_typed_owner_original(original, active, effect) -> OriginalRetirementPoll` signature (was grant -> Outcome)
- `semio-plugin-ownership` genesis: `1107-result-groupcallers.md` (Round 7+).
- `semio-plugin-actor-host`: `1107-result-toolsplit.md` (W-toolsplit-128).
- `semio-plugin-original-job-ownership`: `1107-result-ownercallers.md` (W-ownercallers-3).
- Store slot/owner recipe (StoreMutationFor / OpBinaryReceiving bounds, effect-shaped retire_owned_mutation): `1107-result-memberstore.md`, `1107-result-gateway.md`, `1107-result-replayport.md`.

## Gateway command (the coordinator runs it; lanes run their own crate checks)
`"$C/1105b-tools/chk.sh" W-gateway-<n> Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws`

## Standing rulings (coordinator, 11-09)
- An unclaimed upstream red that blocks your crate is yours: claim it and fix it without asking.
- Pending keeps state and retries next turn; never turn Pending into a refusal or error (no sync-over-reply).
- Denied turns charge nothing; quotes equal actual cost (reply handoff included); allocations are receipted in the turn that makes them.
- Owner-less closes run in `OriginalRetirementTurn` from `OriginalRetirementPump`; Source-aware work via `with_original_source_effect`.
- Never game the source budget (no formatting collapse); split crates or move tests to `[[test]]` targets.
- Test binaries: hang guard ≤100 s; add fail-fast stall asserts instead of rerunning spinning suites.
- Turn-less test fixtures get producer invocations from the shared driven harness (value-testing-admission); AuthoringWallet must not be used in tests or production.
- (11-10) Never restore a whole file from git HEAD or rewrite a file from an earlier read: peers' edits are lost (declviews 03:14 and proctransient edits were reverted this way). Edit with exact anchors only; if a file looks corrupted, ask the coordinator via the claims log.
