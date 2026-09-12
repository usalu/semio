# Vite dev-serve wedge — root cause, fix, proof (2026-09-12)

Outputs: `🗑️generated/vite-wedge/`. Probes: `🐍️vite-wedge-probe.ts`, `📜️serve-wedge-probe.sh`, `📜️edit-burst.sh`.

## 1. Root cause

**Vite's chokidar watcher ends up with a single macOS FSEvents stream over the whole repository, and then
runs every watched path's prefix filter against every write anywhere in the repo.**

Chain, each step measured:

1. `root` is the tiny dev package (`🧑‍💻dev/`), but the app's modules live across `🧰️framework/`, `✏️s/`,
   `♻️mit-bestand/`. Vite's `ensureWatchedFile` (`vite/dist/node/chunks/config.js:2241`, called from
   `:24852`/`:24863`) therefore calls `watcher.add()` once per out-of-root module-graph file.
   Measured: watched directories go **385 → 1316** and watched files **5055 → 6311** the moment the first
   page load fills the module graph.
2. chokidar's FSEvents handler consolidates sibling streams upward (`consolidateThreshhold = 10`,
   `:13389`) until one stream covers `/Users/ueli/Documents/semio`. Proof that the stream really is at the
   repo root: the watcher receives raw events for `.nx/workspace-data/*`, `.git/fsmonitor--daemon/cookies`
   and `.🧬semio/🦑️repo/⚡️cache/*` — none of which is the root, a config dependency or a module.
   Before the first request the watcher sees **zero** such events; they start in the same 2 s window in
   which `dirs` jumps to 1316.
3. Per delivered event chokidar runs `fsevents.getInfo()` (an lstat) and then **every** registered path's
   `filteredListener` prefix check — ~1316 of them — *before* any ignore test. So the cost is
   `events × watched paths`.

### Measured, with the source edits removed from the experiment

