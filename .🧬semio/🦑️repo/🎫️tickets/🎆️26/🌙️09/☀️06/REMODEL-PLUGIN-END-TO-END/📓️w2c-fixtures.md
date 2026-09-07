# 🧫️ W2c — exhaustive, real-world-shaped mutation fixtures

Regenerates all 34 pre-existing remodeling mutation vectors onto the CURRENT schema and grows the
catalogue from 34 to **132** committed specification vectors — one `toy`, one `realworld`, one per
distinct guard (`refusal`) and one boundary/no-op (`edge`) for every one of the 35 kinds, as far as
each kind's own semantics reach.

## 1. Vector matrix (kind × role)

| kind | toy | realworld | refusal | edge | diagnostics pinned |
|---|---|---|---|---|---|
| `create-stream` | 1 | 1 | 2 | 1 | duplicate-id, invariant |
| `delete-stream` | 1 | 1 | 1 | 1 | target-missing |
| `change-stream-sync` | 1 | 1 | 1 | 1 | target-missing, no-op |
| `add-stream-frame` | 1 | 1 | 1 | 1 | target-missing, no-op |
| `remove-stream-frame` | 1 | 1 | 1 | 1 | target-missing (index out of range) |
| `replace-stream-source` | 1 | 1 | 1 | 1 | target-missing |
| `create-asset` | 1 | 1 | 1 | 1 | invalid-asset-payload |
| `delete-asset` | 1 | 1 | 1 | 1 | target-missing (+ cascade info ×2) |
| `create-camera-calibration` | 1 | 1 | 1 | — | duplicate-id |
| `update-camera-calibration` | 1 | 1 | 1 | 1 | target-missing, no-op |
| `delete-camera-calibration` | 1 | 1 | 1 | 1 | target-missing |
| `create-rig-extrinsic` | 1 | 1 | 2 | — | duplicate-id, invariant |
| `delete-rig-extrinsic` | 1 | 1 | 1 | 1 | target-missing |
| `update-rig-extrinsic` | 1 | 1 | 1 | 1 | target-missing, no-op |
| `create-gcp` | 1 | 1 | 1 | 1 | duplicate-id |
| `delete-gcp` | 1 | 1 | 1 | 1 | target-missing (+ cascade info) |
| `add-gcp-observation` | 1 | 1 | 1 | 1 | target-missing, no-op |
| `remove-gcp-observation` | 1 | 1 | 1 | 1 | target-missing (index out of range) |
| `update-ingest-params` | 1 | 1 | 1 | 1 | invariant, no-op |
| `update-feature-params` | 1 | 1 | 1 | 1 | invariant, no-op |
| `update-match-params` | 1 | 1 | 1 | 1 | invariant, no-op |
| `update-sfm-params` | 1 | 1 | — | 1 | no-op |
| `update-dense-params` | 1 | 1 | — | 1 | no-op |
| `update-mesh-params` | 1 | 1 | — | 1 | no-op |
| `update-motion-params` | 1 | 1 | — | 1 | no-op |
| `update-geo-params` | 1 | 1 | 1 | 1 | invariant, no-op |
| `replace-job` | 1 | 1 | — | 1 | no-op |
| `replace-sparse` | 1 | 1 | — | 1 | no-op |
| `replace-dense` | 1 | 1 | — | 1 | no-op |
| `replace-mesh-result` | 1 | 1 | 1 | 1 | incomplete-mesh, no-op |
| `replace-trajectory` | 1 | 1 | 1 | 1 | target-missing (clear an absent value) |
| `replace-tracks` | 1 | 1 | — | 2 | no-op |
| `replace-geo-products` | 1 | 1 | 1 | 1 | target-missing |
| `replace-qc` | 1 | 1 | 1 | 1 | target-missing |
| `commit-reconstruction` | — | — | 3 (+1 shared) | — | invalid-reconstruction-{sparse,asset,mesh} |
| **total** | **34** | **34** | **31** | **33** | |

