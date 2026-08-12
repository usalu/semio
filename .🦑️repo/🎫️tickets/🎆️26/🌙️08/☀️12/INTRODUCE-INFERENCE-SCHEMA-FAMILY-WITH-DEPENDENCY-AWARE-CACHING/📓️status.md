# Status

## ✅ DISK RESOLVED (was 100% full)

The user approved deleting the repo-root `target/` (428 G of regenerable build cache); the parent
session executed `rm -rf target/`. **99% used / 16 GiB avail → 49% used / 442 GiB avail.** A few
subpaths threw "Directory not empty" mid-delete because something was writing concurrently, but the
space recovery confirms the bulk succeeded. Verified independently here: `df` reports 442 Gi free.

Cargo has room again — but **re-verify before trusting any build result**, and see the still-open
framework-plugin blocker below. The historical analysis is kept below for the record.

## 🟢 `semio-framework-plugin` — RE-VERIFIED GREEN. The "still red" report was itself stale.

The trinity worker reported framework-plugin as **still red with a NEW error shape (E0308/E0599)**, concluding UCAS was mid-propagation and that the green record here was stale. **I re-verified live before amending anything, and it does not reproduce:**

```
cargo check -p semio-framework-plugin               → 0 errors, Finished in 1.05s
cargo check -p semio-framework-plugin --all-targets → 0 errors, Finished in 1.98s
```

Both targets, zero errors. I ran `--all-targets` specifically because the lib-only blind spot has bitten this session repeatedly and E0308/E0599 are exactly the shape that hides in test code — it is green there too.

**So the green record stands and was NOT amended.** The worker saw a genuine transient mid-propagation state that UCAS has since finished. Its report was accurate when written and stale by the time it reached me.

*Live-predicate lesson, 4th instance today — and the sharpest one:* this time the stale derived artifact was **our own worker's report**, relayed to me as current fact with an explicit instruction to correct this file. Had I applied that correction on trust, I would have written a false "still red" record and held trinity on a blocker that no longer existed. **Verify before amending a record, even when the amendment comes from your own agent.**

## (historical) GATE BLOCKER CLEARED — `semio-framework-plugin` GREEN

Polled directly (`cargo check -p semio-framework-plugin`, ticket `CARGO_TARGET_DIR`):

```
warning: `semio-framework-plugin` (lib) generated 37 warnings
    Finished `dev` profile [unoptimized] target(s) in 4m 20s
```

**0 errors.** UCAS finished propagating the document→artifact field rename; all three errors
(E0499 / E0560 / E0609) are gone. Scoped plugin gates are now meaningful again — a red
`cargo check -p semio-s-plugin-*` from this point forward is real and probably ours.
Raw output: `scratch-fwplugin-poll.txt`. Both running workers were notified directly.

## (historical) GATE BLOCKER — `semio-framework-plugin` was red repo-wide, and it was NOT OURS

3 errors: E0499 `self.children` double-mutable-borrow, E0560 `TutorialBase` missing `document_dsl`,
E0609 `ExampleDefinition` missing `document_json`. **UCAS is mid-propagating a document→artifact field
rename** — confirmed live by DKM via file mtime. Explicit peer instruction: **nobody patches this,
including us.** Retry and wait.

**⏳ COLD-REBUILD TAX (new, expect this all evening):** deleting root `target/` means every crate now
rebuilds from scratch. A scoped `cargo check -p semio-framework-plugin` **exceeded 10 minutes without
finishing** on the first post-deletion run. Gates are not fast right now and a slow build is not a hung
one — do not kill them, do not interpret slowness as breakage, and budget accordingly. Poll in the
background rather than blocking on a foreground run.

Every plugin crate depends on it, so every scoped `cargo check -p semio-s-plugin-*` can be red for
this reason alone. Our own run confirms it verbatim: `error: could not compile 'semio-framework-plugin'
(lib) due to 1 previous error; 38 warnings emitted`. **Do not self-blame, and do not blame APA's
relocation, when a red trace leads here — check against this first.** Workers are instructed to
distinguish "red because of framework-plugin" from "red because of my edit" by reading which crate the
errors name, and to report honestly if they never got a clean gate rather than fake one.

## (historical) MACHINE-WIDE BLOCKER — DISK WAS 100% FULL

```
/dev/disk3s5   926Gi   860Gi   5.2Gi   100%   /System/Volumes/Data
```

Every cargo invocation now dies with `No space left on device (os error 28)` — confirmed live in our
own `scratch-puzzle-verify-for-apa.txt` (failed to link `crc32fast`, failed to write `rmeta` for
`libm`/`arrayvec`/`smallvec`/`serde_core`/`log`/`zerofrom`/`bytemuck`/`simd-adler32`, and sccache
failing to write `deps.d` under `/var/folders/...`). This is not our ticket, not contention, not churn.

**Cause, measured:**

| path | size |
|---|---|
| **`target/` (repo root)** | **428 G** |
| `.🦑️repo/` (all tickets) | 31 G |
| ↳ `SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS` ticket (a 6th session) | 13 G — `🎯️target-w3-stdio` 5.5G, `-w3-cad` 3.8G, `-w3-en1990` 2.4G, `-w1-harness` 809M, `-w3-stdio-check` 642M |
| our ticket folder | 124 K |

The repo-root `target/` is **93% of the problem** and is pure regenerable build cache. Repo policy is
that everyone uses a per-ticket `CARGO_TARGET_DIR`, so root `target/` is almost certainly stale
accumulation nobody is writing to — but it is shared state and 5+ sessions have builds in flight.

