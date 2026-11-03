# FIXTURE-AND-ORACLE (wave 2)
Python only, niced, no cargo/`repo run`, no git writes.

## 1. en1997 `verify_fixtures.py`: 2 failures -> 0 (222 scenarios, 209 inverted)
- Cause: tool bug, not fixture content. `schema-evolve/l_mutations.py` `clone_family` cloned crates, registries, protocol and fixtures of `insert-/remove-<collection>` but never the rows of `🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`. Without the `edit` metadata (`rule`, `path`, `selectors`, `index`, `item`) the oracle engine could not address `characteristicBases` ("addresses None", "collection this document does not carry").
- Tool fix: new `taxonomy_clone` (called from `clone_family`): one row per cloned kind, derived from the reference kind's row via the same rename pairs; idempotent, inserted after the artifact's last row. (Not run through `evolve.py`; syntax-checked only.)
- Data: the two missing rows `insert-characteristic-basis` / `remove-characteristic-basis` added to the taxonomy (what the tool now produces).
- Also fixed: copied labels in the two mutation schemas ("Layer water regime" -> "Characteristic basis", DE "Charakteristische Grundlage"). Open tool item: `clone_kind` does not rename the reference entity's label inside the schema JSON (only `native("..")` labels), so the next collection repeats it.

## 2. en1992 check-outcome oracle
- `column-C1` / `wall-SW1` sentinel 56250: data, not oracle. nK was positive = tension (member convention, Rust `n_comp = -n_ed`): 1.26 MN tension on the bars, outside the interaction domain, M_Rd = 0. Builder and fixtures now carry compression (nK -600/-300 kN); fail variant column mK 250 kNm (flexure 1.16, sigma_c 1.89) replaces N = 3 MN.
- `studs.slab-FS-corner` 1.255: real data defect (spacingRadial 0.24 m > 0.75 d = 0.191 m; Rust `S_R_MAX` agrees). Corner rows now 0.17 m (0.889).
- New in the oracle: `anchor_effects` / `anchor_checks` (EN 1990 6.10b N_Ed/V_Ed as the Rust combination, ties to last) on the mechanics fastening kernel's Python twin (loaded by path): `en1992-4.{steel,steel.sls,cone,pullout,splitting,pryout,edge,steel-shear,interaction}.<anchor>`. Self-check gained two hand checks of the cone. Splitting follows the Rust formula (psi = 0.5 + 0.5 min(c1, c_cr)/c_cr), unverified against the standard like the kernel constants.
- Anchor data: anc-1 interaction was 1.12 in the pass document (genuine) -> actions x0.7 (0.656); fail document scales anc-2 actions x4.
- Builder `🧰️scripts/showcase/en1992.py` updated (`--check` clean), 3 showcase fixtures patched, DSL regenerated. Packs not regenerated.
- Feature: all `rc-building-showcase` rows regenerated (133 rows; zero-utilisation rows dropped, ids renumbered). Dry run on `🚦️check-outcome-en1992-1`: 220 ok, 0 failed (was 194).

## Files
Tool `🧰️scripts/schema-evolve/l_mutations.py`; en1997 taxonomy json and `➕insert-/➖remove-characteristic-basis/🧬️schema/🔣️.json`; en1992 oracle `🔮️oracles/🚦️check-outcome/🐍️.py`, `🧪️tests/🚦️check-outcome-en1992-1/🥒️.feature`, fixtures `rc-building-showcase-{pass,fail}` and `🎪️showcase`, DSL asset, builder. Leftover: empty-able `🗑️generated/fixture-and-oracle/` (rm denied; delete by hand).
- ready for rebuild: fixture-and-oracle (build-en1992, build-en1997 latest.md)