By outcome: **83 applied**, **31 rejected**, **18 warned no-op**. `—` in the refusal column means the
kind's ONLY non-applying path is `mutation.no-op` (a finiteness invariant JSON cannot even express) —
its `edge` row pins that instead of inventing a refusal it does not have. `—` in the edge column means
the verb appends and has no index/ordering semantics at all.

Every scenario is planned twice where its inverse holds: **253 feature scenarios**
(132 mutate + 118 inverse + 2 shared commit-reconstruction + identity-round-trip).

## 2. The two base scenes

**`toy`** — the two-stream unit scene the 34 pre-existing vectors were authored against, corrected on
three counts: `durableArtifacts` is present (previously absent everywhere); `stream-a`'s video source
is now 1024×768 so it agrees with `cam-a`'s own principal point (cx=512, cy=384) instead of claiming
1920×1080 against a 1024×768 intrinsic; and its durable leaf is real — `asset-a`'s committed
`childId` was brute-forced back to its own `(mime, data)` pair (`image/jpeg`, `ZnJhbWUtYQ==`) and the
leaf recomputed from it, so `remodeling_asset` can reconstitute it (which is what `delete-asset`'s
inverse depends on; see §5).

**`realworld`** (`orbit-survey`) — modelled on this subset's own `📚️examples/🛰️synthetic-orbit`:
10 frames at 320×240, fx=fy=272 Brown–Conrady with the example's own distortion, a two-stream rig
over three calibrated cameras (two placed, one spare), 11 captured assets with **11 real durable
leaves**, a 4-point GCP network with 7 observations spread across both streams and six different
frames (one point deliberately unobserved), a **104-point sparse cloud taken from the example's own
ground truth**, a 64-point dense cloud with confidence and classification lanes, a real mesh handle
with a watertight report, a 10-pose trajectory from the ground-truth extrinsics, 3 motion tracks, geo
products and a QC report. `realworld-cleared` is the same document with trajectory/geo/qc nulled, used
by the three "clear an already-absent value" refusals.

Every `assets[key].childId` in BOTH scenes is a real `image_asset_child_handle` digest — Rust
`DefaultHasher` (SipHash-1-3 keyed `(0,0)`) reproduced in Python and checked against the committed
`create-asset` vector's own minted handle. No handle in either document is an invented string.
Every `format: float` lane is f32-quantized so the committed lexeme is already the fixed point.

## 3. What was written

- 132 case directories under `…/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`, each with
  `📸️snapshot/{⬅️before,➡️after}/🔣️.json`, `🦠️mutation/🔣️.json`, `🎯️outcome/🔣️.json`, a mounted
  `🦀️.rs`, and either `🔺️diff/🔣️.json` or — for the 31 refusals — `🔺️diff/🚫️.absent`, the
  repository-wide marker for a deliberately absent file (93 prior uses under `✏️s/`, e.g. trinity's
  `✂️delete-edge/🧪️tests/🚫️rejects-cutting-an-0cb007`). Refusal outcomes follow that same precedent:
  `{"status": "rejected", "code", "path"}`.
- New case directory names follow the real convention: the 2026-09-05 path-shortening pass's own
  `truncateWithHash`, reproduced and **verified against three names it actually produced here**
  (`🎥️adds-stream-c-458900`, `⏱️shifts-stream-a-5b442c`, `🔳️doubles-the-c245d5` all reproduce exactly).
  The generator refuses any two siblings whose readable stems collide.
- All 132 case `🦀️.rs` modules regenerated. They no longer use `serde_json`, which W9's
  serde-elimination removed from this type graph: they decode with `pack::from_json_str` and compare
  through `pack::json_from_dsl_value(&dsl::ToValue::to_value(…))` against `pack::parse_json`. The 34
  committed modules would not have compiled as they stood.
