# Naming and Import Consistency (Norm Plugin)

## Decisions

1. Project name: `@semio-s/plugin-norm-mechanics` is the only `@semio-s/` project name in the repo. All other plugin projects use `@semio-tech/*`, and the norm Rust packages use `@semio-tech/norm-<code>-rs`. Renamed to `@semio-tech/norm-mechanics-rs`.
   - Left unchanged: the Rust crate name `semio-s-plugin-norm-mechanics` (Cargo convention, shared with many `semio-s-*` crates and Cargo.lock files). Renaming it would require Cargo.lock regeneration across several workspaces.
   - Left unchanged: historical references in older ticket audits and notes.
2. Process helper import: canonical is `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript` (package `@semio-tech/process`, a workspace member in the root `package.json`, used by 4141 files and by the dev harness). The `🧰️framework/📦️packages/🟦️typescript/🏃️process` copy is a stale duplicate from Oct 21, with 110 differing or missing files. norm-js and the bridge now import from the canonical path.

## Files changed

- `✏️s/🔌️plugins/📕️norm/🧮️mechanics/📦️packages/🦀️rust/📋️project.json` (project name)
- `📋️build-owners.json` (project name entry)
- `✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📜️script.ts` (process imports)
- `✏️s/🔌️plugins/📕️norm/🏭️bridge/📜️script.ts` (process imports)

## Verification

`repo run @semio-tech/s-norm-composition-tests:verify-norm-document-contract`: see the result section below. Log: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/verify-norm-document-contract.log`.
