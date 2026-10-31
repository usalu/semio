# 🧭️ 1030 Fleet Brief — mit Bestand Demonstrator End To End (wave 1030)

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` (folder = C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`). Coordinator = main chat (Opus). NEVER open/close/reopen the ticket. Read `C/1028-brief.md` (incl. addenda) and `C/1028-HANDOVER.md` section "WAVE 1029" first; your lane's prior result file is named in your prompt.

Overrides for 1030:
- Generated output: `C/🗑️generated/1030/<you>-*.{txt,rc}`; scripts: `C/1030-tools/<you>/` (you may copy/adapt `C/1029-tools/<lane>/` scripts; fix their output dir to 1030); result file `C/1030-result-<you>.md` (STATUS line first, update after every check). Never delete `🗑️generated` (coordinator owns it).
- Goal: `@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → `serve` 6029 → `test-e2e` green; all panes render and act. Coordinator alone runs activate/prepare/serve/test-e2e/`dependencies sync|lock` unless your prompt says otherwise.
- Cargo: always via `repo cargo --store=shared …` (raw cargo is refused). Image headroom admits ~3 cold cargos at once: batch crates into ONE check per workspace, `--keep-going`, never several parallel checks of your own. Detached check + `.rc` marker + bounded wait loop (`until [ -f x.rc ]; do sleep 30; done` with a max), never end your turn while your own check is queued or running. macOS has no `timeout`.
- No sub-agents, foreground only. Never pattern-kill (`pkill -f`); kill only pids you recorded. No git modifying commands (commit/stash/checkout/restore/rm).
- Peers (other agents, other sessions, Codex) edit the same tree concurrently: re-read before every edit, compile-atomic edits, do not revert foreign changes. No compat shims/adapters, no limit raises, no new `#[allow]`, no `.unwrap()/.expect()` in non-test code, temporary logs prefixed `[DEBUG] ` and removed before you finish.
- Policy (AGENTS.md): schema-first, concise code, no comments inside definitions, docstrings start with a unique emoji, multi-language tests where a feature is new.
- Final message: ≤5 lines + result path.

## Addendum 11:30 — queue waits
Capacity queue waits of 30-90 min are normal now (host RAM 32 GiB + 48 GiB rust image). Never end your turn while your own detached check runs: wait with repeated foreground Bash calls `for i in $(seq 1 18); do [ -f <rc> ] && break; sleep 30; done` (each ≤ 9 min), as many as needed. Background children die when your turn ends — if your driver script is gone, relaunch it with nohup. The activation (coordinator) runs with SEMIO_LEASE_PRIORITY=interactive; do not set it yourself.

## Addendum 15:20 — usage checkpoint (until the 5h window resets ~16:10 local)
Finish your current edit (compile-atomic), update your result file STATUS, then ONLY wait on already-running checks with long bounded waits (≤9 min each, minimal output). No new investigations, no new reviews, no long reasoning until the coordinator sends "resume". Keep your drivers alive.

## Addendum 16:12 — RESUME
The 5h window reset. Addendum 15:20 is lifted: continue your lane at full speed (addendum 11:30 still applies).

## Addendum 18:10 — EDIT FREEZE during activation
Until the coordinator lifts it: do NOT edit files under `🧰️framework/🛍️products/💻️os/**`, `🧰️framework/🔨️modules/📡️replication/**`, `🧰️framework/🔨️modules/🏃️process/**`, `🧰️framework/🔨️modules/🏪️store/**` (activation source admission of os-plugin-describe-rs restarts on every peer save and gives up). Prepare such edits in your result file / a scratch copy and apply them after the freeze is lifted. Plugin and other framework edits are fine. When you must define + re-export, write the definition first and the re-export last.

## Addendum 18:55 — FREEZE LIFTED
Addendum 18:10 is lifted (source admission now waits out recently edited files). Apply your prepared framework fixes now: compile-atomic, one file at a time, definitions before re-exports, no rapid repeated saves of the same file.

## Addendum 00:30 — peer work-API migration
A peer session is migrating framework ◻️2d, 🔲️pixels and now 🌱️value/♻️retirement (+ 📋️list/🔗️persistent) to a work-accounting API (RetainedWorkLedger, 🎟️work schemas). Do NOT edit inside 🧰️framework/🔨️modules/🌱️value/♻️retirement, ◻️2d, 🔲️pixels unless the coordinator assigns it. Compile failures rooted there are expected; prepare your own fixes and rerun when the coordinator announces the area quiet. Keep working on non-blocked items (tests to write, TS/Python, schema, design).

## Addendum 11:55 (10/31) — PORT WAVE (new goal: finish end to end, all agents)
We no longer wait for the peer. Port lanes update CALLERS of the peer's new 🌱️value / 🧵️job / ⏳️async work-accounting API (e.g. `close_step` → `RetainedWorkOutcome`, `RetirementStep::{Failure,Progress}` extra u64, `NativeReceive` without `receive_birth_demand`/`begin_receive`, RetainedWorkLedger-driven `advance`). Rules: read the peer's current definitions first and mirror them exactly; never revert or redesign the peer's API; if a file you need was modified by someone else in the last 10 minutes, skip it and retry later; compile-atomic edits; one batched `repo cargo --store=shared` wasm `--lib` check per lane (rust image is 96 GiB now); report per-crate green/red in your result file. Integrator = lane fwfix (re-probes and assigns the next wave).
