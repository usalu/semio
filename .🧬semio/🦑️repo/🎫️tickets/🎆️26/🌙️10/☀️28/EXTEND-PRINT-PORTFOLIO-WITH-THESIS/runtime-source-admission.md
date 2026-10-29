# Runtime Source Admission Diagnosis

Read-only investigation of canonical69241 and retained generated/canonical/run-2.log. No source, manifest, schema, store or queue edits were performed and no build was launched.

The log reports Closed runtime Source entries required while preparing stale projects, then explicitly falls back to a refresh. That failure is separate from the subsequent front-door peer/custody wait, which core is investigating. The log contains no failing target identity or stack; it cannot determine which invalid target was encountered first.

## Exact Authored Producer Defects

A bounded scan of the actual root owner registry found64authored targets with runtimeSourceEntries. Exactly2 fail the schema using independent AJV. Each also deterministically throws the identical retained-log error through the actual owned readBuildRuntimeSourceEntriesV1 consumer. Both place JSON data resources in the executable module entry list:

### @semio-tech/repo-search-go:test-oracle

Manifest: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/📋️project.json`. Independent AJV original=false, actual error=`Closed runtime Source entries required`. Source roles separated in memory: executable/module schema and resource schema both pass=true. No rewrite was applied.

Executable entries:

- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧵️shards/🟦️.ts`

Data entries incorrectly declared executable:

- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧫️fixtures/🧵️shards/🔣️.json`
- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json`
- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧫️fixtures/🌲️snapshot/🔣️.json`

### @semio-tech/repo-workspace-go:generate-source-policy

Manifest: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/📋️project.json`. Independent AJV original=false, actual error=`Closed runtime Source entries required`. Source roles separated in memory: executable/module schema and resource schema both pass=true. No rewrite was applied.

Executable entries:

- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🙈️source-opacity/🟦️.ts`

Data entries incorrectly declared executable:

- `{workspaceRoot}/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json`
- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json`
- `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`

## Consumer Contract And Proposed Correction

The project schema RuntimeSourceEntries permits only explicit non-declaration JS/TS modules (js, jsx, ts, tsx and c/m variants), with finite counts, paths, no globs and uniqueness. readBuildRuntimeSourceEntriesV1 in graph/admission rejects malformed declarations with the exact error. targetScriptSources in graph/inference/script-inputs calls this reader before granting executable source authority at line547. Separate SourceResourceEntries/readBuildSourceResourceEntriesV1 already admit exact data resources, consumed from metadata.semio.closure.resources.

The narrow semantic correction is to retain each existing TS entry in runtimeSourceEntries, move only the listed JSON resources into metadata.semio.closure.resources, and preserve their existing exact target inputs for freshness. Both targets already list these JSON paths in inputs. This uses the existing schema distinction and does not widen executable authority to data. The in-memory split was independently accepted by AJV and the actual owned readers for both targets. Root may authorize/coordinate the manifest correction with its owner; no changes were made during this read-only task.

This establishes two sufficient authored declaration errors. It does not assert that fixing them alone resolves custody/front-door scheduling or makes a canonical build pass. Normal admitted target execution and inspection are still required.

## Independent Current Recheck — Resource Channel Discrepancy

A fresh read-only recheck after the reported owner correction finds that executable declarations are fixed, but the claimed explicit JSON resource admission is not yet consumed by the current source. No edits were made.

- @semio-tech/repo-search-go:test-oracle: actual runtime reader accepts1 TS entry; independent AJV agrees. Supplied runtimeSourceResourceEntries has3 JSON files and passes the standalone resource-array schema, but actual consumed closure.resources is absent and the owned reader therefore returns an empty array. Full authored project schema valid=true; target-related schema errors: `[]`. Missing exact inputs among the newly supplied resource paths: `["{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧫️fixtures/🌲️snapshot/🔣️.json"]`.

- @semio-tech/repo-workspace-go:generate-source-policy: actual runtime reader accepts1 TS entry; independent AJV agrees. Supplied runtimeSourceResourceEntries has3 JSON files and passes the standalone resource-array schema, but actual consumed closure.resources is absent and the owned reader therefore returns an empty array. Full authored project schema valid=true; target-related schema errors: `[]`. Missing exact inputs among the newly supplied resource paths: `[]`.

The current schema/consumer channel remains metadata.semio.closure.resources (script-inputs line527). runtimeSourceResourceEntries appears nowhere in current admission/schema/inference code and is not included in the target source-input digest projection (line889). Standalone validation of that array does not prove the target actually admits it. Both original sets of exact inputs remain present; search also now names an additional snapshot JSON outside the target explicit inputs. The parent was informed immediately.
