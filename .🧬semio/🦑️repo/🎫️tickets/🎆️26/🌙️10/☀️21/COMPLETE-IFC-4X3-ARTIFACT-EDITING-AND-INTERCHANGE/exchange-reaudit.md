# IFC4x3 Exchange Fix Re-Audit

Read-only production audit after exchange-fixes.md. Only new retained ticket input is exchange-reaudit-repro.ts; no source, configuration, git state or generated outputs changed. Direct reads/Bun commands use the previously documented cached repo CLI limitation fallback. Rust authentic ingress was outside this audit.

## Confirmed New Findings

### P1 — Explicit authored BIM property length units are discarded

Source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/bim.ts:102–105`.

The property map retains only NominalValue, discarding IfcPropertySingleValue.Unit. All Height/CutHeight values are multiplied by project length scale regardless of a valid explicit unit override. Repro starts with generated BIM export, changes ground CutHeight to IFCLENGTHMEASURE(1200), and supplies explicit IFCSIUNIT(LENGTHUNIT,MILLI,METRE). Expected destination cut_height=1.2 metres, or atomic unsupported result. Actual successful partial artifact has cut_height=1200. A valid authored cut plane moves by factor1000. Height suffers the same problem; cumulative placement checking can catch some multi-storey instances but does not resolve the wrong unit semantics, especially for the highest storey. Resolve/validate each property's explicit unit and typed nominal measure before projection. Source immutability confirmed.

### P2 — Shared section property sets produce duplicate FEM identities

Source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem.ts:169–170`.

IfcRelDefinesByProperties can validly assign one section property set to several members. Import derives section ID from property-set ID but appends a section record on every member iteration. Repro adds a second structural curve member with proper two endpoint relations and analysis membership, sharing the first material and section property set. Expected one shared destination section and two references to its ID (or unique valid destination IDs). Actual partial artifact has sections IDs `["ifc50","ifc50"]`. Immediately passing this artifact to fem3dToIfc4x3 refuses with `fem.identity: Duplicate FEM identity`. Cache/deduplicate section creation by property-set ID. Source immutability confirmed. A related source-inspection concern is boundary condition sharing: support IDs use the shared condition ID at fem.ts:149 although support instances refer to distinct nodes; apply per-connection support identity if the target requires global support-ID uniqueness.

## Executed Verification

- Original exchange-review-repro.ts: all five prior adversarial cases now return unsupported with no artifact. Confirmed inertia dimension, analysis membership, factored load assignment, translated site and contradictory storey placement guards.
- New exchange-reaudit-repro.ts: both new findings reproduced; exact graph equality before/after each import confirmed with bigint-preserving serialization. Shared-section re-export independently fails with fem.identity.
- Full Bun exchange suite, with SEMIO_IFC_ORACLE_OUTPUT pointing to retained generated/ifc-results: **48 passed, zero failed, 189 assertions** across five files. Includes original guards, releases/additional conditions/supported length/condition axes, unsupported action placement, action membership and duplicate load assignment, raw malformed Unicode, official fixture roundtrips, IfcOpenShell reconstructed graphs and three.js bounds.

The new physical-dimension resolver preserves exponent vectors and refuses the prior label-only weakness. Existing placement/membership/load guards inspected here match their narrow documented supported subset. No production source mutation was performed, and successful tests do not supersede the two newly reproduced foreign-input gaps above.
