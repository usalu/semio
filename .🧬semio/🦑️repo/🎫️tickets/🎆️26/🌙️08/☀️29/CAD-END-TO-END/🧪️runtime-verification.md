# 🧪️ CAD Runtime Verification

Goal: get real runtime proof (browser + console) that CAD works end to end. **This was not achieved this session** — the dev server never bound its port. What follows is (a) the environmental blocker with hard evidence, and (b) everything that *could* be verified statically in its absence, reported honestly per-check.

## 0. What happened, with timestamps

- `08:46:31Z` — started `cad-react` (`bun ./📜️script.ts dev cad`, port 6020) via `preview_start` (the react renderer variant, per the brief).
- Log immediately showed `program build scope: cad, stdio` then `Blocking waiting for file lock on build directory`.
- Polled `curl http://localhost:6020` continuously for **~28 minutes** (`08:46:31Z` → `09:14:13Z`, when I stopped the server). **Every single poll returned connection-refused (`000`)** — the vite/dev HTTP server never came up, so the Browser pane never had anything to load. No screenshot was taken because there was never a page to screenshot.
- At `~09:12Z` (≈20 min into that one build step) the dev tool's own budget guard killed the CAD wasm build and printed, verbatim:
  ```
  [budget] cargo rustc -p semio-s-plugin-cad --target wasm32-wasip2 --profile dev -- -C link-arg=-zstack-size=8388608 exceeded 1200000ms — killed. Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying.
  plugin build failed, continuing with remaining targets: cad
  error: spawnSync cargo ETIMEDOUT ...
  ```
  It then moved on to the `stdio` target and immediately hit the **same** `Blocking waiting for file lock on build directory` message again.

### Corroborating evidence this is environmental, not a CAD defect

- `uptime` throughout the wait: load averages climbing from **27 → 55** (this machine is heavily oversubscribed).
- `lsof +D target/debug` consistently showed **10–12 concurrent `cargo`/`rustc` processes** (from several other sessions' scratchpad target dirs plus the shared root `target/debug`) all holding the shared build-directory lock at once.
- My own CAD build's processes (`cargo check -p semio-s-plugin-cad`, `cargo rustc -p semio-s-plugin-cad`) sat at a sustained **0.0% CPU, state `S`** for their entire 18–20 minute lifetime — genuinely idle/blocked, not slowly compiling.
- Two other sessions working this exact ticket independently hit and documented the identical blocker before me: `📓️status.md` ("`semio-s-plugin-cad` depends on `semio-s-plugin-stdio`, which another session is actively refactoring (~1000 modified paths, 398 compile errors)... sits on the build-directory lock") and `📓️rust-and-oracle-verification.md` ("CPU time stopped advancing... consistent with the process blocking on the shared cargo target-dir lock").
- **Zero errors originate in CAD's own Rust sources** — this is purely lock contention from a concurrent session's in-flight refactor, exactly as the ticket brief anticipated.

### Why I didn't route around it

- The task's hard rule is to launch the dev server only via the Browser pane's `preview_start` against a `.claude/launch.json` entry — never plain Bash. That schema (`runtimeExecutable`/`runtimeArgs`/`port`) has **no env-override field**, so there was no way to inject an isolated `CARGO_TARGET_DIR` (the mitigation other concurrent sessions are visibly using in their own scratchpads) without either running the dev command directly via Bash (prohibited) or hand-editing the shared `.claude/launch.json` (already mid-edit by someone else per git status, and its schema doesn't support env vars anyway). Per the brief's own instruction to attribute rather than chase concurrent-session issues, I did not attempt either.

**Verdict for check 1 (dev server starts, CAD editor surface loads): FAILED — blocked, not broken.** The dev server process never bound port 6020 in ~28 minutes of waiting; the CAD plugin's own wasm build was killed by the tool's 20-minute budget due to shared cargo lock contention from other concurrent sessions on this machine. No screenshot exists because nothing ever rendered.

## 1–5. Runtime checks — not directly observable; static fallback evidence only

Since the app never booted, checks 2–5 (model-definition discovery, typology/action/interaction counts, the `primitive.box` interaction, the `spatial.shape.from_geometry` transformation) **could not be exercised at runtime** as instructed. I did not substitute static analysis for the honest verdict above, but I did re-confirm the two underlying fixes and the disk-level asset inventory the runtime would consume, so the next attempt has a clean starting point:

- **Glob-path fix (5-level→9-level) is in place**: `.../✏️editor/⚙️engine/🏃️runtime/🟦️component.ts` still has 9× `../` in every `import.meta.glob(...)` call, correctly reaching from `.../✳️any/✏️editor/⚙️engine/🏃️runtime/` back up to the plugin's `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/` root. Verified by direct file read, not by inference.
- **`mutation` discriminator fix is in place**: `.../⚙️engine/🎬️actions/🟦️component.ts` dispatches `applyTransition`/`applyEffectAsync` on `a.mutation === "assign" | "clear" | "append" | "kernel.query" | "interaction.call" | "action"` — the old `.operation` discriminator is gone from this dispatch path.
- **Disk inventory** (the exact 12 glob targets, counted directly under `.../📚️examples/🖼️assets/🏗️modelDefinitions/`):
  | Asset | Count | Expected (brief) |
  | --- | ---: | --- |
  | `🔣️modelDefinition.json` | **9** | 9 — matches: `spatial.shape`, `aec.building`, `aec.building.energy`, `aec.building.structure`, `aec.building.structure.classic`, `aec.building.structure.fem.{line,surface,solid}`, `aec.building.concrete` |
  | `🗂️typologies/**/🔣️typology.json` | **38** | ~38 — matches |
  | `🎬️actions/*.json` | **97** | 88+ — matches ("higher" per sibling session's added `aec.building.*` action kit) |
  | `🎬️interactions/*.json` | **60** | 49+ — matches (same reason) |
  | `🔀️transformations/**/*.json` | **10** | — |
  These are the same 9/38/97/60/10 that `import.meta.glob` would resolve to at runtime if the boot succeeded — this is disk-level, not a runtime observation.
- **Target assets for the two scripted interaction/transformation checks exist on disk**, unexercised:
  - `.../📐️spatial.shape/🎬️interactions/🔣️box.json` and `.../📐️spatial.shape/🎬️actions/🔣️createBoxFromCorners.json` (typology `spatial.shape.primitive.box`).
  - `.../🔥️aec.building.energy/🔀️transformations/🔀️from_geometry/🔣️transformation.json` (id `from_geometry`).

**Verdict for checks 2–5: NOT VERIFIED (blocked upstream by check 1).** No count, no committed object, no derived model was observed in a running app this session.

## 6. Console errors / failed network requests

**N/A.** The Browser pane tab never successfully loaded a page (browser reported "no site is open" / navigation denied since the origin refused every connection), so there is no console or network log to report.

## Honest summary

Nothing in this session contradicts the two fixes recorded in `📓️status.md` — both are still present in source exactly as described, and the disk-level asset counts line up with what the runtime globs are supposed to discover. But **no one has yet actually seen the plugin run**, and this session did not change that: the CAD plugin's own dev build was killed by a 20-minute internal budget after sitting idle on the shared `target/debug` cargo lock, which ~10–12 concurrent cargo processes from other sessions were holding throughout, with system load averages of 27–55. This is the same blocker two prior sessions in this ticket already hit and documented independently. Re-attempting later — once the concurrent `semio-s-plugin-stdio` refactor lands or the machine's cargo queue drains — is the correct next step, not a code change to CAD.

## Files touched this session

- None (read-only investigation; started and later stopped one `preview_start` dev-server process, no repo files modified).
