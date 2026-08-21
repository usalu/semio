# 🧪️ Handcrafted mutation fixtures — norm `📘️en1991` / `📘️en1999` / `📘️en1994`

80 cases, one per mutation leaf (32 + 26 + 22). Every case is `applied`; every case carries the
mandatory `🔺️diff/🔣️component.json`. No rejected case exists in this slice — see *Why no rejected
cases* below.

## Lint

`cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree`

```
🧬️ 115 artifact mutation trees · 1558 mutations · 806 covered · 752 uncovered
```

None of the three trees appears in the `--by-tree` uncovered list any more, and none appears in the
CLI's (40-row-truncated, repo-wide) error list.

Because that error list truncates, the lint's own `lintArtifact`/`lintCase`/`declaredMutations`
functions were re-extracted verbatim from `📜️script.ts` into
`scratchpad/scoped-lint.ts` and run scoped to just these three trees, printing **all** findings:

```
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations  32/32 covered  errors=0  warns=256
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations  22/22 covered  errors=0  warns=176
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations  26/26 covered  errors=0  warns=208
TOTAL scoped errors=0 warns=640
```

640 = 80 cases × 8 derived encodings (`.op/.spr/.patch/.patch.spr` + `.dsl/.pack` per snapshot side),
all pending `fixtures generate` (contract D1/D12) — expected and correct, never hand-forged.

## Structural verification (no `cargo`, nothing claimed to pass)

| Check | Result |
| --- | --- |
| `#[path]` targets resolve | 80/80 |
| `include_str!` targets exist | 400/400 |
| `rustfmt --edition 2021 --emit stdout` parses | 83/83 (80 test files + 3 mutations-root files) |
| snapshot JSON keyset == Rust snapshot fields (serde camelCase), in declaration order | 80/80 |
| diff JSON keyset == Rust `<Ty>Diff` fields (serde camelCase), in declaration order | 80/80 |
| before→after differs in exactly one field | 80/80 |
| diff sets exactly that one field, all others `null` | 80/80 |
| mutation JSON shape == the enum's own serde tagging + the payload struct's own `rename_all` | 80/80 |
| every float rendered with an explicit `.`/exponent (so `Value` compares as `f64`) | 80/80 |

## Wiring

`📦️glue.rs` was **not** touched. Each artifact's mutations-root `🦀️component.rs` gained one
`//#region 🧪️FixtureTests` block at the end:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "<leaf-dir>/🧪️tests/<case>/🦀️component.rs"]
    mod tests_<leaf>_<case>;
    // …
}
```

A top-level `#[path]` in a non-`mod.rs` file is relative to that file's own directory, so `"."` is
the `🧬️mutations/` directory. Same idiom as the pre-existing, load-bearing
`🧊️procedural3d/🧬️mutations/🦀️component.rs` (`#[path = "."] pub mod create_widget { … }`).

## Base documents

One realistic base document per artifact (the puzzle5d "Fixture Base" pattern); each case's
`⬅️before` is that document, `➡️after` is it with exactly one field moved. Every base value differs
from `Default::default()` so nothing leans on the defaults. Values are dyadic / plain-decimal
(`0.5`, `0.625`, `6.25`, `360.0`, `2000000.0`) so serde renders them without exponents.

- `📘️en1991` — 240 m² Category B floor, DE annex, 2 notional lanes / 24 m bridge, HC2 crane, 12 m silo.
- `📘️en1999` — AW-6082-T6 member, N_Ed 120 kN, 5 mm fillet weld, 2.5 mm sheet, r/t = 600/5 shell.
- `📘️en1994` — composite beam, M_Ed 240 kNm, 19 × 100 mm studs, C30 deck, R60 trapezoidal.

## Per-test assertions (seven, all worded for that mutation)

1. `applies_to_committed_after` — names the target field, its literal new value, **and** a rotating
   neighbouring field that must survive untouched.
