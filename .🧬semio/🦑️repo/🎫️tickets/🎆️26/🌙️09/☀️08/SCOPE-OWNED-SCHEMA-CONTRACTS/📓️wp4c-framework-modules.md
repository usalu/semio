# WP4c — `🧰️framework/🔨️modules/**` rows 47a, 47b, 51, 39 + `framework.ui.contract` registration

Partition: `🧰️framework/🔨️modules/**` except the registry module `🧰️framework/🔨️modules/🧬️schema/**` and
every `🧬️schema/🧬️mutations/**` subtree. Continuation of `📓️wp4-framework-modules.md` and
`📓️wp4b-framework-modules.md`. W10c (this row set) was restarted; **nothing of its work was on disk** —
`git status --porcelain -- '🧰️framework/🔨️modules/🌱️value' '🧰️framework/🔨️modules/🖱️ui/🧬️schema'` was empty
at start, and `🖱️ui/🧬️schema/🔣️.json`'s last commit (`9869c6e99b`, 17:55) is W4/W10b's `ownedInterface`/
`expected`/`PresenceOverlayFixture` pass, not W10c's.

## 1. Result per row

| row | request | result |
|---|---|---|
| 47a | remove `field_rename_all()`'s `rename_all_fields → rename_all` fallback | **done** — plus 15-case serde-oracle test suite, 3 in-partition containers repaired, full repo impact measured (53 enums, 51 outside this partition) |
| 47b | internally tagged scalar newtype: decode unwraps `value` | **done** — encode untouched (it is the observable wire), decode is now its exact inverse; round-trip law over 7 payload shapes |
| 51 | widen `framework.ui` per `📓️wp4-plugins.md` §6B items 1–6 | **done** — items 1 and 4 were already applied by the prior pass; 2, 3, 5, 6 applied here |
| 39 | tighten lane/disposition/execution to one vocabulary | **done in the schema**; the 7 fixtures + 9 owner schemas + ~8 readers that still carry the retired spellings are a coupled plugin-partition change (§5.1), measured exactly |
| — | register `framework.ui.contract`'s exports | **done** — the registry crate split has landed, WP4b §5.2's blocker is gone; registration proven by a runtime law, `cargo tree` and `wasm32-wasip2` re-asserted |

## 2. Row 47a — the fallback is gone

`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:226-231`:

```rust
fn field_rename_all(&self) -> Option<String> {
-   self.rename_all_fields.clone().or_else(|| self.rename_all.clone())
+   self.rename_all_fields.clone()
}
```

All eight call sites of `field_rename_all()` are inside `Data::Enum` arms (variant named fields, all three
representations, encode + decode + the `deny_unknown_fields` allow-set). A plain struct's fields are cased
at `:393` from `container.rename_all` and are **unaffected** — asserted, not assumed
(`plain_struct_fields_are_still_cased_by_rename_all`).

Two docstring regions that stated the retired behaviour as fact were corrected: the module header's
`rename_all_fields` paragraph (`:102-108`) and the method docstring.

### 2.1 The test (new file, 15 cases, all green)

`🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/🧪️tests/🐫️variant-field-casing.rs`, registered in
`Cargo.toml` as `[[test]] name = "variant_field_casing"` (Cargo cannot derive an ASCII crate name from an
emoji file stem). Every casing case declares the **identical** `#[value(…)]` and `#[serde(…)]` attribute
pair on one type and asserts `serde_json::Value::from(&x.to_value()) == serde_json::to_value(&x)` — a real
third-party oracle, not a belief about serde. serde 1.0.228 in `Cargo.lock`, ≥ the 1.0.190 that introduced
`rename_all_fields`.

Coverage: `rename_all` alone (internally tagged, externally tagged, adjacently tagged) leaves variant
fields verbatim; `rename_all_fields` cases them; the two attributes are independent axes
(`snake_case` tags + `camelCase` fields); `deny_unknown_fields`'s allow-set follows the same rule (accepts
`brep_id`, rejects `brepId`); plain-struct fields still follow `rename_all`; and our decoder accepts
exactly the wire `serde_json::from_str` accepts and rejects exactly what it rejects.

