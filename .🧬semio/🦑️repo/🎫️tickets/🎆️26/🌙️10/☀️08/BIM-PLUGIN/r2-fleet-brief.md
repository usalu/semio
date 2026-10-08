# 📋️ BIM Fleet Brief (read fully before touching code)

T = `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\BIM-PLUGIN` (Git Bash: `/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`).
Repo root: `C:\git\semio` (Git Bash `/c/git/semio`). Windows 11, Git Bash + PowerShell.

## Read first
1. `T/r2-design.md` — the target design (names, snapshot, diff, mutation catalogue, inference catalogue, UI, IO).
2. `C:\git\semio\AGENTS.md` — repo rules. Highlights: bun + nx; scripts only in `📜️script.ts`; no runtime external
   libraries; test-driven with language-agnostic tests and third-party oracles; schema-first; multi-language (en first,
   de second, no default); accessible + customizable UI; CQRS/event-sourced, no CRUD/CRDT; progress + cancellation for
   expensive work; concise code; NO comments inside definitions; docstrings start with a unique fitting emoji; no
   legacy/compat/adapters/deprecations; `[DEBUG] ` prefix for temporary logs; never claim a test passes without running it.
3. The `T/r1-explore-*.md` reports relevant to your scope (artifact mutations, inferences, editor/viewer, plugin
   anatomy, tests/tooling, io/persistence, aec/geometry).
4. The diff-only laws: `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES\📋️design.md`
   (L1–L5, gate rules R8–R16). The BIM plugin is born compliant.

## Templates to copy (in this order of preference)
- Whole compliant artifact: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting` (leaf triads with concrete inverses + sum-law tests,
  editor, viewer, inferences, io, examples, oracles).
- Inferences: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/.../🧬️schema/💡️inferences` (`InferredField`), gismap.
- 3D window: `✏️s/🔌️plugins/💠️lowpoly` model window, `📐️cad` building window. 2D canvas window: `🖍️draw` canvas.
- Do NOT copy drawing's test style (`apply_drawing_mutation(&mut …)`) or whole-list replaces.

## Concurrency (hard rules)
- Other agents (BIM fleet and unrelated humans/agents) edit files concurrently, sometimes the same files. Never run
  modifying git commands (`commit`, `stash`, `checkout`, `restore`, `reset`, `clean`), never use worktrees.
- Re-read a file immediately before editing it; prefer small `Edit` calls; keep every file compile-atomic (never leave a
  file half-converted). If an `Edit` fails because the text moved, re-read and retry.
- Shared registration files (`✏️s/Cargo.toml`, `🌎️hub/Cargo.toml`, `🔣️taxonomy.json`, `.vscode/launch.json`, the
  mutation aggregate `🧬️mutations/🦀️.rs`, the artifact root `🦀️.rs` mount tree, the inference aggregate): add your rows
  with minimal surgical `Edit`s; never rewrite these files wholesale.
- Stay inside your scope. If a compile error is in a file you do not own, wait (re-check after a few minutes) and note
  it; do not "fix" peers' in-progress work unless it is a trivial typo blocking you, and say so in your report.
- Scratch/output files ONLY under `T/🗑️generated/<your-label>/`. Input scripts you want to keep go to `T/` named
  `r<wave>-<label>-<what>.<ext>`.

## Builds (hard rules)
- Every cargo call goes through the gate: `"$T/🚦️gate.sh" <your-label> -- cargo <args>`. The gate picks one of 6 slots
  and sets everything itself: `RUSTC_WRAPPER=""`, `CARGO_BUILD_JOBS=4`, new-layout/fine-grain-locking OFF, and a warm
  per-slot `CARGO_BUILD_BUILD_DIR`/`CARGO_TARGET_DIR` under `.🧬semio/🦑️repo/⚡️cache/cargo/{build,target}-bim-<slot>`
  (the shared default cache deadlocks). Never set these variables yourself, never create private build dirs.
  Foreground only (never `run_in_background`, never `Monitor`). A first build in a slot may take ~5 min.
- Bless/test env vars (e.g. `BIM_BLESS=1`) go before `cargo` inside the gate call: `"$T/🚦️gate.sh" x -- env BIM_BLESS=1 cargo test ...`.
- Never kill cargo/rustc processes you did not start.
- Artifact crate: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --message-format=short`
  then `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model <filter>` (native). Wasm check:
  add `--target wasm32-wasip2`. Run from `/c/git/semio`. Pipe long output to `T/🗑️generated/<label>/*.txt` and grep it.
- Use a timeout of 600000 ms on cargo calls; if a build exceeds it, re-run (incremental).

## Reporting
- Write `T/r<wave>-exec-<your-label>.md`: what you built (files created/updated), before → after, exact commands run
  and their results (pass/fail counts), open issues. Final chat message ≤ 12 lines + report path.
- "WRITTEN BUT UNVERIFIED" with a precise list is acceptable only when a build was impossible; say why.
