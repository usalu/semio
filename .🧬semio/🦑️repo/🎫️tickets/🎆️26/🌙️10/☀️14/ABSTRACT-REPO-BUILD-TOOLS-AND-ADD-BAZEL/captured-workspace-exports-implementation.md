# Captured Workspace Exports Implementation

The current authorizer capture and immutable compiler now accept the actual private first-party workspace package export. They preserve all256-file,4MiB-per-file,16MiB Source,24,117,248-byte snapshot and8MiB output bounds. Nothing is externalized, resolved live during compilation, or copied into ticket Source files. No SDK/store/Native runtime was opened.

## Authority And Resolution

Bun locates a bare package candidate from the actual importing file; realpath supplies only a candidate coordinate. The capture first reads actual bounded root package.json bytes, requires a concrete unique published workspaces list, selects the candidate's exact declared package boundary, then reads that member's package.json through the nofollow FD reader. Its exact name, private:true, explicit Source export and target path must match the candidate. The Source entry is captured only after these checks. An ambient node_modules link cannot authorize an undeclared package, foreign path, false name/private flag or escaping export. Unrelated member manifests are not scanned; root membership cardinality does not become a new global quota.

The pure resolver is `authorizers/📦️workspace/🟦️.ts`. Supported captured exports are explicit Source/JSON targets and a shared import/require/default target, with a types annotation ignored for runtime. Divergent conditions, wildcard lookup, implicit main/index fallbacks, declaration-only targets, escaping paths and nonprivate/undeclared packages refuse. This is a closed first-party snapshot contract, not a claim to implement every npm resolution feature.

The existing manifest now requires sorted unique `resolutions` records. Root and selected package manifests enter the same Source/hash/cap census. The compiler reconstructs the mapping from those captured bytes and verifies the manifest's projection and target presence. Its virtual onResolve uses this verified map plus its captured file map only; it never invokes the locator, realpath or live workspace discovery.

The immutable snapshot is detached and deeply frozen before asynchronous compilation. Its schemas and existing limits are privately captured. Caller mutation during Bun.build cannot change the Original entry or resolution projection. This was a separately reproduced actual RED, not an inference from the new code.

## Source FD Lifetime

`readNxAuthorizerSourceV1` opens only the bounded nofollow file after ancestry checks, compares named and descriptor identities, reads exactly the initial size plus a single growth probe, compares dev/ino/size/nlink/mtime/ctime before/after, then rechecks ancestry and closes. Short reads, growth, substitutions and cancellation refuse. Undefined/frozen primary is retained when close also fails; a successful body with close failure refuses. A final signal check occurs after successful close. This strengthens the existing Source reader for authenticated package metadata without adding an image/store capability.

## Actual TDD And Oracles

- `G/workspace-exports-red.log`: actual existing capturer refused the generated private package alias; terminal1,0pass/1fail.
- `G/workspace-exports-immutable-first.log`: incorrect new test-helper relative import failed before execution; retained as an import error, not semantic RED.
- `G/workspace-exports-immutable-second.log`:2pass/42assertions/2.23s. Actual Bun snapshot compiled after every admitted live file was removed; independent esbuild census, Node package self-reference resolution and Node execution agreed.
- `G/workspace-exports-reader-faults.log`:2pass/72assertions/2.24s including11 actual reader-body fault cases using explicit IO memory ports and genuine narrowed phases.
- `G/workspace-exports-mutable-snapshot-red.log`: mutation of caller snapshot.manifest.entry after compile began caused the asynchronous actual compiler to refuse its Original entry; terminal1. The parent test/accepted failure remained failed.
- `G/workspace-exports-frozen-green.log`:2pass/73assertions/4.07s after snapshot detachment. Portable16-case Ajv/SQLite corpus,11 FD fault cases, actual Bun/esbuild/Node oracle and mid-await mutation counterlaw passed. No Source copies, SDK installation or canonical store were involved.

The new test file is imported by the existing permanent authorizer test entry. Root already owns the actual publication/SDK role recapture and any eventual runtime qualification. Current final gates and Source receipt are appended below.

## Actual Runtime-Authorizer Source Census

Source-only actual capture4259 returned0:241 files,1,539,900 bytes; digest `9ae4694e678fdca49a02fb96dda27622159890684a7a9af091c250bbe2dc19a2`. It resolved `@semio-tech/schema-validator-ts` to the actual private captured package entry. `G/workspace-exports-actual-runtime-capture.log` retains the complete path/SHA manifest and exact resolution. Metadata counted inside the same256 bound; no all99-member manifest scan occurred. This is a point-in-time Source capture, not an installed SDK or future-currentness assertion.

## Final Hold

Final Source66596 terminal0:2pass/77assertions/1.61s (`G/workspace-exports-final-source.log`). Final four-root strict73100 terminal0, no diagnostics (`G/workspace-exports-strict-held.log`). Low's independent captured-only mapping/FD/frozen data review found no concrete blocker. All own Source sessions are closed; production capture/resolver/compiler bytes are held.

`G/workspace-exports-source-held.json` retains eight actual nofollow FD SHA/byte/stamp rows and the gate bindings. Source fixture files can evolve separately from these production captures; Root owns the next complete SDK role capture/currentness proof.

The existing full authorizer fixture entry still has three direct strict receiver calls and ignored accepted store callback phases; Root separately assigned that test-only followup after this production hold. These full store/SDK fixtures were not executed or accepted by the captured-export Source gate.
