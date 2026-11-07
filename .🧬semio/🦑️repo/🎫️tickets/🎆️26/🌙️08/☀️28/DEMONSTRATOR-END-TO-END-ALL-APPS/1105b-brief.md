# 🧭️ 1105b Fleet Brief — mit Bestand Demonstrator End To End (wave 1105b)

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Folder = C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Coordinator = main chat (Opus). NEVER open, close or reopen the ticket. No sub-agents. Foreground only.

## Goal
`@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → serve on 6029 → `test-e2e` green and `C/1105b-tools/e2eready/run.sh http://127.0.0.1:6029/` all PASS. All 8 panes (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) boot, show their fixture, act (with undo/redo) in en and de in a real browser. The coordinator alone runs activate, prepare, serve, test-e2e, graph queries and `dependencies sync|lock`.

## State at 10:15 Nov 5
- Nothing is running. The previous coordinator (1105 session) is gone; its agents cannot be resumed — you inherit your lane. Its last checkpoint is `C/1105-root-checkpoint-0229.md` (READ IT): many source slices were written but most native proofs are UNRUN.
- Last activation attempt: act21 (11/02) stopped on a path-length fault (fixed by renames). No activation since. No pane has passed in a browser.
- Host disk is 99 % full (16 GiB free). Rust cache image `.r/i` (96 GiB, 59 GiB free inside). Host RAM 32 GiB.
- Prior waves: `C/1028-HANDOVER.md` ("WAVE 1102"), `C/1028-coordination.md` (tail), `C/1102-result-<lane>.md`, the `C/1103-*`, `C/1104-*`, `C/1105-*` reports of your area.

## Output
- Generated output: `C/🗑️generated/1105b/<lane>-*.{txt,rc}`. Never delete or sweep `🗑️generated` (no rm on it, no clean/purge targets).
- Scripts: `C/1105b-tools/<lane>/`. Result: `C/1105b-result-<lane>.md`, first line `STATUS: ...`, updated after every check. Say GREEN only for what a finished run proved; say UNVERIFIED otherwise. Cite the log file.
- Final message: at most 5 lines plus the result path.

## Building and testing
- Cargo only through `repo cargo --store=shared --configuration=prototype -- <check|test> --manifest-path <M> [--target wasm32-wasip2] --keep-going -p <crate>...`. Raw cargo is refused. Template: `C/1105b-tools/chk.sh <lane>-<n> <manifest> <wasm|native> <args>` (check); copy it to `C/1105b-tools/<lane>/` and adapt for `test`. Usual manifests: `🌎️hub/Cargo.toml` or the plugin workspace `Cargo.toml`.
- EVERY `repo cargo`/`repo test`/`repo run` goes through the fleet slot semaphore `C/1105b-tools/slot.sh <lane>-<n> <command...>` (chk.sh does). Slot count is in `C/1105b-tools/slots.k` (coordinator only). Never bypass it, never remove other lanes' slot dirs.
- Native cargo misses wasm-gated code: a compile verdict for a guest crate needs `--target wasm32-wasip2`.
- ONE cargo/repo run per lane at a time, small batches. Never set `SEMIO_LEASE_PRIORITY`. Queue waits of 30+ minutes are normal; never raise deadlines or limits.
- Run detached with nohup and an `.rc` marker, then wait in the foreground with repeated Bash calls: `for i in $(seq 1 18); do [ -f <rc> ] && break; sleep 30; done` (≤ 9 min per call). macOS has no `timeout`. Background children die when your turn ends: never end your turn while your own run is in progress; record its pid and kill only that tree if you must stop. While waiting do static work (reading, writing patches, tests).
- `bun test` only with explicit `./` paths.
- Always `cd /Users/ueli/Documents/semio` in every Bash call. Use `/usr/bin/grep`. zsh: no bare `=====`, no word-splitting of unquoted vars.
- Never pattern-kill (`pkill -f`, `killall`). No Monitors. Do not start servers. No `repo show`/`repo graph`/`repo run …:activate*`/`dependencies sync|lock`.

## Phase now: EDIT PHASE (no activation running)
Lanes may edit production source in the crates/files their lane owns. A freeze is announced by an addendum before the next activation (marker `C/🗑️generated/1105b/freeze`); when that marker exists, production edits become patches in `C/1105b-tools/<lane>/patches/`.

