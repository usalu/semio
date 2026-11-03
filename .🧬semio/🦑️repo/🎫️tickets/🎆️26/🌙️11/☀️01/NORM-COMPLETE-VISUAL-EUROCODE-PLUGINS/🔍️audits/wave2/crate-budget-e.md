# crate budget E (wave 2)
Scripts: `🧰️scripts/crate-budget/e_test_support.py` (new), reused `twin_leaves.py`, `model_twin_leaves.py`, `compact-schema-json.py`. No cargo/rustc run; compile-untested.

## 1. Source-boundary gate (final scan: no crate over)
- en1995-mutations (4786) and en1995-model (4726): the four generated facet texts (ts, graphql, json schema, proto) moved into 8 twin crates (`...-mutations-*-twin`, `...-model-*-twin`, `LEAF` consts, files keep their paths). Remaining ~1.4k lines each; v7 additions to facets land in the twins (json twins 2.9k / ~190 lines).
- en1991-diff (6796) and en1992-diff (5786): same twins plus `🔺️diff/🔣️.json` compacted (4511 -> 119, 3.4k -> 107 lines; content unchanged).
- en1997-checks (4522): the feature-gated oracle wiring (353 lines + 719-line fixture) is now its own crate `semio-s-artifact-norm-en1997-checks-oracle-wiring` (file stays in place); `oracle-wiring` feature of checks removed; the two `sut` adapters and `🔮️oracles/🔣️.json` point to the new crate.
- en1995-scene (264597 B): pure `🎵️modes` module is the crate `...-scene-modes` (`pub use ... as modes`). Headroom is only about 1.5 KB; the next step needs the diagrams helpers lifted into a lower crate.
- Workspaces, deps, script.ts lists and Cargo.lock edges registered by the scripts. All announced in `📌️important/requests.md`.

## 2. Mutation/payload serde `test-support`
483 crates whose lib closure derived serde under `cfg_attr(test, ..)` (all `mut-*` leaf groups, `-mutations`, `-diff`, `-schema`, `-inferences`, roots of en1990-1999, DIN/ISO/VDI, three config crates) now use `cfg_attr(any(test, feature = "test-support"), ..)` (2327 files), optional `serde` and `test-support = ["dep:serde", <each gated dependency>/test-support]` (forwarding, so a group crate enables its leaves). 246 dev-dependency feature entries were added to every crate whose tests call `serde_json` conversions (closure scan). Closure scan re-run: 0 unresolved, 0 missing. Lock edges added.

## 3. Checks
`verify build-owners sync` passed (added=7 twin/new crates); `check-cargo-authority.py` 28 crates, 0 errors; source-boundary scan clean. `- ready for rebuild: crate budget E` appended to build-en1991/1992/1995/1997 `latest.md`.
