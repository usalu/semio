# 📓️ SVG 1.1 `✳️base` — reader-oracle retrofit

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️base`

Mirrors the AVI 1.0/`✳️any` reference (`riff` + `riff-avi-codec`) end-to-end. Built entirely from
scratch: this subset had no `🏭️generator`, no `🔬️probes`, and an empty `fixtureManifests` before
this work.

## What was built

- `🏭️generator/🦀️quick-xml-svg-codec/{Cargo.toml,src/main.rs}` — a standalone crate (own
  `[workspace]`, depends on ONLY `quick-xml = "0.42"`) with `build <recipe-id> <out-dir>`,
  `project <path>` and `list-recipes` subcommands. Independent of, and shares no code with,
  this subset's own `🧪️oracle/🦀️component.rs` (left byte-for-byte untouched) and with the
  sibling 📰xml 1.0 `✳️base` subset's own `quick-xml`-backed crate.
- `🏭️generator/📜️script.ts` — shells out to the codec per recipe, writes fixture bundles,
  prints `fixtureManifests` JSON. Mirrors `avi/any`'s generator CLI (`generate`/`manifests`,
  `--only`, `SEMIO_FIXTURE_OUT`).
- `🔬️probes/📜️script.ts` — three probes (`svg-import`, `svg-project`, `svg-compare`), `ProbeReport
  v2` schema. Marshals to the codec's `project` subcommand and reads only; the structural compare
  (name-sorted attributes, order-significant children, `viewBox`/`transform` as raw strings) lives
  in this file, computing no SVG semantics.
- `🧫️fixtures/<recipe>/{before.svg,after.svg}` — 9 recipes, 18 files, all committed.
- `🧪️oracle/🔣️.json` — additive: new oracle `quick-xml-svg-1-1-mutate-reader` (`kind:
  third-party-library`), new comparisonProfile `svg-1-1-quick-xml-reader-v1`, new
  comparisonPipeline `svg-1-1-quick-xml-compare-v1`, 3 new probes, `oracleRequirements[].oracle`
  added to all 9 mutations, 9 new `fixtureManifests` entries. The pre-existing
  `quick-xml-svg-1-1-mutate` oracle entry, `semantic-svg-1-1-v1` profile, `mutationCatalogs`, and
  `component.rs` are untouched.

## Witnessability (Step 1/2 of the playbook)

All 9 declared kinds — `set-declaration`, `set-doctype`, `insert-element`, `remove-element`,
`set-element-name`, `set-attribute`, `set-text`, `set-view-box`, `set-transform` — are witnessable
by `quick-xml` and none needed the `-uncarried` convention:

- 7 kinds (`set-declaration`, `set-doctype`, `insert-element`, `remove-element`,
  `set-element-name`, `set-attribute`, `set-text`) are witnessed SEMANTICALLY: XML declaration
  fields, DOCTYPE raw text, and generic element/attribute/text tree structure are exactly what
  `quick-xml` is a reference reader for.
- 2 kinds (`set-view-box`, `set-transform`) touch `viewBox`/`transform`, which only have SVG
  meaning through the grammar this subset's OWN `component.rs` hand-writes. `quick-xml` has no
  such grammar. Per the ticket's own guidance, these are still witnessed HONESTLY at the
  raw-attribute-string level (a changed `viewBox`/`transform` value is a string that differs) —
  registered as such, explicitly, in the new oracle's own `rationale` and in the new
  `svg-1-1-quick-xml-reader-v1` comparisonProfile's own description, which states plainly it does
  NOT decompose these into numeric geometry (unlike the sibling `semantic-svg-1-1-v1` profile).

## Outcome classes (Step 2 of the playbook)

Grepped each of the 9 `SvgMutation` leaves' own `diff()`
(`🧬️schema/🧬️mutations/✏️<kind>/🦀️.rs`): every one unconditionally returns
`protocol::MutationOutcome::new(..)` — never `empty`/`error`/`fatal`. This matches the existing
`mutationManifests[0].mutations[*].outcomes` in the oracle registration, which already declared
`["applied"]` only for all 9 kinds before this work. So there is no `-rejected-*` recipe to build:
9 recipes, each `<kind>-applied`, is the complete corpus for this subset.

(Rejection/validation logic — `mutation.apply.missing-target` etc. — lives one layer down, in
`SvgDiff::apply`'s own `validate_svg_node`/`validate_svg_attrs`/`validate_svg_children`, and is a
property of specific target-path parameters at apply time, not a per-kind classification any
`SvgMutation` leaf declares.)

## Fixtures (Step 5 of the playbook)

One shared base document (declaration + DOCTYPE + `<svg viewBox="0 0 100 100">` containing
`<g transform="translate(10,20)">` with a `<rect>` and a `<text>Hello</text>`), cloned per recipe,
each touching exactly the field the real `SvgMutation` leaf's own `diff()` touches:

| recipe | mutation | before bytes | after bytes |
|---|---|---|---|
| set-declaration-applied | set-declaration | 359 | 375 |
| set-doctype-applied | set-doctype | 359 | 299 |
| insert-element-applied | insert-element | 359 | 401 |
| remove-element-applied | remove-element | 359 | 330 |
| set-element-name-applied | set-element-name | 359 | 362 |
| set-attribute-applied | set-attribute | 359 | 370 |
| set-text-applied | set-text | 359 | 359 |
| set-view-box-applied | set-view-box | 359 | 359 |
| set-transform-applied | set-transform | 359 | 368 |

All 9 `before.svg` files are byte-identical (sha256
`1082a4032f75bc8b615cdb2fcaff429fe365d4563a912a301ec859f3d57adbf1`), as expected — every recipe
clones the same base document. `class: "third-party-generated"`, `comparisonProfile:
"svg-1-1-quick-xml-reader-v1"`, `reproducible: true`.

**The named trap (attribute ordering)**: the wrapper's `QNode::Element.attrs` is a `Vec<(String,
String)>` built from a fixed literal order in every recipe (never a `HashMap`), matching
`quick_xml::Writer`'s own insertion-order-preserving behaviour, so `build` is deterministic. Proven
per-fixture below, not by a whole-corpus double-run (reproducibility.md lesson 2).

## Verification — real commands, real output

**`contract`** (whole-repo; `--artifact`/`--subset` are not accepted by this script's `contract`
subcommand, confirmed by testing it — it always scans the full registry):

```
exit=1 (pre-existing repo-wide breaches unrelated to this subset — see below)
```

Two breach lines reference MY new registration:

1. `testing/oracle … quick-xml-svg-1-1-mutate-reader is registered as a qualifying third-party
   oracle, but this owner predicts mutation output in its own Rust` — a FALSE POSITIVE from
   `reimplementationOracleBreaches`, a heuristic that flags every qualifying oracle in a
   subset whose directory ALSO contains a `component.rs` matching `/fn apply_kind/` +
   `/has no oracle implementation/`, without distinguishing which registered oracle actually does
   the predicting. Confirmed this is not something introduced by this work: the exact same
   heuristic fires for the reference instance too —
   `riff-avi-1-0-mutate-reader is registered as a qualifying third-party oracle, but this owner
   predicts mutation output in its own Rust` (avi/any's own component.rs matches the same regex).
   Not fixed here — fixing the heuristic itself is framework code outside this ticket's scope and
   would affect every sibling subset mid-retrofit.
2. No `testing/fixture … generated by X, whose kind is cross-semio-implementation` breach for my
   fixtures (unlike avi's own 22 fixtures, which DO carry this breach because avi's generator
   attributes `generator.oracle` to the cross-semio entry `riff-avi-1-0-mutate` instead of the
   reader `riff-avi-1-0-mutate-reader`). This subset's generator correctly attributes
   `generator.oracle: "quick-xml-svg-1-1-mutate-reader"`, so this specific breach class does not
   fire here.

All other lines in the full `contract` output that mention this subset (`svg-1-1-mutate` capability
mismatch on the OLD `quick-xml-svg-1-1-mutate` entry, `Unknown mutation catalog
@mutations-svg-1-1-any`, `Mutation catalog svg-1-1-base (9 kinds) is claimed by no feature`, `No
runtime inventory has been produced for s.stdio.svg@1.1/base`, the wire-record gap in
`💾️binary/📡️component.protocol.semio`) reference either the pre-existing, untouched oracle entry,
pre-existing `.feature`/catalog files, or the runtime-inventory bridge — none of it created or
changed by this work, and none of it fixable without touching files outside this ticket's scope
(Gherkin feature authoring, the protocol.semio wire schema, or the `semio-s-plugin-stdio` compile
blocker the ticket itself flagged as a known peer-in-flight issue).

**`fixture verify --artifact svg --subset base`**:

```
[fixture verify] 9 fixture(s), 0 file problem(s)
```

**`fixture reproduce`, once per fixture** (9 separate invocations, per reproducibility.md lesson 5):

```
=== set-declaration-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-doctype-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== insert-element-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== remove-element-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-element-name-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-attribute-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-text-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-view-box-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-transform-applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

