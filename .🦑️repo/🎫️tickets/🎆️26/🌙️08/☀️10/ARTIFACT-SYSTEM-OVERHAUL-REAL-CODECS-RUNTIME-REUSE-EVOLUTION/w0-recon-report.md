# W0 Recon Report — Artifact Schema Overhaul

Plan: `~/.claude/plans/the-current-schemas-are-scalable-journal.md`. Full ownership ledger: appended as `## Schema Overhaul W0 Recon — Ownership Ledger (2026-08-11)` in this folder's `STATUS.md` (copied here in condensed form below). Design doc placed: `🧬️schema-design.md` in this folder. This report is what the orchestrator should read next before dispatching S1.

## 1. Corrected standard count

**28 stdio artifacts, 31 standards** — not "29 artifacts (30 standards)" per the plan's own opening line. Confirmed by directory `ls` count and independently by `catalog.json`'s `counts.stdio_artifacts: 28`. 25 artifacts have 1 standard; gif, pdf, and dwg each have 2 (87a/89a, 1.4/1.7, ac1018/ac1024) → 25 + 6 = 31 standards total. `glb` no longer exists as a top-level artifact (merged into gltf by V2a, confirmed real — see §4).

## 2. Test baseline

State changed mid-recon (external churn from concurrent sessions, exactly as the brief warned). Two full runs of `cargo test -p semio-s-plugin-stdio --lib`:
- **Start of recon**: 315 passed / 4 failed (2 dwg D2-decompression "invalid backref" errors, 2 pdf 1.7 real-fixture bugs: xref field-width decoding, WinAnsi text extraction).
- **End of recon** (after other sessions' own fixes landed — confirmed against this ticket's STATUS.md V6/V2b "DONE" entries, and independently re-verified by re-running the exact 4 previously-failing test names): **318 passed / 0 failed. Fully green.**

Use 318/0 as ground truth for dispatch decisions. All per-standard counts in the ledger are from the second (green) run.

## 3. Ownership ledger (condensed — full table with defects/live-signals is in STATUS.md)

Maturity tiers used: **stub** (bare `Vec<u8>`/`serde_json::Value` where structure is expected, or trivially thin), **generic** (some real structure, materially short of the target spec), **partial** (real per-field/op-slot structure but incomplete field/variant coverage), **rich** (matches or exceeds target spec). Diff maturity is almost universally **generic-template** (the 34-line `XDiff{snapshot:Option<XSnapshot>}` full-replace shape) or worse (**apply-and-capture** for svg) across all 31 standards — this is the single most uniform finding of the recon.

| Tier | Snapshot | Diff | Mutations |
|---|---|---|---|
| **rich** | xml, zip, svg, obj, xlsx (bordering) | *(none — zero of 31 have handcrafted sparse diffs yet)* | *(none — zero of 31 have the target ~14-20 variant vocabulary)* |
| **partial** | csv, step/ifc(generic, shared-type defect), docx, pptx, bcf, gif 89a, pdf 1.7, dwg ac1024, las | gif 89a, pdf 1.7 (op-slot pattern, not sparse) | svg (7 var), gif 89a (6 var), pdf 1.7 (7 var) |
| **generic** | dxf, gltf, xlsx-shared-detail-gaps | *(everything not listed above/below)* | — |
| **stub** | txt, json, deflate, png, jpg, md, bmp, tiff, ply, bmp, dwg ac1018 | 27 of 31 (the untouched generic-template) | 28 of 31 (`{NoMutation, SetSnapshot}` only) |

**Every one of the 31 standards' `apply_<x>_mutation` function still returns `()`, not a `Diff`** — including gif 89a and pdf 1.7, the two most-advanced standards. This confirms the plan's core complaint holds with zero exceptions found.

## 4. Specific verifications requested

