# Closing the unregistered-vocabulary hole for 📋️forms, 🗒️note, ✒️writer, 📖️playbook, 🏛️architect

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Scope: the 8 `unregistered-mutation-vocabulary`
breaches belonging to those five plugins. Every command quoted below was actually run; exit codes were
read from the tool's own status, never through a pipe.

## What the 8 breaches actually were

The gate walks every `🧬️mutations` directory and asks whether a manifest whose owner is
`dirname(dirname(rel))` declares a catalog. For these plugins the vocabulary directory appears TWICE
under one owner — `<subset>/🧬️schema/🧬️mutations` (the enum) and `<subset>/🚪️io/🧬️mutations` (that
enum's text/binary codecs) — so 📋️forms, 🗒️note and ✒️writer each raised two breaches for ONE
vocabulary. 📖️playbook and 🏛️architect keep their mutation codecs inside `🧬️schema/🧬️mutations/`
and raised one each. **8 breaches = 5 vocabularies**, and one manifest per subset closes both.

| subset | enum | kinds | committed vectors that MOVE the document |
|---|---|---:|---|
| `✒️writer 1/✳️any` | `WriterMutation` | 4 | 3 of 4 |
| `📋️forms 1/✳️any` | `FormMutation` | 10 | 1 of 10 |
| `🗒️note 1/✳️any` | `NoteMutation` | 33 | 33 of 33 |
| `📖️playbook 1/✳️any` | `PlaybookMutation` | 9 | 1 of 9 |
| `🏛️architect 🏛️program 1/✳️any` | `ProgramMutation` | 266 | 260 of 266 |

322 kinds, 649 expanded scenarios.

## The oracle question, answered plainly

All five are semio-NATIVE artifacts: they persist as `.dsl.semio` text and `.pack.semio` binary
through their own codecs, and no third party reads or writes either envelope. There is no credible
reference implementation to register, and registering a weak one would be worse than none. All five
therefore carry a recorded `noOracleDecision` with substitutes `["specification-vectors",
"metamorphic-laws"]` — the same shape the 19 `🧿️semio` Pattern-B subsets already use.

**Consequence, stated rather than hidden:** a recorded no-oracle case is never dispatched in the
oracle role (`runPhases` only executes it when `decision.implementation !== null`). All five cases
report `not-exercised` in `oracle exhaustive`, exactly like the 19 existing semio cases. Their laws
are asserted in the SUBJECT role, which cannot compile today (see Blockers).

## What was written

Per subset:

1. `pub const KINDS: &[&str]` beside the enum, in declaration order, with the kind spellings taken
   from each triad leaf's own `SemanticDescriptor::kind` (not from the variant name).
2. A plain `#[test] kinds_match_the_enum_and_the_catalog` asserting `KINDS` against
   `<Enum as protocol::SemanticMutation<Snapshot>>::kinds()` — length, order and spelling — and
   against the committed manifest's catalog via `include_str!`.
3. Sync external bridges beside the enum, each carrying the repo's `// 🚫️async: E1` exemption line:
   `apply_<x>_mutation_outcome` (returns the whole `MutationOutcome`, diagnostics included — the
   existing async wrappers answer `Result<_, _>` and drop the messages; 🏛️architect never had an
   apply helper at all), `inverse_<x>_mutation_steps`, `decode_<x>_mutation_json`,
   `decode_<x>_snapshot_json`, `encode_<x>_snapshot_json`, plus `parse_<x>_dsl`/`print_<x>_dsl` in
   the DSL facet (`store::TextError` is unnameable outside the crate).
4. `<subset>/🧪️oracle/🔣️component.json` — the `noOracleDecision` and the `mutationCatalog`.
5. `<artifact>/🧪️oracle/🔣️component.json` — ONLY `oracleHostPackages`, naming
   `semio-s-plugin-stdio-test-oracle` by path so the generated host can reach the shared
   `⚖️law` module. The `oracles` feature is deliberately not enabled, so no third-party crate enters
   a host. `bun ./📜️script.ts dependency` still reports `test-oracle=30`, unchanged.
6. `<artifact>/🧪️tests/mutate-<fmt>-1/component.feature` + `🦀️component.rs`.

### Why the cases register SUBJECT handlers only

A recorded no-oracle case runs no oracle role. `mutate-semio-mesh` and its 18 siblings register
oracle closures anyway that read the committed `after`/`before` literally; w11 §4 then counted those
20 handlers as asserting nothing. These five register no oracle handler at all and say so in the
adapter header, so nothing in a listing reads as coverage that cannot execute.

### The laws, and where each is asserted

Every handler goes through `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs`:

* `mutate-<kind>` — `law::divergence` against the committed `➡️after`, the raised `mutation.*` codes
  against the committed `🎯️outcome` (both outcome shapes are read: an applied outcome's `messages`
  array and a rejected outcome's single `code`), then `law::mutation_is_observable` with the
  subset's declared `GUARD_VECTORS` exemption list.
* `inverse-<kind>` — `law::inverse_restores`, zero tolerance, zero ignored keys, on the committed
  `⬅️before` projection. Unexempted in all five subsets, including for the guard vectors: a refused
  mutation's inverse is EMPTY, and applying nothing must still land on the before-snapshot.
* `identity-round-trip` — `law::round_trip_preserves` on the projection AND `law::carrier_is_exact`
  on the bytes. The exact-bytes law rather than the no-byte-pass-through tripwire because the
  committed `🗣️example.dsl.semio` is the subset's OWN printer's output; that is the documented
  byte-preserving-carrier case the wave brief allows, and it is the STRONGER claim of the two.

### The observability exemptions, all declared

| subset | exempt | reason, cited in code and in the feature |
|---|---:|---|
| 🗒️note | 0 of 33 | none needed — every vector moves the document |
| 🏛️architect | 6 of 266 | `knowledge`/`benchmarks` are the only two of 66 registers that are composed `s.stdio.semio.table` children; their rows live in a working scene a fresh process never populated, so only the `mutation.target-missing` branch is reachable from a committed snapshot |
| ✒️writer | 1 of 4 | `edit-text`'s vector pins that a save with no keystrokes must not re-mint the content-addressed handle; the handler asserts that handle stability explicitly |
| 📖️playbook | 8 of 9 | the step flow lives behind a content-addressed `s.stdio.semio.flow` child a successful mutation re-mints |
| 📋️forms | 9 of 10 | same, for the `structure`/`results` children — the leaf itself says hand-authoring an effect `➡️after` would mean forging a value out of `std`'s unspecified `DefaultHasher` |

## Findings worth their own work

1. **📋️forms and 📖️playbook have no effect vectors.** 9 of 10 and 8 of 9 committed vectors pin a
   rejection or a no-op. The cause is documented in the leaves and is structural, not laziness:
   a successful mutation re-mints a `DefaultHasher`-addressed child handle. Until the fixture
   generator can mint that handle, those two vocabularies cannot be shown to DO anything.
2. **The composed-child subsets' committed before-state is only half committed.** For 📋️forms and
   📖️playbook the working-scene rows behind the child handle live in each leaf's Rust test as a
   literal, so an external host cannot reconstruct them. The cases carry that half as a `scene`
   column in their own `Examples` tables, transcribed from the leaf's `before()` and cited there.
   That is the one input in these five cases that is not a digest-pinned committed file. **Fix:
   commit the scene beside the snapshot as a fixture of its own.**
3. **`⚖️law` should not live under 🗄️stdio.** Its own doc says it is dependency-free and
   format-neutral; it is the shape of the argument, not knowledge of a format. Five plugins now
   declare a cross-plugin `oracleHostPackage` purely to reach it. Mounting it in
   `semio-repo-test-host` would delete that coupling — a framework edit, out of scope here.
4. **The vocabulary rule double-counts.** `dirname(dirname(rel))` makes `🚪️io/🧬️mutations` and
   `🧬️schema/🧬️mutations` two breaches over one owner and one vocabulary. Harmless but inflates the
   "70 vocabularies" headline.

## Blockers — what could NOT be verified

* **`cargo test` cannot run for any of the five plugin crates.** A concurrent framework-wide de-async
  refactor leaves `semio-framework-plugin` broken:
  `error[E0728]: await is only allowed inside async functions and blocks` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:30943`, and
  `error: could not compile semio-framework-plugin (lib) due to 7 previous errors`. Every plugin
  crate depends on it. The five `kinds_match_the_enum_and_the_catalog` tests are therefore written
  and unrun. What WAS verified: all 15 edited/created Rust files parse (`rustfmt --edition 2021
  --emit stdout`, no diagnostics), and the crate the adapters newly link,
  `semio-s-plugin-stdio-test-oracle`, builds without the `oracles` feature (`cargo check --lib`,
  exit 0) and passes its `law::` unit tests (`12 passed; 0 failed`).
* **The subject phase cannot run**, for the same reason, so the 649 scenarios have no execution
  evidence yet. This is the wave-wide `parity=0/0` blocker, not a new one.
* **`carrier_is_exact` on the five `identity-round-trip` scenarios is the assertion most likely to go
  red first** when the subject phase is unblocked, because it demands the printer reproduce the
  committed example byte-for-byte. That is deliberate: a red there means the committed artifact has
  drifted from the codec that wrote it, which is a finding, not a reason to weaken the law.

## Commands actually run

```
bun ./📜️script.ts contract                                     → exit 1, 29 breaches repo-wide, 0 in this scope
bun ./📜️script.ts discover                                     → all five cases discovered, [rust]
bun ./📜️script.ts oracle exhaustive --owner <p> --case <c>     → exit 0, not-exercised=1 for each of the five
bun ./📜️script.ts dependency                                   → exit 0, ecosystems=4 entries=232 test-oracle=30
cargo check --lib (stdio test-oracle crate)                      → exit 0
cargo test --lib law:: (stdio test-oracle crate)                 → 12 passed; 0 failed
cargo check -p semio-s-plugin-note --lib                         → fails in semio-framework-plugin (pre-existing)
```

Scenario expansion, read out of the parser:

```
mutate-program-1  scenarios=533 errors=0 mutate=266 inverse=266
mutate-note-1     scenarios=67  errors=0 mutate=33  inverse=33
mutate-forms-1    scenarios=21  errors=0 mutate=10  inverse=10
mutate-playbook-1 scenarios=19  errors=0 mutate=9   inverse=9
mutate-writer-1   scenarios=9   errors=0 mutate=4   inverse=4
```