## Editing
- Peers edit the same tree concurrently: re-read before every edit, compile-atomic edits (definition before re-export), never revert foreign changes. If a file was modified by someone else in the last 10 minutes, do other work and retry. Don't stop because others work nearby.
- No git modifying commands (commit, stash, checkout, restore, rm, worktree). No `sed -i` codemods without a fail-closed scope. No automated rewrites across crates.
- Per AGENTS.md: schema-first, concise code, no comments inside definitions, docstrings start with a unique emoji, no compat shims/adapters/deprecations, no limit raises, no new `#[allow]`, no `.unwrap()/.expect()` in non-test code, temporary logs prefixed `[DEBUG] ` and removed before you finish, paths < 256 chars, UI strings en + de with no default language. New features get a language-agnostic test plus a third-party oracle where feasible.
- The work-accounting API (`RetainedWorkOutcome{result,work}`, `RetirementStep::{Progress,Failure}(..,u64)`, staged `RetainedCloneSourceAdmission`, `NativeReceive::receiver()`, `RetainedWorkLedger`, original caller/decoder/view-read owners from 1105) is final: mirror it, never redesign or revert it.
- Fix root causes in the owning crate. Root cause in another lane's crate → write it in your result file AND message the coordinator one line.
- PRIORITY: the shortest path to a green activation and a pane that works in the browser. Prefer finishing/proving existing half-done slices over starting new redesigns. Run the native mounted test of your pane early: a green wasm check is not runtime-ready.

## Addendum 10:35 — repo cargo blocked (authorizer closure)
Every `repo cargo` currently fails in build:source-admission: "Authorizer source closure capacity exceeded: bytes=16788269/16777216". NOT a compile error. Lane hub owns the fix (shrink by root cause, no limit raise); marker `C/🗑️generated/1105b/admission-ok` appears when fixed. Until then: do static work (read, edit your crates, write tests/laws, bun tests), then rerun your check once the marker exists. Do not edit repo-library authorizer/graph files unless you are hub or actinfra.

## Lane agent ids (SendMessage `to`) — authoritative
disk a99ac1d46b8fc8259 · hub a422b4a04f9ce1faf · runtime ab5d53c73baeac213 · flow aefa042877b1cdaff · proc a1f6148f09cd19d01 · reacthost (ex-drawing) a36f6e3455c1070bd · koordinator a01f03cc9bc5afdbd · aggregator af4d4d8f361e2acb8 · aussuchen a0b73156048e94641 · bearbeiten a341cd7955fa00149 · verfolgen a8782c396d8556597 · energie ab4e1ea749f88b0fa · statik ac5740f3ccbf1c31c · actinfra aa65e868d26e8083e · e2e afdffef8e79bd7bc2 · wit (ex-session) ae08ed87e5db4b631 · gates a04342ffed0133d46 · audits: aud-panes aa76c54931c59d1f9, aud-policy a7d67fe790926d12a, aud-e2emap a41b77ff451f639b5. Earlier messages from the coordinator may have named wrong ids; this table wins.

## Addendum 10:40 — admission fixed; NEW blocker: rust store image
The authorizer overflow is fixed (hub). Every repo cargo now dies at `hdiutil resize -size 98304m .cache/b/r/fixed.dmg` (exit 22). Lane disk owns it; marker `C/🗑️generated/1105b/storage-ok` appears when a repo cargo gets past storage. Wait for BOTH admission-ok and storage-ok before rerunning cargo; static work meanwhile. Never create/resize/delete cache images yourself.

## Addendum 11:00 — Cargo.lock refresh refused (half-done renames); WIT redesign
- Every repo cargo now dies in the Cargo.lock refresh (CargoLockRefused / no matching package) because of half-finished crate renames from the 11-05 morning commits. Lane `renames` (a04342ffed0133d46, ex-gates) is the SOLE owner of package names, dependency keys and `semio_*::` crate paths across all Cargo.toml/.rs. Do NOT rename packages or dependency keys yourself; send renames the exact straggler (file + key). Marker `C/🗑️generated/1105b/lock-ok` when the refresh is clean; wait for it, static work meanwhile.
- The plugin WIT (11-05 08:45) moved turn inputs to `admit-actor-input(wire, budget)` + `actor-context-budget`. Lane `wit` (ae08ed87e5db4b631) completes that redesign (wire encoding, guest macro, host, TS bridge). Don't revert the WIT; route WIT/ABI compile errors to wit.
- 11:10: slot.sh is GATED while `C/🗑️generated/1105b/gate` exists: only `renames*`/`W-*` labels get slots. Your slot.sh call simply waits; do static work in parallel and wait with bounded loops. Nobody but renames edits package names / dependency keys.