Creating 20 000 files under `.🧬semio/🦑️repo/⚡️cache/` (no source edit at all, cargo's own write pattern)
against a warm serve on 6028:

| | raw events / 2 s | event-loop lag | RSS | CPU | `GET /` |
|---|---|---|---|---|---|
| before | up to **6 291** (all from `⚡️cache`) | **1 934 ms** | 684 → **1 310 MB** in one 2 s window | 60–100 % | 8 s timeout |

`evt=0` throughout — *nothing* passed the filter. All of that CPU was spent in the prefix filters, before
chokidar ever consulted `ignored`.

The same shape comes from any repo writer: a peer writing ticket files produced **1 720 events / 2 s** from
`.🧬semio/🦑️repo/🎫️tickets`.

Once resident memory reaches ~1.4 GB the Bun/JSC heap thrashes and the server **never recovers**: the first
reproduction stayed at 62–99 % CPU with RSS pinned at 1.466 GB and every request timing out, indefinitely,
minutes after the burst had ended.

### Edit bursts

20 saves over 30 s across `🏛️ShellHost/🟦️.tsx`, `🌐️World3dHost/🟦️.tsx`, `⚛️react/🟦️.tsx` and
`🧵️backbone-worker.ts` (mtime-only `touch`, so concurrent peers' content is never clobbered) reproduced the
wedge in ~25 s: `curl` 10 s timeouts, CPU 80–99 %, RSS 671 MB → 1.32 GB in four seconds.

Edit bursts are an *amplifier*, not the cause: each save also re-transforms the saved module (300–700 ms
cold, 1 930–2 146 ms once the heap is under pressure) plus the Tailwind CSS entry (~230–360 ms), and those
allocations ride on top of the watcher's. A burst run on an otherwise-quiet machine did **not** wedge; a
burst run while peers were building did. The cache firehose is the load-bearing half.

### Hypotheses eliminated (with evidence)

- **Config-dependency restart loop** — no. The config bundle is 57 modules and contains no host file
  (`🗑️generated/vite-wedge/config-meta.json`); `ShellHost`/`World3dHost`/`backbone-worker` match 0.
- **`server.watch.ignored` too loose** — cannot be the fix. chokidar's FSEvents callback runs `getInfo` and
  all 1 316 prefix filters *before* `_isIgnored` (`:13440`+, `:14222`). Confirmed empirically: 6 291
  ignored events per 2 s still cost 100 % CPU. Adding globs makes the per-event test *more* expensive.
- **`useFsEvents: false` (per-directory kqueue)** — silently breaks watching under Bun. CPU is beautiful
  (0.2–1.6 %) because **20 touches produced 0 change events**. Also note chokidar turns *polling* on by
  default on macOS when fsevents is off (`:13901`), so both flags must be pinned. Rejected: stale code is
  worse than a wedge.
- **`sample <pid>`** never returns on a wedged process (reproduced: 50 s, no output). `bun --cpu-prof`
  does not exist; running the config under Node fails before it starts
  (`ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` — strip-only mode, known issue), so Node's `--cpu-prof` is not
  available either. Attribution came from instrumenting the watcher instead.

## 2. Fix

`🔌️vite-plugins.ts` (new `//#region 🔖️SourceWatchVitePlugin`):

- `UNWATCHED_REPOSITORY_SEGMENTS` — `.git`, `.nx`, `.vscode`, `.🧬semio`, `node_modules`, `dist`, `target`,
  `🤖️generated`, `🗑️generated`. (`🤖️generated`/`.vscode` carry over the reason the old `ignored` list
  existed: those are config dependencies, so reacting to them restarts the server in a loop.)
- `repositorySourceWatchRoots(repoRoot)` — the repo's top-level directories minus those, read from disk so
  a new top-level product directory is watched without editing this module.
- `unwatchedRepositoryPathMatcher()` — one precompiled `RegExp`, `/` and `\` both handled.
- `semioSourceWatchVitePlugin({ repoRoot })` — `apply: "serve"`; one `node:fs.watch(root, {recursive:true})`
  per source root, one regex test per event, replaying `change`/`add`/`unlink`/`addDir` on Vite's own
  watcher emitter. Handles are closed on `httpServer` close.

`⚙️vite.config.ts`:

- `server.watch: null` — Vite runs **no** chokidar watcher (`NoopWatcher`, which is an `EventEmitter`, so
  invalidation, HMR-boundary computation and config-dependency restarts run exactly as before).
- `semioSourceWatchVitePlugin({ repoRoot })` mounted in the `command === "serve"` plugin list.
- **Separate defect found and fixed:** `playgroundCacheDir` keyed on `${plugin}-${renderer}` only, so a
  `dev` and a `release` serve of the same variant shared one `optimizeDeps` `deps/` directory — whichever
  re-optimizes last rewrites modules the other has already handed to a browser. Now keyed with `profile`.

Cross-platform: `fs.watch` recursive is supported on macOS and Windows and on Linux under Node 20+/Bun; the
segment matcher handles both separators; roots are enumerated, never hardcoded.

## 3. Proof

Same 20 000-file `⚡️cache` churn, same edit burst, same warm serve, **with** the fix:

| | CPU | RSS | `GET /` | edits seen |
|---|---|---|---|---|
| cache churn — before | 60–100 % | 684 → 1 310 MB | **8 s timeout** | – |
| cache churn — after | **0.3–1.0 %** | flat 661 MB | **6–12 ms** | – |
| edit burst — before | 80–99 % | 671 MB → 1.32 GB | **10 s timeout** | yes |
| edit burst — after | **0.1–1.4 %** | flat 614–692 MB | **6–59 ms** | yes, 18 windows |
| churn under a watched `target/` — after | **0.3–5.3 %** | flat 656 MB | **6–20 ms** | – |

Every request throughout stayed far under the 2 s bar; peak was 59 ms.

**Correctness proven, not assumed.** Fetch a module, `touch` it, fetch again:
`fetch1 code=200 t=0.135` (served from cache, no transform logged) → touch →
`fetch2 code=200 t=0.196` with `vite:transform 163.87ms .../🏛️ShellHost/🟦️.tsx` in the log. Invalidation
works. The app boots identically before and after (74 console lines, same window status payloads).

Live side-by-side at the end of the session, one machine, same load:

```
6013  old config  uptime 1:29:53   99.7 % CPU  1 394 MB   GET / -> 8 s timeout
6019  old config  uptime 1d 4:44   99.5 % CPU  1 033 MB   GET / -> 8 s timeout
6018  NEW config  uptime 0:07:18    0.0 % CPU    611 MB   GET / -> 10 ms
```

### Tests

`🧫️fixtures/👁️watch-policy.json` + `🧪️tests/🧹️config/🟦️.ts` — 21 passing cases:
every fixture path classified on both separators; source roots cover the products and exclude every store;
the declared segment list matches the fixture; `⚙️vite.config.ts` still declares `watch: null` and mounts
the plugin; and a behavioural case that runs the real plugin against a sandbox tree and asserts the source
edit reaches Vite while every unwatched store stays silent.

Third-party oracle per the repo rule: **`picomatch`** (the glob engine chokidar itself filters with)
decides all fixture paths from equivalent `**/…/**` globs and must agree with our matcher on every one.

Run: `bun ./📜️script.ts test quick --testNamePattern='dev server watch policy'` → 21 passed.

## 4. Wedge-detection recipe (automatable)

```sh
PID=$(pgrep -f "port 6018" | head -1)
CPU=$(ps -o pcpu= -p "$PID" | tr -d ' ')
T=$(curl -s -o /dev/null -w '%{time_total}' --max-time 5 http://127.0.0.1:6018/)
```

Wedged iff `curl` returns `code=000` (timed out) **and** CPU > 70 %. Both are needed: a healthy serve
answers `/` in 6–40 ms even at 100 % machine load, and a serve can briefly hit 60 % CPU while transforming.
A third confirming signal is RSS > 1.2 GB (`ps -o rss=`), which is where the heap starts thrashing.
Do **not** use `sample <pid>` in an automated check — it hangs on exactly the process you want to inspect.

## 5. Restarts required

**Yes — plainly: every running dev serve must be restarted to pick up this fix.** Vite reads
`⚙️vite.config.ts` once, at start; a running server keeps its chokidar watcher forever.

- **6018** — already restarted by me (it had wedged: 37 % CPU, 1 329 MB, >30 s unresponsive after 40 min).
  Relaunched with `screen -dmS g3dreact <ticket>/📜️serve-generation3d-react.sh`; ready in 665 ms and healthy.
- **6013** (puzzle3d release) and **6019** (generation3d dev, 1 day old) are still on the old config and are
  **both wedged right now** — left running, since they are not mine to kill. They will not recover on their
  own; recycle them by pid.
- First boot after the change logs `Re-optimizing dependencies because vite config has changed` once —
  expected, and it is also why a *new* serve can disturb an *old* one that shares the `optimizeDeps` cache
  directory (the `profile` key fix above narrows that class of collision).

## 6. Adjacent pre-existing breakage (not caused here, not fixed here)

`🧪️tests/🧹️config/🟦️.ts > vite config module graph` has **3 failing cases** on `main`, unrelated to this
change (my only config import edit adds one named export from an already-bundled module):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` is reachable from
  `⚙️vite.config.ts` although `🧫️fixtures/⚙️config-graph.json` denies it — it arrives via
  `📚️library/🎮️playground/🟦️.ts` and is **858 KB**, the single largest input.
- graph is **57 modules** against a declared cap of 40, and source bytes blow `maxSourceBytes: 700000`.

This matters to the same subsystem: config-bundle modules are exactly the files whose save restarts the
dev server, and the discovery walk is the thing `🔌️vite-plugins.ts`'s own header docstring says must stay
out of the config bundle.