- **gltf/glb merge**: REAL, not a stub and not "glb simply gone with no replacement." `encode_glb`/`decode_glb` in gltf's engine (`🧊️gltf/🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs`) implement a genuine 12-byte GLB header, magic/version validation, chunk walker (JSON + BIN chunks), BIN-chunk embedding into `buffers[0]`, `GltfSourceForm::{Json,Glb}` round-trip tracking, and even carry a regression test for a real prior bug (BIN-chunk padding-length mismatch across alignments). The remaining gap (gltf's `document` field is still `serde_json::Value`, not the fully typed 2.0 model) is exactly the F4-scope work the plan already expects — not a surprise, not a regression.
- **Registry / V4 check**: `register_document_codec` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:629-632`) is confirmed **still the plain flat `HashMap::insert`** (silent last-write-wins) by direct read. V4 (scoped-down, per this ticket's own STATUS.md) deliberately did not touch it — genuinely additive elsewhere (`ArtifactEnvelope.dialect`/`.migrated_from`, a standalone migration registry). **S-5 is clear to implement with zero conflict.**
- **Git rename false positives**: Confirmed pure git content-similarity heuristic noise. Read the actual current content of the gif 89a files `git status -M` reports as renamed from `🧊️glb/...` — zero `glb`/`gltf`/`GLB`/`Glb` string matches anywhere in gif 89a's `🧬️schema/` tree, and `GifSnapshot`/`GifDiff`/`GifMutation` references throughout confirm genuinely gif-89a-shaped content. **No copy-paste-residue defect exists for F3's gif agent to clean up on this point** — the STATUS.md ledger row says so explicitly so nobody re-investigates this.

## 5. New finding not anticipated by the plan: pdf has gif's exact S-6 problem

The plan's S-6 spine row only mentions gif (87a currently wired as canonical/primary despite 89a being richer — needs flipping so 89a becomes primary). **pdf has the identical problem, unmentioned in the plan**: glue.rs's `pdf::schema`/`pdf::engine` shims point at `standards::v1_4` (the 87-line `PageDoc`-based stub) rather than `v1_7` (the real 1794-line object-graph engine). 1.7 was built registering itself under a deliberately separate schema id `stdio.pdf.1.7` specifically to dodge this collision — its own doc-comment cites gif 89a's precedent by name ("same rationale as gif 89a"). Concretely: `crate::artifacts::pdf::schema::snapshot::PdfSnapshot` (what everything outside the standards tree imports) currently resolves to the STUB type, not the real one.

**Recommendation**: extend S-6 (or add a lightweight twin spine row) to flip pdf's shim the same wave gif's gets flipped, so `pdf::schema`/`pdf::engine` point at v1.7. Otherwise F4's pdf agent has no natural place to land this fix within the planned glue.rs-ownership discipline (fan-out agents never touch glue.rs; only wave-closers do) and it either gets skipped or done ad hoc.

## 6. Spine-sizing corrections (both larger than the plan's own estimates — relevant to S1 risk/time budgeting, not blockers)

- **S-2** (`ArtifactBuilder::mutate` signature flip): **252 impl blocks repo-wide** (`grep -rl "impl ArtifactBuilder for"`), not a "sweep ALL implementors incl. non-stdio" scoped near the stdio count. This is the single biggest sizing risk this recon found for S1. The plan's own staged approach (additive-first, flip-second) is the correct mitigation and needs no change — just budget for ~8x the impl-site count the plan's phrasing implies.
- **S-3** (delete dead `ArtifactEngine` trait + impls): **85 impl blocks repo-wide**, not "~30." Confirmed genuinely dead code — grepped for `dyn ArtifactEngine` / generic `<E: ArtifactEngine>` bounds / `ArtifactEngine::new` and found zero real construction sites; every artifact's XEngine struct is built via its own inherent `::new()`. Deletion is safe, just bigger than estimated.
- **S-4** (`ArtifactSchemaDescriptor` gains `mutations: FacetLeaves`): confirmed exactly as the plan expects, 3-field struct, 391 `include_str!` call sites all still 3-block. No surprises, S2 scope as planned.
- **S-7** (keep vcs `CollectionDiff`/`CollectionMutation`, stdio-ban only): confirmed real users beyond the plan's own citation (flow, store) — also space and dag modules. Safe as planned.

## 7. Other confirmed defects worth flagging to specific waves (full detail in STATUS.md ledger)

- **F2**: `MeshVertex`/`MeshTriangle` shared verbatim between stl and ply (the plan's "MeshVertex×4" claim, 2 of 4 instances located precisely). `RasterImage` shared between png/jpg/tiff (bmp uses an inline equivalent-shape anti-pattern — likely the plan's implied 4th instance).
- **F3**: svg's diff is confirmed **apply-and-capture** by its own doc-comment despite 23/23 green tests — a prior wave fixed the symptomatic test failures (whitespace-node counting, attribute-order via `Vec<XmlAttr>` not `HashMap`) without fixing the underlying architecture defect the plan explicitly bans. Flag this so F3's svg agent doesn't treat "tests pass" as "done."
- **F4**: ifc's `IfcSnapshot.document` is literally `step::engine::part21::Part21Document` — the same Rust type, cross-artifact, with zero `IfcEntity`/`IfcValue` wrapper. This is the most severe copy-paste-type instance found (it's the persisted type itself, not a value substruct) and should be called out explicitly in ifc's brief. docx's OPC layer is confirmed real/target-matching (not the wrong `Vec<ZipEntry>` shape) — only its document-body layer (paragraphs/runs, no tables/props/styles) is shallow. pptx's shape tree is confirmed **flattened away entirely** — reconstructing it needs re-deriving from `opc.parts` XML since the current model already discarded shape boundaries.
- **F5**: dwg ac1018 is a **deliberately frozen legacy shim** per an explicit "Decision #5" doc-comment in its own `to_snapshot()` — not a less-finished ac1024. Do not dispatch "bring ac1018 to decode parity." ac1024 reached D1+D2 fully (all 13 real sections on the 145KB architectural.dwg fixture locate and decompress cleanly, confirmed via fresh re-run) — D3-D5 have zero region-marker presence anywhere, confirmed out of scope, not silently started.

## 8. Recommended F1-F5 roster

**Confirms the plan's draft roster as-is.** No standard needs deferral to a later wave — nothing found "live and blocking" as of this recon's final state. Only actionable changes:

- **F1**: re-poll csv's git status immediately before dispatch (its schema/snapshot file itself was mid-edit, −77/+15 lines, at recon start — by recon end it's unclear whether that session finished; re-check before an F1 agent starts).
- **F2**: no roster change; brief should flag ply and tiff as needing the most net-new field design (weakest snapshots of the 6).
- **F3**: no roster change; brief should flag svg's apply-and-capture defect explicitly (see §7) and gif 89a's real remaining scope (~14 of ~20 target mutations still missing, `apply_gif_mutation` still returns `()`, known op-slot absorb bug per the plan's own intro — this is the most-advanced standard but still meaningfully incomplete).
- **F4**: needs the pdf S-6-twin spine fix (§5) landed in the same wave, ideally via the wave-closer mechanism (fan-out agents don't touch glue.rs). Brief should flag ifc's shared-type defect explicitly (§7).
- **F5**: no roster change; brief must explicitly state ac1018 stays frozen at D0 by design and ac1024's D1/D2 decompression bugs are ALREADY FIXED (as of a concurrent session's work that resolved during this very recon) — remaining dwg scope is snapshot/diff/mutation enrichment within the existing honest D1/D2 boundary, not bugfixing.

No artifact is recommended for removal from its planned wave, and none needs to move earlier or later. The crate is fully green (318/0) and no standard shows active in-progress editing as of this recon's end — S1 can proceed on the plan's existing wave structure.
