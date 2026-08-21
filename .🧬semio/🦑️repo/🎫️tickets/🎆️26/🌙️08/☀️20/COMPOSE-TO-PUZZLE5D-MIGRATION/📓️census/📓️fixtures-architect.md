# 🏛️ Architect / program — 266 handcrafted mutation fixtures

Tree: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Scope: all 266 mutation leaves, one `🧪️tests/<case>/` each. 1596 new files + 1 modified file.

## 📊️ Result

```
🧬️ 115 artifact mutation trees · 1558 mutations · 1230 covered · 328 uncovered
(architect/program is absent from the --by-tree uncovered list and from the error list)

scoped re-run of the lint's own rules, architect/program only:
variants=266 leaves=266 covered=266 uncovered=0
ERRORS=0  WARNINGS(derived-encoding, expected)=2116
```

The repo-wide CLI truncates its error list at 40 rows, so the lint's rules
(`lintArtifact`/`lintCase` from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`) were
re-implemented verbatim and re-run scoped to this tree only —
`🔧️architect-fixture-tooling/scoped_lint.py`. **0 errors.** The 2116 warnings are the expected
derived-encoding gaps (`.op.semio`/`.spr.semio`/`.patch.semio`/`.dsl.semio`/`.pack.semio`) that
`fixtures generate` owns; none were hand-forged.

## 🧬️ How the 266 leaves decompose (read from source, never from names)

Every leaf's `🦠️mutation`, `🔺️diff` and `↩️inverse` was parsed and normalised; the tree collapses to
exactly **four** register templates plus ten bespoke leaves — verified by substitution-equality of
the diff bodies, not by directory naming:

| shape | count | diff the builder constructs |
|---|---|---|
| `create-*` | 62 | fatal `mutation.duplicate-id`, else `added = [row]` |
| `delete-*` / `disconnect-*` | 62 + 2 | error `mutation.target-missing`, else `removed = [id]` |
| `rename-*` | 62 | error / warn `mutation.no-op`, else `patched = [{id, {name: Some(..)}}]` |
| `replace-*` | 62 | error / warn, else `patched = [{id, FULL Patchable::diff_patch}]` |
| `connect-adjacency` | 1 | endpoint guard, `normalize_pair`, `normalized = true`, added-vs-patched branch |
| `connect-trace` | 1 | added-vs-patched branch, endpoints deliberately unchecked |
| `rename/replace` × `meta`/`project`/`governance` | 6 | whole-facet `Option<T>` scalar, no delta |
| `create/delete/rename/replace` × `knowledge`/`benchmarks` | 8 | composed `table` child handle, re-minted per call |

260 cases are `applied` (with a mandatory `🔺️diff/🔣️component.json`), 6 are `rejected`
(with `🔺️diff/🚫️component.absent`).

## 🔺️ The diff files

`ProgramDiff` is `#[serde(rename_all = "camelCase", default)]` with **no `skip_serializing_if` on any
of its 82 fields**, so serde emits every one — the committed diff JSON therefore carries all 82 keys,
`null` on the 81 the mutation must not touch. That is what makes assertion 5 load-bearing here: a
mutation that reached the right end state by writing a second collection would fail on a `null`.

`ProgramDiff.documents` is the delta for the snapshot field named `artifacts` (the apply arm reads
`self.documents → next.artifacts`). The committed **snapshot** key is `artifacts`, per the Rust
struct; note that `🧬️schema/📸️snapshot/🔣️component.json` disagrees and lists `documents` — that
generated JSON-schema file is stale against serde. Fixtures follow serde.

Patch JSON is the full declared field list of each `<Entity>Patch` (order = the `impl_patchable!`
order, checked equal for all 67 entities). `rename` emits `name` plus 30-odd `null`s; `replace`
emits every field, because `PatchRow::diff_row` snapshots `other`'s value unconditionally.

## 🧪️ The seven assertions

Applied cases: `applies_to_committed_after`, `inverse_restores_before`, `committed_json_is_canonical`
(both snapshots + the payload), `declared_outcome_holds` (status **and** "raised no diagnostic at
all"), `produces_committed_diff`, `committed_diff_is_canonical`, `committed_diff_applies_to_after`.

Rejected cases carry seven differently-worded assertions: `leaves_the_before_snapshot_untouched`,
`has_an_empty_inverse`, `committed_json_is_canonical`, `declared_rejection_holds` (asserts the
`mutation.target-missing` code **and** the offending path against the committed `🎯️outcome`),
`produces_no_diff` (`outcome.diff() == &ProgramDiff::default()`), `carries_the_absent_diff_marker`,
`after_snapshot_repeats_before`.

Test fn names are prefixed with the mutation kind (`create_stakeholder_produces_committed_diff`, …),
and every `expect`/`assert` message is prefixed `"<kind>/<case>: "`. Written de-async (no `.await`),
matching `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` and puzzle5d's reference fixture.

## 🔌️ Wiring

`📦️glue.rs` was **not touched**. The 266 `#[cfg(test)] mod` declarations live in this tree's own
`🧬️mutations/🦀️component.rs`, in a new `//#region 🧫️FixtureTests`:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🔗🧲connect-adjacency/🧪️tests/connects-reception-to-waiting/🦀️component.rs"]
    mod tests_connect_adjacency;
    …266 lines, sorted by module name so rustfmt's reorder_modules is a fixed point
}
```

## ✅️ Verification actually performed (cargo NOT run; no test is claimed to pass)

1. **Lint** — repo CLI + scoped re-run: **0 errors, 0 uncovered** (above).
2. **`#[path]` resolution** — all 266 targets exist on disk; independently, `rustfmt --check` on
   `🧬️mutations/🦀️component.rs` *recursed into all 266 mounted modules*, which only happens if every
   `#[path]` resolves.
3. **`include_str!` targets** — all 1332 references resolve (0 missing).
4. **rustfmt** — `rustfmt --edition 2021 --config-path rustfmt.toml --emit stdout` parses all 266
   test files and the mutations `🦀️component.rs`; after formatting, `--check` reports **0 diffs**
   across the root and every mounted module.
5. **Serde-shape validator** (`🔧️architect-fixture-tooling/validate.py`) — re-derives the emitted
   key set from the parsed Rust type table (flatten, `skip_serializing_if`, `rename_all`, enum
   variant renames) and checks every committed `before`/`after`/`mutation`/`diff` JSON is a decode →
   encode fixed point with no missing, extra, or wrongly-typed key: **266 cases, 0 problems**.
6. **Apply simulator** (`🔧️architect-fixture-tooling/sim.py`) — replays
   `ProgramDiff::apply_to_artifact` + `apply_collection_delta` + `impl_patchable`'s `apply_row`
   semantics in Python and asserts `before + committed diff == committed after` for every case:
   **266 cases, 0 failures**. This is an independent check of the `after` snapshots, not a restatement
   of how they were built.
7. **`ProgramArtifact::to_snapshot`** covers all 70 `ProgramSnapshot` fields (apply routes through it).

## ⚠️ Things a reviewer should know

- **`knowledge` / `benchmarks` are composed `table` children, not collections.** Their rows live in a
  `thread_local!` working-scene cache keyed by a content hash; a fresh test process has never
  populated it, so `program_knowledge(base)` / `program_benchmarks(base)` return `[]`. That makes
  `delete`/`rename`/`replace` on those two registers genuinely reject with `mutation.target-missing`
  — those are the 6 rejected fixtures, and the docstrings say exactly why. This holds under
  `--test-threads=1` too: the cache key is content-derived, so the empty-list key can only ever map
  to an empty list.
- **Two fixtures pin a `DefaultHasher` value.** `create-knowledge-record` / `create-benchmark-record`
  must commit the `childId` that `knowledge_child_from_records` mints, i.e.
  `format!("architect-knowledge-{:016x}", DefaultHasher(serde_json::to_string(&records)))`. It was
  computed with a standalone std-only `rustc` program on this toolchain (`hash.rs`), from the exact
  compact serialization of the committed row:
  `architect-knowledge-b3743ce016d5422b`, `architect-benchmarks-ebb8ef7bad26edae`. The empty-list
  handle `architect-{slot}-7904dd65836c8ff4` is the base snapshot's value in all 266 fixtures (it is
  what `empty_plugin()` produces). If `DefaultHasher` ever changes, these two fixtures move — that
  brittleness is in the production design, not in the fixture.
