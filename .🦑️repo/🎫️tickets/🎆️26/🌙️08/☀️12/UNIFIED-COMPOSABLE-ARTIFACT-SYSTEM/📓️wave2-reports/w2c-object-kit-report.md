# W2c — `✳️object` and `✳️kit` (stdio, the two COMPOSITE subsets)

**`ucas-status: object complete, kit complete`**

Scope: author the final two subsets of the `s.stdio.semio` roster inside
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`. These are the **first two
subsets in the whole ticket that carry real `store::ArtifactChild<S>`/`store::ArtifactLink`
composition fields** — every subset authored before this wave (`text`/`table`/`graph`, and the 14
leaves before them) is a pure leaf with no child/link slots.

Union count: 16 domain subsets + `✳️any` → **18 domain subsets + `✳️any`** after this wave.

## What was built

### `✳️object` (130 files) — one spatial thing

`SemioObjectSnapshot { schema, transform: SemioTransform, brep: Option<ArtifactChild<SemioBrepSnapshot>>,
mesh: Option<ArtifactChild<SemioMeshSnapshot>>, properties: Option<ArtifactChild<SemioValueSnapshot>> }`

- `🧬️schema/📸️snapshot/🦀️component.rs:33-50` — the snapshot struct with three `#[child(kind = "…")]`
  slots (`brep`→`s.stdio.semio.brep`, `mesh`→`s.stdio.semio.mesh`, `properties`→`s.stdio.semio.value`),
  each `Option<store::ArtifactChild<S>>` (a spatial thing may carry zero-to-one of each geometry
  representation plus zero-or-one property tree). `transform` reuses the ALREADY-EXISTING shared
  `engine::geometry::SemioTransform` (`translation`/`rotation`/`scale`) rather than inventing a new
  transform type. Hand-rolled hex/bracket `ArtifactDsl`/`ArtifactPack` (`:65-268`), real child-handle
  codec primitives (`enc_child`/`dec_child` — exactly `[<hex child_id>,<hex target-uri>]`, never
  content), `demo_object_snapshot()` with all three children populated.
