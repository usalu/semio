# Canonical IO Metadata Host Bridge Plan

Read-only/ticket-input preparation during fresh dependency maintenance. No production/provider source, native executor or producer was changed or launched.

## Single Routing Truth

Actual registered IoEntry retains from,into,fidelity,direction,sniff function,run and owned_serializer. descriptor_of already preserves from,into,fidelity and sniff presence. Registered-only io_native_routes drops fidelity/sniffs; describe then writes owner/counterpart/direction into the manifest. io_entries also includes legitimate synthesized SQLite routes and must not replace the exact20 registered-only publication.

Keep canonical registration as the only source. Do not hand-add20rows to app.io, invent Exact/false values, or use composer-only declarations to fabricate executable route metadata.

## Proposed Schema-first Shape

Use the existing product-neutral framework IO-schema IoEntryDescriptor as the sole full directed edge record. Strengthen the manifest contribution to:

```text
IoRouteContribution:
  route: framework.io.IoEntryDescriptor
  direction: import | export
```

Native owner and counterpart are derived losslessly from direction and route endpoints; they are not stored again. The registered-only runtime projection likewise retains this full edge plus native direction. Remove the old lossy manifest row rather than keep two parallel metadata representations or a compatibility parser.

The existing IO-schema fidelity spelling is Exact,Canonical,Semantic,Lossy. Direction is existing lower-camel import/export. sniffs means the actual callback exists; it does not assert positive confidence. Each field is required; unknown/missing fidelity/sniffs must not receive defaults.

Keep actual composer_entries in their existing separate contribution channel. Describe's current composer-import insertion into io_entries has no original full edge properties and must not be promoted with guessed fidelity/sniff values.

The manifest currently Serde-derives its contribution graph while the neutral IO descriptor only implements first-party ToValue/FromValue. Reusing the exact type therefore requires explicitly qualifying that existing publication serialization port. Prefer declaring the neutral data type's serialization through the already used first-party value serde interface, with no new direct third-party runtime dependency, or completing the actual manifest's existing native publication interface; do not add a second fidelity enum/string parser or a runtime compatibility adapter. This is an implementation prerequisite to review, not a claim that the proposed nested shape compiles already.

## Exact Producer/Consumer Frontier

- IO mechanism: IoNativeRoute declaration and its two registered-entry projection arms888/889. Project descriptor_of(entry), retaining actual properties.
- Describe plugin_io_contributions133–158: native rows preserve the complete edge; composer declarations remain composer_entries only. Keep native-kind ownership filtering derived from direction.
- Manifest plugin-manifest431 and actual TypeScript schema projection rows-t-z706: replace the lossy shape, schema/type/parser/export facets together. Existing generated/committed descriptor pairs require normal owned describe settlement, not manual20row edits.
- Capability ContributionRow and catalog compilation: derive native/foreign keys from exact direction; preserve format capability identity.
- OS Run wasmtime-node-host.rs501: pass the actual published full route edges into IoRouter.register_plugin instead of [] and preserve the separate composer slice.
- MCP artifact resolve_plugin_export_formats133–145: use canonical published export destinations, per native dialect when actual document binding supplies it; plugin-wide union must be explicitly scoped, not misrepresented as available for every artifact.

Existing IoRouter full edge schema and constructor rows already retain fidelity/sniffs. Do not churn their fields or synthesized SQLite metadata just to change publication.

## Discovery Is Not Execution

Current MCP artifact_export uses resolve_plugin_export_formats only for declaredExportFormats diagnostics. Its requested format is matched against media output ports, then workspace.export_artifact_media executes that selected port. Canonical format discovery alone will not make JSON/TXT/ZIP/drawing IO execute.

A real format export path must resolve the actual document/native dialect, select the explicit full target dialect, resolve the registered route with caller fidelity/hop controls, capture actual native document/read authority and use the genuine owned serializer or public worker IO job with progress/cancel/output/paid source closure. The existing media-port operation is a different capability; do not relabel it as canonical IO conversion or guess a default destination. Actor/OriginalCore and native owned-job prerequisites remain open.

