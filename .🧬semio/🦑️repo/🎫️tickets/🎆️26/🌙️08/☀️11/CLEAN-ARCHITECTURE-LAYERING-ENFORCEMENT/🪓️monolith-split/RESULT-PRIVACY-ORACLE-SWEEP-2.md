# Result PRIVACY-ORACLE-SWEEP-2 (prefix `pos2`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git cat-file`, `ls-files`, `ls-tree`, `diff --numstat`), no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/pos-tools/` (round-1 tool, extended; new files listed in section 8). Logs, JSON and reports: `F/🗑️generated/pos2-*` (nothing deleted). Sandboxes (the only other place anything is written): `scratchpad/pos2-work/<family>/{cur,base}`. Baseline revision of every `base` side: `1bae6115bc5` (auto-commit of 2026-10-16 04:57, before wave 5); crates that did not exist there are skipped and listed as `absent`. No real `cargo` ran (compile admission is closed for the fleet); everything is static evidence from `rustc +nightly-2026-07-20 --emit=metadata` over a stub layer.

## 0. Verdict

* **The rustc early stop of round 1 (section 4 of `RESULT-PRIVACY-ORACLE-SWEEP.md`) is found and fixed in the tool.** Two causes, both proved by micro-tests: (A) a trait reference with `Res::Err` (unresolved, not a trait, or imported through a failed `use`) in a bound, supertrait, `dyn`, `impl Trait` or impl header raises a fatal error in ty lowering, so no body is type-checked; (B) a rustc ICE while type-checking one body aborts all later bodies. The runner now finds every case from `rustc -Zunpretty=hir-tree` (exact spans, silent ones included) or from the ICE query stack and neutralises exactly those spots in the sandbox. Result: no family needed a file quarantine any more; the unions that round 1 could not check now reach body type-check: the 49-crate kernel union `okernel` (blocked in round 1), the 47-crate store union, the 63-crate `okwp8` union (lib and test), `smg` (1 file quarantined in round 1), `dlo` (8 files), `rhs` (1 file); `ri2` (76 crates) reaches it without a cull (section 4).
* **19 slices, 46 sub-families, 92 configuration runs (lib and `--test`) reached body type-check** (column `canary` of section 2: the E0616, E0624 and E0308 of the probe module are reported, so bodies were checked). 0 files quarantined, 0 families blocked.
* **Real carve defects found and fixed (section 3): 58 one-line edits (36 widenings, 4 imports, 10 manifest dependency lines, 8 lock lines) in 34 files.** Privacy: `okwp8` 150 (lib) and 167 (test) E0624/E0616/E0451 sites on 29 members of the store crates, `rf8` 10, `ftcu` 6, `ftcj` 3, `ftcv` 2, `otcps` 1; each family re-run to **0 privacy errors**. Missing trait imports: `okwp8`/`okernel` (`OpBinary`, `Backbone`), `ftcv` (`RetainedCloneCursor`), `wbr`/`pgg` (`InteractiveJob`, static only). Missing dependency edges: 10 (6 store crates, the history decode companion, the sqlite snapshot companion, the transient-law crate).
* **Needs a design decision (section 5):** `codec-factory` calls `P::sqlite_codec()` of a trait that lives in `os-io`, a crate it does not depend on; `ri2` has 298 privacy errors in the renderer crates of the running slices `ri2`/`ri3` (not touched); the shared presence retirement test in `ephemeral` lacks 4 dev-dependencies; the ledger API surface grew by 13 members.
* **Not verified by compilation.** The oracle hides stub-layer noise; the rules are listed in section 6.

## 1. What changed in the tool (all in `F/pos-tools/`, read-only on the repo except `pos2widen.py apply`)

