# SurfaceVisible View Context Cap Independent Audit

Read-only actual-route census. Ticket surface-view-cap-audit.ts imports the production encodePackValue/viewContextWireValue and current parseResolvedPluginViewState, reads the exact current Rust capacity constants, and records generated/surface-view-cap-audit.json. No native decode or component execution occurred; authentic native compiler ingress remains refused.

## Which cap applies

Surface refresh creates a SurfaceVisible event whose viewState is encodePackValue(viewContextWireValue(real projected view)). The native event dispatch passes its view_state bytes to plugin_mount_surface, which checks MAX_SURFACE_VIEW_CONTEXT_BYTES and MAX_SURFACE_BODY_KEY_BYTES before decode_wire_serialized<ViewModel>. This is not a public addressed JSON invocation and does not call validate_public_json_envelope. PUBLIC_INVOCATION_BODY_BYTES=262144 and PUBLIC_INVOCATION_STRING_BYTES=4096 therefore do not size this binary context route. They still govern separately addressed action/command JSON payloads; no tool-specific cap widens those.

Current native context cap is exactly 1,242,240 bytes; body-key cap is 1,024 bytes. The formula comprises 289,808 character/digit allowance × four UTF8 bytes = 1,159,232, plus 1,297 encoded-value allowances ×64 framing bytes =83,008. This is a conservative schema-derived allowance, not the actual fixed pack header overhead. Current schema capacities: six short identifiers plus two session fields at256 characters, two long strings at65536 characters,64 utility entries,64 window instances,64 trace cursor entries,128 tree-window requests, and numeric widths16 decimal digits.

## Actual production pack measurements

| Admitted current view | Pack bytes | JSON UTF8 bytes |
|---|---:|---:|
| locale=en, terminology=native only |41|38|
| both long strings65536 ASCII each |131152|131149|
| both long strings65536 astral characters each |524368|524365|
| all current collection/scalar capacities with distinct astral identifiers |1124109|1130064|

Every measured view passes the actual current TypeScript parser and is below the native byte ceiling. The astral long-string case exceeds the public action body ceiling and still encodes normally on this separate route. Values above are actual encodePackValue results, not estimates from JSON. Existing native capacity-test commentary that JSON is never smaller than pack is not universally true: minimal and long-string cases above have pack three bytes larger. A JSON-only size oracle is therefore not general transport proof, though the current conservative allowance comfortably covers these tested cases.

The symbol-table pack has variable, data-dependent overhead: short/repeated string leaves are interned, keys are inline; collection/type tags and varints add bytes. There is no single fixed overhead applicable to every new staged payload. Numeric view fields use schema-specific exact integer carriers before encoding; ordinary JS numbers otherwise become f64.

## Proposed one-MiB staged data is not admitted yet

A hypothetical extra stagedForm payload containing one MiB ASCII encodes to1048644 bytes with only minimal context, which fits the current native byte ceiling. The same hypothetical payload added to the current filled view encodes to2172712 bytes and exceeds it. These are deliberately unadmitted hypothetical values: current closed view schema rejects stagedForm. They do not prove the proposed feature can cross native ingress. A future cap must budget complete maximal existing context plus exact staged request/token/pack overhead; increasing one payload constant is insufficient.

Native decode uses default PackLimits through decode_wire_value: max depth64, max items64000000, max symbols1000000, max total allocation4GiB, with much larger generic byte/segment limits than the mount ceiling. A proposed staged subtree depth64 must also reserve the enclosing view/request/token/args container depth; do not assume a subtree64 passes a global pack depth64. Staged65536-node/one-MiB limits must be enforced by its original receiving owner rather than relying on these loose global defaults.

## Original control and cold copies

The browser encoder builds a symbol map, full number[] output and final Uint8Array synchronously; this census does not claim original native funding for it. WIT poll converts the event Vec and SurfaceVisible carries a Vec<u8>; event dispatch receives an original cx for the turn, but plugin_mount_surface does not accept it. decode_wire_serialized is async in signature but directly calls cold decode_wire_value followed by T::from_value, with no original StepContext/loan. SurfaceContexts::insert projects/stores context; SurfaceContexts::get clones body_key and ViewModel at lines104–106. A later controlled details render/source Arc lease does not retroactively fund these ingress owners/copies.

The owning worker removed staged_form before mounting and left guided body unadvertised. Keep that refusal boundary until finite shared caps are supported by actual complete transport and original-controlled receiving/projection/retirement; do not raise broad existing constants or claim one-MiB native support from the minimal-context measurement.

## Exact source map

- manifest/🖼️view-model/🦀️.rs:247–337: current capacities and separate mount ceiling.
- manifest/👁️view-state-and-public-invocation/🟦️.ts:79/89/143: public cap versus current closed view admission.
- framework/🔨️modules/🎁️packs/🟦️.ts:207/385/428: symbol selection, actual encoder, integer-carrier projection.
- OS plugin/reactor/runtime/ops/surface-render/🦀️.rs:25: mount cap and cold decode; plugin/reactor/turn/event-dispatch/🦀️.rs:57: SurfaceVisible route.
- OS plugin/builder/program/lifted/runtime/🦀️.rs:45: decode_wire_serialized; OS pack/bridge/🦀️.rs:82: default decoder route; replication/codec/🦀️.rs:27: PackLimits.
- OS plugin/reactor/surfaces/🦀️.rs:104: cold cloned context.

Paths above are domain-relative under C:/git/semio/🧰️framework and the existing ticket script holds exact absolute paths. No production source, constants, builds or lifecycle state were changed.
