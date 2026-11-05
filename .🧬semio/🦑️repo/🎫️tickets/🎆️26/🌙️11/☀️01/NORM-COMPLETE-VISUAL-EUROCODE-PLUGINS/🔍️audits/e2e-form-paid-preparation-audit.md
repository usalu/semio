# Form Paid Preparation Audit

Bounded read-only source inspection. No tests/builds or implementation changes.

## Existing Physical Projection Interface

Owner: `🧰️framework/🔨️modules/🌱️value/📽️projection/🦀️.rs`, public snapshot_projection API.

SnapshotProjection declares associated Output:ProjectionOutput<Self> and Projection:SnapshotProjectionCursor<Self>, projection_birth_demand(), and begin_projection(grant)->RetainedWorkOutcome<(Projection,RetainedCloneProgress),ValueError>.

SnapshotProjectionCursor declares demands(original:RetainedCloneRef,body), advance(original,body,grant), ready, take_demand, take(grant), begin_close, close_demands, close_step and terminal_is_empty. Every owned candidate has explicit preparation, output transfer and retirement frontiers. ProjectionOutput separately quotes/executes placement; output extraction cannot silently become an unpaid copy.

Working common caller: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔌️drivers/🦀️.rs`. Public PagedProjection<P:SnapshotProjection> implements SemanticProjection<P>; it delegates paid birth/advance/close to the cursor, combines take and placement demand, subtracts placement grant before taking, executes placement and combines real progress. ClonedProjection<P:RetainedClone> in the same owner is the alternative paid deep-clone driver.

The parent NativeSemanticPreparation owner handles real source admission and phased projection/edit/inverse/messages/output retirement. These are reusable existing owned machinery; ColdReplayProjection alone is a cold retirement guard, not a cooperative physical preparation API.

ImmutableRootProjection is only appropriate for fixed immutable roots; it explicitly does not traverse variable payloads. Do not implement it on an arbitrary norm document merely to advertise a cheap projection receipt.

## JSON Decode Infrastructure

Owner: `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🫳️borrowed/🦀️.rs`.
JsonSourceCursor<S:JsonReadSource+Copy,V:JsonParsedValue> exposes new(source,policy,limits), normal_step_demands(), normal_step_progress(), last_turn_work(), and step(maximum_units,&mut NativeDecodeControl,grant)->Result<Option<V>,JsonError>. It can convert unfinished grammar state into explicitly funded retirement.

This pays actual JSON parsing. It does not by itself pay subsequent schema-to-ActionArgDef traversal, value coercion or model preparation.

## Missing Mutation Input Preparation Seam

`🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs` mutation_input_defs synchronously executes InputSchemaReader::parse(schema_json,resolver,false)?.inputs(). It returns an allocated Vec<ActionArgDef>. registered_input_schema_document allocates a vector of schema text candidates and parses a selected text into a new DslValue. No borrowed cached ActionArgDef interface or retained incremental mutation-input reader was found in the inspected owners.

Actual history caller `🔌️plugin/⏪️time-travel/🗄️store-state/🦀️.rs` invokes mutation_input_defs while building its owned editor. This establishes shared semantics, not paid physical construction. Its existing Vec is session-owned; borrowing it is only valid while that owner lives and cannot justify a global cache assumption.

The staged-form StagedBodyCursor already pays rendering of prepared controls. StagedFormRequestCursor pays receiving bounded request values. Neither accounts for synchronous mutation_input_defs, NormInspectorBodySource::prepare allocation, arbitrary schema traversal, or resolve_norm_edits' Clone/ToValue/diff work.

## Integration Consequence

Use existing projection/clone cursor machinery to prepare typed private P with actual progress and closure. This solves only projection acquisition. ColdMutation diff, full ToValue materialization, schema input construction and coercion remain synchronous operations unless their real owners expose controlled cursors or immutable prepared borrowed metadata.

The reusable framework gap is controlled mutation-input preparation or an owned immutable prepared-schema cache with explicit paid construction/retirement and borrowed read access. It should live in the common input-schema owner, rather than thousands of duplicated norm-specific parser lines. No existing one-call API was found that makes these operations cooperative. A guessed OriginalConstructionDemand around synchronous work would not establish physical receipts.