| File | Change |
| --- | --- |
| `poshir.py` (new) | `rustc -Zunpretty=hir-tree` piped through awk extracts every `TraitRef { path: Path { span, res: Err } }`; spans are rewritten in the sandbox to numbered stub traits `crate::__ext_cull::Tr<n>` (also macro bodies and `$crate` paths); ICE query stacks (`[typeck_root] type-checking ...`) name the body that is replaced by `loop {}`. Every rewrite is reported (`HIR-CULL`, `ICE-BODIES`). |
| `pos.py` | runs the cull after the fix-up rounds when the canary is missing and refreshes sibling configurations whose sandbox was rewritten; `ctx_trait` computed from the byte offset of the span (the whole statement head, so `where` clauses on later lines count); no text fix for diagnostics inside macro expansions; whole-word replacement; `POS_PREFIX`; logs `-final.err` and `-priv.err` of the last run; `is_trait_scope` (rustc says the trait is implemented but not in scope); `inherited` resolve rows (every name is also unresolved in the baseline) and `__m_<crate>` normalised in the diff key; stub-noise rules (section 6). |
| `postrait.py` | `deep_trait_context`, `statement_head`; E0404 always a trait position; the generics test only for `fn/impl/struct/enum/trait/type/union ... <`; bare failed aliases (`impl Identified<String> for X`) are neutralised; whole-word replacement. |
| `posunion.py` | `use crate as native;` rewritten to the crate module; missing `.rs` files that are mounted by `#[path]` (generated files absent at the baseline revision) become empty placeholders in the sandbox. |
| `posstub.py` | a path segment that is not an identifier (`$module` of a macro) never becomes a stub module. |
| `poslib.py` | manifest index ignores dot directories. **Round-1 defect found here:** the index also listed 145 manifests under `.🧬semio/🦑️repo/🎫️tickets/**` (drafts) and resolved 21 real crate names (for example `semio-framework-pack`, `semio-framework-hash`, `semio-framework-replication`) to a draft manifest. No round-1 family contains one of them as a member, so no round-1 number changes; both indexes are kept (`pos2-manifest-index-r1flaw.json`). |
| `pos2fam.py`, `pos2mk.py`, `pos2batch.py`, `pos2table.py`, `pos2dbg.py`, `pos2cull.py`, `pos2deps.py`, `pos2widen.py` (new) | crate lookup, family writer (`families/*.json`, `families/groups2.json`), parallel batch runner and summary, report table, single rustc re-run on a sandbox, file-level culprit finder (superseded by the HIR scan), undeclared-dependency scan (no rustc), widening planner (`plan`) and verified one-line applier (`apply`). |

## 2. Results per slice and sub-family

Columns: `canary` = body type-check was reached; `errors cur (noise)` = errors of the current tree (of which stub-layer noise, hidden); `base` = errors of the baseline revision (absent for absolute families); the five category columns count **non-stub, new** rows (new against the baseline; absolute for `okwp8` and families without baseline) as `first sweep > final`. Only the `privacy` and `trait-scope` columns are comparable between the two runs: the first sweeps ran with older noise rules, so the `resolve`, `type` and `other` columns of the first sweep contain stub noise that later rules hide. `quarantine` = files emptied (always 0); `HIR-cull / ICE` = trait references and bodies neutralised.

Sub-families (`F/pos-tools/families/<name>.json`): `okwp8`; `dm2v` value test support (18 crates), `dm2d` 3d (14), `dm2h` history (3), `dm2k` compiler-resources and routing-workspace (7), `dm2u` ui (6), `dm2p` pack (5), `dm2m` mcp (4), `dm2e` plugin host (5), `dm2t` transient law (4), `dm2l` mutation laws (9); `frs`; `tgm`; `orv`; `svm`; `otcps/d/c/m/h/ch/q/g/x` (services, directory, directory-client, command, history-record-decode, channel, sqlite-snapshot, config, dsl-derive); `ftcb/p/s/q/j/x/w/a/u/v/m` (io-base64, print, server, quiz, pack-json, pixels, workflow-run, async, ui-host, value, mesh-engine + space-collection); `ttc`; `hdc` (24 hub crates); `opd`, `orc`; `pqs`; `rf8`; `wbr`; `pfa`; `pfb`; `pch`; `pgg`.

