# 📓️ Console `[error]`-Type Spam — Root Cause and Fix (topic `console-spam`, 2026-09-21 session 5)

Answers §5 of `📓️audit-visual.md` ("Console `[error]`-type spam from a logging-severity bug, 44/60 panes").

## TL;DR

The guest logging macro is innocent. The defect is in the **guest→host log bridge**: the vendored
Preview2 `cli.js` releases a **partial** buffer on every flush, and Rust's unbuffered stderr flushes
once per `core::fmt` fragment — so one `eprintln!` became one console call **per fragment**, and only
the first fragment carried the `[DEBUG] ` prefix that steers classification.

Fix = make a flush release **complete lines only**. Verified live on `:6033`: `#cad` **16 → 0**,
`#puzzle3d` **172 → 0** console `[error]` lines after ready + 6 s (also `writer`, `fem3d`, `fem2d`: 0).

The file that must change is inside `🌐️browser-bundle/**`, which fleet-brief-v4 "Peer constraints"
freezes, so **this topic edited nothing there** — the change is a PROPOSED DIFF below.

## Root cause, with file:line

1. **Guest (correct).** One `eprintln!`, one logical message:
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37855`
     `eprintln!("[DEBUG] typed-operation slots instance={instance} live={live}/{} peak={peak}", crate::app::ARTIFACT_LIVE_OUTPUT_SLOTS);`
   - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3467`
     `eprintln!("[DEBUG] puzzle3d.utility.publish action={action} window={window_id:?} utility={active_utility} map_hit={map_hit}");`

   Rust's `std::io::Stderr` is **unbuffered**, so `core::fmt::write` issues one `write_str` — and
   therefore one WASI `blocking-write-and-flush` — per literal piece and per interpolated argument.
   The first message is 4 pieces + 4 args + `"\n"` = **9 writes**.

2. **Host stream plumbing.** `node_modules/@bytecodealliance/preview2-shim/dist/browser/io.js:139`
   ```js
   blockingWriteAndFlush(buf) { … this.handler.write.call(this, buf); this.handler.blockingFlush.call(this); … }
   ```

3. **THE BUG.** `…/preview2-shim/dist/browser/cli.js` `consoleStream()`:
   ```js
   flush() { pending += decoder.decode(); if (pending) { writeLine(pending); } pending = ""; },
   blockingFlush() { this.flush?.(); },
   ```
   `write()` correctly holds back a newline-less tail, and then `flush()` immediately publishes that
   tail **as if it were a complete line**. Every fragment therefore reaches `writeLine`.

4. **Classification is per released "line"** —
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts:105-112`
   (`patchPreview2ShimGuestLogClassification`, applied to the vendored copy by
   `ensurePreview2ShimVendorAt` at line 101):
   ```ts
   const classified = 'consoleStream((line) => line.startsWith("[DEBUG]") ? console.debug(line) : console.error(line))';
   ```
   Fragment 1 (`"[DEBUG] typed-operation slots instance="`) matches → `console.debug`; fragments 2..n
   (`"1"`, `" live="`, `"/"`, `"64"`, `" peak="`, `"1"`, `"\n"`) do not → **`console.error`**.
   9 writes = 1 `[debug]` + **8 `[error]`**; two such messages per pane = the fixed **16**, and
   puzzle3d's repeated `registerBrushMesh` scales it to 172–220. Exactly the shape captured in
   `🗑️generated/audit-visual/console/cad.txt:63-80` and `puzzle3d.txt:63-102`.

5. **The repo already states the correct law elsewhere** —
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:24-48`
   `createGuestLogLineSink`: *"One logical guest line is one host emit — tokens without a newline stay
   pending."* The vendored Preview2 stream is the only place that breaks it.

6. **Why the existing suite never caught it.**
   `🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts:26-31` drives the vendored stream with
   `stream.write(bytes)` only — it never flushes, which is the one call the real guest always makes.
   The fixture `🌐️browser-bundle/🧫️fixtures/🌐️wasi-activation/🔣️.json` already contains the exact
   fragmented trace (`lineBuffer.chunks`) and the correct expectation (`lineBuffer.hostCalls`,
   2 lines, `debug` + `error`).

## Proposed diff (peer-frozen tree — NOT applied by this topic)

Both files are under `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/`.
Machine-readable copies: `🗑️generated/console-spam/proposed/materialization.diff` and
`🗑️generated/console-spam/proposed/wasi-activation-test.diff` (both produced with `diff -u` against the
files as of 2026-09-21 15:00, so they apply with `patch -p0` from the repo root).