One deviation from the sibling `🪗️flatten-with-skip.rs` pattern: it compares `serde_json::to_string` texts,
which also compares **key order**. serde emits an internally tagged enum's discriminator first while this
derive's `serde_json::Value` bridge does not preserve that order, so a text comparison fails on ordering
even when every name matches (observed: `{"brep_id":…,"kind":…,"mesh_id":…}` vs
`{"kind":…,"brep_id":…,"mesh_id":…}`). The assertions therefore compare `serde_json::Value`s (map equality,
order-independent) **and** separately assert the emitted key names in emission order via a `wire_keys`
helper — which is the actual subject of the row, and is stronger than a text compare on the names.

### 2.2 In-partition containers that encoded the fallback

A text scan (`wp4c-rename-all-fields-scan.py`, kept in this ticket folder) finds every
`#[derive(…ToValue|FromValue…)]` **enum** declaring `#[value(rename_all)]` without `rename_all_fields` that
has a multi-word named variant field. Five hits inside `🧰️framework/🔨️modules/**`; three needed repair:

| container | evidence for the intended wire | fix |
|---|---|---|
| `🎠️kernel/🦀️.rs:357 Effect` (26 multi-word variant fields across `OpenWindow`/`ClipboardWrite`/…) | its **own** `#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]` — serde already cased the fields, `value` only agreed through the fallback | `#[value(…, rename_all_fields = "camelCase")]` |
| `🖱️ui/🧬️contract/📦️packages/🦀️rust/📐️layout.rs:270 WindowLayoutNode` (`window_kind_id`, `instance_id`, `template_id`, `active_window_kind_id`) | every committed document spells it `windowKindId` (`os/📇️directory/🧬️schema`, `🎯️action-handoff` schema + fixture, every `🔌️plugin-modules/*/🔣️.json`) — and its `#[serde(tag, rename_all)]` was **already** divergent from `value` before this row | `rename_all_fields = "camelCase"` on **both** `#[serde]` and `#[value]`, which is the first time the two codecs agree here |
| `◻️2d/⚙️engine/🦀️.rs:40 PathSegment` (`large_arc`) | `◻️2d/🟦️.ts:48` declares `readonly largeArc` and `:301` reads `segment.largeArc`; this crate's `Cargo.toml:34` marks `ToValue`/`FromValue` as "the plugin-facing wire encoding for `PathSegment`" | `rename_all_fields = "camelCase"` on both; the comment above it asserted the retired serde semantics as fact and was rewritten |

Two hits are **deliberately left alone**, because removing the fallback is what makes them correct:

- `🛂️manifest/🦀️.rs:4893 MediaWireFormat` (`format_kind`) — its production `#[serde(rename_all, tag)]` has no
  `rename_all_fields`, so serde always wired `format_kind`, and the owned TypeScript projection
  `🧰️framework/📦️packages/🦀️rust/🦀️.rs:928` declares
  `{ "kind": "binary", format_kind: string }`. The `value` codec now agrees with both. (The camelCase
  `"formatKind"` in `💻️os/🖥️host/🧪️tests/🕸️media-projection/🧪️fixture/🔣️.json` is **not** counter-evidence:
  `🖥️host/🦀️.rs:2634` hand-writes that document with `json!({"kind":"binary","formatKind": …})` and never
  goes through the derive.)
- `🛂️manifest/🦀️.rs:5166 MediaPayload` (`blob_hash`, `format_kind`) — no serde derive, no TypeScript
  declaration, no committed document anywhere in the repo. No wire evidence exists, so verbatim (= what
  serde would do) is the honest outcome.

Re-scan of the partition after the repairs: `affected-enums=2`, both the deliberate ones above.

### 2.3 Repo-wide blast radius (measured, for the build wave)

