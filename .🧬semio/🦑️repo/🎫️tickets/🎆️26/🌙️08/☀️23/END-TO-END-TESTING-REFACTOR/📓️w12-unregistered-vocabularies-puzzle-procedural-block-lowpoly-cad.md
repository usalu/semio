# Wave 12 — the eleven unregistered vocabularies of 🧩️puzzle, 🌀️procedural, 🧱️block, 💠️lowpoly, 📐️cad

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Scope: the eleven
`unregistered-mutation-vocabulary` breaches owned by those five plugins (267 mutation kinds).

## What landed, per subset

| Subset | Kinds | Catalog | Case |
|---|---|---|---|
| `💠️lowpoly/💠️lowpoly` | 17 | `lowpoly-1-any` | `mutate-lowpoly-1` |
| `🧩️puzzle/◻2d` | 26 | `puzzle-2d-1-any` | `mutate-puzzle-2d-1` |
| `🧩️puzzle/🖐️5d` | 28 | `puzzle-5d-1-any` | `mutate-puzzle-5d-1` |
| `🧩️puzzle/🧊️3d` | 35 | `puzzle-3d-1-any` | `mutate-puzzle-3d-1` |
| `🌀️procedural/🧊️procedural3d` | 14 | `procedural-3d-1-any` | `mutate-procedural-3d-1` |
| `🌀️procedural/🌀️procedural2d` | 14 | `procedural-2d-1-any` | `mutate-procedural-2d-1` |
| `🌀️procedural/🧩️assembly` | 9 | `assembly-1-any` | `mutate-assembly-1` |
| `🧱️block/◻2d` | 26 | `block-2d-1-any` | `mutate-block-2d-1` |
| `🧱️block/🖐️5d` | 41 | `block-5d-1-any` | `mutate-block-5d-1` |
| `🧱️block/🧊️3d` | 37 | `block-3d-1-any` | `mutate-block-3d-1` |
| `📐️cad/📐️cad` | 20 | `cad-1-any` | `mutate-cad-1` |

Each subset got `pub const KINDS` + `kinds_match_the_enum_and_the_catalog` (plain `#[test]`) beside
its enum, a `🧪️oracle/🔣️.json` recording a `noOracleDecision`, and a case whose feature
names the committed specification vector for EVERY kind by full path.

## Why no oracle, in every one of the eleven

All eleven are semio-NATIVE documents (`.dsl.semio`/`.pack.semio`, some also `.op.semio`/`.spr.semio`).
No third party reads that envelope and none is authoritative over the vocabularies, which ARE the
subsets' own specifications. Substitutes recorded: `specification-vectors` (the committed per-kind
`(before, mutation, diff, outcome, after)` quintets) and `metamorphic-laws`.

## The laws asserted in role

- `mutate-<kind>` (`@mode-conformance`): the committed payload's discriminant really is this kind
  (internal `"mutation"` camelCase for puzzle/block/cad, external PascalCase variant key for lowpoly
  and the three procedural artifacts), the outcome status is `applied`, and `law::mutation_is_observable`.
- `inverse-<kind>` (`@mode-property`): FOOTPRINT COMPLETENESS — `before` and `after` differ on exactly
  the fields the committed diff declares, resolved through each subset's own alias table.
- `identity-round-trip` (`@mode-round-trip`): `law::round_trip_preserves` + `law::reparsed_not_copied`
  over the richest committed snapshot, plus a per-subset structural guard.

The full inverse law `apply(inverse(m), apply(m, base)) == base` is NOT claimed by these cases: the
committed diffs record removals as bare ids, so a removed record cannot be reconstructed by any
reader that does not link the subset's own codec. It stays with the production `inverse()` and the
per-leaf fixture tests. This is stated in every feature and every adapter docstring.

## Real findings surfaced by the laws

1. **`CadDiff`'s four fixed child slots lose the vacate intent on the wire.** `Option<Option<T>>`
   renders `Some(None)` as `null`, indistinguishable from untouched, so the committed diff for
   `delete-shape-model` is entirely empty. `VACATE_COLLAPSES` names those four fields and still
   demands the vacated value BE `null`.
2. **Three kinds ship only a no-op vector**: `🧩️puzzle/◻2d replace-node-handle`
   (`rekind-handle-1-is-noop`), `🧩️puzzle/🖐️5d replace-kind-catalogs` (`null-catalogs-is-noop`),
   `🧩️puzzle/🧊️3d replace-object-vortex` (`rekind-vortex-1-is-noop`). Their `🎯️outcome` records a
   `mutation.no-op` warning, so the case asserts the OPPOSITE of observability rather than passing
   silently. That those kinds have no vector which actually replaces anything is a real gap in the
   production fixtures.
3. **Three diffs do not mirror their snapshots**: `🧩️assembly` splits every collection into
   `<name>Removed`/`<name>Upserted`; `🧱️block/🖐️5d` renames `2d`/`3d` to `part2d`/`part3d`;
   `🧱️block/🧊️3d` FOLDS `catalog` + `vortexKindExtra` into one `vortexKinds` field.

## Verification actually run

- `bun ./📜️script.ts contract` → `EXIT:0`, `0 high-priority breach(es) across 0 rule(s)`. Zero
  breaches scoped to any of the eleven subsets or cases.
- `bun ./📜️script.ts oracle exhaustive --owner <plugin>` → every case reports
  `not-exercised (recorded no-oracle decision … — its evidence is discharged by the subject phase)`.
  That is the framework's designed behaviour for a `@no-oracle-` case, not a defect in these cases.
- `bun ./📜️script.ts subject exhaustive --owner <plugin>` → cannot complete. The generated subject
  host additionally links the owner's plugin crate, and none of the five compiles at HEAD
  (`dsl::Fault` does not implement `Display` and non-`Send` futures in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/../../🧵️shard/`; `semio-framework-ui`
  fails for 🧩️puzzle and 🌀️procedural; `semio-s-plugin-lowpoly` reports 706 errors of its own).
  All framework/peer-side, none in this wave's files.
- The adapters themselves were compiled and executed against the platform's OWN generated plans, by
  building each generated host without the `sut` feature (the exact build the oracle role would use):

```
res-mutate-assembly-1.jsonl      19 {'passed': 19}
res-mutate-block-2d-1.jsonl      53 {'passed': 53}
res-mutate-block-3d-1.jsonl      75 {'passed': 75}
res-mutate-block-5d-1.jsonl      83 {'passed': 83}
res-mutate-cad-1.jsonl           41 {'passed': 41}
res-mutate-lowpoly-1.jsonl       35 {'passed': 35}
res-mutate-procedural-2d-1.jsonl 29 {'passed': 29}
res-mutate-procedural-3d-1.jsonl 29 {'passed': 29}
res-mutate-puzzle-2d-1.jsonl     53 {'passed': 53}
res-mutate-puzzle-3d-1.jsonl     71 {'passed': 71}
res-mutate-puzzle-5d-1.jsonl     57 {'passed': 57}
TOTAL scenarios 545
```

- `rustfmt --edition 2021 --emit stdout` parses all eleven patched production files. The
  `kinds_match_the_enum_and_the_catalog` tests could NOT be RUN — `cargo test -p semio-s-plugin-*`
  fails for the reasons above.

## Honest gaps

- No production code is exercised by these cases. They measure the committed specification vectors
  and the laws over them; the vectors' agreement with the implementation is asserted by the in-crate
  per-leaf fixture tests, which are equally unrunnable while the plugin crates are broken.
- The eleven `kinds_match_the_enum_and_the_catalog` tests are written and parse but have never run.
- Three kinds are covered only by a declared no-op vector (finding 2 above).
