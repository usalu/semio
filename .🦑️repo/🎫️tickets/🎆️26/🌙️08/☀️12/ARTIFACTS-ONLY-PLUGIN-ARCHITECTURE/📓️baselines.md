# Durable baselines — distinguish "new breakage" from "the tree was already like this"

## 📊 THE TREE-WIDE BASELINE (2026-08-12 ~22:10) — the first honest one

```
RUSTC_WRAPPER="" cargo check --workspace --all-targets --keep-going
→ 27 of 96 workspace members fail, 804 errors total
```

**The same command without `--keep-going`, same tree, same minute, reported 3. A 9× undercount** — and the short answer was the plausible-looking one.

**framework / os (8):** `semio-compose-rs`, `semio-framework-os`, `semio-framework-os-flow`, `semio-framework-os-infinite`, `semio-framework-os-kernel`, `semio-framework-os-renderer-wgpu`, `semio-framework-os-run`, `semio-framework-ui`

**plugins (19):** `architect block cad dag draw flow forms gis layout mathematical procedural process reasoning-mindmap remodel sequence sourcing space vcs writer`

**This is a snapshot of a tree six sessions are writing concurrently, not a fixed target.** Diff against it; do not treat it as ground truth an hour later. APA's own repair wave covers 11 of these crates and is running as it was taken.

**Provenance note that matters more than the number:** the session that produced it declined to claim its own four touched plugins were clean. It had independently verified only `block` (all errors tracing to stdio glue and framework store/spr/dsl, **zero** touching the artifact its exemplar dissolved); `flow`, `sequence` and `writer` rest on their agents' attribution, which it flagged as not yet re-derived. That distinction — *what I verified* versus *what an agent told me* — is what makes the rest of the number usable.

**A live warning attached to it:** `playbook`, `demonstrator` and `raster` are in APA's repair wave but **not** in the 27. Either they already compile or their failures were absorbed elsewhere. Agents must check whether a crate is actually broken before "repairing" it — the cost of a fix applied to a healthy crate is invisible and permanent.

## ⚠️⚠️ THREE INSTRUMENTS THAT RETURN A CONFIDENT, WELL-FORMED, WRONG ANSWER

Every cargo verification in this repo must defeat all three. They share a failure mode: **none of them looks broken.** Each returns a short, clean, plausible result rather than an error, so the reader has no cue to doubt it.

| invocation | blind to | the false conclusion it produces |
|---|---|---|
| `cargo check` without `--all-targets` | everything in tests/benches/examples — **exactly where a vocabulary rename lands** | "the crate compiles" |
| `cargo check` without `RUSTC_WRAPPER=""` | anything sccache serves while failing (`rustc-wrapper = "sccache"` is a **repo-wide default**, so you get it without opting in) | "0 errors" |
| `cargo check --workspace` without `--keep-going` | **every crate after the first failure** — cargo stops scheduling once something fails | "only 3 crates are red, and none are mine" |

**The mandatory form:**
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate> --all-targets
# or, workspace-wide:
RUSTC_WRAPPER="" cargo check --workspace --all-targets --keep-going
```
**And a green result requires the `Finished` line AND exit status 0.** "Grep found no errors" and "the build completed" are different claims; an aborted run satisfies the first.

### How the third one was caught, because the method generalises

A peer ran `cargo check --workspace --all-targets` to produce a per-crate breakage count. It reported **3 failing crates, no plugins**. That contradicted their own earlier spot-check showing `semio-s-plugin-forms` with 17 errors. **Rather than pick the number they preferred, they re-ran forms alone** — exit 101, `could not compile semio-s-plugin-forms`, errors in its *own* files. The workspace run had died in `compose-rs`/`kernel`/`ui` and never scheduled the other 20+ crates.

This was aimed squarely at APA's plan to run "one comprehensive sweep across all 24 migrated crates". Run as `--workspace` without `--keep-going`, that sweep would have stopped at `semio-compose-rs` — red for reasons belonging to neither ticket — and reported three crates, none of them APA's. **The correct reading of that output is "20 crates were never compiled"; the natural reading is "my 24 are clean."**

### The rule underneath all three

When two measurements disagree, **do not choose the more convenient one — re-measure the disputed item in isolation and attribute it.** Every instance today was resolved that way, and every one of them had a plausible wrong answer available for free.

## ⚠️⚠️ sccache produces a FALSE CLEAN — disable it for every verification run

`.cargo/config.toml:2` sets `rustc-wrapper = "sccache"` repo-wide. sccache is currently failing with `Operation not permitted`, and the failure mode is **a green result, not an error**. Every cargo verification in this repo must be run as:

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate> --all-targets
```