```
$ python3 <ticket>/wp4c-rename-all-fields-scan.py
affected-enums=53
  ✏️s/🔌️plugins 33
  🧰️framework/🛍️products 17
  🧰️framework/🔨️modules 3      # before the §2.2 repairs; now 2, both deliberate
```

**`📓️wp4b-stdio-mutations.md` §7.6's list of 17 is incomplete.** Its scan missed every enum whose variants
are written on one line (`Perspective { view_point: …, up_vector: … }`), which is most of them, and it did
not look outside stdio at all. Under `✏️s/🔌️plugins` there are **33**, not 17: the 17 it lists plus
`PathCommand` (svg), `SvgElement`, `HtmlNodeDiff`, `CatalogueValue`/`PartNumberRule` (norm iso16757),
`FemElement`/`FemLoad` ×2 (fem 2d + 3d), `WorkingSolid` (process), `Effect`/`DisplayItemSpec` (cad
interaction-spec), `GeometryNode` (norm), `PathSegment` (draw), `RasterLayerNode`, `NoteBlockNode`.

A further **17** live in `🧰️framework/🛍️products/💻️os/**`, which no ticket row has claimed:
`DagNodeKind`, `Attachment` (mcp), `Widget`, `WidgetDescriptor` (flow), `CollectionMutation` (vcs),
`PersistenceBinding`, `RemoteState`, `ArtifactEvent`, `BackboneWorkerRequest`, `BackboneWorkerResponse`,
`RawFixtureInbound` (store sync), `ArtifactCommand` (store), `GenerationMutation` (playbook), `RunTrigger`,
`RunDiff`, `WorkflowDiff` (workflow), `ArtifactBody` (space collection). Full list with file:line and the
exact moving fields: `🗑️generated/wp4c-rename-all-fields-repo.txt`.

For each, the owner decides between `rename_all_fields = "camelCase"` (keep the wire, re-case nothing) and
accepting the verbatim wire (re-case the fixtures). Where a sibling `#[serde(…)]` attribute exists it
settles the question by itself, as it did for `Effect` above.

## 3. Row 47b — the internally tagged scalar newtype round-trips

`✨️derive/🦀️.rs:976-1020`, decode side only. `expand_to_value` is unchanged: a payload whose `to_value()`
is not an object is carried as the single entry `{"value": <payload>}` beside the tag, and per
`📓️wp4-stdio-mutations.md` §8.4 that encode side is the observable wire the schemas describe.

The decoder is now the exact inverse of that **runtime** branch: the tag-stripped object is offered to the
payload's `FromValue` first, and only if that fails is a lone `value` entry unwrapped and offered bare.
The order is load-bearing and is not a fallback in the CLAUDE.md sense — a payload type whose only field is
literally named `value` produces an indistinguishable key set, so a key-shape test alone would mis-decode
`struct P { value: String }`. Object-first decodes **both** correctly; the test
`the_value_named_field_payload_is_not_mistaken_for_the_scalar_carrier` pins exactly that pair
(`{segment:"Wrapped", value:"inner"}` vs `{segment:"Key", value:"inner"}` — same keys, same value, only the
tag differs). A payload that is neither shape returns the payload type's own object-form error, not a
confusing carrier error.

Covered payload shapes, all round-tripping (`every_internally_tagged_newtype_payload_shape_round_trips`):
`String`, `u64`, `f64`, `bool`, `Vec<u64>`, an object struct (spliced beside the tag, no carrier), and the
`value`-named-field struct. This is the shape of stdio's `JsonPathSegment` (json/i-json, 7 leaves) and
`XlsxCellValue` (xlsx, 5 leaves) — those two remain outside this partition and now decode.

The oracle here is serde's **refusal**, executed rather than quoted:
`serde_refuses_the_shape_this_derive_carries_under_value` asserts `serde_json::to_value` on
`#[serde(tag = "segment")] enum { Key(String) }` errors with a message containing
`tagged newtype variant`. serde has no carrier for this shape, which is why the wire form is this derive's
to define.

## 4. Rows 51 + 39 — `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`

