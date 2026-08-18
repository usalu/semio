# 1-B worker transport — report

## Premise correction (read this first)

The brief describes the target as `pluginWorkerSource` / `PluginWorkerClient` (kernel
`🟦️component.ts`) and a "serialized-run catch path" (`runSerialized`). None of these exist in the
tree anymore: a **concurrent, unrelated ticket** (`MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, H2 —
see its `📓️status.md`, live-modified in git status at session start) already replaced the
one-Worker-per-plugin transport with a bounded shard pool:

- `pluginWorkerSource` → **`shardWorkerSource`** in `🌐plugin-web-materialize.ts` (same file, still
  in my lease — edited).
- `PluginWorkerClient` in kernel `🟦️component.ts` → **deleted outright** (see
  `🟦️component.ts:1413-1422`, the H2 packet's own deletion note) and replaced by **`ShardClient`**
  in `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` — a file in a
  different module, **not covered by my lease** (`kernel/🟦️component.ts` **`PluginWorkerClient`
  region only** — that region no longer exists to edit).
- The old `runSerialized` busy-retry/reload loop is deleted too (`pluginComponentBridgeSource`'s own
  docstring says so); there is no separate "serialized-run catch path" left — the one catch site is
  `shardWorkerSource`'s `self.addEventListener("message", …)` handler, which I edited.

I applied the letter of the ticket's *intent* (real worker stack survives the postMessage hop, is
graftable, classified, and logged) against these current names. The worker-side half is done and in
my lease. The client-side half needs a file outside my lease — see `sharedFileRequest` below; I did
**not** edit it, per worker-brief rule 2.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
  (in lease, full file) — `shardWorkerSource()`:
  - `Error.stackTraceLimit = 200;` at the very top of the generated worker script (before anything
    else runs), so a deep guest-recursion stack survives instead of being truncated to V8's default
    10 frames.
  - `replyError(requestId, error, frames)` widened to also postMessage `stack` (from `error.stack`,
    read directly — never reconstructed from a stringified form), `type` (from
    `error?.constructor?.name ?? typeof error`), and `framesBytes` (byte length of `frames` when it's
    a `Uint8Array`/`ArrayBuffer`, else `JSON.stringify(frames).length`, the whole computation guarded
    in `try/catch` so the error reporter itself can never throw). The pre-existing `error` string field
    and its `payload=` detail suffix are unchanged — purely additive, so an **older worker bundle**
    (missing these three fields) still round-trips fine against a newer client; a **newer worker**
    against an older client is likewise safe since the client only reads `message.error` today.
  - The sole catch site (`self.addEventListener("message", …)`'s `catch (error) { replyError(requestId,
    error, msg.events); }`) now passes `msg.events` — the `turn` request's bulk event-array payload,
    the closest current analog to the brief's "frames" (there is no field literally named `frames` in
    the current wire shape; `events` is the large, recursion-prone payload for the one case
    (`case "turn"`) most likely to blow the stack).

## Commands run + result counts (real tail)

`bun nx run @semio-tech/framework-renderer-react:test` at the default `fundamental` (15s) budget hit
the harness's own wall-clock kill switch before finishing — not a test failure, the runner's `[budget]`
guard. Re-ran at `SEMIO_TEST_LEVEL=long` (300s budget); log saved to
`$T/🧪️1-b-framework-renderer-react-test.txt`:

```
 Test Files  1 failed (1)
      Tests  11 failed | 325 passed (336)
     Errors  1 error
   Start at  13:45:19
   Duration  15.18s
```

**This is not a new failure I introduced.** The brief's stated baseline was 322 passed / 9 failed;
current tree is 325 passed / 11 failed / 336 total — a different total test count, meaning the suite
itself has drifted under concurrent Wave-2 edits since that baseline was captured (per
`📌️important.md`'s "Known live-tree hazards" — the tree is live-edited by other sessions).
Verification that none of it traces to my change:

- `grep -rl "plugin-web-materialize"` across the repo shows `🧪️index.test.ts` (the only failing test
  file) references it **once, in a comment** (`🧪️index.test.ts:1184`, describing
  `inFlightTurnActors`) — never imports or executes `shardWorkerSource()`/`replyError`. My edit is
  template-string content for a `Worker`, never evaluated by this test target.
- All 11 failure titles are about window action panels, `resolveWindowActions`, command-category
  registries, `ShellFaultBoundary`, shell option locks, and virtual-file-system scenes — none
  plugin/worker/transport-related (full titles in the saved log).
- `git status --short` shows `🌐plugin-web-materialize.ts` as my only modification to any file this
  test target touches; `git log --date=iso -1 -- 🧪️index.test.ts` → `2026-08-18 10:22:00 …
  🚩️533`, i.e. the test file itself was auto-committed by a peer session after the brief's baseline
  was taken — consistent with Wave-2 shell work already landing, not with anything in my lease.
- A standalone syntax check of the generated worker source (`new Function(...)` over
  `shardWorkerSource()`'s output) parses clean; both `Error.stackTraceLimit = 200;` and `framesBytes`
  are present in the emitted string as expected.

No unit-test harness exists for `🌐plugin-web-materialize.ts` (it is a codegen/dev-runner script, not
covered by any `🧪️*.test.ts` — confirmed by directory listing, no test file present anywhere under
`🔌️plugin/📦️packages/🟦️typescript/`). Per the verify instructions, reporting this rather than
inventing a fixture.

## sharedFileRequest

**File:** `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` (not in my
lease; owned by nobody in this ticket's Wave-1 table either — it postdates the contract, created by
the concurrent H2 packet).

**Region:** `//#region 📨️WireMessages` (`InboundMessage`, ~line 93-97), `PendingEntry` (~line 121),
`ShardClient.handleMessage` (~line 198-214), `ShardClient.send` (~line 291-301).

**Exact change needed** (this is the client half of the stack-preservation fix — the worker now sends
it, nothing reads it yet):

1. `InboundMessage`'s error variant (line 95) gains three optional fields:
   ```ts
   | { readonly kind: "result"; readonly requestId: string; readonly ok: false; readonly error: string; readonly stack?: string; readonly type?: string; readonly framesBytes?: number }
   ```
2. `PendingEntry` (line 121) gains `readonly actorId: string`, populated at its one construction site
   inside `send()` (line 298) — every `OutboundMessage` variant carrying a `requestId` also carries an
   `actorId`, so this is a mechanical narrow, e.g. `actorId: "actorId" in message ? message.actorId :
   undefined`.
3. `handleMessage` (line 213), replace `else entry.reject(new Error(message.error));` with:
   ```ts
   else {
     const err = new Error(message.error);
     if (message.stack) err.stack = `${message.stack}\n    ↳ main: ${err.stack}`;
     console.log(`[DEBUG] program worker ${entry.actorId} error type=${message.type ?? "unknown"} framesBytes=${message.framesBytes ?? "n/a"}`);
     entry.reject(err);
   }
   ```
   Note: the brief's exact log line names `<pluginId>`, but `ShardClient` only knows `actorId` —
   `pluginId` is a `ActivationRegistry`-level concept (kernel `🟦️component.ts`, one layer up, which
   maps `actorId → pluginId`). Log `actorId` here; if the coordinator wants `pluginId` specifically,
   that mapping needs to happen at the `ActivationRegistry`/kernel caller of `ShardClient`, not inside
   `ShardClient` itself.
4. Degrades gracefully already by construction: `message.stack`/`message.type`/`message.framesBytes`
   are all optional-chained/defaulted, so a worker running an older bundle (pre-1-B, no `stack` field)
   still rejects with a plain `Error(message.error)` and no `[DEBUG]` log — exactly the old behavior,
   no crash.

**Why I didn't just make this edit anyway:** the worker-brief (rule 2) is explicit — "if you need a
file or region that is not yours, STOP editing it, write a sharedFileRequest… and continue with the
rest of your lane." `shard-client.ts` is real, live, has its own substantial inline test suite
(`describe("ShardClient …")`, ~15 cases from line 492 on) that a Wave-1 audit will run against whatever
lane actually owns it, and no lane in the ownership table claims it — I'd be creating exactly the kind
of unowned-file collision risk the brief's leasing model exists to prevent.

## What is NOT done

- The client-side stack graft + `[DEBUG] program worker <pluginId> error type=… framesBytes=…` log
  line described in the brief is **not applied anywhere** — the worker now emits the data, but no
  consumer reads it yet. See `sharedFileRequest` above for the exact, ready-to-apply diff.
  Consequently the STEP 2 e2e (`bun ./📜️script.ts verify collab`) will still show the collapsed
  `RangeError … at worker.onmessage` single-frame stack until that client-side half lands — this lane
  alone does not fix the diagnosability gap end-to-end, only the worker→client transport half of it.
- Because `PluginWorkerClient` no longer exists, my second lease target
  (`kernel/🟦️component.ts` **`PluginWorkerClient` region only**) had nothing to edit; I made no
  changes to that file.
- No unit test added for the worker protocol change (no test harness exists for
  `🌐plugin-web-materialize.ts`; confirmed above rather than assumed).

## Logs

- `$T/🧪️1-b-framework-renderer-react-test.txt` — full `SEMIO_TEST_LEVEL=long` test run (325
  passed / 11 failed / 336 total; all 11 failures pre-existing/unrelated, see above).
