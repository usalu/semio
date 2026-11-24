# en1990 twins and viewport probe, 2026-11-24

## 1. Schema twins and wire fixtures

### Causes

- `FactorOverride.factor` carried a JSON `enum` (schema-labels agent) but stayed a `String` in the Rust model. Three things followed from that single gap:
  - the generator refused the record: `FactorOverride.factor has no derivable Rust type (an inline enum ...): state it with "x-semio-rust": { "type": ... }` (the `schema twins check --strict` problem),
  - the TypeScript twins of the snapshot and the diff were stale (committed `normWireString`, generated `normWireLiteral("gamma", ...)`, `drift=2`),
  - the sync checker reported `json enum [...] vs rust string` / `vs ts string` (3 type mismatches: snapshot rust, snapshot ts, diff ts).
- The 42 wire-twin failures had a different, older cause. Every committed en1990 mutation bundle that carries a factor override row (all bridge-damping, member, traffic-group, factor-override bundles) held the generic placeholder `"factor": "normal"` (and `"normal-alt"` as `newFactor`) from the original fixture authoring. These were never members of the enum, so the strict Ajv oracle refused the `before`, `after` and `diff` files of those bundles. The schema twins gap did not cause them; the enum made the oracle strict about them.
- Not the mutation-vectors agent's area: its `diff.json` regeneration concerns en1999 `reliabilityClass`/`forceMethod` nulls. No `diff.json` was regenerated here; only the placeholder token inside existing files changed.

### Fixes

1. Typed enum, schema-first as for `LoadGroup`:
   - `FactorOverride.factor` in `📸️snapshot/🔣️.json` now carries `"x-semio-rust": {"type": "FactorKind"}`.
   - New `FactorKind { Gamma, Psi0, Psi1, Psi2, Xi, KFi }` in `📸️snapshot/🦀️.rs` (wire ids `gamma psi0 psi1 psi2 xi k_fi`, `ALL`, `as_str`, `from_id`, `per_action`, `Display`), derive set identical to the sibling enums.
   - Leaf `change-factor-override-factor`: `newFactor` is now the enum with the select widget and `x-semio-rust.type = ...::FactorKind`.
2. Twins regenerated with the project generator, no hand edits:
   `repo ./📜️script.ts schema twins generate --under "✏️s/🔌️plugins/📕️norm"` wrote 6 files (snapshot rust and ts, diff ts, leaf rust and ts). Afterwards `schema twins check --under ... --strict`: `identical=8321 style=0 drift=0` (json/ts/proto/graphql/rust all 0 drift).
3. Rust use sites typed:
   - diff macro `En1990FactorOverridePatch` (`factor: FactorKind`),
   - sqlite snapshot (Rust: `row.factor.as_str()`, `read_factor_kind`; TS reader: `choice(..., ["gamma", ...])`),
   - `🎚️factor-overrides` inference: `allowance_id`, `value_ok`, `verdicts`, `action_override`, `global_override`, `annex_global`, `annex_for_action`, `deviations`, `rows_for`, `annex_value`, `push_factor_overrides` all match on `FactorKind`. `Verdict::UnknownKind` and the `KINDS` string table are gone: an unknown kind is no longer representable, the wire refuses it.
   - scene: `factor_symbol(FactorKind)` shared by the badges and the handles; `FactorKind::per_action()` replaces three `matches!(.., "gamma" | "psi0" | "psi1" | "psi2")`.
   - editor field meta: `factorOverrides[].factor` is now a choice field (`FACTOR_KINDS`, labels from the schema options), as the field-meta choices law requires for enum fields.
   - tests: `override_row` and the scene unit test build typed rows; the verdict test lost its `chi` row (`UnknownKind` verdict) and expects 3 failing override checks instead of 4.
4. Wire fixtures: the placeholder tokens were rewritten by `rewrite-en1990-factor-placeholders.py` (this ticket folder): `normal` -> `gamma`, `normal-alt` -> `psi0` in `factor`/`newFactor` members of every role file under `🧪️tests/🧫️fixtures/🧬️mutations` (43 files, before, after, mutation and diff alike, so `after == apply(before, mutation)` and the diff stay consistent by construction). There is no generator that authors these bundles (the committed fixtures are the source; `SEMIO_MUTATION_VECTORS_PRODUCED_DIR` only reproduces diffs), so the token rewrite is the same kind of scripted bundle edit that `rc-threading` used.
5. Unrelated defect fixed on the way: `🧬️mutations/🧪️tests/🔬️unit` called `MutationDiff::absorb` (moved to `ColdMutationDiff`, bare trait path no longer compiles); now `absorbed.absorb(..)` with `ColdMutationDiff` in scope.

