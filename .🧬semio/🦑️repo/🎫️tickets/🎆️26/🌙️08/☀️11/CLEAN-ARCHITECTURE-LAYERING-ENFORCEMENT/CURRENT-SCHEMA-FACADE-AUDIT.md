# Schema Facade Audit

Observed 2026-10-03 through the actual registered framework-schema:test-neutrality route: 15 original laws ran, 13 passed and two failed. The failures require OS to stop exporting State/Composition aliases and main Schema to stop forwarding the lower State/Composition APIs. They are source ownership failures, not compiler-carrier or matcher defects.

The current OS root publicly exports StateClass from semio-framework-schema-state and aliases semio-framework-schema-composition as os_schema_composition. Main Schema's component publicly forwards StateClass and five Composition types. Thirteen actual source candidates were captured across 24,620 Rust and Cargo files under Framework and S, with complete before texts, hashes and exact matching lines in schema-facade-inputs/candidates.json. This is a textual candidate census. It does not establish every macro expansion consumer or include Hub and other repository roots yet.

Six artifact test consumers and two higher implementations still access Composition through store::os_schema_composition. The artifact packages already declare direct State/Composition dependencies. The MCP package and the Playbook procedural extension need explicit Composition dependencies. Four interaction schema test bindings access private registry types/functions through main Schema and should bind directly to its existing canonical Registry dependency. Their original expectations can remain exact.

One actual OS Mutations producer emits an OS StateClass parameter in its generated registration function. A direct canonical State provider roster for that macro's real consumer owners has not been admitted. The TextError constructor provider census proves a different interface and cannot be reused as evidence for these macro consumers. Retiring the OS State alias requires that separate provider admission and actual native compiler validation.

No shared Rust or Cargo mutation has been made for this audit. The controlled error and Record floor is being completed in a coordinated native epoch before this separate retirement. Both failing architectural assertions remain enabled; no re-export substitute, wrapper, classifier exemption, compatibility alias or test exclusion has been added.

The subsequent implemented cut is recorded in CURRENT-SCHEMA-FACADE-RETIREMENT.md. That later epoch removes the audited facades, preserves all original laws and passes the complete 15-test portable suite. Its explicit provider census and native limitations supersede the initial candidate-only planning state above.
