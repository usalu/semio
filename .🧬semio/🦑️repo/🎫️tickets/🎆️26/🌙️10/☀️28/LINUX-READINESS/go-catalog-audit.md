# Go Entity Catalog Ownership

The missing client `EntityKindCatalog` and `EntityKindByEmoji` symbols belong to the existing Repo CLI Go projection. The framework schema producer continues to own TypeScript and Rust. No new module, compatibility API, framework producer ownership change, or Go workspace edit was required.

The existing pure CLI-owned emitter and standard generated-file writer refreshed the 58-entry Go source through registered Repo tooling. Its canonical source SHA256 is `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242`. The ownership test's stale fixture reference was corrected to the actual owner test hierarchy.

The client root-package suite passed 472 top-level tests in 18.015 seconds, including its framework-catalog projection checks. The existing neutral Ajv/projection ownership proof also passed. The authored `@semio-tech/repo-client:generate-entity-kinds` target remains live for integration proof; the pure refresh does not claim an issued Script Original invocation.

See [native-final-validation.md](native-final-validation.md) for the complete native gate results and remaining integration work.