| family | cfg | crates | canary | errors cur (noise) | base | privacy before > after | trait-scope b > a | resolve b > a | type b > a | other b > a | quarantine | HIR-cull / ICE |
| --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| okwp8 | lib | 63 | yes | 23161 (22612) | - | 150 > 0 | 5 > 3 | 66 > 57 | 843 > 429 | 69 > 60 | 0 | 0/1 |
| okwp8 | test | 63 | yes | 54049 (50798) | - | 167 > 0 | 5 > 3 | 160 > 145 | 3794 > 3004 | 110 > 99 | 0 | 0/0 |
| dm2v | lib | 18 | yes | 4981 (4871) | 4972 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 2/0 |
| dm2v | test | 18 | yes | 8682 (8422) | 10069 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 24/0 |
| dm2d | lib | 14 | yes | 4724 (4469) | 4714 | 0 > 0 | 0 > 0 | 7 > 7 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2d | test | 14 | yes | 7210 (6908) | 9870 | 0 > 0 | 0 > 0 | 7 > 7 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2h | lib | 3 | yes | 1710 (1670) | 1686 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2h | test | 3 | yes | 2505 (2385) | 3428 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2k | lib | 7 | yes | 2251 (2225) | 2589 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2k | test | 7 | yes | 2987 (2971) | 4316 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2u | lib | 6 | yes | 643 (562) | 633 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2u | test | 6 | yes | 1065 (917) | 1076 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2p | lib | 5 | yes | 279 (274) | 1230 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2p | test | 5 | yes | 743 (721) | 2495 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2m | lib | 4 | yes | 1159 (1129) | 1159 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2m | test | 4 | yes | 2368 (2306) | 2382 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2e | lib | 5 | yes | 1168 (1127) | 749 | 0 > 0 | 0 > 0 | 1 > 1 | 16 > 16 | 0 > 0 | 0 | 4/0 |
| dm2e | test | 5 | yes | 1974 (1918) | 1286 | 0 > 0 | 0 > 0 | 8 > 8 | 18 > 18 | 0 > 0 | 0 | 0/0 |
| dm2t | lib | 4 | yes | 8670 (8349) | 8606 | 0 > 0 | 0 > 0 | 2 > 2 | 6 > 6 | 0 > 0 | 0 | 0/0 |
| dm2t | test | 4 | yes | 14502 (13768) | 14561 | 0 > 0 | 0 > 0 | 4 > 4 | 8 > 8 | 0 > 0 | 0 | 0/0 |
| dm2l | lib | 9 | yes | 1758 (1581) | 1640 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| dm2l | test | 9 | yes | 1773 (1596) | 1655 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| frs | lib | 4 | yes | 1114 (1091) | 1105 | 0 > 0 | 0 > 0 | 6 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| frs | test | 4 | yes | 1193 (1170) | 1790 | 0 > 0 | 0 > 0 | 6 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| tgm | lib | 6 | yes | 1380 (1363) | 1380 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| tgm | test | 6 | yes | 1758 (1682) | 2911 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| orv | lib | 5 | yes | 3217 (3142) | 3994 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 | 0/0 |
| orv | test | 5 | yes | 4578 (4453) | 6148 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 | 0/0 |
| svm | lib | 6 | yes | 1900 (1801) | 1899 | 0 > 0 | 0 > 0 | 1 > 1 | 11 > 0 | 0 > 0 | 0 | 0/0 |
| svm | test | 6 | yes | 3046 (2915) | 3054 | 0 > 0 | 0 > 0 | 9 > 1 | 64 > 0 | 0 > 0 | 0 | 0/0 |
| otcps | lib | 2 | yes | 489 (477) | 489 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcps | test | 2 | yes | 979 (930) | 979 | 1 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpd | lib | 3 | yes | 126 (110) | 126 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpd | test | 3 | yes | 337 (310) | 337 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpc | lib | 4 | yes | 778 (750) | 721 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 1 | 0 > 0 | 0 | 0/0 |
| otcpc | test | 4 | yes | 1079 (1043) | 1659 | 0 > 0 | 0 > 0 | 4 > 0 | 5 > 1 | 0 > 0 | 0 | 0/0 |
| otcpm | lib | 2 | yes | 147 (136) | 134 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 1 | 0 > 0 | 0 | 0/0 |
| otcpm | test | 2 | yes | 894 (791) | 888 | 0 > 0 | 0 > 0 | 75 > 0 | 84 > 0 | 17 > 0 | 0 | 0/0 |
| otcph | lib | 2 | yes | 1138 (1113) | 1138 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcph | test | 2 | yes | 1900 (1859) | 1918 | 0 > 0 | 0 > 0 | 1 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpch | lib | 2 | yes | 878 (870) | 877 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpch | test | 2 | yes | 1206 (1194) | 1201 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpq | lib | 2 | yes | 262 (240) | 261 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpq | test | 2 | yes | 963 (606) | 1074 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 0 | 0 | 0/0 |
| otcpg | lib | 3 | yes | 486 (440) | 486 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpg | test | 3 | yes | 1859 (1779) | 1859 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpx | lib | 4 | yes | 239 (228) | 239 | 0 > 0 | 0 > 0 | 7 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| otcpx | test | 4 | yes | 426 (419) | 425 | 0 > 0 | 0 > 0 | 7 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcb | lib | 2 | yes | 0 (0) | 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcb | test | 2 | yes | 74 (38) | 74 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcp | lib | 2 | yes | 735 (718) | 735 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcp | test | 2 | yes | 1304 (1209) | 1304 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcs | lib | 2 | yes | 390 (372) | 284 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 | 14/0 |
| ftcs | test | 2 | yes | 1165 (1019) | 1160 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcq | lib | 3 | yes | 665 (662) | 665 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcq | test | 3 | yes | 3393 (3319) | 3393 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcj | lib | 2 | yes | 983 (971) | 983 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcj | test | 2 | yes | 2683 (2544) | 2683 | 3 > 0 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 | 0/0 |
| ftcx | lib | 2 | yes | 151 (133) | 151 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcx | test | 2 | yes | 651 (627) | 651 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcw | lib | 4 | yes | 802 (786) | 704 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcw | test | 4 | yes | 1281 (1223) | 1281 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftca | lib | 3 | yes | 199 (192) | 195 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftca | test | 3 | yes | 494 (463) | 495 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| ftcu | lib | 2 | yes | 520 (516) | 357 | 0 > 0 | 0 > 0 | 0 > 0 | 5 > 0 | 0 > 0 | 0 | 0/0 |
| ftcu | test | 2 | yes | 1151 (1122) | 788 | 6 > 0 | 0 > 0 | 0 > 0 | 13 > 2 | 0 > 0 | 0 | 0/0 |
| ftcv | lib | 19 | yes | 4995 (4885) | 4972 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 0 | 0 > 0 | 0 | 2/0 |
| ftcv | test | 19 | yes | 10028 (9672) | 10069 | 0 > 0 | 0 > 0 | 5 > 0 | 12 > 0 | 0 > 0 | 0 | 24/0 |
| ftcm | lib | 4 | yes | 1745 (1679) | 1619 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 4 | 0 > 0 | 0 | 0/0 |
| ftcm | test | 4 | yes | 2763 (2635) | 2800 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 4 | 0 > 0 | 0 | 0/0 |
| ttc | lib | 5 | yes | 1005 (990) | 1004 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 0 | 0 | 0/0 |
| ttc | test | 5 | yes | 2225 (2047) | 2211 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 3 > 0 | 0 | 0/0 |
| hdc | lib | 24 | yes | 8248 (8101) | 8184 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 0 | 0 > 0 | 0 | 0/0 |
| hdc | test | 24 | yes | 14254 (13953) | 14412 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 0 | 0 > 0 | 0 | 0/0 |
| opd | lib | 11 | yes | 4465 (4383) | 6022 | 0 > 0 | 0 > 0 | 18 > 3 | 46 > 21 | 0 > 0 | 0 | 0/0 |
| opd | test | 11 | yes | 6260 (6108) | 8963 | 0 > 0 | 0 > 0 | 0 > 3 | 0 > 21 | 0 > 1 | 0 | 0/0 |
| orc | lib | 14 | yes | 6899 (6753) | 6232 | 3 > 0 | 0 > 0 | 14 > 2 | 46 > 27 | 10 > 10 | 0 | 1/0 |
| orc | test | 14 | yes | 8864 (8638) | 9078 | 0 > 0 | 0 > 0 | 0 > 2 | 0 > 27 | 0 > 11 | 0 | 0/0 |
| pqs | lib | 3 | yes | 1276 (1264) | 1276 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pqs | test | 3 | yes | 2506 (2414) | 2506 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 1 | 0 | 0/0 |
| rf8 | lib | 6 | yes | 2068 (2004) | 2068 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| rf8 | test | 6 | yes | 2403 (2338) | 2403 | 10 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| wbr | lib | 4 | yes | 1056 (835) | 999 | 0 > 0 | 0 > 0 | 0 > 0 | 17 > 0 | 0 > 0 | 0 | 0/0 |
| wbr | test | 4 | yes | 1355 (1081) | 1296 | 0 > 0 | 0 > 0 | 0 > 0 | 18 > 1 | 0 > 0 | 0 | 0/0 |
| pfa | lib | 9 | yes | 15161 (14822) | 15138 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pfa | test | 9 | yes | 29541 (28678) | 29523 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pfb | lib | 7 | yes | 1128 (777) | 1128 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pfb | test | 7 | yes | 1811 (1440) | 1813 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pch | lib | 12 | yes | 1342 (1316) | 544 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 > 0 | 0 | 0/0 |
| pch | test | 12 | yes | 1633 (1580) | 745 | 0 > 0 | 0 > 0 | 22 > 2 | 9 > 0 | 0 > 0 | 0 | 0/0 |
| pgg | lib | 5 | yes | 3863 (3339) | 3837 | 0 > 0 | 0 > 0 | 0 > 0 | 1 > 0 | 0 > 0 | 0 | 0/0 |
| pgg | test | 5 | yes | 4412 (3777) | 4384 | 0 > 0 | 0 > 0 | 0 > 0 | 2 > 1 | 0 > 0 | 0 | 0/0 |

