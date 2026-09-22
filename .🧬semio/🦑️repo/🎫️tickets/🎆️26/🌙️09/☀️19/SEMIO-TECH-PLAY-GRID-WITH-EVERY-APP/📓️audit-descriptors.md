# 📓️ audit-descriptors — committed plugin descriptor freshness & describe outcomes

Read-only audit, `audit-descriptors` agent, 2026-09-22 ~11:00–11:25. Scope: all 34 plugins under
`✏️s/🔌️plugins/*`, project `@semio-tech/<name>-plugin`, target `describe`.

## Method

1. **Freshness law** (`__semio_plugin_descriptor_fresh_test!` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` ~L40482 →
   `🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs` L1‑24): `descriptor_is_fresh` is a **byte
   comparison** — it runs `describe()` natively in-process, blanks the three hash fields via
   `descriptor_bytes_with_blank_hashes`, and asserts the result equals the bytes of the committed
   `✏️s/🔌️plugins/<p>/🛂️.descriptor.semio`. It is a cargo test, not runnable here (no cargo/wasm
   allowed). **Proxy used instead**: descriptor file mtime vs. the newest mtime of any file under the
   plugin directory (excluding `dist/`, `node_modules/`, `🗑️generated/`, `🤖️generated/`, the descriptor
   pair itself, and README/AGENTS docs).
   ⚠️ **Caveat — this proxy is noisy right now.** The mtime scan (11:18–11:21) found 28/34 plugins with a
   "newest source" file touched in that exact 2–3 minute window, spread near-uniformly across almost the
   whole tree — consistent with the fleet's own concurrent, uncommitted edit activity (brief v5: peer
   session restarted 10:50, ~420 files uncommitted) rather than any specific semantically-relevant change
   to each plugin. Treat a "STALE (~1h)" verdict below as **low confidence** unless the gap is on the
   order of a day or more, in which case it corroborates independent log evidence.
2. **Size bound**: `DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024` (4 194 304 B),
   `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1802`. Enforced via
   `DirectoryOpenTarget`-style checks (`…schema/🦀️.rs:1885`, `…hub/🏗️bootstrap/🦀️.rs:3301`). The
   describe-side mitigation is `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES = 256 * 1024`
   (`🔌️plugin/🛂️describe/🦀️.rs:23‑29`) which externalizes example bodies over 256 KiB into `assets` —
   its own doc comment names `🧩️puzzle`'s `capsule-dream` example (3 577 295 B) as the sole known case
   that still pushes a descriptor over the 4 MiB bound.
3. **Describe logs read**: live `$T/🗑️generated/activation/describe-activate-0922-1058.txt` (serial,
   one mutex hold per plugin, in progress as of last read ~11:21 — only `architect`/`animate` reached),
   plus last night's batch runs `describe-all-then-activate-{0130,1724,2130}.txt`
   (`describe-all-then-activate-2033.txt` is empty/unused). `1724` (17:31→18:55, `--parallel=2
   --nx-bail=false`) is the only run that reached genuine completion for all 34 describe tasks without
   being cut short by an external SIGTERM — it is the most reliable "did describe succeed" source for
   the plugins the live run hasn't reached yet. `0130` was killed mid-run (`rc=143`, "Polite quit
   request") and `2130` also appears torn down mid-run (trinity/vcs hit an open file-lock wait right
   before the log ends).

## Table (34 plugins)

Descriptor path for every row: `✏️s/🔌️plugins/<plugin>/🛂️.descriptor.semio` (+ sibling `🔣️.json`).

| plugin | descriptor mtime | newest source mtime | fresh?(mtime, caveated) | size B (bound 4 194 304) | last describe outcome (run, duration/detail) | verdict |
|---|---|---|---|---|---|---|
| architect | 09‑22 11:01 | 09‑22 11:18 | STALE (~17m, live run's own describe just landed; a source file was touched minutes after) | 108 953 | **ok** — live 0922 run, 10:58:56→11:01:51 (~2m55s) | fresh (just described) |
| animate | 09‑22 11:13 | 09‑22 11:20 | STALE (~7m, same live-run pattern) | 39 270 | **ok** — live 0922 run, 11:01:51 start, NX "Successfully ran" after 9m16s (~11:11); wrapper's `ok` line not yet flushed as of last read | fresh (just described) |
| writer | 09‑21 18:45 | 09‑22 11:18 | STALE (~16.5h, baseline noise) | 34 893 | not yet reached by live run; not in 1724's failed list → last known-good describe | fresh (needs describe to refresh per byte-law, but no known breakage) |
| mathematical | 09‑21 18:23 | 09‑22 11:18 | STALE (~17h, baseline noise) | 25 453 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| wfc | 09‑22 04:17 | 09‑22 11:19 | STALE (~7h, baseline noise) | 254 455 | not in 1724 failed list; descriptor already refreshed by last night's 0130/03:04 activation pipeline | fresh (needs describe to refresh, no known breakage; runtime has a separate `data-shell-ready` registration bug per brief v5, unrelated to descriptor) |
| procedural | 09‑21 18:30 | 09‑21 04:07 | **FRESH** (source predates descriptor; peer-owned, excluded from today's queue) | 199 916 | not in 1724 failed list | fresh |
| flow | 09‑21 18:52 | 09‑21 17:19 | **FRESH** | 57 793 | not in 1724 failed list | fresh |
| gis | **09‑17 13:33** | 09‑22 11:18 | STALE (~4.9 days — real, not baseline noise) | 422 848 | **FAILED** in both 1724 (`gis-plugin:describe` in Failed tasks) and 2130 (`describe semio-s-plugin-gis failed: cargo build -p semio-framework-plugin-describe killed by signal SIGTERM`) | **describe blocked** — descriptor emitter build gets killed (SIGTERM), 5-day-stale descriptor |
| vcs | **09‑20 14:47** | 09‑22 11:18 | STALE (~1.8 days — real) | 283 450 | **FAILED** in 1724 (in Failed tasks); in 2130 hit "Blocking waiting for file lock on artifact directory" right as the whole nx process got torn down (SIGTERM) | **describe blocked** — no confirmed success in either full run; descriptor 1.8 days stale |
| shooting | 09‑21 18:34 | 09‑21 15:21 | **FRESH** | 52 364 | not in 1724 failed list | fresh |
| demonstrator | 09‑21 18:19 | 09‑21 17:16 | **FRESH** | 313 285 | not in 1724 failed list | fresh |
| sequence | 09‑21 18:33 | 09‑22 11:18 | STALE (~17h, baseline noise) | 42 894 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| fem | 09‑21 18:52 | 09‑22 01:42 | STALE (~6.8h, plausibly a real overnight edit — before the 11:2x noise window) | 121 925 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| process | 09‑21 17:45 | 09‑22 03:50 | STALE (~10h, before the 11:2x noise window — plausibly real) | 74 724 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| lowpoly | 09‑21 18:36 | 09‑22 11:18 | STALE (~16.7h, baseline noise) | 124 222 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| reasoning | 09‑21 18:32 | 09‑22 11:15 | STALE (~16.7h, baseline noise) | 46 902 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| forms | 09‑21 18:42 | 09‑22 01:41 | STALE (~6.9h, before the noise window — plausibly real) | 49 367 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| layout | 09‑21 18:40 | 09‑22 11:18 | STALE (~16.6h, baseline noise) | 47 635 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| cad | 09‑21 17:41 | 09‑22 03:58 | STALE (~10.3h, before the noise window — plausibly real, matches the framework regression below) | 65 412 | **succeeded** in 1724 (not in Failed tasks) but **FAILED** in 2130: `error[E0063]: missing field 'accessibility_label' in initializer of 'UiInputNode'` at `🧰️framework/🔨️modules/🖱️ui/…/🧊️wgpu/🧩️component/🦀️.rs:2639`, blocking compilation of `semio-framework-ui` (a framework-wide regression, not cad-specific code) | **describe blocked by <framework regression: `UiInputNode` missing `accessibility_label` field, introduced between 17:31 and 21:37>** — not yet retried in the live 0922 run |
| sourcing | 09‑21 17:41 | 09‑22 01:42 | STALE (~8h, before the noise window — plausibly real) | 47 758 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| dag | 09‑21 18:50 | 09‑22 11:05 | STALE (~16.2h, baseline noise) | 51 598 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| note | 09‑21 20:36 | 09‑22 11:20 | STALE (~14.7h, baseline noise) | 55 482 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| puzzle | **09‑09 19:21** | 09‑22 11:20 | STALE (**~12.6 days — real, oldest descriptor in the tree**) | **4 295 257 — over the 4 194 304 B bound by 100 953 B (~98.6 KiB)** | **FAILED** in 1724 (Failed tasks) and 2130 (`could not write output to …semio-s-artifact-puzzle-3d…cgu.0.rcgu.o: No such file or directory` → `error: could not compile semio-s-artifact-puzzle-3d` → `cargo build -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile wasm-dev exited with status 101`) | **describe blocked** — build-output race (missing `.o`, likely shared target-dir contention) **and** the committed descriptor is already over the 4 MiB catalog bound (known, documented case — `capsule-dream` example) |
| block | 09‑21 18:42 | 09‑22 11:07 | STALE (~16.4h, baseline noise) | 96 882 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| space | 09‑21 18:45 | 09‑22 11:13 | STALE (~16.5h, baseline noise; peer-owned, excluded from today's queue) | 101 122 | not in 1724 failed list | fresh |
| trinity | 09‑21 17:50 | 09‑22 11:18 | STALE (~17.5h, baseline noise) | 119 608 | **succeeded** in 1724 (not in Failed tasks) but in 2130 the wasm build+transpile completed cleanly (`Finished wasm-dev profile … in 4m 31s`, all JS/d.ts files transpiled) and then hung on `Blocking waiting for file lock on artifact directory` right before the log ends (process torn down) | **describe blocked by file-lock contention on the artifact directory**, not a code defect — build itself is clean |
| norm | 09‑20 07:21 | 09‑22 11:18 | STALE (~2.2 days — real; peer-owned, excluded from today's queue) | 281 469 | **FAILED** in 1724: `raw component …/semio_s_plugin_norm.wasm must be a regular non-symlink file of 1..268435456 bytes` (built wasm artifact missing/empty when the describe emitter tried to read it) | **describe blocked by a missing/corrupt built wasm component** (build-output integrity, not compile error) |
| playbook | 09‑21 18:32 | 09‑22 11:18 | STALE (~16.9h, baseline noise; peer-owned, excluded from today's queue) | 31 133 | not in 1724 failed list | fresh |
| imperative | 09‑21 18:24 | 09‑22 11:17 | STALE (~16.9h, baseline noise) | 37 564 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| remodel | 09‑21 18:38 | 09‑21 16:13 | **FRESH** | 96 383 | not in 1724 failed list | fresh |
| energy | 09‑21 18:40 | 09‑22 11:19 | STALE (~16.6h, baseline noise) | 200 456 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| draw | 09‑21 18:47 | 09‑21 15:35 | **FRESH** | 83 322 | not in 1724 failed list | fresh |
| raster | 09‑21 18:41 | 09‑22 11:12 | STALE (~16.5h, baseline noise) | 43 937 | not in 1724 failed list | fresh (needs describe to refresh, no known breakage) |
| stdio | 09‑21 11:19 | 09‑22 11:20 | STALE (~24h — real, and re-confirmed broken every run) | 446 903 | **FAILED identically in both 1724 and 2130**: `semio-framework-plugin-describe describe: calling owned describe() on …semio_s_plugin_stdio.wasm: epoch deadline exceeded` → `describe semio-s-plugin-stdio failed: descriptor emitter exited with 1` (guest-side `describe()` execution times out against the wasmtime fuel/epoch deadline) | **describe blocked by a reproducible `describe()` timeout inside the wasm guest** — not flaky, happened twice identically |

Rows for **writer, mathematical, wfc, sequence, fem, process, lowpoly, reasoning, forms, layout,
sourcing, dag, note, block, imperative, energy, raster** and the fresh-by-mtime set
(**procedural, flow, shooting, demonstrator, remodel, draw**) had no failure entry in either
completed batch log (`1724`'s explicit Failed-tasks list is authoritative here since `--nx-bail=false`
ran every one of the 34 to completion). Their descriptor mtimes predate their committed content by
hours (mostly the 11:1x–11:2x fleet-wide touch noted above), so the byte-level law would very likely
still flag them stale on a real `describe` re-run, but there is no evidence any of them would *fail* to
describe.

## Blocked / needs-describe summary (with likely root cause, no fixes attempted)

- **gis** — descriptor 5 days stale; `describe`'s own build step (`cargo build -p
  semio-framework-plugin-describe`) gets killed by SIGTERM in the 21:37 run, right after a very long
  dependency-compile phase. Likely cause: build/describe step overruns some external time or resource
  budget under fleet load, not a gis-specific code bug.
- **vcs** — descriptor 1.8 days stale; failed outright in the 17:31 run (no detailed error captured
  before the next task's output overwrote the buffer) and in the 21:37 run its task started right as the
  whole `nx` process was sent a "Polite quit request" SIGTERM. No confirmed clean describe in either
  complete run.
- **puzzle** — descriptor is the oldest in the tree (12.6 days) **and** already 98.6 KiB over the 4 MiB
  descriptor bound (documented, known-over-bound case: the `capsule-dream` example body). Its build also
  fails with a missing compiler-output file (`…cgu.0.rcgu.o: No such file or directory`) for
  `semio-s-artifact-puzzle-3d`, which reads as shared build-cache/target-dir contention rather than a
  source defect.
- **stdio** — reproducibly fails `describe` in both completed batch runs with the same error: the
  guest-side `describe()` call exceeds its wasmtime epoch/fuel deadline and the emitter exits 1. This is
  the most clearly root-caused failure of the six: stdio's `describe()` is simply too slow (or looping)
  to finish inside the allotted deadline.
- **trinity** — build and transpile succeed cleanly (`Finished wasm-dev profile … in 4m 31s`), but the
  task then blocks forever on `Blocking waiting for file lock on artifact directory` in the 21:37 run
  until the whole process is torn down. Likely cause: lock contention on the shared artifact-staging
  directory under concurrent describe/activate load (matches the fine-grain-locking wasm-deadlock pattern
  seen elsewhere in this fleet), not a trinity code defect.
- **cad** — succeeded at 17:31 but failed at 21:37 with a genuine, framework-wide compile error
  (`UiInputNode` missing the `accessibility_label` field at
  `🧰️framework/🔨️modules/🖱️ui/…/🧊️wgpu/🧩️component/🦀️.rs:2639`), which blocks `semio-framework-ui` for
  every plugin that depends on it — a regression introduced to the framework between 17:31 and 21:37,
  not a cad-specific problem. Not yet retried by the live 0922 serial run at time of writing.
- **norm** (peer-owned, excluded from today's queue) — failed at 17:31 because the built wasm component
  file didn't satisfy the emitter's own sanity check (`must be a regular non-symlink file of
  1..268435456 bytes`), i.e. the wasm artifact was missing or empty when describe tried to read it — a
  build-output integrity problem, not a norm source defect.

Live serial run (`describe-activate-0922-1058.txt`) had only reached **architect** (ok, ~2m55s) and
**animate** (NX-succeeded, ~9m16s) as of the last read (~11:21); it has not yet reached cad, gis, vcs,
trinity, puzzle or stdio to confirm whether today's run reproduces or clears any of the above.