**NOT deleting it unilaterally.** Escalated to the parent session for a user decision. Deleting is a
one-line fix that frees ~428 G instantly; the cost is a full cold rebuild for everyone. Needs an owner,
not a coordinator guessing.

**⏳ IT IS GETTING WORSE, FAST.** Free space measured twice, minutes apart: **5.2 GiB → 2.7 GiB**, with
~10 cargo processes still running and still writing. At this rate the machine runs out entirely within
the hour, at which point failures stop being confined to builds. This is now time-critical, not merely
blocking.

**Our own ticket contributes nothing** — our `🎯️target` does not even exist on disk (the failed builds
never materialised one) and the entire ticket folder is 128 K. There is nothing for us to clean up;
the space has to come from root `target/` or another session's dirs.

All three reachable peer sessions have been alerted with the measurements and the "do not delete
unilaterally" caveat, plus the warning that all cargo evidence in this window is worthless.

**Until it is resolved, treat ALL cargo output repo-wide as meaningless** — including any red result
peers report. Nothing can be verified by building.

## Upstream compile blocker — `semio-framework-plugin` is RED, and it is NOT OURS

3 errors: E0499 `self.children` double-mutable-borrow (UCAS's composition round), E0560 `TutorialBase`
missing `document_dsl`, E0609 `ExampleDefinition` missing `document_json` (a stale rename call-site
pair; the symbol lives in `semio-framework`, not the plugin SDK).

**Every plugin crate depends on this crate**, so every scoped `cargo check -p semio-s-plugin-*` in the
repo is red for this reason alone. Our own plain check confirms it verbatim:
`error: could not compile 'semio-framework-plugin' (lib) due to 1 previous error; 38 warnings emitted`.

Our only historical touch to `🔌️plugin/🦀️component.rs` is the session-1 `ArtifactInferrer` trait
addition — compiling at the time, unrelated to all three symbols, and that file is UCAS-frozen to us.
**Do not self-blame or blame APA's relocation when a red check trace leads here. Check against this
first.** (Recorded in `scratch-puzzle-baseline-check.txt`.)

Coordinator: Opus 5 session. Executors: Sonnet 5 agents. Explorers: Haiku 4.5 agents. Plan authored by a Fable session at `/Users/ueli/.claude/plans/finish-introduce-inference-schema-family-iridescent-sprout.md`.
**Only the coordinator edits this file.** Agents append to their own report files.

## 🔴 ACTIVE GATE BLOCKER — flag 495's `📚️examples` move left 32 stale `include_str!` paths across 24 plugins

**This is fresh peer churn from 18:08 today, NOT ours, NOT pre-existing. It is why our fem gate came back red.**

Commit **`fd01661f06`** (flag 495, 08-12 18:08, **1551 files**) relocates every artifact's examples dir:
`🗿️artifacts/<a>/📚️examples/` → `🗿️artifacts/<a>/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`
(git records them as `R100` pure renames). Almost certainly the SUBSET-CONFORMANCE session — that commit also carries `.cursor/plans/subset_conformance_roundtrips_c57a3e1a.plan.md`.

The migration **updated some** `📸️snapshot/📝️text/🦀️component.rs` include paths (cad, norm×4 confirmed in the commit) but **missed 32 files across 24 plugins**. Those still read:

```rust
pub const FEM2D_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
```

7 `../` resolved correctly to the OLD artifact-level location. The correct depth is now **3** (`../../../📚️examples/…`, landing at `🪆️subsets/✳️any/`) — verified: the 3-up path resolves on disk, and rustc's own `help:` suggests exactly that.

**Affected (32 files / 24 plugins):** block×3, puzzle×3, trinity×2, fem×2, gis×2, procedural×2, and one each for note, draw, imperative, playbook, layout, forms, reasoning, lowpoly, process, architect, sequence, demonstrator, shooting, animate, vcs, flow, mathematical, writer.

**Consequence: those 24 plugin crates cannot compile at all** — `error: couldn't read …: No such file or directory (os error 2)`, which is a hard error, not a warning. This blocks the gate for **trinity and fem (ours), and puzzle**.

**We are NOT fixing it** — most files sit in peer lanes and it is the tail of another session's in-flight migration. Textbook "never fix another session's file". Broadcast to peers; retry our gates once they finish.

### ✅✅✅ FULLY RESOLVED repo-wide — but FILESYSTEM-verified, NOT compiler-verified

**Independent coordinator sweep** over every `include_str!`/`include_bytes!` under `✏️s/🔌️plugins`, resolving each target against disk: **0 unresolved.** APA reports the same from their side (4343 targets, 0 unresolved). Two independent methods agreeing.

**`🔱️trinity` specifically is clear**, contradicting an intermediate DKM audit that listed 5 trinity paths as still broken (`🎛️apps/🔌️jack/🦀️component.rs:40,41`, `♻️rewrite/…:52,759,969`) and warned they were a harder cross-artifact class needing target re-resolution rather than depth adjustment. Verified directly: all three surviving `include_str!` sites now read `../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/…` and **all resolve**. APA fixed them between DKM's audit and this check.

*Live-predicate lesson, third instance today:* DKM's audit was a **derived artifact** and went stale within minutes; the filesystem was the live predicate. Same class as trusting a peer's report file over the tree itself. Check the disk.

**⚠️ Do NOT treat any of this as a pass.** Both APA and this sweep verified only that *files exist at the referenced paths*. Nothing has been compiler-verified. **A gate is green only when `cargo check --all-targets` says so.** Gates re-running: `scratch-p1-fem-gate2.txt`, `scratch-p1-trinity-gate.txt`.

### 🔻 SELF-CORRECTION — the 19:13 fem fix was OUR worker, not APA. I made the same attribution error twice.

Earlier in this file I wrote "APA's fix has landed — verified from disk at 19:13". **That was wrong, and wrong by exactly the reasoning I had just banned one section above.** I saw two fem files change at 19:13, knew APA had announced a fix wave, and inferred authorship from timing. Textbook co-location inference.

**Content evidence says otherwise:** our own fem worker's report (`📓️p1-fem-report.md` §"Out-of-scope blocker fix", files-touched list) explicitly claims both edits, at `◻2d/…/📸️snapshot/📝️text/🦀️component.rs:15` and the `🧊️3d` twin. Both files are still uncommitted (`git status` → `M`), mtimes 19:13:22 / 19:13:37, matching that worker's run window. Authorship testimony beats timing every time.

So I have now made this error twice in one session — once assigning the examples move to SUBSET-CONFORMANCE on commit co-location, once assigning our own worker's edit to APA on mtime. The rule above is not theoretical; it is written from repeated failure.

**What the worker did, and how bad it is:** it corrected `../` depth 7→3 in two `📸️snapshot/📝️text/` files. That facet was outside its assignment (`💡️inferences/` only) but inside the `🏗️fem` plugin boundary I explicitly set for it, and the breakage was blocking *any* gate signal at all for the whole crate. It documented the deviation prominently, kept it to one line per file, and verified the target both ways. **The end state is correct** and identical to what APA landed across the other 60 targets. Reported to APA regardless, since fem is their claimed lane and the files sit uncommitted in their working set.

**Worth recording — the worker's churn-detection reasoning was sound but its conclusion was wrong.** It argued: "reproduced the identical failure across two independent full compiles minutes apart, so this is a pre-existing template bug, not in-flight churn." It even *noticed* flag 495 was newer than the HEAD at its session start, and dismissed it. In fact it was fresh churn — APA was mid-sweep across 62 targets and simply had not reached fem yet. **Reproducibility does not distinguish "stable pre-existing bug" from "in-flight migration that hasn't reached this file yet."** Both are stable across minutes. The distinguishing evidence is the rename records, not repeated observation.

### (historical) 19:13 fem fix — originally, and incorrectly, attributed to APA

Confirmed directly, not from a claim:
- `🏗️fem/◻2d/…/📸️snapshot/📝️text/🦀️component.rs:15` now reads `include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")` — and that path **resolves on disk** (checked every `include_str!` target in the file; all OK).
- **Zero** files repo-wide still carry the 7-up pattern (was 32).
- File mtime 19:13, consistent with APA's fix wave.

Both blocked gates re-launched: `scratch-p1-fem-gate2.txt`, and the trinity gate was already in flight through its dependency phase when the fix landed, so it will pick it up.

### ✅ APA HAS TAKEN IT — and corrected me on two counts

**Scope is bigger and messier than I reported.** APA's full audit: **62 unresolved `include_str!`/`include_bytes!` targets, not 32**, across **several different wrong-depth patterns**, not one. Example: `🕸️dag`'s snapshot-text module is **4 deep where 3 is correct**. So my proposed mechanical `7→3` substitution **would have missed dag and could have broken others.** APA is re-resolving each path against its real on-disk target and verifying with the compiler instead of pattern-substituting. Correct call — my fix recipe was wrong, and confidently so, because I generalised from the two fem files I actually looked at.

**APA is fixing every plugin they have an active claim on** (their 24 migrated today), and will **not** touch SMO-held plugins (architect, animate, process, reasoning, writer, vcs, flow, sequence) — asking SMO about those separately. Any genuinely ambiguous target gets reported rather than guessed.

**`🏗️fem` IS covered — confirmed from evidence, not assumed.** APA's ticket folder contains `📓️w3-semio-s-plugin-fem-report.md`, and their `📓️status.md` lists `🏗️fem` under **In flight (7)** alongside mathematical, procedural, shooting, layout, norm, remodel. Their status file also records that a concurrent session had git-renamed all 8 of fem's compute files into the artifact engines while leaving `📦️glue.rs` pointing at the old paths — 8 dangling `#[path]` mounts APA repaired. **So both our blocked plugins (fem, trinity) are APA-owned and APA-covered.** No separate owner needed; nothing to escalate.

### ⚠️ ATTRIBUTION RULE (standing, applies for the rest of this ticket)

**Content evidence attributes; timing evidence does not.**
- ✅ **Use**: `R100` rename records, the actual contents of a diff, which symbols a change touches, which files a peer's own report claims. These survive auto-commit bundling.
- ❌ **Never use**: which flag a change landed in, what else is in that commit, or how close two changes are in time. The repo auto-commits the **whole tree on a timer**, so every flag bundles every session's in-flight work — co-location carries **zero** attribution signal.

Apply this before attributing any future red build to a peer. I got it wrong once already today (below) and fed the bad inference to three peer sessions before retracting it.

### ⚠️ METHODOLOGY CORRECTION — the mistake that produced the rule above

**Do not cite an auto-commit flag as proof of authorship.** This repo auto-commits the **whole tree on a timer**, so any single flag bundles every session's in-flight work at that moment. My "flag 495 also carries SUBSET-CONFORMANCE's plan file, therefore the examples move is theirs" was **invalid reasoning** — APA's own fem-relocation agents are in that very same commit.

The valid evidence was the **`R100` pure-rename records** for the examples dirs themselves — content evidence about what the change *is*, not co-location evidence about when it was committed. Correct phrasing is "almost certainly X, per the rename records", never "the commit proves X".

This is the same failure mode as the memory note *Live Predicate, Not Derived Artifact*: a commit, like a written report, is a derived artifact. Timing co-location inside an auto-commit carries **no** attribution signal whatsoever.

**Attribution for our own work is clean:** the fem failure is in `📸️snapshot/📝️text/`, a facet our worker never opened. Our worker touched only `💡️inferences/📝️text/🛰️component.proto` (new, not yet compiled) and the two `💡️inferences/📦bounds/🦀️component.rs` test blocks. **Zero of our edits appear in the error output.**

## ⚖️ EVIDENCE HYGIENE RULE (DKM, adopted repo-wide — applies to us)

**Greens from the disk-outage window (~16:00–18:13 today) are exactly as suspect as reds.** A "green"
may have been a build that silently never reached its target rather than one that passed. Re-run before
trusting either.

**Audit of our own outage-window evidence — no false greens found:**

| artifact | verdict |
|---|---|
| `scratch-p0-d2-testbuild-raw.txt` (16:39) | **Void, visibly failed** — contains only `Blocking waiting for file lock` + `failed to link proc-macro2 build script` / `No space left on device (os error 28)`. Never reproduced the 144 errors it was sent to collect. D2 re-tasked. |
| `scratch-puzzle-baseline-check.txt` (16:38) | Void — red, but from the upstream `semio-framework-plugin` breakage, not puzzle. |
| `scratch-puzzle-verify-for-apa.txt` (16:40) | Void — died on `No space left on device`. Never sent to APA (correctly). |
| P0-A1/A2/A3/A4 audits | **Unaffected** — all four were forbidden from running cargo, so their evidence is filesystem-only and remains valid. |

Every piece of build evidence we hold from that window is *visibly* failed, not silently green. Both
diagnostics (D1 stdio, D2 os-kernel) were re-tasked with fresh instructions and the explicit warning
that a green from the old window proves nothing.

## Peer session map (corrected, authoritative)

| Session name | Ticket | Short |
|---|---|---|
| semio-9f | SEMANTIC-MUTATIONS-OVERHAUL #2545 | SMO |
| semio-52 | ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE #2549 | APA |
| semio-b2 | UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM #2548 | UCAS |
| (5th, uds:/tmp/cc-socks/53352.sock) | DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS #2550 | DKM |
| this session | INTRODUCE-INFERENCE-SCHEMA-FAMILY #2546 | IIF (us) |

**`📜️script.ts` / `🔣️taxonomy.json` write-slot queue — UPDATED:**
**APA IS DONE AND OUT.** New queue: UCAS-W6 → SMO → **US (position 3)** → DKM. APA's write landed 5 report-mode policy rules / 1727 breaches, all `priority: "medium"`, nothing gates. DKM is waiting on us: their `policyEngineRepEscapeBreaches` / `policyEngineConsumptionOutsideFacetBreaches` name the inference facet as the sanctioned home for derived compute.

**`🔣️taxonomy.json` `pluginChildDirs` → `["🎛️apps"]` is CONFIRMED LANDED** (no longer in flight). Any P3 discovery.ts reasoning must read the live file, not the design doc's stale value.

**Lane-clearance rule correction (from APA, worth internalising):** absence from SMO's release ledger means **FREE**, not held — that ledger only lists plugins SMO ever had a lane on. APA had 5 agents wrongly stop on plugins that were never anyone's. Does **not** change trinity/puzzle (both explicitly IN the ledger as released, so the existing hold/verify logic stands), but for any plugin SMO never mentions, treat silence as free.

**APA's socket moved to `uds:/tmp/cc-socks/40638.sock`** (old 3026 went stale after a session-limit hit).

**P3 taxonomy-flip protocol (widened):** `schemaChildDirs += 💡️inferences` is allowlist-free, so flipping it before every owning subset complies would red-gate SMO's ~21 lanes, UCAS's stdio work and APA's plugin migration simultaneously. Announce on ALL FOUR peer channels immediately before and after the flip — not just script.ts writes. Open question for DKM: does "fan-out complete" include `✳️brep`/`✳️drawing`/`✳️mesh`, or do we structure the completeness check to exclude subsets DKM has not authored yet? DKM's call — ask them directly, it affects P3 timing.

## Lane clearance ledger

| Lane | Holder | State | Source |
|---|---|---|---|
| trinity (♻️rewrite, 🔌️jack) | **US — CLEARED, IN PROGRESS** | APA's relocation landed cleanly. **No split** — all four compute dirs (`🌳️ast`, `🔤️lexer`, `🧮️executor`, `🗣️language-service`) went to `🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (jack-owned; rewrite consumes as same-crate cross-artifact kernel). Module names in `📦️glue.rs` unchanged, only `#[path]` targets repointed — **our 14 call sites' module paths need no rewrite.** `🔨️modules/🔌️jack/{🐚️shell,🧠️lsp}` untouched (separate crates). P1 trinity worker dispatched. | APA report, wave 2 |
| puzzle (◻2d, 🖐️5d, 🧊️3d) | SMO (released) / APA (edits done, unverified) | **VERIFYING → then ours.** APA's `📓️w3-semio-s-plugin-puzzle-report.md` is `apa-status: partial`: edits complete, Step 6 never ran (build lock), 3 `<REPLACE_WITH_REAL_…>` markers outstanding. We are running `cargo check --all-targets` + `cargo test --lib` on OUR target dir and sending APA the raw output. **APA pre-approved our puzzle edits as non-conflicting — green ⇒ lane is ours, no re-ask.** `--all-targets` specifically, not plain check: SMO's `🕸️dag` is a live counterexample (green plain check, red test build). | APA reply, wave 1 |
| stdio (🗄️stdio/**) | UCAS #2548 | **BLOCKED** — 🧿️semio roster restructure 13→18 subsets in flight (adds text/table/spatial-object/graph/kit; renames workflow→flow, object→value). No P2 until UCAS confirms roster settled. Narrow carve-out requested from UCAS for the 5 csv/html/json/pdf/md inference files that are our own bugs (not 🧿️semio subsets, so orthogonal to the restructure) — awaiting reply. | SMO + UCAS msgs, wave 1 |
| `📜️script.ts` | APA (writing now) | Writer order APA → UCAS-W6 → SMO → **us (last)**. Our cluster must sit textually away from SMO's `🔧️PolicyRuleMutationArtifactEngines` ~5280–6050 + its two allowlist constants. Announce immediately before/after writing. | SMO msg, wave 1 |
| `🔣️taxonomy.json` | APA #2549 | APA already flipped `pluginChildDirs`→`["🎛️apps"]`, adding `📝️draft` to `appChildDirs`. Our `schemaChildDirs += 💡️inferences` must be coordinated with APA directly, lands in P3 only. | SMO msg, wave 1 |

## Known constraints carried in from peers

- **puzzle 5d slimming ⇒ SMO mutation vocabulary impact.** SMO derives mutation vocabulary from snapshot shape. Dropping `part_3d.origin`/`orientation` + `part_2d.x`/`y` staleness their change-*/replace-* mutations + triads. We MUST either (a) hand SMO the exact dropped field list, or (b) drop the matching triads ourselves per SMO's `📓️taxonomy.md` + `📓️derivation-rules.md`. Chosen: **(a) report the exact list to SMO** (coordinator relays). Not optional.
- **trinity `SetState` is gone.** SMO replaced it with 7 field-level mutations (edit-before-fixture/lhs/rhs, change/remove-parameter-binding, change/remove-rule-layout-point). Any of our 14 call sites that constructed `SetState` needs the new vocabulary, not a naive swap.

## Work items attributed to THIS ticket by peers (new, P0/P1-adjacent)

5 test failures traced by both SMO and UCAS independently via `git log` to the earlier fan-out session's `💡️inferences` dirs (last committed at flag 491 / `a46ac1f883`, before either peer ticket began — attribution solid, not churn):

1. stdio csv `…::inferences::component::tests::inference_default_law`
2. stdio html — same law
3. stdio json — same law
4. stdio pdf — same law
5. stdio md `…::inferences::outline::component::tests::collects_headings_and_counts_words_and_blocks`

### ✅ D1 DIAGNOSIS COMPLETE — `📓️p0-d1-stdio-failure-diagnosis.md`. **Two unrelated root causes, not one bug.**

| # | artifact | root cause | verdict | action |
|---|---|---|---|---|
| 1 | **html, json, pdf-1.4** (3 of 5) | Outline structs use `#[derive(Default)]`, which disagrees with what `compute()` honestly returns for that artifact's **non-empty** `Snapshot::default()` — default HTML has a root element, default JSON root is `Null`, this PDF subset always has exactly 1 page. | **Test correct, CODE wrong** | Hand-write `impl Default` to match honest compute output. The framework spine's own `AddInference` exemplar (`📡️spr/🎮️command/🦀️component.rs:1119-1125`) documents this as the correct pattern. 3 small blocks. |
| 2 | **csv** (1 of 5) | Same class as above. | **DO NOT TOUCH** | A peer session is fixing it **right now** — uncommitted `git status M` on that exact file, with a fix structurally identical to what D1 independently derived. Touching it collides. |
| 3 | **md** (1 of 5) | Off-by-one in a hand-written fixture: expects `block_count == 3`, but **4 is correct** per the outline's own docstring (a block-quote container and its nested contents both count). | **Code correct, TEST wrong** | Correct the fixture's expected count. Do **not** change the compute logic. |

**Note the shape of this**: the same 4-artifact symptom split into a real code bug (3), a peer collision (1), and a bad test (1). Worth remembering — "4 identical failures must share one cause" was my working hypothesis and it was only 60% right.

**Dispatch status: HELD.** All 5 live under `🗄️stdio/**`, which UCAS still holds; our narrow carve-out request for exactly these files has had **no reply**. The work is small and fully specified, so it can go the moment clearance lands. Do not start it before then.

UCAS's stdio long-profile baseline is now **2021 passed / 5 failed / 3 skipped of 2026**. That is OUR baseline; anything beyond those 5 is a new regression.

**~~Unclear ownership, ours to triage~~ → ✅ RESOLVED / CLOSED.** `semio-framework-os-kernel`'s 144-error lib test build. Full report: `📓️p0-d2-oskernel-triage.md`.

**VERDICT: PRE-EXISTING / UNOWNED — not ours — and no longer reproducing.**
- Cluster A (`tempfile` used but never a dev-dependency): introduced by **`8baa5706ec`** (Aug 6, flag 450). The test code was never buildable — it landed unverified, nobody regressed it.
- Cluster B (`DemoSnapshot`/`DemoMutation` failing `ArtifactPack`/`OpText`/`OpBinary`): **`9391e1ed2b`** / **`b92a614cad`** (Aug 7, flags 462–463, ticket RUNTIME-INSTALLABLE-EXTENSIONS) — a derive-migration wave removed auto-generated impls and the follow-up sweep missed `🏪️store/🔄️sync/🦀️component.rs`'s demo fixtures.
- Both predate our spine commit **`a714dbc6f1`** (flag 489) by 5–6 days. Neither the trait definitions nor os-kernel's `Cargo.toml` were ever touched by our work. Confirms SMO's original read.
- **Bonus:** D2's fresh post-disk-cleanup reproduction shows **0 errors** — both root causes were already fixed by other concurrent sessions (one committed, one uncommitted) that D2 never touched.

Closure relayed to peers. No further action.

## Waves

### P0 — Audit: SUBSTANTIALLY DONE — see `📓️audit-matrix.md`
**Result: the earlier fan-out session's work is GOOD.** All 72 families verified by the coordinator directly to have 5/5 root leaves, 6/6 binary leaves, exactly 1 slug dir with real rs+ts, and a real snapshot-reading derivation. 71/72 have 8/8 text leaves.

**Only 5 confirmed real gaps repo-wide** (full detail in the matrix): puzzle 5d's `🎛flat-position` is a 752-byte re-export shim (fixed by P1's slimming, not separate work) · `🏗️fem/◻2d` missing `📝️text/🛰️component.proto` · both `🏗️fem` slug leaves have 0 tests · `💠️lowpoly` compute entry point needs a read · the 5 stdio failures.

**Two of four Haiku sub-audits produced invalid findings** and are formally retracted in the matrix (a `foo`-vs-`footer` grep false positive; treating `📝️text`/`💾️binary` representation dirs as slug dirs; and flagging the plan-sanctioned pure-fn leaf shape as a missing `InferredField`). **Do not dispatch repairs from `📓️p0-a1-*` or `📓️p0-a2-*` without a coordinator cross-check.** A3 (wiring) + A4 (laws/spine) still running; same cross-check rule applies to them.

Consequence for planning: there is **no bulk repair lane**. P1 shrinks to the two W-B pilots + ~4 small fixes.

### P1 — trinity IN PROGRESS, puzzle still blocked-churn
- **trinity — ✅ EDITS COMPLETE + LIB-CLEAN VERIFIED; `--all-targets` blocked on ONE external issue (not three).**

  **Coordinator-verified live, superseding the worker's 3-blocker report:**
  ```
  cargo check -p semio-s-plugin-trinity               → 0 errors, EXIT=0, Finished in 10.87s   ✅
  cargo check -p semio-s-plugin-trinity --all-targets → EXIT=101, 1 error — in semio-s-plugin-stdio, NOT trinity
  ```
  The sole remaining error:
  ```
  error[E0004]: non-exhaustive patterns: `&…SemioSubsetSnapshot::Table(_)` and `…::Graph(_)` not covered
  error: could not compile `semio-s-plugin-stdio` (lib)
  ```
  **`Table` and `Graph` are literally two of the variants UCAS told us they are adding** in the 🧿️semio roster restructure (text/table/spatial-object/graph/kit). That is content evidence of ownership, not timing — this is UCAS's live work, in the exact lane we are already holding P2 for. Trinity depends on stdio (`Cargo.toml:34`), so it inherits the breakage.

  **Two of the worker's three blockers had already cleared by the time I checked:** framework-plugin is green on both targets (see above), and SMO's uncommitted `os_store::test_support` E0432 no longer appears at all. **Zero errors from any file we touched, on either target.**

  **Verdict recorded: `edits complete, lib-clean, --all-targets gate-blocked on UCAS stdio E0004`.** Not "passed", not "failed". This resolves when UCAS finishes the roster restructure — the same event that unblocks P2 — so it is not worth holding the session on. Re-run `--all-targets` at that point.
- (dispatch note) Full report: `📓️p1-trinity-report.md`. Created `🎛flat-position/` (`JackFlatPosition{positions: BTreeMap<String, JackFlatPositionUv>}`, pure-fn `compute_flat_position`, BFS/seed ported verbatim, 4 tests ported 1:1). Wired into `JackInference` (`#[state(inferred)]` field + `InferenceFieldSpec` reads `nodes`/`edges`/`root_node_id`), all 4 cross-language family-root leaves updated, glue mount added beside `topology`. Deleted `recompute_derived` + both helpers, `DerivedPropertyReadonly` + its mutation use site, the manifest's `flatPosition`/`"derived"` declaration, and 6 tests. **10 distinct call sites converted across 9 files** (the ground-truth table's 14 rows double-counted some sites). Only 3 `recompute_derived` mentions survive repo-wide, all module doc-comments recording provenance — verified by the coordinator.

  **🚩 Out-of-bounds coherence gap, flagged not fixed:** `🧰️framework/🔨️modules/🧮️math/🤖️generated/🦀️nakagin.rs` and its TS twin still embed `flatPosition`/`"kind":"derived"` in their baked-in `NAKAGIN_MANIFEST_JSON`. math's `build.rs` only regenerates when the file is **missing** (the `bun … generate` call is gated behind `if !generated.is_file()`, despite a `cargo:rerun-if-changed` on the JSON source), so editing the trinity-owned JSON source does not resync it. **Inert but incoherent** — nothing writes `flatPosition` any more, so the stale declaration is never exercised. Belongs to whoever owns `🧰️framework/🔨️modules/🧮️math/`; relayed to peers. Correct call by the worker to flag rather than cross the boundary.