- `🧬️schema/🔺️diff/🦀️component.rs` — `SemioObjectDiff { transform: Option<SemioTransform>, brep:
  Option<Option<ArtifactChild<…>>>, mesh: Option<Option<ArtifactChild<…>>>, properties:
  Option<Option<ArtifactChild<…>>> }` — per-field `Option<…>` slots (image's own convention), with
  the CHILD fields DOUBLE-`Option`'d: outer = "diff touches this slot", inner = "the new value,
  possibly None to clear it". `impl MutationDiff`/`impl protocol::command::DiffAlgebra`, hand-rolled
  `DiffCodec` (`;`-joined `tag=value` fields).
- `🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum, **9 triads**, real vocabulary (below).
- `🚪️io/🦀️component.rs` — composer + `SemioObjectValidator` (decode PLUS a REAL referential check:
  every present child handle's `target.dialect` must name the kind its slot declares — `object` is
  the first subset with something genuinely cross-referential to validate at decode time, unlike
  every leaf's decode-only validator) + `register()` + the 6 conformance-law tests.
- `📚️examples/📦️crate/` — new example dir (checked all 19 existing sibling names first), real
  `print_dsl`/`encode_pack` fixture bytes (380/267 bytes DSL/pack), captured via a temporary
  `[DEBUG]`-prefixed `debug_dump_fixture_bytes` test, removed after capture.

### `✳️kit` (166 files) — semio's type/design domain

`SemioKitSnapshot { schema, types: Vec<SemioKitType>, designs: Vec<SemioKitDesign>, objects:
Vec<ArtifactChild<SemioObjectSnapshot>>, models: Vec<ArtifactChild<SemioModelSnapshot>>, properties:
Option<ArtifactChild<SemioValueSnapshot>>, representations: Vec<ArtifactLink> }`

- `🧬️schema/📸️snapshot/🦀️component.rs:74-102` — the snapshot struct. **Two owned CHILD collections**
  (`objects`→`s.stdio.semio.object` many, `models`→`s.stdio.semio.model` many), **one optional owned
  CHILD slot** (`properties`→`s.stdio.semio.value`), and **one LINK collection**
  (`representations`, `#[link_slot(roles("representation"))]`) — the FIRST facet in this whole
  ticket to declare a real link slot (`object` has none). `SemioKitType { id, name, category }`
  (`:33-37`) and `SemioKitDesign { id, name, pieces: Vec<SemioKitPiece>, connections:
  Vec<SemioKitConnection> }` (`:45-70`) are plain in-document value collections, not composition
  primitives — a design's `pieces` reference a `type_id` by string, a piece's own geometry stays
  wherever `objects`/`models` puts it. **Design decision**: `representations` (link pool) joins to
  `SemioKitType` by `role == type.id` — documented in the module's own doc comment (`:9-14`) — rather
  than adding a second per-type field, since the derive only classifies fields on the TOP-LEVEL
  snapshot struct, never on a value type nested inside a `Vec`.
- `🧬️schema/🔺️diff/🦀️component.rs` — `SemioKitDiff` with SIX `Option<…>` fields (one per composition/
  value collection), five backed by whole-list wrapper newtypes (`SemioKitTypeList`/
  `SemioKitDesignList`/`SemioKitObjectChildList`/`SemioKitModelChildList`/`SemioKitLinkList`, same
  `RunList` shape `✳️text` uses) plus the double-`Option` child slot for `properties`.
- `🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum, **15 triads**, real vocabulary (below).
- `🚪️io/🦀️component.rs` — composer + `SemioKitValidator` (decode + real kind-check on
  `objects`/`models`/`properties`; `representations` deliberately NOT kind-checked — a link may point
  at any independent artifact kind, there is no single expected kind to assert) + `register()` + 6
  conformance-law tests.
- `📚️examples/🪑️furniture/` — new example dir, real fixture (734/498 bytes DSL/pack), same
  debug-dump-then-delete capture method.
- Absorbs (by design, not yet by code — see "Out of scope" below) the duplicated `kit.catalog`
  artifact kind puzzle/three-block apps currently declare separately (`objectKinds`/`vortexKinds`
  fragments with `representations`/`kindCompatibility` — confirmed by reading
  `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`'s `kit:in` port).

## Child-slot / link-slot declarations and why

| Subset | Field | Primitive | Kind / roles | many | Why |
|---|---|---|---|---|---|
| object | `brep` | `ArtifactChild<SemioBrepSnapshot>` (Option) | `s.stdio.semio.brep` | no | precise b-rep representation, owned |
| object | `mesh` | `ArtifactChild<SemioMeshSnapshot>` (Option) | `s.stdio.semio.mesh` | no | tessellated/preview representation, owned; independent of `brep` (an object may carry both at once) |
| object | `properties` | `ArtifactChild<SemioValueSnapshot>` (Option) | `s.stdio.semio.value` | no | property-set tree, owned |
| kit | `objects` | `Vec<ArtifactChild<SemioObjectSnapshot>>` | `s.stdio.semio.object` | yes | owned example/instance spatial things |
| kit | `models` | `Vec<ArtifactChild<SemioModelSnapshot>>` | `s.stdio.semio.model` | yes | owned example/instance BIM elements |
| kit | `properties` | `ArtifactChild<SemioValueSnapshot>` (Option) | `s.stdio.semio.value` | no | kit-level shared property set |
| kit | `representations` | `Vec<ArtifactLink>` | roles `["representation"]` | yes | independent-lifecycle geometry references, reused across catalogs — the only LINK slot in the whole wave |

Both subsets' snapshot AND top-level artifact structs (`SemioObjectArtifact`/`SemioKitArtifact`)
carry byte-identical `#[child]`/`#[link_slot]` attributes (mirrors, per convention) — never
hand-written slot tables anywhere; `#[derive(ArtifactSchema)]` emits `child_slots()`/`link_slots()`
from these attributes.

## Mutation-vocabulary decisions

### `object` — 9 triads, no link verbs (no link slots exist)

| Slug | Verb | Entity | Record |
|---|---|---|---|
| `move-object` | move | object | MovedObject |
| `rotate-object` | rotate | object | RotatedObject |
| `scale-object` | scale | object | ScaledObject |
| `create-brep` / `delete-brep` | create / delete | brep | CreatedBrep / DeletedBrep |
| `create-mesh` / `delete-mesh` | create / delete | mesh | CreatedMesh / DeletedMesh |
| `create-properties` / `delete-properties` | create / delete | properties | CreatedProperties / DeletedProperties |

`move`/`rotate`/`scale` split the single `transform` field into its three domain components per
`📌️important.md`'s explicit domain-verb table (each mutation replaces exactly the named component
of `SemioTransform`, real diff built from `(payload, base)`, self-inverse — another mutation of the
same kind carrying the BASE-state value). `create-X`/`delete-X` for each singular optional CHILD
slot: `create-X`'s inverse is NOT a bare "delete" — it inspects `base.X` and, if occupied, restores
THAT prior handle (a create can overwrite an existing handle; undo must restore what was actually
there, not merely clear the slot). `delete-X`'s inverse escrows the handle from `base` and is
`Vec::new()` when already absent (real absent-target no-op, matching `✳️text`'s `RemoveRun`
precedent).

