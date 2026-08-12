# Important — read first, every agent

Ticket: `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` ("EAM"), issue #2553, goal `🎯r2602🎯runningsketchpad`.
Approved plan: `/Users/ueli/.claude/plans/artifacts-must-not-have-magical-hanrahan.md`.
Design reports in this folder: `📓️design-a-dissolution.md`, `📓️design-b-machine.md`, `📓️design-c-state-classes.md`.

## The thesis (one paragraph)

An **artifact** is data and pure transforms: a `🧬️schema` (with `📸️snapshot`, `🔺️diff`, `🧬️mutations`, `💡️inferences`) and a `🚪️io` system. **It has no engine.** An **app** has an engine, and that engine **is a state machine** — controlled by the app, communicating bidirectionally, built on one core framework implementation. Transitions emit into four state classes: persisted-shared (artifacts, VCS-tracked mutations), persisted-local (config), ephemeral-local (draft), ephemeral-shared (presence, broadcast within a space). **One field may belong to several classes at once** — a camera is persisted locally *and* broadcast ephemerally — declared once, never copy-pasted. Today the repo inverts all of this: 95 artifact `⚙️engine` dirs hold 153,457 LOC while 50 of 53 app engine dirs sit empty; the statechart kernel is locked inside one plugin; and two of the four lanes are dead.

## What makes this ticket different from a cleanup

Three of the shapes we are removing are not merely tolerated — they are **mandated, and the things they exist to serve never shipped**:

1. `📜️script.ts:6426` **fails** an artifact that lacks an `⚙️engine`, and `:11433` requires ≥5 `register_language` calls inside it. The trait that mandate serves, `ArtifactEngine`, **never shipped** — `grep -rn "trait ArtifactEngine"` returns zero, and `🏪️store/🦀️component.rs:3092` says so in its own comment. 95 directories were built around a placeholder.
2. Two validators (`🧬️schema/🦀️component.rs:576` and its **TypeScript twin** at `📜️script.ts:8261`) force app config/presence facets to be **class-pure**, which is exactly what forbids the owner's overlap requirement.
3. `CommandOutcome<Diff>` — a struct with `persistent`/`shared_ui`/`local_ui`/`preview`/`effects` fields, i.e. the fan-out router we need — **already exists** at `📡️spr/🎮️command/🦀️component.rs:649` and is constructed exactly once, in its own test. `Mutation::state_class()` is likewise read exactly once, in a test. **The routing spine is built and nothing is plugged into it.**

So this is a **repeal and a reconnection** before it is a migration, and the repeal crosses two other tickets' boundaries.

## Hard rules (non-negotiable)

1. **No git-modifying commands, ever** — no `commit`, `stash`, `checkout`, `reset`, `rebase`, no worktrees. `isolation: "worktree"` is forbidden on every Agent/Workflow call. The tree is shared with a human and five other live agent sessions.
2. **The repo AUTO-COMMITS** (commits look like `🐙️ueli🎆️26🌙️MM☀️DD🚩️<n>`). `git status` is therefore **not** a churn detector: work that landed minutes ago shows clean, and a vanished edit was committed, not lost. Detect churn with `git log --oneline -5 -- <path>` and `stat -f '%Sm' <path>`. Never run a git command to "recover" anything.
3. **All temp files, logs, scratch and reports go inside this ticket folder.** Scratch logs are `.txt` or `.md`, **never `.log`** — `*.log` is repo-gitignored and silently drops out of `ticket_close`'s file list.
4. **Never call `ticket_close` or `ticket_reopen`.** Only the coordinator closes this ticket, **with an explicit path** — the path-less default closes whichever ticket is newest repo-wide, which is usually someone else's.
5. **Never run bare `cargo check`/`build`/`test`.** Always `-p <crate>`, always `CARGO_TARGET_DIR=<this-ticket>/🎯️target`. The lock serialises concurrent checks — "Blocking waiting for file lock on build directory" is normal; wait, do not kill it. **A red workspace is not a red crate.**
6. **For `semio-framework-plugin`, always `--all-targets`.** UCAS lost time to this today: the lib compiled green while the test target had 10 errors, and both of the day's repo-wide blockers hid there. Our `fn handle` sweep touches 57 impls and the test modules are full of them, so a lib-only check would tell us nothing.
7. **`bun` + `nx` only.** Permanent scripts live in `📜️script.ts` at the relevant directory; `project.json`/`package.json` only call it. Temporary one-shot scripts go in this ticket folder.
8. **Greenfield.** No compatibility layers, no legacy support, no deprecations, no migration scripts, no adapters. Delete and handcraft the replacement. **Regenerate fixtures; never upcast them.** (The `StateClass` alias table below is *not* a compatibility layer — the six spellings remain the real vocabulary; only their meaning is made precise.)
9. **Regions** (`//#region Name` / `//#endregion Name`) for all structure. Extend existing files; no new files outside the taxonomy shapes. No new test files — extend existing ones. No example files.
10. `[DEBUG] ` prefix on every temporary log line, removed before the wave is reported done.
11. **Docstrings start with a unique fitting emoji.** No comments inside definitions.
12. **Never claim a test passed without running it.** Paste the real command and its real output into your report. A Haiku scout re-runs your claimed commands independently — a discrepancy is a failed wave, not a rounding error.
13. **A bare identifier grep is a *search*, not a *census*.** "Referenced 70 times" and "70 things to fix" are different claims. Every count in this ticket that drives a decision was produced by brace-matching or a parser, and yours must be too.

