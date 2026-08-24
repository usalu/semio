# Wave 12 — 🧿️semio artifact, all 19 subsets handcrafted

Date 2026-08-24. Scope: the `🧿️semio` artifact only (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`).
Every command quoted below was actually run from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` and the exit code read from the tool's own status,
never through a pipe.

---

## 1. The structural fact that governs everything here

All 19 semio subsets record a **no-oracle decision**. `runPhases` (`📜️script.ts:531`) resolves the
oracle implementation from the feature's `@oracle-` tag; a no-oracle feature has none, so
`decision.implementation === null` and the oracle phase **can never execute a single scenario** for
any of them. `bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-<x>` prints
`not-exercised` by construction, for all 19, and will keep doing so.

That means the ORACLE handlers in these 19 adapters are the written statement of the reference
answer, not a running party — and every law each case claims had to be asserted inside the SUBJECT
handler or it is asserted nowhere.

**Before this wave, 3 of 18 cases did that (cad, document, flow). 15 did not.** Their subject
handlers applied the mutation, checked only that no `MutationOutcome` message was raised, and
returned the projection. Nothing ever consumed that projection (`parity=0/0`), so once the subject
phase unblocks those ~550 scenarios would pass iff `apply_semio_*_mutation` did not complain — the
committed after-snapshot was never compared against anything.

## 2. What was done

### 2.1 The 19th subset: `✳️drawing` now exists end to end

`semio v1 / ✳️drawing` had a handcrafted 17-kind vocabulary and 17 committed `(before, mutation,
after, diff)` fixtures, and was invisible to every gate: no `KINDS` const, no oracle manifest, no
catalog, no case.

* `…/✳️drawing/🧬️schema/🧬️mutations/🦀️component.rs` — added `pub const KINDS` (17 entries) and
  `kinds_match_the_enum_and_the_catalog`, which pins the const against the derive's own
  `SemioDrawingMutation::kinds()` AND against the committed manifest.
* `…/✳️drawing/🧪️oracle/🔣️component.json` — **new**. Records the `semio-drawing-mutation-semantics`
  no-oracle decision naming `usvg`/`resvg` and `lyon`/`kurbo` as SURVEYED AND DECLINED (with the
  reason: semio's drawing model has no node ids at all, so its structural `NodePath` addressing has
  no SVG counterpart and the four hierarchy verbs would be reimplemented on top of SVG rather than
  confirmed by it), plus the `semio-v1-drawing` catalog.
* `🧿️semio/🧪️tests/mutate-semio-drawing/` — **new** feature + adapter, 35 scenarios.

Coverage for this artifact is now **19/19**.

### 2.2 In-role assertions across all 19 cases

Every `mutate-<kind>` handler now decodes the committed AFTER snapshot and fails unless the applied
snapshot equals it. Every `inverse-<kind>` handler now fails unless the mutation's own computed
inverse restores the committed BEFORE snapshot. Failures print both documents.

Additionally fixed:
* `mutate-semio-image` / `mutate-semio-value` — `identity-round-trip` checked only that the
  full-replace was not rejected; it now asserts the rebuilt snapshot equals the committed one.
* `mutate-semio-animation` / `-audio` / `-video` — `identity-round-trip` now also asserts that the
  real committed artifact decodes to exactly the before-snapshot every specification vector starts
  from, so a mistake in the vectors is a red scenario rather than a quietly agreeable one.
* `mutate-semio-any` — the envelope routing law (subset tag, fault codes, whether the routed
  document still equals its input) is now stated ONCE in `expected_routing` and read by both roles;
  the subject is checked against it instead of merely reporting its own measurements.
* `mutate-semio-image` — its `no-mutation` kind borrows `move-frame`'s leaf directory, so
  `after_uri("no-mutation")` points at `move-frame`'s after-file. The subject now mirrors the
  oracle's special case and expects the BEFORE snapshot, which is what identity means.

### 2.3 The `protocol::Mutation` compile blocker

Six adapters wrote `use protocol::Mutation;` inside the `sut`-gated subject module (`brep:77`,
`graph:121`, `mesh:165`, `object:143`, `table:97`, `text:103`). `protocol` is the plugin's PRIVATE
`extern crate semio_framework_os_kernel as protocol;` (`📦️glue.rs`); the generated host links only
`semio-repo-test-host` and the plugin, so that import cannot resolve and the `--features sut` build
would fail on all six. `mutate-semio-kit`'s own doc comment had already diagnosed this exactly.

Fixed the way `✳️kit` did — thin permanent wrappers in each subset's OWN production code whose
signatures name only reachable types:

| subset | added to `🧬️mutations/🦀️component.rs` | added to `📸️snapshot/🦀️component.rs` |
|---|---|---|
| ✳️brep | `inverse_semio_brep_mutation`, `decode_semio_brep_mutation_json` | `encode/decode_semio_brep_snapshot_json` |
| ✳️graph | `inverse_semio_graph_mutation`, `decode_semio_graph_mutation_json` | `encode/decode_semio_graph_snapshot_json`, `parse/print_semio_graph_dsl`, `encode/decode_semio_graph_pack` |
| ✳️mesh | `inverse_semio_mesh_mutation`, `decode_semio_mesh_mutation_json` | `encode/decode_semio_mesh_snapshot_json` |
| ✳️object | `inverse_semio_object_mutation`, `decode_semio_object_mutation_json` | `encode/decode_semio_object_snapshot_json`, `parse/print_semio_object_dsl`, `encode/decode_semio_object_pack` |
| ✳️table | `inverse_semio_table_mutation`, `decode_semio_table_mutation_json` | `encode/decode_semio_table_snapshot_json` |
| ✳️text | `inverse_semio_text_mutation`, `decode_semio_text_mutation_json` | `encode/decode_semio_text_snapshot_json`, `parse/print_semio_text_dsl`, `encode/decode_semio_text_pack` |
| ✳️drawing | `inverse_semio_drawing_mutation`, `decode_semio_drawing_mutation_json` | `encode/decode_semio_drawing_snapshot_json`, `parse/print_semio_drawing_dsl`, `encode/decode_semio_drawing_pack` |
| ✳️kit | — | `parse/print_semio_kit_dsl`, `encode/decode_semio_kit_pack` |

### 2.4 `mutate-semio-object`: 12 placeholder `Err` handlers removed

`create-brep`/`delete-brep`/`create-mesh`/`delete-mesh`/`create-properties`/`delete-properties` —
6 kinds × mutate+inverse = 12 registrations — returned a self-documenting `Err` because a `create-*`
payload carries a `store::os_io::ArtifactRef` and a `delete-*` before-snapshot carries a populated
`store::ArtifactChild<S>`, neither nameable outside the plugin. The 3 working kinds ran against a
hand-built `identity_snapshot()`, not the committed fixture.

Constructing those values by hand was genuinely impossible; DESERIALIZING them never needed a
nameable path. With `decode_semio_object_snapshot_json`/`decode_semio_object_mutation_json` the
adapter now decodes all 9 committed fixtures, **all 9 kinds are real**, and the transform kinds run
against the committed before-snapshot instead of a synthetic one. This was the only case in the
fleet with per-kind placeholder handlers; there are now none.

### 2.5 Hand-transcribed Rust-literal fixtures replaced

`mutate-semio-graph` (11 kinds), `mutate-semio-mesh` (17), `mutate-semio-object` (9) and
`mutate-semio-text` (7) kept a second, hand-transcribed copy of every fixture as a Rust literal
inside the subject module — 44 fixtures free to drift silently from the JSON they claimed to mirror.
All four now decode the same committed bytes the oracle reads.

### 2.6 `identity-round-trip` added where a real artifact exists

`text`, `graph`, `mesh`, `object`, `kit`, `drawing` had no round-trip scenario. Each now decodes its
real committed artifact through BOTH committed encodings (`🗣️example.dsl.semio` and
`🎒️example.pack.semio`, separate files written by separate codecs) and asserts they agree, that
print→reparse is lossless, and that encode→decode is lossless. Byte-identical re-emission is the
EXPECTED result here — the committed text is our own codec's output, not a foreign writer's — so the
wave's "output must not equal input" tripwire deliberately does not apply and the text/binary
cross-check carries that evidence instead; this is stated in each feature description rather than
contrived around.

`brep` gets none: it has no committed example artifact, and inventing one would be fabricating
evidence.

### 2.7 Feature descriptions

Every one of the 19 now states (a) what genuinely distinguishes THIS subset, (b) why the committed
fixtures were chosen against that — an index that exists, an insert AHEAD of an existing member, a
delete that must leave a sibling slot alone — and (c) plainly that the runner executes no oracle
role, so the assertion is in the handler. The misleading trailing sentence "The `ordered-json-v1`
profile compares the two structurally" was removed everywhere it appeared on a no-oracle case,
including from `cad`/`document`/`flow`, because that profile never receives two sides for these
cases.

---

## 3. Verification — the real output

### Contract, whole owner — exit 1, and **zero semio breaches**

```
$ bun ./📜️script.ts contract --owner 🗄️stdio
8 high-priority breach(es) across 1 rule(s):
      8  testing/contract

  testing/contract  …/📷️png/🧪️tests/mutate-png-1-2/component.feature  Step at line 30 is outside a Background or Scenario
  testing/contract  …/📷️png/🧪️tests/mutate-png-1-2/component.feature  Step at line 35 is outside a Background or Scenario
  testing/contract  …/📐️step/🧪️tests/mutate-step-ap214-cc1  Test case has no implementation adapter
  … cc2 … cc3 … cc4 … cc5 … cc6
```

All 8 belong to `📷️png` and `📐️step`, both being edited by concurrent sessions during this run (the
png pair appeared, disappeared and reappeared while I worked). Per-case, every semio case is clean:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-text      # exit 0
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-drawing   # exit 0
0 high-priority breach(es) across 0 rule(s):
```

### Oracle phase, all 19 — exit 0, all not-exercised

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-<x>   # exit 0, ×19
[test] not-exercised …/🧿️semio/🧪️tests/mutate-semio-<x> (recorded no-oracle decision semio-<x>-mutation-semantics — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
```

Identical for animation, any, audio, brep, cad, document, drawing, flow, graph, image, kit, mesh,
model, object, presentation, table, text, value, video. **This is structural, not a regression** —
see §1.

### Subject phase — still impossible, and not because of this work

```
$ bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case mutate-semio-text   # exit 1
error: could not compile `semio-framework-ui-contract` (lib) due to 5 previous errors
[test] …/mutate-semio-text: no result stream at …/📤️results.jsonl

$ cargo check -p semio-s-plugin-stdio --no-default-features --lib             # exit 101
error: could not compile `semio-framework-job` (lib) due to 6 previous errors
error: could not compile `semio-framework-ui-contract` (lib) due to 5 previous errors
```

**Two** concurrent framework refactors block it, not one:
* `semio-framework-job` — the `ManuallyDrop<Option<RetainedJobPayload>>` / `JobPayloadPageSource`
  migration w10 recorded, now 6 errors at `🦀️component.rs:490,524,534,594,692,706`.
* `semio-framework-ui-contract` — 5 errors (`UiFixedBytes: Eq` unsatisfied, `SurfaceDoc` has no
  `clone`, two borrow conflicts in `🦀️limits.rs`/`🦀️document.rs`, a closure-arity mismatch in
  `🦀️builder.rs`). This one is NOT in w10's status line.

`semio-s-plugin-stdio` is never reached, so **no Rust written or changed in this wave could be
type-checked** — not the 24 production bridge functions, not any subject handler. What WAS verified:
every touched `.rs` file parses (`rustfmt --check` over all 19 adapters and all 19 subsets'
snapshot+mutations modules reports no parse error), every `include_str!` path resolves on disk, and
every field name used against a snapshot type was read out of that type's declaration.

---

## 4. What is still not true

1. **`parity=0/0` for all 19, and it will stay that way.** These cases have no oracle by design;
   their only possible parity evidence is a SECOND independent implementation, and the only other
   language surface (`🟦️component.ts` per subset) is types, not behaviour. Writing a TypeScript
   implementation by translating the Rust would be comparing our output against our own output —
   the exact failure the platform exists to prevent — so it was not done.
2. **`executed=0` for all 19.** The ~600 scenarios these cases now carry still have zero executed
   evidence until the subject phase compiles. What changed is that when it does, a green will mean
   something: previously a scenario passed iff no message was raised.
3. **No production test was added for the 24 new bridge functions.** They are exercised end-to-end by
   the adapters, and the crate cannot compile, so a unit test could not be run either way. The one
   test added (`kinds_match_the_enum_and_the_catalog` for `✳️drawing`) is written with a plain
   `#[test]` rather than the `#[semio_framework_async_macros::async_test]` its file's siblings use.
4. **`✳️brep` has no committed example artifact**, so it is the one semio subset with no
   `identity-round-trip` scenario.
