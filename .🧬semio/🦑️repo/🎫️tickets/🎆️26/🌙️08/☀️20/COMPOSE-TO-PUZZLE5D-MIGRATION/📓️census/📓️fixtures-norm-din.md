# 🌡️ Handcrafted mutation fixtures — `📗️din16798` (62) + `📕️din4108` (22)

Ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`, contract D1/D6. 84 cases, one per mutation leaf.
Slice owned exclusively by this lane; `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` was **not**
touched.

## 📁️ What landed

```
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
  <leaf>/🧪️tests/<case>/📸️snapshot/⬅️before/🔣️component.json
                        /📸️snapshot/➡️after/🔣️component.json
                        /🦠️mutation/🔣️component.json
                        /🔺️diff/🔣️component.json
                        /🎯️outcome/🔣️component.json
                        /🦀️component.rs                     ← 7 assertions, worded for that leaf
  🦀️component.rs                                             ← + self-wired `🧪️FixtureTests` region
```
…and the same shape under `📗️din16798`. 84 case directories × 6 files = 504 committed files, plus
the two mutation-root `🦀️component.rs` edits (purely additive).

## 🪡️ Wiring — self-wired, glue untouched

Each artifact's own mutations-root `🦀️component.rs` gained one appended region:

```rust
//#region 🧪️FixtureTests
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🪜change-category/🧪️tests/retypes-the-assembly-as-office/🦀️component.rs"]
    mod tests_change_category_retypes_the_assembly_as_office;
    // …one line per case
}
//#endregion 🧪️FixtureTests
```

`📦️glue.rs` is shared with the agents migrating the other thirteen norm artifacts, so it carries the
production `diff`/`inverse`/`mutation` mounts only. The stale docstring in each mutations root that
claimed glue owns *all* wiring was amended in place to say "production mounts only".

The `#[path = "."]` re-basing rule was **verified against rustc**, not assumed: a scratch crate that
mirrors norm's exact nesting (glue → four nested `#[path = "."]` inline mods → an emoji-named
`🦀️component.rs` loaded by `#[path]` → `#[cfg(test)] #[path = "."] mod fixture_tests` → an emoji leaf
dir → an emoji case dir → `include_str!`) compiles and runs its test:

```
test artifacts::din4108::schema::mutations::component::fixture_tests::tests_change_category_retypes_it::reads_its_own_fixture ... ok
```

Children of a `#[path]`-loaded non-mod-rs file resolve against **that file's own directory**, so the
leaf-relative paths above are correct and no `../` escape is needed.

## 🎞️ Wire shapes — the two artifacts genuinely differ

Read off each `🧬️mutations/🦀️component.rs`, never assumed:

| | `📕️din4108` | `📗️din16798` |
|---|---|---|
| enum attrs | *none* → **externally tagged** | `#[serde(tag = "mutation", rename_all = "camelCase")]` → **internally tagged** |
| payload struct attrs | *none* → snake_case fields | `rename_all = "camelCase"` on every payload |
| example | `{"ChangeCategory":{"new_category":"office"}}` | `{"mutation":"changeTOpC","newTOpC":24.5}` |

Snapshots and diffs are `rename_all = "camelCase"` in both. Nested `LayerDocument` (din4108) is
camelCase too: `{"thicknessM":0.24,"lambdaWMk":0.81}`.

## 🔺️ The diff file

Both `<Artifact>Diff` structs are `#[serde(rename_all = "camelCase", default)]` with **no**
`skip_serializing_if`, so serde emits **every** field — `null` for the untouched ones. The committed
`🔺️diff/🔣️component.json` therefore carries all 20 keys (din4108) / 66 keys (din16798), exactly one
of them non-null, and `artifact` + `selectedCheckIndex` always `null`. Diff-apply for these artifacts
never fails (`Ok(...)` unconditionally), so a rejection is expressed only through
`MutationOutcome`'s messages — which is what assertion 4 checks.