- 132 `#[path]` mounts in `📦️packages/🦀️rust/🦀️.rs` (targeted region edits; 11 stale mounts pruned).
- `🥒️.feature` rewritten: 7 outlines + the identity scenario, `| id | kind | vector | code |` tables.
- `🦀️.rs` (subject) and `🐍️.py` (reference) registrations generated FROM the same table, so a row that
  gains or loses a vector cannot leave either half behind.
- `🐍️.py` reference rewritten: generic `mutate`/`inverse` handlers plus a `refusal_of` statement per
  kind, derived from what each verb MEANS (a stream may not be created twice, an index must address a
  frame that exists, a commit publishes only what a staged run produced), not from production's flow.
  `create-asset` now also states the durable leaf independently and in full — only the content-address
  it is filed under is adopted.
- `🧫️fixtures/🏁️commit-reconstruction/` fixed: `⬅️before.json` and `➡️after.json` are now the same
  schema-valid document (`job.stage = bundle-adjusting`, `results.mesh` non-null), and the
  `🦠️mutation.json` the feature file named is on disk. The three case-local duplicates were removed and
  the feature now addresses the owner-shared triple by `shared://`. `🔮️oracle/🔣️.json`'s
  `fixtureManifests[0]` follows (`outcome: rejected`, three files with fresh sha256/bytes).
- `🔮️oracle/🔣️.json`: `mutationCatalogs[0].vectors` = 35 kinds / 132 scenarios;
  `fixtureCoverage.vectors` = 132.

## 4. Validation actually run

```
python3 🐍️schema-validate.py       → 500 documents, 0 violations; 132 payloads, 0 violations;
                                      normative leaves vs the Rust keyword allowlist: no problems
python3 🐍️fixture-audit.py         → 400 uris, 0 missing, 400 resolved, 0 unexpanded
python3 🐍️mount-check.py           → 0 dangling
python3 🐍️python-oracle-dryrun.py  → 253 planned, 253 registered, 253 passed, 0 failed, 0 unknown
bun nx run @semio-tech/remodel-js:test --skip-nx-cache → 4 files, 1333 passed (was 386)
```

`🐍️schema-validate.py` gained two honest fixes: it honours `🔺️diff/🚫️.absent` instead of reporting a
refusal's missing diff as a gap, and it validates a `🦠️*.json` under `🧫️fixtures/` against the mutation
schema rather than the snapshot schema.

**cargo: run pending.** No `🗑️generated/w11-check-*.txt` exists, so the gate never opened; nothing was
compiled from this lane. The 132 case modules are asserted structurally (mount-check) and semantically
(the TypeScript twin replays every one of them byte for byte), not by rustc.

## 5. Disagreements and findings

