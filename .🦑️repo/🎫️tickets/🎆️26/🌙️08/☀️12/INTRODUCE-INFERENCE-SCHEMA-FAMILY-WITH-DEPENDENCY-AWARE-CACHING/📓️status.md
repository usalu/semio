# Status

Coordinator: Opus 5 session. Executors: Sonnet 5 agents. Explorers: Haiku 4.5 agents. Plan authored by a Fable session at `/Users/ueli/.claude/plans/finish-introduce-inference-schema-family-iridescent-sprout.md`.
**Only the coordinator edits this file.** Agents append to their own report files.

## Peer session map (corrected, authoritative)

| Session name | Ticket | Short |
|---|---|---|
| semio-9f | SEMANTIC-MUTATIONS-OVERHAUL #2545 | SMO |
| semio-52 | ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE #2549 | APA |
| semio-b2 | UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM #2548 | UCAS |
| (5th, uds:/tmp/cc-socks/53352.sock) | DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS #2550 | DKM |
| this session | INTRODUCE-INFERENCE-SCHEMA-FAMILY #2546 | IIF (us) |

**`📜️script.ts` / `🔣️taxonomy.json` write-slot queue (5-deep, confirmed):**
APA → UCAS-W6 → SMO → **US (position 4)** → DKM (position 5). DKM is waiting on us: their `policyEngineRepEscapeBreaches` / `policyEngineConsumptionOutsideFacetBreaches` name the inference facet as the sanctioned home for derived compute.

**P3 taxonomy-flip protocol (widened):** `schemaChildDirs += 💡️inferences` is allowlist-free, so flipping it before every owning subset complies would red-gate SMO's ~21 lanes, UCAS's stdio work and APA's plugin migration simultaneously. Announce on ALL FOUR peer channels immediately before and after the flip — not just script.ts writes. Open question for DKM: does "fan-out complete" include `✳️brep`/`✳️drawing`/`✳️mesh`, or do we structure the completeness check to exclude subsets DKM has not authored yet? DKM's call — ask them directly, it affects P3 timing.

## Lane clearance ledger

| Lane | Holder | State | Source |
|---|---|---|---|
| trinity (♻️rewrite, 🔌️jack) | SMO #2545 (released) / APA #2549 (in flight) | **HOLD** — SMO's live predicate file lists trinity RELEASED, but APA has a relocation agent in flight moving `🌳️ast`, `🔤️lexer`, `🧮️executor`, `🗣️language-service` + `🔨️modules` into trinity's artifact engines. **We go second** (APA's reasoning is sound: if we rewrite the 14 call sites first and APA then moves those files, our fresh references break; the reverse only wastes the small deleted portion). APA's agent will NOT preserve `recompute_derived`/`DerivedPropertyReadonly` — we delete them at whatever the new path is. **Wrinkle: APA may SPLIT the four dirs across 🔌️jack and ♻️rewrite. Our 14 call sites span both artifacts, so get the new per-artifact path map from APA before converting — do not assume the old layout.** | APA reply, wave 1 |
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

UCAS's stdio long-profile baseline is now **2021 passed / 5 failed / 3 skipped of 2026**. That is OUR baseline; anything beyond those 5 is a new regression.

**Unclear ownership, ours to triage:** `semio-framework-os-kernel` lib test build fails with 144 errors, all in `🔨️modules/🏪️store/🔄️sync/🦀️component.rs` — `tempfile` used but not a dev-dependency, plus `DemoSnapshot`/`DemoMutation` fixtures failing `ArtifactPack`/`OpText`/`OpBinary` trait bounds. SMO says it predates their ticket (commits 492/480/467) and they touch neither the module nor its Cargo.toml. We must determine whether inference work touches it; if not ours either, report back to SMO for a wider broadcast. **Do not silently "fix" it.**

## Waves

### P0 — Audit: SUBSTANTIALLY DONE — see `📓️audit-matrix.md`
**Result: the earlier fan-out session's work is GOOD.** All 72 families verified by the coordinator directly to have 5/5 root leaves, 6/6 binary leaves, exactly 1 slug dir with real rs+ts, and a real snapshot-reading derivation. 71/72 have 8/8 text leaves.

**Only 5 confirmed real gaps repo-wide** (full detail in the matrix): puzzle 5d's `🎛flat-position` is a 752-byte re-export shim (fixed by P1's slimming, not separate work) · `🏗️fem/◻2d` missing `📝️text/🛰️component.proto` · both `🏗️fem` slug leaves have 0 tests · `💠️lowpoly` compute entry point needs a read · the 5 stdio failures.

**Two of four Haiku sub-audits produced invalid findings** and are formally retracted in the matrix (a `foo`-vs-`footer` grep false positive; treating `📝️text`/`💾️binary` representation dirs as slug dirs; and flagging the plan-sanctioned pure-fn leaf shape as a missing `InferredField`). **Do not dispatch repairs from `📓️p0-a1-*` or `📓️p0-a2-*` without a coordinator cross-check.** A3 (wiring) + A4 (laws/spine) still running; same cross-check rule applies to them.

Consequence for planning: there is **no bulk repair lane**. P1 shrinks to the two W-B pilots + ~4 small fixes.

### P1 — Repairs + W-B pilots: puzzle VERIFYING, trinity HOLD
- puzzle ◻2d missing family + puzzle 5d snapshot slimming (single serial writer — shared puzzle `📦️glue.rs`). Gated on our own green verification run, then pre-approved.
- trinity `recompute_derived` / `DerivedPropertyReadonly` deletion + 14 call-site conversion. Gated on APA's relocation landing + their new per-artifact path map. Mind SMO's `SetState` removal (replaced by 7 field-level mutations).
- Small fixes: fem 2d proto leaf, fem 2d+3d slug tests, lowpoly entry point.

### P2 — stdio fan-out remainder: BLOCKED on UCAS
**Target is now 22 subsets, not 34.** 🧿️semio v1 = **11** (14 minus `✳️brep`/`✳️drawing`/`✳️mesh`) + geometry/BIM = 11 (ifc×2, step/ap214, dwg×2, dxf, stl, gltf, obj, ply, las) + media 4 + containers 3 + bcf/epw 2. **`✳️brep`/`✳️drawing`/`✳️mesh` are reassigned to DKM outright** — not deferred, off our plate — since their derived fields (tessellation, mass-properties, validation-report, flattened-scene) are by-products of DKM's engine dissolution. Contingency: DKM's write access to those stdio dirs is still an open request to UCAS; if it falls through DKM hands the three back.
Target roster is actively moving under us — do not start.

### P3 — Policy cluster + taxonomy flip + final verify: NOT STARTED
Gated on P1 + P2 complete, our `📜️script.ts` writer slot (position 4), APA taxonomy coordination, and DKM's `✳️brep`/`✳️drawing`/`✳️mesh` landing.

**Unresolved design question that must be settled BEFORE the policy cluster is written** — see `📓️audit-matrix.md`: only **8 of 72** families use `InferredField`; the other 64 are pure-fn folds. `📌️important.md` rule 13 says pure-fn leaves are breaches; the approved plan says they are sanctioned. The policy cannot be written until this is decided. Escalated to the parent session.

**Related:** `inference_cache_transparency_law` / `inference_incrementality_law` appear zero times repo-wide — the behaviours are proven under descriptive names at the spine and in the puzzle3d pilot. P3 must decide whether the policy checks law *names* (⇒ rename ~10 files) or law *behaviours* (⇒ nothing to do). Demanding them on the 64 pure-fn families would manufacture 64 vacuous tests.

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