`$id https://semio.tech/schema/framework/ui/schema.json`. Exports went 5 → 7, `definitions` 13 → 19.

### 4.1 §6B items

| item | state |
|---|---|
| 1 `retainedCommandOracle` + `ownedInterface`/`expected` | already applied in `9869c6e99b`; verified present, unchanged |
| 2 `retainedCommandByteBudget` + `artifactStoreBytes`/`decodedItems` | **applied** (optional, `integer ≥ 0`) — unblocks `Fem3dRetainedCommandLimits` |
| 3 `retainedCommandLane` + `transient` | **applied**, see 4.2 |
| 4 `RetainedCommandRoutesDocument` + `maximumRawBytes`/`maximumWorkItems` | already applied in `9869c6e99b`; verified present |
| 5 new `RetainedCommandCohort` export | **applied** — `{schemaVersion, factory, publicationContracts[], routes[], apps[], expected, oracle}` over four new `definitions` (`retainedCommandCohortFactory`, `retainedCommandCohortRoute`, `retainedCommandCohortApp`, `retainedCommandCensus`), read from norm's real `NormRetainedCommandDispositions`/`NormRetainedRoute`/`NormRetainedApp`/`NormRetainedPublicationContract` |
| 6 new `RetainedCommandRoute` export | **applied** — `{"$ref": "#/definitions/retainedCommandRoute"}`; `RetainedCommandRoutes.items` now points at the export rather than into `definitions` |

`RetainedCommandCohort` has, today, **no consumer** — contract §B says a consumerless export is dead. It is
published because §6B item 5 asked for it so `NormRetainedCommandDispositions` can join the family;
the exact norm rewiring is in §5.2. If norm does not adopt it, delete it rather than leave it standing.

Two shapes in the cohort route are genuinely absent from the three existing route dialects and are the
reason a fourth definition was needed rather than a widening: a cohort route separates `emittedLanes` from
`publicationLanes` (the three dialects collapse both into one `lanes`), and its `factory` is an object,
not the single string `retainedCommandDeclaredLimits.factory` carries.

### 4.2 The tightening, and one deviation from the brief

Three `definitions` were extracted so each vocabulary is declared once and referenced:

```
retainedCommandLane        artifact | config | host-only | draft | presence | transient | child
retainedCommandDisposition migrated | batch-only | fail-closed      (disposition, admission, status)
retainedCommandExecution   bounded-first-step | bounded | batch | resumable
```

`retainedCommandRouteDisposition.disposition`, `retainedCommandRouteExecutionFeature.admission` and
`.status` now `$ref` the disposition vocabulary; both route dialects' duplicated `execution` enums now
`$ref` the one execution definition.

**Deviation, reported not silently applied (contract preamble).** The brief named
`artifact|config|host-only|transient` for the lane. That set is exactly "the spellings observed in the 11
fixtures, normalized, ∪ transient". The runtime enum
`ArtifactToolPublicationLane` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12811`) declares
**seven** lanes — `HostOnly, Artifact, Config, Draft, Presence, Transient, Child` — and five plugin
`📜️script.ts` files (`🌊️flow:16`, `🏭️process:9`, `🧱️block:6`, `🧩️puzzle:16`, plus norm's own schema) mirror
all seven. Restricting the shared shape to four would make it reject lanes the runtime supports and that
norm's cohort law already uses, so the enum carries all seven in the brief's kebab casing. Everything the
brief wanted still holds: one spelling per lane, `transient` present, `host-only` not `HostOnly`/`hostOnly`.
Owners narrow with `const`, which is the shape's stated purpose.

**Open question, flagged not decided.** Folding `disposition`/`admission`/`status` onto
`migrated|batch-only|fail-closed` (the brief's list) collapses `BatchOnlyPendingRewrite` and
`batch-only-pending-rewrite` onto `batch-only`, dropping the "pending rewrite" distinction that the runtime
`InteractiveJobClassification` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:836`) still carries as a distinct
variant alongside `unclassified`/`forbiddenFromUi`/`deleted`. If those three matter to the fixture
vocabulary, the enum should be
`unclassified|migrated|batchOnlyPendingRewrite|forbiddenFromUi|deleted` + `failClosed` in the runtime's own
camelCase instead. Follow-up for the coordinator; every fixture normalizes cleanly either way.