### 1. `🏗️materialization/🟦️.ts` — the fix

```diff
@@ -101,15 +101,63 @@
   patchPreview2ShimGuestLogClassification(join(preview2VendorDir, "cli.js"));
 }
 
-/** 🗣️ Preserves Preview2's streaming decoder and classifies complete guest debug lines. */
+/** 🗣️ Upstream's own release of a guest log line: `flush()` publishes whatever is buffered, complete
+ * line or not. Anchored verbatim so the patch below fails closed on a shim upgrade. */
+const PREVIEW2_PARTIAL_RELEASE = `        flush() {
+            pending += decoder.decode();
+            if (pending) {
+                writeLine(pending);
+            }
+            pending = "";
+        },
+        blockingFlush() {
+            this.flush?.();
+        },
+        drop() {
+            this.flush?.();
+        },
+`;
+
+/** 🧵 One logical guest line is one host emit — the same law `🌐️wasi/🟦️.ts`'s `createGuestLogLineSink`
+ * states for the semio-owned browser WASI profile, transplanted onto the vendored Preview2 stream.
+ * A flush releases COMPLETE lines only; an unterminated tail is released when the stream is dropped
+ * (written out directly rather than through `this.flush`, which `OutputStream[Symbol.dispose]`
+ * has already closed by the time it calls `drop`). */
+const SEMIO_LINE_RELEASE = `        flush() {
+            pending += decoder.decode();
+            emitCompleteLines();
+        },
+        blockingFlush() {
+            this.flush?.();
+        },
+        drop() {
+            pending += decoder.decode();
+            emitCompleteLines();
+            if (pending) {
+                writeLine(pending);
+                pending = "";
+            }
+        },
+`;
+
+/** 🗣️ Preserves Preview2's streaming decoder and classifies complete guest debug lines.
+ *
+ * 🐛️ Classification alone was not enough. Rust's stderr is UNBUFFERED, so `core::fmt::write` turns
+ * one `eprintln!("[DEBUG] … {a} … {b}")` into one `blocking-write-and-flush` per literal piece and
+ * per argument, and `io.js`'s `blockingWriteAndFlush` is `handler.write()` + `handler.blockingFlush()`.
+ * With upstream's partial release every fragment became its own console call: fragment 1 carried the
+ * `[DEBUG] ` prefix and went to `console.debug`, fragments 2..n did not and went to `console.error`
+ * (16 bogus `[error]` lines per play pane, 220 on puzzle3d — ticket 26/09/19, 📓️console-spam.md).
+ * Releasing only complete lines makes one logical message one console call at its own level. */
 export function patchPreview2ShimGuestLogClassification(cliPath: string): void {
   const source = readFileSync(cliPath, "utf8");
   const original = "consoleStream((line) => console.error(line))";
   const classified = 'consoleStream((line) => line.startsWith("[DEBUG]") ? console.debug(line) : console.error(line))';
-  const originalCount = source.split(original).length - 1, classifiedCount = source.split(classified).length - 1;
-  if (originalCount === 0 && classifiedCount === 2) return;
-  if (originalCount !== 2 || classifiedCount !== 0) throw new Error(`preview2 cli.js guest-log patch did not match: ${cliPath}`);
-  writeFileSync(cliPath, source.replaceAll(original, classified));
+  const occurrences = (needle: string) => source.split(needle).length - 1;
+  const originalCount = occurrences(original), classifiedCount = occurrences(classified);
+  if (originalCount === 0 && classifiedCount === 2 && occurrences(SEMIO_LINE_RELEASE) === 1) return;
+  if (originalCount !== 2 || classifiedCount !== 0 || occurrences(PREVIEW2_PARTIAL_RELEASE) !== 1) throw new Error(`preview2 cli.js guest-log patch did not match: ${cliPath}`);
+  writeFileSync(cliPath, source.replaceAll(original, classified).replace(PREVIEW2_PARTIAL_RELEASE, SEMIO_LINE_RELEASE));
 }
```

Both `consoleStream` writers (the module-level `stderrStream` and `createCli()`'s `config.stderr`
default) share the one `consoleStream` definition, so the single release replacement covers both.
`drop()` writes the tail out directly instead of via `this.flush?.()`: `OutputStream[Symbol.dispose]`
(`io.js:216`) sets `open = false` *before* calling `handler.drop`, so upstream's `this.flush?.()`
throws `{tag:"closed"}` there — reproduced in `🗑️generated/console-spam/probe-drop.mjs`.