## ⛔ NEVER move a directory containing a `Cargo.toml`

Before relocating any directory, run `find <dir> -name Cargo.toml`. **If it returns anything, STOP.**

A dangling `#[path]` mount is a *local* compile error in one crate. A dangling **workspace member** is a *global* failure: cargo refuses to load the workspace graph at all, so **every cargo command in every session on this machine fails**, with an error naming a plugin most of those sessions have never touched. This already happened in this tree, to an agent that began relocating the very `🔄️fsm` crate we are promoting.

**Therefore the `🔄️fsm` → `🔄️machine` promotion is create-and-delete, never `mv`:** create the new crates, handcraft the content, add the two new workspace members — at which point **both old and new exist simultaneously and nothing is broken** — then repoint draw, and only last delete the old member entries and subtree. Deleting a `Cargo.toml` directory is safe; moving one is not.

## Verified facts you do not need to re-derive (but must not contradict)

| Fact | Evidence |
|---|---|
| `associated_type_defaults` **works** on `nightly-2026-07-07` | probed and ran: `probe-atd.rs` / `probe-atd-result.txt` here, exit 0, zero warnings. So `type Machine: AppMachine<Self> = NoMachine<Self>` costs non-adopting apps **zero lines** |
| Adding a lane to `Emit` is **source-compatible for all 1,125 annotations** | arity histogram: 5 one-arg, 1,006 two-arg, 114 three-arg. The 1,006 already rely on the `DraftMutation` default — proof the trick works. `app_commands!` generates 2-arg and is untouched across all 50 invocation sites |
| …and breaks **exactly one** literal | brace-matched census of all 179 `Emit { … }` literals: 176 use `..Default::default()`; of the 3 without, 2 are false positives. The one real break is `dispatch_emit`'s destructure at `🔌️plugin:5763` |
| The presence **transport is complete except for a producer** | `PresencePeer.presence_pack` (`📡️spr/📡️wire:711`) → `ArtifactActorMsg::PresenceHeartbeat` (`🔄️sync:128`, handled at `:930` and `:1476`) → hub fan-out (`🌎️hub/📦️bin.rs:440`) → `ArtifactEvent::Presence` → `ViewModel.presence_peers_json`. **`grep PresenceHeartbeat` finds only the enum decl and its two consumer arms — nothing constructs it.** `presence_pack` is `Some(..)` in zero non-test sites |
| The **projection mechanism already exists** for artifacts | `XSnapshot` == the `persistent`-projection of `XArtifact`, verified mechanically: **OK=94, BAD=14** (the 14 are stdio codec envelopes). And `CadArtifact` already mixes 13 `persistent` + 22 `local_ui` + 10 `shared_ui` + 4 `preview` in ONE struct. **`ArtifactSchemaFields` never forbade mixing — only the app facets are class-pure, forced by the two validators** |
| The fan-out router **already exists, dormant** | `CommandOutcome<Diff> { persistent, shared_ui, local_ui, preview, effects }` at `📡️spr/🎮️command:649`, constructed once (its own test, `:1215`) |
| **`local-ui` is not local** | `SpaceBundle::config_pack_path` (`🏃️run:534`) is a thin alias for `artifact_pack_path` — config lands in the same `artifacts/` tree that "syncs the same way over `file://` or a semio_hub backbone". **The (persisted, local) quadrant has no storage location today** |
| The app engine slot is **already reserved and empty** | `⚙️engine` is in both `appChildDirs` and `appComponentDirs`; 53 dirs exist, **50 are empty**. No taxonomy edit is needed to give every app a machine |
| `🔣️taxonomy.json` is **NOT at the repo root** | it is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. The peer sessions' negotiated write-queue names a path that does not exist — **re-negotiate before touching it** |
| `💡️inferences` is **taxonomy-illegal today** | absent from `schemaChildDirs` while 72 dirs / ~15.8k LOC use it. Must be added; coordinate with IIF so we don't both add it |
| 87 of 89 `*Engine` structs are **pure deletions** | their fields are just the artifact and/or its snapshot — a durable duplicate of `ArtifactStore` state. Only `📸️remodel::ReconstructionEngine` and `📏️layout::LayoutEngine` survive |
| Lane liveness | `type Draft = NoDraft` in **56/56** apps; `grep presence_mutations` = **0**; but **53 of 56** apps declare a real (non-`NoPresence`) presence type. The lanes are declared and starved, not unwanted |