## 5. Cross-partition requests

### 5.1 W6b plugins — row 39 is a coupled data + Rust + schema change (the tighten is already in)

The tightened shape and the committed plugin data disagree today, exactly as row 39 anticipated. Measured
(§6): **4 of 11 fixtures pass as committed, 11 of 11 pass once the re-casing below is applied**. The
mapping is total and mechanical:

```
lanes/emittedLanes/publicationLanes  Artifact→artifact  Config→config  HostOnly|hostOnly→host-only
                                     Draft→draft  Presence→presence  Transient→transient  Child→child  host→host-only
disposition|admission|status         Migrated→migrated  BatchOnlyPendingRewrite→batch-only
                                     batch-only-pending-rewrite→batch-only  failClosed→fail-closed
```

**130 values across 7 fixture files** (`bun <ticket>/wp4c-retained-command-fixtures.mjs` prints the
per-file, per-pair counts; reproduced in §6). Three further edit sets travel with them:

- **9 owner schemas that `$ref` `framework.ui` and narrow with the retired `const`s** —
  `🎞️animate/🎬️presentation`, `🎥️shooting`, `🏗️fem/🧊️3d`, `💡️reasoning/🔌️wires`, `📜️imperative/📜️procedure`,
  `📸️remodel/📸️remodeling`, `🪐️space/🏠️home`, `🪐️space/🪐️space`, `🪐️space/🧬️schema`. (`🌿️vcs` already uses
  `artifact|config|host-only` and needs nothing.) A tenth, `🌊️flow/🎬️action-cohort`, and seven other plugin
  schemas narrow the same vocabulary but do **not** `$ref` `framework.ui` — sweep them for consistency,
  they are not gated by this change.
- **Rust readers that parse the fixture lane strings**: `🪐️space/🦀️.rs:948` (`"hostOnly"`) and `:965`
  (`"HostOnly"`), `🪐️space/⚙️engine/🪐️space/🦀️.rs:1445`, `🎥️shooting/…/✏️editor/🦀️.rs:865`,
  `📸️remodel/…/✏️editor/🦀️.rs:1584`, `🏗️fem/…/✏️editor/🦀️.rs:1300` (`HostOnly => "HostOnly"`), plus the
  `publicationContracts` readers in `🎥️shooting/…/🧪️tests/🔬️unit/🦀️.rs:44`,
  `📸️remodel/…/🧪️tests/🔬️unit/🦀️.rs:38`, `🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs:70`,
  `🪐️space/🧪️tests/🔬️interactive-job-catalog/🦀️.rs:46`.
  `🌿️vcs/…/✏️editor/🦀️.rs:1310` already maps `"artifact"|"config"|"host-only"` and needs nothing.
- **`✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`**: `:135`
  `assert.equal(…lanes?.[0], "Config")` → `"config"`, and `:431`
  `route.disposition === "Migrated"` → `"migrated"`.

### 5.2 W6b plugins — let norm adopt `RetainedCommandCohort`

`✏️s/🔌️plugins/📕️norm/🧬️schema/🔣️.json`'s `NormRetainedCommandDispositions` becomes an `allOf` of
`https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandCohort` and its existing
`const` narrowings (`schemaVersion: 2`, the three tool ids, the fifteen variants, the census). Its
`NormRetainedPublicationContract`/`NormRetainedRoute`/`NormRetainedApp` definitions then reduce to those
narrowings. Two content changes go with it: the lane arrays re-case per §5.1 (including `host` →
`host-only` in `emittedLanes`), and `NormRetainedRoute.execution`'s `placeholder` member has **no
instance** — all three routes are `const: "bounded"` — so it is dropped rather than added to the shared
`retainedCommandExecution`. Proven runnable: the normalized norm document validates against
`RetainedCommandCohort` (§6). `📕️norm/🖥️app-surface/🧪️tests/🔬️retained-disposition-oracle/🦀️.rs:80`'s forged
probe `json!(["Artifact"])` re-cases to `["artifact"]`.

