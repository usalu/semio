# Stdio Registry Contract Audit

## Verdict

NO-GO. The registry contract is split across stdio schema validation, the TypeScript structural gate, runtime assembly, and framework capability registration. A registry-only or glTF-only patch is invalid.

## Confirmed State

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` requires `runtime_capabilities`, but none of the thirty-six artifact definition JSON files supplies it. Parsing fails before catalog validation.
- The Rust validator rejects non-`implemented` rows and then unconditionally reports that a typed executable mapping is required. Current glTF rows are `unimplemented` with `executable_registration: false`.
- Root `📜️script.ts` rejects `runtime_capabilities` as unknown and separately requires `executable_registration: true`, contradicting the Rust validator and current JSON.
- `registry::build()` does not map codec, mutation, or inference declarations to framework capabilities. glTF schema, inference, composer, and document-codec declaration calls therefore lack registered capability evidence.
- Framework typed executable identity plumbing exists, but stdio neither attaches nor verifies those identities.
- The active glTF roster is six codecs, eighteen mutations, and fifteen inferences. The earlier recorded twenty-eight-mutation roster is not current and must be reconciled explicitly.

## Required Atomic Contract Scope

The smallest safe lease is the stdio registry/schema contract owner jointly with the framework capability-registration owner. It must decide whether support `status` is independent of executable registration, define an owned typed declaration-to-capability mapping, and update the owning schemas and validators atomically. Do not add only JSON fields or weaken a glTF validation rule.

## Required Evidence

1. All thirty-six definitions parse and pass the TypeScript structural gate.
2. `registry::artifact_assemblies()` and `stdio::plugin()` complete.
3. A focused test proves glTF capability IDs, declaration registration, typed executable bindings, and catalog/support-ledger parity.
4. The intended mutation roster is explicitly resolved.
5. Scoped taxonomy is rerun after the contract is complete.

The audit was read-only. It made no source, configuration, generated-output, or ticket-state changes.
