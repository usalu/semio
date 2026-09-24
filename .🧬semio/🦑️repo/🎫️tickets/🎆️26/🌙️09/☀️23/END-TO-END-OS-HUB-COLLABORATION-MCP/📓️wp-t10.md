# WP-T10: Protocol Outcome Vocabulary In Leaf Descriptors, No Severity Projection

Slice: T10 (session 10). Captures: `.tmp-ticket/wp-t10/generated/`. Inputs: `.tmp-ticket/wp-t10/*`.
Inherits: T8 §5–§6 (`wp-t8/projection-survey.py`). Freeze: stdio, gis, kernel pack/store, framework plugin crate.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Schema-first outcome vocabulary (schema, Rust types/derive, bridges, scaffolder, vector reader) | **Prepared, waiting for the freeze to lift.** One atomic patch set, dry run 2,546 files / 0 problems (§1) | `switch-changed.json` |
| 2. Handcraft every leaf outcome, fixture classes | **Table built for 2,803 leaves** (lands with 1). **78 no-op vectors relabelled, landed**: 17 crates, lib tests run (failures: 6 remodel timing, 1 raster control; neither involves outcomes). **18 os.config vectors authored, landed**: 142/142. Oracles run over 18 owners; no failure traces to the relabel (§2) | `relabel-cargo-test-1/2.txt`, `oracle-relabel-1.txt`, `oracle-failures-1.txt` |
| 3. os.config `UiPreferencesConfigMutation` manifest + inventory | **Manifest + bridge coordinate landed; inventory produced** (9 runtime / 9 declared; the 9 differences are the `no-op` outcome waiting for the switch) | `config-inventory-1.txt` |
| 4. Owner debt cad 5, layout 1, gis 3 (after freeze) | **cad 5 + layout 1 landed.** layout: 26/26, **0 differences**. cad: 24/24, runtime-only 5 → 0; its 4 remaining differences are the `no-op` outcome waiting for the switch. **gis 3 → 1:** T11 found the root cause. The terrain window-config manifest claimed the document coordinate `s.gis.gisterrain@1/any`. I deleted that editor-layer `mutationManifests` (JSON only; no crate includes it). The document manifest already owns both document kinds. Left open: one `Catalog gis-gisterrain-1-config … no mutation manifest owns it`, which is T11's editor-layer catalog rule | `layout-inventory-1.txt`, `cad-inventory-1.txt`, `contract-2.txt` |
| 5. Rerun inventories, contract, test-platform; stdio after freeze | Interim contract (before the switch): **504** → **492** high after the gis fix (T8: 632; peers' work included) | `contract-1.txt`, `contract-2.txt` |

## 1. The switch (prepared; lands in one pass after the freeze)

**Vocabulary.** The protocol's `MutationOutcomeClass` (`🧪️test/🧬️schema/🔣️.json`): `applied | no-op | empty | disjoint | rejected`. It is now the leaf descriptor vocabulary. The severity projection is deleted everywhere, so there is one authority.

**Why it cannot land in pieces.** The `MutationLeaf` derive reads every leaf's `🔣️.json` at compile time. The enum lives in `semio-framework-replication`, which every guest links. So the enum, the derive, every leaf JSON (stdio and gis included) and every bridge must change together. The coordinator agreed (one pass, then an immediate `cargo check`, then a message to W1).

**Patch set.** Everything is in the ticket; nothing is a repository script.
- `wp-t10/switch.py` edits:
  - the replication `MutationOutcomeClass` (`Applied, NoOp, Empty, Disjoint, Rejected` plus `as_str`; `ToValue` goes through it);
  - the derive (enum, parser, quote);
  - spr: the unit test, the fixture `enumWireValues` and the `$defs`;
  - the library leaf-descriptor schema: enum plus description;
  - the test platform: `outcomeClassesOf` is now verbatim and throws on foreign values; the descriptor type is `readonly MutationOutcomeClass[]`; the scaffolder maps `::error|::fatal` → `rejected` and `::empty()` / `"mutation.no-op"` → `no-op`; the vector reader reads the status verbatim;
  - every bridge: `protocol_outcomes` is deleted and `class.as_str()` is emitted. That is 39 bridges, including the three stdio semio bridges written in a different style;
  - 13 hand-written Rust descriptors (`wp-t10/hand-descriptors.json`, gis presence among them) plus the vcs native-codec test string;
  - every leaf JSON, from `wp-t10/leaf-outcomes.json`.
  - Dry run: **2,545 files, 0 problems**. The derive fixture's 40+ `applied, warning` rows and its full-roster rows are included.
- `wp-t10/manifest-align.py --write` then sets every manifest row's `outcomes` to the table (dry run: 1,898 rows change, 683 already agree; 1 stdio row, binary `splice`, has no leaf at all: manifest-only debt).

**Per-leaf outcomes, handcrafted from code** (`wp-t10/leaf-evidence.py` → `leaf-review.py` → `leaf-table.py`, overrides in `leaf-overrides.json`):
- Each leaf's own diff code is read, with the `MutationOutcome`-returning helpers it calls resolved within the plugin, type aliases such as fem's `Rejection` included:
  - `::new` → `applied`;
  - `::empty()`, a warned `mutation.no-op`, or `::new(<Default diff>)` → `no-op`;
  - `::error | ::fatal | MutationMessage::error|fatal | PlanError` → `rejected`;
  - composite plans → `applied` + `rejected`.
- 2,801 leaf descriptors, including architect's two-level `🧬️mutations/<entity>/<verb>` leaves (266) and fixture leaves.
- Hand-reviewed groups:
  - the os.config macro leaves (`optional_setting_impl!` / `keyed_setting_impl!` → `applied` + `no-op`);
  - the declared-but-unseen `rejected` (procedural `change-schema` / `clear-widget-layout`: their code cannot reject);
  - the helper-resolved non-stdio leaves (layout's own `diff_*`, forms, fem `guards::`);
  - the demo/store fixture leaves (`delete-n` is `unwrap_or_default` → `no-op`).
- **Dispatch rejects through `apply_to` as well.** Production `mutate` runs `diff.apply_to(snapshot)`, which turns a failing diff `apply` into a Fatal message. So `rejected` is also reachable where a leaf's diff names an entry that its artifact's `apply` can refuse (`missing-target`, `invalid-index`), with no explicit guard. t5 confirmed this for bcf `remove-topic`.
  - Measured (`generated/apply-can-err.json`, `apply-err-reasons.txt`): for every non-stdio leaf with no explicit rejection, the artifact's refusing `apply` paths are collection deltas the leaf never populates (scalar setters: `change-seed`, `set-camera`, `rename-*`), or the leaf guards first (vcs `add-tag`, cad `delete-*-model`). So the non-stdio table stands.
- **Stdio** (`wp-t10/stdio-review.py`):
  - helpers are scoped per artifact crate, and dispatcher helpers (`agg_diff`'s `match`) are cut to the leaf's own variant arm;
  - `…::between(` diffs are `no-op` on equal input;
  - keyed leaves in an artifact whose `apply` can fail (remove, keyed set/replace/rename/truncate, keyed insert: 309 leaves) gain `rejected`;
  - json i-json leaves take `rejected` (their `lower()`) plus the base leaf they lower to;
  - dwg ac1018 `set-version-info` (no Rust in the leaf) takes the ac1024 arm.
  - Stdio classes are rule-derived per artifact and spot-checked by hand (bcf, pdf `page_patch` / `diff_set_page_content`, json); they were not run leaf by leaf.
- 2,803 leaf descriptors in the table; 2,046 differ from the old `info→applied` projection. Most of the difference is `rejected` that was never declared: T8's "173 neither" drift across the repository, e.g. norm's `fatal` finite-value guards.

## 2. Vectors: the `no-op` class made real now (independent of the switch)

Fixtures already speak the protocol vocabulary, so a committed vector whose outcome is a warned `mutation.no-op` is a `no-op`, not `applied`.
- `wp-t10/relabel.py` relabelled **78 vectors** (remodel 18, fem 14, reasoning 6, flow 6, animate 5, forms 5, playbook 4, mathematical 4, trinity 4, puzzle 3, sequence 3, energy 2, imperative 2, writer 1, raster 1).
- It also updated the 76 scenario tests that assert the declared status, including a `"no-op"` arm in puzzle's two status `match`es. `relabel-prose.py` fixed the outcome prose in those tests.
- The case oracles were changed to derive or emit `no-op`:
  - Python: procedure, wires, forms, playbook, energy (`no_op()` helper, 188 call sites), playground, s-home, wfc grid2d (its `no_op` wrongly said `rejected`);
  - TypeScript: the remodel suite derives `no-op` from an empty diff;
  - prose: jack, raster, writer, puzzle-3d.
- Found, not fixed: energy's `change-zone-volume/✅️resizes-zone-one` vector is a no-op (all-null diff) despite its name.

**os.config ui-preferences:**
- per kind, an `applied` and a `no-op` vector (18 quintets under each leaf's `🧫️fixtures`);
- scenario tests that run the implementation against them;
- mounts in the leaves (`wp-t10/ui-preferences-vectors.py`).
- The catalog plus the host case (`🔌️plugin/🖥️host/🧪️tests`, i.e. the plugin crate) comes after the freeze.

**Fixture-class census after the switch** (`wp-t10/vector-census.py`): missing (leaf, class) vectors: `applied` 1,333, `no-op` 1,503, `rejected` 1,766. No contract rule measures per-class vector coverage today; `mutation-outcome-mismatch` only compares declared sets.

**Verification of §2.**
- `cargo test --lib --no-fail-fast` over the 17 crates holding relabelled scenarios: every relabelled scenario passes.
  - Failures: remodel 6 (worker-ceiling timing laws under fleet load) and raster 1 (`raster_standalone_control_max_plus_one…`). Neither touches outcomes.
  - puzzle-5d's `committed_mutation_fixtures_equal_their_canonical_regeneration` asserted `applied|rejected`. Fixed: it now accepts `no-op`, and requires after == before for it. Rerun green, 378/378.
- os-config: 142/142, including the 90 new scenario tests.
- Oracle phase (`oracle exhaustive --owner …`, 18 owners): the only failures mentioning a status are pre-existing.
  - Stale fixture paths in wires and procedure: `shared://🧬️mutations/<leaf>/🧪️tests/...` no longer exists since the fixtures moved to `🧫️fixtures`. Those two oracles never ran, before or after.
  - puzzle-3d `replace-object-vortex` refuses by design.
  - remodel 275/275, raster 37/37, puzzle 273/276 (the 3 are the design refusals); energy's failures are unrelated snapshot mismatches.

**os.config ui-preferences, oracle.** It gained a surveyed `noOracleDecision` (electron-store, confy), the same form as its three sibling vocabularies.

## After the freeze (ordered)
1. `python3 wp-t10/switch.py`, then `manifest-align.py --write`. Then immediately `cargo check -p semio-framework-replication -p semio-framework-os-kernel` (derive + spr) plus a sample of guests (cad, norm, stdio, gis), `--target wasm32-wasip2` for replication. Then tell W1 (full describe pass).
2. gis: done before the freeze (see Status 4); T11 owns the remaining editor-layer catalog row.
3. os.config ui-preferences catalog + host case under `🔌️plugin/🖥️host/🧪️tests` (the plugin crate).
4. Rebuild the bridges, re-inventory every subset, run the contract and the test-platform suite, record the deltas. stdio: T11 has moved rows and edited the stdio bridge; my patches apply on top.

## Processes (pids)
All exited:
- inventory chain 68052
- oracle batch 84989
- relabel cargo test 85261

No servers, no ports, no wasm or hub cargo. `wp-t10/target` deleted.

## Files changed (T10)
- **Manifests:** cad `…/📐️cad/…/✳️any/🔮️oracles/🔣️.json` (+5 object rows), layout `…/📏️layout/…/✳️any/🔮️oracles/🔣️.json` (+`rotate-frame`), os.config `🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json` (+ui-preferences manifest, +decision, comment).
- **Bridge:** `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/🦀️.rs` (ui-preferences coordinate).
- **os.config vectors:** 9 leaves under `🎚️config/🧬️schema/🧬️mutations/<leaf>/`: `🧫️fixtures/{✏️sets-*,🟰️keeps-*}/…`, `🧪️tests/{…}/🦀️.rs`, and the mounts in each leaf `🦀️.rs`.
- **Relabel:** 78 `🎯️outcome/🔣️.json`, plus the 76 scenario tests in `generated/relabel-tests-changed.txt`, plus the puzzle-5d `🔬️committed-fixtures/🦀️.rs`.
- **Oracles:** the procedure, wires, forms, playbook, energy, playground, s-home, grid2d, jack, raster, writer and puzzle-3d `🐍️.py`; the remodel suite `🟦️.ts`.
- **Ticket inputs** (`wp-t10/`): `switch.py`, `manifest-align.py`, `manifest-rows.py`, `leaf-census.py`, `leaf-evidence.py`, `leaf-review.py`, `leaf-table.py`, `leaf-overrides.json`, `leaf-outcomes.json`, `hand-descriptors.json`, `stdio-review.py`, `relabel.py`, `relabel-prose.py`, `vector-census.py`, `ui-preferences-vectors.py`, `oracle-failures.py`, `list-cases.ts`.