### 5.3 W8c stdio + whoever owns `💻️os` — row 47a's real coupling

Rows 46 × 47a are coupled as `📓️wp4b-stdio-mutations.md` §7.6 says, but over **33** plugin enums, not 17,
and **17** more under `🧰️framework/🛍️products/💻️os` that no row currently owns. §2.3 lists both sets; the
machine-readable list with the moving field names is `🗑️generated/wp4c-rename-all-fields-repo.txt`, and
`wp4c-rename-all-fields-scan.py` regenerates it in ~40 s. The three enums `📓️wp4-stdio-mutations.md` §7
blamed for four fixture failures (`AviStreamFormat`, `DocBlock`, `GeometryRef`) are in the list; those four
fixtures are now correct against the derive and need no rewrite.

### 5.4 Coordinator / os boot — call `framework.ui.contract`'s registration

`semio_framework_ui_contract::schema_metadata::register_scope_exports()` now exists but nothing calls it in
production, the same gap `📓️wp4b-framework-modules.md` §7.4 opened for
`semio_framework::interaction::schema::register_scope_exports()`. `register_scope_schema_exports` accepts
an exact duplicate declaration but rejects a differing one, so both want exactly one boot-time call site.

### 5.5 W2c tooling — catalog regeneration

`📚️library/🔣️schema-catalog.json`: `framework.ui` gains `RetainedCommandRoute` and
`RetainedCommandCohort` (5 → 7 exports). `framework.ui.contract` now registers its two named exports in
Rust, so its catalog row should carry the `🦀️.rs` formats entry for them.

## 6. Verification — real output