### Test lines

- `"$T/run-norm-js-native.sh"` (log `🗑️generated/norm-js-native.log`): before 12006 pass / 44 fail, after **12049 pass / 1 fail**. The remaining failure is `field-meta choices > every choice token is a member of the schema enum it edits`: `🐍️check.py` reports `choices=286 mismatches=2`, both in en1993 (`members[].momentDiagram`, `fireExposures[].rating`: labels differ from the schema options, run `🐍️generate.py`). en1990 itself reports `mismatches=0`. en1993 is being edited by another agent; not touched here.
  - schema sync "every JSON schema, its Rust model and its TypeScript, proto and GraphQL twins agree": pass.
  - "every committed twin equals what its JSON Schema generates": pass.
  - `norm wire twins > ⚖️en1990 > ...`: all 42 pass.
- Rust, run from the en1990 workspace with `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-<crate> ... --offline` (script `run-en1990-rust-tests.sh`; logs `🗑️generated/en1990-rust-others.log`, `en1990-rust-mutations-*.log`):
  - model: builds, 0 tests. diff: 0 tests. inferences: 6 passed. io: 5 passed. io-text: 2 passed.
  - mut-factor-overrides, mut-factor-overrides-rows, mut-traffic-groups, schema: build, no failing test.
  - scene lib: 73 passed, 1 failed (`unit::projecting_a_thousand_members_stays_interactive`: `projection took 2.5s`, a wall-clock gate on a machine under heavy build load).
  - editor lib and `integration`: 42 passed, 9 failed in `integration`, and 6 `live_acceptance`/incremental failures in the lib run. Reasons seen, none touching factor overrides:
    - `kind 'combination': the document has no collection 'combinations'` and `kind 'accidental': ... accidentals.0.expected an object, found Null` (palette and insert kinds, shared app-surface),
    - `every_unit_declares_all_it_reads`: undeclared reads of `members.N.*` in `global:document`,
    - asset tests: `supervisionLevel.unknown variant DSL3`, `expected Enum, found Text("building")`, `.dsl.semio is stale (2728 bytes, expected 2728)` (DSL scalar spelling drift in the committed `.dsl.semio` and pack assets),
    - `history_edits_end_to_end`, `conflict_history_edits_end_to_end`: conflict law on `change-reference-period-years`.
    I did not run these against the pre-change tree (no stash allowed), so "unrelated" rests on the messages: no failing assertion names a factor override, `FactorKind` or an en1990 fixture I changed.
    The editor lib `latency_bench` test ran over 15 minutes and was killed (pid-specific kill) so cargo could continue.
  - mutations: the lib test target did not compile before the `absorb` fix (above); `replay_preparation_physical` still does not compile (20 errors, known from the mutation-vectors note), so the vectors target is run alone (`en1990-rust-mutations-vectors.log`): `committed_vectors_hold` FAILED at its first bad bundle, `change-reliability-member-id/apply`: `mutation.apply.duplicate-target ... permanents inserted 1`, the duplicate permanent key `G` in `before.json` that `mutation-vectors-2026-11-23.md` already lists (with `change-test-pairs` and `change-test-theory-variation`). The law stops at the first bundle, so it did not reach the bundles rewritten here; the lib unit target was not rerun after the `absorb` fix (a rerun costs a full queue slot).

## 2. en1990 viewport Play probe

### Cause

The probe is not failing on en1990's switching or drawing. At stage `alternate example` it picks the `__none__` ("No example") row, because the current host offers it (en1992's passing log of 00:56 shows `alternative.blank: false`, i.e. the host had no blank row then). It then waits until the model viewport holds **0 instances** (`if (blank) return instances === 0`).

For the Eurocode families the blank example is the framework's `D::default()` (`roster_example_snapshot`: empty id is the initial snapshot), and `En1990Snapshot::default()` is by documented design the worked example (`example(AnnexChoice::En)`; the same holds for en1991 to en1999). So "No example" correctly loads a populated, drawn scene. Observed in a live Play session: picker `No example`, `data-world-geometry-revision` 1 -> 2, 73 instances, `canvas[data-world-geometry-drawn]` at revision 2 (initial example had 98 instances). The probe's own failure dump shows the same: picker `No example`, model still full of geometry. The 180 s timeout was the unmet `instances === 0`.

