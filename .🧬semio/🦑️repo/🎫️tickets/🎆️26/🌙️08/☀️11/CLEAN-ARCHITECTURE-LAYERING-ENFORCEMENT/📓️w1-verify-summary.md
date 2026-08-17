# W1 Post-Wave Verification Summary

Verifier: w1-verify subagent. Read-only checks only, no fixes applied.

## Overall status: RED (workspace does not build; both aggregate checks fail)

| Work item | Self-reported status | Cross-checked verdict |
|---|---|---|
| geometry-relocation (semio-s-3d/2d → semio-framework-3d/2d) | Crates rename clean; expected consumer-`.rs` fallout in os-flow, os-host, ui-wgpu Shell, 6 s-plugins; grep-gate should be 0 hits | **CONFIRMED, accurate.** `rg semio-s-3d\|semio-s-2d` = 0 hits repo-wide (grep-gate #1 PASS). But `cargo check --workspace` shows exactly the predicted `semio_s_3d`/`semio_s_2d` unresolved-import fallout in `🦀️host/../../🦀️component.rs` (8 sites) and `🌊️flow/../../📐️brep-geometry/🦀️component.rs` + `.../🖍️drawing/🦀️component.rs` (9 sites) — none of these consuming `.rs` files were in this agent's ownership. Self-report is honest and matches observed reality. |
| registry-genericization (PLUGINS_AREA→PLUGIN_AREAS, isStudioPluginFilter→isHostPluginFilter, studioMode→hostMode) | Own files (registry `📜️script.ts`, vite.config.ts) fully renamed; flagged `dev/📜️script.ts`'s `isStudioPluginFilter` import (out of its ownership) as broken; `check` fails only with pre-existing baseline taxonomy violations (5349, byte-identical) | **CONFIRMED, accurate — but the flagged breakage is still live.** grep-gate #3 (`PLUGINS_AREA\s*=` in registry `📜️script.ts`) = 0 hits, PASS. grep-gate #2 (`isStudioPluginFilter\|studioMode` in `🧰️framework`) = **59 hits, FAIL** — all in files this agent explicitly said were out of its ownership: `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (import + 3 call sites of `isStudioPluginFilter`, still unrenamed) and `📺️renderer/…/ShellHost/🟦️component.tsx` + `ShellHelpers/🟦️component.tsx` + `🎠️kernel/🟦️component.ts` (54 `studioMode` occurrences — the renderer files were never claimed by anyone in this wave's progress files, so `expandPluginRegistry`'s param rename to `hostMode` in kernel `component.ts` is itself now inconsistent with its remaining local `studioMode` var and all its call sites). Registry check violation count independently confirmed byte-identical to baseline (5349) — this agent's own slice introduced zero *new* registry-check violations, consistent with self-report. |
| depcruise (`.dependency-cruiser.cjs` layering rules) | Config loads cleanly, new rules verified by direct regex test against real paths (bunx TS resolution gap prevents live `.ts` cruising) | **CONFIRMED, accurate and honest about the tooling gap.** Not independently re-verified beyond code review (task didn't include re-running dependency-cruiser); self-report's caveat about `.ts` files being silently skipped is a legitimate concern for Wave 2 but is pre-existing tooling behavior, not new breakage. |
| catalog-relocation (stdio owner-table → `🗄️stdio` plugin) | Move confirmed byte-identical; own isolated script proves 0 breaches in the 4 stdio policy functions; both `verify gate` and `policy` fail but on unrelated pre-existing/concurrent breakage upstream (missing `🪵️sourcing` glue.rs targets, missing `◻2d/⚙️engine/🦀️component.rs`) | **CONFIRMED for the catalog move itself.** `verify gate` and `registry:check` full output both confirm the `🪵️sourcing` curate-app missing-file breaches this agent cited, unrelated to the catalog path. However `cargo check --workspace` independently surfaces a *different*, more severe class of stdio breakage this agent's isolated TS-only script could not see (see below) — the stdio plugin does not compile at all right now, for reasons unrelated to the catalog relocation. |

## `cargo check --workspace`: RED — 5 crates fail to compile, 64 errors total

Crate failures:
- **`semio-framework-os-kernel-db`** — `error: couldn't read 🧰️framework/…/🛢️db/📦️packages/🦀️rust/../../📄️document/🦀️component.rs: No such file or directory`. Confirmed absent from `📸️baseline-cargo-check.txt` → fresh concurrent churn by an agent outside this wave's 4 tracked work items, not caused by any of the 4 agents above.
- **`semio-compose-rs`** — 12× `error[E0433]`/`E0432` unresolved `dsl`/`vcs` crates in `compose/client/lib/rs/lib.rs` (lines 716, 717, 720, 723, 7919, etc). Verbatim in `📸️baseline-cargo-check.txt` → pre-existing, predates this ticket, unrelated to Wave 1.
- **`semio-framework-os-flow`** — 9× `error[E0433]`: `semio_s_3d`/`semio_s_2d` unresolved in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📐️brep-geometry/🦀️component.rs:10,11,222,445,480,529` and `.../🖍️drawing/🦀️component.rs:35,157,169`. Direct, expected fallout of geometry-relocation, as predicted by that agent.
- **`semio-framework-os`** (host) — 8× `error[E0433]`: `semio_s_3d` unresolved in `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs:2839,2846,2858,2864,2875(×2),2883(×2)`. Same cause as above.
- **`semio-s-plugin-stdio`** — 16× `error[E0277]` (`DslField` trait bound not satisfied on JFIF/JPG snapshot/diff schema types in `v_jfif_1_01`), 2× `error[E0599]` (`print_op`/`parse_op` missing on `DocxMutation` in `🗿️artifacts/📜️docx/…/🧬️mutations/🦀️component.rs:372` and a nearby `parse_op` site), 1× `error[E0124]` (duplicate `label` field, 3 sites in ecma-376 `any`/`strict`/`transitional` composer `component.rs` files). This is new churn absent from baseline, per the geometry-relocation agent's independent finding — unrelated to the catalog-relocation agent's stdio-catalog move, and blocks `semio-s-plugin-cad`/`semio-s-plugin-lowpoly` from being reachable in the full workspace check.

## `bun nx run @semio-tech/plugin-registry:check`: RED (pre-existing)

Fails at "plugin taxonomy tree violations (area(s) \"✏️s/🔌️plugins\" is \"clean\")" — 5349 violation-shaped lines (`does not exist on disk`/`is not declared by any`/`is missing`), **byte-identical count to `📸️baseline-verify-gate.txt`**. Confirms the registry-genericization agent's claim: this wave introduced zero new registry-check violations.

## `bun ./📜️script.ts verify gate`: RED

Same root failure as above — gate's first step (`nx run plugin-registry:check`) fails, so the gate never proceeds further. Matches catalog-relocation agent's self-report.

## Grep-gate results

1. `rg -n "semio-s-3d|semio-s-2d" --glob '!*/target/*'` (repo root) → **0 hits — PASS.**
2. `rg -n "isStudioPluginFilter|studioMode" --glob '!🤖️generated/*'` (in `🧰️framework`) → **59 hits — FAIL.** All in `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (4: 1 import + 3 call sites of `isStudioPluginFilter`), `📺️renderer/…/ShellHelpers/🟦️component.tsx` (2), `📺️renderer/…/ShellHost/🟦️component.tsx` (~52), `🔨️modules/🎠️kernel/🟦️component.ts` (2, `component.ts:1118,1128`).
3. `rg -n "PLUGINS_AREA\s*=" registry `📜️script.ts`` → **0 hits — PASS.**

## Recommendation for fix-up pass before Wave 2

1. **Rename fallout (blocking, in-scope for this wave, no new agent needed to investigate — just execute):**
   - Bulk-replace `semio_s_3d::` → `semio_framework_3d::` and `semio_s_2d::` → `semio_framework_2d::` in: `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/../../🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../📐️brep-geometry/🦀️component.rs`, `.../🖍️drawing/🦀️component.rs`, plus the 6 s-plugins the geometry agent listed (cad, draw's artifact engine, process, demonstrator, procedural, lowpoly) — those didn't surface in this run only because `semio-s-plugin-stdio` failing upstream hid them from the workspace graph; they will surface once stdio compiles.
   - Bulk-replace `isStudioPluginFilter` → `isHostPluginFilter` in `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (line 39 import, lines ~102/826/853 call sites).
   - Rename `studioMode` → `hostMode` (or decide on final naming) in `📺️renderer/…/ShellHelpers/🟦️component.tsx`, `📺️renderer/…/ShellHost/🟦️component.tsx`, and reconcile `🔨️modules/🎠️kernel/🟦️component.ts:1118,1128` (currently mixes the new `hostMode` param name with a same-named local `studioMode` var feeding it) — these renderer files were never claimed as owned by any of the 4 tracked agents this wave and need an owner.
2. **Unrelated concurrent churn (not this wave's responsibility, but blocking `cargo check --workspace` from going green — needs its own owner before Wave 2's gate can be green):**
   - Missing `🧰️framework/…/🛢️db/📦️packages/🦀️rust/../../📄️document/🦀️component.rs`.
   - `semio-s-plugin-stdio`: `DslField` trait-bound failures on `v_jfif_1_01` schema types, `print_op`/`parse_op` missing on `DocxMutation`, duplicate `label` field (E0124) in 3 ecma-376 composer files.
   - `semio-compose-rs`: pre-existing `dsl`/`vcs` unresolved crates (present since before this ticket — lowest priority, does not regress).
3. Once (1) and (2) are cleared, rerun `cargo check --workspace` and `bun nx run @semio-tech/plugin-registry:check` — the registry check's remaining 5349-violation baseline failure is a separate, larger pre-existing taxonomy-cleanup effort outside Wave 1's scope and should not gate Wave 2 on its own (per registry-genericization agent's baseline comparison), but the 5 crate compile failures and the 59 grep-gate hits above must be zero before Wave 2 starts.

## Files referenced
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/📓️w1-verify-cargo-check-full.txt` (full cargo check output, 14807 lines)
- `.../📓️w1-verify-registry-check-full.txt`
- `.../📓️w1-verify-gate-full.txt`
- `.../📓️w1-verify-cargo-check.txt`, `.../📓️w1-verify-registry-check.txt`, `.../📓️w1-verify-gate.txt` (tail -400/-150 excerpts per task instructions, see below)
