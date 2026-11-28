# Project graph read: "Actual declared requested Source scope required (7566ad79)"

## Message origin
`javascriptDependencySourceOwnerRequirementsV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` throws it when a requested scope is not a scope of the published Source-owner model. `7566ad79` is the hashed reference the command shell prints for that internal error.

## Cause
`ensureBuildScopesV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts`) took the "declared scopes" from the root `package.json` `workspaces` list (134 entries). Those are member packages that the ownership publication writes into the root manifest; the real installation scopes are only 6 (`.`, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`, per `discoverBunWorkspaces`). Project roots were mapped onto member packages, so the scoped sync requested owners such as `🧰️framework/🔨️modules/🛂️manifest` that are no scope, and the owner closure rejected them after the ~20 min capacity wait and the stale-receipt re-sync (`reason: scopes`).

## Fix (already present in the working tree, written concurrently by another agent; same diagnosis, verified here)
- `javascriptDeclaredScopesV1(root)`: `.` plus every top-level folder whose `package.json` declares `semio.workspace`; `ensureBuildScopesV1` uses it instead of the `workspaces` list.
- Test `declared scopes of the checkout are owners, not members, and project roots map onto them` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts`.

## Verification
- `scope-compare.ts` (this folder): `javascriptDeclaredScopesV1` equals `discoverBunWorkspaces` directories exactly (6 scopes).
- `repo test-file .../🧪️tests/🎯️closure/🟦️.ts`: 6 pass, 0 fail.
- `repo capacity run --memory=2600 --cpu=1 -- repo run @semio-tech/os-plugin-describe-rs:build`: see `🗑️generated/describe-build-2.log`.

Result of the run: after about 6 min in the capacity queue it passed the dependency step and was reading the project graph slice (74526 source observations, 425 owners admitted) with no "Source scope" error, until my own 580 s `timeout` sent SIGTERM ("Original Script process terminated (reference 557d4d71)" is that cancellation, not a defect). A full build to completion is not yet shown; rerun with a longer timeout.