- **`Option<Option<T>>` patch fields do not survive JSON.** `TraceLinkPatch.label` (and every
  `Option<T>`-typed entity field routed through `PatchRow::diff_row`) encodes both `None` and
  `Some(None)` as `null`, so a committed diff can never express *clearing* such a field. Every
  fixture is authored so no `Option` field changes value: `connect-trace` takes the `added` branch,
  and `replace-*` only ever moves a non-`Option` field (`String`/`bool`/`f64`/enum/`Vec<String>`,
  49 distinct fields across the 62 leaves; the `header.name` fallback was never needed).
- **`disconnect-adjacency`'s before-snapshot carries both endpoint elements** even though the
  disconnect itself does not need them: its inverse is `connect-adjacency`, which *does* guard on
  endpoint existence, so without them assertion 2 would fail.
- Base snapshot is `empty_plugin()`-shaped with fixed ids (`document-fixture`, `project-fixture`,
  `governance-fixture`), `meta.title = "Fixture Program"`, `project.code = "FIX-000"`,
  `governance.framework = "ISO 9001"` — the three values the scalar `rename-*` leaves move.

## 🛠️ Tooling

`🔧️architect-fixture-tooling/` holds the parse → plan → build → emit → verify pipeline plus its
intermediate JSON (`types.json`, `plan.json`, `shape.json`, `fixtures.json`, `wiring.json`) so the
derivation is auditable and re-runnable. It is authoring tooling: nothing in it is imported by the
tests, which are 266 standalone files with no shared harness, helper, or macro.
