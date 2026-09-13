# Test Selection Consumer Review

Date: 2026-09-13  
Status: **bounded source-consumer repair accepted; Admin registered route passes and Remodel semantic parity remains red.**

This review read the bounded repair report and current sources without running Nx, Vitest, browser, MCP, Rust, or Remodel processes.

## Admin

`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts` selects the component case and the separate command-routing case, retains its in-source I18n test through `includeSource`, and resolves setup from the package root. The command-routing owner at `🧪️tests/🧪️command-routing/🟦️.ts` has Vitest’s per-file Node directive before imports. It reads the package’s `📜️script.ts`, checks that `TestScript` consumes the test level before forwarding Vitest arguments, and preserves the owning config’s empty-selection failure. Its `process.cwd()` location is valid because the package router invokes Vitest with `this.root`. The component case no longer imports the Node command API into jsdom.

## MCP

The MCP config selects direct suite leaves under its own test concern and excludes only `🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts` as a standalone file. That exclusion does not remove its behavior: `🌉️mcp/🟦️.ts:162-164` dynamically imports `registerTests1` when `import.meta.vitest` is present, and `includeSource` loads that owner. The helper registers the path matrix plus explicit executable, missing-artifact, and mutable-target controls; it is not an untested orphan. The corrected inference command targets the existing `🧪️tests/💡️inference-bridge/🟦️.ts` owner.

The coordinator’s actual native-loader MCP focused result is therefore appropriately limited: one source owner, eight unit cases, no MCP process or Rust build claim.

## Remodel

The current config includes committed example cases and the schema suite at `🧬️schema/🧪️tests/🧩️suite/🟦️.ts`. The schema suite reads vectors only from the subset-owned `🧫️fixtures/🧬️mutations` root and the demo asset from `🖼️assets/🎬️demo`; it binds each vector to `🧬️schema/🧬️mutations/<slug>/🧪️tests/<case>/🦀️.rs`, checks tag coverage and unique fixture identities, and preserves byte comparisons for snapshots and non-refused diffs.

A read-only filesystem census in this review found 134 fixture vectors across 35 mutation slugs and zero missing corresponding native case files. That corroborates the repaired native-case ownership link. It does not execute the TypeScript oracle or its native tests.

## Evidence and Limits

The coordinator’s retained portable projection reports the three selection rows green; its native MCP source-owner run is 8/8. Its first registered Admin and Remodel invocations are correctly retained as failures that exposed the jsdom and stale-reader defects. I did not rerun the pending registered targets, so this report makes no claim about their final execution result.

## Final Coordinator Delta

The coordinator’s final isolated registered Admin route selected all three current files and passed 18 tests. Vitest took 18.39 seconds and Nx 20.7 seconds with cache skipped, after the shared setup limited DOM polyfills and cleanup to environments where `Element` exists. MCP remains 8 passing source-owner cases.

The final isolated Remodel route selected all four files and reached 1,353 tests: 1,023 passed and 330 failed in 36.2 seconds. The retained 116 grouped failure assertions contain no missing-path or missing-module errors. They expose real schema/diff protocol mismatches, including an extra `locale: null` value and `0` versus `0.0` serialized-byte differences. This closes the source-consumer repair only; it does **not** establish full Remodel semantic parity.
