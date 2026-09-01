# 📓️ XML 1.0 `✳️base` — READER-based external oracle retrofit

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base`

Followed the AVI 1.0/`✳️any` reference (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any`)
end to end. This subset had **no generator, no probes, no fixtures** before this session — everything
below was built from scratch.

## 1. What was already there (verified, not assumed)

- `🧪️oracle/🔣️.json` had exactly one oracle, `quick-xml-1-0-mutate`, `kind: "cross-semio-implementation"`,
  package `quick-xml` `0.42` — left **byte-for-byte untouched**, as required.
- `🧪️oracle/🦀️component.rs` (840 lines) — left **byte-for-byte untouched**, as required. It computes
  what each mutation should produce via a hand-written `quick-xml`-based reader/writer, and its own
  header explicitly narrows `GeneralRef` (entity) resolution to numeric character references plus the
  five predefined XML entities only (never a custom DTD-declared entity) — the exact scope this
  session's new reader wrapper mirrors independently.
- `mutationCatalogs[0].kinds` declares exactly **6** mutation kinds: `set-declaration`, `set-doctype`,
  `insert-element`, `remove-element`, `set-attribute`, `set-text`.
- `mutationManifests[0].mutations`: all 6 entries declare `outcomes: ["applied"]` **only** — no
  `"rejected"` outcome anywhere in this manifest. So the fixture corpus needed is exactly 6 recipes,
  each `<kind>-applied`, before+after only. No inverse/rejected fixtures were built because none are
  declared.
- Every `oracleRequirements[]` entry had `qualifyingKind: "third-party-library"` but **no `oracle`
  field** — this is what was missing and is now filled in for all 6.

## 2. Witnessability (Step 1 of the playbook)

All 6 kinds are generic XML tree/attribute/text/declaration/doctype operations — exactly the kind of
thing a generic XML event reader (no schema/DTD validation) can see. Checked individually, not assumed:
`set-declaration` (the `<?xml?>` PI's version/encoding/standalone), `set-doctype` (name + SYSTEM/PUBLIC
external id + typed `<!ENTITY>` declarations — the same narrowed DOCTYPE-subset scope the production
oracle itself models), `insert-element`/`remove-element` (child list at a path), `set-attribute`
(attribute map), `set-text` (text content, including the entity-reassembly trap). All 6 are witnessed
by every one of the 6 fixtures — **no `-uncarried` narrowing was needed** for this subset.

## 3. What was built