- (dispatch note) `recompute_derived` / `DerivedPropertyReadonly` deletion + 14 call-site conversion. Mind SMO's `SetState` removal (replaced by 7 field-level mutations).
- **puzzle — STILL BLOCKED (churn), do not start.** APA says puzzle will not be trustworthy to verify tonight even with the disk fixed, because of ongoing churn/build instability. Holding.

  **🔬 STANDING INSTRUCTION for when puzzle unblocks (parent ruling, wave 2):** verify the *content* of puzzle 5d's `🎛flat-position` facet before planning against it. Do NOT inherit the plan's "already exists, just needs the anti-pattern removal + rewiring" premise. We have already measured it: a **752-byte re-export shim** with 0 `Snapshot` references, 0 compute fn, 0 tests, whose entire body re-exports `flatten_snapshot_inplace` — the very anti-pattern the wave exists to delete. **Budget for puzzle 5d resembling trinity's case** (author-from-scratch or substantially extend) rather than a simple deletion + rewire. The plan's framing has now proven wrong on this exact premise twice (trinity: facet absent entirely; puzzle 5d: facet present but hollow) — treat it as unverified wherever it appears, including for puzzle ◻2d.
- **`🏗️fem` small fixes — EDITS LANDED, gate running.** Worker stalled waiting on its own monitor before writing a report, so the coordinator verified the work directly and took over the gate.
  - fem 2d `📝️text/🛰️component.proto` **created** — now 8/8 leaves. **Grammar honesty checked and PASSED**: it differs from the 3d sibling only in package name, which is correct, because the minimal envelope `message Artifact { string schema = 1; bytes payload = 2; }` is the repo-wide norm for this leaf — verified identical-but-for-package across jack, rewrite, playbook, raster, vcs and en1990. Not a copy-paste breach.
  - both `📦bounds` slug leaves now carry **3 tests each** (were 0 — the only two such leaves among all 72 families). **Anti-vacuous bar met**: `inference_determinism_law` runs over a non-empty hand-built fixture rather than `::default()`, and `bounds_matches_hand_built_node_extent` asserts real coordinates (min `[-2.0, 1.0]`, max `[5.0, 7.5]`, node_count 3, element_count 1).
  - **✅ GATE GREEN — first fully verified P1 lane.** Report: `📓️p1-fem-report.md`.
    ```
    cargo check -p semio-s-plugin-fem --all-targets → 0 errors (warnings only, pre-existing)
    cargo test  -p semio-s-plugin-fem --lib -- bounds → test result: ok. 8 passed; 0 failed
    ```
    8 = our 6 new tests (3 per `📦bounds` leaf) + 2 pre-existing `bounds_matches_node_extent` that matched the filter. Coordinator re-ran independently and saw the same 8/8.
    Cost note for planning: that `cargo test` took **12m 18s** even warm, under ~64 concurrent cargo processes. The worker had two runs killed by harness timeouts at `Blocking waiting for file lock on package cache` — that is **global `~/.cargo` package-cache contention, not our scoped `CARGO_TARGET_DIR`**, so a per-ticket target dir does not isolate you from it. A detached (`nohup … &`) launch survives the harness timeout; budget for this on every remaining gate.
  - (historical) Gate was RED on the stale-include blocker — NOT on our edits: `cargo check -p semio-s-plugin-fem --all-targets` fails with 2 hard errors, both `couldn't read …📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio: No such file or directory (os error 2)` in `📸️snapshot/📝️text/🦀️component.rs` for 2d and 3d. **That facet was never opened by our worker** — our edits are the new `💡️inferences/📝️text/🛰️component.proto` (not yet compiled) and two `💡️inferences/📦bounds` test blocks. Zero of our edits appear in the error output. **HOLD; re-run `--all-targets` after APA pings that their fix landed.**
