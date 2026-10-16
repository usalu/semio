# TypeScript Declaration Owner and Checked Syntax Boundaries

The declaration inspector now has one focused, dependency-free owner at `library/🔍️discovery/🟦️typescript/📋️declarations/🟦️.ts`. The exact old declaration region was removed from the large discovery module, which explicitly reexports the existing public inspector and its seven fact/span/diagnostic types. Repository Source search found only the neutral declaration test as a direct consumer; it now imports the focused owner. No production call or authority API was added. Root’s other discovery regions and CPU’s compiler/barrel work were preserved.

## Source and Behavior

The extracted region had approximately 789 lines; its focused successor is 796 lines / 53,790 bytes, including its existing marker/docstrings. It imports no library or filesystem module. Discovery was 936,818 bytes / 11,882 lines at the final census; peers also edit that module, so these aggregate values are observations rather than exclusive change attribution. The focused owner’s absolute path is 132 UTF-16 code units.

Required parser token access now checks actual token presence and enters the existing explicit EOF recovery when absent. Optional lookahead remains optional. Scalar scanner code-point reads explicitly reject EOF instead of using non-null assertions. Lexical code-unit tests use `charAt` only within bounded loops or lexical lookahead; Unicode identifier reads still use complete code points. Conditional branch ranges are typed pairs, and heritage iteration uses actual entries. No blanket non-null assertion or cast was added to suppress strict errors.

The output schema and all existing neutral semantic vectors remain unchanged. A closed `boundaryCorpus` definition and 16 new neutral cases cover truncated declarations, generic/member/parameter boundaries, named aliases, Unicode escapes, strings, regexes, templates and TSX. Each new case is independently rejected by the actual TypeScript compiler’s syntax diagnostics; the owned output must remain schema-valid, incomplete and coordinate-bounded.

## Executed Evidence

- Original expanded strict log `🗑️generated/vite-domain-strict-expanded.log` contains 36 unchecked declaration-region diagnostics.
- Strengthening the existing standalone grammar check with `noUncheckedIndexedAccess` produced genuine RED: 94 tests passed, one strict grammar law failed; `typescript-facts-unchecked-red.log`, actual handle3691 collected exit1.
- First extracted/checked successor: 95/95 tests, 1,153 assertions; `typescript-facts-prototype1.log`, actual66372 exit0.
- Final prototype54953: 96 tests, 1,326 assertions, 2.47s, exit0; `typescript-facts-prototype-final.log`.
- Final release22075: 96 tests, 1,326 assertions, 2.55s, exit0; `typescript-facts-release-final.log`.
- Final expanded strict ES2022/noUnchecked roots include the direct tests/oracle, actual generic Vite module and existing private prerequisite: prototype89605 and release78904 both collected exit0. Logs `typescript-facts-types-prototype-final.log` and `typescript-facts-types-release-final.log`. The standalone grammar law also independently compiles the exact focused Source text through TypeScript’s compiler API.

The existing `sdk-derivative/📜️script.ts` `vite-environment` and `vite-environment-types` routes now include these focused roots. Their already-registered editor commands remain the executable entry points; no new script or command was created. The 96-test totals include the existing five Vite environment laws alongside 91 declaration laws. All own handles are terminal. No Rust/native/store/CLI/SDK/mount execution occurred.