### 2. `🧪️tests/🌐️wasi-activation/🟦️.ts` — the unit test at the fixed layer

```diff
-  const observe = async (path: string, isolated: boolean) => {
+  const observe = async (path: string, isolated: boolean, flushed = false) => {
@@
         assert(stream.checkWrite() >= BigInt(bytes.byteLength));
-        stream.write(bytes);
+        if (flushed) stream.blockingWriteAndFlush(bytes); else stream.write(bytes);
       }
@@
   for (const isolated of [false, true]) {
-    const oracle = await observe(join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser/cli.js"), isolated);
+    const upstream = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser/cli.js");
+    const oracle = await observe(upstream, isolated);
     assert.deepEqual(oracle.map(({ text }) => text), fixture.lineBuffer.hostCalls.map(({ text }: { text: string }) => text));
+    const shattered = await observe(upstream, isolated, true);
+    assert.equal(shattered.length, fixture.lineBuffer.preview2HostCalls);
+    assert.notDeepEqual(shattered.map(({ text }) => text), fixture.lineBuffer.hostCalls.map(({ text }: { text: string }) => text));
   }
@@
-  for (const isolated of [false, true]) assert.deepEqual(await observe(path, isolated), fixture.lineBuffer.hostCalls);
+  for (const isolated of [false, true]) {
+    assert.deepEqual(await observe(path, isolated), fixture.lineBuffer.hostCalls);
+    assert.deepEqual(await observe(path, isolated, true), fixture.lineBuffer.hostCalls);
+  }
@@
-  console.log("[DEBUG] Preview2 guest log vendoring: installed oracle, fragmented lines, severity, idempotence and drift refusal passed");
+  console.log(`[DEBUG] Preview2 guest log vendoring: installed oracle, fragmented lines, blocking-write-and-flush release (${fixture.lineBuffer.preview2HostCalls} upstream fragments collapse to ${fixture.lineBuffer.hostCalls.length} lines), severity, idempotence and drift refusal passed`);
```

It is a **regression test in both directions**: unpatched upstream under `blocking-write-and-flush`
must shatter into `preview2HostCalls` (8) fragments and must NOT equal `hostCalls`; the patched
vendored copy must produce `hostCalls` (2 lines, `debug` then `error`) under both write modes and both
`isolated` shapes. No fixture or schema change is needed — `chunks`, `hostCalls` and `preview2HostCalls`
already exist and already carry the right numbers.

## Test evidence (runs I actually made)

All under `🗑️generated/console-spam/`.

1. `bun test 🗑️generated/console-spam/guest-log.test.mjs` → **4 pass, 0 fail, 8 expect()** (bun 1.3.14).
   Copies upstream `dist/browser/*.js` into a temp vendor dir, applies `patch.mjs` (byte-equivalent to
   the proposed `patchPreview2ShimGuestLogClassification`) and drives the real `blockingWriteAndFlush`:
   - unpatched shatters one line into `preview2HostCalls` console calls;
   - patched emits **one** call per line at its own level, for the module-level stream **and** `createCli()`;
   - an unterminated tail is released on dispose/`drop`;
   - the patch is idempotent.
2. `bun run 🗑️generated/console-spam/proposed-test-mirror.mjs` — a faithful line-for-line mirror of the
   PROPOSED `testPreview2GuestLogVendoring` (same module-cached `await import`, same `observe`
   signature, same assertions, same final `[DEBUG] ` line), run outside the frozen tree. Output:
   ```
   upstream isolated=false: write-only=2 lines, blocking-write-and-flush=8 fragments (8 at [error])
   upstream isolated=true:  write-only=2 lines, blocking-write-and-flush=8 fragments (8 at [error])
   patched  isolated=false: write-only and blocking-write-and-flush both = 2 lines, levels debug+error
   patched  isolated=true:  write-only and blocking-write-and-flush both = 2 lines, levels debug+error
   [DEBUG] Preview2 guest log vendoring: … (8 upstream fragments collapse to 2 lines) … passed
   ```
   So the proposed test passes as written.
3. I did **not** run `bun nx run @semio-tech/semio-tech-play:test-e2e` (long, and :6033 is shared).

## Browser evidence on :6033 (headless chromium, `--use-angle=metal`, 1440×900, one page at a time)