- (dispatch note) fem 2d's missing `📝️text/🛰️component.proto` (7 of 8 leaves) + real tests for both `📦bounds` slug leaves (the only two slug leaves among all 72 families with zero `#[test]`). Lane is free: `🏗️fem` appears in **no** peer ledger, and per APA's correction, ledger-silence means free, not held.
- **`💠️lowpoly` entry-point check — NOT dispatched.** lowpoly is explicitly in SMO's HELD list. Deferred until SMO releases it.

#### ⚠️ Two corrections to the trinity brief, verified from disk before dispatch

1. **`recompute_derived` and `DerivedPropertyReadonly` are NOT in `⚙️engine/🧮️executor/`.** Both are defined in `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs` — `pub fn recompute_derived(&mut self)` at **:289** (on `Graph`) and the `DerivedPropertyReadonly` `TrinityRamError` variant at **:66**. The executor merely *calls* it (:353). A worker sent to the executor would have found nothing to delete.
2. **There is NO `🎛flat-position` inference facet in trinity.** The plan asserted both trinity artifacts "already have `💡️inferences/🎛flat-position/` facets — only the anti-pattern removal + consumer rewiring remains." False. Actual slug dirs: jack → **`🧭topology`** only; rewrite → **`📦bounds`** only. So the flat-position inference must be **created**, porting the BFS/seed layout math out of `recompute_derived`. **This makes trinity materially bigger than the plan's estimate** — it is create-facet + wire + delete + convert, not just delete + convert.