## Ticket-only Neutral Inputs

io-route-metadata.case.json and io-route-metadata.schema.json are retained inputs, not production registries. They explicitly author:
-20 registered edges:7imports,13exports.
- Every native/foreign/from/into identity, direction, genuine fidelity and sniff presence.
- Per-native discovery and8unique plugin destinations.
-8 route reachability/fidelity/hop cases, including exact refusal of lossy drawing and obsolete DWG standard.
-10 malformed/publication/ownership refusal cases, including missing properties, direction contradictions, cross-owner conflict and explicit same-owner policy.
- Composer separation and omission of synthesized SQLite from this20row roster.

Expected values come from the current actual2d/3d/5d typed route declarations: JSON/TXT/ZIP are Exact; drawing exports are Lossy; every import constructor installs its deserializer_sniff callback, so its flag is true even when confidence is None; serializer exports have no sniff callback. These are explicit expectations, never default inference from file suffixes.

The fixture includes desired reject-before-mutation semantics for same-owner changed metadata. Existing host tests qualify identical same-owner idempotence and cross-owner rejection; changed same-owner property rejection remains a deliberately new schema-first requirement, not an existing observed pass.

## Actual Test Mounts and Oracles

Strengthen the existing Puzzle Hub --lib9 owner so actual registered entries→actual describe bytes→independent full decoded metadata→actual typed host registration→canonical discovery preserves all20rows. The existing independent Hub roster remains the identity source; avoid a duplicate production list.

Existing publication selector:
semio-plugin-tests-06-2 --lib package_descriptor_lists_its_io_mechanism_rows_with_their_native_side.
Retain its two real entries and inspect actual emitted descriptor pack→independent serde_json; add exact full metadata and composer-separation assertions.

Existing host selector:
semio-plugin-host --lib io_router_.
Retain route-order/max-hop/ownership controls and add the actual published-row registration bridge, no runtime doubles as the descriptor-projection oracle.

Existing TS framework-kernel registerTests5 and strict IO-schema parser provide the independent language implementation. Ajv validates the strengthened actual wire schema; serde_json independently decodes actual native emitted metadata. Neither test runner nor two first-party graph implementations count as a third-party graph oracle.

Use the existing test-only SQLite engine behind its first-party interface for an independent graph witness: recursive directed paths with cycle prevention/hop bound, minimum edge fidelity and strongest-minimum then minimum-hop selection. Compare reachability/rank/hops and validate all returned edge keys; equal optimal paths may differ. This adds no runtime graph dependency. The recursive query and native/TS assertions are proposed, not executed.

## Gates Still Open

No fixture/schema executor, descriptor rebuild, host graph execution or native test has been run for this new bridge. Fresh managed descriptor/strict pair publication, actual IO format execution, material output and progress/cancel/cleanup, actor, TXT, VM, UI and all prior native/persistence gates remain required. Existing empty vectors and current staleness reports are not runtime success evidence.

## Fixture Wire-shape Qualification

The neutral20 rows contain native/foreign and coordinate strings as independent expectations. Actual proposed wire rows contain only nested route plus direction, using typed dialect objects; native/foreign must not become redundant wire fields. Negative mutation paths now explicitly target /route/fidelity, /route/sniffs or /direction. Direction contradiction checks derive the native endpoint then require the plugin's actual declared native identity. Registration ownership is an argument, not an invented descriptor property. Same-owner changed-property reject carries proposedRequiresRootDecision: current host owner-only conflict checks/or_insert retain old metadata, so this new stronger policy is not an observed current invariant.

Parent approved the stronger consistency rule after this qualification: identical same-owner metadata is idempotent; changed metadata for the same owned directed key must reject atomically before graph mutation. The corresponding ticket case is now approvedNewHostPolicy, still requiring new production test/implementation. Parent's SQLite checker qualified8route and7discovery assertions over20neutral rows (not production bridge/router).
