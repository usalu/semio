# Nx Semantic Source Ignore Correction

CE1 actually terminated1 before tests after140064ms: Nx rejected a static npm:ajv edge whose sourceFile was Process native-build/lease/🎯️target/tests/🟦️.ts because it was absent from both its project file map and nonproject file map. The file physically exists; its lease test caller literally imports it dynamically. The retained selected Process graph `selected/9787f5208c9b27647dff7736883575c19c02e69987ed2aa15b4fce43028f5034.json` includes the exact file in Process moduleGraph metadata and the exact static edge. The failure was not evidence of file deletion or installed-material failure.

## Defining Cause And Change

Root `.gitignore` had two global basename rules, `🎯️target/` and `🎯️target-*/`. The first excludes this genuine semantic source directory from Nx's Git-aware native index even though explicit selected import discovery consumes it. The portable graph/Nx edge projection correctly preserves sourceFile; Nx's original validateDependency/getFileData correctly requires that source in its real map. Neither guard was weakened.

The correction removes only those two ambiguous emoji rules. It preserves raw Cargo `target*` exclusions and canonical `.cache`, `.🧬semio/🦑️repo/⚡️cache`, node_modules, dist and generated-output exclusions. There is no exception naming Process, no source rename, edge suppression, implicit-edge substitution or file-map adapter. No graph schema, output interface, Source authority, writer/namespace, original control, quota or cache reuse changed.

Current authored output inspection found `.cargo/config.toml` target/build under the ignored canonical Rust cache, with ordinary target/build leaf names. Controlled Source-directory enumeration of framework and S, excluding known dependency/cache/generated partitions, found exactly one directory matching emoji target/target-*: the actual semantic Process lease owner. Exact authored configuration/source references found no emoji compiler output declaration outside those ignored partitions. This inspection does not certify every arbitrary local unqualified artifact as Source; actual registered guards still apply.

## Portable Law And Evidence

The new language-neutral11-case corpus is owned by `build/inputs/membership/tests/ignore`. It declares Source paths with emoji target, emoji target-native and plural targets; generated cases cover raw target/debug, target-prototype, actual compact cache, canonical Rust target/build, generated output, node_modules and dist. Existing Git-ignore syntax is the defining configuration contract; no test-corpus schema or parallel runtime grammar was introduced.

The law reads the actual current root ignore configuration. The independent existing `ignore` library checks every expected exclusion. A genuine installed Nx `WorkspaceContext` indexes a physical portable fixture using the same configuration; every Source must occur under the declared project's actual file map, and every generated case must stay absent. These libraries are test-only and are not new runtime dependencies. The law is imported by the existing workspace build graph Source suite.

True RED before the ignore edit: independent ignore refused the first genuine semantic Source case (expectedfalse, actualtrue). Receipt: [nx-ignore-red2.log](🗑️generated/actor-cargo-cut/nx-ignore-red2.log). Current GREEN: **1 pass, 0 fail, 25 assertions**, both independent ignore and genuine Nx native inventory agree on all11 cases, with three positive project Source ownership checks. Receipt: [nx-ignore-green.log](🗑️generated/actor-cargo-cut/nx-ignore-green.log). Focused strict/noUnchecked TypeScript: **0 diagnostics**, [nx-ignore-types.json](🗑️generated/actor-cargo-cut/nx-ignore-types.json).

The first attempt additionally imported Nx validateDependency; that import correctly refused the unissued original fixed namespace before any assertion. Its negative receipt is [nx-ignore-red.log](🗑️generated/actor-cargo-cut/nx-ignore-red.log). No fabricated namespace or Store witness was supplied. That extra imported validator is absent from the pure law; genuine registered Nx graph validation remains Root-owned and unqualified by this isolated pass.

## Current Freeze And Limits

Four owned files are frozen in [nx-ignore-source-inputs.json](nx-ignore-source-inputs.json): root ignore, neutral corpus, focused law and its existing graph-suite import. Root ignore SHA-256 is11166ab084616a0b59687559ef53e2942e9ef66d52459718b758ab87adcd1295. All absolute paths remain within256 characters. The portable fixture is removed by its paid test cleanup; all ticket receipts remain retained.

No actual factory, frontend, Cargo/native compilation, dependency initializer or Store was executed. No inference/observation Source or concurrent Cargo admission/endpoint/Trace/Async Macros files were edited. Runtime separately owns the source-analysis provider failure. Old44/47 Source receipts remain historical; Root must collect a fresh combined current roster and original captures after all concurrent lanes freeze. The original registered CE2/Actor qualification has not run here.

Read-only adjacent frontier: OS and Repo tsconfig exclusions also contain `**/🎯️target/**`, and workspace-cleanup protection classifies emoji target basenames as Cargo output. Those files were not changed by this bounded graph/ignore task; Root was notified to coordinate their defining owners. This report does not treat them as corrected.
