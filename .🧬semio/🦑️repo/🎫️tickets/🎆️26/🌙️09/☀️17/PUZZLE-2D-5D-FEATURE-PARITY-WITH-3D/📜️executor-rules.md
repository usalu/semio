# 📜️ Executor rules (every wave agent reads this first)

Repo root: `/Users/ueli/Documents/semio`. Ticket folder (TICKET):
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`.
Read the root `AGENTS.md` and `✏️s/🔌️plugins/🧩️puzzle/AGENTS.md` (vocabulary: Node=Object=Part, Handle=Vortex=Grip, Edge=Attraction=Fastener, Wire=Cable=Rope).

## Paths
- Puzzle plugin: `✏️s/🔌️plugins/🧩️puzzle` (PLUGIN). Editors: `PLUGIN/🗿️artifacts/{◻️2d|🧊️3d|🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor` (EDITOR2 / EDITOR3 / EDITOR5).
- Crates: `semio-s-artifact-puzzle-2d`, `-3d`, `-5d` (5d depends on 3d). Feature gate: `component-app-assembly`.
- Registries every app verb must appear in (grep the 3d editor for one existing verb, e.g. `focusSelection`, to see all of them): the `puzzle<N>d_command_variants!` macro, `TOOL_JOB_IDS`/`PUZZLE<N>D_RETAINED_TOOL_IDS`, the `bounded_first_step_tool_proofs!` tools list, `build_tool_job` match, `PUBLICATION_CONTRACTS` (lanes must list EVERY store an emit can touch — an undeclared lane is a runtime fault), `.action_with`/`.action_interactive_job(id, InteractiveJobClassification::Migrated)` in `create_puzzle<N>d_app`, `command_from_action`, terminology labels EN+DE (`🗣️terminology/🦀️.rs`, compile-checked), and the fixtures `…/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json` + `PLUGIN/🧫️fixtures/🔏️publication-authority/🔣️.json`.
- Publication audit (cheap, TS, allowed): `cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit` (optionally `Puzzle2dPlayApp|Puzzle3dPlayApp|Puzzle5dPlayApp`) — must exit 0.
- Audit reports with file:line evidence: `TICKET/📓️E1…E10*.md`. The 3d reference inventory is `📓️E1-3d-feature-inventory.md`.

## Hard rules
1. `cd /Users/ueli/Documents/semio` explicitly at the start of EVERY Bash call (cwd drifts; 0 grep hits is usually cwd drift). Quote emoji paths.
2. NEVER run a modifying git command (`commit`, `stash`, `checkout`, `restore`, `reset`, `worktree`, `clean`). Read-only git is fine. An auto-committer and many other agents work in this tree at the same time.
3. Other agents edit the SAME files concurrently (your sibling slices and unrelated peers). Re-read the region right before each edit, make small uniquely-anchored `Edit`s, never rewrite or reformat a whole file, never revert or "clean up" changes you did not make, never delete code because it looks unused unless your slice owns it. If the workspace is momentarily uncompilable because of someone else's half-landed edit, wait 2–3 minutes and retry; only fix it yourself if it is inside your slice's files.
4. Builds: FOREGROUND only (background children die with your turn). Allowed: `CARGO_INCREMENTAL=0 cargo check -p <crate> --features component-app-assembly`, the same with `--target wasm32-wasip2` (wasm-gated code is invisible natively), and FILTERED unit tests `CARGO_INCREMENTAL=0 cargo test -p <crate> --features component-app-assembly --lib -- <filter>`. macOS has no `timeout` command. Never set a private `CARGO_TARGET_DIR`. Never `pkill cargo`. Forbidden: `nx` builds/activate/serve, dev servers, Playwright, wasm-pack — the coordinator owns those. Require warnings/"Finished" lines as proof that a check really type-checked; capture output to `TICKET/🗑️generated/<slice>/<name>.txt`.
5. Never touch any `🗑️generated` folder other than writing into `TICKET/🗑️generated/<slice>/`. Never open, close or reopen tickets. Never edit `AGENTS.md`.
6. Code style (AGENTS.md): concise; NO comments inside definitions; docstrings start with a unique fitting emoji; schema-first (update `🧬️schema` `.ts`/`.json`/`.graphql`/`.proto` twins by hand when a config/transient/mutation shape changes); no legacy/compat layers, no deprecations, no migration scripts; every label EN + DE; temporary logs carry the `[DEBUG] ` prefix and are removed before you finish; test-driven — every feature gets at least one law (unit test) and, for document mutations, a language-neutral fixture vector plus the Python second implementation where the artifact has one.
7. Known runtime traps in this codebase (do not reintroduce): `BatchOnlyPendingRewrite` verbs are dead in the app; an unclassified verb aborts the descriptor probe; undeclared publication lanes fault; `UiFixedList` has 32 slots and `UiText` 512 bytes (use `UiText::clipped`, paged/tree-window sections); map keys of `UiValue` maps must be ascending; never encode a scene straight into the 32 KiB surface doc (use `scene_surface` lanes); stay under 64 KiB contiguous guest requests; every store dies after 64 applied edits so a multi-placement gesture must be ONE edit; per-frame state belongs in the window transient, not config; work must declare an honest extent and fold footprint (`work_items`).
8. Port from 3d (or from the sibling artifact that already has the feature) — read the source you port from fully; do not invent a second mechanism when one exists. Prefer reusing/generalising shared code that sits close by over copy-paste when both artifacts can share it without a new dependency edge.
9. Deliverable: working, compiled source + laws, and ONE report `TICKET/📓️wave-<slice>-report.md` (what landed with file:line, registries touched, commands run with verdicts and warning counts, what is NOT verified, hand-offs to other slices). Your final chat reply is ≤12 lines pointing at the report. Do not stop halfway or ask whether to continue; finish the slice.

## Addendum 2026-09-17 15:15 (after the 11:50 session-limit outage)
- The whole fleet was cut mid-slice by a usage limit and resumed. Re-read your own region of the tree before continuing: siblings and peers kept editing, and the auto-committer ran.
- Disk hit 0 GiB during the outage (incremental caches). `CARGO_INCREMENTAL=0` on EVERY cargo invocation is mandatory.
- Build gate: 18 concurrent checks of one crate starved everybody. Before any cargo command wait until fewer than 3 cargo invocations for your crate are alive:
  `until [ "$(pgrep -f 'cargo (check|test).*semio-s-artifact-puzzle-[2]d' | wc -l)" -lt 3 ]; do sleep 20; done` (use `[5]d` for the 5d crate; the bracket keeps your own shell from matching). Batch your edits and check at milestones, not after every edit.

## Addendum 2026-09-17 19:55 (after the SECOND usage-limit outage, 16:45–19:50)
- The fleet burns the 5-hour usage window in about one hour, mostly in build-wait/poll loops against a crate that siblings keep half-broken. NEW RULE: spend tokens on source, not on waiting.
  1. Finish your remaining SOURCE work (code, laws, fixtures, schema twins, labels) first.
  2. Then run at most ONE gated native `cargo check` of your crate with `--message-format=short` into your generated folder. Fix only errors whose path is inside YOUR slice's files (grep the output by path). Errors in other slices' files are NOT yours — list them in your report and move on; never loop waiting for siblings.
  3. Run your filtered tests / wasm32-wasip2 check only if that check was green. Otherwise mark them "owed to integration" in the report.
  4. Never sleep-poll for more than 5 minutes in total. Write the report and END your turn — two integrator agents take each crate to green afterwards.
- Do not start serve supervisors or any other long-lived process (seven duplicate supervisor loops were found and killed).

## Addendum 2026-09-17 20:10 — CORRECTED build gate (the 15:15 one deadlocks: `pgrep -f` also matches every WAITING shell, whose argv contains the cargo command that follows the loop)
Count real cargo binaries only:
`until [ "$(ps -axo args= | awk '$1 ~ /\/cargo$/ && /semio-s-artifact-puzzle-2d/' | wc -l)" -lt 3 ]; do sleep 20; done`
(swap the crate name). If you are stuck in the old gate, abandon it and use this one.
