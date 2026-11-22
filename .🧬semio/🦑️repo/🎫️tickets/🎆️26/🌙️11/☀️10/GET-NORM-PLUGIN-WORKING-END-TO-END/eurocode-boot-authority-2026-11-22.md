# Eurocode boot `native-return-refused: authority`

## Symptom

`norm-eurocode` (rebuilt 2026-11-22 01:55 from the current tree) failed on every route (EN 1990/1991/1997/1998) with `Framework OS boot failed Error: original-turn.native-return-refused: authority` and the shell showed "No plugins loaded".

## Root cause

Not an authority, ownership or return-ledger accounting fault. The `authority` tag is the WIT `actor-call-failure::authority(invalid-value)` of the very first `reactor.poll`, which carried the `instance-open` event and spent exactly zero items, bytes, capacity, release and Work.

Instrumenting the published bridge (`probe-boot-refusal.mjs`, route-patching `throw nativeRefusalError(failure) ?? failure;`) captured:

- failure `{tag: authority, val: invalid-value}`, `workUnitsSpent 0`, all four physical axes 0, epoch 13, full grant unspent
- the event payload keys `instance, activationGeneration, requestSequence, appId, actor, config, assets, capabilities, quotas`, with no `purpose`

The guest lift refuses it: `event_from_value` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧬️wit-bridge/🎟️event-lift/🦀️.rs` (`"instance-open"` arm) now reads `field(val, "purpose")?` and returns `ValueRefusalKind::InvalidValue` through `outside()`. `poll_wit` (`⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`) maps a lift error to `failure(input, spent, 0, Some(error.kind))`, i.e. the authority failure with zero spend; the bridge's `nativeRefusalError` prints only the tag, which hid the cause.

The natural-source rollout (all Rust files touched 2026-11-22 00:36) added `purpose: actor-instance-open-purpose` to `events.wit` `instance-open-event`, the Rust host codec (`🖥️host/🧬️component-codec/🎟️retained-turn/🦀️.rs`, `wire_open_purpose` emitting `{"tag":"ordinary"}`) and the guest lift. The TypeScript producers of `instance-open` (shard client lifecycle lease, worker-side document actor activation) were not updated, so the browser path never sent `purpose`. The four older components predate the stricter lift (they read named fields only) and are unaffected at runtime.

The 55-file list contained no accounting change involved; `reserved_birth`, ladders and reserved-mount are not on this path.

## Fix (schema-first, TypeScript producers complete the WIT record)

- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts`: `ShardNaturalSourceDeclaration` and `ShardInstanceOpenPurpose` mirror `actor-instance-open-purpose`.
- `…/🔐️activation/🟦️.ts`: `ShardInstanceOpenInput.purpose` is required (no default).
- `…/🚪️instance-lifecycle/🟦️.ts`: the `instance-open` payload forwards `purpose: input.purpose`.
- Callers pass `{ tag: "ordinary" }`: `PluginRuntime/🧩️module-loader`, `plugin-bridge/🔌️plugin-module-loader/🔌️plugin-handle`, store worker `🚀️guest-activation`.
- Tests and fixtures updated to the closed schema; the captured-instance-lifecycle test additionally asserts `events[0].payload.purpose` equals `{tag: "ordinary"}`.

No Rust guest change, hence no component rebuild is required: the already published `norm-eurocode` guest was built from the current Rust and boots with the corrected producer. The other four components keep their builds: the extra `purpose` field is ignored by their older lift, and a rebuild would only pick up the strict check, which the producers now satisfy.

## Verification

- Before: `probe-boot-refusal.mjs en1997` shows the refusal with the payload keys above.
- After (Play server restarted so the frozen Vite transform cache picks up the edited TS): same probe boots into "semio · Eurocode 7 / Compliant geotechnical package".
- Full check `probe-norm-base-companions.py en1990 en1998` on port 6037: both shells ready with Eurocode 0 and Eurocode 8 windows, EN 1998 acknowledged its initial archive (120/120). Both still exit 1 only because of an unrelated concurrent edit: `World3dHost` renders `<CatalogueDropGhost controllerId hostRef meshVisuals palette/>` while `🗂️catalogue-drop/🟦️.tsx` still exports the old `{preview, meshes, palette}` signature, throwing `Cannot read properties of undefined (reading 'meshUrl')` and removing the viewport (draw witness times out). That half-finished World3dHost refactor belongs to another agent.
- Unit: `bun test ./🧪️tests/🎭️actor-transport/🟦️.ts -t "captured instance lifecycle"`, 9 passed.

## Changed files

Producers: shard-client `📜️wire`, `🔐️activation`, `🚪️instance-lifecycle`; `PluginRuntime/🧩️module-loader`; wgpu `plugin-handle`; store worker `guest-activation`.
Tests: shardclient settlement kit and captured-instance-lifecycle, retryable-lifecycle-deadline, ticket-owned-browser-host-staging routes-and-bridge, UiDocumentStore resident-pool and typedwire-helpers, renderer turn-patch-batch, package-integration, ui-intake-admission, turn-dispatch-lifecycle.
Ticket: `probe-boot-refusal.mjs`, this note.
