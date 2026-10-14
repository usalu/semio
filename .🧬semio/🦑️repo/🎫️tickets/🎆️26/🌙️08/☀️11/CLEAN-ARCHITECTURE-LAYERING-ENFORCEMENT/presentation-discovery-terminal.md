# Presentation Discovery Terminal Evidence

Actual supported caller: `bun repo run semio-framework-os-infinite-directed-presentation:test-source`.
Working directory: `/Users/ueli/Documents/semio`.
Tool execution session: `30528`; initial output chunk `6accf7`; final wait output chunk `7859c6`; final exit status `1`.

No filesystem terminal log was captured. The tool transcript retains the actual output; the final output was truncated by the tool (21,395 original tokens, 1,035 lines). This report preserves the visible exact diagnostic and caller identity rather than inventing a complete log or a failing TOML pathname. Process IDs `47069`/`47070` mentioned by Root were not independently captured by this caller.

The frontend printed its original package script:

```text
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo run semio-framework-os-infinite-directed-presentation:test-source
```

The terminal advanced through `repository:build` stages `cache:graph-store` and `build:discover`, then failed before the presentation assertion body. The visible diagnostic was:

```text
TomlError: Invalid unicode, character in range 0xD800 - 0xDFFF is reserved
 fromTOML: true,
  wrapped: null,
      col: 29,
      pos: 6632,
```

Visible stack locations were the existing third-party TOML parser `@iarna/toml/lib/toml-parser.js:781:32`, `memoizedFileRecord` in observed build source `👁️observation/🟨️.mjs:206:78`, `memoizedObservedComputationV1` at `72:6`, inferred graph source `🕸️graph/📥️inference/🟨️.mjs:656:104`, and `compute` at `688:12`. The exact failing TOML source path was not visible in the retained excerpt. A subsequent read-only scan of visible Cargo manifests under Framework, S and Hub plus root Cargo found no escaped surrogate tokens. This does not exclude ignored/generated or independently mounted discovery inputs.

Root and graph-runtime enforcement were notified. No rerun, raw assertion-body invocation, duplicate lock initialization or shared process/lock changes were performed. Presentation native/source acceptance remains unqualified.


## Fresh Original Route After Dependency Publication

The Root-authorized fresh invocation used `bun repo run semio-framework-os-infinite-directed-presentation:test-source`, preserving original caller resources. Session 46442 returned status 1. Actual full output is captured under `🗑️generated/presentation-source.log`. It reused the validated immutable Bun generation `0eb52d0359036bd9343b4c996f46362c7488ba5100e3a52420c00c7e6433ad33`, then failed before build discovery or test assertions in graph-store resource cleanup. Process leases `🟦️.ts:231` `rmSync` returned EFAULT on the actual queue ticket pathname. That pathname and original stack remain in the captured log. There is no fresh parser failing input or source-law acceptance. This is the original route's observed terminal failure, not a new Cargo/build/assertion receipt. Subsequent manifest changes require Root's fresh actual dependency publication and remain independently unqualified.
