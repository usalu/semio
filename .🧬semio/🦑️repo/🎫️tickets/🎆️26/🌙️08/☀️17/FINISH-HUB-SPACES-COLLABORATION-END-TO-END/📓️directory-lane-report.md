# Directory lane wiring — report

## Bottom line

**The directory lane already works, end to end, on the code as currently committed.** No source
change was needed or made. I could not reproduce the handoff note's symptom ("only `/auth/sessions/me`
is ever called; no `/directory/spaces`, `/directory/events`, or `/directory/ws`") against a fresh page
load. Every live probe below shows the opposite: the shell opens exactly one `/directory/ws` per
identity, replays events, and folds them into Home reactively — including a full two-browser,
no-reload, create-in-one-see-in-the-other round trip.

The most likely explanation for the handoff note's negative observation: it was taken against browser
tabs that were already open before the relevant identity/directory-lane commits landed (`0b9f1d3a04`,
2026-08-17 12:10:50, `backbone-worker.ts` + `component.ts`; `101a6b4ea8`, 2026-08-17 15:59:36,
`ShellHost/component.tsx`). React Fast Refresh does not reliably re-run an already-mounted `useEffect`
whose dependency array is unchanged after a hot patch to its body, so a stale tab can keep running an
old closure indefinitely without an error. Every probe in this report used a **fresh navigation**
(`page.goto`), which is what any real user reload or new tab would also do, and is enough on its own to
pick up the current, working code — see run 2 below.

All three files in the lease's most likely-suspect list (`ShellHost/component.tsx`,
`os/backbone-worker.ts`, `os/component.ts`) were clean (`git status` showed no local diff) at the time
I probed and diagnosed. By the time I wrote this report, `ShellHost/component.tsx` and
`backbone-worker.ts` had picked up unrelated live edits from a concurrent session (window/pane
"open in new window" support, and a defensive `postMessage` origin guard, respectively — checked with
`git diff`, neither touches identity bootstrap, `directory-open`, or `foldDirectoryEvents`). I made no
edits to any file myself. **Changed files: none.**

## Diagnosis method

1. Read `📋️contract-freeze.md` §C1/§C3/§C6 and lane `2-C`'s own report
   (`.../☀️16/PRESERVE-SEEDED-DIALOG-CONTEXT-ARGUMENTS/📓️w2-c-report.md`) to learn the intended
   mechanism: identity bootstrap effect (`ShellHost` ~line 1277) opens the identity document, resolves
   `me()`/`mintSession`, then posts one `directory-open` to the backbone worker
   (`ShellHost/component.tsx:1340-1343`); the worker's `openDirectory` (`backbone-worker.ts:544-553`)
   creates one `DirectoryClient.stream()`, which opens a real `WebSocket` at
   `component.ts:3957-4029` (`🔖️HubBinding`); incoming events post back as `directory-message`
   and are folded via `dispatchDirectoryEventBatch` → the `foldDirectoryEvents` view action
   (`ShellHost/component.tsx:3882-3895`) → `…ConfigMutation::FoldDirectoryEvent`.
2. Checked the one place this could silently no-op even when wired correctly: whether the *compiled
   WASM* `store_worker` (the primary path; `backbone-worker.ts` falls back to a TS re-implementation
   only when that package is unavailable) understands `directory-open` at all. It does not —
   `BackboneWorkerRequest`/`BackboneWorkerResponse` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:229-252` only have
   `Open`/`Close`/`Send` / `Event`/`Ready` variants, no `DirectoryOpen`/`DirectoryCommand`/
   `DirectoryClose` counterparts, and the Rust worker host
   (`🔨️modules/🏪️store/👷️worker/🦀️component.rs:29-66`) has no match arm for them either — **this is a
   real latent bug**, but not the active one: `@semio-tech/store-worker` does not exist anywhere in
   this repo (no `node_modules` package, no vite alias, no built `.wasm`; confirmed by `find`/`grep`
   across the tree), so `ensureRustHost()` (`backbone-worker.ts:34-45`) always throws on the dynamic
   `import()` and always falls back to the TS actor, whose `🔖️Directory` region does implement
   `directory-open` correctly. I flag this as a `sharedFileRequest`-worthy latent bug below — it would
   silently break the directory lane again the day someone wires up the compiled worker — but it is
   not live today.
3. With static analysis unable to find an active break, I ran a real headless Playwright probe against
   the two already-running dev servers (per instructions, not rebuilt) rather than guess further.

## Proof 1 — the shell opens `/directory/ws` and folds events (single browser)

`🧪️dirlane-probe-baseline.txt` / `🧪️dirlane-probe-user1-full.txt` (fresh navigation to
`http://127.0.0.1:6072/`):

