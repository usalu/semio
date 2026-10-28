# IFC2x3 Catalog-Aware Details

## Implemented

The base IFC2x3 editor now renders an owned Ifc2x3DetailsProvider from 🚪️io/🪟️details. It reads the pinned IFC2x3 TC1 catalog, its own Ifc2x3Snapshot/Part21Instance/Part21Value and its own UpsertInstance mutation family. No IFC4 snapshot/value conversion or IFC4 declaration namespace is used. The current native Part21 manual codec exposes the same typeName/arguments fields as the TypeScript interface.

Full Details describes every inherited attribute of all 653 entities, optional unset/create, derived read-only slots, closed enum literals, SELECT variants, exact positive-u64 reference fields, list/array order and cardinality controls, SET/UNIQUE reference admission, complex constituent direct attributes and type variants. Simple instances use inherited slots; complex instances use direct constituent slots and effective derived redeclarations. The IFC2x3 wall has eight attributes, no PredefinedType, OwnerHistory remains required, IfcProject inherits IfcObject, and IfcLabel resolves to unbounded STRING.

Reference normalization handles both append and replacement, checking all explicit target constituents and enclosing declared SET/UNIQUE direct-reference membership. Existing instance type changes preserve identity and compatible values, generate missing GlobalIds, normalize derived slots and emit one own UpsertInstance. Existing identity rename upsert/remove pairs remain on their previous route. Original editor context propagates cancellation/fuel through lookups, duplicate scans, exact-value walks and controlled template reference searches, then checks row capacity and publication bytes before returning the existing Emit. Two invertible items cover the existing identity-rename pair; ordinary typed edits use one.

Only declaration metadata is shared across owned models: shared details/control.ts builds the common control descriptor from neutral declaration/value observations. Shared 🪪️identity primitives encode native 128-bit identities without any graph model. Native IFC4 and IFC2x3 use this same primitive; TypeScript own planners also use the extracted identity helper. Version-specific occupied-ID scans retain their own graph types.

## Actual Validation

Nine actual TypeScript source groups pass with zero strict diagnostics: all 653 entity attribute slots, independent existing AJV enum/coordinate bounds and SET uniqueItems, exact max-u64 references, unrelated arbitrary-decimal coefficient/scale/exponent preservation, complex target constituents, every callback cancellation stop, and own type variants. The identity vectors also execute against the existing independent base64-js library, with explicit IFC alphabet mapping. This library is test-only; no dependency manifests or runtime dependency was added.

The IFC4 shared suite remains passing after declaration-only metadata extraction and identity reuse; parent verification includes its publication-row tests. Native own provider, model mount, editor hook, shared identity and native test sources parse with the independent installed Rust tree-sitter grammar. Authored native tests consume neutral cases, check actual provider controls, own exact reference/type mutations and central cold diff/inverse restoration, SET replacement refusal, and the actual editor hook under one existing OriginalWorkFixture’s original cancellation/fuel. Native tests have not executed.

## Explicit Gates and Scope

Native compilation and UI/retained-host execution remain blocked by the previously reproduced Windows native build-cache case-sensitivity access-denied gate. Source parsing is not type checking. IfcOpenShell is unavailable locally; no authored IfcOpenShell check is described as executed. Parent’s separate own-catalog report records independent schema metadata comparison.

The IFC2x3 hook is currently bounded one-shot planning. It walks/charges the owned instance before its existing ToValue/edited_subtree/FromValue materialization; source/header fallbacks retain generic framework whole-snapshot materialization. It is not a fully retained/resumable Part21Instance edit-preparation implementation, and large item/inverse payloads are explicitly refused by the common publication limit. Full typed SELECT/collection templates and reference checks do not evaluate every EXPRESS WHERE/UNIQUE/inverse/global rule or full complex-instance conformance. Required reference templates can remain unset when no existing admitted target is available.

## Selected Inspection Across All Views

All eight native IFC2x3 editor/viewer roots—base, CV2.0, SAV and COBie—now declare the own topology domain, selected geometry picks, inspector window and appropriate layouts. Their existing schema/mutation modules explicitly use the exact same owned IFC2x3 base graph and instance mutations; the three view packages already depend on the base package, so their surfaces reuse its own provider/inspector without conversion. Their MVD conformance filters remain separate. Every editor delegates to the same own original-context stable-path/edit/capacity helper; viewers pass editable=false and expose no artifact edit actions.

The selected inspector maps stable ifc-u64 identities to own current instance rows under the initiating snapshot. Canonical exact identity, row reorder, missing IDs and cancellation have language-neutral/actual TypeScript checks. Native source includes actual selected editor mutation cases and readonly/topology manifest assertions for each view package. Reference navigation follows resolved own targets; missing targets are nonclickable localized labels, with explicit navigation work/depth/256-target limits. Full snapshot Details remains available separately in editor layouts.

The shared standard-owned geometry render uses no IFC4 namespace. Nonselected rendering passes no selection; selected rendering receives the caller’s own domain/granularity. Native static review corrected both selected/nonselected render_scene callsites. All eight roots and 18 additional surface/layout sources were independently syntax-checked; actual native UI/host execution remains unexecuted under the existing gate.

## Reproducible Source Checks

Ticket input harness ifc2x3-details-check.mjs exports runIfc2DetailsChecks(root,ticket). It runs own base/🚪️io/🪟️details/🧪️tests/🟦️.test.ts, which includes the selected-path vectors from own io/🔎️inspector/🧪️tests/🔣️.json and identity vectors from ifc/🪪️identity/🧪️tests/🔣️.json. Native provider tests are adjacent, with separate selected-inspector tests and view package editor/viewer unit tests.
