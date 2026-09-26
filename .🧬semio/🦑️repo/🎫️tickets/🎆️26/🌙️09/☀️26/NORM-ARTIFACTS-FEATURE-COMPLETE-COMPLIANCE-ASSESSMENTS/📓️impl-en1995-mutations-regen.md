# 🪵️ EN 1995 — mutation vocabulary regeneration for the redesigned `TimberMember` / `TimberConnection`

Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/` only. Everything below was produced by the
deterministic generator `🐍️regen-en1995-mutations.py` (kept in this ticket folder) plus a handful of
hand edits listed at the end. Re-run with `python3 🐍️regen-en1995-mutations.py` from the repo root
whenever the snapshot shape changes again.

## 1. Vocabulary (66 leaves, `binaryTag` = index, `kind` = `to_kebab(variant)`)

| tag | variant | kind | dir |
|---:|---|---|---|
| 0 | `ChangeAnnex` | `change-annex` | `🌍️change-annex` (kept, files untouched) |
| 1–2 | `InsertMember` / `RemoveMember` | `insert-member` / `remove-member` | `➕️…` / `➖️…` |
| 3–29 | `ChangeMember{LabelEn,LabelDe,Role,StrengthClass,ServiceClass,Support,B,H,Span,SupportLength,BearingLength,BucklingY,BucklingZ,LateralRestraint,NotchDepth,NotchDistance,MCrit,MassPerM,MassPerM2,Damping,FireDuration,BridgeNObs,BridgeTLYears,BridgeBeta,BridgeA,BridgeB,BridgeCrowd}` | `change-member-…` | emoji + kind |
| 30–31 | `InsertMemberAction` / `RemoveMemberAction` | `insert-member-action` / `remove-member-action` | |
| 32–41 | `ChangeMemberAction{Kind,Category,LoadDuration,QLine,FPoint,MK,VK,NK,NTK,FC90K}` | `change-member-action-…` (`…-mk`, `…-fc90-k`) | |
| 42–43 | `InsertConnection` / `RemoveConnection` | | |
| 44–60 | `ChangeConnection{LabelEn,LabelDe,FastenerType,StrengthClass,ServiceClass,Diameter,Number,Rows,Spacing,EdgeDistance,EndDistance,T1,T2,SteelPlate,SteelPlateThickness,ShearPlanes,FUK}` | `change-connection-…` (`…-fuk`) | |
| 61–62 | `InsertConnectionAction` / `RemoveConnectionAction` | | |
| 63–65 | `ChangeConnectionAction{Kind,LoadDuration,FK}` | `change-connection-action-{kind,load-duration,fk}` | |

Payloads: scalar member/connection leaves `{ member_id|connection_id, new_value }`; action leaves
`{ member_id|connection_id, action_id, new_value }`; insert/remove of collections `{ index[, item] }`;
insert/remove of actions `{ member_id|connection_id, index[, action] }`. Every leaf has `🔣️.json`
(14-key descriptor), `🧬️schema/🔣️.json`, `🦀️.rs`, `🔺️diff/🦀️.rs`, `🔁️inverse/🦀️.rs` and one named
scenario test under `🧪️tests/<scenario>/🦀️.rs` (apply → assert field → `assert_ne!` → inverse
replays to base → `assert_op_line_round_trip`).

Removed stale leaf dirs (old shape): `↕️change-member-v-ed`, `⏳️change-connection-load-duration`,
`⏳️change-member-load-duration`, `⤴️change-member-m-ed`, `🏋️change-member-f-c90-ed`,
`🏋️change-member-n-ed`, `🏋️change-member-n-t-ed`, `📐️change-member-w-inst`,
`🔁️change-member-bridge-n-cycles`, `🔢change-connection-{number,rows,shear-planes}`,
`🔢change-member-psi2`, `🔩change-connection-{fastener-type,steel-plate}`, `🔩️change-connection-f-ed`,
`🛡️change-connection-f-u-k`, `🦶️change-member-floor-a-vert`.

## 2. Files written / rewritten (all under `🏅️standards/🔖️1/🪆️subsets/✳️any/` unless noted)

- `🧬️schema/🧬️mutations/<leaf>/…` — 65 regenerated leaves (see above).
- `🧬️schema/🧬️mutations/🦀️.rs` — `En1995Mutation` (66 variants, derive order = `KINDS` order), `KINDS`,
  `from_snapshot` (annex, member/connection remove→insert→per-field change, nested action remove→insert→change).
- `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — `every_mutation()` covering all 66 variants,
  label smoke test, `from_snapshot_reaches_every_variant`, `from_snapshot_carries_between_examples`
  (all four snapshot examples pairwise).
- `🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs` — mounts the 66 scenario tests (snake_case module names).
- `🧬️schema/🧬️mutations/📝️text/🦀️.rs` + `📝️text/🧪️tests/🔬️unit/🦀️.rs` — demo cases and
  `every_mutation` text round trip for all variants.
