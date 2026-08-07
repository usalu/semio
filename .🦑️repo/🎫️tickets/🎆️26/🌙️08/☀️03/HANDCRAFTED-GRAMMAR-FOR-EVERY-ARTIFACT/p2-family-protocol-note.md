# P2 family protocol fragments

`taxonomy.json` `artifactSpecFilenames` only names spec files under artifact facet dirs (`🗣️dsl`, `🔧️op`, …), not under `👪️family/*`.

Family dirs today hold only `📖️family-*.grammar.semio` + `🦀️component.rs`. Adding `📡️family-*.protocol.semio` would be an unlisted leaf filename unless taxonomy is extended.

**Skipped** standalone `📡️family-*.protocol.semio` files for P2. Protocol vocabulary for families stays deferred until M1 `use` loads grammar-adjacent protocol paths or taxonomy allows extra family leaves.

**F8 engineering:** No `family-eng` dir — extended `family-sheet` with `eng-node` / `eng-record` for fem2d/3d/vcs (same F3/F8 sheet family per `family-notation-guide-v2.md`).
