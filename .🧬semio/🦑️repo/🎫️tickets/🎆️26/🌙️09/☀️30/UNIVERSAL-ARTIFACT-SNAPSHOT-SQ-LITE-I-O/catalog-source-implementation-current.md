# Catalog Source Implementation Current

Read-only 2026-10-09; no execution.

Mounted Source `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🪪️bindings/🏛️catalog/🟦️.ts` reuses authentic directory package/artifact/surface/grant/browser actor types, clone-publishes rows, and structurally compares installed provenance before proposal. Duplicate batch key refuses; installed identical row idempotent; owner/capability excluded from key correctly permit conflict detection.

Concrete issues:

- Imports2/3 use four parents `../../../../📇️directory`, resolving to os/📇️directory (absent). Three parents `../../../📇️directory` resolves actual modules/directory (exists). Both type imports need actual path; type-only import may be erased at runtime, so passing runtime test alone cannot qualify source typecheck.
- Key11 includes contributor.componentSha256. Changing actual package digest at same plugin/package/version gives a different row key and bypasses provenance conflict. If package/version identifies the declaration, keep digest in compared metadata rather than key. Add same-version changed digest conflict law; owner digest mutation already conflicts because owner is excluded.
- propose21–24 accepts runtime rows without closed-schema/semantic validation. Types protect authored TS only; schema-law Ajv checks separate fixtures, not arbitrary inputs passed to proposal. Either document/require validated first-party input authority or implement domain validator before proposal. Reject empty factory/SQL, malformed digest, missing target fields, extra capability fields, invalid protocol before clone publication. Do not introduce Ajv runtime dependency.
- Recursive equality uses Object.keys for arrays; sparse array length is not compared. Current domain interface arrays are only browser importInterfaces; malformed sparse arrays can compare equal incorrectly. Validate dense string array or explicitly compare array length/indices.

Ownership behavior is good for ordinary data: structuredClone prevents original caller mutations changing proposed row; existing map rows remain untouched. Returned proposed Map remains mutable by caller, so installed registry must take its own immutable snapshot rather than storing caller-held mutable map. No Source registry is mounted here.

Meaningful additional Source edges: mutated contributorSHA sameversion conflicts; ownerpackage/digest/capability/grants changes conflict; distinct windows/role/renderer retained; malformed linkedGuest schema differs from artifact schema refuses; browseractor source hashes match contributor/target package with actual rendererpolicy; missing/extra fields fail actual proposal admission; original row mutation after proposal does not change clone. Existing Socket GUI route already executes neutral policy; no new command needed. No Source runtime/typecheck result claimed.