```
{"type":"request","url":"http://127.0.0.1:8787/auth/sessions/me"}
{"type":"response","status":200,"url":"http://127.0.0.1:8787/auth/sessions/me"}
{"type":"ws-open","url":"ws://127.0.0.1:8787/directory/ws?token=...&since=0"}
{"type":"ws-recv","url":"ws://127.0.0.1:8787/directory/ws?...","data":"{\"kind\":\"event\",...\"kind\":\"user.created\"...}"}
```

DOM after boot (`🧪️dirlane-user1-home.png`): one `data-row-id="space:default"` row, "Demo Studio /
atelier / private / 1 / 0 / local" — the pre-existing local-only space, correctly rendered by the same
Home table that also renders hub-origin rows (proved next).

## Proof 2 — the hub's *existing* space does **not** appear, and that is correct, not a bug

The coordinator's handoff said "1 space in it" for `/directory/spaces`. Querying the hub directly:

```
$ curl :8787/admin/api/events        # admin, bypasses authz
[... seq1 user.created "seed" ..., seq2 space.created spaceId="default" spaceKind="studio"
     visibility="private" ownerUserId="seed", seq3 member.upserted spaceId="default" userId="seed" ...]
$ curl :8787/directory/spaces -H "Authorization: Bearer <user1 token>"
[]
```

The hub's only pre-existing space (`default`) is **private and owned by `seed`** — neither `user1` nor
`user2` is a member. Per contract §C2 (`GET /directory/spaces -> member spaces + public spaces`) and
the decider laws, this space correctly does **not** appear for either test user; `/directory/events`
also correctly omits its `space.created`/`member.upserted` events from a non-member's replay (visibility
filtering, confirmed: `/directory/events?since=0` for user1 returns only the `user.created` event, while
`/admin/api/events` — which bypasses authz — returns all three). This is the authorization contract
working as designed, not the directory lane failing to relay a space it should show. I did not fake a
row for it and did not weaken the filter.

## Proof 3 — real cross-user propagation, no reload, no fixture

Full script: `🧪️dirlane-e2e-probe.mjs`. Two independent Playwright pages, one per dev server
(`http://127.0.0.1:6072/` = user1, `http://127.0.0.1:6073/` = user2), driven purely through the real UI
(`#s-home-create-space` → "Create Space" button → dialog → `name` input, `visibility` combobox →
"Public", submit). Clean run: `🧪️dirlane-e2e-run2.txt`; screenshots
`🧪️dirlane-e2e-user1-after-run2.png` / `🧪️dirlane-e2e-user2-after-run2.png`.

- Both shells open exactly **one** `/directory/ws` each, receive the seed replay.
- user1 submits `Create Space` → `POST /directory/commands` → **202** → the hub emits
  `space.created` + `member.upserted` (`visibility:"public"`) → **both** sockets receive both events
  live (`t=1786986821620` user1, `t=1786986821612/613` user2 — within ~10ms of each other, no polling).
- Home's table updates in **both** browsers, with `data-row-id="space:<hub-minted-id>"`, `Origin: hub`
  (not `local`) — confirming the row is sourced from the folded directory read model, not a fixture.
- User2 never reloaded. Result: `appears in user1 own table: true`, `appears in user2 table: true`.