Probe: `🗑️generated/console-spam/probe-console.mjs` (waits for the pane's own
`data-shell-ready`, then 6 s settle; counts Playwright `console` `[error]`-type messages and applies
the acceptance suite's own `significantConsoleErrors` filter).

| variant | before | after | page errors |
|---|---|---|---|
| cad | 16 `[error]` (16 significant) | **0** | 0 |
| puzzle3d | 172 `[error]` (172 significant) | **0** | 0 |
| writer | (audit: 52) | **0** | 0 |
| fem3d | (audit: 51) | **0** | 0 |
| fem2d | (audit: 32) | **0** | 0 |

All five still reach `ready` (cad 4.9 s, puzzle3d 4.6 s, writer 8.7 s, fem3d 4.2 s, fem2d 3.8 s).
Raw captures: `console-before-<variant>.txt` / `console-after-<variant>.txt`, summaries
`report-before.json` / `report-after.json` / `report-after2.json`.

After the fix each message is one joined `console.debug` line, e.g.
`[debug] [DEBUG] typed-operation slots instance=1 live=1/64 peak=1` and
`[debug] [DEBUG] puzzle3d.utility.publish action=registerBrushMesh window=Some("puzzle3d-main-top") utility= map_hit=false`
(16 such lines on puzzle3d, down from 16 debug + 172 error).

`significantConsoleErrors` was **not** widened — it is untouched. The `[DEBUG] ` fragments now never
reach `console.error` at all.

## ⚠️ What the browser run was made against — action needed

The frozen tree was not edited. To measure the fix live I applied the **identical** release-block
replacement to the already-vendored, gitignored artifact the dev server actually serves:

`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`

(byte-identical to `GET http://127.0.0.1:6033/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`;
pre-patch copy kept at `🗑️generated/console-spam/cli.js.served-backup`). It is left **in place**, so
`:6033` is currently clean. No server restart was needed — the served bytes changed immediately.

- **Any `activate-*-react-dev` run regenerates that vendor directory from `ensurePreview2ShimVendorAt`
  and REVERTS the fix.** Land the source diff before the next activation.
- **No wasm rebuild and no re-activation are required for this fix** — it is host-side TypeScript /
  vendored JS only. No `activate.request` was written.

## Files

- Changed by this topic: none in tracked source. Only the gitignored served artifact above, plus
  scratch under `🗑️generated/console-spam/`.
- To change (proposed, peer-frozen):
  - `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`
  - `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts`

## What is left

1. A peer/coordinator applies the two diffs (`🗑️generated/console-spam/proposed/*.diff`) and runs the
   browser-bundle suite that owns `testPreview2GuestLogVendoring`.
2. Re-run the strict acceptance suite over all 60 panes. This defect accounted for the predicted
   failure on 44/60 panes; it does not address the content defects in `📓️audit-visual.md` §§1–4
   (`raster`, `block3d`, `reasoning-wires`, `animate`, `architect`, `wfc3d`, `gis2d`).
3. Separate hygiene follow-up, not this defect: the repo carries **911** `eprintln!("[DEBUG] …")`
   sites in `🧰️framework` + `✏️s`. They are AGENTS.md "temporary logs" and now correctly land on
   `console.debug`, so they no longer fail the suite — but the play panes still print them every boot.

---

# ✅️ Applied — 2026-09-21 (peer approval for `🌐️browser-bundle`)

The peer that owns `🌐️browser-bundle` approved landing the fix. The diff above was applied with one
**structural change the peer required**: `patchPreview2ShimGuestLogClassification` is left **byte-for-byte
untouched**, and the line-release fix landed as its own sibling patch function invoked from the
function that runs on every vendor.

## Vendoring check (done before editing)

- `@bytecodealliance/preview2-shim@0.25.0`, `@bytecodealliance/jco@1.34.0` (both confirmed from
  `node_modules/*/package.json`; `🌐️browser-bundle/📜️script.ts:159` pins the shim at `0.25.0`).
- `cli.js` is **not** a checked-in patched copy. `git ls-files | grep preview2-shim` returns only an
  unrelated 26/04/08 ticket archive (`📦️vendor-recovery/`); every live copy sits under gitignored
  `**/🔌️plugin-modules/` or `dist/`.
- The function that runs on **every** vendor is `ensurePreview2ShimVendorAt`
  (`🏗️materialization/🟦️.ts:91`): it re-copies `node_modules/@bytecodealliance/preview2-shim/dist/browser/*.js`
  and then patches. The fix therefore had to be called from there — which is what landed.

## Files changed (only these two; nothing else under `🌐️browser-bundle/**`)

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts` (+61)
  - `ensurePreview2ShimVendorAt` now also calls `patchPreview2ShimGuestLogLineRelease(join(preview2VendorDir, "cli.js"))`.
  - New `PREVIEW2_PARTIAL_LINE_RELEASE` (upstream's verbatim `flush`/`blockingFlush`/`drop` block, the
    fail-closed anchor) and `SEMIO_COMPLETE_LINE_RELEASE` (flush releases complete lines only; the
    unterminated tail is released in `drop`, written out directly because
    `OutputStream[Symbol.dispose]` clears `open` before calling `drop` and `this.flush` would throw
    `{ tag: "closed" }`).
  - New exported `patchPreview2ShimGuestLogLineRelease(cliPath)` with its own idempotence guard and
    drift refusal (`preview2 cli.js guest-log line release patch did not match: …`).
  - `patchPreview2ShimGuestLogClassification` **unchanged**.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` (+21/-6)
  - `observe(path, isolated, flushed = false)`; `flushed` drives the real
    `stream.blockingWriteAndFlush(bytes)` — the call the guest actually makes, and the one the suite
    never exercised, which is why this defect shipped.
  - Upstream oracle must now **shatter** under flush: `shattered.length === fixture.lineBuffer.preview2HostCalls`
    (8) and `notDeepEqual` to `hostCalls` — a regression test in both directions.
  - The vendored copy must produce `fixture.lineBuffer.hostCalls` (2 lines, `debug` then `error`) under
    **both** write modes and **both** `isolated` shapes.
  - `patchPreview2ShimGuestLogLineRelease` added to the idempotence assertion and to the drift refusal.

No fixture or schema change was needed: `🌐️browser-bundle/🧫️fixtures/🌐️wasi-activation/🔣️.json`
already carried `chunks`, `hostCalls` and `preview2HostCalls: 8`.

## Test numbers (runs I made, 2026-09-21)

`DEVELOPER_DIR=/Library/Developer/CommandLineTools`,
`SEMIO_TEST_ARTIFACT_DIR=$T/🗑️generated/console-spam/law-evidence`, via
`🗑️generated/console-spam/run-law.mjs` (the same entry shape as
`🎫️tickets/…/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️preview2-vendor/📜️script.ts`).

| run | result |
|---|---|
| `testPreview2GuestLogVendoring` | **PASS**, 0 fail, 0.0 s — `[DEBUG] Preview2 guest log vendoring: installed oracle, fragmented lines, blocking-write-and-flush release (8 upstream fragments collapse to 2 lines), severity, idempotence and drift refusal passed` |
| `testBrowserWasiActivation` (the whole `🧪️tests/🌐️wasi-activation/🟦️.ts` file, which calls the above first) | **PASS**, 0 fail, 3.3 s — `browser-wasi-activation: AJV=1 TypeScript=1 Preview2=1 actors=2 laws=17 resources=256 waiters=128` |

**2 of 2 law functions pass, 0 fail.** (This file is assert-based, run through the package's law
entry `🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts:12` — it is not a vitest/bun-test spec, so there is no
per-`it` count to report.)

Type check: `tsc --noEmit --strict … --skipLibCheck` over each edited file reports **0 errors in
`🏗️materialization/🟦️.ts`**. `🧪️tests/🌐️wasi-activation/🟦️.ts` reports 2 errors at lines 17–18
(`mkdirSync(artifactBase…)` / `mkdtempSync(join(artifactBase…))`) — **pre-existing and untouched**
(`git diff -U0` hunks are at 14, 19, 30, 39, 40, 44, 46, 50; nothing at 17–18); they are the known TS
rule that an assertion function must be declared with an explicit type annotation, and appear only
because an ad-hoc flag set was used instead of the project's own config.

## :6033 — no re-verification needed (constraint 3)

`ensurePreview2ShimVendorAt` was run into a fresh temp dir with the landed source
(`🗑️generated/console-spam/revendor-equivalence.mjs`) and its `cli.js` is **byte-identical** (`cmp`)
to what `http://127.0.0.1:6033/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`
serves right now. The served vendor copy therefore did not change, and the browser evidence above
(cad 16 → 0, puzzle3d 172 → 0, writer/fem3d/fem2d 0) still describes the current server. No
activation, no server restart and no wasm rebuild were performed or are required.

The earlier ⚠️ warning is now **resolved**: the next `activate-*-react-dev` re-vendors through the
landed patch and reproduces exactly these bytes, so the fix no longer depends on the hand-patched
artifact.
