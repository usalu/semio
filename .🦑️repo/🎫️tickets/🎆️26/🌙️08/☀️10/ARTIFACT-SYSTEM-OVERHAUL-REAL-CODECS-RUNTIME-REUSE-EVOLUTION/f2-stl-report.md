# F2 — stl (standard: ascii) — Fan-out Report

Wave: F2 (stl, obj, ply, las, bmp, tiff — parallel). Agent scope: exactly
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/**` + this report. No `📦️glue.rs`, `📜️script.ts`, SDK,
schema module, io module, or `🏪️store` edits (none needed or made).

## 1. What changed

### 1.1 Snapshot — killed the shared `MeshVertex`/`MeshTriangle` indexed-mesh type

W0 confirmed `MeshVertex`/`MeshTriangle` were byte-identical types independently defined in both
stl and ply (not literally imported from one shared module, but structurally the exact
"generic code" the recipe targets for elimination). Replaced with `StlSnapshot`'s own,
format-accurate model per the brief:

```rust
pub struct StlTriangle { pub normal: [f64; 3], pub vertices: [[f64; 3]; 3] }
pub struct StlSnapshot { pub schema: String, pub solid_name: String, pub triangles: Vec<StlTriangle> }
```

This is a real snapshot redesign, not just a rename: ASCII/binary STL triangles are NOT an
indexed mesh (no shared vertex pool) — every facet carries its own 3 vertices independently, so
`StlTriangle` owns its vertices directly rather than referencing indices into a separate
`vertices: Vec<MeshVertex>` array. `solid_name` is a genuinely new field: the old snapshot
silently discarded the `solid <name>`/`endsolid <name>` header/trailer entirely.

### 1.2 Codec honesty fix (beyond the brief, required for a complete snapshot)

The **old** `⚙️engine` codec never persisted the real per-facet `facet normal` value — it always
recomputed the normal from vertex winding on encode, silently discarding whatever was actually
written in the file (a real-world "lazy writer" pattern: `facet normal 0 0 0` is common and
legitimate). Since the new `StlTriangle` snapshot has a genuine `normal` field, honesty requires
persisting exactly what's read and never silently rewriting it on encode — this is now the
codec's behavior (`decode_stl_ascii`/`encode_stl_ascii`, `decode_stl_binary`/`encode_stl_binary`
all round-trip the real normal). Covered by a dedicated regression test,
`ascii_facet_normal_is_persisted_not_recomputed`.

Binary STL's 80-byte header (spec-opaque) is now used as `solid_name` (trimmed of trailing
NUL/whitespace) — matching common real-world binary-STL-writer convention — instead of being
silently dropped. `f64` (snapshot) <-> `f32` (binary wire) narrowing/widening is documented as a
lossy, spec-mandated normalization, not fabrication.

### 1.3 Diff — handcrafted sparse struct, index-keyed triangles triple

```rust
pub struct StlDiff { solid_name: Option<String>, triangles: Option<StlTrianglesDiff> }
pub struct StlTrianglesDiff { removed: Vec<usize>, modified: Vec<StlTriangleModified>, added: Vec<StlTriangleAdded> }
pub struct StlTriangleDiff { normal: Option<[f64;3]>, vertices: Option<[[f64;3];3]> } // whole-value replace per field
```

No `snapshot: Option<StlSnapshot>` full-replace slot anywhere, including `SetSnapshot` (its diff
is `StlDiff::between(base, next)`, same machinery every other mutation's diff composes from).

**Absorb** (the hard part for an index-keyed, flat/unkeyed collection): implemented via the same
label-simulation technique as this ticket's `txt` artifact (`TxtLinesDiff`'s `absorb_pair`) —
walks a virtual `Lbl::Base(0..l1)` array through `d1`'s then `d2`'s remove/insert position algebra,
reading back `removed`/`modified`/`added` from which labels survive and where. Unlike `txt` (whose
line value is a single scalar so `d2` LWW-overwrites `d1`'s pending text), a per-base-survivor
`modified` patch is a genuine **recursive per-field absorb** (`absorb_triangle_diff`, LWW per
field) between `d1`'s and `d2`'s patches — matching the recipe's "d2 patch on a surviving base
item recursively absorbs into the matching m1 entry" contract more precisely than a whole-value
overwrite would.

Verified this exact algorithm standalone in a scratch crate (`stl_scratch`, this ticket's session
scratchpad) before porting — all canonical cases plus associativity passed prior to compiling in
the real crate; see §3.

### 1.4 Mutations — 7 variants, all diff()/inverse() handcrafted

`NoMutation`, `SetSnapshot`, `SetSolidName`, `InsertTriangle{index,triangle}`,
`RemoveTriangle{index}`, `SetTriangleNormal{index,normal}`, `SetTriangleVertices{index,vertices}`
— exactly the brief's vocabulary. Every variant's `diff()` calls a handcrafted `schema::diff`
builder (never apply-and-capture); every variant's `inverse()` looks the prior value up in
`base`, degrading to `NoMutation` for a stale/out-of-range index (graceful no-op, matching the
recipe's apply contract).

### 1.5 Facet mirrors + grammar leaves

Handcrafted honestly for `snapshot`/`diff`/`mutations` (ts/graphql/json/proto at the facet level,
plus the artifact-level `🧬️schema/` facet). Grammar leaves: fully handcrafted all 7 leaf types
(`.g4`/`.ebnf`/`.grammar.semio` under `📝️text/`, `.abnf`/`.protocol.semio`/`.ksy`/`.spicy` under
`💾️binary/`) for the **snapshot** facet (real ASCII-STL grammar + real binary-STL protocol
description, both matching the actual codec). For **diff**/**mutations** facets, matched this
wave's established precedent (confirmed against `zip`'s F1 output): handcrafted only the 2
live-wired leaves per facet (`📖️component.grammar.semio` under text, `📡️component.protocol.semio`
under binary — the ones `register_pilot_languages` actually references), describing the real
`serde_json`-line op-text/op-binary wire shape. The 4 un-wired sibling leaves per facet
(`.g4`/`.ebnf` under diff's/mutations' `text/`, `.abnf`/`.ksy`/`.spicy` under their `binary/`) were
left as the pre-existing placeholder — same documented deviation zip's closer accepted for F1
(these are OpText/OpBinary/DiffCodec-facet grammars, explicitly the program's final wave per the
plan's user decision #2).

### 1.6 A pre-existing dead triad-dir file needed a signature fix

`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (the inert, unused-by-anything-real
`set-snapshot` triad scaffold S2's report documented) called the OLD single-argument
`diff_set_snapshot(snapshot)`. Updated its own signature to take `(base, snapshot)` matching the
new 2-arg `diff_set_snapshot`. This file is inside my own artifact's tree
(`🗿️artifacts/🟪️stl/**`), so in-scope to fix directly (no `glue_followup` needed — same file
already mounted).

## 2. Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::stl"` → **21 passed, 0 failed.** All 6 law
suites present: `mutation_diff_law`, `inverse_law`, `absorb_law` (+ `absorb_law_associativity`),
`between_roundtrip_law`, `codec_retention_law`, `field_sweep_covers_every_mutable_field` (name
contains `field_sweep`). Plus the recipe's 3 canonical absorb cases as standalone structural
tests: `insert_then_remove_before_matches_canonical_shape`,
`insert_insert_same_index_both_survive`, `add_then_set_field_patches_into_added`.

Grep gates: `snapshot: Option<` — 0 hits in the diff struct (1 hit only in a doc-comment
describing what was REMOVED). `impl DiffAlgebra` — present. `field_sweep` test — present, name
contains the substring.

**`field_sweep` structural design** (the F1 txt-bug trap this ticket flagged): `sweep_a`/
`sweep_b` use **asymmetric** triangle-list lengths (2 vs 3), and assertions are split across both
`between()` directions — `between(a,b)` proves `modified`+`added` (b is longer), `between(b,a)`
proves `modified`+`removed` (a is now the shorter side) — exactly the documented fix pattern, not
the impossible "all three from one call" shape that broke txt's original test. Caught and fixed
one real bug of my own here during verification: my first `sweep_a`/`sweep_b` fixtures gave the
modified triangle the same "seed" in both snapshots, so only `normal` differed and `vertices`
never got exercised — fixed by using different seeds, confirmed the assertion actually fails
without the fix (`vertices must be diffed` panic) before landing the correction.

**Absorb algorithm pre-verified in isolation**: before porting into the real crate, wrote a
standalone scratch crate (`stl_scratch` in this session's scratchpad, `cargo run`) reimplementing
`apply`/`between`/`absorb_pair` verbatim and re-ran the exact canonical cases + associativity +
field-sweep-shaped scenarios — all passed before the real crate even compiled, giving early
confidence the label-simulation port was correct (later confirmed identically in the real crate's
own tests).

**Full-crate gate**: `cargo test -p semio-s-plugin-stdio --lib` → **795 passed, 0 failed**,
crate-wide, at time of this report (includes all of F1's work plus whatever F2's other 5
concurrent agents — obj/ply/las/bmp/tiff — had landed by the time this closed out). Mid-session
the crate was blocked by clearly-external, actively-changing concurrent churn in `☁️ply`,
`☁️las`, and briefly `🧊️obj` (`git status` showed those files under active modification —
`M`/`??` — by other sessions throughout; error signatures were exactly "their own artifact's
fields/functions don't exist yet", never anything in a `🟪️stl` file); polled several times over
roughly 8 minutes until those cleared naturally, per the ticket's "poll, don't chase" guidance.
Zero of the ~40+ compile errors seen at any point during that window were in any `🟪️stl` path.

**Policy check** (`bun ./📜️script.ts policy`): filtered the regenerated
`.🦑️repo/⚡️cache/breaches/compose.json` for the 4 new S-8 rule kinds
(`stdio-artifacts/{diff-algebra,field-sweep-presence,grammar-honesty,facet-mirror-drift}`) scoped
to `🟪️stl`: **13 breaches, all 13 confirmed `low`-priority/stale** (the S2-seeded allowlist
entries describing the pre-this-wave placeholder state — real content now supersedes them; a
future wave's closer prunes these, per the F1 precedent). **Zero real (non-stale) breaches.**

## 3. Deviations from a maximal reading of the brief

1. **Diff/mutations facets' 4 un-wired sibling grammar leaves per facet** (`.g4`/`.ebnf` under
   `text/`, `.abnf`/`.ksy`/`.spicy` under `binary/`) were left as the pre-existing placeholder,
   matching zip's F1-accepted precedent — these describe the OpText/OpBinary wire format, which
   the plan's own user decision #2 scopes to "the final wave of this program", not F-wave scope.
   The 2 live-wired leaves per facet (`grammar.semio`, `protocol.semio`) ARE handcrafted for both
   diff and mutations, beyond the strict minimum (only snapshot was required by a literal reading
   of "your main job is diff/mutation design, not snapshot enrichment").
2. `StlTriangle::normal`/`StlTriangle::vertices` use whole-value replacement in `StlTriangleDiff`
   (no sub-diffing of individual `f64` components) — explicitly sanctioned by the brief
   ("whole-value replacement for the fixed-size arrays is fine").
3. Did not touch `☁️ply`'s own `MeshVertex`/`MeshTriangle` — per the brief and W0's finding, each
   artifact's own agent kills its own copy independently; ply's is that artifact's own F2 agent's
   responsibility (confirmed via `git status` that ply's own agent was actively mid-refactor on
   its own snapshot during this session).

## 4. `glue_followup`

None. No new top-level directory was needed — all real work (new snapshot/diff/mutation fields,
new mutation variants, absorb logic) fit inside already-mounted
`🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs`, `⚙️engine/🦀️component.rs`, and sibling
facet leaves, per S2's Task 1 resolution.

## 5. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/⚙️engine/🦀️component.rs` — codec rewrite (solid_name, persisted normal, f64, binary header-as-name), tests updated/added.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `StlTriangle`/`StlSnapshot` redesign.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — full handcrafted `StlDiff`/`StlTrianglesDiff`/absorb rewrite.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — full mutation vocabulary + all 6 law-suite tests + canonical-case tests.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `StlArtifact` field rename to match snapshot.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — signature fix for the new 2-arg `diff_set_snapshot`.
- Facet mirrors (handcrafted, real content): `🧬️schema/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` and the same 4 files under `📸️snapshot/`, `🔺️diff/`, `🧬️mutations/`.
- Grammar leaves (handcrafted, real content): all 7 leaf types under `📸️snapshot/{📝️text,💾️binary}/`; the 2 live-wired leaves (`📖️component.grammar.semio`, `📡️component.protocol.semio`) under `🔺️diff/{📝️text,💾️binary}/` and `🧬️mutations/{📝️text,💾️binary}/`.
- This report.
- Scratch (session scratchpad, not in repo): `stl_scratch/` (Cargo.toml + src/main.rs) — standalone absorb-algebra verification crate.

## 6. Summary

`artifact: stl`, `standards: [ascii]`, `tests_passed: 21`, `tests_failed: 0` (own filter);
`full_crate_passed: 795`, `full_crate_failed: 0`; `field_sweep_present: true`;
`laws_present: [mutation_diff_law, inverse_law, absorb_law, absorb_law_associativity,
between_roundtrip_law, codec_retention_law, field_sweep_covers_every_mutable_field]`;
`policy_shrink: 13/13 stl-scoped S-8 breaches stale, 0 real`; `glue_followup: []`.