Reading notes (every residual non-stub row was opened):
* **`okwp8` is absolute.** The pre-carve tests lived in the kernel facade, which cannot be unioned with the store crates it mounts, so there is no baseline; the family is the 14 test crates, the 2 helpers and the 47 store crates. First sweep (b4) versus final: privacy 150 > 0 (lib) and 167 > 0 (test); the 3 remaining trait-scope rows are `progress` on an uninferred `_` (the suggested traits are unrelated); the remaining resolve rows are glob and alias imports of crates outside the union (kernel facade, `os-spr`), the 6 undeclared-crate imports of item 11 are gone in the final run (h4).
* **Identical errors on both sides.** `wbr`/`pgg` `next_close_depth_demand`, `pqs` and `opd` `couldn't read ...` (sandbox path of `include_str!`), `otcpm` `len` on `!`, `otcpc` `into_client_request`, `ftcu` `try_send` (`AbiPort` stub trait without methods), `ftcm` `?` on `()`: stub cascades; the same row exists in the baseline.
* **Crates that do not exist at the baseline** (`dm2e` host-wake and engine, `dm2t` transient-law crate, `dm2l`, `pch`) have no baseline rows, so their stub cascades count as new; they carry no privacy and no trait-scope row, which is the claim that matters.
* **`dm2d`: 7 resolve rows are foreign.** `HashMap` (`3d/🥽️mesh/🛠️modeling/🦀️.rs:575`), `Vec3f64` (lines 856 to 889) and `point` (`🔁️reconstruction/🦀️.rs:271`) are missing imports in the 3d mesh carve of a running peer slice; I did not touch it. Before the crates `3d-mesh-contract/-clipping/-document/-reconstruction` were added to the family the same 3d files showed 745 resolve rows.
* **`otcpd`:** `crate::client::SocketGrantReceiptV1` in the directory schema unit test (lines 309 and 314) does not resolve in the baseline either (os-directory has no `client` module): a defect that predates the slice, in the test that `otcp` moved; it is hidden as `inherited`.
* **`hdc` (trait-in-scope risk of the port replacement):** 24 crates, 0 trait-scope and 0 privacy rows in lib and test.
* **`opd`/`orc`:** privacy 0; the residual `PackVerificationLevel` and `ArtifactEnvelope` imports and `&Mutation` method rows come from glob re-exports of crates outside the union.

