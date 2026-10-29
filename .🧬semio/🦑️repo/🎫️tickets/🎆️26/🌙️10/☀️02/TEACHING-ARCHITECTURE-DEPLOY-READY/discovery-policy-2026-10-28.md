# Captured Discovery Policy Packaging

Qualified owner build 80251 completed with authoritative exit 1 before application execution. Its first failure was nx:authorizers: the generated authorizer compiler tried to open .cache/b/t/m/🔣️policy.json. No qualified product receipt or dep-info coverage was published.

The exact cause was the eager readFileSync at caching/🔍️discovery/📂️source/🟦️.ts. Its import.meta.url was the generated compiler bundle location, so ../../🔣️policy.json resolved into the generated tools store.

The source now imports the authored policy as static JSON and clones the captured value, like its adjacent authored schema imports. CACHE_POLICY_PATH remains exported and resolves through the existing physicalSourceOriginV1 identity. Existing authorizer snapshot machinery captures the policy bytes and SHA, transforms the physical module identity, and loads the closed input image; no policy is copied into the cache, no synthetic fallback or runtime authority is introduced.

The new language-neutral fixture and closed schema live under the discovery source’s 🧪️tests. Its actual regression uses captureNxAuthorizerSourcesV1, snapshotNxAuthorizerSourcesV1, and compileNxAuthorizerSnapshotV1 under genuinely issued test Original/control. The generated module is relocated into ticket generated output without a policy companion. Before the fix it reproduced the exact ENOENT. After the fix it passes: **1 test / 8 assertions**, including captured policy SHA and snapshot membership, exact original policy coordinate, equal policy contents, and source selection matching the fixture and independent Lodash filtering/sorting. Ajv validates the fixture.

Existing focused owner checks also pass: **2 tests / 66 assertions**, acyclic cache-owner typing and source walking without following symlinks or generated stores. Handle 13416 completed with authoritative exit 0.

Separate existing fixture failure: the coordinate test passes policy and bootstrap coordinates, then fails at command-source test line 218 because it expects the cleanup source to mention caching/📜️script.ts. Current cleanup imports CachePruneScript from caching/🧹️pruning/📋️orchestration/🟦️.ts. Read-only inspection confirms this module actually owns the exported class and caching/📜️script.ts itself imports that same owner for routing. The direct consumer is structurally consistent with the current separation of class ownership and CLI entry; exact peer authorship/intent was not independently established. No unrelated cleanup source or fixture was changed, and this separate textual expectation does not invalidate the actual policy bundle regression.

Changed files:
- caching/🔍️discovery/📂️source/🟦️.ts
- caching/🔍️discovery/📂️source/🧪️tests/🟦️.ts
- caching/🔍️discovery/📂️source/🧪️tests/🔣️.json
- caching/🔍️discovery/📂️source/🧪️tests/📋️schema.json

Retained generated evidence: proctor-qualified-build.log, discovery-bundle-tests.log, discovery-policy-focused.log, discovery-policy-consumer.log. These are input to the parent ticket cleanup; no source was restored from Git and no modifying Git commands were used.

## Separate Command-Source Line218 Failure — Read-Only Audit

Current line218 expects the workspace cleanup consumer to contain ⚡️caching/📜️script.ts. Actual cleanup Source imports CachePruneScript directly from ⚡️caching/🧹️pruning/📋️orchestration/🟦️.ts and invokes new CachePruneScript(cachingRoot, this.repoRoot, this.invocation).run(dry ? ["--dry-run"] : []). This preserves original invocation and canonical owner without re-entering the router. Its process-table refusal and dry-run forwarding remain visible. The failing expectation is stale architecture evidence, independently of the bundle policy repair. Do not change production routing to satisfy it.

Recommended coherent test correction:

1. Keep coordinates.rootPruneConsumer at the actual cleanup Source path, and keep policy/bootstrap coordinates checked independently. Rename the coordinate test so it does not imply pruning still launches an executable/router.
2. Add the cleanup consumer to fixture.consumers with owner 🧹️pruning/📋️orchestration/🟦️.ts. Existing test95–104 then binds its actual relative owner edge through the same fixture machinery as other consumers. The closed schema already supports this consumer row; no schema extension or new owner is required.
3. Replace the router substring at218 with an actual import-declaration assertion resolving the CachePruneScript named import to that owner. Keep the cache-prune log assertion only as supplementary output evidence. For behavior/custody, assert the actual constructed owner receives cachingRoot, this.repoRoot and this.invocation and that run forwards the dry-run branch; the existing TypeScript AST dependency can inspect this without executing destructive cleanup. Do not settle for an arbitrary substring that can match a comment.
4. Preserve all owner graph checks49–93, especially no owner import/export back edge to caching/📜️script.ts, strict diagnostics and DFS cycle detection. The cleanup file is an external CONSUMER, not another cache-domain owner; do not add it to owners or change the13-owner count.

Declared input identity is already coherent: cacheCommandSources explicitly includes both pruning orchestration Source and the workspace-cleanup consumer Source. Keep those entries and the fixture/schema/test paths. No product Source, fixture, schema, tests, capture inputs or live handles were changed by this audit. No gate was run or added before stable qualifiedretry44967.

Qualified retry 44967 subsequently passed the real authorizer boundary, normal owner build and receipt publication with authoritative exit 0. No package fallback was needed.
