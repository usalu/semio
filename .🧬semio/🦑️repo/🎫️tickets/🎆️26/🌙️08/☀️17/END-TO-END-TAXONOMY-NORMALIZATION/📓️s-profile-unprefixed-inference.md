# Profile Unprefixed Inference

## Decision

The schema now distinguishes canonical emoji admission from legacy unprefixed inference. `inferWithoutEmoji: false` is frozen for `mutation-test-profile` and `standard-subset-profile`: canonical `🪆️1-any` remains valid in its exact parent context, but a generic unprefixed test name no longer matches a projected profile merely because it contains a hyphen.

This keeps catalog/profile authority explicit and lets an ordinary unprefixed directory such as `host-protocol-parity` beneath `🧪️tests` resolve uniquely as `test-case`. The normalization resolver applies the flag only to unprefixed matching; emoji-prefixed exact matching and parent constraints are unchanged.

## TDD and parity evidence

Before the change, the language-agnostic fixture failed with `directory-kind-ambiguous: mutation-test-profile, test-case`. The retained JSON vectors cover one generic unprefixed test case, one exact mutation profile, and one exact standard/subset example profile. `fast-glob` independently verifies the physical fixture census before the production inventory resolves it.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️profile-unprefixed-inference.test.ts'
1 pass
0 fail
8 expect() calls
```

A fresh inventory of the framework test module changed from one live directory ambiguity to zero: 47 entries, 17 remaining non-ambiguity violations, 2,667 ms. Direct shipped-schema validation returned `problems=[]`. The canonical taxonomy JSON SHA-256 at this checkpoint is `eed7efff44845bb774ec1723d71a6a96dc2b6b86fd65cb4fc5c863044b3286a2`.

## Scope

This closes generic unprefixed profile/test-case ambiguity. Exact catalog projection remains responsible for mutation profile identities; no regex was widened and no arbitrary test name was registered as a profile.
## Portable Rerun

The ticket test now resolves the repository from `import.meta.dir` and imports normalization through a repository-relative path; it no longer embeds the coordinator's macOS checkout path. Independent rerun:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️profile-unprefixed-inference.test.ts'
1 pass
0 fail
8 expect() calls
3.49s
```
