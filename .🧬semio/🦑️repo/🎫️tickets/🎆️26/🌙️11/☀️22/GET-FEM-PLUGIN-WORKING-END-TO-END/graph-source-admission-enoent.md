# Source admission: a path that vanishes mid-capture

## Cause
`build:source-admission` runs `captureNxAuthorizerSourcesV1` (`⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts`) in the service process while it starts. The walk resolves imports and lists `🧬️schema` DATA directories, then `lstat`s the ancestors and files. While the fleet edits the tree, a directory or file (`…/🚀️offload/🧰️command`, `…/🏗️buildscript/📦️run-wire`) was gone by then: ENOENT escaped as "Internal error" and the service start failed.

## Fix
* `traverseNxAuthorizerSourcesV1` now wraps one walk (`walkNxAuthorizerSourcesV1`) in a bounded restart (schema policy `SourceMembershipRetry.attempts` = 4). ENOENT/ENOTDIR, `tools.authorizer-ancestry-changed`, "Authorizer source changed during capture", "Authorizer DATA directory changed" and "Original authorizer descriptor required" are membership changes: walk again from the entry (so the vanished path is simply not imported/listed any more). A path still gone after the last walk refuses with the original filesystem error and lineage as before.
* New capture option `probe(kind,path,attempt)` (kinds `read`, `entry`): deterministic seam called right before a source read and before the per-entry `lstat` of a DATA listing.
* Exit report: `explainServiceLossV1` said "killed without an exit record (its cgroup is gone)" when no cgroup was ever recorded (Windows / unplaced). The real cause was that the service failed in admission before `runHostForegroundV1` installed its crash recorder, so no exit row exists. It now says "ended without an exit record (it was not placed in a cgroup: it failed before it could write one, e.g. while admitting its sources, or was ended from outside; its log says which)". cgroup cases keep the old text. Not fixed: writing an exit row for failures before `runHostForegroundV1` (the door script prints "Internal error" outside this product).

## Tests (bun, per directory)
* `🧪️tests/🧭️ancestry`: 8 pass (2 new: vanishing dir between resolve and read; vanishing DATA entry between listing and probe, and a persistent missing path refusing after 4 walks). Also fixed an unrelated Windows separator assertion there.
* `🧪️tests/🧭️closure-law` 4 pass, `🧊️memo` 3 pass, `🚪️entry-law` run earlier, `🏡️resident/🧪️tests/🧾️exit-log` 3 pass.
* Closure-law failed once before the fix while the tree was moving (2 fail), passed after.

## Note
Another writer overwrote the authorizer `🧬️schema/🔣️.json` and dropped `SourceReadAhead` (used by the ts) plus my entry; I re-added both. Nothing was stopped or restarted.

Files: authorizers `🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🧭️ancestry/🟦️.ts`; `🏗️build/🏡️resident/🧾️exit-log/🟨️.mjs` and its test.
