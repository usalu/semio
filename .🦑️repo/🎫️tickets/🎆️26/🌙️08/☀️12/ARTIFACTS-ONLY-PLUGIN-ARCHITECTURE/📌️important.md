# Important — read first, every agent

Ticket: `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` ("APA"), issue #2549, goal `🎯r2602🎯runningsketchpad`.
Approved plan: `/Users/ueli/.claude/plans/the-new-architecture-is-prancy-pearl.md`. Census: `📓️w0-census.md`.

## The thesis (one paragraph)

A plugin consists of **exactly two things: `🎛️apps` and `🗿️artifacts`** — plus a root `🦀️component.rs`, `AGENTS.md`, and `📦️packages` wiring. All IO, all state changes, all registration and all side effects belong to artifacts, never to apps and never to a setup facet. The named violation that opened this ticket is `✏️s/🔌️plugins/💠️lowpoly/🔧️setup/🦀️component.rs`, which mutates OS-host global registries from a plugin setup facet and registers IO for `3d.mesh`, a kind lowpoly does not own. Everything in this ticket exists to make that shape impossible rather than merely absent.

## Hard rules (non-negotiable)

1. **No git-modifying commands, ever** — no `commit`, `stash`, `checkout`, `reset`, `rebase`, no worktrees. The tree is shared with live human and agent sessions. `isolation: "worktree"` is forbidden on every Agent/Workflow call.
2. **The repo AUTO-COMMITS** (commits look like `🐙️ueli🎆️26🌙️06☀️04🚩️<n>`, incrementing). `git status` is therefore **not** a churn detector: work that landed minutes ago shows clean, and a vanished edit was committed, not lost. Detect churn with `git log --oneline -5 -- <path>`, `stat -f '%Sm' <path>`, and `git log --oneline -3`. Never run a git command to "recover" anything.
3. **All temp files, logs, scratch, reports go inside this ticket folder.** Scratch logs are `.txt` or `.md`, **never `.log`** (`*.log` is repo-gitignored and silently drops out of `ticket_close`'s file list). Never create files anywhere else outside your assigned boundary.
4. **Never call `ticket_close` or `ticket_reopen`.** This ticket is shared across many agents; only the coordinator closes it, with an explicit path.
5. **Never run bare `cargo check` / `cargo build` / `cargo test`.** Always scope `-p <crate>`, always with `CARGO_TARGET_DIR=<this-ticket>/🎯️target`. The lock serializes concurrent checks — "Blocking waiting for file lock on build directory" is normal; wait, do not kill it. **A red workspace is not a red crate.**
6. **`bun` + `nx` only.** Permanent scripts live in `script.ts` at the relevant directory; `project.json`/`package.json` only call it. Never create another script file. Temporary one-shot scripts go in this ticket folder.
7. **Greenfield.** No compatibility layers, no legacy support, no deprecations, no migration scripts, no adapters. Delete and handcraft the replacement. Regenerate fixtures; never upcast them.
8. **Regions** (`//#region Name` / `//#endregion Name`, `#region`/`#endregion` in C#, `pub mod` in Rust) for all structure. Extend existing files; do not create new files outside the taxonomy shapes. No new test files — extend the existing ones. No example files — implement in the dependent parts.
9. `[DEBUG] ` prefix on every temporary log line, removed before the wave is reported done.
10. **Docstrings start with a unique fitting emoji.** No comments inside definitions.
11. **Never claim a test passed without running it.** Never claim a feature works without confirming runtime behaviour. Paste the real command and the real output into your report.

## ⛔ NEVER move a directory containing a `Cargo.toml` — it breaks cargo for the whole machine

Before relocating any directory, run `find <dir> -name Cargo.toml`. **If it returns anything, STOP and inventory instead of moving.**

A dangling `#[path]` mount is a *local* compile error in one crate. A dangling **workspace member** is a *global* failure: cargo refuses to load the workspace graph at all, so **every cargo command in every session on this machine fails before compiling anything**, with an error naming a plugin most of those sessions have never touched.

This happened. A W3 agent began relocating `✏️s/🔌️plugins/🖍️draw/🔄️fsm/`, which is a separate crate with **two** workspace-member entries (root `Cargo.toml:66-67`) and a path dependency (`🖍️draw/📦️packages/🦀️rust/Cargo.toml:27`). It correctly reverted on discovering the crate boundary, but during the window a peer session hit `failed to load manifest for workspace member` and reasonably diagnosed it as a permanent break. The agent's judgment was right; **the packet instructions were incomplete** — they carried this exception for `🧩️extensions` but did not generalize it to every crate-bearing directory.

The rule, generalized: a directory is **inventory-only, never moved**, if it contains a `Cargo.toml`. Relocating it is a workspace-topology change (members, path dependencies, crate names), which is a different and much larger operation than moving source files, and it is never in scope for a per-plugin packet.

## Evidence discipline — the coordinator got this wrong twice in one afternoon

Both times the pattern was identical: a **plausible story** stood in for a **measurement**, and both times the disproof was `stat -f '%Sm' <path>` — one command, available before the claim was made.

1. **"Orphaned debt from a closed ticket."** Two compile errors were traced to a rename whose ticket had closed, so they were declared unowned and a patch was offered to four sessions. A peer checked mtime: the file had been modified minutes earlier. It was live in-flight work.
2. **"That session may be gone."** A peer went quiet across several messages while a build stalled, so silence plus a stalled build was read as an interrupted session, and criteria were proposed for treating their file as abandoned. The file's mtime was three minutes old and the crate compiled green shortly after. They were heads-down, not gone.

**The specific error is treating absence of a reply as evidence.** It is only absence of evidence — a peer working intently is indistinguishable, over the channel, from a peer that has stopped. The two are told apart by the file, never by the silence.

### The general form (DKM's statement of it, the sharpest anyone reached)

> **A document written by a session is a derived artifact of that session, not a live predicate about it.**

SMO said this first about report files; every session has since been caught by some version of it. It applies to a peer's `📓️status.md` exactly as much as to a report directory — and the worked example is the one that matters here. UCAS's status doc says *"W1 (kernel): CODE COMPLETE. A1, A2, B1, B2, C1 all landed."* That is their own authoritative record, minutes old, written by the owner. It is still **not** a release, because another line of the same document says *"Signal APA when C1 unfreezes the file"* — the document defers to a signal that has not been sent.

**"Complete" and "released to you" are different claims, and only the owner can make the second.** APA holds W1 on exactly that basis. DKM independently holds its stdio handoff on the identical basis, despite having two of its three gates open — because two of three is not open.

**The rule (from DKM, and better than the version this ticket was reaching for):**

> If a file you need is owned and its owner is unresponsive, the answer is to work on something else and say so — **never to define a threshold past which taking it becomes acceptable.**

This is stronger than a carefully-calibrated threshold, because a threshold is something you can be argued down. Any criteria for "when it becomes acceptable to take another session's file" will eventually be satisfied by a session that is merely busy — the evidence available over a channel cannot distinguish busy from gone. Removing the procedure removes the failure mode.

**And the meta-rule:** when a conclusion would license an action you would otherwise consider off-limits — patching another session's file, adopting their claim, overriding their sequencing — that is exactly when the evidentiary bar goes **up**, not down. Before asserting anything about ownership or abandonment:
- `stat -f '%Sm' <path>` — how old is the file, really?
- `git log --oneline -3 -- <path>` — never `git status`; the repo auto-commits, so recent work reads as clean.
- State the measurement in the message. "Its mtime is 03:50, 14h old" is a claim a peer can check; "this looks orphaned" is not.

Corollary, learned the same way: **a result that says you are finished deserves more scrutiny than one that says you are not.** A breach-cache query reported 0 APA violations — total success — because the cache's top-level key is `breachs`, not `breaches`, and the wrong key returns an empty list rather than an error. It was caught only because 16 plugins still visibly carried the directories the rule counts.

## ⛔ After relocating ANYTHING, grep the WHOLE TREE for the old path — not just your plugin

A per-plugin agent verifies inside its own boundary. **A dangling reference to a moved file lives wherever the referrer is, which is often a different plugin.** So per-plugin structural verification is structurally incapable of catching this class.

It happened here: APA's `📐️cad` agent relocated `🖼️assets` into the artifact's `📚️examples`. `💠️lowpoly` reached across plugins into cad's old asset path with `include_str!("../../../../../../../📐️cad/🖼️assets/🎮️play/…")`. cad's own verification passed — correctly, the break wasn't in cad. lowpoly's verification had already passed before the move. Nobody's boundary contained the defect.

**Rule:** after any move, `grep -rn "<old-path-fragment>"` across the entire repo, and fix or report every hit. Relocation is not complete when the files are in the new place; it is complete when nothing points at the old one.

## ⛔ Fix broken paths by RE-RESOLUTION, never by pattern substitution

When a tree-wide relocation leaves stale `include_str!`/`include_bytes!` paths, the tempting fix is a depth rewrite (`7×../` → `3×../`). **That silently corrupts a large fraction of cases.** Measured on the real incident — 14 broken files under `✏️s/🔌️plugins`:

| class | count | correct fix |
|---|---:|---|
| depth-only, too deep | 6 | remove `../` — but two were *not* `📚️examples` paths (a framework font, a manifest), so a `📚️examples`-scoped rewrite misses them |
| **structurally different target** | **7** | insert `🏅️standards/🔖️1/🪆️subsets/✳️any/` mid-path — trinity ×5, space ×2 reach into *another artifact's or plugin's* examples dir |
| **needed to get DEEPER** | **1** | 7-up → 9-up; lowpoly's target moved further down |

**8 of 14 would have been silently broken by the substitution**, and one would have been broken in the opposite direction from the recipe. The correct method: locate the real file on disk, compute `os.path.relpath` from the referring file's directory, and only rewrite when the target is unambiguous — reporting the ambiguous ones rather than guessing.

## Repo gotchas that cost other sessions real time

12. **Derive crates keep two byte-identical copies**: `<module>/✨️derive/🦀️component.rs` **and** `<module>/✨️derive/📦️packages/🦀️rust/📦️glue.rs`. Cargo compiles the **glue** copy — editing only `component.rs` silently does nothing. Edit one, mirror it exactly, then `diff -q` the pair before reporting done. (`mcp__repo__file_integrate` has corrupted this mirroring before — mirror by hand.)
13. **`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` is dual-mounted** — into `semio-framework` as `io`, and into `semio-framework-os-kernel` as `os_io` (via `#[path]` in `💻️os/📦️packages/🦀️rust/📦️glue.rs`). One edit, two crates. This determines where the `Registrar` token can live.
14. **`🛂️manifest` names two different things.** `🧰️framework/🔨️modules/🛂️manifest/` is a framework module owned by UCAS. `✏️s/🔌️plugins/*/🛂️manifest/` are the per-plugin facet dirs APA deletes. **Never glob `**/🛂️manifest/**`** — always anchor to `✏️s/🔌️plugins/*/🛂️manifest/`.
15. **Two similarly-named host files.** `💻️os/🖥️host/🦀️component.rs` is APA's. `💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (the `IoRouter`) is UCAS's. Read the full path before editing.
16. **The semantic-vocabulary policy greps raw file content including comments.** Prose merely *naming* the three banned mutation identifiers trips it. Do not write them in docstrings or comments anywhere under `✏️s/`.
17. **Additive struct fields still break struct literals** (serde `default` affects (de)serialization, not Rust construction). After adding a field, `grep -rln "TypeName {"` across the whole workspace and fix or file every literal.
18. **Adding an enum variant is expensive** where the enum is matched exhaustively. Measure with `grep -rln "EnumName::"` first.

## Cross-session protocol (THREE sessions are in this tree)

| Session | Ticket | Channel |
|---|---|---|
| APA (us) | `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` #2549 | this session |
| UCAS | `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` #2548 | `uds:/tmp/cc-socks/30425.sock` |
| SMO | `SEMANTIC-MUTATIONS-OVERHAUL` #2545 | `uds:/tmp/cc-socks/86743.sock` |

**Negotiated and binding as of 2026-08-12 15:2x:**

- **UCAS ceded registration consolidation to APA.** Their `declare_artifact!`/`plugin!` macro plan is deleted; `ArtifactDeclaration` + `.artifact()` + `Registrar` + capability gating + sealing policy is APA's, whole. Their W2 will NOT convert stdio's registration — stdio's conversion is APA's, in one pass, after their roster freezes.
- **`🔌️plugin/🦀️component.rs` is frozen to APA until UCAS signals C1 landed.** Their C1 keeps only composition runtime (`Emit.child_emits`, child-store map, `dispatch_emit` group routing, `ArtifactChildren`, `DerivedArtifactSpec::Children`, WIT `resolve-artifact-link`). The `register_mesh_exporter`/`register_app_io` symbols that live in that file are APA's to remove **after** C1 unfreezes.
- **Repo-root `📜️script.ts` order: APA → UCAS-W6 → SMO.** All three agreed. Single writer at a time. **Announce on both channels immediately before starting to write it and immediately after stopping.** APA's five regions go at the END of the policy block (SMO's are ~5280–6050) and land in **report mode only** — they census, they never gate — until APA W5.
- **`🔣️taxonomy.json`: APA takes it before UCAS W6, but the `pluginChildDirs` flip to `["🎛️apps","🗿️artifacts"]` must land WITH OR AFTER the per-plugin cleanup reaches stdio.** Flipping early turns UCAS's W2 red on violations only APA can fix. The flip is the LAST thing APA does.
- **Plugin ordering: UCAS-W4 before APA, per plugin.** UCAS deletes plugin-local types and repoints them at stdio subsets; APA moves files between directories. Moving first invalidates their in-flight paths. Clearance signal: the existence of `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave4-reports/<plugin>-report.md`.
- **SMO holds the remaining plugin lanes**, actively rewriting app `🦀️component.rs`, `🎮️commands/**`, `📌️panels`, `⚙️engine` and per-plugin `📦️glue.rs` — the same files APA's Draft-lane work touches. Do not enter an SMO lane.

### ⚠️ Plugin clearance — ONE authority, never a copy

**The single source of truth is SMO's live predicate file:**
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`

**Read it at dispatch time. Do not trust any clearance list written in this ticket** — including in `📓️status.md`, `📓️w0-e-peer-state.md`, or an agent's own dispatch prompt. Those are snapshots, and clearance changes by the hour.

### How to read that file — ABSENCE MEANS FREE, not held

The predicate has four sections: RELEASED, HELD (lane in flight), HELD (between waves), and NOT SMO'S TO RELEASE. **A plugin that appears in none of them was never in SMO's scope at all, and is therefore free.** SMO confirmed this directly about `📐️cad`: *"cad is not in my held list at all, so it's been free the whole time."*

The gate is therefore: **proceed unless the plugin is explicitly HELD or explicitly listed as someone else's.** Not "proceed only if explicitly RELEASED."

This correction exists because the stricter reading cost a whole wave: five agents (`📐️cad`, `🏗️fem`, `🖍️draw`, `🌀️procedural`, `📋️forms`) correctly followed a dispatch prompt that said "if not RELEASED, stop", found their plugin absent from the ledger, and stopped without editing. They obeyed the instruction exactly; **the instruction was wrong**, and every one of them flagged the gap in their report rather than guessing — which is precisely the behaviour to keep. The failure mode to avoid is not caution, it is a gate that cannot distinguish "held by someone" from "never anyone's".

Known-free-by-absence at the time of writing: `📐️cad`, `🏗️fem`, `🖍️draw`, `🌀️procedural`, `📋️forms`. Re-derive rather than trusting this list.

This rule exists because it already went wrong: an earlier version of this file said "do not take note", `📓️status.md` said note was released, and `📓️w0-e-peer-state.md` ruled it LATER — three documents in one ticket, all disagreeing, because each froze a fact that had since moved. A W3 agent hit the contradiction and correctly refused to touch the plugin rather than guess. **It was right to stop, and the fault was the duplication, not the agent.**

This is the same defect all five sessions diagnosed independently this afternoon: a derived artifact encodes "what was true when it was written", while every consumer needs "what is true now". The fix is not to keep the copies better synchronised — it is to have no copies. If you need to know whether a plugin is free, read SMO's predicate file and, for anything UCAS has in flight, ask on their channel. If a dispatch prompt and that file disagree, **the file wins and the prompt is stale** — report the contradiction and stop, exactly as that agent did.

For UCAS's side, the same applies: their `📓️wave4-reports/<plugin>-report.md` carries an explicit `ucas-status: complete | partial` line. Read the line; do not infer freedom from the file's existence.
- **stdio is UCAS's** and is transiently red mid-rename. **Everything depends on stdio, so no plugin-side `cargo check` passes for anyone right now.** Treat red plugin crates as churn: retry `-p <crate>` up to 3× at 60s intervals, then record it under `## Concurrent-churn observations`, prove no error originates in your own boundary (grep the cargo output for your paths), and report `blocked-churn`. **Never "fix" another session's file.**

## Draft lane — SETTLED, read `📓️draft-lane-spec.md` before touching any app

APA turns the dead Draft lane (54 apps write `type Draft = NoDraft`) into real typed per-app draft state, replacing `thread_local!` scratch. SMO ruled on all three open questions on 2026-08-12; the full spec is `📓️draft-lane-spec.md` and is **binding**. Summary:

- **Shape**: `🎛️apps/<app>/📝️draft/🧬️schema/{📸️snapshot, 🔺️diff, 🧬️mutations/<emoji><slug>/{🦠️mutation,🔺️diff,↩️inverse}}` — the third sibling of `🎚️config` and `👥️presence`. `📝️draft` is added to `appChildDirs` (additive; **not** the deferred `pluginChildDirs` flip).
- **Every SMO policy rule applies automatically** — their `policyFindAllMutationsDirs` walks all of `✏️s` and does not exclude `🎛️apps`. **APA's draft facets gate SMO's ticket exit criteria.** They must pass the four mechanical gates from the outset. A facet that cannot be authored conformingly gets an EMPTY enum and no triad dirs, reported — never invented vocabulary.
- **Verbs**: closed table only. `create-stroke` + `insert-stroke-point{index}` (ordered-index law); **never mint `extend`** (synonym of insert/add). `bind`/`unbind` for a gizmo session; `move`/`drag`/`rotate`/`scale` for the transform itself — **never `update`** (reserved for an inseparable ≥2-field facet; measure the fields that actually move before choosing). Cancel → `unbind-*`. The pre-blessed domain verb `paint-stroke` is available for lowpoly only if a stroke is genuinely indivisible; default to the core decomposition.
- **Inverses required, no lane exemption.** Return `Vec::new()` where nothing is restorable — that is the sanctioned one-line answer, and it keeps draft diffs inside the law harness.
- **Per-app verb sets go to SMO for review before authoring.** No app is touched before its plugin is released by **both** SMO and UCAS.

## Report shape (every agent, every wave)

Append to your assigned report file in this ticket folder. Sections, in order: what changed (with file:line and grep anchors) · files touched (created/updated/removed) · verification commands run **with their real output pasted** · `## sharedFileRequests` (file, region, reason, patch file path) · `## Concurrent-churn observations` · honest pass/fail. A wave is not done until its report exists.
