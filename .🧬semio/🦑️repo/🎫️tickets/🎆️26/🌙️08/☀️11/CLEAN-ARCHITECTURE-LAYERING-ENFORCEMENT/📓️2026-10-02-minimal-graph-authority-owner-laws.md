# Minimal Graph Authority Owner Laws

Read-only source inspection; no jobs/tests/native execution. Provider replay 51453 is parent-reported queued, not a receipt.

All owner-relative paths below use Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Existing light graph loop

`🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:197–201` already runs an in-memory pure graph loop, without rustc. Its fixture `🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json:1051–1057` has moduleGraphCases rows `{id,target,files,expectedManifests}` including absolute-mount-is-not-relative and malformed-manifest. It asserts only manifest membership and cannot expose stale targets. The row named ambiguous intentionally expects two legitimate manifest roots. No dedicated closed JSON schema for this fixture was found at the conventional schema path; this test's schemaPath :21 refers to taxonomy.json, not a schema for moduleGraphCases.

The stronger canonical schema-first owner is existing rust-source-direction fixture/schema/test. Its schema already owns `$defs/moduleScopeFact` (:1425) and `$defs/moduleScopeProof` (:1461); test imports graph/fact/selector APIs :9 and validates scope facts in native macro cohort :360–361. Add a separate pure graph authority law in that existing test file and a closed `graphAuthority` table in its existing fixture/schema, rather than extend the native30 macroOwners cohort. This gives a cheap pure Bun law in the already registered producer route.

## Minimal closed data contract

Each row: required `id`, `files` (string-to-string authored bytes), `unreadable` (unique string source locators, allowed empty), `target`, `expectedContexts` (array of closed `{crateRoot,manifestPath,modulePath,sourceScope}`), `expectedTargets` (array of closed `{crateRoot,modulePath,target}`), `expectedInvalidManifests` (unique string locators), and `expectedScopes` (closed array using moduleScopeFact). No raw NUL-delimited keys in fixture; derive API keys from crateRoot/modulePath at test boundary. Use callback `path => unreadable.includes(path) ? undefined : files[path]` while inventory remains Object.keys(files). Preserve defined empty string explicitly.

Five minimum rows:

1. **readable-empty-root**: manifest lib path root.rs; root.rs bytes `""`; expected root context, no targets, scopes root [] [0,0), invalid manifests [].
2. **unreadable-inventoried-root**: same files, unreadable [root.rs]; expected no context/target, scopes [] (missing source authority, not inspectRustModuleGraphFacts("") output).
3. **readable-empty-mounted-leaf**: root bytes `mod sealed;`, sealed.rs `""`; expected leaf module context and target sealed, root leaf scope [0,0).
4. **unreadable-inventoried-leaf**: same files, unreadable [sealed.rs]; expected no leaf context/target, scopes [] for unavailable leaf; readable root context may remain according to unresolved child prefix contract.
5. **invalid-manifest-revokes-known-target**: valid manifest; root first mounts existing sealed.rs then `#[path="/absolute.rs"] mod bad;`; target sealed.rs. Expected invalid manifest [Cargo.toml], no contexts and no targets from that root. Existing absolute fixture only contains the bad module, so it cannot witness a previously granted target surviving revocation. Add reverse declaration order if final-pass independence is not otherwise asserted.

The valid-empty positives are necessary: both newly identified grants concern absent callback bytes, not an empty Rust body, which is legal. Existing conventional-not-manifest fixture is readable nonempty and does not distinguish them. The current30 native macro owners are all nonempty source strings and likewise provide no empty-source positive.

## Oracle and harness limit

For callback omission, the independent oracle is plain captured membership/readability (`Object.hasOwn(files,path)` plus unreadable set), distinct from the production graph; no native compiler can validate an unavailable callback while a file exists in memory. Retain already owned native scope cohort as independent Rust structural evidence. For target revocation assert complete sorted target entries and context projections, not only target-source manifest census; this is precisely the missed stale-map state. Direct parser scopes for missing bytes must be selected through the captured authority layer, never obtained by calling parser with a replacement empty string.

Existing physical-reference pure loop can receive matching context/target assertions if its contract is also closed, but adding schema ownership there first costs more than using the existing Rust source-direction schema. No new runner, compiler cohort or generated artifacts are needed.