## 3. Defects found and fixed

Every edit is one line (a visibility, an import, or a manifest dependency), made with the editor or by `pos2widen.py apply` (it re-reads the line, asserts it is exactly the planned text, edits it, and `git diff --numstat` was checked afterwards: 1:1 per widened line, no collateral). `rustfmt --edition 2021 --emit stdout --config skip_children=true` parses all 23 edited `.rs` files.

| # | Family | File (repo root relative, tail) | Error before | Edit | Evidence after |
| --- | --- | --- | --- | --- | --- |
| 1 | `rf8` | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧠️session-core/🦀️.rs:741` | E0616 `SessionPort.authority` x10 (test target `tessellation`, mounted by the parent) | field `pub(super)` | test 10 > 0 |
| 2 | `wbr`, `pgg` | `…/🧵️simulation-session/🦀️.rs:30` (energy model) | trait `InteractiveJob` no longer imported in the root after the split; the unit test reaches `next_close_depth_demand` through `use super::*` | `InteractiveJob` added to the `#[cfg(test)] use semio_framework_job::{..}` | **static only**: the stub trait has no methods, so the identical E0599 stays on both sides |
| 3 | `otcps` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🪣️http-pool/🦀️.rs:42` | E0624 `TokenBucket::try_consume` | `#[doc(hidden)] pub fn` | test 1 > 0 |
| 4 | `otcph` | `…/📜️history/🛬️decode/🧪️tests/🧿️companion/📦️packages/🦀️rust/Cargo.toml`, root `Cargo.lock` | E0432 `semio_framework_os_history_testing_retirement` (dependency of the companion missing) | dependency line and lock edge | resolve 1 > 0 |
| 5 | `ftcj` | `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1165, 1912` | E0624 `release_demand` x2, `retained_capacity_bytes` x1 | `#[doc(hidden)] pub fn` | test 3 > 0 |
| 6 | `ftcu` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs:132` | E0616 `NativeClipboardJob.operation` x6 | `#[doc(hidden)] pub` field | test 6 > 0 |
| 7 | `okwp8` | 14 files of the store crates: `📒️ledger` (13 lines), `🔁️replay` (2), `🎮️one-item/📬️publication` (3), `🔗️backbone/♻️retirement` (2), `🔗️read`, `🚪️io/💾️binary/🪪️operation-identity` (2), `📦️codec/🧬️owned-schema/🧱️fields`, `📨️messages/✂️clamp`, `🧵️canonical-edit/🧳️source`, `🧩️composition/🚪️open` (2), `🔄️sync` | 150 (lib) and 167 (test) E0624/E0616/E0451 sites: `hash_record`, `effective_edit_digest`, `ALL`, `CLEAN`, 12 `new`/`mint`/`new_counted`/`admit_attached`/`constructor_capacity_bytes`, `PendingCommandReport.{edit_ids,messages,worst,outbound}`, `StackMirrorDirt.{applied,redo}`, `MemberOpenFrame.{snapshot_start,history_start}`, `ArtifactMailboxSender.authority` | `pub(crate)`/`pub(super)`/private became `pub` on 29 definition lines (plan `pos2-okwp8-all-widen-plan.json`, applied list `pos2-okwp8-all-widen-applied.txt`) | lib 150 > 0, test 167 > 0 (h3) |
| 8 | `okwp8` | `…/🏪️store/🔄️sync/🖥️native/🎭️actor/🦀️.rs:4` | E0599 `encode_op` on `BackboneMessage`: trait `OpBinary` not imported | added to the `semio_framework_os_spr::{..}` import | trait-scope 2 > 0 |
| 9 | `okernel` | `…/🏪️store/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs:66` | E0599 `send` on `Backbones`: trait `Backbone` not imported (the monolith root imported it) | `Backbone` added to `use backbone::{..}` | trait-scope 2 > 0 (v2) |
| 10 | `ftcv` | `🧰️framework/🔨️modules/🌱️value/🧪️tests/🥣️companion/🌱️retained-clone/📋️paged/🦀️.rs:5` and `🧬️retained-clone/📦️paged/🦀️.rs:293-294` | E0599 `advance`/`begin_close`/`close_step`/`terminal_is_empty` x13 (trait `RetainedCloneCursor` missing in the wrapper) and E0616 `PagedUtf8BoundedOrdCursor.{left,right}` x2 | import and `#[doc(hidden)] pub` fields | trait-scope 13 > 0, privacy 2 > 0, type 6 > 0 (g5) |
| 11 | `okwp8` | `Cargo.toml` of `os-store-{codec-factory,config,space,space-member}` (`semio-framework-os-store-backbone`), `os-store-testing-fixtures` and `os-store-tests-09` (`semio-framework-os-store-test-support`); root `Cargo.lock` | the crates use the other crate but do not declare it (E0432 `semio_framework_os_store_backbone`, `..._test_support`; the oracle sees these because the extern table comes from the manifests) | one dependency line each (no cycle: checked on the manifest graph) and the matching lock edge | 6 rows > 0 (`okernel` v2 for the four backbone edges, `okwp8` h4 for the two test-support edges) |
| 12 | `otcpq`, `dm2t` | `Cargo.toml` of `semio-framework-io-sqlite-snapshot-tests` (`io-control`, `value`) and of `semio-framework-test-transient-preparation-law` (`job`); `🧰️framework/Cargo.lock` for the first | `pos2deps.py` (undeclared `semio_*` heads; the oracle cannot see them because the crate is also a stub) | 3 dependency lines and 2 lock edges | `pos2deps` 0 undeclared; the transient-law crate is absent from both lock files (stale): `cargo update --workspace --offline` |

