# Generators of the norm plugins: safety contract and usage

All generators of this ticket live in this folder. They write into `✏️s/🔌️plugins/📕️norm` (artifact trees, fixtures, registries), `📋️build-owners.json` and a few lock files while other owners work on the same files. This document is the single place for what each tool does, what it reads, what it guarantees and how to run it.

## 1. Safety contract

Every supported entry point is:

1. **Additive.** An existing file is only rewritten when the new text keeps every old line. A line counts as kept when it appears verbatim, when an added line contains it (an inserted field, an appended comma), or when only whole-number counters in it did not decrease (table counts, indices, tags). Anything else is refused. Existing lanes, mounts, crates, roots, kinds and protocol tags are never removed.
2. **Tag-stable.** Binary protocol tags are read from the current `📡️.protocol.semio`; new records get `max + 1 ...`. No tool renumbers.
3. **Idempotent.** An immediate re-run changes nothing (proved in a sandbox, see section 5).
4. **Non-deleting.** `os.remove`, `rmtree`, `rename`, `move` under the repository are skipped and reported. Retiring kinds or directories is a manual step; the retiring/renaming migration scripts are guarded (section 3).
5. **Checkable.** `--check` never writes; it prints the files that a run would create or change and exits 1 when there are any, 0 otherwise.

Mechanisms (use the first that applies):

| layer | what | where |
|---|---|---|
| `safe_run.py` | wrapper for any generator: write guards, all-or-nothing commit, memory-overlay `--check` | `safe_run.py` |
| `Files` (leaf-kinds) | native staged writer, additive merge by default, native `--check`, tag-preserving protocol (`protocol_merge`), no directory removal | `leaf-kinds/common.py`, `keyed.py`, `wire.py`, `en1993_wire.py` |
| `demo_gen.py` | native: union with the existing demo values and cases, `emit` with `--check` | `demo-values/demo_gen.py` |
| unsafe-run guard | one line at the top of destructive or non-idempotent one-shot scripts: exit 2 unless `TH_FORCE=1` | `tools-hardening/guard_unsafe.py`, list in `tools-hardening/guarded.txt` |

`safe_run.py` is all-or-nothing: if any write of a run would lose lines, nothing is written (exit 3, message `aborted`). A half-applied run would otherwise make the next run apply the remaining layers again (the cause of duplicated fields in the EN 1998 field adder).

## 2. Usage

```
python3 safe_run.py [--check] SCRIPT [ARGS...]        # any generator, relative to this folder
python3 safe_run.py --check leaf-kinds/en1992_run.py --apply
python3 safe_run.py field-adder/field_adder.py field-adder/specs/en1994-wave3.json
python3 leaf-kinds/en1992_run.py --check              # native check of a leaf-kinds driver
python3 demo-values/demo_gen.py en1995 --check
```

Use `/home/deck/semio/.venv/bin/python` when a script needs numpy, shapely, scipy, sectionproperties or structuralcodes (oracle scripts). Scripts that import siblings (`from common import *`, `import spec`) must not be started with `python3 -I` (isolated mode drops the script directory); `safe_run.py` puts the script directory on the path.

Limits of `--check` through the wrapper: directory listings and glob do not see files that the same run would create. Subprocesses spawned by a generator do not see buffered writes.

## 3. Tools

### Leaf-kind drivers (`leaf-kinds/`)
Purpose: one typed set-field kind per snapshot leaf plus insert/remove row kinds per collection, wired through every aggregate (root enum, KINDS, ts/json/graphql/proto twins, text codec, binary protocol, edit rules, harness, oracle manifest, taxonomy, crates, fixtures). Inputs: the artifact's snapshot schema, `common.py` constants, the entity profile inside each driver. Guarantees: staged under `🗑️generated/leaf-kinds`, additive merge on apply, tags continue after the highest existing tag, group crates are mounted additively (`mount_into`, `derived_mounts` keep every mount and add every unmounted leaf directory of the group), retirement lists are ignored (directories are kept).
- `en1990_run.py`, `en1991_run.py`, `en1992_run.py`, `en1992_placements_run.py`, `en1993*.py`, `en1994_actions*.py`, `en1997*.py`, `en1999*.py`, `placements.py <en1996|en1997>`: `python3 X.py [--apply|--check]`; without `--apply` only staging (nothing in the tree).
- `en1992_run.py` fixes: the existing-crate detection matched only quoted names; the diff lanes (`optional { tightness }`, placements) and the protocol tags are no longer replaced; fixtures only gain missing lanes.
- Libraries (no entry point): `common.py`, `keyed.py`, `wire.py`, `index_selector.py`, `diff*.py`, `leaf.py`, `model.py`, `fixtures.py`, `keyed_fixtures.py`, `delta.py`, `en1993_base.py`.
- Retired: `rename*.py`, `retire_*.py`, `binary_varint.py` (guarded).

### Field adders (`field-adder/`)
- `field_adder.py specs/<spec>.json`: adds snapshot fields to row records through model, twins (ts, json, graphql, proto), SQLite codecs, field meta, edit rules, fixtures. Profiles per artifact: `en1990_profile.py`, `en1996_profile.py`, `en1997_profile.py`, `en1998_profile.py` (+ `en1998_root.py`, `en1998_rows.py`). Specs: `field-adder/specs/*.json`. Run through `safe_run.py`; already applied specs are no-ops (guard on the model field).
- `new_collection.py specs/<spec>.json` (profile en1991) add id-keyed row collections through every layer (the schema generator writes the twins). Run through `safe_run.py`.

### Placements (`placements-adder/`)
`padd.py <artifact-key>`: clone the last row collection as `placements` (id, x, y, z, yaw) through every twin. `en1990.py` ... `en1998.py` are the per-artifact profiles. Through `safe_run.py`.

### EN 1995 collections (`en1995-collections/`, `en1995-v6/`)
- `spec.py` (library): the declarative entity spec. `schema.py [--dry] [--only plural,plural]`: incremental mode; only missing collections are added, anchored after the last collection present, table and field numbers continue the existing ones (the previous `tapers` skip is gone). `kinds.py [--dry]`: kinds for entities that are not yet in the root enum, tags after the highest existing tag, proto numbers sequential after the highest. `nested.py`, `enums.py`: nested rows and closed enums, guarded by their own markers. `remove_layup_kind.py` (guarded).
- `en1995-v6/entities_rs.py`: moves the checks entities module to layer rows (guard `PatternFacts`).

### EN 1999 wave 2 (`en1999-wave2/`)
`spec.py` is the shared entity spec (library); `s0_split.py` ... `s7_scene.py` are the per-layer stages; run through `safe_run.py` in numeric order. `s2_diff.py --check` is native.