```
$ CARGO_TARGET_DIR=<scratchpad>/target-w10 RUSTC_WRAPPER="" cargo test -p semio-framework-value-derive
     Running 🧪️tests/🛡️deny-unknown-fields-enums.rs
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
     Running 🧪️tests/🪗️flatten-with-skip.rs
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running 🧪️tests/🆔️newtype-transparent.rs
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running 🧪️tests/🐫️variant-field-casing.rs
running 15 tests
test deny_unknown_fields_allows_the_verbatim_name_and_rejects_the_cased_one ... ok
test a_payload_that_is_neither_shape_reports_the_payload_types_own_error ... ok
test every_internally_tagged_newtype_payload_shape_round_trips ... ok
test externally_tagged_variant_fields_stay_verbatim_like_serde ... ok
test internally_tagged_object_newtype_still_splices_its_entries_beside_the_tag ... ok
test internally_tagged_scalar_newtype_encodes_under_a_value_carrier ... ok
test plain_struct_fields_are_still_cased_by_rename_all ... ok
test rename_all_alone_cases_the_variant_tag_and_leaves_its_fields_verbatim ... ok
test rename_all_alone_rejects_the_camel_cased_field_names_exactly_as_serde_does ... ok
test adjacently_tagged_variant_fields_stay_verbatim_like_serde ... ok
test rename_all_alone_round_trips_and_decodes_the_same_wire_serde_decodes ... ok
test rename_all_and_rename_all_fields_case_independently_like_serde ... ok
test rename_all_fields_cases_the_variant_fields_byte_for_byte_with_serde ... ok
test serde_refuses_the_shape_this_derive_carries_under_value ... ok
test the_value_named_field_payload_is_not_mistaken_for_the_scalar_carrier ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

(48 tests, 0 failures. The first run of this suite failed 3 cases on key **order** only — see §2.1 — and
those three assertions were rewritten; no name ever disagreed with serde after the fix.)

```
$ cargo check -p semio-framework-ui-contract -p semio-framework-2d
    Checking semio-framework-ui-contract v0.1.0 (…/🖱️ui/🧬️contract/📦️packages/🦀️rust)
    Checking semio-framework-2d v0.1.0 (…/◻️2d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1m 39s

$ cargo test -p semio-framework-ui-contract --lib scope_schema_export_law -- --nocapture
warning: `semio-framework-ui-contract` (lib test) generated 7 warnings   # pre-existing, unrelated files
    Finished `test` profile [unoptimized] target(s) in 16.56s
running 1 test
test schema_metadata::scope_schema_export_law::registers_and_resolves_every_declared_export_and_format ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s

$ cargo check -p semio-framework-ui-contract --features typegen --tests
    Finished `dev` profile [unoptimized] target(s) in 32.10s

$ cargo check -p semio-framework-ui-contract --target wasm32-wasip2
    Checking semio-framework-schema-registry v0.1.0 (…/🧬️schema/📇️registry/📦️packages/🦀️rust)
    Checking semio-framework-ui-contract v0.1.0 (…/🖱️ui/🧬️contract/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2m 46s

$ cargo tree -p semio-framework-ui-contract --edges normal | grep -c os-kernel
0
```

The **full** `--lib` run cannot report a summary: a pre-existing, unrelated test
(`action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`) panics
inside `UiValueRetirement::drop` and aborts the binary with SIGABRT, so the 164-test run prints no result
line at all. That is why the registration law is run under a filter above. Not this pass's regression — the
crate's own `Drop` witness, unchanged here.

```
$ bun <ticket>/wp4-framework-validate.mjs
modules=116 exports=217 badDialect=0 badId=0 noExports=0 problems=0
```
(215 before this pass; +2 = `RetainedCommandRoute`, `RetainedCommandCohort`.)

```
$ bun <scratchpad>/w10c-ui-ajv.mjs           # focused ajv compile + behaviour probes of the ui module
ajv=8.20.0 dialect=http://json-schema.org/draft-07/schema# id=https://semio.tech/schema/framework/ui/schema.json
  compiled PresenceOverlayFixture: ok
  compiled UIDialogModalFixture: ok
  compiled RetainedCommandLimits: ok
  compiled RetainedCommandRoute: ok
  compiled RetainedCommandRoutes: ok
  compiled RetainedCommandCohort: ok
  compiled RetainedCommandRoutesDocument: ok
lane accepts: "artifact"=true "config"=true "host-only"=true "draft"=true "presence"=true "transient"=true "child"=true
lane rejects: "Artifact"=false "HostOnly"=false "hostOnly"=false "Config"=false "Transient"=false
disposition accepts: "migrated"=true "batch-only"=true "fail-closed"=true
disposition rejects: "Migrated"=false "BatchOnlyPendingRewrite"=false "batch-only-pending-rewrite"=false "failClosed"=false
byteBudget base=true +fem3d keys=true +unknown=false
RetainedCommandCohort normalized-norm=true
RetainedCommandCohort rejects PascalCase lanes=false
RetainedCommandRoute single row=true bad=false
```

```
$ bun <ticket>/wp4c-retained-command-fixtures.mjs
fixtures=11 committed-pass=4 normalized-pass=11

committed failures (7) — the plugin data still carries the retired spellings:
  🎞️animate/🎬️presentation/…/🧫️retained-command-limits/🔣️.json   routes/0/disposition not in enum
  🎥️shooting/…/🧫️retained-command-limits/🔣️.json                publicationContracts/0/lanes/0 not in enum
  💡️reasoning/🔌️wires/…/🛣️retained-command-routes.json           routes/0/disposition not in enum
  📜️imperative/📜️procedure/…/🛣️retained-command-routes.json       routes/0/disposition not in enum
  🪐️space/⚙️engine/🪐️space/…/🧫️retained-command-limits/🔣️.json    publicationContracts/6/lanes/0 not in enum
  🪐️space/🏠️home/…/🧫️retained-command-limits/🔣️.json             routes/0/disposition not in enum
  🪐️space/🪐️space/…/🧫️retained-command-limits/🔣️.json            publicationContracts/0/lanes/0 not in enum

pending plugin re-casing: 130 values across 7 files
  🎞️animate/🎬️presentation    14× BatchOnlyPendingRewrite → batch-only, 2× Config → config,
                              2× HostOnly → host-only, 4× Migrated → migrated
  🎥️shooting                  37× failClosed → fail-closed, 2× hostOnly → host-only
  💡️reasoning/🔌️wires          8× batch-only-pending-rewrite → batch-only
  📜️imperative/📜️procedure    10× batch-only-pending-rewrite → batch-only
  🪐️space/⚙️engine/🪐️space      6× hostOnly → host-only
  🪐️space/🏠️home               3× Artifact → artifact, 4× Config → config,
                              11× HostOnly → host-only, 18× Migrated → migrated
  🪐️space/🪐️space              9× hostOnly → host-only
```

`normalized-pass=11` is the load-bearing number: the tightened shape accepts the entire real corpus once
§5.1 lands, so the transient break is a data re-casing and nothing else.

```
$ python3 <ticket>/wp4c-rename-all-fields-scan.py 🧰️framework/🔨️modules
🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4893 MediaWireFormat rename_all=camelCase fields=format_kind
🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:5166 MediaPayload rename_all=camelCase fields=blob_hash,format_kind
affected-enums=2
```
(Both deliberate — §2.2. `Effect`, `WindowLayoutNode` and `◻️2d::PathSegment` no longer appear.)

## 7. Files changed

Production:

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` — rows 47a + 47b, plus the two docstring regions.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` — `[[test]] variant_field_casing`.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/🧪️tests/🐫️variant-field-casing.rs` — **new**, 15 cases.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `Effect` gains `rename_all_fields`.
- `🧰️framework/🔨️modules/◻️2d/⚙️engine/🦀️.rs` — `PathSegment` gains it on both codecs; comment corrected.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📐️layout.rs` — `WindowLayoutNode`, both codecs.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-schema-registry`.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs` — `schema_metadata` mounts unconditionally.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs` — `🔖️ScopeSchemaExports` region + runtime law; the
  TypeScript projection moved behind `#[cfg(feature = "typegen")]` **inside** the file so the registration
  runs in every build.
- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` — rows 51 + 39.

Ticket folder (inputs, kept):

- `wp4c-rename-all-fields-scan.py` — the row-47a impact scan.
- `wp4c-retained-command-fixtures.mjs` — the 11-fixture committed/normalized validator and the re-casing table.

Ticket folder (generated, delete at ticket close):

- `🗑️generated/wp4c-rename-all-fields-repo.txt`, `🗑️generated/wp4c-retained-command-fixtures.txt`,
  `🗑️generated/wp4c-value-derive-test.txt`.

## 8. Open questions

1. **`batch-only` vs `batchOnlyPendingRewrite`** — §4.2. The brief's three-value disposition vocabulary
   loses a distinction the runtime `InteractiveJobClassification` still makes, and which
   `🪐️space/📦️packages/🦀️rust/📜️script.ts:441` asserts on by name
   (`InteractiveJobClassification::BatchOnlyPendingRewrite`). Coordinator call.
2. **Seven lanes or four** — §4.2. The deviation is reported, not silent; reverting it to the brief's four
   is a three-line edit, but it would make the shared shape narrower than the runtime enum.
3. **`RetainedCommandCohort` has no consumer until §5.2 lands.** Contract §B would delete it. It is
   published because §6B item 5 asked for it; if norm does not adopt it this pass, it should go.
4. **The 17 `💻️os` enums in §2.3 have no owner.** They are the same coupling as rows 46 × 47a, one product
   over, and no ticket row covers them.
5. **`framework.ui.contract` provides only `🦀️.rs` + `🔣️.json`.** The registration declares empty
   TypeScript/GraphQL/proto leaves and its law asserts those three resolve to an error. If the format
   coverage rule wants an `"x-semio-formats"` annotation on `ConformanceCatalogFixture`/`ContractFixture`
   instead, that is a one-line addition to the module.