```
=== FINAL RESULT ===
spaceName: E2E Proof Space 1786986818002
appears in user1 own table: true
appears in user2 table: true
```

## A real bug found along the way, outside my lease

The **first** attempt at proof 3 (`🧪️dirlane-e2e-run1.txt`) failed — not because of the directory
lane, but because the **hub process itself died** mid-run. Evidence:

- Both user1's and user2's `/directory/ws` sockets closed **simultaneously**
  (`t=1786986516294` for both — a server-side event, not two independent client failures), then every
  reconnect attempt (`since=5`, exponential backoff 0.5s→1s→2s→4s→8s, exactly matching
  `HUB_RECONNECT_MIN_MS`/`MAX_MS`) opened and closed on the same millisecond — connection refused.
- `ps`/`curl` afterward: **no `os-hub` process was listening on 8787 at all.** I had not touched
  `🌎️hub/**` (out of my lease) and issued no destructive commands.
- I restarted it myself from the same persisted `OS_HUB_DATA` (`.🧬semio/🌐hub/hub-dev/`, per
  `.vscode/launch.json`'s `🛠️dev🗄️os-hub` entry), capturing stdout this time
  (`🧪️dirlane-hub-restart.txt`, pid 41378, still running and healthy at report time). After restart,
  `directory.db`'s **mtime had not moved since 12:58**, and only the original 3 seed events survived —
  the space named `"a"` that a prior session had created (`recordedAtMs` ~16:06, well before my
  session) was gone too. Whether that old, long-running process had a persistence bug (writes never
  reaching sqlite) or crashed before flushing, I cannot tell without its stderr, which I never had
  (it was started by an earlier session/terminal, not me).
- Retried proof 3 clean against the **freshly restarted** hub (run 2, above): it passed, and
  `directory.db`'s mtime **did** advance this time, and `headSeq`/`spaces` in `/admin/api/overview`
  correctly reflect the new space. I did not observe a second crash.

This is a `🌎️hub/**` concern (process stability / write-durability of the directory event log under
live dev-server conditions), entirely outside my lease (`🧰️framework/🛍️products/💻️os/**`,
`✏️s/🔌️plugins/🪐️space/**`) and outside my time budget to root-cause without hub stderr from the
original crash. Flagging rather than fixing:

**sharedFileRequest**: `🌎️hub/**` owner — investigate why the long-running `os-hub` dev process died
during this session (no panic captured, since it predated my restart) and why `directory.db`'s mtime
had not moved since process start despite ongoing directory command traffic. Not urgent — a fresh
restart behaves correctly — but worth a look before this is relied on for longer dev sessions.

**sharedFileRequest** (latent, not active): `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
`backbone_worker_wire::BackboneWorkerRequest`/`BackboneWorkerResponse` (lines 229-252) and
`🔨️modules/🏪️store/👷️worker/🦀️component.rs`'s `BackboneWorkerHost::handle_request_bytes` (lines 29-66)
have no `directory-open`/`directory-command`/`directory-close` request variants or
`directory-message`/`directory-status`/`directory-command-result` response variants, unlike the TS
wire types they're supposed to mirror. Currently harmless because `@semio-tech/store-worker` (the
compiled WASM host that would receive these) does not exist in this repo yet, so every dev/test run
falls back to the TS actor twin, which does implement the directory lane. The day that package is
wired up, `directory-open` will silently fail to decode on the Rust side (`decode_request` returns
`Err`, `handle_request_bytes` throws, the throw lands in an un-caught `.then()` — an unhandled
promise rejection, not a crash) and the whole directory lane will regress exactly like the symptom
this ticket describes. Whoever owns `🏪️store/**` should add the three request/response variants and a
proxying implementation (or explicitly document that the Rust worker intentionally excludes the
directory lane and it must stay TS-only).

## What is NOT done / not applicable

- No code change was needed for the stated task, so nothing was fixed in
  `ShellHost/component.tsx`, `backbone-worker.ts`, `os/component.ts`, or the Home surface.
- Did not fix the two `🌎️hub/**` and `🏪️store/**` findings above — both outside my lease.

## Regression checks (both blocked by unrelated, concurrent, in-progress work — not by anything here)

I made no source changes, so there is no regression risk from this lane. Ran the two required checks
anyway to confirm the baseline; both are currently **red**, but the failure is 100% inside
`semio-s-plugin-stdio`, which is explicitly forbidden to me (`✏️s/🔌️plugins/🗄️stdio/**`), not inside
`semio-s-plugin-space` itself:

```
$ cargo test -p semio-s-plugin-space --lib
error: could not compile `semio-s-plugin-stdio` (lib) due to 166 previous errors
  # all: error[E0063]: missing field `corner` in initializer of `WindowLayoutWindowNode`
  #      (✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/🦀️component.rs, ~166 call sites)
```

Root cause confirmed live and in-progress, not caused by me: `WindowLayoutWindowNode`'s definition
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:676-689`) just gained a
new `corner: Option<WindowStackCorner>` field; `git status` shows that exact file as `M ` (modified,
uncommitted) right now — a concurrent session mid-way through adding the field and has not yet updated
every `stdio` artifact's struct literal. Retried once after other work (~1 minute later): still red,
same 166 errors, file still `M `. Per the worker-brief ("other sessions are editing this tree live...
never revert or clean up a foreign change") and my lease (`stdio/**` explicitly forbidden), I did not
touch it. Logs: `🧪️dirlane-cargo-test-space-lib.txt`, `🧪️dirlane-cargo-test-space-lib-retry.txt`.

```
$ cargo check -p semio-s-plugin-space --target wasm32-wasip2
error: could not compile `semio-s-plugin-stdio` (lib) due to 166 previous errors; 2 warnings emitted
```
Same cause. Log: `🧪️dirlane-cargo-check-space-wasm.txt`.

The last clean baseline for both (205 passed/0 failed; 0 wasm errors), captured minutes before this
lane started by the concurrent `space-imports` work in this same ticket folder, is in
`🧪️space-imports-test-final.txt` / `🧪️space-imports-check-wasm.txt` — nothing in this lane invalidates
that baseline; it is simply not re-confirmable right now because of the unrelated `stdio` churn.

## Evidence files (all in this ticket folder)

- `🧪️dirlane-probe1.mjs` / `🧪️dirlane-probe2.mjs` — single-shell boot probes (network + console + DOM).
- `🧪️dirlane-probe-baseline.txt`, `🧪️dirlane-probe-user1-full.txt` — their output.
- `🧪️dirlane-user1-home.png` — Home before any hub-visible space exists.
- `🧪️dirlane-probe3-dialog.mjs`, `🧪️dirlane-probe4-combobox.mjs`, `🧪️dirlane-dialog-snapshot.txt`,
  `🧪️dirlane-dialog.png`, `🧪️dirlane-combo.png` — reconnaissance of the real Create Space dialog's DOM
  (needed to drive it without any hardcoded/fixture shortcuts).
- `🧪️dirlane-e2e-probe.mjs` — the two-browser end-to-end proof script.
- `🧪️dirlane-e2e-run1.txt` — first attempt, hub died mid-run (see above).
- `🧪️dirlane-hub-restart.txt` — my hub restart's captured stdout (pid 41378, still running).
- `🧪️dirlane-e2e-run2.txt` — clean, fully-passing rerun.
- `🧪️dirlane-e2e-user1-after-run2.png`, `🧪️dirlane-e2e-user2-after-run2.png` — final screenshots, both
  browsers showing the same hub-origin row.
- `🧪️dirlane-cargo-test-space-lib.txt`, `🧪️dirlane-cargo-test-space-lib-retry.txt`,
  `🧪️dirlane-cargo-check-space-wasm.txt` — regression-check attempts, both blocked by foreign `stdio`
  churn.

## Changed files

None.