### Showcase documents (`showcase/`)
`en1993.py`, `en1994.py`, `en1998.py`, `en1999.py` build the showcase fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` from committed fixtures with leaf edits and validate it against the schema. `common.py` is the library. They own that one file; run through `safe_run.py`.

### Demo values (`demo-values/`)
`demo_gen.py <code> [--check]` (codes en1990, en1991, en1995, en1996, en1997): generates `cases/🌱️demo/🔣️.json`, `🌱️demo/🦀️.rs` and the cases of the catalogue demo document from `values_<code>.py`, the vocabulary and the committed catalogue fixture. Union semantics: existing demo values and case rows stay, missing ones are added. `--check` exits 1 when a file would change or when vocabulary inputs are unresolved. Helpers (`dv_lib.py`, `validate.py`, `scan*.py`, `dump.py`, ...) are read-only or one-shot patches.

### Others
`schema-sync/sync.py [--apply] [--phase P] [--artifact A]` (schema twins sync), `spec-trace/` (trace writer), `crate-budget/*` (crate splitting and serde test-support wiring; one-shot, `d_test_support.py` guarded), `catalogue-bridge-*`/`doc-oracles-*`/`oracle-*` (catalogue and third-party oracle generators; those that rewrite existing files are guarded), `en19xx-*` per-code one-shot generators from earlier rounds. `source-boundary-scan.py`, `check-cargo-authority.py`, `handle-*-check.py` are read-only audits.

## 4. Guarded scripts (unsafe-run guard)
These rewrote or deleted existing content, or were not idempotent, in the sandbox. They exit 2 immediately. `TH_FORCE=1 python3 X.py` lifts the guard; only do that on a scratch copy. The list is `tools-hardening/guarded.txt`; to guard another script: `python3 -I tools-hardening/guard_unsafe.py REL_PATH`.

## 5. Proof method
`tools-hardening/sandbox.py` copies the norm tree, the ticket scripts and the few root files that generators write (`📋️build-owners.json`, `🔒️dependencies.json`, `🌎️hub/Cargo.lock`, `Cargo.lock`, `Cargo.toml`) to `🗑️generated/tools-hardening/scratch` and runs a command in a read-only bubblewrap namespace with the scratch copies bind-mounted over the real paths (everything else is read-only; low priority via `nice -n 19 ionice -c3`). `tools-hardening/prove.py` runs each script twice per variant (`--apply` too where it exists), classifies run 1 (additions only? lines lost? files deleted?) and run 2 (must change nothing), probes `--check` before and after, and restores the touched files. `--matrix` runs the argument-driven cases of `tools-hardening/cases.json`; `--safe` runs through `safe_run.py`. `inventory.py` prints the table of section 6. The scratch copy is deleted when the check is done.

## 5b. Per-tool hardening notes (wave 2)
- `kind-convention-v3/en1992_tightness_kinds.py [--check]`: tags are read from `📡️.protocol.semio` (existing kinds keep their tag, new ones get max + 1; the hard-coded 153/154 are gone), a file is created only when missing, an existing file only grows (a rewrite that drops a line is kept and reported), taxonomy rows already present are not inserted again. `--check` writes nothing, exits 1 when anything would change.
- `typed-enums/convert.py`: writes only with `--apply`; without it (or with `--dry`) it lists the files it would change; `--check` exits 1 while a site is unconverted. It is a type-changing rewriter by nature (String to enum), so run `--check` first, and only on a clean tree.
- `showcase/*.py [--check]`: the showcase fixture is merged additively (existing values win, only missing keys and id rows are added); nothing is written when validation fails; `--check` exits 1 when the file would change. The en1993, en1994, en1998 and en1999 builders currently fail their validation against the current schemas (stale builders, nothing is written).
- `leaf-kinds/en1992_run.py`, `en1992_diff.py`: `Files` merge (see section 1); `en1992_diff.py` is a library.
- `demo-values/demo_gen.py <code> [--check]`, `placements-adder/padd.py <code>`, `field-adder/*`: native or through `safe_run.py`.

## 5c. Superseded and retired tools
- `compact-schema-json.py`: removed earlier; `field-adder/schema_twins.py compact|indent FILE...` replaces it.
- `field-adder/new_collection_en1998.py`, `_v4`, `_v5`, `_v6`: deleted. They hand-edited twins and generated structs that `schema twins generate` owns; nothing referenced them. Use `field-adder/new_collection.py <spec>` or `schema_twins.py add_record`.
- `en1992-diff-patch-fields.py`, `en1992-gen-member-field-leaves.py`: deleted; `leaf-kinds/en1992_run.py` (with `en1992_diff.py`) generates the same leaves and diff lanes additively.
- Applied one-shot migrations stay guarded (`tools-hardening/guarded.txt`, exit 2 unless `TH_FORCE=1`).

## 6. Inventory
| script | purpose | status |
|---|---|---|
| `adjudicator-dryrun.sh` |  | read-only |
| `app-surface-deps.py` | Symbol-level module dependency scan for the app-surface crate (non-test and test files separately). | read-only |
| `app-surface-gen-cargo.py` | Writes the Cargo.toml of every split app-surface crate (and the facade) from one table. | safe_run |
| `app-surface-layers.py` | Checks a candidate crate assignment: every file may only reference symbols owned by its own crate or a lower o | read-only |
| `catalogue-bridge-check-coverage.py` |  | read-only |
| `catalogue-bridge-en1991-build.py` | Builds catalogue-bridge-en1991-inputs.tsv. EN 1991 evaluators work in SI (m, Pa, N, kg, s), temperatures in °C | safe_run |
| `catalogue-bridge-en1991-units-review.py` | Applies the EN 1991 units review to catalogue-bridge-en1991-inputs.tsv: re-derived units, scoped rows and clos | safe_run |
| `catalogue-bridge-en1992-build.py` | Superseded by catalogue-bridge-en1992-units.py (hand-reviewed units from the evaluators); do not run, it would | guarded (TH_FORCE=1) |
| `catalogue-bridge-en1992-units.py` | Hand-reviewed unit, kind, label and choice vocabulary of the EN 1992 design cases, re-derived from the evaluat | safe_run |
| `catalogue-bridge-en1995-lift.py` | Lifts the EN 1995 `cases` layer onto the shared design-case rows (compliance `design_case::{CaseRow, CaseRowDe | guarded (TH_FORCE=1) |
| `catalogue-bridge-en1996-build.py` | Builds catalogue-bridge-en1996-inputs.tsv: EN 1996 evaluators work in SI and name the unit in the key suffix ( | safe_run |
| `catalogue-bridge-extract-keys.py` |  | safe_run |
| `catalogue-bridge-gen-inputs.py` |  | safe_run |
| `catalogue-bridge-plug.py` | Plugs a catalogue into the shared design-case bridge: generates `📚️inputs` (vocabulary json + rust) and `🗂️cas | safe_run |
| `catalogue-bridge-read-keys.py` |  | read-only |
| `catalogue-bridge-review-table.py` |  | safe_run |
| `check-cargo-authority.py` | 🩺 Validates cargo member authority of every norm crate: workspace key, members list, glob, deps, lib path. | read-only |
| `crate-split-deps.py` |  | read-only |
| `crate_split.py` |  | safe_run |
| `direct-manipulation-rollout.py` |  | safe_run |
| `doc-oracles-v2-en1996-fixtures.py` | 🧬 Hand-built EN 1996 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python | safe_run |
| `doc-oracles-v2-en1997-fixtures.py` | 🧬 Hand-built EN 1997 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python | safe_run |
| `doc-oracles-v2-en1998-fixtures.py` | 🧬 Hand-built EN 1998 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python | safe_run |
| `doc-oracles-v2-manifest.py` | 🧾 Appends one sentence to the rationale of `enXXXX-1-compliance-independent` in an artifact's oracle manifest: | safe_run |
| `doc-oracles-v3-en1996-fixtures.py` | 🧬 Hand-built EN 1996 detailing check-outcome documents (pass and fail): fire thickness, chases, movement joint | safe_run |
| `en1992-flatslab-fixture.py` | Writes the EN 1992 flat-slab punching fixture (the JSON twin of En1992Snapshot::flat_slab_punching) next to th | safe_run |
| `en1992-geometry-apply.py` |  | safe_run |
| `en1992-geometry-apply2.py` |  | safe_run |
| `en1992-geometry-diff.py` |  | safe_run |
| `en1992-geometry-fixtures.py` |  | safe_run |
| `en1992-reach.py` |  | safe_run |
| `en1992-relative-humidity.py` |  | safe_run |
| `en1992-section-oracle-vectors.py` |  | safe_run |
| `en1992-units-report.py` | Writes the change table of audits/wave2/units-review-en1992.md body from the old vocabulary json (argv[1]) and | read-only |
| `en1992-wave2-vectors.py` |  | safe_run |
| `en1993-annex-register.py` | Idempotent: registers the sibling crate semio-s-artifact-norm-en1993-clauses-annex (checks/🏛️clauses-annex) in | safe_run |
| `en1993-annex-spec.py` | Field spec of the EN 1993 annex blocks (v4): ring-stiffened shells under external pressure (1-6), corrugated w | read-only |
| `en1993-built-up-outline-spec.py` | Field spec of the EN 1993-1-1 6.4 built-up member chord outline (wave 2 v6): the chord cross-section as a dime | read-only |
| `en1993-built-up-spec.py` | Spec of the EN 1993 `BuiltUpMember` entity (laced, battened and closely spaced built-up compression members, V | read-only |
| `en1993-diff-split.py` | Idempotent: split the five en1993 diff twins into sibling leaf crates so the diff crate closure stays inside t | safe_run |
| `en1993-entity-gen.py` | Adds one entity (spec module given as argv[1]) after its predecessor entity to every EN 1993 layer except the  | safe_run |
| `en1993-frame-bar-spec.py` | Spec of the EN 1993 `FrameBar` entity (member of a general planar frame between two frame nodes with the secti | read-only |
| `en1993-frame-eigen-register.py` | Idempotent: registers the sibling crate semio-s-artifact-norm-en1993-frame-eigen (checks/🕸️frame-eigen) in the | safe_run |
| `en1993-frame-node-spec.py` | Spec of the EN 1993 `FrameNode` entity (node, support and nodal load of a general planar frame for the mechani | read-only |
| `en1993-hollow-joint-spec.py` | Spec of the EN 1993 `HollowJoint` entity (RHS K/N gap joints, EN 1993-1-8 7.4; V-440). Executed by en1993-enti | read-only |
| `en1993-joint-row-spec.py` | Spec of the EN 1993 `JointRow` entity (one bolt row of an end plate joint for the EN 1993-1-8 component method | read-only |
| `en1993-oracle-documents.py` | Committed oracle documents of the EN 1993 `check-outcome` case (class 4 bending, lateral-torsional buckling an | safe_run |
| `en1993-parametric-section-spec.py` | Spec of the EN 1993 `ParametricSection` entity (welded I, RHS and CHS sections by dimensions, compared with th | read-only |
| `en1993-remove-plated-torsion.py` | Removes the entered PlatedPanel fields `transverse_stiffener_torsion` and `transverse_stiffener_polar` (wave 2 | guarded (TH_FORCE=1) |
| `en1993-restraint-spec.py` | Spec of the EN 1993 `Restraint` entity (supports and restraints of a member: V-113b Mcr by FE, V-123 general m | read-only |
| `en1993-schema-twin-split.py` | Idempotent: split the facet twins of an en1993 schema crate (argv[1]: diff/model) into sibling leaf crates so  | safe_run |
| `en1993-special-gen.py` | Appends the SPEC fields of en1993-special-spec.py to every site of the EN 1993 artifact (schema-first, idempot | safe_run |
| `en1993-special-showcase.py` | Emits `En1993Snapshot::special_parts_showcase()` from the `show` values of en1993-special-spec.py (exec'd by e | read-only |
| `en1993-special-spec.py` | Field spec of the EN 1993 special-part wiring: appended snapshot fields per existing entity. Imported by en199 | read-only |
| `en1993-sqlite-contract.py` | Idempotent: derives the EN 1993 SQLite semantic contract (schemaBytes, table widths, table and column limits)  | safe_run |
| `en1993-stiffened-ndp.py` | Registers the EN 1993-1-5 Annex A, B and C rows of the stiffened plate (wave 2 v5) in the NDP registry through | read-only |
| `en1993-stiffened-spec.py` | Field spec of the EN 1993-1-5 Annex A stiffened plate (wave 2 v5): the spacing of equally spaced longitudinal  | read-only |
| `en1993-storey-fixtures-gen.py` | Adds the empty `storeys` collection (and empty delta) next to `craneRunways` in every EN 1993 snapshot/diff fi | guarded (TH_FORCE=1) |
| `en1993-storey-io-gen.py` | Adds the `Storey` table to the EN 1993 SQLite snapshot codecs (SQL, Rust, TypeScript, admission, semantic cont | safe_run |
| `en1993-storey-model-gen.py` | Adds the `Storey` entity to the EN 1993 snapshot model, its five twins, the diff layer and the schema (idempot | safe_run |
| `en1993-storey-spec.py` | Spec of the EN 1993 `Storey` entity (frame/storey, V-020..V-027). Needs `HERE` (the scripts dir) in scope. Imp | read-only |
| `en1993-v7-spec.py` | Field spec of EN 1993 wave 2 v7: the stiffener outline of a plated panel (I_t, I_w, I_p by the mechanics kerne | read-only |
| `en1993-v8-spec.py` | Field spec of EN 1993 wave 2 v8: the warping support of a member for the bimoment row (V-066). Imported by `en | read-only |
| `en1993-v9-fixtures.py` | EN 1993 v9: derives the check-outcome oracle documents for the feature-row gaps (6.2.4.n fail, 6.2.6.vy pass a | safe_run |
| `en1993-weld-group-spec.py` | Spec of the EN 1993 `WeldGroup` entity (eccentrically loaded weld group, elastic vector method, EN 1993-1-8 4. | read-only |
| `en1994-action-position.py` | Adds the nested action position leaves `x_m` / `y_m` (xM / yM) of `CharacteristicAction` (beam and slab action | safe_run |
| `en1994-beam-fire-oracle.py` | Independent cross-check of the Rust catalogue module `beam_fire` (EN 1994-1-2 composite beam): scipy method-of | read-only |
| `en1994-bridge-oracle.py` | Independent derivation of the EN 1993-1-5 Table 3.1 effective-width factors used by the EN 1994 bridge module  | read-only |
| `en1994-column-fire-oracle.py` | Independent numpy re-implementation of the EN 1994 column fire heat-conduction model (quadrant finite volumes, | read-only |
| `en1994-column-mn-oracle.py` | Third-party N-M cross-check of the EN 1994 composite column interaction curve: python -I en1994-column-mn-orac | read-only |
| `en1994-continuous-oracle.py` | Independent oracle for the EN 1994 continuous-beam solver (flexibility method with cracked EI2 zones, cl. 5.4. | read-only |
| `en1994-continuous-points-oracle.py` | PyNite cross-check of the EN 1994 action internals on continuous spans for concentrated forces at `xM` and UDL | read-only |
| `en1994-crack-width-oracle.py` | Reference for catalogue::crack_width (EN 1992-1-1 7.3.4 crack width of the hogging zone) with structuralcodes: | read-only |
| `en1994-diff-split.py` | Idempotent: split the five en1994 diff twins into sibling leaf crates so the diff crate closure stays inside t | safe_run |
| `en1994-encased-oracle.py` | Independent exact-integration oracle for the EN 1994 encased I-section interaction polygon (piecewise-constant | read-only |
| `en1994-fatigue-oracle.py` | Independent cross-check of the Rust catalogue module `fatigue` (EN 1994-2 6.8): the endurance is found numeric | read-only |
| `en1994-girder-detail-oracle.py` | Independent oracle for the EN 1994 bridge girder detail checks and the crack-width band. | read-only |
| `en1994-leaf-kinds.py` | Generates one `change-<entity>-<leaf>` mutation leaf per editable EN 1994 snapshot leaf across every layer (id | safe_run |
| `en1994-oracle-documents.py` | Committed oracle documents of the EN 1994 `check-outcome` case: python -I en1994-oracle-documents.py [--write] | safe_run |
| `en1994-point-load-oracle.py` | PyNite cross-check of the simple-span point force at `xM`: M = F a b / L, V = F max(a, b) / L (python -I en199 | read-only |
| `en1994-split-slab-action.py` | Splits the EN 1994 action row type: beams keep `CharacteristicAction` (no `yM`), slabs get `SlabAction` (with  | guarded (TH_FORCE=1) |
| `en1994_extent_replay.py` | Replays the independent SQLite extent (rows, value bytes, schema bytes, table widths) of the EN 1994 fixture f | safe_run |
| `en1995-gen-placement-kinds.py` | Generates the EN 1995 placement mutation kinds (`change-placement-x/y/z/yaw`, `insert-placement`, `remove-plac | safe_run |
| `en1995-wave2-leaves.py` | EN 1995 wave 2: generates the 12 mutation leaves of the catalogue-case entity and the ULS equation (crate mut- | safe_run |
| `en1995-wave2-registry.py` | EN 1995 wave 2 part 2: fixtures for the new leaves, hand-fix of every existing fixture to the new document sha | read-only |
| `en1995-wave2-twins.py` | EN 1995 wave 2: hand-applies the snapshot/diff schema change (mCritNm removed; ulsEquation + cases added) to j | safe_run |
| `en1996_gen_diff.py` | Applies the EN 1996 wave 2 entities to the diff (rust macros, ts, json). Idempotent. | read-only |
| `en1996_gen_fieldmeta.py` | Adds the EN 1996 wave 2 fields (EN + DE) to the editor field-meta table. Idempotent. | read-only |
| `en1996_gen_fixtures.py` | Hand-fixes every committed EN 1996 mutation fixture for the wave 2 entities and writes the apply vectors of th | read-only |
| `en1996_gen_kinds_v5.py` | EN 1996 owner v5: generates the id-selector mutation kinds of the new wall fields (`bracingSystem`, `diaphragm | native additive + --check (Files) |
| `en1996_gen_leaves.py` | Generates the EN 1996 wave 2 mutation leaves (rust, diff, inverse, descriptor, payload schema, ts twin), their | guarded (TH_FORCE=1) |
| `en1996_gen_model.py` | Applies the EN 1996 wave 2 entities to the snapshot model (rust), its twins (ts, json, proto, graphql) and the | read-only |
| `en1996_gen_register.py` | Registers the EN 1996 wave 2 mutation group crates: workspace members/dependencies, dependents, four Cargo.loc | read-only |
| `en1996_gen_register_masonry.py` | Registers the EN 1996 masonry checks crate split out of the schema crate (source budget). Idempotent. | read-only |
| `en1996_gen_sqlite.py` | Applies the EN 1996 wave 2 entities to the SQLite snapshot layer (ddl, rust codec, admission). Idempotent. | read-only |
| `en1996_gen_sqlite_tests.py` | Applies the EN 1996 wave 2 entities to the SQLite tests and the shared sqlite fixture. Idempotent. | read-only |
| `en1996_gen_sqlite_ts.py` | Applies the EN 1996 wave 2 entities to the SQLite TypeScript codec. Idempotent. | read-only |
| `en1996_gen_vectors.py` | Registers the EN 1996 wave 2 kinds in the Python vocabulary adapter, the mutate feature and the oracle manifes | read-only |
| `en1996_spec.py` | EN 1996 wave 2 entity spec shared by the generators (`en1996_gen_*.py` import this by exec). Input script, kep | safe_run |
| `en1997-v2-ndp.py` | EN 1997 v2: registers the unverified constants of the Annex A chord rotation, Annex B liquefaction screen, Ann | safe_run |
| `en1998-field-adder-rows.py` | SUPERSEDED by `field-adder/field_adder.py field-adder/specs/en1998-wave2.json` (EN 1998 profile); do not run.  | safe_run |
| `en1998-field-adder.py` | SUPERSEDED by `field-adder/field_adder.py field-adder/specs/en1998-wave2.json` (EN 1998 profile); do not run.  | safe_run |
| `en1998-fix-records-d.py` | Repairs hyphenated Pascal record names written by one generator run (leaf-kinds D); one-off, kinds derived fro | safe_run |
| `en1998-fix-records.py` | One-off repair of the en1998 leaf run: record names were written hyphenated (`Change-building-ns-q`); rewrites | safe_run |
| `en1998-gen-field-mutations.py` | Generates one `change-<entity>-<field>` mutation leaf per editable EN 1998 snapshot leaf (idempotent: skips ki | safe_run |
| `en1998-gen-root-twins.py` | Regenerates the EN 1998 mutation root `🔗️.graphql` and `🛰️.proto` from the root `KINDS` and each kind's payloa | guarded (TH_FORCE=1) |
| `en1998-sapr-1-snapshot.py` | Replaces site.seismicZone (DeSeismicZone) by site.sApR (f64, m/s2) in the snapshot model and its five schema t | safe_run |
| `en1998-sapr-2-rename.py` | Renames the mutation leaf change-site-seismic-zone to change-site-s-ap-r everywhere, rewrites its leaf files f | guarded (TH_FORCE=1) |
| `en1998-sapr-3-fixtures.py` | Converts every JSON fixture of the EN 1998 artifact from site.seismicZone to site.sApR (zone0..3 -> 0, 1.0, 1. | safe_run |
| `en1998-v2-ndp.py` | EN 1998 v2: registers the unverified constants of the Annex A chord rotation, Annex B liquefaction screen, Ann | safe_run |
| `en1998-v3-ndp.py` | EN 1998 v3: reconciles the Table F.1 rows of the shared NDP registry with the target spec (`📐️specs/en1997-en1 | safe_run |
| `en1998-v4-ndp.py` | EN 1998 v4: registers the two unverified constants of the overturning and time-history checks in the shared ND | read-only |
| `en1998-v4-variable-lane.py` | EN 1998 v4: replaces the whole-Vec `variables` setter of the storey, bridge and tower patches by a keyed eleme | guarded (TH_FORCE=1) |
| `en1998-v5-crate-split.py` | EN 1998 v5 crate budget: moves the structure views of the scene into `semio-s-artifact-norm-en1998-scene-struc | guarded (TH_FORCE=1) |
| `en1998-v5-ndp.py` | EN 1998 v5: registers the unverified constants of the bridge, tank and tower checks in the shared NDP registry | read-only |
| `en1998-v6-deck-kinds.py` | Clones the id-addressed kind `change-bridge-pier-shear-rd-n` into `change-bridge-deck-length-m` and `change-br | safe_run |
| `en1998-v8-site-kinds.py` | Clones the site leaf `change-site-v-s30-m-s` into one kind per ground-classification site field (idempotent, r | safe_run |
| `en1998-v8-wall-kinds.py` | Clones the member leaf `change-member-wall-thickness-m` into one kind per masonry wall row field (idempotent,  | safe_run |
| `handle-edit-rules-check.py` |  | read-only |
| `handle-paths-check.py` | 🧭 Handle write paths against edit rules: every `[id={..}]`/`{}` path literal in the scene dirs, normalised to  | read-only |
| `incremental-units-wire.py` | Wires one artifact to its incremental units: editor module + tests, family evaluate, inferences live source. I | safe_run |
| `io-orphan-owner-mount.py` |  | safe_run |
| `live-wiring-rollout.py` |  | safe_run |
| `no-defaults-v2.py` |  | safe_run |
| `oracle-en1991-v3-bless.sh` |  | read-only |
| `oracle-en1991-v3-gen.py` | 🧪️ EN 1991 wave 3 oracle cases (`third-party-v3-extended.json`): placeholder expectations for the bridge wind  | safe_run |
| `oracle-third-party-bless.sh` |  | read-only |
| `oracle-third-party-diff.py` | 🔍️ Compares the Rust formulas (transcribed line by line, NOT executed: no build slot) with the blessed third-p | read-only |
| `oracle-third-party-gen.py` | 🧪️ Generates the third-party oracle case files for the Eurocode artifacts with placeholder expectations; run t | safe_run |
| `oracle-third-party-register.py` | 📇️ Registers the third-party oracle claims: PyNiteFEA and anastruct in the norm root registry, one `enXXXX-1-< | safe_run |
| `oracle-wave2-bless.sh` |  | read-only |
| `oracle-wave2-gen.py` | 🧪️ Wave-2 third-party oracle case files (`third-party-extended*.json`) for EN 1990 to EN 1999 with placeholder | safe_run |
| `oracle-wave2-register.py` | 📇️ Registers the oracle claims of every Eurocode artifact from its case files: one `enXXXX-1-<library>` entry  | safe_run |
| `oracle-wave2-v2-bless.sh` |  | read-only |
| `oracle-wave2-v2-gen.py` | 🧪️ Oracle v2 case files (`third-party-v2-extended.json`, `third-party-mechanics-v2-extended.json`) for the cla | safe_run |
| `safe_run.py` | 🛡️ Safe runner for every generator script: python3 safe_run.py [--check] [--no-lose] SCRIPT [ARGS...] | safe_run |
| `schema_json.py` |  | read-only |
| `shared-v7-wire-viewers.py` | Wires the ten Eurocode viewers to the ephemeral local view lane (shared v7). Idempotent: skips viewers that al | safe_run |
| `source-boundary-scan.py` | Scan every Cargo crate under a root for the repo source boundary (32 files, 262144 bytes, 4000 active lines pe | read-only |
| `split-en1992-checks.py` |  | safe_run |
| `split-en1993-checks.py` |  | safe_run |
| `split-en1997-checks.py` |  | safe_run |
| `split-en1998-mutations.py` |  | safe_run |
| `vis-en1996-rollout.py` |  | guarded (TH_FORCE=1) |
| `wave2-features-dryrun.py` | Wave 2 features agent: dry-run of the Python oracle adapters of the norm step layer WITHOUT the platform host. | read-only |
| `wave2-features-normalise.py` | Wave 2 features agent: one-shot hand-normalisation of the norm catalogue `🥒️.feature` files to the shared step | safe_run |
| `wave2-features-v2-normalise-prose.py` | Wave 2 features v2: makes the remaining prose `🥒️.feature` files of the shared crates, app surface and artifac | safe_run |
| `wave2-features-v2-parse-all.py` | Parses every non-mutation norm `🥒️.feature` with the platform's own gherkin parser and prints error counts. Us | read-only |
| `wave2-features-v2-roundtrip.py` | Wave 2 features v2: writes the `mutation-round-trip` feature and its two adapters for the ten Eurocode artifac | safe_run |
| `wave2-features-v3-check-outcome.py` | Wave 2 features v3 (input script): generates the `check-outcome` case `🧪️tests/🚦️check-outcome-enXXXX-1` of th | safe_run |
| `wave2-features-v3-manifest.py` | Wave 2 features v3 (input script): idempotent edits of an artifact's `🔮️oracles/🔣️.json` — subject host packag | safe_run |
| `wave2-features-v3-promote-prose.py` | Wave 2 features v3 (input script): moves every undiscovered prose feature `<X>/🧪️tests/🥒️.feature` into the di | guarded (TH_FORCE=1) |
| `wave2-features-v4-catalogue-rows.py` | Wave 2 features v4 (input script): appends catalogue-example rows for the registry entries a catalogue feature | safe_run |
| `wave2-features-v4-rail-fixtures.py` | Wave 2 features v4 (input script): writes the hand-made EN 1990 documents of `🧪️tests/🚄️rail-and-overrides-en1 | safe_run |
| `adjudicator/registry.py` | Adjudicator wave 2: NDP registry rows decided from the clause text. Idempotent. Run: python -I registry.py, th | safe_run |
| `catalogue-bridge-v2/bridge_ext.py` | Fork-local twin variants: typed root twins (json `definitions`, typed ts) and path-parametrised injectors. | read-only |
| `catalogue-bridge-v2/catalogue_doc.py` | Fork-local: the full-catalogue demo document (one seeded case per catalogue entry), mirroring `design_case::se | read-only |
| `catalogue-bridge-v2/compact_defs.py` |  | read-only |
| `catalogue-bridge-v2/compact_oneof.py` |  | read-only |
| `catalogue-bridge-v2/crates.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_catdoc.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_checks.py` |  | read-only |
| `catalogue-bridge-v2/en1990_fixtures.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_leaves.py` |  | read-only |
| `catalogue-bridge-v2/en1990_meta.py` |  | read-only |
| `catalogue-bridge-v2/en1990_registry.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_rust.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_sqlite.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_sqlite_ts.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1990_text.py` |  | read-only |
| `catalogue-bridge-v2/en1990_twins.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_checks.py` |  | read-only |
| `catalogue-bridge-v2/en1991_fixtures.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_leaves.py` |  | read-only |
| `catalogue-bridge-v2/en1991_leaves2.py` |  | read-only |
| `catalogue-bridge-v2/en1991_meta.py` |  | read-only |
| `catalogue-bridge-v2/en1991_registry.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_rust.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_sqlite.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_text.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1991_twins.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_checks.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_fixtures.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_leaves.py` |  | read-only |
| `catalogue-bridge-v2/en1992_meta.py` |  | read-only |
| `catalogue-bridge-v2/en1992_registry.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_rust.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_sqlite.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_text.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1992_twins.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1995_cleanup.py` |  | read-only |
| `catalogue-bridge-v2/en1995_crates.py` |  | read-only |
| `catalogue-bridge-v2/en1996_checks.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_fixtures.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_leaves.py` |  | read-only |
| `catalogue-bridge-v2/en1996_mutwire.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_registry.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_rust.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_sqlite.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1996_twins.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1997_checks.py` |  | read-only |
| `catalogue-bridge-v2/en1997_fixtures.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1997_leaves.py` |  | read-only |
| `catalogue-bridge-v2/en1997_rust.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1997_sqlite.py` |  | guarded (TH_FORCE=1) |
| `catalogue-bridge-v2/en1997_sqlite_ts.py` |  | read-only |
| `catalogue-bridge-v2/en1997_text.py` |  | read-only |
| `catalogue-bridge-v2/en1997_twins.py` |  | read-only |
| `catalogue-bridge-v2/fixtures.py` | Fixtures of the case collection: `cases: []` in every snapshot document, the empty case delta in every diff do | read-only |
| `catalogue-bridge-v2/fixtures2.py` | Fixtures of the case collection: `cases: []` in every snapshot document, the empty case delta in every diff do | read-only |
| `catalogue-bridge-v2/harness.py` | Harness, oracle and taxonomy registration of the seven case kinds (flavour of EN 1997: rs KINDS + fixture_text | read-only |
| `catalogue-bridge-v2/jsonspan.py` | Format-preserving JSON edits: locate an object/array by key path and append a member/item textually (keeps the | read-only |
| `catalogue-bridge-v2/leaves.py` | Installs the seven design-case leaves of EN 1995 into another artifact: leaf directories, family crate `mut-de | safe_run |
| `catalogue-bridge-v2/lib.py` | Shared helpers of catalogue-bridge v2: immediate, anchor-asserting edits and the EN 1995 template access. Run  | safe_run |
| `catalogue-bridge-v2/lib2.py` | Process-local patch of lib after the kind-convention rename: set-case-number/text are change-case-number/text. | read-only |
| `catalogue-bridge-v2/registry_ext.py` | Fork-local registration of the seven case kinds in oracle manifest, taxonomy and feature tables. | read-only |
| `catalogue-bridge-v2/shared_twins.py` |  | read-only |
| `catalogue-bridge-v2/twins.py` | Case twins for the snapshot, root and diff schema facets: json (format-preserving), ts, graphql and proto, tak | read-only |
| `catalogue-bridge-v3/check_catalogue.py` | Reads one v3 catalogue and checks entry inputs against the vocabulary and the evaluator accessors. Usage: pyth | read-only |
| `catalogue-bridge-v3/emit_catalogue.py` | Emits the cases file and the unit tests of a v3 catalogue from the EN 1993 template. Usage: python3 -I emit_ca | safe_run |
| `catalogue-bridge-v3/en1990_extent_replay.py` | Replays the independent SQLite extent (rows, value bytes, schema bytes, table widths) of the EN 1990 fixture f | read-only |
| `catalogue-bridge-v3/gen_gap.py` | Writes the v3 gap file from the spec-trace JSON, the unreached-function TSVs and the added catalogues. Usage:  | safe_run |
| `catalogue-bridge-v3/gen_inputs.py` | Runs the shared vocabulary generator with extra unit tokens. Usage: python3 -I gen_inputs.py OUT_DIR SOURCE_TS | read-only |
| `catalogue-bridge-v3/spec_en1994.py` |  | read-only |
| `catalogue-bridge-v3/spec_en1998.py` |  | read-only |
| `catalogue-bridge-v3/unreached_fns.py` | Lists pure pub fns of an entity-driven EN code that no check reachable from a snapshot-iterating file calls (d | read-only |
| `catalogue-bridge-v5/geometry_gen.py` | Generates the Rust geometry tables of the design-case catalogues (BRIDGE v5) from the hand-maintained JSON sou | safe_run |
| `catalogue-bridge-v6/law_check.py` | Python twin of the remaining clamping behaviour of the case forms (counts, angles, ignored roles, single pile  | read-only |
| `compile-gate-h/collide.py` |  | read-only |
| `compile-gate-h/enumck.py` |  | read-only |
| `compile-gate-h/featck.py` |  | read-only |
| `compile-gate-h/fieldck.py` |  | read-only |
| `compile-gate-h/gate.sh` |  | read-only |
| `compile-gate-h/methck.py` |  | read-only |
| `compile-gate-h/patchck.py` |  | read-only |
| `correctness-2/add-fire-load-ndp.py` |  | safe_run |
| `correctness-2/add-gk1-ndp.py` |  | safe_run |
| `crate-budget/d_en1992_checks.py` | 🪚 EN 1992 checks: the generated input vocabulary becomes `-checks-case-inputs`. | read-only |
| `crate-budget/d_en1993_special.py` | 🪚 EN 1993 checks-special: frame, storey, restraint, section, built-up, weld, parametric and hollow-joint check | read-only |
| `crate-budget/d_en1995_scene.py` | 🪚 EN 1995 scene: the pure `facts` module becomes `-scene-facts`. | read-only |
| `crate-budget/d_en1998_mutations.py` | 🪚 EN 1998 mutations: `demo_mutation_cases` and the codec unit tests that consume it become `-mutations-demo`. | read-only |
| `crate-budget/d_lib.py` | 🪚 Crate-budget D helpers: create a sibling crate whose lib is an existing module file, register it everywhere. | read-only |
| `crate-budget/d_test_support.py` | 🧪 Applies the en1992 `test-support` serde pattern to every artifact model crate and enables it in the dev-depe | guarded (TH_FORCE=1) |
| `crate-budget/d_test_support_lock.py` | 🔒 Adds the new dev-dependency edges of `d_test_support.py` to every Cargo.lock that knows the crate. | read-only |
| `crate-budget/e_test_support.py` | 🧪 Applies the `test-support` serde feature to every norm crate whose lib closure derives serde under `cfg_attr | read-only |
| `crate-budget/en1993_clauses.py` | 🪚 Splits en1993-clauses into a core crate (classification, effective width, resistance, stability, global, joi | read-only |
| `crate-budget/en1993_move_tests.py` | 🧪 Moves the en1993-scene unit tests that reach the eigen-modes and detail-sketches internals next to that code | guarded (TH_FORCE=1) |
| `crate-budget/en1999_parts.py` | 🪚 Moves the en1999 checks `📏️parts` directory into its own crate `en1999-check-parts`; the checks root re-expo | read-only |
| `crate-budget/f_en1991_case_inputs.py` | 🪚 Crate-budget F: lift the generated EN 1991 case input vocabulary (data only) out of the checks crate. | read-only |
| `crate-budget/f_en1994_catalogue.py` | 🪚 Crate-budget F: lift the pure EN 1994 catalogue (formulas, continuous, encased, fire, fatigue, bridge, crack | read-only |
| `crate-budget/f_en1994_clone_kinds.py` | 🪚 Crate-budget F: add the mutation kinds `change-beam-steel-root-radius-m` and `change-column-rebar-product` b | safe_run |
| `crate-budget/f_en1994_fixtures.py` | 🪚 Crate-budget F: add `steel.rM`, `steelRM` (diff patch) and `columns[].reinforcementProduct` to every committ | safe_run |
| `crate-budget/f_spec_trace.py` | 🪚 Crate-budget F: lift the generated spec-trace table (data only) of an artifact out of its checks crate: f_sp | read-only |
| `crate-budget/g_en1992_oracle_wiring.py` | 🪚 Crate-budget G: the EN 1992 checks oracle wiring (feature-gated, 517 lines) becomes the crate `-checks-oracl | read-only |
| `crate-budget/g_en1998_scene.py` | 🪚 Crate-budget G: EN 1998 scene `response`, `capacity` and `geotechnical` become the crate `-scene-analysis` ( | read-only |
| `crate-budget/g_lib.py` | 🪚 Crate-budget G helpers: `lib` with a fast lock-file search (find with a depth bound, then grep) and `new_cra | read-only |
| `crate-budget/g_panels.py` | 🪚 Crate-budget G: the inspection half of the app-surface panels becomes `semio-s-plugin-norm-app-surface-panel | read-only |
| `crate-budget/g_scene_case_kit.py` | 🪚 Crate-budget G: register the shared crate `semio-s-plugin-norm-structural-scene-case-kit` (members, script l | read-only |
| `crate-budget/h_en1991_scene_members.py` | 🪚 Crate-budget H: EN 1991 scene members: lane bands and the stack level move into `layout`; traffic, hazards a | read-only |
| `crate-budget/h_en1995_schema.py` | 🪚 Crate-budget H: the EN 1995 timber check formulas become the crate `-schema-timber` (root = the module file) | read-only |
| `crate-budget/h_en1997_checks.py` | 🪚 Crate-budget H: all EN 1997 check modules except the project checks and the units become the crate `-checks- | read-only |
| `crate-budget/h_en1998_inferences.py` | 🪚 Crate-budget H: EN 1998 inference helpers, part details and geotechnical details become the crate `-inferenc | read-only |
| `crate-budget/h_en1998_schema.py` | 🪚 Crate-budget H: the inline EN 1998 formula modules (na_de, part_1..part_6) of the schema become the crate `- | read-only |
| `crate-budget/h_en1998_structural.py` | 🪚 Crate-budget H: the EN 1998 structural system checks become the crate `-inferences-structural-system` (root  | read-only |
| `crate-budget/h_en1999_checks.py` | 🪚 Crate-budget H: the EN 1999 member, connection, fire/fatigue, thin-wall, weld-run, outline and free-outline  | read-only |
| `crate-budget/h_lib.py` | 🪚 Crate-budget H helpers: move whole child modules of a crate root into a new sibling crate (files stay in pla | read-only |
| `crate-budget/lib.py` | 🪚 Crate-budget helpers: register a new sibling crate in the artifact workspace, every Cargo.lock that knows th | safe_run |
| `crate-budget/masonry_catalogue.py` | 📇️ Moves the masonry catalogue dir into its own crate (whole files), moves the shared helpers into the formula | guarded (TH_FORCE=1) |
| `crate-budget/masonry_formulas.py` | 📐️ Registers the en1996-masonry-formulas crate (modules already moved) in workspace, locks, script.ts and the  | read-only |
| `crate-budget/model_twin_leaves.py` | 🍃️ Moves the four facet texts (TypeScript, GraphQL, JSON Schema, proto) of a model (snapshot) crate into sibli | read-only |
| `crate-budget/protocol_twin.py` | 📡️ Moves the binary wire-protocol text of a mutations crate into a twin crate (`LEAF`), mounted by the codec t | read-only |
| `crate-budget/scene_members.py` | 🎬 Splits a scene crate: moves all module dirs except the named facade ones into a sibling `<scene>-members` cr | guarded (TH_FORCE=1) |
| `crate-budget/split_mounts.py` | 🪚 Moves whole kind mounts of a mutation leaf crate into a new sibling crate; the old crate re-exports them (`p | read-only |
| `crate-budget/twin_leaves.py` | 🍃️ Moves the four generated facet texts (TypeScript, GraphQL, JSON Schema, proto) of a schema root crate into  | read-only |
| `demo-values/catalogue_doc_local.py` | Fork-local: the full-catalogue demo document (one seeded case per catalogue entry), mirroring `design_case::se | read-only |
| `demo-values/cov.py` |  | read-only |
| `demo-values/coverage_rows.py` | Lists the registry entries without a row in the catalogue feature of each code. Run with python3 -I coverage_r | read-only |
| `demo-values/demo_gen.py` | Generates, per code, cases/🌱️demo/🔣️.json (demo values beyond the vocabulary examples), cases/🌱️demo/🦀️.rs and | safe_run |
| `demo-values/dryrun_all.sh` |  | read-only |
| `demo-values/dump.py` |  | read-only |
| `demo-values/dv_lib.py` | Shared reader of the five catalogue artifacts: registry entries, input vocabulary, demo document, oracle modul | read-only |
| `demo-values/gen_probe.py` | probe: which (entry,key) pairs of a code have no value in its table | read-only |
| `demo-values/oracle_comparison_gen.py` | Generates the `oracle-comparison` case directory (feature, Python oracle adapter, Rust subject adapter) of eac | safe_run |
| `demo-values/oracle_probe.py` | Evaluates every demo case of a code with the independent Python oracle: OK (named values), UNSUPPORTED (no for | read-only |
| `demo-values/rows_gen.py` | Catalogue feature rows from the demo cases: for every registry entry not yet covered by the catalogue feature, | safe_run |
| `demo-values/rows_gen_fixture.py` | Catalogue-fixture-case rows from the demo cases (EN 1995): demo cases of entries the independent oracle evalua | safe_run |
| `demo-values/scan.py` |  | read-only |
| `demo-values/scan2.py` |  | read-only |
| `demo-values/static_inputs_check.py` | Static proof that the demo cases carry every input the evaluators read with `c.n("key")?` / `c.s("key")?` / `c | read-only |
| `demo-values/validate.py` | Replicates the design-case `issues` check (unknown input, wrong kind, not a choice, out of range) on the commi | read-only |
| `demo-values/values_en1990.py` | EN 1990 demo values come from the first committed catalogue fixture case of each entry (independent Python ora | read-only |
| `demo-values/values_en1991.py` | Demo values of the EN 1991 catalogue beyond the vocabulary examples: a plausible two-storey office building wi | read-only |
| `demo-values/values_en1995.py` | Demo values of the EN 1995 catalogue beyond the vocabulary examples: a GL24h glulam beam 200 x 400 mm over 5 m | read-only |
| `demo-values/values_en1996.py` | Demo values of the EN 1996 catalogue beyond the vocabulary examples: a 365 mm clay masonry wall 2.75 m high (u | read-only |
| `demo-values/values_en1997.py` | EN 1997 demo values come from the first committed catalogue fixture case of each entry (independent Python ora | read-only |
| `demo-values/vocab_patch.py` | Adds the vocabulary rows the registry entries read but the vocabulary lacks, and fixes the kinds of text rows  | safe_run |
| `demo-values/vocab_patch2.py` | EN 1996 vocabulary: the exposure choices of M-60 are the EN 1996-2 classes MX1..MX5 the kernel reads (the voca | safe_run |
| `demo-values/vocab_patch_en1995.py` | EN 1995 vocabulary and registry defects found by reading the kernel against the vocabulary (labels and units t | safe_run |
| `demo-values/wire_rust.py` | Wires cases/🌱️demo into `catalogue_demo_cases` of each code (idempotent). Run with python3 -I wire_rust.py. | safe_run |
| `demo-values/wire_tests.py` | Appends the demo-values tests (every demo case evaluates; the committed demo document equals the seeded cases; | safe_run |
| `doc-oracles-v2/add-rows.py` | Switches a check-outcome case to the new oracle and appends rows computed by it: python -I add-rows.py <code>  | safe_run |
| `doc-oracles-v2/gen-en1990.py` | Generates the EN 1990 check-outcome fixture documents from a committed mutation snapshot: python -I gen-en1990 | safe_run |
| `doc-oracles-v2/gen-en1991.py` | Generates the EN 1991 check-outcome fixtures: every assumed value of a committed snapshot is scaled so that th | safe_run |
| `en1990/run.sh` |  | read-only |
| `en1990/s10_json_fixtures.py` |  | safe_run |
| `en1990/s11_dsl.py` |  | safe_run |
| `en1990/s12_examples_rs.py` |  | safe_run |
| `en1990/s13_schema_rs.py` |  | safe_run |
| `en1990/s14_fieldmeta.py` |  | safe_run |
| `en1990/s15_inferences.py` |  | safe_run |
| `en1990/s16_diagrams.py` |  | safe_run |
| `en1990/s17_scene.py` |  | safe_run |
| `en1990/s1_model.py` |  | read-only |
| `en1990/s2_snapshot_twins.py` |  | safe_run |
| `en1990/s3_diff_rs.py` |  | safe_run |
| `en1990/s4_diff_twins.py` |  | safe_run |
| `en1990/s5_diff_proto_gql.py` |  | safe_run |
| `en1990/s6_sql.py` |  | safe_run |
| `en1990/s7_sqlite_rs.py` |  | safe_run |
| `en1990/s8_sqlite_ts.py` |  | guarded (TH_FORCE=1) |
| `en1990/s9_sqlite_ts_tests.py` |  | safe_run |
| `en1990/spec.py` | EN 1990 schema spec shared by the generator steps (ticket-local input script). | safe_run |
| `en1990/sqlite_extent.py` | Independent model of the EN 1990 SQLite snapshot extent: rows and valueBytes of a fixture. | read-only |
| `en1990-v4/fo_diff.py` | Layer 2: the keyed diff lane of `factorOverrides` (rs + json, ts, graphql, proto twins), modelled on the place | read-only |
| `en1990-v4/fo_fixtures.py` | Layer 4: `"factorOverrides"` in every committed EN 1990 JSON fixture (text insertion after the `cases` value,  | read-only |
| `en1990-v4/fo_lib.py` | Shared helpers of the EN 1990 v4 factor-override cascade (ticket-local input script; ROOT override with FO_ROO | safe_run |
| `en1990-v4/fo_schema.py` | Layer 1 of the factor-override cascade: model, root schema struct, snapshot and root twins (json, ts, graphql, | read-only |
| `en1990-v4/fo_sqlite.py` | Layer 3: SQLite (DDL, Rust and TS codecs, admission, tests, contract) of `factorOverrides`; also repairs the a | read-only |
| `en1990-v4/oracle_rail.py` | Third-party oracle (scipy) of the EN 1990 v4 rail resonance estimate: simply supported deck, analytic sine mod | read-only |
| `en1990-v4/run.sh` |  | read-only |
| `en1990-v7/lib.py` | Shared helpers of the EN 1990 v7 cascade (ticket-local input script). | safe_run |
| `en1990-v7/rc_a.py` | Task 1 layer A: reliability class as a schema enum, K_FI no longer stored (model, root schema struct, diff, mu | read-only |
| `en1990-v7/rc_b.py` | Task 1 layer B: mutation layer (payload enum, ChangeKFiDeclared removed everywhere), edit rules, text io, DSL  | safe_run |
| `en1990-v7/rc_c.py` | Task 1 layer C: schema twins (json, ts, graphql, proto) of the reliability class enum and the dropped K_FI fie | read-only |
| `en1990-v7/rc_d.py` | Task 1 layer D: inference, scene, editor and spec-trace code on the typed reliability class (K_FI only from th | read-only |
| `en1990-v7/rc_e.py` | Task 1 layer E: examples, reads/outline lists. | read-only |
| `en1990-v7/rc_f.py` | Task 1 layer F: SQLite codec (sql, rs, ts, admission, tests, contract) of the reliability class text column; K | read-only |
| `en1990-v7/rc_g.py` | Task 1 layer G: every JSON fixture (text edits keep the formatting): reliabilityClass N -> "RCN", kFiDeclared  | read-only |
| `en1990-v7/rc_h.py` | Task 1 layer H: SQLite semantic contract (schemaBytes, tableWidths, case extents) recomputed by the python sql | read-only |
| `en1990-v7/tg_a.py` | Traffic groups layer A: model, enum, root struct, snapshot/root twins (json, ts, graphql, proto), diff lane, e | read-only |
| `en1990-v7/tg_b.py` | Traffic groups layer B: SQLite (ddl, rs, ts, admission, tests, contract) and the empty `trafficGroups` in ever | read-only |
| `en1990-v7/tg_c.py` | Traffic groups layer C: field-meta rows, outline field, test path lists. | read-only |
| `en1990-v7/tg_d.py` | Traffic groups layer D: inference (effective group, registry coefficient, row checks, group combinations), int | read-only |
| `en1990-v7/tg_e.py` | Traffic groups layer E: scene (kit registry, deck projection of the active group's components, group chips, de | read-only |
| `en1990-v7/tg_f.py` | Traffic groups layer F: python oracle (table-driven groups, row coefficient) and the two check-outcome fixture | read-only |
| `en1990-v7/tg_lib.py` | Shared constants of the traffic-group cascade (ticket-local input script). | read-only |
| `en1991-v10/trace_v10.py` |  | safe_run |
| `en1991-v11/ndp_v11.py` |  | read-only |
| `en1991-v5/add-fire-load-basis.py` | Adds the typed snapshot field `fireLoadBasis` (enum occupancyTable / userSpecified, default occupancyTable) af | safe_run |
| `en1991-v6/make-spec.py` |  | safe_run |
| `en1991-v9/add_curve_radius.py` | Adds the snapshot field bridgeCurveRadius (horizontal deck radius in m, 0 = straight deck) after siloHopperAng | safe_run |
| `en1991-v9/ndp_v9.py` |  | read-only |
| `en1992-v4/demo-gen.py` | Generates cases/🌱️demo/🦀️.rs and the demo-document fixture `cases` from cases/🌱️demo/🔣️.json and the input voc | safe_run |
| `en1992-v4/handle-rules-check.py` | Static check: every field path a scene grip, dimension or marker of the EN 1992 scene binds is a SetFieldRule  | read-only |
| `en1992-v4/prestress-fixtures.py` | Writes the two prestressed check-outcome fixtures (a resolved tendon with control points) from the tank wall m | safe_run |
| `en1992-v4/seed-globals.py` | Writes demo example values (column 5) and corrected units (column 4) into the EN 1992 input vocabulary source, | safe_run |
| `en1992-v4/trace-realise.py` | Marks every spec id that the reach table routes to an entity check or a member/anchor check as `check` (with i | safe_run |
| `en1992-v8/bar_fields_fixtures.py` | EN 1992 v8: appends `length` + `bend` to every bar row of every committed en1992 JSON fixture (text insertion, | safe_run |
| `en1992-v8/bar_fields_patch_fix.py` | EN 1992 v8: a bar patch row (any of layerId/y/z/diameter null) carries `length`/`bend` null instead of the row | safe_run |
| `en1992-v8/bar_fields_sqlite.py` | EN 1992 v8: BarPoint `length` + `bend` through the SQLite layer (sql, rs codec, ts codec, cell census, fixture | safe_run |
| `en1992-v8/bar_fields_twins.py` | EN 1992 v8: adds `length` (m, 0 = derived) and `bend` (string vocabulary auto/straight/hook/u-hook) to BarPoin | safe_run |
| `en1994-nested/lanes.py` | EN 1994 nested `actions` lanes: grows `En1994{Beams,Columns,Slabs}ActionsRows` from `modified` to `removed` (` | safe_run |
| `en1994-v10/beam_rebar_fixtures.py` | 🪚 EN 1994 v10: add `beams[].reinforcementProduct` (snapshot rows `"hot_rolled"`, diff patches `null`) to every | safe_run |
| `en1994-v10/beam_rebar_kind.py` | 🪚 EN 1994 v10: add the mutation kind `change-beam-rebar-product` by cloning `change-beam-support` through ever | safe_run |
| `en1994-v10/beam_rebar_leaf.py` | 🧵 EN 1994 v10: add the snapshot leaf `beams[].reinforcementProduct` (String, hot_rolled/cold_worked, default h | safe_run |
| `en1995-collections/enums.py` | Turns every enum-like string field of `spec.py` (field key `enum`, set `ENUMS`) into a closed Rust enum throug | read-only |
| `en1995-collections/kinds.py` | Generates the EN 1995 mutation kinds of the typed collections of `spec.py`: `insert-<slug>`, `remove-<slug>` a | safe_run |
| `en1995-collections/nested.py` | Replaces the `panels[].layup` string by typed child rows `panels[].layers[]` and adds the optional child rows  | safe_run |
| `en1995-collections/remove_layup_kind.py` | Removes the retired `change-panel-layup` mutation kind (the layup string is replaced by typed `panels[].layers | guarded (TH_FORCE=1) |
| `en1995-collections/schema.py` | Adds the typed EN 1995 collections of `spec.py` that the artifact does not have yet to every schema layer, ide | safe_run |
| `en1995-collections/spec.py` | EN 1995 typed physical entities (wave 2 v4): one declarative spec read by `schema.py` (model, twins, diff, io, | read-only |
| `en1995-v11/detail_inputs.py` | Declares the detail inputs of the EN 1995 catalogue entries T-95 (truss layout) and T-96 (bracing layout) in t | safe_run |
| `en1995-v6/entities_rs.py` | EN 1995 v6: moves the entities module of the schema checks from the `layup` string and the regular fastener pa | safe_run |
| `en1995-v9/registry.py` | EN 1995 v9: NDP registry rows. Idempotent. Run: python -I registry.py, then the ticket codegen. | safe_run |
| `en1997/catalogue-demo-gen.py` | Generates the committed EN 1997 full-catalogue demo document `🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣 | guarded (TH_FORCE=1) |
| `en1997/crate_split.py` |  | safe_run |
| `en1997/diff_compact.py` |  | safe_run |
| `en1997/diff_gen.py` |  | safe_run |
| `en1997/diff_gen2.py` |  | safe_run |
| `en1997/diff_twins.py` |  | safe_run |
| `en1997/dsl_assets.py` |  | safe_run |
| `en1997/edit_rules_gen.py` |  | safe_run |
| `en1997/fixtures_gen.py` |  | safe_run |
| `en1997/mut_gen.py` |  | safe_run |
| `en1997/mut_unit.py` |  | safe_run |
| `en1997/mut_wire.py` |  | safe_run |
| `en1997/mutate_tests.py` |  | safe_run |
| `en1997/rows.py` |  | read-only |
| `en1997/seepage_map_check.py` | Numerical verification of the sheet-pile conformal map used by HYD-02 and the scene streamlines: h = H/2 - H/p | read-only |
| `en1997/snapshot_twins.py` |  | safe_run |
| `en1997/spec.py` |  | read-only |
| `en1997/sqlite_gen.py` |  | safe_run |
| `en1997/sqlite_independent.py` |  | safe_run |
| `en1997/sqlite_pins_wave2.py` | EN 1997 wave 2 SQLite pins for the five pile and wall columns: recomputes `schemaBytes` and `tableWidths` by r | safe_run |
| `en1997/sqlite_tests.py` |  | safe_run |
| `en1997/sqlite_ts_gen.py` |  | safe_run |
| `en1997/v2_cases.py` | EN 1997 v2: appends the catalogue cases of GI-17, PILE-10, PILE-11, HYD-02 and SLOPE-05 to the shared fixture  | safe_run |
| `en1998-v10/1-wall-plan-kinds.py` | v10: clones the member kind into one kind per wall plan field (wall_start_x_m, wall_start_y_m) and mounts them | safe_run |
| `en1998-v11/2-joint-kinds.py` | v11: clones the bearing row-kind family (change-bearing-*, insert-bearing, remove-bearing) into the joint fami | safe_run |
| `en1998-v11/3-wall-poisson-kind.py` | v11 (copy of v10 for wall_poisson_ratio): clones the member kind into one kind per wall plan field (wall_start | safe_run |
| `en1998-v12/0-make-kinds-script.py` | Generates 1-stiffness-kinds.py from the v11 wall-poisson kind script: one parameterised clone script for the m | safe_run |
| `en1998-v12/1-stiffness-kinds.py` | v11 (copy of v10 for wall_poisson_ratio): clones the member kind into one kind per wall plan field (wall_start | safe_run |
| `en1998-v13/0-schema-json.py` | v13 step 0: adds the `StiffnessBasis` enum and the two basis properties to the en1998 schema JSONs (schema, sn | safe_run |
| `en1998-v13/1-fixtures-basis.py` | v13 step 1: adds the stiffness basis next to every stored stiffness ratio in the en1998 JSON fixtures (idempot | safe_run |
| `en1998-v13/2-dsl-assets.py` | v13 step 2: adds the stiffness basis columns to the four en1998 DSL assets (idempotent, python -I). | safe_run |
| `en1998-v13/3-basis-kinds.base.py` | v11 (copy of v10 for wall_poisson_ratio): clones the member kind into one kind per wall plan field (wall_start | safe_run |
| `en1998-v13/3-basis-kinds.py` | v11 (copy of v10 for wall_poisson_ratio): clones the member kind into one kind per wall plan field (wall_start | safe_run |
| `en1998-v9/1-rename.py` | en1998 v9 step 1: renames wall_storey -> wall_storey_id and liquefiable_thickness_m -> liquefiable_layer in fi | safe_run |
| `en1998-v9/10-sqlite.py` | en1998 v9 step 10: SQLite snapshot for the retyped fields: member kind CHECK text, wall storey id text, liquef | read-only |
| `en1998-v9/11-fixtures.py` | en1998 v9 step 11: fixtures. wallStoreyId 1.0 -> "" (every storey), 2.0 -> "s2"; liquefiableLayer 0.0 -> false | safe_run |
| `en1998-v9/12-spectrum.py` | en1998 v9 step 12: the spectrum ground type follows the foundation soil layers (precedence documented in part_ | read-only |
| `en1998-v9/13-scene-test.py` | en1998 v9 step 13: scene test for the foundation-layer precedence of the spectrum. python -I | read-only |
| `en1998-v9/2-twins.py` | en1998 v9 step 2: model, diff and wire twins for MemberKind (role), liquefiableLayer (bool) and wallStoreyId ( | read-only |
| `en1998-v9/3-labels.py` | en1998 v9 step 3: label text of the retyped fields (liquefiable layer, wall storey id, member kind) across the | safe_run |
| `en1998-v9/4-leaves.py` | en1998 v9 step 4: payload types of the three retyped mutation leaves (liquefiable layer bool, wall storey id s | read-only |
| `en1998-v9/5-kernel.py` | en1998 v9 step 5: kernel (masonry walls grouped by storey reference, member kind), checks, scene call sites, f | read-only |
| `en1998-v9/6-scene-walls.py` | en1998 v9 step 6: scene masonry walls grouped by storey reference; column-size lookup by member kind; field me | read-only |
| `en1998-v9/7-tests.py` | en1998 v9 step 7: unit tests and oracle for the typed fields. python -I | read-only |
| `en1998-v9/8-tests2.py` | en1998 v9 step 8: compliance test helper and the numpy oracle for storey references. python -I | read-only |
| `en1998-v9/9-integrity.py` | en1998 v9 step 9: member wall storey references are validated against buildings[].storeys[]. python -I | read-only |
| `en1998-v9/lib.py` |  | safe_run |
| `en1999-stiffeners/add_stiffeners.py` | EN 1999 wave 2: adds TransverseStiffener as a snapshot collection after RingStiffener through model, root, fiv | safe_run |
| `en1999-stiffeners/ndp.py` | EN 1999 transverse web stiffeners: registers the unverified structure constants of the stiffness and strength  | read-only |
| `en1999-stiffeners/run_kinds.py` | EN 1999 transverse stiffener kinds: runs the leaf-kinds generator with a private stage (the shared stage belon | safe_run |
| `en1999-stiffeners/s6_extent.py` | Stage 6: recomputes the EN 1999 SQLite semantic contract (schemaBytes, tableWidths, rows and valueBytes of bot | safe_run |
| `en1999-stiffeners/spec_st.py` | EN 1999 wave 2 transverse web stiffener spec: one id-keyed collection `transverseStiffeners[]` on top of the w | read-only |
| `en1999-wave2/s0_split.py` | Stage 0: moves the four diff twin leaves of EN 1999 into sibling twin crates (pattern of en1993) so the diff c | safe_run |
| `en1999-wave2/s1_model.py` | Stage 1: adds WeldRun and SectionVertex to the EN 1999 model, root artifact, snapshot and root twins (rust, js | safe_run |
| `en1999-wave2/s2_diff.py` | Stage 2: diff lanes (rust, json, ts, graphql, proto) for WeldRun and SectionVertex cloned from the Placements  | safe_run |
| `en1999-wave2/s3_meta.py` | Stage 3: EN + DE field metadata (editor field-meta table and schema field-metadata.json) for WeldRun, SectionV | safe_run |
| `en1999-wave2/s4_sqlite.py` | Stage 4: SQLite contract of WeldRun and SectionVertex (sql, rust, ts, admission, tests, fixtures). Idempotent. | safe_run |
| `en1999-wave2/s6_extent.py` | Stage 6: recomputes the EN 1999 SQLite semantic contract (schemaBytes, tableWidths, rows and valueBytes of bot | safe_run |
| `en1999-wave2/s7_scene.py` | Stage 7: 3D weld beads, HAZ zones and outline extrusion, handles and diagrams for weld runs and outline vertic | safe_run |
| `en1999-wave2/spec.py` | EN 1999 wave 2 entity specs (weld runs, section outline vertices) shared by the generators in this folder. | read-only |
| `en1999-wave3/grid_torsion.py` | Prototype of the EN 1999 free-outline section solver (Prandtl torsion constant and warping constant on a Q4 ra | read-only |
| `en1999-wave3/run_proto.py` |  | read-only |
| `en1999-wave3/sp_cases.py` | Writes the three free-outline torsion cases (solid, closed box, I polygon) into third-party-extended.json with | safe_run |
| `en1999-wave3/sp_ref.py` |  | read-only |
| `en1999-wave4/miner_ref.py` | EN 1999 v4 reference for the fatigue spectrum: independent numpy implementation of the bilinear S-N curve and  | read-only |
| `en1999-wave4/wagner_ref.py` | EN 1999 v4 reference for the free-outline stability: mirror of Rust `free_outline_stability` (principal frame, | read-only |
| `en1999-wave5/add_entities.py` | EN 1999 wave 5: adds FatigueCategory, PatchLoad and RingStiffener as snapshot collections through model, root, | safe_run |
| `en1999-wave5/ndp.py` | EN 1999 v5: registers the unverified coefficients of the web crippling (1-4 6.1.7), transverse web load (1-1 6 | read-only |
| `en1999-wave5/s6_extent.py` | Stage 6: recomputes the EN 1999 SQLite semantic contract (schemaBytes, tableWidths, rows and valueBytes of bot | safe_run |
| `en1999-wave5/spec5.py` | EN 1999 wave 5 entity specs: fatigue categories, web patch loads, shell ring stiffeners. Shares the helper cla | read-only |
| `en1999-wave5/wave5_ref.py` | EN 1999 v5 references of the three new rules, third-party: ring stiffener composite section (sectionproperties | read-only |
| `en1999-wave6/ndp.py` | EN 1999 v6: registers the stiffener flexural rows, re-words the ten web crippling coefficient rows with exact  | read-only |
| `en1999-wave6/thin_walled_ref.py` | EN 1999 v6 reference of the open thin-walled extrusion constants, third-party: sectionproperties (FE warping a | read-only |
| `en1999-wave6/wave6_ref.py` | EN 1999 v6 references of the stiffener flexural rules, third-party: the Winkler strut critical load (scipy spa | read-only |
| `en1999-wave7/ref.py` |  | read-only |
| `feature-rows/append_rows.py` | Append check-outcome rows (pass/fail by the oracle utilisation) to a code's check-outcome feature. Usage: appe | safe_run |
| `feature-rows/crosscheck1992.py` | Third-party cross-check of the EN 1992 oracle values behind the new feature rows: M_Rd of the beam section (st | read-only |
| `feature-rows/crosscheck1998.py` | EN 1998 third-party reproduction with scipy: exact T1 of the shear building (scipy.linalg.eigh) against the or | read-only |
| `feature-rows/gen1992.py` | EN 1992 feature-rows fixtures: derived-load beams for 7.3 crack, 7.4 deflection, 7.4 l/d, 6.2 shear and 6.1 ac | safe_run |
| `feature-rows/gen1993.py` | EN 1993 feature-rows fixtures from the committed class1-beam-column oracle document: tension (6.2.3), shear (6 | safe_run |
| `feature-rows/gen1994.py` | EN 1994 feature-rows fixtures derived from the committed oracle documents: shear (vpl), construction, shear co | safe_run |
| `feature-rows/gen1998.py` | EN 1998 feature-rows fixtures derived from seismic-rich-pass and seismic-rayleigh-cracked: pass/fail for the f | safe_run |
| `feature-rows/gen1999.py` | EN 1999 feature-rows fixtures derived from web-stiffener-pass: an overloaded variant (members, connection, she | safe_run |
| `field-adder/en1990_profile.py` | EN 1990 profile of the field-adder: adds `f64` fields to the `members` and `bridgeSls` rows through the schema | read-only |
| `field-adder/en1996_profile.py` | EN 1996 profile of the field-adder (wave 2, owner v5): adds scalar and enum fields to the `MasonryWall` row an | read-only |
| `field-adder/en1997_profile.py` | EN 1997 profile of the field-adder: adds one `f64` field to a row of the snapshot (`slopes`, `piles`, `retaini | read-only |
| `field-adder/en1998_profile.py` | EN 1998 profile of the field-adder: dispatches the spec fields of `specs/en1998-*.json` to the root (site, bui | safe_run |
| `field-adder/en1998_root.py` | EN 1998 field-adder, root entities (site, building): appends one plain f64 field through every layer, idempote | safe_run |
| `field-adder/en1998_rows.py` | EN 1998 field-adder, row entities (bridge, assessment, foundation, tower, member): appends one plain f64 field | safe_run |
| `field-adder/field_adder.py` | Schema-first FIELD-ADDER: adds one snapshot field to a record of a norm artifact through every layer, idempote | safe_run |
| `field-adder/jsonspan.py` | 🧵 Structure-preserving JSON text editor: parses a JSON text into nodes that carry their character spans and in | read-only |
| `field-adder/new_collection.py` | Schema-first NEW-COLLECTION template: adds id-keyed row collections to a norm artifact through every layer, id | safe_run |
| `field-adder/schema_twins.py` | Schema-first bridge of the field-adder and collection tools: a field, a record or a collection is written into | safe_run |
| `forms-v3/audit.py` |  | read-only |
| `forms-v3/coverage.py` | Grip coverage audit of the six design-case catalogues (FORMS-V3): declared roles of every geometry row against | safe_run |
| `forms-v3/keys.py` |  | safe_run |
| `forms-v3/show.py` |  | safe_run |
| `forms-v4/scan.py` | FORMS-V4: lists (and with --write declares) the geometric case inputs (length, angle units) that no geometry r | safe_run |
| `kind-convention/en1999_collections.py` | 🗂️ Insert/remove kinds for the seven top-level en1999 collections (materials, sections, connections, fire scen | native additive + --check (Files) |
| `kind-convention/en1999_collections_repair.py` | 🩹 Re-adds the 14 collection insert/remove kinds to the en1999 root enum, KINDS and binary protocol after a con | safe_run |
| `kind-convention/en1999_collections_split.py` | ✂️ Splits the en1999 `collection-rows` crate (43 files, over the 32-file cap) into `collection-rows` (material | safe_run |
| `kind-convention/rename.py` | 🏷️ Applies the kind-name convention rename through every layer (dirs, rs/json/ts/graphql/proto/py/feature/semi | guarded (TH_FORCE=1) |
| `kind-convention/retire_en1998_site.py` | 🗑️ Retires the whole-site `update-site` kind and the dead `replace_all` helper of en1998 through every layer.  | guarded (TH_FORCE=1) |
| `kind-convention-v2/en1991_accidental_force.py` | 🚗 The one en1991 index selector the lane template cannot generate: `change-accidental-assumed-force` addresses | safe_run |
| `kind-convention-v2/leaf_coverage.py` | 🧮 Content-leaf coverage of an artifact: every leaf of the snapshot JSON Schema against the `SetFieldRule` path | read-only |
| `kind-convention-v2/migrate_ids.py` | 🆔 Index selectors to id selectors for the set, insert and remove kinds of one norm artifact. | guarded (TH_FORCE=1) |
| `kind-convention-v2/survey.py` | Survey: for every edit rule of an artifact, which list rows addressed by index lack a stable `id` in the snaps | read-only |
| `kind-convention-v2/verify_fixtures.py` | 🧪 Third-party-style check of the committed mutation vectors of one artifact with the independent Python norm v | read-only |
| `kind-convention-v3/cases_ids.py` | 🆔 `remove-case` by design-case id for the six artifacts that embed the shared `cases` table (en1990, en1991, e | safe_run |
| `kind-convention-v3/en1992_tightness_diff.py` | 🩹 EN 1992 `members[].tightness` in the diff: the local `nested_patch!` macro gains an `optional { field: Type  | safe_run |
| `kind-convention-v3/en1992_tightness_kinds.py` | 🚿 EN 1992 `members[].tightness` set and clear kinds: `change-member-tightness {memberId, newTightness}` and `c | safe_run |
| `kind-convention-v3/en1994_lists.py` | 📋 EN 1994 diff lanes on `list_delta!`: the positional hand-written lanes `beams`, `columns`, `slabs`, `placeme | safe_run |
| `kind-convention-v3/en1994_lists_twins.py` | 🔣 EN 1994 diff twins (json schema, ts, graphql, proto) and committed diff vectors for the keyed `list_delta!`  | safe_run |
| `kind-convention-v3/en1998_nested.py` | 🏢 EN 1998 nested building rows: `insert-/remove-` `member`, `storey`, `system` of `buildings[]`. | safe_run |
| `kind-convention-v3/id_rules.py` | 🆔 Rewrites one artifact's edit rules so that a list is removed by row id: the `RemoveItemRule` of the list bec | read-only |
| `kind-convention-v4/en1993_single_base.py` | 🧪 Regenerates the before/after snapshots of the en1993 insert/remove frame-bar, frame-node and joint-row vecto | safe_run |
| `kind-convention-v4/verify_fixtures.py` | 🧪 Third-party-style check of the committed mutation vectors of one artifact (v4: collections and selectors com | read-only |
| `leaf-kinds/binary_varint.py` | Rewrites an artifact's binary mutation codec to a varint tag looked up in `📡️.protocol.semio` (schema-first) p | guarded (TH_FORCE=1) |
| `leaf-kinds/common.py` | Shared helpers of the leaf-kinds generator: naming, staged writes, schema reading. | safe_run |
| `leaf-kinds/delta.py` | Delta-macro variant of the leaf-kinds generator: id-keyed rows whose diff lane is a `list_delta!` row delta wi | read-only |
| `leaf-kinds/diff.py` | Regenerates the sparse per-row patch lanes of an id-keyed diff crate from the entity leaves: rust, json, ts, g | read-only |
| `leaf-kinds/diff97.py` | Macro-diff template (`protocol::row_patch!` / `list_delta!`) for id-keyed artifacts: set-list rewrite, nested  | read-only |
| `leaf-kinds/en1990_run.py` | EN 1990 profile and driver: python -I en1990_run.py [--apply]. Typed set-field kinds per row leaf (selector `i | native additive + --check (Files) |
| `leaf-kinds/en1991_run.py` | EN 1991 leaf-kinds F: python -I en1991_run.py [--apply]. Id-selector set-field kinds for every content leaf wi | native additive + --check (Files) |
| `leaf-kinds/en1992_diff.py` | Regenerates the En1992 diff crate (rust patches/lanes) and its json/ts/graphql/proto twins from the entity lea | read-only |
| `leaf-kinds/en1992_placements_run.py` | EN 1992 placements driver (scene v4): python -I en1992_placements_run.py [--apply]. Retires the eight inline ` | native additive + --check (Files) |
| `leaf-kinds/en1992_run.py` | EN 1992 profile and driver: python -I en1992_run.py [--apply]. One typed set-field kind per snapshot leaf, ins | native additive + --check (Files) |
| `leaf-kinds/en1993.py` | EN 1993 profile and driver (Delta variant): python -I en1993.py [--apply]. Stages under 🗑️generated/leaf-kinds | native additive + --check (Files) |
| `leaf-kinds/en1993_arrays.py` | EN 1993 array-element kinds: insert-/remove-/change- kinds for the eight `Vec` element lists (`actions`, `spec | safe_run |
| `leaf-kinds/en1993_base.py` | Realistic EN 1993 base document for mutation vectors: the high-strength-connection example plus the showcase r | read-only |
| `leaf-kinds/en1993_frame.py` | EN 1993 additive kinds for entities added after the first leaf pass (FrameNode, FrameBar, Placement, materials | safe_run |
| `leaf-kinds/en1993_regen_fixtures.py` | Rewrites the generated EN 1993 leaf and row vectors from the realistic base document (examples + showcase rows | safe_run |
| `leaf-kinds/en1993_twins.py` | Splits the four schema twins of the EN 1993 mutation root into sibling twin crates (like the diff crate) so th | safe_run |
| `leaf-kinds/en1993_wire.py` | EN 1993 wiring: group crates, row kinds cloned from the crane template, externally tagged root twins, edit rul | safe_run |
| `leaf-kinds/en1994_action_leaves.py` | EN 1994 nested action position leaves: `change-<beam/slab>-action-x-m` and `change-slab-action-y-m` for `beams | native additive + --check (Files) |
| `leaf-kinds/en1994_actions.py` | EN 1994 nested action kinds: `insert-<entity>-action` / `remove-<entity>-action` for `beams/columns/slabs[].ac | native additive + --check (Files) |
| `leaf-kinds/en1997.py` | EN 1997 profile and driver (macro diff, id selectors, nested lanes): python -I en1997.py [--apply]. | native additive + --check (Files) |
| `leaf-kinds/en1997_nested.py` | EN 1997 nested row kinds: insert-/remove-<row> for loadCases, testProfiles, tests, points addressed by parent  | native additive + --check (Files) |
| `leaf-kinds/en1997_structure.py` | EN 1997 top-level leaf `structureId`: clones the `change-design-situation` kind (string scalar, no row selecto | native additive + --check (Files) |
| `leaf-kinds/en1999.py` | EN 1999 profile and driver: python -I en1999.py [--apply]. | native additive + --check (Files) |
| `leaf-kinds/en1999_entity_rows.py` | 🗂️ Insert/remove kinds for the wave-5 en1999 collections (fatigue categories, web patch loads, ring stiffeners | native additive + --check (Files) |
| `leaf-kinds/en1999_nested.py` | 🪆 EN 1999 nested row kinds: insert-/remove- member-action, connection-action, section-element, sheet-action, s | native additive + --check (Files) |
| `leaf-kinds/en1999_rows.py` | EN 1999 row insert/remove kinds (insert-/remove- weld run, outline vertex, placement) cloned from the insert-m | safe_run |
| `leaf-kinds/fixtures.py` | Committed mutation vectors (before, mutation, after, outcome, diff) of generated kinds, and diff padding of ex | safe_run |
| `leaf-kinds/index_selector.py` | Index-selector template of the leaf-kinds generator: set-field, insert-row and remove-row kinds for keyed `lis | read-only |
| `leaf-kinds/keyed.py` | Wires `index_selector` kinds into one keyed-diff artifact: group crates, registration (Cargo.toml, Cargo.lock, | native additive + --check (Files) |
| `leaf-kinds/keyed_fixtures.py` | Committed `✅apply` vectors (before, mutation, after, outcome, diff) of `index_selector` kinds, and null-paddin | safe_run |
| `leaf-kinds/leaf.py` | Per-kind files of an id-keyed artifact: payload leaf, diff, inverse, schema json/ts, catalogue json. | read-only |
| `leaf-kinds/lock93.py` | Adds the new EN 1993 mutation crates to the Cargo.lock files: package blocks sorted like cargo, and the depend | safe_run |
| `leaf-kinds/lock97.py` | Adds the new EN 1997 mutation crates to the Cargo.lock files: package blocks sorted like cargo, and the depend | safe_run |
| `leaf-kinds/lock_placements.py` | Adds `semio-s-artifact-norm-<art>-mut-placements` to the outer Cargo.lock files (hub, bridge, fixture-sweep):  | safe_run |
| `leaf-kinds/model.py` | Reads an id-keyed artifact snapshot schema into entities and leaves (schema-first). | read-only |
| `leaf-kinds/placements.py` | Placement kinds profile (leaf-kinds E): python -I placements.py <en1996/en1997> [--apply]. Four `change-placem | native additive + --check (Files) |
| `leaf-kinds/placements94.py` | EN 1994 positional `placements` lane (`En1994PlacementsRows { removed, inserted, modified }`): diff bodies for | safe_run |
| `leaf-kinds/rename.py` | Renames generated kinds to their short names across one artifact tree (dirs, text, taxonomy, oracle), then enf | guarded (TH_FORCE=1) |
| `leaf-kinds/rename_en1999.py` |  | guarded (TH_FORCE=1) |
| `leaf-kinds/rename_groups_en1999.py` | Renames the numbered en1999 group crates (`-2`) to family names across dirs, Cargo, script.ts and build-owners | guarded (TH_FORCE=1) |
| `leaf-kinds/retire_en1997.py` | Retires the 11 whole-row `update-<row>` kinds of en1997 through every layer, rewrites `resolve` to the typed r | guarded (TH_FORCE=1) |
| `leaf-kinds/retire_en1999.py` | Retires the 8 whole-collection kinds of en1999 through every layer (idempotent). | guarded (TH_FORCE=1) |
| `leaf-kinds/wire.py` | Wires generated kinds into one id-keyed artifact: group crates, root enum, twins, codecs, edit rules, harness, | safe_run |
| `mechanics-section/cases.py` | Blessed section cases (SI metres): polygons shared by the prototype, the sectionproperties oracle and the Rust | read-only |
| `mechanics-section/check_profiles.py` |  | read-only |
| `mechanics-section/profiles_proto.py` | Python mirror of the Rust profile builders, compared with the sectionproperties library polygons and (via prot | read-only |
| `mechanics-section/proto.py` | Python mirror of the Rust section kernel (mesh: ear clipping with hole bridges + Rivara 4T-LE refinement; T6 S | read-only |
| `mechanics-section/run_proto.py` |  | read-only |
| `mechanics-section/run_thin.py` |  | read-only |
| `mechanics-section/sp_ref.py` | sectionproperties reference values for the blessed cases (run with the repo venv, -I). | read-only |
| `mechanics-section/thin_proto.py` | Python mirror of the thin-walled open-section solver (centreline walls, open tree). | read-only |
| `path-budget/align_dirs_with_oracle.py` | 🧭 One-shot: renames fixture kind folders that still carry `sourceMutationDirectoryName` to the `mutationDirect | safe_run |
| `path-budget/flatten_fixtures.py` | 🧭 One-shot: moves every mutation vector bundle of the shared harness users (norm, block) from the nested role  | safe_run |
| `path-budget/rewrite_generators.py` | 🧭 One-shot: moves the ticket's fixture generators to the flat role files, on lines that write or read a mutati | safe_run |
| `path-budget/rewrite_refs.py` | 🧭 One-shot: rewrite the fixture role paths in text files from the nested layout to the flat one. | safe_run |
| `path-budget/shorten_module_dirs.py` | 🧭 One-shot: shortens the 13 module folders whose files breach the 240 byte path budget; renames the folder and | safe_run |
| `path-budget/verify_refs.py` | 🔎 Every flat fixture role reference in the norm and block plugins resolves to a committed file, and no nested  | read-only |
| `placements-adder/en1990.py` |  | read-only |
| `placements-adder/en1991.py` |  | read-only |
| `placements-adder/en1992.py` |  | read-only |
| `placements-adder/en1993.py` |  | read-only |
| `placements-adder/en1994.py` |  | read-only |
| `placements-adder/en1995.py` |  | read-only |
| `placements-adder/en1996.py` |  | read-only |
| `placements-adder/en1997.py` |  | read-only |
| `placements-adder/en1998.py` |  | read-only |
| `placements-adder/padd.py` | Adds the persisted `placements` collection (id, x, y, z, yaw) to a norm artifact by cloning the artifact's own | safe_run |
| `rc-threading/core_en1992.py` |  | read-only |
| `rc-threading/core_en1993.py` |  | read-only |
| `rc-threading/core_en1994.py` |  | read-only |
| `rc-threading/core_en1995.py` |  | read-only |
| `rc-threading/core_en1996.py` |  | read-only |
| `rc-threading/core_en1997.py` |  | read-only |
| `rc-threading/core_en1998.py` |  | read-only |
| `rc-threading/core_en1999.py` |  | read-only |
| `rc-threading/rc_leaf.py` | Clones the `change-annex` leaf as `change-reliability-class` through its whole registration footprint of one n | read-only |
| `rc-threading/rc_lib.py` | Helpers of the rc-threading tool: idempotent text edits that clone the root `annex` entry as `reliabilityClass | safe_run |
| `rc-threading/rc_run.py` | Shared driver steps of the rc-threading v2 tool: schema twins, leaf kind, fixtures and DSL assets of one artif | read-only |
| `rc-threading/thread_en1994.py` |  | read-only |
| `rc-threading/thread_en1996.py` |  | read-only |
| `schema-additions/en1997_concrete_class_contract.py` | en1997 `piles[].concreteClass` SQLite contract: schemaBytes/tableWidths replayed with python sqlite3 from the  | safe_run |
| `schema-additions/en1997_concrete_class_fixtures.py` | en1997 `piles[].concreteClass` in every mutation fixture (snapshots `before`/`after`, diffs, insert-pile paylo | safe_run |
| `schema-additions/en1997_concrete_class_schema.py` | en1997 `piles[].concreteClass`: enum def + property in the snapshot and artifact JSON, patch property in the d | read-only |
| `schema-additions/en1997_concrete_class_sqlite.py` | en1997 `piles[].concreteClass` in the SQLite codec (sql, rs, ts, admission), idempotent; the contract numbers  | safe_run |
| `schema-codegen/batch.sh` |  | read-only |
| `schema-codegen/classify.py` | Classifies every content difference between the committed twins (BEFORE tree) and the generated twins (AFTER t | read-only |
| `schema-codegen/importcheck.py` |  | read-only |
| `schema-codegen/roundtrip.sh` |  | read-only |
| `schema-sync/check.py` | 🧬 Schema-sync checker: the JSON schema (source of truth) against the Rust, TypeScript, proto and GraphQL twins | safe_run |
| `schema-sync/sync.py` | Schema-sync fixer. Phase A: JSON schema := intended current Rust model. Phase B: TS, proto and GraphQL twins : | safe_run |
| `showcase/common.py` | Shared helpers of the showcase-document generators (python3 -I). A showcase document is built from committed t | safe_run |
| `showcase/en1993.py` | Generates the EN 1993 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (python3 -I en1993.py). | read-only |
| `showcase/en1994.py` | Generates the EN 1994 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: one bridge-kind composite str | read-only |
| `showcase/en1998.py` | Generates the EN 1998 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (python3 -I en1998.py). | read-only |
| `showcase/en1999.py` | Generates the EN 1999 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: the committed neutral 14-coll | read-only |
| `spec-trace/migrate.py` |  | safe_run |
| `spec-trace/spec_ids.py` |  | read-only |
| `typed-enums/census.py` |  | read-only |
| `typed-enums/convert.py` | 🧬 Typed-enum generator: turns one `String` field of a norm artifact into a schema enum in every twin, codec, l | safe_run |
| `typed-enums/convert_selftest.py` | Self-test of the re-run safety of `convert.py`: a value that already is a variant is never rewritten (no defau | read-only |
| `typed-enums/en1991_manual.py` | One-shot: hand edits for the en1991 `FireOccupancy` conversion (document-facing fire load functions take the e | safe_run |
| `typed-enums/en1992_bond_condition.py` | One-shot: en1992 BarLayer.bond_condition String -> enum BondCondition { Good, Poor }. Every replacement assert | safe_run |
| `typed-enums/en1992_bond_condition_fixtures.py` | One-shot: en1992 fixtures for BondCondition. Wire tokens `good`/`poor` become `Good`/`Poor`; the apply fixture | safe_run |
| `typed-enums/en1992_manual.py` | One-shot: hand edits of en1992 call sites that take the typed enums (functions with `&str` parameters, Display | safe_run |
| `typed-enums/en1992_remove_column_method.py` | One-shot: delete the dead en1992 `FireSpec.column_method` in every layer (model, twins, diff lane, leaf `chang | safe_run |
| `typed-enums/en1993_manual.py` | One-shot: the hand-written remainder of the en1993 StiffenerShape / WarpingSupport conversion (diff twins, gra | safe_run |
| `typed-enums/en1995_manual.py` | One-shot: hand edits for the en1995 conversions (string-keyed helpers take the typed enums, Display uses, sqli | safe_run |
| `typed-enums/en1995_snow_high.py` | EN 1995 `ImposedCategory::SnowHigh`: the EN 1990 Table A1.1 snow row for sites above 1000 m a.s.l. (own psi0/p | safe_run |
| `typed-enums/en1995_strength_audit.py` | Audit of en1995 strength-class values in data files: compares the ordered strength-class tokens of every data  | read-only |
| `typed-enums/en1995_strength_audit_flat.py` | Audit of en1995 `strengthClass` values in the flattened fixture tree (PATH-BUDGET layout): maps every flat fil | read-only |
| `typed-enums/en1995_strength_recover.py` | Exact recovery of the en1995 strength-class fixture values that the v2 repair guessed ("most common pattern"). | safe_run |
| `typed-enums/en1995_strength_repair.py` | One-shot repair: the first en1995 StrengthClass run built its variants from the editor list (`GL28h`, wire `gL | safe_run |
| `typed-enums/en1999_manual.py` | One-shot: hand edits of en1999 call sites that take the typed enums (`&str` parameters, Display uses, validati | safe_run |
| `typed-enums/probe.py` |  | read-only |
