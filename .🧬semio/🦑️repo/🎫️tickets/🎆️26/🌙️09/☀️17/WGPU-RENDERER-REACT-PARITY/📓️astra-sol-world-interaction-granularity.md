# World3d Instance Interaction Granularity Parity

## Scope

This packet aligns the WGPU World3d instance interaction target with React's `world3dInstanceInteractionTarget`: one rendered instance resolves to the exact pair `(interactionGranularityId ?? domainGranularityId, interactionId ?? id)` for pick, hover, and marquee. It does not change component/sub-object dispatch or scene dispatch provenance.

## Fail-first receipt

Root's full World target receipt `Nextest 04b955e0-536c-40c1-b891-9e1deb5ad4dd` ran 238 tests: 236 passed, two failed, 243 were outside the filter. The failures were:

- `an_instance_pick_selects_the_topology_target_at_object_granularity`
- `a_marquee_release_replaces_with_the_deduplicated_topology_targets`

Both failures read the shared fixture's expected `handle` granularity while the native implementation hardcoded `object`.

## Schema and neutral fixture

The renderer schema now exports `World3dInstanceInteractionV1`. It declares `interactionId` and `interactionGranularityId` as optional instance-record fields. The neutral oracle separately measures UTF-8 bytes against the native 256-byte identifier authority so JSON Schema’s code-point `maxLength` is not misrepresented as a byte limit.

The shared pointer fixture retains one instance without an override, so it resolves through the scene's `handle` default. A second rendered instance declares `interactionGranularityId: "node"` while sharing the same `interactionId`. The cases therefore distinguish:

- scene fallback: `(handle, extrude@solid)`;
- explicit override: `(node, extrude@solid)`;
- exact pair dedup: the full marquee retains both pairs even though their ids match.

The independent Node oracle validates every fixture instance through Ajv against `World3dInstanceInteractionV1`, derives the target pairs without React, and keys dedup with `granularity + NUL + id`. The mounted React suite plays the same fixture through the real `World3dHost`.

## Native implementation

`World3dSceneInstanceEntry` parses the optional camel-case override. Bridge admission is bounded by the existing authorities rather than a new cap:

- encoded instance lane: `WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY`;
- retained instance count: `WORLD_INTERACTION_OBJECT_CAPACITY` (1,024);
- render, mesh, topology, and granularity identifier bytes: `WORLD_INTERACTION_ID_BYTE_CAPACITY` (256).

The bridge retains explicit overrides in `instance_interaction_granularity_ids`. Pick and hover use the same resolver. Hover retains the granularity beside the topology id so moving between two instances that share an id but use different granularities is not suppressed as a duplicate. Marquee reserves override bytes, emits each instance's resolved pair, and deduplicates the exact serialized pair rather than the id alone. The cfg-test pointer helpers use the same resolver.

Dynamic retirement removes one topology-id entry or granularity entry per step, then clears the retained hover granularity. Its terminal predicate now requires these owners to be empty.

## Fixed-slot receipt

Native79 measured the legitimate retained-state footprint increase caused by the 48-byte bounded override map and 24-byte exact-pair hover owner:

- `Option<AdmittedSurfaceEntry<World3dState>>`: 23,464 -> 23,536 bytes;
- `AdmittedSurfaceMap<World3dState>`: 50,040 -> 50,184 bytes for its two retained slots.

The committed heap-first fixture records those measured values. Capacity remains 256 and `boundedThreadStackBytes` remains 1 MiB; no capacity or stack bound was widened.

## Validation

Canonical command:

```text
bun nx run @semio-tech/framework-renderer-react:world3d-interaction-check --skip-nx-cache
```

Receipt `🗑️generated/astra-runtime/world-granularity-react.log`:

- independent pointer oracle: 75 checks clean;
- interaction selection-set oracle: 17 checks clean;
- merge vocabulary oracle: 89 checks clean;
- mounted React: 16/16 passed;
- Vitest: 5.42 s;
- Nx: 7.0 s, cache skipped.

`rustfmt --emit stdout` parsed the modified native source and pointer-law file. Cargo was not run by this agent; root owns the native lane.

## Exact native filters

```text
test(the_instance_interaction_carrier_is_schema_bounded_and_retires_one_entry_per_step)
test(an_instance_pick_selects_the_topology_target_at_its_declared_granularity)
test(an_instance_hover_reports_the_resolved_granularity_on_the_pointer_channel)
test(a_marquee_release_replaces_with_the_deduplicated_topology_targets)
test(admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack)
```

The first four belong to the World target. The fixed-slot law belongs to the renderer target.

## Full World seam receipt

Root's full World receipt `04b955e0-536c-40c1-b891-9e1deb5ad4dd` ran 239 tests: 237 passed and two failed, with 243 outside the filter. Both failures were stale expectations after the carrier repair:

- the committed scene bridge explicitly publishes render instance `extrude@solid#0` with topology `interactionId: extrude@solid`; the pick and hover laws now assert the retained topology id while keeping the physical-hit assertion on the render id;
- the standalone marquee fixture uses `World3dState::new`, whose scene default granularity is `item`; the law now compares the emitted target with `resolved_domain_granularity_id(&state)` instead of hardcoding the retired `object` value.

These changes only update fixture expectations to the already-validated scene carrier and shared React semantics. `rustfmt --emit stdout` parses the changed World law source. The root-owned World rerun remains pending.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs`
- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs`
## Full World receipt

Root's source-current full World rerun is green: Nextest `e9710256-55e5-4fde-8064-28a96c76e80d` ran 239 selected laws with 239 passed, zero failed, and 243 outside the filter in 776 ms; Nx completed in 29.2 seconds. The retained per-instance interaction granularity, exact pair deduplication, full heap census, and the two corrected fixture seams therefore pass together. The receipt is `🗑️generated/astra-runtime/world-granularity-full2/run.log`.