en1992 would fail the same way against the current host; its passing log predates the blank row.

### Fix (at the root: the wrong expectation)

`probe-eurocode-viewports.mjs`: the blank special case is removed. After any alternative, including "No example", the probe requires the picker label, a new geometry revision, instances > 0 and the canvas drawn at that revision (the same check as for a real alternative). No guest code changed for this, so the `norm-eurocode` guest was not rebuilt for part 2.

### Outcome

Before: `status: failed`, `stage: alternate example`, `TimeoutError 180000ms`, `exampleAtFailure: "No example"` (`🗑️generated/norm-published-eurocode-viewport-en1990.before.log`).

After (`SEMIO_TECH_PLAY_PORT=6039 VERIFY_SIGNED_MODULES=1 PLAY_TEST_PICKER_ROUNDTRIP=1 PLAY_TEST_OBSERVE_ARCHIVES=1 PLAY_TEST_REQUIRE_DRAW=1 PLAY_TEST_PICKER_WAIT_MS=120000 python3 probe-norm-base-companions.py en1990`, `🗑️generated/probe-en1990-after.log`): `[DEBUG] en1990 browser outcome 0`, `status: passed`, faults `[]`, failedRequests `[]`, initial 98 instances at revision 1, alternative `No example` at revision 2, restored 98 instances at revision 3 drawn, 1240 model colours, camera zoom changed, diagram 1220 colours, 42 s.

The probe ran against the published guest built before the Rust changes of part 1. Those changes touch en1990 crates that are part of the `norm-eurocode` guest, so a rebuild (`rebuild-and-probe.sh e1990 norm-eurocode en1990`) is still due before the next published delivery; it was not started because the compiler image queue was already blocked by other builds and part 2 does not depend on it.

## Changed files

Schema and generated twins (en1990 subset `🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `🧬️schema/📸️snapshot/🔣️.json`, `🧬️mutations/🎚️change-factor-override-factor/🧬️schema/🔣️.json` (source)
- generated: `🧬️schema/📸️snapshot/🟦️.ts`, `🧬️schema/🔺️diff/🟦️.ts`, `🧬️mutations/🎚️change-factor-override-factor/🧬️schema/🟦️.ts`, `🧬️mutations/🎚️change-factor-override-factor/🦀️.rs`, `🧬️schema/📸️snapshot/🦀️.rs` (struct field generated, `FactorKind` enum hand-written beside `LoadGroup`)

Rust and TypeScript use sites: `🧬️schema/🔺️diff/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, `🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`, `🧬️schema/💡️inferences/🦀️.rs`, `🧬️schema/💡️inferences/🎚️factor-overrides/🦀️.rs`, `🧬️schema/💡️inferences/🧰️report-helpers/🦀️.rs`, `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`, `🎬️scene/⚓️actions/🦀️.rs`, `🎬️scene/🕹️handles/🦀️.rs`, `🎬️scene/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🏷️field-meta/🦀️.rs`, `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`.

Fixtures: 43 files under `🧪️tests/🧫️fixtures/🧬️mutations/` (bundles `change-bridge-damping-ratio`, `change-factor-override-{factor,justif,target,value}`, `insert-factor-override`, `remove-factor-override`, nine `change-member-*`, `change-traffic-group-{group,variable-id}`, `insert-traffic-group`, `remove-traffic-group`).

Ticket folder: `rewrite-en1990-factor-placeholders.py`, `run-en1990-rust-tests.sh`, `probe-eurocode-viewports.mjs` (blank expectation), this report. Generated copies `*.before.log` sit under `🗑️generated/`.

## Open

- Rust vector law (`mutation_vectors`) cannot confirm the rewritten bundles until the three duplicate-`G` `before.json` of the mutation-vectors agent are fixed; the rewrite replaces one token consistently across before, after, mutation and diff, and the strict Ajv oracle plus generated TS twins (wire-twins suite) accept all 42 bundles.
- en1993 field-meta label drift (2 mismatches) keeps `field-meta choices` red; owner is the en1993 work.
- Editor/scene Rust failures listed above are in shared app-surface, asset and palette code, outside this change.
- Rebuild of the `norm-eurocode` guest before the next publication.