## The two-axis state model (workstream C)

The four quadrants are durability × scope. The existing six-variant `StateClass` **stays as the annotation vocabulary** — 4,684 JSON `x-semio-state` and 4,441 GraphQL `@state` sites keep parsing verbatim — but gains a precise meaning via an alias table:

| Spelling | Quadrant | Note |
|---|---|---|
| `persistent` | (Persisted, Shared) | 3,252 uses |
| `local-ui` | (Persisted, Local) | 874 uses. **Today it conflates persisted-local and ephemeral-local** — that ambiguity is the bug |
| `shared-ui` | (Ephemeral, Shared) | 398 uses |
| `preview` | (Ephemeral, Local) | 77 uses, 40 on artifact facets. Promoted from vague "scratch" to the honest name for the **Draft lane** — which is why Draft is dead in 56/56 apps: it had no declaration vocabulary |
| `inferred` | **none** — provenance, not a quadrant | 81 uses; lives in the separate `💡️inferences` facet with its own registry |
| `effect` | **none** — a channel, not a field class | 1 use, and that one is a breach (see below) |

`Effect` and `Inferred` stay parseable so `parse_state_class_kebab` remains total, but return `None` from `.facet()`.

**⚠️ `preview` collides with `Lane::Preview`** (`📡️spr/📡️wire:24`), which is a *transport* lane — and the one presence heartbeats already ride. Two meanings, one word. Never write `Preview` unqualified in this ticket's code.

**Overlap is expressed by repetition, not a new syntax.** GraphQL `@state` becomes `repeatable` (spec since June 2018), so all 4,441 single-use sites stay valid and a two-class field simply writes the directive twice. `x-semio-state` becomes a union: scalar stays canonical for singletons, array is the multi form. `field_states()` returns `StateClasses` (a `u8` bitset) instead of `StateClass`. **The validator flips from `assert_eq!` to `assert!(contains)` — that single change unblocks the whole requirement, and it must be made in BOTH the Rust validator (`🧬️schema:576`) and its TypeScript twin (`📜️script.ts:8261`).**

**Presence becomes a generated projection, never a hand-written twin.** Precedent: `XSnapshot` is already the persistent-projection of `XArtifact` (94/108). The divergence this fixes is real and measured: of 187 presence properties across 39 apps, **139 (74%) are also declared in config**, 48 are presence-only, and cad has drifted outright — its presence invented `cameraPosition`/`cameraTarget`/`cameraFov`/`cameraZoom` where config says `camera`, and only 17 of its 22/30 `local-ui` fields overlap at all. A generated projection also kills the DKM violation for free: **32 of 40 presence files contain `self.clone()`** as their `apply`, because the framework's own `impl_whole_record_config!` macro emits whole-value replace as its blessed shape. A projection has no mutation type of its own — it reuses the config mutation's real field-level diff.

