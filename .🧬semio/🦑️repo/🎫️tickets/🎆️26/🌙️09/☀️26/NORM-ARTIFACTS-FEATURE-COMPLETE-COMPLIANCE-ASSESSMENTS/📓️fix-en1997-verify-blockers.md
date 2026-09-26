# Fix — EN 1997 verify blockers (Wave D)

Closed the 9 blocking items from `📓️verify-en1997.md`:

1. `NormFieldChoice` en+de labels in `lookup_field_meta`
2. DIN 1054 BS-P/T/A via `resolve_params` + utilization test
3. DA2* characteristic soil + action/resistance factors (not DA1 dual combos)
4. Sliding passive ≤0.5·E_p,k wired through `kp_coulomb`
5. §6.6 oedometric settlement; id-path remedy; removed 0.88 influence
6. Complete field-meta leaves + enums
7. Remedy-law tests (≥2) with strict Pass on bearing
8. Python oracle + jsonschema in Rust tests
9. Honest `📓️impl-en1997.md` with real Summary line

Runner: `Summary [   1.075s] 72 tests run: 72 passed, 0 skipped`
