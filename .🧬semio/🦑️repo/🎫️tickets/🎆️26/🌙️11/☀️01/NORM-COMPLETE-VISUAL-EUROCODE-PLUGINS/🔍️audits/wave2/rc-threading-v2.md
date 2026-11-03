# RC threading v2 (wave 2): `reliabilityClass` in en1992, en1993, en1995, en1997, en1998, en1999, en1994

Status: all seven done end to end, nothing compiled or built (no cargo, no git write). `verify build-owners sync` passed (added=5, not from this work: no crate was added). Tools: `🧰️scripts/rc-threading/{rc_lib,rc_leaf,rc_run}.py` plus per artifact `core_enXXXX.py` (layers) and `thread_en1994.py`; `--apply` writes. Announcements and `- ready for rebuild: rc threading` are in `📌️important/requests.md` and `🗑️generated/build-enXXXX/latest.md`.

## Common to all seven
- `reliabilityClass` is the LAST field of snapshot, artifact and diff (Rust and JSON/ts/graphql/proto twins in the same order, checked with `schema-sync/check.py`: no reliability finding). Type `semio_s_plugin_norm_compliance::ReliabilityClass` (JSON enum `RC1|RC2|RC3`).
- SQLite: column + CHECK in `🗄️.sql`, codec rs and ts, native admission `Enumeration(<root field id>, [RC1,RC2,RC3])`, choices fixture, fixture value RC3, contract (`🎛️semantic.json`) replayed with python `sqlite3` (schemaBytes, document width, valueBytes +3 per case) and the ts test literals regenerated. In several artifacts the stored contract was already stale against the owner's newer DDL (en1993 49428 -> 49632, en1995 19674 -> 20704+, en1997 16496 -> 19285+ with 4 table widths, en1999 16061 -> 16552+, en1994 10840/10804 -> 10951+): the replay now matches the current DDL, the per-case valueBytes of fields the owners added after the contract was written remain whatever they were (+3 only).
- Kind `change-reliability-class` (`🎯` dir in the group crate of `change-annex`, tag = max+1), mutation union/KINDS/ts/json/proto/graphql/text codec/binary protocol tag/oracle manifest/feature rows/py harness/taxonomy row, edit rule `SetFieldRule { path: "reliabilityClass", .., selectors: &[] }`, apply (and noop where the artifact has it) fixtures. `leaf_coverage.py` for all seven: 0 content leaves without a kind.
- Field-meta picker `Reliability class RC` / `Zuverlässigkeitsklasse RC` with `RC1 (K_FI 0.9)` ... choices EN/DE, outline, inference reads, incremental unit reads, all snapshot-shaped JSON fixtures (sweep: 0 missing) and DSL assets (`reliability-class=rc2`, inserted right after `annex=`).
- Fix to en1992/en1996 found by STATUS-MATRIX: snapshot/artifact JSON schemas had a dangling `$ref` to `ReliabilityClass` (en1992) and the twins listed the field after `annex` while Rust has it last; json schema now an inline enum, the field is last in json/ts/graphql/proto of en1992, en1993, en1995, en1996, en1997(en1998, en1999 by construction); schema twins re-compacted with `compact-schema-json.py` (`dump_schema`).

## Per artifact
| artifact | root field id / cell | threading | tests |
|---|---|---|---|
| en1992 | cells 6 (before ieee cols), cells-extent width 11, role `Reliability` | `action_factors()` reads `self.reliability_class`; oracles `combinations(.., reliability_class)` | `reliability_class_rc3_raises_the_governing_design_effect_by_k_fi`, `reliability_class_of_the_document_reaches_the_evaluation` |
| en1993 | admission id 27, cell 2 | local `gamma_factors()` replaced by `gamma_factors(document)` over `ActionFactors` | same two names |
| en1995 | admission id 15, cell 3 | `AnnexParams::for_annex` -> `AnnexParams::new(annex, class)` + class-free `AnnexParams::charring_rate`; `facts::deflection(.., class, ..)` | same two |
| en1997 | admission id 20, cell 8, native-extent +3, independent oracle fixture column | `DesignApproach::resolve/annex_params/resolve_params(.., class)` scale gamma_G/gamma_Q by K_FI (not accidental); catalogue cases read text input `reliabilityClass` (default rc2 like `situation`) | same two |
| en1998 | no admission (derive), cell 2 | none: the seismic combination has gamma_F = 1, K_FI does not apply | `..._but_not_the_seismic_one`, `..._without_changing_seismic_verdicts` |
| en1999 | admission id 16, cell 2 | `part_en1990::uls_combinations(.., class)` + 7 check functions + scene `Ctx` | same two |
| en1994 | admission id 10, cell 7 | 35 `part_en1990` call sites, `factors/gamma_g/gamma_q(.., class)`, psi via `psi_row_of_key` | same two (second in inferences) |

Shared: `impl Default for ReliabilityClass` removed (plus the one assertion in the shared unit test). Explicit RC2 remains only in reference tables/helpers (en1991 bridge effects, en1995 `AnnexParams::en()/de()`, en1994 editor catalogue, en1997 editor catalogue) which have no document.

## Unverified / risks
- Nothing compiled. Expect rustc errors around: the new `reliability_class` parameters in the long positional signatures (en1994 `part_en1990`, en1999 check functions, en1997 `find_required_width`/`buckling_force`), `use` of `parse_reliability_class` through `use super::*` in the en1997 catalogue modules, `ReliabilityClass` imports in the en1998 sqlite codec (full paths used), `Option<ReliabilityClass>` in the en1998 diff (`Some(payload.new_reliability_class)`), the en1993 compliance test importing `uls_rows`/`combine_member_actions` from the crate root.
- Tests assume `K_FI(RC3) = 1.1` from the unverified registry rows; en1997 scaling of the DA gamma sets by K_FI (and DIN 1054) is an assumption (EN 1990 B3.3) and marked unverified in the docstring.
- Python oracles of en1993 use a local K_FI table (0.9/1.0/1.1), en1999 reads the registry; en1998 oracle has no gamma.
- Rust-side ordering of `leaf` tags, protocol files, proto numbers (max+1) assumes dense tags; proto/graphql twins were edited by script, not regenerated.
- Fixed tool bugs found on the way (rerun-safety of twin moves, flat fixture layout of en1999, inline proto/graphql of en1993/en1997/en1998); leaf doc strings of en1992/en1993/en1996 still mention the annex in a few comment lines.