### `kit` — 15 triads, first real use of `bind`/`unbind`/`change` for a link

| Slug | Verb | Entity |
|---|---|---|
| `create-object` / `delete-object` | create / delete | object |
| `create-model` / `delete-model` | create / delete | model |
| `create-properties` / `delete-properties` | create / delete | properties |
| `bind-representation` / `unbind-representation` | bind / unbind | representation |
| `change-representation-pin` | change | representation-pin |
| `add-type` / `remove-type` / `rename-type` | add / remove / rename | type |
| `add-design` / `remove-design` / `edit-design` | add / remove / edit | design |

- **`create`/`delete` for `objects`/`models`** (Vec-of-child collections): id-addressed
  (`child_id`), append-on-create / filter-on-delete, same overwrite-aware-create /
  escrow-on-delete pattern as `object`'s singular slots, generalized to a collection.
- **`bind`/`unbind` for `representations`**, per `📌️important.md`'s ruling verbatim: "a link fills a
  named slot as a handle, not an edge row" — `bind-representation` appends a new `ArtifactLink`
  (FINAL-state addressing, mirrors `insert`); `unbind-representation` removes by BASE-state index,
  escrowing for its inverse.
- **`change-representation-pin`, not `update-…`**: re-pins the SINGLE `pin` field of an existing
  link while `target`/`role` stay put — `change` is reserved for exactly this ("re-pin a link, i.e.
  `change-link-pin`" — the ticket's own worked example), `update` would be wrong since this is one
  narrow field, not an inseparable ≥2-field facet.
- **No `extract`/`inline`**: deliberately unused this wave. `extract-object`/`inline-object` (owned
  `objects[i]` handle ↔ standalone `representations` link) was designed and would be a natural fit,
  but was cut to keep the triad count bounded (15, already the largest facet in the ticket) — flagged
  under `sharedFileRequests` below as a clean follow-up, not invented vocabulary filling a gap (the
  slots that would need it already exist and are real).
- **`add`/`remove`/`rename` for `types`** (id-keyed, no positional meaning — per addressing rule
  #2/#3, `add`/`remove` for id-keyed collections): `rename-type` sets `name` only (single narrow
  field ⇒ `rename` per the closed table, not `change`+`generic-field`, matching the taxonomy's
  named-entity convention).
- **`add`/`remove`/`edit` for `designs`**: `add-design` creates an EMPTY design (id+name only);
  `edit-design` replaces a design's `pieces`+`connections` WHOLESALE — a design's arrangement is one
  authored unit, `edit` = "replace an authored content body" per `📓️taxonomy.md`, the same shape
  `✳️text`'s `edit-run` uses one level down for a run's `content`. `remove-design`'s inverse is a
  genuine **2-step** `Vec<Mutation>` — `[add-design, edit-design]` — since `add-design` alone can
  only recreate an EMPTY design; restoring a design that had pieces/connections needs the follow-up
  `edit-design` to repopulate them. This is the one place in either subset where `inverse()` returns
  more than one op.

### Order-insensitive collections (documented design decision, not a bug)

`objects`/`models` (kit), `representations` (kit) are id/role-keyed SETS with no user-meaningful
display order — same precedent `✳️graph`'s W2fix report established for `nodes`/`edges`: a
cascading `create`-on-undo always APPENDS, so a restored entry can legitimately land at a different
POSITION than it started at without the SET itself being wrong. `kit`'s own `round_trip` test helper
(`🧬️mutations/🦀️component.rs` tests module) sorts these three fields by `child_id`/`role` before
comparing, and compares `types`/`designs`/`properties` exactly (those ARE positionally/uniquely
addressed — no reordering happens on them). Documented inline at the test helper itself.

## Traps avoided (per `📌️important.md` / prior wave reports)

1. **`round_trip` helper** — both subsets' helper diffs the inverse against the evolving `restored`
   state (`back.diff(&restored)`), never the stale `base`. Copied `✳️text`'s corrected version, not
   the original `din4108`-derived one.
2. **`✳️any`'s two hand-maintained grammar `.semio` files** — both `🔺️diff/📝️text/…` and
   `🧬️mutations/📝️text/…` got `| "object"` then `| "kit"` appended to their `tag` alternation.
   Verified via the `diff_grammar_conformance_law`/`ops_grammar_conformance_law` tests actually
   running (not just `cargo check`, which is silent about this).
3. **Fixture bytes** — both subsets' `.dsl.semio`/`.pack.semio` fixtures are genuine
   `print_dsl`/`encode_pack` output, captured via a temporary `[DEBUG]`-prefixed
   `debug_dump_fixture_bytes` test (`--nocapture`), written byte-exact via Python
   (`bytes.fromhex(...)` for the pack, raw string write with explicit `newline="\n"` for the DSL —
   never hand-transcribed), the test then removed. Confirmed via `fixture_honesty_law` passing on
   the FIRST run after regeneration (no stale-cache retries needed).
4. **Multi-line grammar `|`-continuation** — `kit`'s `mutations` grammar's `op = …` alternation
   (15 keywords) was FIRST authored split across 3 physical lines with a leading `|` continuation
   (matching `✳️graph`'s own documented W2fix bug) — caught immediately by `ops_grammar_conformance_
   law` (`expected Ident, found Pipe`), fixed by joining to one physical line, per the same repo
   convention every sibling subset's grammar file already follows.
5. **`( )` grouping in grammar files** — `kit`'s three `.grammar.semio` files were first authored
   using `( … )?` for optional groups (e.g. `"[" (child {"," child}*)? "]"`), which this dialect's
   grammar-of-grammars does NOT support — `( )` is reserved exclusively for macro-call argument
   lists (`🗣️dsl/📖️grammar/🦀️component.rs:446-459`'s own doc comment: "Grouping uses `{ }`, never
   `( )`"). Caught by `committed_facet_files_parse`/`grammar_conformance_law`
   (`expected a symbol, found LParen`), fixed by converting every grouping `( … )` to `{ … }`
   across all three grammar files. **New trap for this ticket, not previously documented** — flagging
   for any future subset agent.
6. **`str::as_str()` instability** — `dec_pin`'s slice-pattern match originally compared
   `tag.as_str() == "h"` where `tag: &&str` (from `Vec<&str>::as_slice()`'s pattern binding);
   `str::as_str()` is an unstable library feature on this toolchain (`E0658`). Fixed to `*tag == "h"`
   (direct `&str`/`&str` comparison via deref). New trap, not previously documented — the first
   subset in this ticket to slice-pattern-match a `Vec<&str>` result with a literal-string guard.
7. **Emoji-path autofill glitch** (previously documented in `w2fix`'s Concurrent-churn #2, confirmed
   independently in this session): the `Write`/`Edit` tools intermittently substituted
   `🏅️standards/🔖️v1/🪆️subsets/✳️<subset>` with a corrupted `🏅️标准` fragment when a deep multi-emoji
   path was typed inline in a tool call, even when the intended path was correct in my own
   reasoning. Mitigation used for the rest of both subsets: every file write after the first
   occurrence went through `Bash`heredocs (`cat > "$PATH" <<'EOF' … EOF`) with a `$BASE` shell
   variable set once per triad, and every in-place edit went through a Python script reading/
   writing the file by content match rather than retyping the path a second time. Any stray
   `🏅️标准` directory created by the glitch was caught immediately (`ls` returns "No such file or
   directory" once removed) and deleted with `rm -rf` before it could be committed. No dangling
   corrupted directories remain (verified via `find` sweep in Verification below).

## Verification (commands run, actual results — every number below is from a completed run this
session actually executed and read; none are estimated)

```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-stdio --tests
```
Clean. `Finished 'dev' profile [unoptimized] target(s) in 0.47s` (incremental; full first pass after
each subset's files landed took 15-25s, also clean, see `scratch-w2c-1.txt`/`-10.txt`/`-15.txt`).
784 warnings, all pre-existing patterns (dead helper functions never called by any test, glob
`artifact_state`/`snapshot_state` fields — spot-checked several, none newly introduced by
`object`/`kit`).

```
CARGO_TARGET_DIR=".../🎯️target" cargo nextest run --profile long -p semio-s-plugin-stdio --no-fail-fast
```
**Final result: 2174 tests run: 2168 passed, 6 failed, 5 skipped** (`scratch-w2c-nextest-1.txt`).
The 6 failures — **exactly** the ticket brief's documented baseline, unchanged, none mine:

| Failure | Owner |
|---|---|
| `dwg::…::fixture_honesty_law` | unowned (DWG schema-id ticket) |
| `html::…::inference_default_law` | INTRODUCE-INFERENCE-SCHEMA-FAMILY ticket |
| `ifc::…::fixture_honesty_law` | unowned |
| `json::…::inference_default_law` | INTRODUCE-INFERENCE-SCHEMA-FAMILY ticket |
| `md::…::collects_headings_and_counts_words_and_blocks` (outline) | INTRODUCE-INFERENCE-SCHEMA-FAMILY ticket |
| `pdf::…::inference_default_law` | INTRODUCE-INFERENCE-SCHEMA-FAMILY ticket |

Passing count is up from the prior documented baseline (2115 passing, 2121 total before this wave)
to **2168 passing, 2174 total** — this wave net-added exactly 53 tests, all green, zero regressions.

Targeted sub-suite runs (all in this session, all actually executed):
- `subsets::object::` — **24/24 passed** (`scratch-w2c-5.txt`).
- `subsets::kit::` — **27/27 passed** (`scratch-w2c-14.txt`).
- `artifacts::semio::standards::v1::subsets::any::` — **32/32 passed** (`scratch-w2c-16.txt`), incl.
  `all_eighteen_subset_tags_round_trip_text_and_binary`, `all_eighteen_subset_tags_round_trip_
  empty_nested_diff`, `wrapped_object_kind_diff_and_inverse_route_correctly`, `wrapped_kit_kind_
  diff_and_inverse_route_correctly` (new tests this wave).

Four mechanical gates, verified directly (not merely asserted):
1. **Triads ↔ variants 1:1**: `object` 9 enum variants ↔ 9 triad directories; `kit` 15 ↔ 15 (counted
   both ways via `sed`-extracted enum block + `find -maxdepth 1 -type d`).
2. **Unique emoji per triad dir**: `object`'s 9 (`🚚🔄📏🧱💥🕸️🧨🏷️🚫`) and `kit`'s 15
   (`🏗️🪓🏛️💣🏷️🚫🔗✂️📌➕➖✏️🆕🗑️🖊️`) each visually confirmed pairwise-distinct within their own facet.
3. **Real leaves**: every triad has a real `impl protocol::MutationKind<…>` with a non-trivial
   `SEMANTICS`, a real `pub fn diff(payload, base)` built directly from the two arguments (never
   apply-then-capture — spot-checked every one while authoring), a real `pub fn inverse(payload,
   base)` reconstructing from `base` (returning `Vec::new()` for every absent-target case, `Vec`
   with 2 elements for `remove-design`'s escrow-and-repopulate case).
4. **Non-stub `.ts`**: 30 (`object`) + 48 (`kit`) = 78 `🟦️component.ts` files, zero matching
   `^export {};$` (grep-verified), real glue `#[path]` mounts (never inline `#[path = "."]`
   self-wiring beyond the established module-container convention every sibling subset uses).

No banned-vocabulary tokens (`SetSnapshot`/`NoMutation`/`CollectionMutation`) anywhere in either
subset's files — grep-verified across both full subtrees.

## Concurrent-churn observations

1. Heavy unrelated concurrent activity from at least 5 other tickets observed via
   `git status --porcelain` at report time (`ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`,
   `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`, `ENGINELESS-ARTIFACTS-AND-APP-STATE-
   MACHINES`, `FIX-RUST-CODE-WARNINGS-AND-ERRORS`, `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-
   DEPENDENCY-AWARE-CACHING`) — none of it touches any file under `✏️s/🔌️plugins/🗄️stdio/**`, my
   claimed subtree. No collisions.
2. No `Blocking waiting for file lock on build directory` observed during any of this session's
   ~20 `cargo check`/`cargo test`/`cargo nextest` invocations — the shared `CARGO_TARGET_DIR` was
   uncontended throughout.
3. `git log --oneline -5` shows the auto-committer had NOT advanced since the start of this session
   (still `fd01661f06` at report time) — consistent with the other concurrent sessions' work being
   mid-flight/uncommitted rather than landed.

## Out of scope (deliberate, matching `✳️text`'s own precedent)

- `🚪️io/📥️import`/`📤️export` leaves bridging `object`/`kit` ↔ any format artifact — hub routing, a
  later wave's concern. `io_entries()` returns `&[]` for both; `reads()` only advertises each
  subset's own native dialect.
- **Dissolving puzzle/three-block's separately-declared `kit.catalog` artifact kind into `✳️kit`** —
  confirmed the duplication is real (`crate::artifacts::puzzle3d::kit_catalog_artifact_kind()`,
  `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:2417`) but repointing those apps'
  `AppSchema::artifact_kind()` registrations at this new subset is a cross-plugin change well beyond
  this wave's boundary (`W2 stdio agent` owns `✏️s/🔌️plugins/🗄️stdio/**` only, per `📌️important.md`'s
  hot-file table) — flagged under `sharedFileRequests`.