Also folded into the worker's scope: a schema-level `PropertyKind::Derived` declaration of `flatPosition` (`🗿️artifacts/🔌️jack/🦀️component.rs:507`) and its guard test (:702) must go once `flatPosition` is no longer a stored property. Chosen leaf shape: **pure-fn**, mirroring the sibling `🧭topology`, whose own module doc gives the rationale verbatim — *"a plain whole-snapshot scalar … no `InferredField`/incremental caching needed for a single BFS pass."* Consistent with the parent's ruling.

### P2 — stdio fan-out remainder: BLOCKED on UCAS
**Target is now 22 subsets, not 34.** 🧿️semio v1 = **11** (14 minus `✳️brep`/`✳️drawing`/`✳️mesh`) + geometry/BIM = 11 (ifc×2, step/ap214, dwg×2, dxf, stl, gltf, obj, ply, las) + media 4 + containers 3 + bcf/epw 2. **`✳️brep`/`✳️drawing`/`✳️mesh` are reassigned to DKM outright** — not deferred, off our plate — since their derived fields (tessellation, mass-properties, validation-report, flattened-scene) are by-products of DKM's engine dissolution. Contingency: DKM's write access to those stdio dirs is still an open request to UCAS; if it falls through DKM hands the three back.
Target roster is actively moving under us — do not start.

