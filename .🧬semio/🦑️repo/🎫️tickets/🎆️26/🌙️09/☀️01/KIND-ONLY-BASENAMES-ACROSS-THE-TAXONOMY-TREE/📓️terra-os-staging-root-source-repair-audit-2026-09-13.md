# OS Staging-Root Source Selection Repair — Independent Audit

> **Current status — accepted for the bounded source-selection and freshness route.** This is not an OS lifecycle, browser, Rust/Wasm, or publication acceptance.

## Independent current-source observations

I read the five changed product authorities, current portable fixture/test, project input graph, seed, and derived launch. The fixture contains **63 unique current source paths**; its test adds the fixture and its own executable path, then requires exact equality with the `stagingRootSources` named input. The resulting additional input set is 65 paths. Every package test target whose command begins `bun ./📜️script.ts test` must include that named input, covering the five selected existing targets.

The fixture has seven profile-specific consumer rows over four meaningful consumer ids: React Vite mount, WGPU Vite mount, WGPU native descriptor source, and preparation/activation. This preserves the distinction between browser mount data and native descriptor data rather than claiming a shared runtime staging directory.

The former nonexistent WGPU package-local paths are not selected and are physically absent:

- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`;
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html`;
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml`.

The replacement source set includes the current WGPU compiler/native, native-entrypoint/module, and browser server/HTML/config authorities. The source test reads every declared row, so an absent declared path fails immediately; it also rejects duplicate identities and stale staging-root/symbol references in the selected source set.

The portable path control derives an oracle root with installed `whatwg-url` and converts it with `fileURLToPath`, avoiding platform-specific URL pathname handling. Its freshness control uses installed `picomatch` for output-directory exclusion. The test's ticket-local sandbox compares its own projected workspace root against the global root, which permits the required ticket-local temporary directory without weakening isolation.

## Registration and cache closure

The OS project declares `stagingRootSources`; the source test compares its full sorted value to the 63 source rows plus fixture/test, rather than checking inclusion only. Coordinator evidence reports installed Nx expansion for all five selected test routes: every one contains all 65 inputs, alongside native default and dependency patterns.

Both launch authorities contain exactly one `⚖️gate🔌️plugin📦️staging-root` entry in group `4_gate`, order `400.21`, dispatching the bounded existing route:

```text
bun nx run @semio-tech/framework-os-dev:test-quick --testNamePattern='one plugin staging root|staged module freshness'
```

The report documents generation of the derived launch through the current renderer: one added launch identity, zero removed identities. It does not treat the generated launch as an independently authored source.

## Execution evidence and limits

The coordinator's current ordinary selected route passed **37 tests with 119 skipped cases in 5.47 seconds**. Its actual cache-skipped registered Nx invocation passed **37/119 skipped**, test body **4.59 seconds**, Nx **6.1 seconds**, exit 0. Initial red results are retained as provenance: the first showed the obsolete WGPU coordinate and a ticket-local sandbox-prefix assumption; the input control then caught missing `stagingRootSources` metadata.

Installed TypeScript parsed the changed test with zero diagnostics. The WhatWG URL and picomatch checks are bounded third-party oracles. I did not rerun the route or launch it.

No unfiltered quick suite, browser/dev server, compiler, Rust/Wasm build, native Windows flow, staging publication, or full OS runtime is claimed. The repair establishes current source selection, exact cache input closure, platform-aware path handling, and a safe filtered launch only.