- **Standalone wrapper crate**: `🏭️generator/🦀️quick-xml-oracle-codec/` — own `[workspace]`, depends on
  ONLY `quick-xml = "0.42"` (same version already pinned in
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`, resolved offline from the local cargo
  registry cache — `cargo build --offline` / `cargo test --offline` both succeed with zero network
  access). Two subcommands, mirroring `riff-avi-codec` exactly: `build <recipe-id> <out-dir>` and
  `project <path>`. 4 unit tests, all passing (`cargo test --offline`):
  `every_declared_recipe_id_resolves`, `encode_decode_round_trips_the_base_document`,
  `general_ref_reassembly_recovers_named_and_numeric_entities_across_events`,
  `every_recipe_after_state_differs_from_before_in_exactly_its_own_dimension`.
- **The named trap, handled explicitly**: the shared base document's `item#i1` text run is
  deliberately written pre-escaped (`Event::Text(BytesText::from_escaped(...))`, bypassing
  auto-escaping) so the literal file bytes contain BOTH a named entity (`&amp;`) and a numeric
  character reference (`&#169;`). Confirmed directly: the on-disk bytes contain `&amp;`/`&#169;`
  literally, and `project` recovers `"Widget & Gadget ©"` — the reader reassembles a text run split by
  `quick-xml` 0.42 across `Text`/`GeneralRef` events into one string. The `set-text` recipe repeats this
  with a DIFFERENT named+numeric pair (`&amp;` + `&#8364;` → `€`) so the discipline is proven to survive
  a mutation, not just the base state.
- **Generator**: `🏭️generator/📜️script.ts` — `generate`/`manifests` commands, mirrors `avi`'s CLI
  shape. Shells out to the wrapper crate; computes no XML semantics itself.
- **Probes**: `🔬️probes/📜️script.ts` — `xml-import` / `xml-project` / `xml-compare`, ProbeReport
  `v2`. Marshal-only; the codec's `project` output is already the `semantic-xml-v1`-shaped projection
  (declaration/doctype/prolog/root, attrs as an unordered map, everything else order-significant), so
  no extra digesting step was needed (unlike AVI's opaque-binary-payload digesting — XML here carries
  no large binary payloads).
- **Fixtures**: 6 recipes, all `<kind>-applied`, `before.xml`+`after.xml`, real bytes written by
  `quick-xml` itself via the wrapper's `build` subcommand — never hand-authored:
  `set-declaration-applied`, `set-doctype-applied`, `insert-element-applied`,
  `remove-element-applied`, `set-attribute-applied`, `set-text-applied`. Each `after.xml` touches
  EXACTLY the field its own mutation kind touches, leaving everything else identical to `before.xml`.
- **`🧪️oracle/🔣️.json`** updated additively (Python-driven `json.load`/mutate/`json.dump`, not
  hand-edited, to avoid formatting drift across a 350+ line file):
  - new oracle `quick-xml-1-0-mutate-reader`, `kind: "third-party-library"`, capability
    `xml-1-0-mutate`.
  - `comparisonProfiles[0].pipeline` now points at the new pipeline.
  - new `probes` array (3 entries) and `comparisonPipelines` array (1 entry,
    `xml-1-0-quick-xml-compare-v1`, GATING).
  - `oracle: "quick-xml-1-0-mutate-reader"` added to all 6 mutations' `oracleRequirements[]`.
  - `fixtureManifests` — 6 entries appended.

## 4. Verification — real command output

### `contract` (repo-wide; filtered below for this subset)

Exit code 1 — but the exit code was already 1 before this session's change, from ~150 pre-existing
`mesh` subset breaches (`manifold3d-three is not registered`) and repo-wide discovery-baseline
overages, none of which this session touched. Lines actually naming this subset's `✳️base` path,
checked one by one:

- `No runtime inventory has been produced for s.stdio.xml@1.0/base` — pre-existing, and identical in
  shape to `s.stdio.avi@1.0/any`'s own line in the same run; means the SUBJECT side hasn't been
  executed (needs the currently-broken `semio-s-plugin-stdio` full-workspace build — out of scope per
  this ticket's own instructions), not a registration problem.
- `reimplementation-registered-as-third-party`: `quick-xml-1-0-mutate-reader is registered as a
  qualifying third-party oracle, but this owner predicts mutation output in its own Rust`, priority
  `high`. **Checked against the reference**: this EXACT same breach ID fires for the proven AVI
  pilot itself (`riff-avi-1-0-mutate-reader ... but this owner predicts mutation output`) and for the
  other completed sibling reader-oracle retrofits (`jszip-bcf-2-1-mutate-reader`,
  `jszip-docx-ecma-376-mutate-reader`) — a known, accepted, fleet-wide checker limitation of this exact
  dual-oracle pattern (one file, `component.rs`, hosts BOTH the untouched `cross-semio-implementation`
  compute path and is co-located with the new `third-party-library` reader registration; the checker's
  heuristic cannot yet tell "two different uses of one crate name" apart from "the reader shares code
  with the predictor"). Not a regression introduced by this registration, and not something in scope to
  fix here (the checker itself lives in `🧰️framework`, outside this subset's tree).
- The `🥒️.feature`/`📡️component.protocol.semio` lines under `📰xml` reference the top-level
  `🧪️tests/mutate-xml-1-0/` feature file and the schema's wire-protocol file — neither was touched this
  session, both pre-existing.

### `fixture verify --artifact xml --subset base`

```
[fixture verify] 6 fixture(s), 0 file problem(s)
```

### `fixture reproduce --artifact xml --subset base --mutation <m> --outcome applied`, run PER FIXTURE in a loop (never batched)

```
=== set-declaration / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-doctype / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== insert-element / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== remove-element / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-attribute / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== set-text / applied ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

6/6, each regenerated and hash-checked individually.

### `matrix --artifact xml --subset base --json`

6 rows, all correctly wired:

```
insert-element  -> fixture insert-element-applied, oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
remove-element  -> fixture remove-element-applied, oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
set-attribute   -> fixture set-attribute-applied,  oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
set-declaration -> fixture set-declaration-applied, oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
set-doctype     -> fixture set-doctype-applied,     oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
set-text        -> fixture set-text-applied,        oracle quick-xml-1-0-mutate-reader (third-party-library), comparisonProfile semantic-xml-v1
```

Each row's `status` is `"missing"` / `"no execution produced a result for this coordinate"` — expected,
since `semio-s-plugin-stdio` does not currently compile as a full workspace member (an unrelated peer's
in-flight migration, per this ticket's own briefing) so the SUBJECT side has never been executed. All
verification therefore ran through the standalone wrapper crate directly, as instructed.

### Compare probe/gate, demonstrated BOTH ways with real numbers

Known-good pair (a real fixture file compared against itself):

```json
{"status":"ok","equal":true,"diffCount":0}
```

Known-bad pair (`set-attribute-applied/before.xml` vs a copy with `qty="2"` changed to `qty="999"`):

```json
{"status":"ok","equal":false,"diffCount":1,"diffs":["$.root.children[0].attrs.qty: \"2\" ≠ \"999\""]}
```

Also confirmed on a real before/after pair (`set-declaration-applied`):

```json
{"status":"ok","equal":false,"diffCount":2,"diffs":["$.declaration.encoding: \"UTF-8\" ≠ \"UTF-16\"","$.declaration.standalone: false ≠ true"]}
```

The gate accepts a known-good pair and rejects a known-bad one, and the diff names the exact field.

## 5. Files touched (all inside this subset's own tree, plus this report)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🏭️generator/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🏭️generator/🦀️quick-xml-oracle-codec/Cargo.toml` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🏭️generator/🦀️quick-xml-oracle-codec/src/main.rs` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🔬️probes/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧫️fixtures/{set-declaration,set-doctype,insert-element,remove-element,set-attribute,set-text}-applied/{before,after}.xml` (new, 12 files)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧪️oracle/🔣️.json` (updated additively)
- `🧪️oracle/🦀️component.rs` — **untouched**, as required.
- This report.

`🦀️quick-xml-oracle-codec/target/` and `Cargo.lock` are present in the working tree (needed for the
offline `cargo build`/`cargo run` calls above) but `target/` is repo-gitignored, matching the sibling
`riff-avi-codec`'s own convention.

## 6. What remains open / unverifiable

- End-to-end SUBJECT execution (`test run`/`test test` actually invoking the production
  `s.stdio.xml@1.0/base` mutation dispatch against these fixtures) could not be verified: it requires
  `semio-s-plugin-stdio` to compile as a full workspace member, which this ticket's own briefing
  confirms is currently broken by an unrelated peer's in-flight migration. Everything above was
  therefore verified through the standalone wrapper crate and the framework's fixture/matrix tooling
  directly, per the ticket's own instruction.
- The `reimplementation-registered-as-third-party` (`testing/oracle`, priority `high`) breach is a
  known fleet-wide checker limitation of this exact dual-oracle pattern, not something introduced or
  fixable by this subset-scoped retrofit — see §4 above for the cross-check against the AVI reference
  and the other completed sibling retrofits.