## 4. The rustc early stop: found (round-1 section 4 is closed)

Two independent causes, both proven with micro-tests (about 40 one-item files in `scratchpad/pos2-work/micro`, each next to the canary of `pos.py`) and on real unions.

**Cause A: a trait reference with `Res::Err` is a fatal error.** When the trait path of a bound, supertrait, `dyn`, `impl Trait` or impl header does not resolve (E0405, E0404, an unresolved module prefix E0433, or a name bound by a failed `use`, which rustc does not even report), ty lowering raises a fatal error. `-Ztime-passes` then shows `type_check_crate` ending right after `coherence_checking` and not one body is type-checked.

| extra item next to the canary | canary (the E0616 of the probe) reported |
| --- | --- |
| `fn f(_t: Missing)`, `use nothing::Foo;`, `let _ = missing_fn();`, `-> Missing` in a method, `<u8 as Missing>::X` in a body, `type Item = Missing` | yes |
| `fn f<T: Missing>`, `trait T1: Missing`, `&dyn Missing`, `impl Missing`, `where T: Missing`, `impl Missing for W`, `type A: Missing`, `struct W<T: Missing>`, `-> impl Missing`, and the same with a struct in the trait position (E0404) | **no** |
| E0185, E0050, E0204, E0407, E0437, E0438, E0220, E0119, extra or missing trait items, unresolved associated types, E0277, E0599, E0308, match errors | yes |

So it is not an error code but one HIR node, `TraitRef { path: Path { res: Err } }`. Round 1 handled four shapes; the residual blocked unions had these:
1. a `where` predicate that continues on a later line (`C::Snapshot: ArtifactPack + ...`): the text heuristic looked at one line only (`smg`);
2. a name imported by a failed `use` and used bare (`impl Identified<String> for Widget`): the alias neutraliser only looked for `alias::` (`dlo`);
3. a generic-angle heuristic that matched comparisons (`a<b`) and rewrote a closure parameter type; a substring replacement (`Mutation<P>` inside `SemanticMutation<P>`) (`otcpm`, `otcpq`);
4. paths produced by `macro_rules!` (`ser::$trait`, `$crate::..::ToValue`): the text fix rewrote the macro **argument** (`sequence!(SerializeSeq, ..)` became `sequence!(crate::..::Tr5, ..)`) and the macro then failed to expand (`dm2e`), and the `$crate` paths were skipped (`dm2v` test).