**Known breaches to fix in passing** (both found by this analysis, neither caught by any rule): `📰xml`'s snapshot declares `"x-semio-state": "identity"`, which is not a `StateClass` at all; `📜️imperative`'s artifact declares `"effect"` on a field.

## Hot-file ownership (binding)

| File / subtree | Owner | Everyone else |
|---|---|---|
| `🧰️framework/🔨️modules/🔄️machine/**` | ours, greenfield | — |
| `🔌️plugin/🦀️component.rs` — `ChildEmit`, `ArtifactChildren`, `dispatch_group`, `SpaceMember` | **UCAS** — ping them, never fix | read-only |
| `🔌️plugin/🦀️component.rs` — everything else | **released to APA**; we queue behind them | negotiate before entering |
| `✏️s/🔌️plugins/🗄️stdio/**` | **UCAS**, roster NOT frozen | do not enter without their explicit signal |
| repo-root `📜️script.ts`, the real `🔣️taxonomy.json` | write-queue, announce before and after | read-only otherwise |
| `💻️os/🔨️modules/🛢️db/**` (`semio-framework-os-kernel-db`, ~53 pre-existing errors) | another session | do not "fix"; do not let it mask our failures |

## Cross-session protocol — SIX sessions share this tree

| Session | Ticket | Note |
|---|---|---|
| EAM (us) | `ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` #2553 | this session |
| UCAS | `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` #2548 | owns composition + the stdio roster |
| APA | `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` #2549 | owns registration (`ArtifactDeclaration`/`Registrar`) + the draft lane |
| SMO | `SEMANTIC-MUTATIONS-OVERHAUL` #2545 | owns the verb vocabulary |
| IIF | `INTRODUCE-INFERENCE-SCHEMA-FAMILY…` | owns the `💡️inference` fan-out |
| DKM | `DISSOLVE-KERNELS-AND-MODULES…` #2550 | owns kernel dissolution; its tier table binds us |

- **Re-resolve addresses with `ListAgents` before every handshake. Names drift; never cache one across waves.**
- **Never infer a live session's state from files it wrote.** That oracle has been proven wrong in both directions in this tree, repeatedly. Ask the session.
- **SMO's plugin clearance is a live file**, `../SEMANTIC-MUTATIONS-OVERHAUL/📓️plugin-release-status.md`. Read it at dispatch time. **Absence from it means FREE, not held** — the stricter reading already cost another ticket a whole wave.
- **Never "fix" another session's file.** On a red compile outside your boundary: retry the scoped check 3× at 60s, grep the output to prove zero errors originate in your own paths, record under `## Concurrent-churn observations`, report `blocked-churn`, and stop.
- **We build ON peers' primitives, never a parallel mechanism.** APA owns registration — our D1 slice defers to them. SMO owns verbs — every mutation we author is submitted before authoring. DKM's tier table settles every "where does this code go" question.

## The D0–D6 classification procedure (workstream A)

One Rust item at a time. **Strictly ordered, first match wins.** Every predicate is a regex over the signature, the enclosing `//#region` name, or the body — no judgment calls.

`SIG` = signature to the opening brace · `BODY` = brace-matched body · `REGION` = outermost enclosing `//#region` label · `SNAP` = `\b[A-Z][A-Za-z0-9]*Snapshot\b` (verified total: 98 distinct types, no counterexample) · `BYTES` = `&\[u8\]|Vec<u8>|&str|String|serde_json::Value|&Path|PathBuf`.

- **D0 Test** — `REGION` ends `Tests`, or `#[cfg(test)]`, or name ends `_law`/`_test` → travels with its subject, **never budgeted**.
- **D1 Registration** — name starts `register`, or a register/registry `REGION`, or body mentions `register_artifact_schema|register_pilot_language|register_document_codec|ArtifactDeclaration|Registrar` → APA's mechanism. **BLOCKED: `ArtifactDeclaration` has zero hits repo-wide.** Leave in place, report deferred.
- **D2 IO** — `SNAP` in `SIG` **and** `BYTES` in `SIG` → `🚪️io/📥️import/🧩️deserializers/` if `SNAP` is in the return, `🚪️io/📤️export/🧵️serializers/` if in the args.
- **D3 Mutation** — `&mut …Snapshot`, or `SNAP` in both args and return → a `🧬️mutations/<slug>/{🦠️mutation,🔺️diff,↩️inverse}` triad, verb from SMO's `APPROVED_VERBS`.
  - **D3a** if the return is a struct literal naming ≥80% of the snapshot's fields, or `..base.clone()`, it is a whole-document replace — **not a mutation at all**. Delete it; `ArtifactStore::reset` is the sanctioned path (locked DKM decision).