- `🧬️schema/🧬️mutations/🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto` — mirrors of the vocabulary.
- `🪵️en1995/🦀️.rs` (crate root) — mount block between `pub use component::*;` and `pub mod text;`
  regenerated: exactly one `#[path]` mount per leaf, `binary` + `text` once at the end; editor/viewer
  mounts untouched. Also removed five `#[dsl(unit = …)]` attributes whose symbols the DSL token table
  does not know (`N·m` ×2, `N/m`, `kg/m`, `kg/m²`, `1/m²`) — they panicked every RecordSpec-law test
  (`dsl: unknown unit symbol 'N·m'`). Those fields serialize as plain `NUM` now (as before the
  redesign). If newton-metre etc. should be first-class units, add them to
  `🧰️framework/…/🗣️dsl/🔤️token/🦀️.rs::UNITS` and restore the attributes.
- `✏️editor/🏷️field-meta/🦀️.rs` — human en/de choice labels (strength classes, service classes,
  role, support, action kind, category incl. `""`, load duration, fastener type, bool); every editable
  leaf path with `[]` wildcards (`members[].bM`, `members[].actions[].qLineNPerM`,
  `connections[].actions[].fKN`, …) with en+de label and SI unit; test
  `every_editable_leaf_has_meta_with_both_labels`.
- `🔮️oracles/🔣️.json` — `mutationCatalogs[en1995-1-any].vectors` and
  `mutationManifests[s.norm.en1995].mutations` rewritten for the 66 kinds (kinds-catalog test green).
- `🧫️fixtures/🧬️mutations/🌍️change-annex/🌍️switches-…/📸️snapshot/{⬅️before,➡️after}/🔣️.json` —
  new-shape `compliant_building_beam()` JSON.
- `🖼️assets/🌉️glulam-footbridge/**` and `🖼️assets/❌️multi-fail-timber/**` (`🎒️.pack.semio`,
  `🗣️.dsl.semio`) — regenerated from the new snapshots via
  `EN1995_REGEN_PACKS=1 cargo test -p semio-s-artifact-norm-en1995 regen_example_pack_assets`.
- Plugin fixture `✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` —
  regenerated (`bun nx run @semio-tech/norm-plugin:mutation-leaf-taxonomy-generate`); 66 en1995 rows.

Hand edits for the example renames (`compliant_glulam_beam` → `compliant_building_beam`,
`noncompliant_multi_fail` → `noncompliant_building`) and the new action model:
`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (shear-divergence test now feeds a `CharacteristicAction`),
`🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, `🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`,
`🧬️schema/📸️snapshot/🟦️.ts`, `🧬️schema/🟦️.ts`, `🧬️schema/🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs`
(+ mount appended to `🔺️diff/📝️text/🦀️.rs`).

## 3. Test state

`cargo test -p semio-s-artifact-norm-en1995` final run: **147 passed, 1 failed**; the crate's own
warnings are down to 4, none in generated or hand-edited files:

- ✅ all 66 leaf scenario tests, unit `from_snapshot` round trips, kinds-catalog, field-meta, text
  and binary round trips, DSL example fixtures, editor/panel/window tests.
- ❌ `python_oracle_matches_rust_utilizations_within_half_percent` —
  `🔮️oracles/🐍️evaluate.py` still reads the removed member fields (`KeyError: 'mEdNm'`). The Python
  oracle mirrors `⚖️timber/🦀️.rs` (action combinations, ψ-factors, support-type internals, bridge
  fatigue) and belongs to the evaluation redesign, not the mutation vocabulary — **left for the
  owner of `⚖️timber`**.

Mid-run a concurrent change to `app_surface::render_catalogue` (5 arguments) briefly broke the
en1995 catalogue panel; its owner updated the caller a couple of minutes later and the final run
above is against that state. The catalogue panel was not touched here.

## 4. Left for the parent / out of scope

- `🔮️oracles/🐍️evaluate.py` — rewrite for the action-based member model (see above).
- `🧪️tests/🪵️mutate-en1995-1/🦀️.rs` (sut-gated, not compiled by this crate) still lists the old
  20-kind vocabulary.
- `✏️s/🔌️plugins/📕️norm/🧬️schema/🔣️.json` pins `NormMutationLeafTaxonomy.rows` to
  `minItems`/`maxItems` = 392 (and the 392 in its description). The regenerated census now has
  **537 rows** across all families (other families grew concurrently), so
  `mutation-leaf-taxonomy-check` fails on `maxItems`. The pin must be set once all families are done —
  not a per-family number, so it was not changed here.
- `#[dsl(unit)]` symbols for N·m, N/m, kg/m, kg/m², 1/m² (framework unit table), if wanted.
