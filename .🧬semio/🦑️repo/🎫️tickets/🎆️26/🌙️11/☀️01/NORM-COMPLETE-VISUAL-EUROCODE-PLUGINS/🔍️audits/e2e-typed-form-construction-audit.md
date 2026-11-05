# Owned Typed Form Construction Audit

Read-only source review. No implementation, tests or builds.

## Existing History Input Route

Public owner `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🪟️panel/🔢️pointer/🦀️.rs` exposes time_travel_default_value, time_travel_input_rows, time_travel_input_at, time_travel_variant_selector, time_travel_switch_variant, time_travel_coerce and pointer insert/remove/set helpers. The time-travel module reexports panel_pointer::*; plugin's public app::time_travel is the facade used by plugin-owned history callers.

Working consumer: `⏪️time-travel/🧵️instance/✍️draft-inputs/🦀️.rs`. draft_time_travel_input stages a private candidate, resolves its current active input, coerces a supplied value, performs list insertion/removal or variant switching, reconstructs discriminators, validates the full payload against its compiled schema, rebuilds the actual mutation, canonically encodes it and only then accepts the draft. Missing validator/inputs refuses schema-unavailable. This method is pub(super) instance plumbing, not an application-callable generic “open form” API.

Executable source examples are `🔌️plugin/🧪️tests/🧪️time-travel-part-2/🔢️input-shapes-lists-unions-and-sources/🦀️.rs`: schema-derived list item seed, excluded numeric floor and union switch assertions.

## Defaults Are Draft Seeds

time_travel_default_value synthesizes numbers from bounds, strings from first option or empty string, booleans false, vectors by dimensions, arrays by minItems and required object fields. Object members prefer explicitly declared ActionArgDef.default; Any becomes Null. References may become empty strings/lists.

These are type-shaped draft seeds, not guaranteed domain-valid rows. They do not supply unique product IDs, meaningful references or engineering-valid configurations. Schema validation and typed/native admission remain necessary. No automatic valid generic ISO/VDI product factory was found on the inspected Add route. An absent InsertItem.value is missing user input, not permission to insert a blank product.

## Tagged Variant Switching

time_travel_switch_variant removes members declared solely by inactive variants, preserves ungrouped/current-variant fields, retains undeclared entries and inserts absent new-variant fields ONLY when their ActionArgDef declares an explicit default. It then sets the discriminator. Thus it avoids retaining known old-only fields, but does not guarantee a complete/valid replacement. The history caller's validation/rebuild is essential. It does not magically repair arbitrary stale undeclared keys, and metadata must describe all variant fields.

The current scalar inspector writing only kind bypasses that process. A clean domain variant edit should stage a replacement of the whole internally tagged typed boundary, gather missing required fields through declared controls, then decode/validate/native-admit it. Reusing only a discriminator write would preserve an invalid old payload.

## Current Framework Staging Interface

Public `semio_framework::action_staging` explicitly reexports StagedFormRequest/Token/Status/Event/Edit/Binding, StagedFormPage, StagedFormLayout and bounded request/census controls from the owned manifest arg-schema crate. Definition owner: `🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📝️staging/🦀️.rs`.

Tokens bind action/revision/window/generation; requests verify window and locale. Status carries ready and diagnostics. Bindings support fixed edits, typed text, choices and Preserve/Replace layout. This is the reusable draft-event protocol, not a persisted document mutation.

Plugin ordinary accessible rendering is at `📜️declaration/👁️view/staged-form/`: StagedBodySource provides already-prepared title/status/controls; StagedBodyCursor builds under retained grants with explicit close/retirement. StagedScalarKind includes Text, Choice, Button, ReadOnly and Status. These controls provide a real readonly mechanism in the staged-form system, separate from the norm inspector's current FieldView.

## Reusable Integration Boundary

A supported Add should open a typed draft for its actual insertion payload, use schema-owned defaults only as draft seeds, collect required values, validate/decode and run domain admission, then emit one existing typed InsertItem mutation. A domain-owned insert palette may offer a fully valid explicit preset; an argument-free generic Add has no such preset today.

A variant change should use the same draft machinery at its typed parent replacement boundary. Shared form/variant helpers can be factored or explicitly reexported from their owned framework module if the public app facade is unsuitable; do not duplicate history-only state transitions or expose an arbitrary JSON editor. No plug-and-play generic “valid product constructor” was established by this review.