- **D4 Inference** — `SNAP` in args, not in return, no `&mut` → `🧬️schema/💡️inferences/` as an `InferredField<P>` with a real `DepHash` chain.
- **D5 Behavioral** — `&mut self` on a non-Snapshot receiver, or `thread_local!|static mut|OnceLock|LazyLock` → the **app's** `⚙️engine`.
  - **D5a (disposes of 87 of 89 cases)** if the receiver is a `*Engine` whose fields are just the artifact and/or its snapshot → **delete the struct and every method on it.**
  - **D5b** if the `&mut self` state is a BVH, halfedge table, tessellation buffer, brep arena or font context rebuilt from the snapshot each call → a **local variable** (DKM tier (d)).
- **D6 Pure algorithm** — everything else (verified decoupled: of 4,011 functions reaching D6, only 27 mention a `*Snapshot` in their body).
  - **D6a** → `🧰️framework/🔨️modules/<domain>/⚙️engine/` if called from ≥2 plugins or the signature mentions only primitives/framework geometry.
  - **D6b** → `✏️s/🔨️modules/<name>/⚙️engine/` otherwise.
  - **D6c (codec clusters)** a D6 fn reachable **only** from D2 entry points in its own dir and referenced by no other plugin is reclassified **D2** and travels with its codec. Keeps `🗜️deflate`/`📷️png`/`📷️jpg`/`🎞️gif`/`🖼️tiff`/`🎥️mp4` whole.
  - **D6d** `📐️cad` TS: `🎬️actions`/`🎰️stately` → D5; `📐️geometry`/`🅱️brepjs`/`🌐️spatial` → D6a; `🔎️query` → D4.
  - **D6e** `🔋️energy`'s 51 non-emoji subdirs violate `requireEmojiPrefixWithVs16` — rename on relocation.

**The doctrine tension, settled:** DKM tier (e) sanctions "an engine crate" for pure algorithms; the owner says artifacts have no engine. Both hold, because they quantify over different things — the owner's rule is about the **artifact taxonomy leaf**, tier (e) is about a **crate**. `taxonomyLeafParentDirs` already lists `⚙️engine` globally, which is why `🧰️framework/🔨️modules/◻2d/⚙️engine/` is legal. Pure algorithms stay legal and **move up one level**; only `subsetChildDirs`/`artifactChildDirs` shrink.

## The ratchet sequencing (workstream A) — the rule that must not be broken

Only `priority: "high"` throws. So report mode and gate mode are the same rule at two priorities; no feature flag.

1. **P0** census rule at `"low"` — counts 95, gates nothing.
2. **P1 repeal** — vocabulary edits + delete `policyArtifactEnginePresenceBreaches` (`:6418`, call `:7066`) and `policySubsetEnginePresenceBreaches` (`:5626`, call `:5807`) + three stale test-assertion fixes. **Must precede every packet:** a packet that deletes an engine dir *before* the repeal creates a high-priority breach and turns the gate red for all six sessions.
3. **P2** packets — the breach count is the burn-down chart, 95 → 0.
4. **P3** raise to `"high"` — only after zero, **verified by counting directories on disk**. Never trust the breach cache: its top-level key is `breachs`, not `breaches`, so a query on the wrong key returns `[]`, which reads as total success.

## Report shape (every agent, every wave)

Append to your assigned report file in this folder. Sections, in order: **what changed** (file:line + grep anchors) · **files touched** (created/updated/removed) · **verification commands run, with real output pasted** · `## sharedFileRequests` (file, region, reason, patch path) · `## Concurrent-churn observations` · honest pass/fail. A wave is not done until its report exists.
