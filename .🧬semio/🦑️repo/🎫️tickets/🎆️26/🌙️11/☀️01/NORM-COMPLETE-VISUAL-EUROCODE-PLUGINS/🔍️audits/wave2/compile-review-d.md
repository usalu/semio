# Compile review D (wave 2): newest uncompiled Rust (v3/v4 EN 1990-1999, NDP typed lookups, live kits, scene v4/v5)

Method: no cargo. Work list = 328 `.rs`/Cargo files newer than compile-review-c. The review-A static checkers (`🗑️generated/compile-review-a/`: `rsck.py` use/arity/struct-literal/duplicate checks, `depck`, `exh`, `traitck`, `leafck`, `diffck`, `diffck2`, `applyck`, `amb`) were run over all ten EN artifacts, DIN/ISO/VDI, compliance, registry and mechanics (about 4400 raw findings, nearly all known macro/closure false positives such as `Diff`, `FromValue`, local closures, fn-pointer params, `row_patch!` `set {}`), plus `source-boundary-scan.py`, `check-cargo-authority.py` (28 crates, 0 errors), an unmounted-leaf scan (1910 mutation leaves) and manual reads of `ndp_cell*`/`ndp_interpolate`, `SceneKit`/`Kit<K>`/`live_scene!` and their ten impls.

## Fixed
| file | issue | fix |
|---|---|---|
| en1994 `🧬️mutations/🧭️beam-design/🦀️.rs` | 4 leaves (`adjacent-span-m`, `bridge-lanes`, `patch-bearing-length-m`, `cracked-length-fraction`) unmounted again after a 21:27 rewrite; `🧬️mutations/🦀️.rs` imports them (E0432) | 4 `#[path]` mounts re-added |
| en1996 `🧬️mutations/➖️remove-{bar,confined,flange,lintel,pier,prestress,tie}/↩️inverse/🦀️.rs` | built `InsertX { .., row: .. }`, payloads have `bar|confined|flange|lintel|pier|prestress|tie` (E0560/E0063); fixtures use those keys | `row:` renamed per leaf |
| en1990 `🧬️schema/📸️snapshot/🦀️.rs` (`sample`) | `Member` literal missed 13 new fields (frame/storey height+drift, material_*, nominal/tolerance/design dimension, member_alpha_cr, second_order_analysis) (E0063) | added `0.0` |
| en1998 `🚪️io/📝️text/🧬️mutations/🦀️.rs` | `print_en1998_mutation` match lacked 6 placement variants (E0004); parser lacked their kinds | 6 imports, print arms and parse arms added (same shape as en1990) |

## Open (routed to requests.md)
- en1999: 14 leaf dirs (`🏗️change-member-*`, `🔗️change-conn-*`) unmounted after the 23:34 rewrite of `mut-member`/`mut-connection` libs, while `🧬️mutations/🦀️.rs` (edited 23:59) still imports them. Owner is mid-split, so not touched. `mut-collection-rows` has 43 files (limit 32).
- Source budgets over: en1992-model 4186 lines, en1997-model 4035, vdi3805-model 4276, iso16757-model 4433 (blank-line trimming is not enough; move a record group to a sibling crate).

## Verified clean
- `ndp_cell`, `ndp_cell_opt`, `ndp_cell_common`, `ndp_interpolate` definitions and the 7 call sites (en1991, en1993, en1996): arity and types ok.
- `SceneKit` impls (10): all derive `Default`, required items present; `Kit<K>` and `live_scene!` pattern `pub mod live = Kit<..>` match all ten invocations and each file imports `live_scene`. Trait check false positive for en1991 `scalar_owner`/`inert_reference` (both are trait defaults).
- Cargo: all new crates are members with deps; the only flagged item (`semio_grammar_conformance` en1992-io) is a local `mod`; en1995 placements dependency exists in the mutations, io and io-text crates.
- Mutation leaf/diff/apply consistency checks: no errors except the known en1992 fixture-side items in compile-review A.
- No unmounted mutation leaf in en1990-1998; no duplicate item definitions.

## Limits
Expression-level typing, borrowck and generics are not checked. Shared crates (structural-scene `scene-kit`, `live-scene`) were read only for macro/trait shape.