9/9 fixtures reproduce byte-identically, each checked in its own process (never a whole-corpus
double-run).

Also ran the TS generator directly (`bun 🏭️generator/📜️script.ts generate` with
`SEMIO_FIXTURE_OUT` pointed at a scratch directory) and `diff -rq`'d the output against the
committed `🧫️fixtures/` tree: **identical**.

**`matrix --artifact svg --subset base --json`**: 9 rows, one per mutation, each resolved to
`oracle: "quick-xml-svg-1-1-mutate-reader"`, `oracleKind: "third-party-library"`,
`comparisonProfile: "svg-1-1-quick-xml-reader-v1"`, `fixtureClass: "third-party-generated"`. Every
row's `status` is `"missing"` ("no execution produced a result for this coordinate") — confirmed
this is the expected state at registration time, not a regression: avi/any's own rows carry the
identical `status: "missing"` today (checked directly). Actual subject-vs-oracle execution needs
the runtime bridge, which the ticket itself flagged as blocked by a peer's in-flight
`semio-s-plugin-stdio` refactor.

**Compare-probe gate, both ways, real numbers** (per playbook step "Always validate the gate both
ways"):

- Known-good pair (`set-attribute-applied/before.svg` against itself):
  ```json
  { "equal": true, "diffCount": 0, "diffs": [] }
  ```
- Known-bad pair (`set-attribute-applied/after.svg`, which carries `fill="red"`, against a
  scratch copy with `fill="blue"` — one deliberately-wrong attribute value):
  ```json
  {
    "equal": false,
    "diffCount": 1,
    "diffs": ["$.root.children[0].children[0].attrs[0][1]: \"red\" ≠ \"blue\""]
  }
  ```
  The diff names the exact field (the `rect` element's `fill` attribute value).

**`svg-import`** against `set-attribute-applied/{before,after}.svg`: `bothImport: true` for both.

**`svg-project`** against `insert-element-applied/after.svg`: `nodeCount: 6`,
`declarationPresent: true`, `doctypePresent: true`, with the appended `<circle>` present as the
3rd child of `<g>` in `root.children[0].children`.

**Wrapper crate's own tests** (`cargo test --offline`, from
`🏭️generator/🦀️quick-xml-svg-codec/`): 3/3 passing —
`every_declared_recipe_id_resolves`, `encode_decode_round_trips_the_base_document`,
`each_recipe_after_differs_from_before`.

## What remains open / unverifiable

- Full subject-vs-oracle execution (an actual run of production `SvgMutation` dispatch compared
  against these fixtures through the comparison pipeline) could not be exercised: the ticket
  itself notes `semio-s-plugin-stdio` does not currently compile as a full workspace member due to
  a concurrent peer's in-flight refactor, unrelated to this work. Everything reachable without that
  bridge (fixture generation/verification/reproduction, both probes, the compare gate both ways,
  the oracle/manifest registration, `matrix`'s row resolution) was run for real and is reported
  above with actual numbers.
- The `reimplementationOracleBreaches` false positive on `quick-xml-svg-1-1-mutate-reader` is
  unresolved, matching the reference instance's own identical unresolved false positive on
  `riff-avi-1-0-mutate-reader`. Fixing the heuristic (to distinguish which registered oracle in a
  multi-oracle subset actually does the predicting) is framework code, out of this ticket's scope.
- Several `contract` breach lines reference this subset (missing Gherkin feature coverage of the
  `svg-1-1-base` catalog, no runtime inventory, a stale capability name on the untouched
  `quick-xml-svg-1-1-mutate` entry, missing wire records in `💾️binary/📡️component.protocol.semio`)
  but are pre-existing, unrelated to this retrofit, and outside the five listed artefacts (oracle
  registration, probes, generator, wrapper crate, fixtures) this ticket asked for.
