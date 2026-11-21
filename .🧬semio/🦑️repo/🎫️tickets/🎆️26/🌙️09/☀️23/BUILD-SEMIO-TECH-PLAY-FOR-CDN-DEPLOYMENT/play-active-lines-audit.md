# Play release active-line boundary audit

The ticket preflight sums source bytes and file count. The compiler additionally sums `activeLines` across the cfg-selected source closure and compares that total against the tighter of the repository policy and the crate's declared limit. Embedded inputs contribute zero active lines.

The [read-only scanner](audit-play-active-lines.mjs) used the compiler's existing source reader on the live workspace. It reconstructed target cfg with the registered Rust compiler's `--print cfg` and reused the feature lists and debug-assertion settings from 27 retained Play release unit graphs in `wgqfntd`. Separate coverage included current Layout, BIM, spatial kernel, Flow and framework B-Rep libraries under browser Wasm cfg with declared default features. The browser cfg includes the observed release `getrandom_backend="wasm_js"` setting. The scan completed **4,164 unit and cfg combinations** with no source-reader failures or active-line overages.

| Priority owner | WASIP2 active lines | Browser active lines | Limit |
| --- | ---: | ---: | ---: |
| `semio-framework-3d-brep-engine` | 3,997 | 3,997 | 4,000 |
| `semio-framework-3d-brep-boolean` | 3,921 | 3,921 | 4,000 |
| `semio-s-spatial-kernel-semio-session` | 3,502 | 3,502 | 4,000 |
| `semio-s-artifact-bim-model-quantities` | 3,207 | 3,207 | 4,000 |
| `semio-s-artifact-layout-layout-editor-commands` | 2,884 | 2,884 | 4,000 |
| `semio-s-plugin-flow-brep` | 2,802 | 2,802 | 4,000 |

The B-Rep engine has three active lines of headroom even though its 237,299 bytes are below the repository byte ceiling of 262,144. The spatial session measured 258,938 bytes. The browser figures reflect default-feature samples. The WASIP2 figures for the listed units use retained release selections.

The separate host descriptor scan used the successful release provenance receipt `cargo-unit-provenance-cargo-artifacts-U5GNsu.json`. All 350 recorded local combinations were readable and below their active-line limits. The highest count was `semio-os-channel` at 3,687 of 4,000 lines.

The active `wGfxJi8` run had no retained unit graph when this audit began. The scan therefore measures the live files under the most recent complete retained component selection available locally, plus browser defaults and a successful descriptor receipt. One historical Note snapshot crate manifest was absent. Cargo could select different features, discover new manifests, or supply cfg from a build script that the retained component graph does not describe. The source reads were observational while the replica was live. The audit ran no Cargo command and changed only ticket files.

## November 21 B-Rep follow-up

The scoped engine-directory scan ran with default features on `wasm32-wasip2` and `wasm32-unknown-unknown` while `wrsWb7f` was waiting for descriptor capacity. All 16 configurations passed the active-line limit with zero scanner errors. Native registry reached 4,000 of 4,000 active lines with 24 files and 219,100 bytes. Solid transaction used 3,999 lines with 32 files and 208,176 bytes. The engine used 3,982 lines with 23 files and 235,954 bytes. The tightest counts were identical on both targets. The scoped scan queried compiler cfg and read source without compiling.