2. `inverse_restores_before` — names the pre-edit value the inverse must re-state.
3. `committed_json_is_canonical` — both snapshots plus the mutation.
4. `declared_outcome_holds` — asserts `outcome.messages().is_empty()`, i.e. the edit trips neither
   the leaf's `mutation.no-op` guard nor its `mutation.invariant` finite-number guard.
5. `produces_committed_diff` — asserts the diff's own typed field equals `Some(<value>)`, asserts the
   neighbour field `is_none()`, then compares the whole serialization to `🔺️diff/🔣️component.json`.
6. `committed_diff_is_canonical` — decodes to `<Ty>Diff`, re-encodes to a fixed point.
7. `committed_diff_applies_to_after` — the committed diff alone carries `before` to `after`.

Norm exposes no `apply_*_mutation` / `inverse_*_mutation` free functions (puzzle5d does), so the
tests go through the artifact's own trait entry points — `protocol::Mutation::diff(…).diff()` then
`protocol::MutationDiff::apply(…)` — exactly as the mutations-root's existing `mod tests` does.
Written de-async: no `.await` anywhere.

## Findings worth carrying forward

1. **The three artifacts do not share a mutation wire shape.** `En1999Mutation` alone carries
   `#[serde(tag = "mutation", rename_all = "camelCase")]`, and its 26 payload structs each carry
   `#[serde(rename_all = "camelCase")]` — so en1999 encodes as
   `{"mutation":"changeITMm4","newITMm4":10240.0}`. `En1991Mutation` and `En1994Mutation` carry no
   serde attribute at all and their payload structs carry none either, so they encode externally
   tagged with snake_case fields: `{"ChangeAreaM2":{"new_area_m2":360.0}}`. Derived from the serde
   attributes, not from names.
2. **`🧬️mutations/🔣️component.json` is stale in all three trees.** It is titled `En1991Mutation`
   (resp. 1994/1999) but its `required`/`properties` are the *snapshot's* 32/22/26 camelCase scalars —
   it describes no mutation at all. Ignored; the Rust serde attributes were used as the oracle.
   Same for `🦠️mutation/🟦️component.ts`, whose en1991 mirror declares `newAreaM2` where Rust emits
   `new_area_m2`.
3. **`Option<Option<u32>> selected_check_index` needed no limitation pin.** All three `<Ty>Diff`
   structs carry it (`#[state(presence)]`), but no mutation writes it, the snapshots have no such
   field, and `<Ty>Diff::apply` ignores it. It serializes as `null` and decodes back to `None`, so
   the canonical-JSON and produced-diff assertions hold unmodified. Had a mutation written it,
   `None` and `Some(None)` would both encode as `null` and the round trip would be lossy.
4. **Every leaf in all three trees has exactly two guards** — `!new.is_finite()` →
   `mutation.invariant` fatal (only on `f64` leaves: 22/24/18 of them) and `base == new` →
   `mutation.no-op` warn — and otherwise emits a single-field `<Ty>Diff`. Verified by normalising and
   counting every `return protocol::MutationOutcome::…` and every `if` in all 80 diff builders; there
   are no cascades, no target lookups, and therefore no `mutation.target-missing` path to reject on.
5. **Why no rejected cases.** A rejection here would need a non-finite payload
   (`NaN`/`Infinity`), which JSON cannot represent, so a `🦠️mutation/🔣️component.json` for it cannot
   be authored at all. The contract asks for one case per leaf; each leaf gets one applied case.

## Files

- 80 × `<leaf>/🧪️tests/<case>/` each containing `📸️snapshot/⬅️before/🔣️component.json`,
  `📸️snapshot/➡️after/🔣️component.json`, `🦠️mutation/🔣️component.json`,
  `🔺️diff/🔣️component.json`, `🎯️outcome/🔣️component.json`, `🦀️component.rs`.
- 3 × `🧬️mutations/🦀️component.rs` (appended `//#region 🧪️FixtureTests` wiring block only).