**The general detector.** `rustc -Zunpretty=hir-tree` still prints the whole HIR of a crate with resolution errors, including every `TraitRef { path: Path { span, res: Err } }` (silent ones too). `poshir.py` filters that dump with an awk pipeline (200 MB for `dlo`, gigabytes for a large union; never stored), the runner rewrites exactly those spans in the sandbox to numbered stub traits, re-runs rustc and refreshes the sibling configuration. Every rewrite is listed under `HIR-CULL`. It replaces file quarantine: it loses the check of one bound, not of a whole file (and the delta-debugging pass is still there as a last resort; it was never needed again).

**Cause B: rustc ICE while type-checking one body.** `okstore` (47 store crates) stopped because `typeck_root` of `StoreLog::open_conflicts` (`engine-core/📥️lifted/🏪️store/🦀️.rs:631`, `impl Iterator<Item=&Conflict>` over a stub field) panicked in `rustc_infer::region_constraints` ("cannot relate bound region"); rustc exits 101 and no later body is checked. The runner parses `query stack during panic`, replaces the named body by `loop {}` in the sandbox (`ICE-BODIES`), re-runs, repeats. The panic comes from the stub layer (error-typed field), not from the repo.

**A third trap, not a rustc cause:** a configuration whose sandbox was rewritten by an earlier configuration of the same side (the `lib` cull) was reported as blocked without a new run (`ftcs` test); the runner now refreshes it.

Validation (the same unions, round 1 versus now):

| Union | Round 1 | Now |
| --- | --- | --- |
| `smg` (5 crates, lib) | 1 file quarantined (`plugin-abi/🚪️io/🦀️.rs`) | 0; canary reached after 20 text-level trait fixes |
| `dlo` (flow/playbook, lib) | 8 files quarantined | 0; 16 fixes |
| `rhs` (hub repo crates, lib) | 1 file quarantined (server root) | 0; 7 HIR refs (gateway instance) |
| `okstore` (47 crates) and `okernel` (49 crates with the kernel facade, lib and test) | blocked, quarantine degenerate | reached; 1 ICE body |
| `okwp8` (63 crates, lib and test) | not run | reached; 1 ICE body |
| `ri2` (76 crates, lib) | reached | reached, nothing to cull (298 privacy errors, section 5) |
| `dm2e`, `dm2v`, `ftcs`, `dm2t` base | not run | reached; 4, 26 and 14 HIR refs (both sides), placeholders for the generated files that do not exist at the baseline revision |

## 5. Findings that need a design decision, belong to other slices, or were left

* **`codec-factory` needs `ArtifactSqliteSnapshotOperations`** (`P::sqlite_codec()` in `📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs:54`). The trait is defined in `os-io` (`sqlite_snapshot_operations`) and is a subtrait of `ArtifactSqliteSnapshotHost`, so the bound `P: ArtifactSqliteSnapshotHost` does not give the method; the monolith root re-exported it. The crate depends on `os-text-io` only. Options: dependency edge to `os-io` (acyclic?), or re-export the trait through `os-store-text-io`. Needs the owner of OS-KERNEL-ENGINE; one E0599 in `okernel` (lib and test).
* **Ledger API surface.** Item 7 made 13 ledger members `pub` (`hash_record`, `effective_edit_digest`, the report and mirror fields, five constructors) because `engine`, `engine-live`, `replay`, `codec-factory`, `initialization-runtime` and `space-member` are separate crates now. A `#[doc(hidden)] pub mod engine_api` (or sealed traits) would keep the public surface small; I used the minimal widening.
* **`ri2` renderer crates: 298 privacy errors** in lib (`scene_render` 87, `scene_ink` 64, `scene_feeds` 37, `canvas2d_host` 34, `scene_canvas2d` 25, `runtime_mailbox` 12, `scene_tables` 11, ...; `GumballSlot` fields, `RendererIoSlot.state`, `try_take`). The slices `ri2`/`ri3` are running in the same files, so I did not edit; `F/🗑️generated/pos2-ri2-v1-report.txt` lists them and `pos2widen.py plan` can produce the one-line plan from `pos2dbg.py ... --priv-only`.
* **`ephemeral` test crate** (`🫧️ephemeral`): the shared presence retirement test (dm2 hand-off row 32) uses `semio_framework_async`, `_io_control`, `_job_worker`, `_job_worker_authority`; the manifest declares none (`pos2-deps-okwp8.txt`). Pre-existing sharing, owner `okwp8`/`dm2` hand-off.
* **Undeclared dependencies that predate the carve** (same in the baseline, `POS2_REV=1bae6115bc5 pos2deps.py`): `semio-hub-boot-laws` (`semio_framework_os_kernel`, the dependency is renamed `directory`), `semio-framework-ui` (`ui_layout`, wgpu layout), `semio-framework-ui-text` (`trace`), `semio-framework-os-mcp-plugin-runtime` (`job`), `norm-vdi3805-inferences` (`schema_registry`), `bim-model` (`stdio_contract`). False positives of the scan: `dsl-derive-expand` (generated code in `quote!`), `pack-intrinsic-media-laws` (macro body).
* **Hand-offs:** `Cargo.lock` regeneration (`update --workspace --offline`) for the 11 manifest edges; the stale-lock crates (`plugin-host-*`, `transient-preparation-law`) are absent from every lock.

