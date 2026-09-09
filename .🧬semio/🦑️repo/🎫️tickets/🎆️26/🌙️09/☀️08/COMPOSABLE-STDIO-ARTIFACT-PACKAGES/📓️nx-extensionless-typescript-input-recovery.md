# Nx Extensionless TypeScript Input Recovery

The ordinary Flow source/oracle retry exited1 during Nx graph construction. Its command input closure encountered Writer window tests importing `../../🧬️schema/🧬️mutations/🟦️` and `../../../🫧️transient/🧬️schema/🧬️mutations/🟦️`. Both TypeScript files exist and Bun resolves these extensionless imports. The shared graph walker used Node `createRequire.resolve`, which did not resolve `.ts` extensions in this host context. No Writer source was rewritten.

The existing language-neutral command-import fixture now uses extensionless TypeScript entry/re-export imports. Its independent esbuild bundle and runtime output succeeded, and the same exact closure was then queried through native Node. Before the repair, Node failed with `MODULE_NOT_FOUND` for `./🟦️`; the scoped diagnostic exited1. The graph resolver now retains normal Node resolution and falls back to the existing TypeScript bundler resolver only for `MODULE_NOT_FOUND`; declaration-only resolutions are rejected. Workspace escape validation still runs on the resolved file.

The scoped regression now exits0: Bun, Node-hosted graph resolution and esbuild report identical runtime input ownership; the bundled output executes correctly, source edits refresh the closure, erased types remain excluded and native createRequire execution still passes. Raw red/green logs are `🗑️generated/nx-extensionless-import-red.txt` and `🗑️generated/nx-extensionless-import-green.txt`. This direct diagnostic was needed because the ordinary Nx graph could not reach a target. Ordinary Flow source and retained-oracle routes are being retried after the repair; their acceptance remains pending.

This changes build tooling only, adds no runtime library dependency, and uses the repository’s existing TypeScript interface inside the Nx graph plugin. The permanent regression remains part of the existing cache-contract test entry point.

## Independent Source Audit

Audit snapshot: 2026-09-09T13:40:36+02:00.

No actionable mismatch found in the settled resolver. It preserves createRequire resolution first and invokes the TypeScript bundler resolver only when that call reports MODULE_NOT_FOUND. The fallback uses the importing absolute path as its containing file, permits JavaScript and JSON inputs, rejects an unresolved target and declaration-only .d.ts, .d.cts, or .d.mts targets by rethrowing the original Node error, then applies the existing workspace-escape check before recursing.

The fixture now makes the entry import the extensionless TypeScript barrel and makes that barrel re-export another extensionless TypeScript module. Its expected closure includes both concrete TypeScript files, while the absent type-only paths and the declaration-only input remain absent. The test compares that closure with esbuild metadata, executes the esbuild bundle, re-queries the closure from a native Node process, checks source-byte cache refresh, and separately executes a createRequire factory. I did not run those commands; I reviewed the supplied red and green receipts. The red receipt shows the expected Node MODULE_NOT_FOUND for ./🟦️ at the pre-fix resolver call, and the green receipt contains the test terminal success line.

The Writer source scan found the two extensionless imports only in the named window test file; the production Writer sources retain their existing import spelling. Native and ordinary Flow acceptance are outside this source audit and remain governed by their terminal receipts.

The ordinary Flow rerun subsequently constructed the complete Nx graph and executed all four generator prerequisites successfully. Its source target then reached AJV and stopped on a separate missing external child-schema registration. This supplies ordinary graph acceptance for the resolver fix while Flow contract acceptance remains a separate pending gate.