**F1 — the TypeScript twin printed every `f32` field at the wrong width (fixed here).** W9 §7c called
the `serde_json` → `pack::json` swap byte-for-byte and listed two deltas; there is a third. Rust's
`impl ToValue for f32` widens with `*self as f64` (`🌱️value/🔁️codec/🦀️.rs:114`) before `pack::json`'s
f64 shortest-round-trip writer sees it, so `0.42f32` now reaches the wire as `0.41999998688697815`,
where `serde_json::serialize_f32` used to write `0.42`. The twin's `writeValueJson` still used the
ryu-f32 rule. The committed toy scene never caught it because every one of its f32 values is exactly
representable (`0.0625`, `0.5`, `12.5`); the real-world scene has 40+ that are not. Confirmed against
the Rust-printed `📚️examples/🛰️synthetic-orbit/🖼️assets/🗣️.dsl.semio`, which already carries
`-0.019999999552965164` and `ratio-test=0.8500000238418579`. Fixed in
`🧬️schema/📸️snapshot/🟦️.ts` (`case "f32"` and the tuple writer now emit at width 64, with the reason
in `floatLexeme`'s docstring). **This is a wire-format change that reaches the IO serializers too**
(`🚪️io/📤️export/…/🔣️json/`), and every remodel `.json`/`.dsl.semio` carrier written by any language
since W9 landed is affected — W6b/W11 should know.

**F2 — 14 applied vectors have no inverse, and two of them are pre-existing.** Every
`↩️inverse/🦀️.rs` in this vocabulary is BASE-derived and APPEND-shaped, so a member removed from
anywhere but the end comes back at the end. The generator computes production's own inverse and only
commits an `inverse-` scenario where it really restores; the 14 that do not each pin that in their own
leaf module (`inverse_does_not_restore_the_before_document`) rather than leaving the law unasserted:

| vector | why production's inverse does not restore |
|---|---|
| `delete-stream` (toy, realworld, first) | the single `create-stream` step never re-connects the GCP observations the cascade severed — and the toy vector DOES sever one, so `inverse-delete-stream` as committed would have failed on its first real run |
| `create-asset` (toy, realworld, upsert) | the `delete-asset` inverse removes `assets[key]` but leaves behind the `durableArtifacts[childId]` leaf the forward step minted; the toy vector has the same gap |
| `add-stream-frame` (realworld) | the `remove-stream-frame` inverse does not un-rewrite `stream.kind`, which the forward verb also writes |
| `remove-stream-frame` (realworld, first), `remove-gcp-observation` (realworld, first), `delete-gcp` (realworld), `delete-camera-calibration` (referenced), `delete-rig-extrinsic` (first) | the member returns at the END of its list, not at the position it was removed from (`taxonomy.md` rule 5 requires the position) |

The Python reference states the *correct* inverse for these (it re-connects the cascade and carries
explicit `__move-*` steps), so the two implementations genuinely disagree; the disagreement is
recorded here and the scenario withheld rather than papered over. Repairing the eight
`↩️inverse/🦀️.rs` leaves is outside this lane's file grant.

**F3 — `delete-camera-calibration` has no referential guard at all.** Deleting a camera that a stream
binds and two rig entries name is accepted silently, with no cascade note — unlike `delete-stream` and
`delete-gcp`, which both report theirs. Pinned by
`🚫delete-camera-calibration/🧪️tests/⛓️removes-a-camera-ee868c`.

**F4 — `update-*-params` guard order is inconsistent between the eight blocks.** `ingest`, `feature`,
`matching` and `geo` check their invariant BEFORE the identical-resubmission warning; `sfm`, `dense`,
`mesh` and `motion` check the warning first. Both orders are now pinned by vectors, but only one of
them can be the intended contract.

**F5 — `🎯️outcome` used two vocabularies.** The TS oracle asserted `"refused"`; the repo-wide fixture
manifest vocabulary (and 93 sibling fixtures) says `"rejected"`. Standardised on `"rejected"`.

**F6 — follow-up, not mine.** `.storybook/stories/remodel/fixture.ts` transcribes the old toy
before-document verbatim (its own comment says so) and is now stale on `durableArtifacts` and on
`stream-a`'s source resolution. It uses a story-local type, so nothing breaks — but W5's populated
scene no longer matches the fixture it claims to mirror.

## 6. Files

Generators (ticket, kept): `🐍️w2c-bases.py`, `🐍️w2c-diff.py`, `🐍️w2c-vectors.py`, `🐍️w2c-generate.py`.
Modified validators (ticket): `🐍️schema-validate.py`.

Repo, under `✏️s/🔌️plugins/📸️remodel/`:
`🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/` — `🧬️schema/🧬️mutations/*/🧪️tests/**`
(132 cases, 800 files), `🧬️schema/📸️snapshot/🟦️.ts` (F1), `🧬️schema/🧪️tests/🟦️.ts`,
`🧪️tests/📸️mutate-remodeling-1/{🥒️.feature,🦀️.rs,🐍️.py}`, `🧫️fixtures/🏁️commit-reconstruction/**`,
`🔮️oracle/🔣️.json`; and `📦️packages/🦀️rust/🦀️.rs` (mounts only).