### P3 — Policy cluster + taxonomy flip + final verify: NOT STARTED
Gated on P1 + P2 complete, our `📜️script.ts` writer slot (position 4), APA taxonomy coordination, and DKM's `✳️brep`/`✳️drawing`/`✳️mesh` landing.

**Unresolved design question that must be settled BEFORE the policy cluster is written** — see `📓️audit-matrix.md`: only **8 of 72** families use `InferredField`; the other 64 are pure-fn folds. `📌️important.md` rule 13 says pure-fn leaves are breaches; the approved plan says they are sanctioned. The policy cannot be written until this is decided. Escalated to the parent session.

**Related:** `inference_cache_transparency_law` / `inference_incrementality_law` appear zero times repo-wide — the behaviours are proven under descriptive names at the spine and in the puzzle3d pilot. P3 must decide whether the policy checks law *names* (⇒ rename ~10 files) or law *behaviours* (⇒ nothing to do). Demanding them on the 64 pure-fn families would manufacture 64 vacuous tests.

#### Optional P3 add-on offered to us by DKM: a report-mode `📦️index.ts` policy

DKM's observation: the 517/567 dead TS export paths exist precisely because **there is no policy on `📦️index.ts` at all**. They suggest whoever holds a script.ts slot add a **small report-mode (non-gating) export-existence rule** — ~10 lines — rather than attempting the 517-file migration. Nobody is claiming it: APA offered, DKM declined, and we cancelled our own W-C step 5 over this same finding.