- `extract-object`/`inline-object` (kit) — designed, not authored, see mutation-vocabulary section
  above.

## sharedFileRequests

1. **Dissolve puzzle/three-block's `kit.catalog` duplication into `✳️kit`.** Owner: whichever ticket
   next touches `✏️s/🔌️plugins/🧩️puzzle/**`/`✏️s/🔌️plugins/🧱️block/**`. Puzzle3d's `kit:in` media port
   (`✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs:2254-2274`) currently declares its own
   `kit_catalog_artifact_kind()` and normalizes an ad hoc `objectKinds`/`vortexKinds`/
   `kindCompatibility` JSON fragment. `s.stdio.semio.kit` (this wave) now exists with a real
   `types`/`designs`/`representations` shape that could subsume it — not attempted here, outside
   this ticket's `✏️s/🔌️plugins/🗄️stdio/**` boundary.
2. **`extract-object`/`inline-object` for `kit`** — a genuinely useful pair (owned `objects[i]` handle
   ↔ standalone `representations` link, "detach an embedded instance into a reusable library item"
   and its reverse) that fits the ticket's approved `extract`/`inline` verbs exactly, deliberately
   cut from this wave to bound the triad count. Not urgent — no test/consumer currently needs it.
3. No file outside `✏️s/🔌️plugins/🗄️stdio/**` was touched. All shared-file edits (`🪆️subsets/
   🔣️component.json`, `⚙️engine/🦀️component.rs`, `📦️glue.rs`, `✳️any`'s three schema facets + two
   grammar files + io dispatch) are within this ticket's own claimed territory (`W2 stdio agent`,
   per `📌️important.md`'s hot-file table) and were applied directly, not requested of anyone else.