**A false clean is strictly worse than a red result**: red makes you look, green makes you stop looking. Found by SMO after it cost them a run; it had already tainted an APA sweep over 24 migrated plugins that was in flight when they reported it — those results were discarded and the sweep restarted rather than quoted.

### ⚠️ This is a DEFAULT, not a mistake — and it catches every subagent you spawn

The framing that matters (#2553): **you are not forgetting to unset something; you are failing to set something the repo silently sets for you.** `rustc-wrapper = "sccache"` is a repo-wide config key, so *the default state of this repo is sccache-on*. Anyone who simply runs `cargo check` — **especially every subagent any session spawns** — gets the wrapper without opting in and receives a fabricated green.

**Therefore: `RUSTC_WRAPPER=""` must be written into the agent's instructions, not merely used by the orchestrator.** An orchestrator that is careful itself and dispatches ten agents without the override has ten unreliable verifications. Every APA batch prompt from the declaration-migration wave onward carries it explicitly for this reason.

Combine with the other two rules and the full trust condition for any cargo evidence in this repo is:
1. `RUSTC_WRAPPER=""` — otherwise the result may be fabricated.
2. `--all-targets` — plain `check` skips `#[cfg(test)]`; four separate repo-wide breaks hid in that gap today, including one that let a ticket close believing it was finished.
3. **Confirm the run actually reached your targets.** A run that halts at an upstream crate never compiles yours, so "zero errors in X" from it means "cargo never got to X". SMO retracted their own claim on these grounds.

## ⚠️ Structural verification cannot see `include_str!`

APA's later W3 batches ran with cargo deliberately disabled and verified structurally — plugin root shape, every `#[path]` mount resolving on disk, dangling-reference greps, `cargo metadata` parsing. That was correct while the SDK was red and the disk was full, and it caught real breakage that a red-tree `cargo check` would have masked (one agent found 8 dangling `#[path]` mounts a *concurrent session* had left behind after a git rename).

But `🖨️raster` was later found broken by an `include_str!` whose target moved one level — a path only the compiler resolves. **Structural checks tell you the shape is right; only the compiler tells you the shape is true.** Any plugin verified structurally must be re-verified by compiler before it is called done.

Three sessions edit this tree concurrently and all three have lost time to that ambiguity. Record a baseline before changing anything, cite it afterwards. Provenance is established with `git log --oneline -- <path>` against the auto-commit flag counter (`🐙️ueli…🚩️<n>`), **never** with `git status` — the repo auto-commits, so recent work reads as clean.

## The disk fix bought a compute problem — and why the framing was wrong

Deleting the shared 428G root `target/` freed the volume and **simultaneously destroyed the shared build cache five sessions were implicitly amortising.** Every session now cold-rebuilds the same dependency graph into its own per-ticket `CARGO_TARGET_DIR`, in parallel. Measured consequence (DKM): **load average 149** on a machine whose healthy load is its core count — roughly 10× oversubscription, with five parallel `rustc` on `semio_s_plugin_stdio` alone.

Operational rules while this lasts:
- **Do not kill a slow build.** Check whether its target dir has *grown*, not whether files changed in the last minutes. DKM's exemplar ran 31 minutes, grew 441M → 943M, and touched nothing for 5 — starved, not stuck.
- **Timing-based judgements are void.** A long-running check is not evidence of a problem.
- **Parallel builds across sessions are negative-sum.** Five concurrent cold rebuilds finish later in aggregate than the same five serialized. APA cut an 8-crate sweep to 1 crate on this basis — 7 of the 8 answered no open question, and the one that mattered (`dag`) got its answer sooner.

**The framing error, recorded because it is more reusable than the incident.** APA escalated the disk decision to the user as *"root `target/` is stale — nothing writes to it"*, supported by a real measurement (zero files changed in two hours). That was true. But **"nothing writes to it" and "nothing reads from it" are different claims, and only the first was measured** — while a build cache's entire value is in being read. The cost was therefore invisible at decision time, and neither the user nor any session could have weighed it from the question as posed.

This is the same shape as the derived-artifact confusion running through this whole ticket: *the property that was easy to measure was substituted for the property that mattered.* When escalating a decision, state which property was actually measured, not just the conclusion it supports.

## Include-path repair (2026-08-12 ~19:20) — resolved, with the method that made it safe

A tree-wide relocation moved every artifact's `📚️examples` dir under `🏅️standards/🔖️1/🪆️subsets/✳️any/` and updated only some consumers, leaving ~30 crates uncompilable (`couldn't read …: No such file or directory (os error 2)` is a **hard error**). This is why every plugin gate run after ~18:08 came back red, independent of sccache and of the SDK window.

Repaired in two complementary passes — SMO cleared the 7-deep family (30 files); APA then fixed the 14 it did not cover. **Current state: 4393 include targets under `✏️s/🔌️plugins`, 4 unresolved, all in `🗄️stdio`'s brand-new `✳️table` subset (UCAS, mid-authoring — expected).**

### Two numbers that look contradictory and are both correct

APA measured `targets=4343 unresolved=0`; SMO measured 4 unresolved minutes later. Neither was wrong: **UCAS's `✳️table` subset added 50 include targets between the runs.** The tree moved, not the measurement. This is the cleanest illustration of the day's recurring hazard — **a count without its timestamp is not a fact**, and several hours were lost across five sessions to numbers that were true when taken and stale when quoted.

### Why the "obvious" fix would have caused damage

A depth substitution (`7×../` → `3×../`) was proposed and retracted. Of APA's 14 files: 6 depth-only (two of which weren't `📚️examples` paths at all — a framework font, a manifest — so a scoped rewrite misses them), **7 needing a structurally different target** (insert `🏅️standards/🔖️1/🪆️subsets/✳️any/` mid-path, for references into *another* artifact's or plugin's examples), and **1 needing to get deeper** (7-up → 9-up). **8 of 14 would have been silently corrupted**, one in the opposite direction from the recipe.

### A false-positive lesson from the E0753 sweep

SMO found an `//!` inner doc comment at file scope after code in trinity's `📦️glue.rs` (E0753 — the crate could not compile) and asked APA to grep the other 23 plugins. A naive "`//!` after code" scan reported **70 hits, all false**: `//!` inside a `mod { … }` block is legal and common in test harnesses. Re-run with brace-depth tracking so only file-scope occurrences count: **0 real sites**. The class is closed, not merely unobserved — but only because the check was made precise before its output was believed.

## Compiler verification — and a RETRACTED claim of my own

| crate | status | evidence |
|---|---|---|
| `semio-s-plugin-lowpoly` | **VERIFIED, 0 errors** | `RUSTC_WRAPPER=""` + `--all-targets`, `Finished` line present |
| `semio-s-plugin-note` | **VERIFIED, 0 errors** | same flags |
| `semio-s-plugin-gis` | ~~0 errors~~ **RETRACTED** | plain `cargo check`, sccache active, no `--all-targets` |
| `semio-s-plugin-fem` | ~~0 errors~~ **RETRACTED** | same |
| `semio-s-plugin-trinity` | ~~0 errors~~ **RETRACTED** | same |

### The retraction, in full, because it is the sharpest lesson in this ticket

gis, fem and trinity were reported as "compiler-verified at 0 errors". They were not. That sweep ran `cargo check -p <crate>` — **no `--all-targets`, and with sccache active** — which is *precisely* the configuration this very document warns produces a fabricated green. **I applied the standard to other sessions' numbers and quoted my own from a run that could not meet it.**

It surfaced only because a peer asked for the exact flags and timestamp before building on the result. That is the behaviour worth copying: they did not take a green light from a session that had been right about several other things.

Re-running trinity under proper flags **never reached trinity** — it died upstream:

```
✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs:6027,:6029,:6031
error: couldn't read …/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations/📄set-snapshot/{↩️inverse,🔺️diff,🦠️mutation}/🦀️component.rs
error: could not compile `semio-s-plugin-stdio` (lib)
```

Three dangling `#[path]` mounts left by another session's targeted removal of `✳️drawing`'s set-snapshot vocabulary (other artifacts still have theirs, so it was not a sweep). **Every plugin depends on stdio, so nothing plugin-side is verifiable until that glue is repaired.**

**The rule, now applied to myself:** a green result only counts if the run *reached* your crate **and** ran under flags that could have shown a failure. My original claim failed the second condition; the honest re-run failed the first.

Structural facts about gis/fem/trinity remain true and independently checkable — every `#[path]` resolves, every `include_str!` resolves, plugin roots are closed. **That tells you the shape is right; only the compiler tells you the shape is true**, and the compiler has not yet spoken for those three.

### `💠️lowpoly`'s two failures — one APA's, one pre-existing

1. **`include_str!` to the relocated `📚️examples`** — APA's, fixed by re-resolution (see the include-path section).
2. **`Value` vs `JsonValue` mismatch ×3 in lowpoly's json IO leaves — NOT APA's.** stdio replaced `serde_json::Value` with its own handcrafted `JsonValue` enum (key-order and lexeme preserving); lowpoly's consumer leaves still used `serde_json::from_value`/`to_value`.

**Provenance, measured not assumed:** the stdio snapshot defining `JsonValue` was last committed at **flag 484/485**, and lowpoly's leaf at **flag 477**, both with mtime `Aug 12 10:50` — all of which predate APA's first commit (**flag 492**, ~15:12). The breakage was in the tree hours before this ticket opened.

**Fixed anyway**, because lowpoly is APA-held and there was an unambiguous working exemplar: `🔱️trinity`'s json leaves bridge through json's own RFC8259 text codec (`parse_json_text` / `write_json_text` / `write_json_pretty`) rather than through `serde_json::Value`, and trinity compiles clean. Applied that same pattern to lowpoly's serializer and deserializer — a pattern copied from a verified sibling, not an invented conversion. stdio exposes no `JsonValue ↔ serde_json::Value` converter at all, which is deliberate per its own module docs ("No `serde_json::Value` anywhere in this file"), so the text bridge is the sanctioned route.

## Flag counter reference

| session | started at flag |
|---|---|
| SEMANTIC-MUTATIONS-OVERHAUL (SMO) | before 485 |
| UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (UCAS) | 491 |
| ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE (APA) | 492 |

A file whose last commit predates a session's start flag cannot have been changed by that session.

## `🦑️repo/…/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

| when | result |
|---|---|
| before APA touched it (2026-08-12 ~15:30) | **132 pass / 22 fail** / 838 expect() |
| after APA's two edits | **134 pass / 20 fail** / 840 expect() |

APA made exactly two changes here, both strictly reducing failures:
1. `pluginChildDirs` literal → `["🎛️apps"]`, paired with the taxonomy flip.
2. `artifactComponentDirs` literal → `["🧬️schema","⚙️engine","🚪️io"]`, fixing a **stale expectation left by a closed ticket**. Provenance: `🔣️taxonomy.json`'s last commit is flag **490**, predating both UCAS (491) and APA (492), so the three-entry value was already in the tree; the likely origin is `26/08/12/DERIVE-ARTIFACT-ANALYZERS-COMPOSERS-AND-BUILDERS`, closed today, which collapsed the artifact lifecycle dirs and updated taxonomy discovery but left this test expecting the old eight-entry list. Confirmed with UCAS before taking it.

**The remaining 20 failures are pre-existing and are NOT APA's.** They span: `dependency-boundary`, `ui scrollbar styling`, `micro-commit`, `playground static sites` (×2), `package boundary guards`, `commit`, `command budgets` (×2), `resolveCargoPackageName` (×2), `loadTaxonomy` (×2 remaining), `validateTaxonomy`, `discoverPackages` (×4), `computeWorkspaces`. Anyone reading this suite red should diff against 20, not against 0.

## Peer-reported baselines (their evidence, recorded here so APA does not re-derive it)

- **stdio** (UCAS): `2021 passed / 5 failed / 3 skipped`. The five failing facets' last commit predates their ticket.
- **workspace** (SMO, ~15:50): `cargo check --workspace` → **0 errors** across framework and all 33 plugins. This is the first point in this session where plugin-side verification became meaningful; before it, stdio was mid-rename and every plugin was transitively red.
- **raster** (SMO): `66 passed / 0 failed`.

## ⚠️ `semio-framework-plugin` is RED (observed 2026-08-12 ~17:50) — nothing plugin-side is verifiable

```
🔌️plugin/🦀️component.rs:5790:41  error[E0499]: cannot borrow `self.children` as mutable more than
                                  once at a time — borrowed in the previous loop iteration
🔌️plugin/🦀️component.rs:3152:38  error[E0560]: struct `TutorialBase` has no field named `document_dsl`
🔌️plugin/🦀️component.rs:3439:35  error[E0609]: no field `document_json` on `semio_framework::ExampleDefinition`
```

**Not APA's** — APA has modified no Rust framework file.

**Ownership: all three are UCAS's, and the file is live.** The renames landed at the definitions (`TutorialBase.document_dsl` → `artifact_dsl` at `🛂️manifest:1436`; `ExampleDefinition.document_json` → `artifact_json` at `🛂️manifest:2682`) and had not yet reached two `#[cfg(test)]` call sites in `🔌️plugin` (`:3152`, `:3439`). **Retry-and-wait is the protocol; do not patch.**

### A mistake worth keeping: "unowned" is a much stronger claim than "I can't tell who owns it"

APA initially diagnosed this pair as **orphaned debt from a closed ticket** (`26/08/10/RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE`) with no live owner, and broadcast that conclusion — plus an offer to patch it — to all four peer sessions. DKM disproved it in one step with the signal that actually settles ownership:

| file | mtime | meaning |
|---|---|---|
| `🛂️manifest/🦀️component.rs` | Aug 12 03:50 | rename landed at the definitions ~14h ago |
| `🔌️plugin/🦀️component.rs` | Aug 12 17:33 | **minutes ago — actively being edited** |

Both files are rows in UCAS's own hot-file table. This is one session's rename mid-propagation, and the `E0499` `self.children` borrow is that same session's composition round in the same file. The broadcast was retracted on all four channels before anyone acted on it.

The root error is instructive: APA reasoned from a *plausible origin story* rather than from evidence, having already told every session that `git status` is useless here because the repo auto-commits — and then not using the mtime check it had itself recommended. **Rule: check mtime before declaring anything unowned.** A patch applied on the strength of "nobody owns this" would have landed inside a live edit, which is precisely the failure the whole cross-session protocol exists to prevent.

Consequences, both material:
1. **Every plugin crate depends on this**, so no per-plugin `cargo check` proves anything until it is green. This is why W3 batches 3 and 4 run with cargo disabled and verify structurally instead — a red SDK makes per-agent cargo gating pure cost.
2. **The E0560/E0609 pair only surfaces under `--all-targets`** (plain `check` skips `#[cfg(test)]`). While it is red, no session can run the triad law harness — the correctness argument for both the mutations and composition tickets.

### `🧩️puzzle` is `blocked-churn`, not green and not broken

`cargo check -p semio-s-plugin-puzzle --all-targets` died on the dependency above before reaching puzzle. **Zero errors originate in any `🔌️plugins/🧩️puzzle` path** (grep-verified), but nothing is proven. Recorded here rather than reported as a pass, because a third session (INFERENCE-FAMILY) is waiting on this answer before building on puzzle, and an optimistic "probably fine" would have them build on an unverified base.

## ✅ RESOLVED — `semio-framework-plugin` is green again (2026-08-12 ~18:41)

UCAS finished propagating the `document`→`artifact` rename. Independently polled: `semio-framework-plugin (lib) generated 37 warnings; Finished dev profile in 4m 20s` — **0 errors**. Scoped plugin gates are trustworthy again for the first time in hours.

**Both directions of the outage window are void.** Any *red* result recorded while the SDK was broken says nothing about the crate under test — it failed upstream. Any *green* result recorded while the disk was full is equally worthless — those builds were failing on `No space left on device`, not passing. Re-run anything gated, baselined or released in that window, including anything concluded to be one's own regression.

Also expect **cold rebuilds** now that root `target/` is gone (deleted by another session's user decision; volume went 99%/16Gi → 49%/442Gi). A scoped check running past 10 minutes is normal. **Slow is not hung — do not kill it.**

### ⚠️ Do not "finish" the rename — `document_json` legitimately survives

`document_json` still appears ~16 times in `🔌️plugin/🦀️component.rs` and **every one is correct.** Only `ExampleDefinition`'s *field* was renamed to `artifact_json`. `ExampleSource` keeps its own vocabulary — its field, constructor, method, and `payload()` alias accessor — plus the conversion sites `ExampleDefinition { artifact_json: self.document_json }` (:3505) and `artifact_json: source.document_json.clone()` (:3521).

**A future grep for `document_json` in this file will look like unfinished migration work and is not.** Recorded here and in DKM's ticket from both sides, because this is precisely the shape of trap that gets "helpfully" cleaned up by a later pass.

## APA's own cargo baseline

`CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p <crate>` — recorded per crate inside each `📓️w3-<crate>-report.md` at step 0, before any edit, so every W3 packet carries its own before/after pair rather than relying on a global snapshot that goes stale within minutes.