## Addendum 11:15 — DEPENDENCY SYNC running (coordinator) → MANIFEST/LIBRARY FREEZE
The dependency generation receipt is stale (nx patch changed 05:31), so every repo run re-verifies it for ~45 min. The coordinator runs `dependencies sync --all` now (25-50 min). Until `C/🗑️generated/1105b/sync-done` exists: NO edits to any package.json, bun.lock, Cargo.toml, Cargo.lock, or files under 🧰️framework/🛍️products/🦑️repo/ and 🧰️framework/🔨️modules/🏃️process/ (each edit restarts the sync). Rust/TS source edits elsewhere are fine; your queued cargo checks may wait on the sync — that's expected. Crate splits needing new Cargo.toml (runtime decl-app split): prepare files fully, then land them right after sync-done.
- 11:16: every crate depending on semio-plugin-decl-app hits the 32-file source refusal until runtime lands the split (after sync-done). Marker `C/🗑️generated/1105b/declapp-ok` = rerun. Don't retry before it.
- 11:25: SYNC DONE (rc 0) — manifest/library freeze LIFTED. declapp-ok exists. Run your checks.
- 11:28: framework compile errors in crates no lane owns: first lane to hit one claims it in `C/1105b-claims.md` (one line), fixes root cause, marks FIXED. Never fix a claimed line.
- 11:31: half-landed value receive refactor (value_native_owned root symbols, OriginalReceivingPrimary, native_decoding NativeReceive, MediaForm::Receiver) → lane value (a04342ffed0133d46, ex-renames). Marker `value-ok`. If your check dies only on those errors, wait for value-ok.
nativetests a642d8a047a7946a8 · offclosure a1b91cbbf7e70e7c8 · aud-claims a91751fee9e6a024d · aud-act22 a17773e78741512c2 · aud-debug a7b9eae85ef286f2b · store (ex-graphinf) ab8efa49ed316a4e2 · value (ex-renames/gates) a04342ffed0133d46
- 17:25: PANE SMOKE PRIORITY — native pane smoke/mounted test runs may use slot labels starting with `W-` (priority slots). Checks and off-path tests keep normal labels.
- 22:01: LOAD-BUDGET finding: browser host grants Interactive 2,000,000 units/turn; fixture caller.json was 32,768 (fixture artefact). Runtime sets load-law fixture authority = host Interactive constant. All pane lanes: after runtime's FIXED line, rerun your native smoke (W- label) and record the first document-load poll work_spent in your result file (runtime needs all 8; flag any >1.5M).
- 22:51: SPIN ROOT CAUSE found (maintenance-ladder stage 18). When runtime's claim line shows FIXED, every pane lane reruns its native smoke (W- label) and removes its temporary [DEBUG] prints.
| runtime-b | a8599011fd5af99bf | retained-command effect retirement (statik) |
| hostcheck | af554483849c36e72 | all semio-plugin-host-* native compile, one keep-going pass |

## ADDENDUM 07:29 — cargo hold until sync-2
All lanes: plugin Cargo.locks lack semio-framework-job-context/replacement in the shared registry. Do NOT start cargo until `🗑️generated/1105b/sync-2.rc` is 0 (lock-22.rc alone is not enough). Then rerun.
## ADDENDUM 07:29 — correction: wait for sync-3.rc = 0 (root Cargo.toml entry removed after sync-2 started; lock-23 + sync-3 follow)
## ADDENDUM 07:48 — cargo HOLD LIFTED (lock-23 + sync-3 rc 0). First plugin run reports whether its Cargo.lock refresh resolves job-context/job-replacement; report CargoLockRefused/registry errors to coordinator immediately.
| aud-context-alias | a4fe4bc814d0e3376 | static audit of SealedShared context aliases on close |
| opbin | (see agent list) | hand-written OpBinaryReceiving impls + round-trip laws |
## ADDENDUM 13:47 — slots.k=7 (critical pane smokes first). Non-W lanes: expect longer queue waits; do not bypass slot.sh.
