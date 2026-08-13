# Important — read first, every agent

Full design + rationale: `/Users/ueli/.claude/plans/the-current-artifact-system-eager-scone.md` (also copied as `📓️design-composition.md` + `📓️design-stdio-subsets.md` + `📓️orchestration.md` in this ticket for the raw agent transcripts).

## Hard rules (repo-wide, non-negotiable)

- No git-modifying commands (`commit`/`stash`/`checkout`/`reset`/worktrees). Live shared tree — another session (SEMANTIC-MUTATIONS-OVERHAUL, "SMO") is editing concurrently.
- All temp files, logs, scratch scripts inside THIS ticket folder. Scratch logs are `.txt`, never `.log` (`*.log` is repo-gitignored and silently drops from `ticket_close`'s file list).
- `bun` + `nx`; permanent scripts only in `script.ts` at the relevant directory; `project.json`/`package.json` just call it.
- Regions (`//#region Name` / `//#endregion Name`) for structure; no new files outside the existing taxonomy shapes; `[DEBUG] ` prefix on any temporary logging, removed before a wave is reported done.
- Greenfield: no compatibility layers, no legacy support, no deprecations. Regenerate fixtures/examples; never upcast/migrate them.
- Semantic mutation vocabulary only: `SetSnapshot`/`NoMutation`/`CollectionMutation` (public) are BANNED. Use the existing `🧬️mutations/<slug>/{🦠️mutation,🔺️diff,↩️inverse}` triad shape and `MutationKind`/`SemanticMutation`/`#[derive(Mutations)]`.
- Never run bare `cargo check` — always scope `-p <crate>`. A red workspace is not a red plugin.
- `CARGO_TARGET_DIR=<this-ticket>/🎯️target` for every cargo invocation (shared; the flock serializes concurrent checks — "Blocking waiting for file lock on build directory" is normal, wait, don't kill it).
- Never close this shared ticket. Never edit `📓️status.md` except the orchestrator. Append your report to your assigned `📓️waveN-reports/` file only.

## Hot-file ownership (binding)

| File / subtree | Owner | Everyone else |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | W1 mechanism agent, then frozen | read-only; file a `sharedFileRequests` entry |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, `🧬️schema/🦀️component.rs`, `🛂️manifest/🦀️component.rs` | W1 mechanism agent | read-only |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/**`, `🏪️store/**`, `🌿️vcs/**` | W1 mechanism agent | read-only |
| `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` | W1 mechanism agent | read-only |
| `✏️s/🔌️plugins/🗄️stdio/**` incl. `📦️glue.rs` + `🦀️component.rs` | W2 stdio agent, then W5 serializer | read-only; consume stdio types only |
| `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` | W2 stdio agent, then W5 serializer | read-only |
| repo-root `📜️script.ts` | W6 ratchet agent only | read-only, even for allowlist entries — request it |
| `🦑️repo/…/📚️library/🔣️taxonomy.json`, its TS `📦️index.ts` | W6 ratchet agent | read-only |
| `✏️s/🔌️plugins/<P>/**` minus `📦️glue.rs`/`📦️index.ts` | that plugin's one fan-out agent | nobody else, ever |
| `✏️s/🔌️plugins/<P>/📦️packages/🦀️rust/📦️glue.rs`, TS `📦️index.ts` | W5 serializer (fan-out agents file `🔧️patches/<P>-glue-patch.txt` + `## sharedFileRequests`) | — |

## Coordination with SEMANTIC-MUTATIONS-OVERHAUL (SMO)

SMO is running wave-2 mass fan-out across `✏️s/🔌️plugins/**/🧬️mutations/**` right now, in another session, on this same tree.

1. ⚠️ **Plugin-level mutual exclusion — DO NOT use report files as the clearance oracle.** `📓️smo-clearance.md` (computed at ticket-open from SMO's report dirs) is **retained only as a historical record; it is NOT authoritative and must not be used to dispatch.** The report-existence oracle was proven unreliable in BOTH directions:
   - **False "clear"**: `demonstrator-playground-1-any-report.md` exists in SMO's `📓️wave2-reports/`, yet SMO has a *live lane* on demonstrator. SMO runs multiple waves per plugin ("Wave R", "Wave C"), so a report means "that wave finished", not "the plugin is done".
   - **False "not clear"**: note/energy/space have no SMO report at all, yet SMO explicitly released energy.
   **The only valid oracle is an explicit per-plugin handshake with the owning session before dispatch** (SMO = `semio-9f`, APA = `semio-52`; names drift, use `ListAgents`). The orchestrator does this, not fan-out agents. Known-bad as of this writing: **demonstrator and note are NOT clear.**
   General lesson, twice-burned in this ticket: **do not infer a live session's state from static files it happens to have written** — an unchanged report count also got misread as dormancy. Ask the session.
2. **Never touch `🧬️mutations/**` of an uncleared plugin.**
3. **stdio is the big overlap risk** — claim it explicitly in `📓️status.md` before W2 starts.
4. **The one forbidden collision**: our W1 and SMO's eventual wave-4 ratchet both want to write `🔌️plugin/🦀️component.rs`. Whoever starts first wins; the other waits. Check SMO's `📓️status.md` before starting W1.
5. **Transient failure protocol**: on a compile error in a file OUTSIDE your boundary — establish whether it is someone else's in-flight work (churn detection below); if so, it is concurrent churn. Retry `cargo check -p <your-crate>` up to 3× at 60s intervals. If it persists, record it under `## Concurrent-churn observations` in your report, prove zero errors originate in your own boundary (grep the cargo output for your path), report `blocked-mechanism`, and stop. Never "fix" someone else's file.

6. ⚠️ **This repo AUTO-COMMITS — `git status` is NOT a reliable churn detector.** A background process periodically commits the whole tree (commits look like `🐙️ueli🎆️26🌙️06☀️04🚩️<n>`, incrementing; one landed mid-wave during this ticket and swept a sibling agent's framework edits). Consequences:
   - Work that landed minutes ago is already committed and shows **clean** in `git status --porcelain`. An empty `git status` does NOT mean "nobody touched this file".
   - If your own edits disappear from `git status`, they were committed, **not lost**. Never run a git-modifying command to "recover" them (forbidden anyway).
   - Detect churn with these instead: `git log --oneline -5 -- <path>` (recent commits touching it), `stat -f '%Sm' <path>` (mtime, macOS), `git log --oneline -3` (has the auto-committer advanced since you started?).
   - Never assume a clean tree means a file is safe to overwrite — read it first.

## ⚠️ Agent dispatch rule — never let an agent wait on a background job

Three agents in this ticket burned **~600k tokens each (≈1.8M total) producing nothing** by spawning background cargo checks / monitors and then idling in a "waiting for the notification" loop. Once in that state they self-report `completed` between wake-ups, so `TaskStop` refuses them as not-running while they keep re-notifying — effectively unkillable zombies.

**Every agent brief must say, verbatim:** run cargo in the FOREGROUND and simply wait; `Blocking waiting for file lock on build directory` is normal under concurrent sessions; do NOT spawn background wait-tasks or monitors; do NOT idle in a wait loop. If a command seems to hang, it is compiling.

Corollary for the orchestrator: an agent's *final message before dying* is worth reading — C1's dying line ("the plugin check failed with real compile errors") is the only reason a false "green" claim didn't go out to another session.

## 🚨 THE AUTO-COMMIT MESSAGE'S DATE IS FAKE — NEVER PARSE IT. Use `git log --date=iso`, always.

**Confirmed 2026-08-13, after it produced two wrong "pre-existing, two months old" claims in this ticket's own reports (cad-report.md, process-report.md — both corrected).** Every auto-commit message has the literal form `🐙️ueli🎆️26🌙️06☀️04🚩️<N>` — that `🎆️26🌙️06☀️04` ("2026-06-04") is a **fixed, stale template string, baked in and identical across every single commit regardless of when it actually happened.** Verified across 20 consecutive commits spanning 2026-08-11 through 2026-08-13 (today): the message text never changes, the real date (`git log --date=iso`) moves every time. **Only the incrementing `🚩️<N>` flag number is chronological in the message; the calendar date next to it is not real.**

**Consequence**: any report in this ticket (including two already written by name above) that says "traced to commit X, dated 2026-06-04, therefore two months before this ticket opened" used the fake date and drew a false conclusion. The commits in question actually landed on **2026-08-12/13 — during this ticket's own active window**, not two months prior.

**The fix, mandatory from now on**: when tracing a failure's origin, always run `git log -1 --date=iso --format="%H %ad %s" -- <path>` (or `git show --stat <hash>`) and read the `%ad`/`Date:` field — **never** the digits inside the commit message's `🎆️🌙️☀️` glyphs. This ticket opened at **2026-08-12 15:02:49** (first commit touching this ticket's folder) — that is the real cutoff for "predates this ticket," not any date parsed from a message.

**Also note**: these auto-commits are repo-wide snapshots bundling every concurrent session's work in one commit (a single commit here touched 17.6k lines across rust/json/typescript, spanning multiple unrelated tickets' summaries in its body). A commit "touching" a file does not mean *this ticket* wrote the change inside it — read the actual diff hunk, and cross-reference against what your own migration touched, before attributing blame.

**If you already filed a "pre-existing" classification based on a message-parsed date, redo it with `--date=iso` before trusting it.** A trivial, safe, unambiguous bug (e.g., an obvious off-by-one or a self-evidently wrong hardcoded test expectation) is often cheaper to just fix outright than to keep chasing provenance — do that instead of leaving an unresolved dating question in a report.

**Cross-confirmed by DKM (2026-08-13):** they independently hit the same trap tracing file provenance tonight and are correcting their own ticket's records. This is a repo-wide gotcha, not specific to this ticket — assume any peer session's date-based provenance claim from before today needs the same re-check if it matters to you.

**Possible resolver seam landing (2026-08-13, unconfirmed, watch for it):** `layout`'s wave-4 agent reported a new `ArtifactView::with_children` method appearing mid-pass in W1-owned framework code (not adopted — flagged for a future wave). Every exemplar and fan-out agent so far has hit the same wall ("no `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle` yet") and built a `thread_local!` scratch-cache workaround. If `with_children` is real and lands, it may retire that workaround pattern — worth checking against `🔌️plugin/🦀️component.rs` directly before the next batch of fan-out agents, rather than having each one independently re-discover the same missing mechanism.

**Moving stdio baseline (DKM, 2026-08-13):** `cargo test -p semio-s-plugin-stdio --lib` was 2246 passed/2 failed ~9h ago, now **2414 passed/5 failed** — net +168 tests, +3 new failures, from live concurrent work (not this ticket's). If W7 diffs against an old stdio number, re-measure first; don't gate on a stale baseline.

**W6 pre-briefing from DKM (2026-08-13), save this for W6 ratchet wave:**
1. **`policyTaxonomyDirsBreaches`'s restrictive walk (`📜️script.ts:4076+`) is effectively dead for every artifact in the repo.** `artifactChildDirs` never includes `🏅️standards`, so the walker's `NestedFacetWalk` never descends into any artifact's actual subset facets (every artifact in the repo is now new-shape, `<artifact>/🏅️standards/🔖️v/🪆️subsets/✳️x/🧬️schema/…`) — it hits the else-branch at `🏅️standards` and stops. Confirmed empirically: `bun ./📜️script.ts policy` → 0 hits for "not a recognized artifact/representation dir", corroborated by `✳️brep/🧬️schema/💡️inferences/✅validation-report/` sitting at the exact contested depth unreported. This means DKM did NOT need a taxonomy amendment to add Rust-only compute subdirs — the walk wasn't restricting that depth to begin with (request retracted). **This is real latent policy debt for W6 to eventually fix** (retarget the walk at `🏅️standards`) — not urgent, but currently invisible governance that looks like it's enforcing something and isn't.
2. **Current policy baseline (2026-08-13, DKM's measurement): 23,792 breach lines total, but 22,274 are ONE kind** (`handcrafted-grammar/spec-distinctness`) — if W6 gates on a raw total, that one policy drowns every other signal. Next-largest: `os-state-authority/item-scope-global` (296), `artifact-schema/facet-completeness` (273), `taxonomy/dead-example-leaf` (255). Diff W6's own baseline against these per-kind numbers, not the raw total.

## 🔎️ Sweep the PATTERN, don't wait for the compiler

A scoped `cargo check -p X` surfaces only the crate you asked for, and in a 100+ crate workspace a systemic defect then reveals itself **one crate at a time over hours** — worse, a crate that fails early (stdio) hides every latent defect downstream of it.

**Proven here**: a script walking every `#[path = "…"]` in the repo and `stat`-ing its target found **2 genuinely dangling mounts** among **8,345 real ones** — both latent, neither yet reached by the compiler:
- `➗️mathematical`'s glue mounted `🎮️commands/📄️document/` where the dir is now `📄️artifact` — the same closed-ticket rename debt that stranded nine `📌️panels` mounts.
- `🌀️procedural`'s mutations dispatch mounted `➕create-widget/` where the dir is `🌱create-widget/` — **the emoji changed**, so a text search for the slug still matched.

Both were latent: the workspace build never reached them because stdio failed first. Both are now fixed and the repo is at **0 dangling mounts**.

⚠️ **And a cautionary tale about the sweep itself.** The first run of that script reported **20** dangling, not 2. Eighteen were false positives: the regex matched `#[path = "…"]` written as *prose inside `//!` doc comments*. That inflated number was broadcast to a peer before being checked — minutes after praising that same peer for retracting an inflated count of their own (they had grepped for a symbol, got 46 files, and reported it as 46 broken files when exactly 1 was).

**So the sweep rule has a second half, and it is not optional**: *grep to find, enumerate to count.* A pattern match locates candidates; it does not size a problem. Before quoting a number — especially before sending it to another session — confirm each hit is the thing you think it is. Strip comments, check the target, count what survives.

**Do this whenever a class of defect is suspected** — dangling mounts, missing struct fields, stale import paths. Peer sessions independently found the same leverage: one grep for `Self::infer` with the trait imported inside `mod tests` found a second instance in `🪐️space` in 30 seconds that the compiler would have surfaced days later.

## Repo conventions learned during this ticket (obey these)

1. **Derive crates keep two byte-identical copies.** `<module>/✨️derive/🦀️component.rs` and `<module>/✨️derive/📦️packages/🦀️rust/📦️glue.rs` must stay identical — Cargo compiles the *glue* copy, so editing only `component.rs` silently does nothing. Verified true for both `🧬️schema/✨️derive` and `🗣️dsl/✨️derive`. Edit one, then mirror it exactly, then `diff -q` the pair before reporting done.
2. **`mcp__repo__file_integrate` misbehaved** on that mirroring (wrapped the whole file in a nested `mod helpers {}`, duplicating everything). Mirror by hand and verify with `diff -q`.
3. **`#[link(...)]` is unusable as a custom field attribute** — `link` is a built-in Rust attribute (extern-block FFI) and applying it to a field is a hard error (E0659/E0539/E0459), not a lint. The composition link-slot attribute is therefore `#[link_slot(roles("a", "b"))]`. The child attribute `#[child(kind = "s.stdio.mesh")]` is fine as-is.
4. **Additive struct fields still break struct literals** (serde `default` only affects (de)serialization, not Rust construction). After adding a field, grep for `TypeName {` across the whole workspace — not just your crate — and either fix the literals or file them under `sharedFileRequests`.
5. **Adding an enum variant is expensive** where the enum is matched exhaustively. Measure with `grep -rln "EnumName::"` before committing to it. (`Shape::` matches in ~20 files — see deviation D1 in `📓️status.md`.)

## 🚨 D2 — 6 failing law tests in `✳️text`. ROOT CAUSE NOT YET CONFIRMED. Fix before authoring more subsets.

> **Diagnosis correction (read this first).** This was initially written up as "whole-list diffs are the defect". That is **not yet established** and may be wrong. Evidence against it: `SemioTextDiff::apply` is correct (`next.runs = list.values.clone()`), and the failing assertion (`restored == ["hello"]` vs base `["hello","world"]`) is only reachable if the **forward mutation had no effect at all** — i.e. `InsertRun.diff(base).apply(base) == base`. That points at the dispatch enum failing to route a variant to its triad's `diff`, producing an empty diff, rather than at the diff's *shape*. The `🔺️diff` file also argues in-place that whole-list is honest for `text` specifically, since the snapshot has exactly one mutable field.
>
> **Find the real root cause before reworking anything.** Both concerns below stand on their own merits, but do not conflate them.

### Concern A — the 6 failing tests (blocking, cause unknown)
`text::…::{insert_remove_run_round_trips, add_remove_mark_round_trips, reorder_runs_round_trips}`, `text::io::…::fixture_honesty_law`, and `any::io::derived_composition::…::{diff_grammar_conformance_law, ops_grammar_conformance_law}`. Start by asserting `forward != base` inside `round_trip` — if that fires, the bug is dispatch routing, not diff shape.

### Concern B — whole-list collection diffs (design, non-blocking)

`✳️text` (and, copied from it, `✳️table` and `✳️graph`) declare their collection diff as a **whole-list replace**:

```rust
pub struct SemioTextDiff { pub runs: Option<SemioTextRunList> }   // SemioTextRunList = the ENTIRE Vec
// …and each 🔺️diff leaf does:
let mut runs = base.runs.clone(); runs.insert(at, payload.run.clone());
SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
```

**This is apply-then-capture in disguise** — it computes the post-state and stores it wholesale — which `📓️taxonomy.md` forbids ("build the sparse diff directly from `(payload, base)` — never apply-then-capture"). It is also a whole-object replace of the collection, the very shape this programme exists to eliminate.

**Consequences already observed**: 6 real test failures — `✳️text`'s `insert_remove_run_round_trips`, `add_remove_mark_round_trips`, `reorder_runs_round_trips`, its `fixture_honesty_law`, plus `✳️any`'s `diff_grammar_conformance_law` and `ops_grammar_conformance_law`. Inverses only restore correctly if each is diffed against the *current* state rather than `base`, which the test harness had to special-case with a comment explaining the fragility. That fragility IS the defect.

**Correct primitive — already exists, use it**: `📡️spr/🎮️command/🦀️component.rs` provides `IndexedTripleDiff<V, Patch>` + `indexed_apply` (`:510`, `:531`) for ordered/index-addressed collections and `NamedTripleDiff<K, V, Patch>` + `named_apply` (`:468`, `:489`) for keyed ones. Ordered run/row/node lists take `IndexedTripleDiff`; name- or id-keyed collections take `NamedTripleDiff`.

**Why the audit missed it**: the four mechanical gates check that a real `pub fn diff` *exists*, not that it is *sparse*. A structural pass cannot catch this — only running the law tests does. **A structural audit is not a correctness audit.**

**Action**: rework `✳️text`, `✳️table`, `✳️graph` onto DiffKit before authoring spatial `object` and `kit`, and before any of this reaches the 33-plugin fan-out.

## Authoring a `🧬️mutations` facet (BINDING — agreed with the SMO session)

Any new or restructured mutation facet lands inside the SEMANTIC-MUTATIONS-OVERHAUL ticket's policy scope. Author it conforming the first time; retrofits cost that ticket real work. Required reading before writing one: `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` (closed verb table, naming, addressing) and `📓️fanout-brief.md` (8-step recipe).

**Forbidden vocabulary** — `SetSnapshot` (whole-document replace is NOT an in-history mutation at all; it goes through `ArtifactStore::reset`, a locked user decision), `NoMutation` (a mutation with nothing to undo returns `Vec::new()` from `MutationKind::inverse`), and `CollectionMutation` in a public `pub enum *Mutation`.
⚠️ **The policy greps raw file content including comments** — prose merely *mentioning* these names trips it. Do not write them in docstrings either. (This doc is exempt; it is not under `✏️s/`.)

**Four mechanical gates:**
1. **Triad dirs ↔ dispatch variants, 1:1 in both directions.** No orphan dir, no variant without a dir.
2. **Unique emoji per directory within a facet** (`policyMutationEmojiUniquenessBreaches`, high priority). Do not reuse an emoji across sibling triad dirs.
3. **Real leaves, not shims.** `🦠️mutation` contains an actual `impl MutationKind<`; `🔺️diff` a real `pub fn diff` building the sparse diff directly from `(payload, base)` — never apply-then-capture, never a snapshot clone; `↩️inverse` a real `pub fn inverse` reconstructing from `base`, returning `Vec::new()` when the target is absent.
4. **Non-stub `🟦️component.ts` beside every triad `🦀️component.rs`**, and real glue `#[path]` mounts — never inline `#[path = "."]` self-wiring in the dispatch file.

**If a facet cannot be authored conformingly, leave its dispatch enum EMPTY with no triad dirs and flag it in your report.** An empty facet is trivial for the SMO session to populate; a non-conforming one is a teardown. Never invent vocabulary to fill a gap.

**This ticket's composition verbs** (all within the approved core, reviewed and confirmed by the SMO session): `create`/`delete` (child lifecycle; `delete` captures the escrowed child content for its inverse), `extract`/`inline` (owned child ↔ standalone link), `bind`/`unbind` (link attach/detach), `change` (re-pin, i.e. `change-link-pin`). Two rulings worth preserving because a later reader will second-guess both:
- **`bind`/`unbind`, not `connect`/`disconnect`** — a link fills a *named slot* on the parent as a handle; it is not an edge row in an edge collection. The taxonomy's rule: "a parameterization gets `bind`/`unbind` instead."
- **`change-link-pin`, not `update-link-pin`** — `update` is reserved for an inseparable ≥2-field facet rewritten atomically. Re-pinning sets the single `pin` field while `target`/`role` stay put, which is exactly `change` (record `ChangedLinkPin`).

## W4 fan-out tracking (orchestrator-maintained)

Batch 1 dispatched 2026-08-13: `process` (→C:flow,brep), `fem` (→ONE fem-core, C:mesh,table R:brep,drawing — largest single migration in the fan-out, partial completion explicitly sanctioned if the full 11-type consolidation can't land in one pass, see the agent brief), `gis` (→terrain C:mesh, map C:drawing+image+value), `flow` (→C:flow, canonical editor). Reports land at `📓️wave4-reports/<plugin>-report.md`.

**`remodel` was skipped for batch 1** — found with a live uncommitted edit (`🎛️apps/📸️remodel/⚙️engine/🎥️video/🦀️component.rs`, an `engine as X` → `subsets::any::io as X` import-path fix) matching ticket #2553's in-flight `⚙️engine`-dissolution fan-out pattern, not this ticket's own work. Do not dispatch a fan-out agent into `remodel` until that settles — re-check `git diff --stat -- ✏️s/🔌️plugins/📸️remodel` first.

Batch 1, 2a, 2b, Cb: ALL DONE. Batch Ca: shooting/layout/animate done; trinity blocked-mechanism (DKM's rename mid-sweep, re-check before retry); raster's migration itself looks sound but final re-verification pending on unrelated external `ArtifactView`/`HistoryView` churn (isolated to raster's one test file, not a broad regression — confirmed by re-checking writer/animate still clean). Batch Da (sourcing, energy, space, vcs) dispatched 2026-08-13, in flight. Remaining: batch Db (puzzle ~46k lines, block ~20k, architect ~43k — puzzle and architect both comparable in scale to fem/norm, expect possible partial completions), `demonstrator` (LAST, still needs SMO clearance re-check), and `remodel`/`note` (deferred — re-check before dispatching: remodel pending #2553's engine-dissolution settling via `git diff --stat`, note pending explicit SMO clearance handshake). **`demonstrator` and `note` are confirmed NOT SMO-clear** — do not dispatch either until re-checked with the SMO session directly.

**Note on DKM's math dissolution**: `semio-framework-math` is actively being split into standalone `geometry`/`graph` (and presumably other) crates across the whole repo tonight, touching files in many plugins transiently (confirmed hitting `🖱️ui`, `dag`'s report, `trinity`). Expect this class of transient churn in ANY plugin during W4 for the next while — before blaming a batch-2b/C/D failure on your own migration, grep the error for `math::`/`geometry::`/`graph::` import-path shape and check `git diff --stat` on the specific broken file; if it's mid-rename, it's DKM's, not yours.

## Report shape (every wave)

Follow the shape of SMO's `📓️wave2-reports/norm-en1994-1-any-report.md` as the reference: what changed, files touched, verification commands + results, `## sharedFileRequests` (file, region, reason, patch file path), `## Concurrent-churn observations`, honest pass/fail — never claim a test passed without running it.
