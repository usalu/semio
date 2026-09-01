# 🛠️ Fleet report — wave 1 (stdio mutation-leaf migration)

15 Sonnet agents, 41 artifacts, plus the earlier mp3 pilot / csv repair / drawing-helper repair.

## Headline

A large fraction of the 41 artifacts turned out to be **already migrated** when their agent arrived —
a parallel sweep (peer session, uncommitted working tree) had done the aggregate + leaf folders but
had **not** fixed the fallout. So most agents' real work was the second half of the recipe:
struct-literal construction sites and `NoMutation` references in `🧪️tests/mutate-*/🦀️.rs` adapters,
`✏️editor/`, `🚪️io/`, `👁️viewer/`, `🧪️oracle/🦀️component.rs`, and cross-subset call sites.

| state on arrival | artifacts |
|---|---|
| already migrated, fallout only | las, ply, stl, html, epw, zip ✳️any, zip ✳️iso21320, gif 87a, gif 89a, pptx, xlsx, mp4, svg ✳️basic, svg ✳️tiny, ifc 2x3, bcf, tsv, json ✳️i-json, dwg, semio ✳️animation, ✳️cad, ✳️model, ✳️document, ✳️drawing |
| aggregate still hand-rolled | xml ✳️valid, md, deflate, binary, docx, avi, wav, ifc 4, step, dxf, obj, semio ✳️any, ✳️value, ✳️audio, ✳️video, ✳️image, ✳️presentation, ✳️flow |

## Cross-cutting decisions taken centrally

**`no-mutation` scenarios.** The retired `NoMutation` variant is still a live scenario id in the
`.feature` files and `🧪️oracle/🔣️.json` catalogs. Agents initially diverged — some let it fall
through to `Err`, some mapped it to an identity mutation. Ruling issued mid-wave and applied from
then on: map it to `XMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })`,
keep the scenario, never `Err`. Where an inverse has nothing to restore, return `Vec::new()`.
Artifacts migrated before the ruling (las, ply, stl, html, xml, md, deflate, binary, docx, mp4, avi,
wav, step, json) need a consistency sweep — see wave 2.

**`#[serde(tag = "mutation")]`.** tiff omits it; most artifacts keep it. Keeping it is correct:
internally-tagged serde flattens a newtype-of-struct identically to the old struct-variant, so
committed JSON fixtures stay byte-identical. Removing it would silently change the wire format.

**Leaf payload field visibility.** Several sweeps left leaf fields fully private, which cannot work —
`agg_diff`/`agg_inverse` live in the parent module and destructure them. Agents fixed these to
`pub(crate)` (or `pub` where an external test-host crate constructs them).

## Defects found in the reference implementation itself

`🖼️tiff 6.0 ✳️baseline` — the artifact the whole recipe was derived from — has a stale `KINDS`:
it still lists `"no-mutation"` first, so `KINDS.len()` is 9 against 8 enum variants and its own
`kinds_match_enum_variants_in_declaration_order` test cannot pass. Four independent agents flagged
it. Fixed centrally rather than imitated.

## Judgment calls flagged for review

1. **`✳️any`'s envelope leaves** — `semio ✳️any` wraps 18 *other subsets'* mutation enums. `MutationLeaf`'s
   derive validates file locality, so those foreign enums cannot be leaves directly. A thin
   delegating leaf per subset was introduced (`pub struct Brep { pub(crate) mutation: SemioBrepMutation }`).
   This has no precedent elsewhere in the repo and is the wave's one genuine design decision.
2. **obj kept `dsl::DslOps` alongside `dsl::Mutations`**, giving each leaf `#[derive(dsl::DslRecord)]`,
   because committed `.grammar.semio`/`.protocol.semio` files and two conformance laws pin its wire
   format. Precedent found in `🔱️trinity/♻️rewrite` and `📸️remodel`.
3. **deflate and binary went the other way** — their `DslOps` could not survive the leaf shape, so
   their `OpText`/`OpBinary` were re-hand-rolled on `serde_json`, mp3-style. Wire format changed.
   These two are the only artifacts where that happened and they should be reviewed together with (2).
4. **`SetTextBoxBlocks`** — the derive's `to_kebab` yields `set-text-box-blocks`, not the leaf's
   original `set-textbox-blocks`. The descriptor identity was changed; the op-text keyword was left
   at its established spelling.
5. Several artifacts' strict `assert_eq!` of `KINDS` against the oracle manifest were loosened to
   containment, because the manifests legitimately still declare `no-mutation`.