**Coordinator's disposition: TAKE IT, but strictly last and strictly conditional.** We already did the investigation, it's cheap, it's report-mode so it cannot gate anyone, and it's directly adjacent to our own P0 finding. Conditions, in order of precedence:
1. It goes in **only after** the inference policy cluster is written, verified and confirmed landing cleanly.
2. It must register at the **`dissolveBreaches`-style site** (which filters to `priority === "high"` before throwing), never the `osBreaches` site (which throws on ANY breach regardless of priority and would red-gate all five sessions instantly). Rule priority must be **`"medium"` or `"low"`**, matching APA's 5 report-mode rules / 1727 breaches which gate nothing.
3. **If it complicates the write, adds risk, or the slot is contended — drop it.** It is polish. The inference cluster is the deliverable and must not be delayed or endangered for it.
4. Watch the Bun `**/` tokenizer trap in its doc comment (a literal `**/` inside a `/** … */` block terminates early → `error: Unexpected 📦`). Describing `📦️index.ts` glob shapes is exactly the case that triggers it — use `//` or reword.

#### P3 intelligence gathered from peers (act on all of this)

- **Verify the queue yourself.** Before writing, run `git log --oneline -5 -- 📜️script.ts` to confirm UCAS-W6's and SMO's writes actually landed. APA's write has STOPPED (done). Do not trust an announcement alone.
- **Two gate registration sites behave differently.** `dissolveBreaches` filters to `priority === "high"` before throwing (safe for report-mode rules); the earlier `osBreaches` block throws on ANY breach regardless of priority. Register our inference rules ONLY at the `dissolveBreaches`-style site — the other one instantly red-gates all five sessions on first run.
- **Bun tokenizer trap.** A `/** … */` doc comment containing a literal `**/` (e.g. a glob like `**/📦️packages` written in prose) terminates early at the embedded `*/` and fails with `error: Unexpected 📦`. Our cluster documents directory shapes in doc comments, so this will bite us specifically. Use `//` or reword; never a literal `**/` inside a block comment.
- **Verify calibration baseline: 22188 pre-existing high-priority breaches across 27 rules** (19601 are handcrafted-grammar/spec-distinctness, unrelated to us). Diff against 22188, not 0.
- **`🧪️index.test.ts` baseline: 20 pre-existing failures** (APA's `📓️baselines.md`). Diff against 20, not 0.
- **`🔍️discovery/🟦️component.ts`:** re-read the live file — `pluginChildDirs` is ALREADY flipped by APA to `["🎛️apps"]`, so the older multi-entry value in the original design doc is stale. Do NOT add `🗿️artifacts` to `artifactFacetChildLevel` — it has no leaf `🦀️component.rs`, is governed separately by `artifactsDirName`, and adding it panics the gate across all 33 plugins.
- **Runtime `assert!` at `🔌️plugin/🦀️component.rs:2226-2235`** reads taxonomy arrays dynamically and requires `<child>/🦀️component.rs` to exist for every listed entry on every owner. Adding `💡️inferences` to `schemaChildDirs` before 100% fan-out panics the gate repo-wide. Announce the flip on all four peer channels before and after.

## Peer-flagged, OUT OF SCOPE unless the user directs otherwise

APA's new plugin-purity rule found **115 impurity breaches** across the plugin tree (36 item-scope `RefCell`, 19 `Mutex`, 11 `Atomic*`, 6 `thread_local!`). APA believes a meaningful share are derived caches — values recomputed from the snapshot and memoised in ambient memory — i.e. inference candidates that would otherwise duplicate a cache we are about to build alongside them. APA's puzzle report already inventories this for puzzle (3d's `precompute`/`fill_display_memo`/`geometry_cache`/`document_sections_cache` and 5d's entire `Puzzle5dPlayApp` are derived-cache, not draft state).

**This is useful context when we are already inside a plugin's files during P1/P2 — it is NOT authorisation to convert those 115 sites.** That would be new scope beyond the approved plan. Escalate only if it becomes directly blocking (e.g. a stdio subset cannot get an honest inference without touching one).
