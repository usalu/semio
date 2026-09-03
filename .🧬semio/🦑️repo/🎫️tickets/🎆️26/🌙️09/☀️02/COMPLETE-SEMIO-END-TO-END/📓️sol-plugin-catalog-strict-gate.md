# Strict Plugin Catalog Completion Gate

## Outcome

The plugin registry now has a separate, fail-closed `catalog-complete` command. It does not change the existing `check-generated` byte-freshness gate or the warning-oriented development `check`, and it never writes descriptors, registry output, cache output, or build artifacts.

The new source preflight independently discovers all component-bearing plugin and extension Cargo manifests, resolves dependency edges through exact Cargo package identity to each package's independent component `pluginId`, validates a bounded dependency-first order, and requires both owner-root descriptor forms as regular non-symlink files. JSON and pack forms are strict-decoded, normalized only across their declared serde/pack enum representations, checked for semantic agreement, and bound to the Cargo component id, role, exact registry hash triple, canonical packed bytes, and descriptor self-hash. Raw, extracted-core, and descriptor artifacts are then accepted only through an explicit isolated build-root verifier; the repository's ambient `target/` and development `plugin-modules/` cache are rejected as authority.

The three-stage execution contract is two-phase: bounded dependency-ordered verification produces immutable receipts, and the publisher is called exactly once with the complete ordered receipt set only after every row verifies. A failed, blocked, or cancelled parent prevents descendant verification and withholds the whole publication. Per-row progress and 64 KiB artifact-hash progress are available, cancellation is checked between rows and chunks, and retained diagnostics are capped at 4096 UTF-8 bytes.

## RED → GREEN evidence

Initial RED:

```text
bun nx run @semio-tech/plugin-registry:test -- --run 🧪️catalog-complete.test.ts
Test Files  1 failed (1)
Tests       5 failed (5)
Cause: orderCatalogNodes / executeCatalogVerificationPlan / validateCatalogDescriptorPair /
       createFreshCatalogBuildVerifier / auditPluginCatalogSources were intentionally absent.
```

Final focused GREEN:

```text
bun nx run @semio-tech/plugin-registry:test -- --run 🧪️catalog-complete.test.ts
Test Files  1 passed (1)
Tests       5 passed (5)

bun nx run @semio-tech/plugin-registry:test -- --run 🧪️launch.test.ts -t 'registers the strict catalog completion target'
Test Files  1 passed (1)
Tests       1 passed | 1 skipped (2)

bun nx run @semio-tech/plugin-registry:check-generated
NX Successfully ran target check-generated
plugin registry generated catalog and launch bytes are fresh.
```

The language-neutral contract is `🧪️tests/🧬️catalog-complete/{🧬️schema/🔣️.json,🔣️.json}`. AJV independently validates the schema and fixture. Node WebCrypto independently reproduces the implementation's SHA-256 results; system filesystem APIs create isolated source/build roots and exercise missing pairs, distinct package/plugin ids, uppercase digest rejection, exact max+1 artifact refusal, raw/core/descriptor one-bit mutation refusal, ambient-root refusal, and exact byte equality.

## Required real-catalog failure census

The full target was run with an explicit empty isolated root:

```text
bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root /tmp/semio-catalog-complete-evidence-20260903
catalog-complete source preflight failed (31 issue(s), 59 manifests)
NX target failed, as required.
```

Source preflight completed all `59/59` manifests before refusing activation. The dependency graph itself is complete and topologically orders all 59 rows. In particular, `sequence` resolves the Cargo package `semio-s-plugin-imperative-control` to the independent component identity `imperative-extension-control`; the gate does not derive one id from the other.

The 31 source issues are:

- 19 missing owner-root JSON+pack pairs: `block`, `flow-extension-bim`, `flow-extension-draw`, `imperative-extension-control`, `imperative-extension-effect`, `imperative-extension-logic`, `imperative-extension-math`, `imperative-extension-text`, `playbook`, `playbook-module-procedural`, `process-extension-concrete`, `process-extension-metal`, `process-extension-robotic`, `process-extension-wood`, `sourcing-module-beams`, `sourcing-module-slabs`, `sourcing-module-windows`, `stdio`, and `trinity`.
- 4 CAD extension pairs contain placeholder `plugin`/`empty` identity rather than their Cargo extension identities: `cad-extension-aec-building-energy`, `cad-extension-aec-building-structure`, `cad-extension-aec-building`, and `cad-extension-spatial-shape`.
- 8 checked-in JSON/pack pairs disagree semantically: `architect` (`data.program` vs `data.🏛️program`), `demonstrator` (`s.sourcing.curation...` vs `s.sourcing.curate...`), `energy` (`data.model` vs `data.🔋️model`), `imperative` (`procedure` vs `imperative` app id), `mathematical` (`equation` vs `mathematical` app id), `procedural` (`2d.generation` vs `2d.procedural`), `sourcing` (`sourcing.curation` vs `sourcing.curate` schema), and `writer` (`interactiveJob: migrated` present only in JSON).

Because source authority failed, the command intentionally did not inspect or publish any of the empty build-root artifacts. The temporary evidence directory was removed afterward.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️catalog-complete.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧬️catalog-complete/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧬️catalog-complete/🔣️.json`
- `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️launch.test.ts`

Concurrent pre-existing launch changes (generation/procedural aliases and hub development environment) were preserved when launch output was regenerated.

## Residual order

1. Build and describe `stdio` from a fresh isolated build root, then commit both owner-root descriptor forms. It is the dependency root for every catalog row.
2. Produce the other 18 missing descriptor pairs only from their fresh component `describe()` output; do not hand-edit generated registry/cache files.
3. Regenerate the four placeholder CAD extension descriptors with their exact extension role/component identities.
4. Regenerate the eight semantically divergent JSON/pack pairs from one exact descriptor value.
5. Clean-build all 59 rows in the gate's dependency order into a newly created explicit root and rerun `catalog-complete`; only that run may yield the complete ordered receipt set for atomic publication.

The verifier treats build-root freshness as a capability supplied by the caller that creates the isolated root. It enforces isolation and exact content but does not claim to attest the compiler invocation itself; signing or build-provenance attestation remains a later release-authority concern.

## Unrelated focused-test residual

The unfiltered pre-existing `🧪️launch.test.ts` inventory law is currently red because the concurrently edited taxonomy declares 16 preview generator contracts while the test still fixes 14 and launch has no `jco-package-adapter` entry. The new catalog launcher law passes independently. This packet did not alter or mask those unrelated generator-contract rows.