### `Option<Option<T>>` limitation, pinned rather than papered over

`selected_check_index: Option<Option<u32>>` (presence lane) cannot distinguish `None` from
`Some(None)` across a JSON round trip — both encode as `null`. None of the 84 mutations writes it, so
the committed `null` is unambiguously `None` and the canonical fixed point holds. Every test asserts
this explicitly rather than relying on it silently:

```rust
assert!(decoded.selected_check_index.is_none(), "…: change-annex is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
```

## 🧪️ The seven assertions

Same seven as puzzle5d's `📍move-part2d/🧪️tests/moves-part-a`, each **worded for its own leaf** and
each carrying extra assertions that name that leaf's own field, its own value and a witness field the
leaf's diff builder provably never writes:

1. `applies_to_committed_after` — `+ assert_eq!(snapshot.duct_class, "D", …)` and
   `+ assert_eq!(snapshot.duct_leakage_m3_s_m2, before().duct_leakage_m3_s_m2, …)`
2. `inverse_restores_before` — `+ assert_eq!(inverse.len(), 1, …)` and the base value restored
3. `committed_json_is_canonical` — both snapshots **and** the mutation payload
4. `declared_outcome_holds` — status **and** the full `(level, code)` message list
5. `produces_committed_diff` — `+ assert_eq!(raised.diff().duct_class.as_deref(), Some("D"), …)` and
   `+ assert!(raised.diff().duct_leakage_m3_s_m2.is_none(), …)`
6. `committed_diff_is_canonical` — `+` the `selectedCheckIndex` pin above
7. `committed_diff_applies_to_after`

Entry points: the artifacts expose no `apply_*`/`inverse_*` wrappers, so the tests drive the kernel
directly — `protocol::apply_mutation(&before(), &mutation())` and
`<Mutation as protocol::Mutation<Snapshot>>::{diff, inverse}` — in the de-async style (no `.await`),
matching `📚️examples/🎬️demo/🧪️tests/🦀️test.rs`.

## 📸️ Base snapshots