## Files touched (this wave)

- **Created** (130 files): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/**`
- **Created** (166 files): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/**`
- **Updated**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔣️component.json`
  (added `"object"`, `"kit"` entries)
- **Updated**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
  (2 `register()` calls + 2 `io_registry::entries()` pushes)
- **Updated**: `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (`pub mod object { … }` — ~131 lines
  — and `pub mod kit { … }` — ~185 lines — mounted as `subsets`-siblings after `graph`, before the
  `subsets` module's closing brace; real `#[path]` mounts throughout, no inline self-wiring)
- **Note on Traps #7**: several transient, immediately-deleted stray `…/🏅️标准` directories were
  created and removed during authoring (editor-side path-autofill glitch, see Traps #7) — none
  persisted; every real edit landed at the correct `🏅️standards/🔖️v1/…` path, confirmed by every
  compile/test run in Verification succeeding (a wrong path would be a hard `E0433`/file-not-found
  error). `find … -name '🏅️标准'` returns empty at report time.
- **Updated**: `✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `SemioSubsetSnapshot::{Object,Kit}` arms,
  16→18, all print/parse/encode/decode dispatch tables, `all_eighteen_subset_tags_…` test
- **Updated**: `✳️any/🧬️schema/🔺️diff/🦀️component.rs` — `SemioDiff::{Object,Kit}` arms, `Replace`
  bumped 18→19, all dispatch tables + demo cases + tests
- **Updated**: `✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `SemioMutation::{Object,Kit}` arms, all
  dispatch tables, `demo_mutation_cases` entries, `wrap_absent_mutation` exhaustiveness arms, 2 new
  `wrapped_<subset>_kind_diff_and_inverse_route_correctly` tests
- **Updated**: `✳️any/🚪️io/🦀️component.rs` — `dispatch_validate` gained `Object`/`Kit` arms
- **Updated**: `✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` +
  `✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — `| "object" | "kit"` appended to
  both `tag` alternations

## Summary

`✳️object` and `✳️kit` are complete: full anatomy each (snapshot/diff/mutations/io, all facet
twins, grammar/protocol leaves, engine registration, `✳️any` 17th+18th arms across all three schema
facets, both hand-maintained grammar files, and the io dispatch table, one regenerated example
each). `object` (9 triads: `move`/`rotate`/`scale` + 3×`create`/`delete` pairs) has three CHILD
slots and no link slots; `kit` (15 triads: 3×`create`/`delete` pairs + `bind`/`unbind`/`change` for
its one link slot + `add`/`remove`/`rename` for types + `add`/`remove`/`edit` for designs) has two
CHILD collections, one optional CHILD slot, and the ticket's first real LINK slot. Both subsets'
`#[child]`/`#[link_slot]` attributes are the ONLY source of their composition slot tables — derive-
generated, never hand-written, per `📌️important.md`'s explicit instruction. `cargo check -p
semio-s-plugin-stdio --tests` is clean; `cargo nextest run --profile long` is **2168/2174 passing**
(+53 tests over the 2115/2121 baseline this wave started from), with the same 6 pre-existing/
unowned failures as documented in the ticket brief and zero new ones. Two new traps were found and
documented for future subset authors (grammar-file `( )` grouping is unsupported — use `{ }`; a
`Vec<&str>` slice-pattern guard must compare via `*tag == "…"`, not `.as_str()`, on this toolchain).
`🚪️io` import/export hub-routing leaves and the puzzle/three-block `kit.catalog` dissolution are
explicitly out of scope per the ticket brief, flagged under `sharedFileRequests` for whichever
wave/ticket owns that cross-plugin boundary next.
