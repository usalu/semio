# Current Member Source Ownership Audit

Read-only inspection on 2026-10-14. HEAD before and after inspection: `7c8af563c74abe9a1c8b0f751224c7201311dfa3`. Concurrent uncommitted edits were present; this is an observed working-tree review, not an immutable commit certification. No tests, runtime execution, Rust compilation, Git mutation, or application edits were performed. This replaces no prior report and does not carry forward its obsolete law count.

## Actionable Finding

The Source guard `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts:114–139` token-scans the extension members document without applying the production-item filtering used by `selected()` at lines 21–42. Therefore an unchanged accepted members document wrapped in `#[cfg(test)] mod false_proof { ... }` retains every token examined by the members invocation loop: the macro invocation, declared enum names, four arm tuple parts, producer type, and constructor spelling. That loop increments `arms` and admits this test-only witness although the production table is absent. This is a static inference directly from control flow; no acceptance execution was authorized or performed.

Call chain: Root `📜️script.ts:6407` → `policyReadMemberStoreProducerSources` at 16893 → `toolJobMemberStoreProducerExact` at Source line 59 → extension member loop at 114. Specific self-tests already exercise extension mutation through the guard but do not currently enforce a test-only members-body refusal. The input contract checks shape and path labels; it cannot distinguish this body attachment. Apply production-context filtering to the member macro invocation and add the refusal in the Specific-owned tests.

## Ownership and Contract Observations

- Generic Source production imports only the existing General schema validator, General Rust tokenizer, its local declaration, and its local domain schema. No Specific import or Specific file read exists in that defining module.
- Generic Source tests construct their body map exclusively from the Generic declaration's ten General roles, with explicit `extensions: []`. The guard itself is imported from its defining module. No Specific import/read appears in these Generic Source tests.
- Generic coverage imports this guard and its tests directly at coverage lines 1–2 and invokes the Generic self-tests at line 665. Existing unrelated Root helper imports at line 3 remain; this is not evidence that the entire coverage suite has no Root-mediated Specific access.
- Root explicitly reads the Semio extension declaration and binds its producer/members paths in `policyReadMemberStoreProducerSources` at 16893–16904. It also checks canonical store-law mounting and the catalog-law include attachment there. This explicit workspace composition is separate from the Generic empty-extension fixture.
- Specific tests import the General domain schema and guard types. The Specific base schema references General `#/$defs/ModuleReference` at lines 60 and 77. The General schema has no reciprocal Specific reference. The inspected references are corrected `$defs` references; this review found no old `#/definitions` reference in those two extension links.
- The actual-input schema is closed at the top level, body level, extension level, declaration level, and role level; extensions are mandatory. Production calls existing `compileJsonSchemaSubset` before inspecting bodies. General role paths are schema constants and checked again against the declaration. Extension path labels are checked against their explicitly supplied declarations. Extra authority keys, missing mandatory fields, and wrong input shapes are intended to refuse through this contract. This review did not execute validator conformance.
- Most semantic checks inspect selected named function/trait/struct/impl bodies rather than concatenated whole-file witnesses. String literals normalize to empty quoted strings, reducing literal decoys. General law checks establish named nonempty function bodies, while mounting is checked by Root; neither constitutes execution of those Rust laws.

## Limits

The guard establishes bounded lexical source intent, not Rust name resolution, control-flow correctness, compilation, runtime ownership behavior, or proof of the named law assertions. Arbitrary caller-supplied path/source labels are not authenticated file reads; Root supplies the actual workspace documents. No claim of Rust runtime proof or passing tests is made. Files were being finalized concurrently, so subsequent fixes require a fresh targeted reread.


## Final Static Recheck After the Production-Root Fix

The earlier actionable finding above is historical. A fresh read on 2026-10-14 observes that `items` now accepts `rootOnly`, includes it in its cache key, skips nested modules and cfg(test) items, and skips nested delimiter groups for root selection. The extension members branch selects exactly one production-root members macro with `items(extension.members.source, extension.declaration.membersMacro, undefined, true)` and inspects only that selected body's declared enum arm block. It no longer scans arbitrary raw whole-document invocation tokens. The previously described test-only module, foreign module, and enclosing wrong-module witnesses therefore have no selected root table under this code path. Specific fixtures now contain all three wrappers, and the Specific self-test applies them through the actual guard.

This is a static confirmation of the targeted repair, not a new test run. Root reports HighRuntime reproduced three accepted RED wrappers and subsequently reported GREEN Generic 65 / Specific 13; this reviewer did not execute those checks and does not independently certify that runtime result.

Generic defining tests still import only the General guard/schema and local fixtures; their file reads derive exclusively from the General declaration roles and use explicit `extensions: []`. There are zero Specific reads/imports in those defining tests. Root still explicitly mounts the Specific extension through its declaration in policyReadMemberStoreProducerSources. Existing unrelated Root imports in the broader Generic coverage suite remain outside that narrower zero-Specific-read statement. The actual input schema remains closed with mandatory extensions and closed bodies/declarations/roles, checked by the existing production compiler before body inspection. No new runtime dependency was observed.

### Current SHA256 Observations

These hashes were measured only during this final recheck; no hashes for the earlier vulnerable body are inferred or invented. Concurrent edits may change them after observation. They identify file bytes, while the initial report's HEAD identifies its observed repository commit.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts`: `7d9055cb1c4354d7b42761683cf29db9f0c217c5cf7357bc4cd4a676a1b3e23c`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧬️schema/🔣️.json`: `edc8ffe6d80c69fbe10b934cc4ac77ed8d9eea33ee75902d7509272e25dfedfb`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧪️tests/🟦️.ts`: `abda851f8631bb19c693df4d1c2208ced436b810340cd79f291db3897db5e59f`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🧪️tests/🧫️fixtures/🔣️.json`: `d67f2355d19e6e8749d7fefbcc244c73d62423b179b13dd41ca1907bfb3da40c`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🧪️tests/🟦️.ts`: `878a7a9cc36e30a69dcceaed80e67993b76cfcb7d0571a36f96977f7ed12bc8b`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📥️source/🧪️tests/🧫️fixtures/🔣️.json`: `93698bb8b3e773716c58f4cce587f33f8fa02077a0dcf24f38ac978c2a9b7719`.
- `📜️script.ts`: `bbc82ae1a46e9a9bcffe02ce3b2c0a6fe30e3bfd3786f18d2771a5842e982b69`.

The bound remains lexical source intent. No Rust execution, runtime ownership proof, test run, or compilation was performed in this recheck. No additional actionable issue was found within the three requested table-wrapper scenarios.