One handcrafted base per artifact (puzzle5d's "Fixture Base" pattern), so a reader can diff any two
cases and see only the field that leaf owns.

* **din4108** — a two-layer exterior masonry wall (0.24 m @ λ 0.81, 0.14 m @ λ 0.04), climate zone 2,
  n50 2.5 h⁻¹, 100 m² envelope, Beiblatt 2 details conforming.
* **din16798** — a 90 m² mechanically ventilated dwelling: 22 °C operative, 50 % RH, 800 ppm CO₂,
  IDA 2, SFP 1500 W/(m³/s), 75 % heat recovery, duct class C.

All values are dyadic or short decimals in the range serde renders in plain decimal, so
shortest-round-trip float printing and the committed text agree byte for byte. Every `f64` is written
with a decimal point and every `u8`/`u32` without, so `serde_json::Value` number equality holds (a
`1005` would not equal a `1005.0`).

## 🔍️ Oracle transcription notes

Every `after` and every diff was derived from that leaf's own `🔺️diff/🦀️component.rs`, and each test's
docstring records the guards that were read there:

* **din16798** — all 62 are `change-<scalar>`. `change-rh-percent` and `change-df-percent` are the
  **only two** with a `0..100` range guard; every other `f64` leaf has `is_finite` only; the
  `String`/`u8`/`u32` leaves have only the `==` no-op guard. Notably
  `change-cooling-utilization-factor` and `change-heat-recovery-eta` are 0..1 ratios in practice but
  carry **no** range guard — the fixture headline says so rather than implying one.
* **din4108** — 17 `change-<scalar>` plus five index-addressed `layers` leaves. `insert-layer`
  **clamps** an out-of-range index and warns `mutation.clamped`; `remove-layer`,
  `change-layer-thickness` and `change-layer-lambda` **reject** with `mutation.target-missing`;
  `reorder-layers` rejects a bad `from`, clamps `to` against the *shortened* list, and warns
  `mutation.no-op` when the landing index equals `from`. All five rebuild and write the **whole**
  `Din4108LayerList` — the diff is the rebuilt list, never a per-element patch, which the layer
  cases' extra assertions state outright.

All 84 committed cases take an accepting, non-no-op path (`{"status":"applied"}`, no messages), so no
`🔺️diff/🚫️component.absent` file was needed. The rejection and clamp branches are documented in each
leaf's fixture headline and are the natural second case per leaf when the fixture set is widened.

## ✅️ Verification

```
$ cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree
🧬️ 115 artifact mutation trees · 1558 mutations · 1230 covered · 328 uncovered
   (neither 📕️din4108 nor 📗️din16798 appears in the ❌️ uncovered list or in any error row)
❌️ 350 error(s)      ← all from other lanes' trees, still in flight
```

The CLI truncates its error list at 40 rows repo-wide, so the lint's own rules
(`declaredMutations`/`lintArtifact`/`lintCase`/`lintReference`, transcribed verbatim) were re-run
scoped to just these two trees via `🛠️lint-scope-norm-din.ts`:

```
🧬️ …/📕️din4108/…/🧬️mutations
   22 enum variants · 22 leaves · 22 covered · 0 uncovered · 22 cases · 0 error(s) · 176 derived-encoding warning(s)
🧬️ …/📗️din16798/…/🧬️mutations
   62 enum variants · 62 leaves · 62 covered · 0 uncovered · 62 cases · 0 error(s) · 496 derived-encoding warning(s)
✅️ scoped: 0 error(s), 672 expected derived-encoding warning(s)
```

The 672 warnings are the expected contract-D1 derived encodings (`.op.semio`, `.spr.semio`,
`.dsl.semio`, `.pack.semio`, `.patch.semio`) that `fixtures generate` will produce once the workspace
compiles — hand-forging them would fake the codec test (contract D12).

Structural checks (`cargo` is not usable; no test is claimed to pass):

| check | result |
|---|---|
| `rustfmt --edition 2021 --emit stdout` on all 84 test files | 84 parsed, 0 failures |
| `rustfmt` on both mutation-root `🦀️component.rs` after the wiring + docstring edits | both parse |
| `include_str!` targets exist | 420/420 |
| `#[path]` mounts resolve to real files | 22/22 + 62/62 |
| `#[path = "."]` + `include_str!` nesting proven under rustc | scratch crate compiles and its test passes |
| before ≠ after (no accidental `mutation.no-op`) | 84/84 |
| exactly one snapshot field differs between before and after | 84/84 |
| exactly one diff field non-null, and it is the same field | 84/84 |
| diff value == after value; mutation payload value == diff value | 84/84 |
| `artifact` and `selectedCheckIndex` null in every diff | 84/84 |
| `f64` written with a decimal point, `u8`/`u32` without, no exponents | all |
| every leaf's `↩️inverse` returns exactly one step for the committed payload | 84/84 |
| `📦️glue.rs` unmodified | `git status --porcelain` empty for that path |

## 🛠️ Authoring aids (ticket-local, kept)

* `🛠️fixtures-norm-din.ts` — writer: canonical-JSON encoder with forced `f64` decimal points,
  serde's own `camelCase` rule, the test-source template, and the self-wiring block writer. Carries
  no mutation knowledge.
* `🛠️fixtures-din4108.ts` — the 22 handcrafted rows (base document, per-leaf new value, witness
  field, guard note, headline), plus five bespoke layer cases.
* `🛠️fixtures-din16798.ts` — the 62 handcrafted rows.
* `🛠️lint-scope-norm-din.ts` — the scoped lint re-run above.
* `🛠️norm-din-extract.ts` — the extraction pass that read each leaf's payload struct, diff field and
  guards out of its own Rust source.