## 6. Limits of this evidence

* Stub layer: methods of external traits are invisible, derive output of external macros is stripped, globs of crates outside the union are unknown. Rows that are stub cascades are hidden when: a name of the diagnostic is defined only by the stub, a span lies in `__pos_root.rs`, the type is `()` or `!`, a method is missing on a type parameter or a `&S`, `?` is applied to `()`, `__dsl_*` items are missing, E0747 (const generics of stubs). Privacy codes are never hidden.
* `NEW` against the baseline is meaningful only for crates present in both revisions; new crates are judged absolute (privacy and trait-scope).
* HIR cull and ICE replacement lose the check of the neutralised bound or body only (listed in `HIR-CULL`/`ICE-BODIES`): `rhs` 7 gateway bounds, `dm2e` 2 macro bounds, `dm2v` 1 + 12, `ftcs` 14, one ICE body (`StoreLog::open_conflicts`, a stub-induced rustc panic, not a repo defect) in `okstore`/`okernel`/`okwp8`.
* Not covered: `[[test]]` integration targets (plan: `rf8` generation3d test roots, kernel `sqlite_snapshot_native_admission`/`artifact_command_retirement`), wasm targets, `cfg(feature)` variants other than the Cargo unification over the family, the skipped running slices `pi1 sf3 psc2 tfe rtb tt2 pgr ri3`.
* Heuristics: widening is driven by diagnostics (definition span), so a member that is used only behind an error-typed receiver appears in a later round; `okwp8` needed two rounds (29 sites, then 0).

## 7. Commands run (summary)

`python3 pos.py index --force` (twice); `python3 pos2mk.py` (46 families); `python3 pos2batch.py --jobs 2|3 --label {b1,b2,b3,b4,f1,g2,g3,g4,g5,g6,h1,h2,h3,h4,v1,v2} --cfgs lib,test [--quarantine] [--no-base] <family>...`; `python3 pos.py run families/{smg,dlo,rhs,okstore,okernel,ri2}.json` for the validation of section 4; `python3 pos2dbg.py ...` for `-Ztime-passes`, the micro-tests (`rustc +nightly-2026-07-20 --crate-type lib --emit=metadata` on 30 one-item files in `scratchpad/pos2-work/micro`); `python3 pos2widen.py plan|apply`; `python3 pos2deps.py <family>...` (also with `POS2_REV`); `rustfmt --emit stdout --config skip_children=true` on the 23 edited files. `lldb` was tried for the abort and hung (not used). Real `bun repo cargo`: not run.

## 8. Files

Created in `F/pos-tools/`: `poshir.py`, `pos2fam.py`, `pos2mk.py`, `pos2batch.py`, `pos2table.py`, `pos2dbg.py`, `pos2cull.py`, `pos2deps.py`, `pos2widen.py`, `families/*.json` (46 new specs and `groups2.json`). Edited in `F/pos-tools/`: `pos.py`, `postrait.py`, `posunion.py`, `posstub.py`, `poslib.py`. Note: another executor (`rw2t`, `pi2a`) runs `pos.py` from the same folder; the edits were `py_compile`d before every swap and keep the default prefix `pos`.
Edited in the repo (section 3): 23 `.rs` files, 9 `Cargo.toml` (+1 lines), `Cargo.lock` (+6 lines), `🧰️framework/Cargo.lock` (+2). Logs and results: `F/🗑️generated/pos2-*` (`-report.txt`, `-result.json`, `-new-*.txt`, `-final.err`, `-priv.err`, `pos2-final-table.md`, `pos2-okwp8-all-widen-plan.json`, `pos2-deps-*.txt`).
