# P8yw Raster Third Remediation Independent Final Audit

## Verdict

RED. The retained initializer's bounds preflight is unsatisfiable for every
snapshot, and the mounted production session cannot supply the byte-valued
fuel its initializer requires. Two owned-map/retirement surfaces additionally
still permit ordinary uncredited ownership destruction. No Cargo, Nx, Wasm,
browser, runtime, or network gate was run, per the concurrent-edit restriction.

## Confirmed Blockers

### 1. The control reserve makes every snapshot fail bounds preflight

`RasterSnapshotBoundsAuthority::step` first adds the nonzero
`size_of::<RasterSnapshot>()` shell to both ledgers, then calls
`fixed_control_backings` ([retained codec](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs):1798-1799).
That helper reserves `RASTER_MAXIMUM_CONTROL_BACKINGS *
RASTER_CONTROL_BACKING_BYTES`: `(51 + 13) * 4096 = 262144` bytes
([same file](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs):33-38,987-991).
`RASTER_MAXIMUM_NESTED_BYTES` is also 262144, and `RasterOwnerTotals::add`
rejects totals above it. The prior shell addition makes this call exceed the
limit before any source field is inspected. Therefore the live clone phase
faults with `raster-store.preflight-byte-capacity` even for an empty snapshot.

Repair the budget model: control credits must not be double-counted against a
content cap, or the aggregate admission cap must include the fixed control
reserve plus the snapshot/content allowance. Add and execute an empty-envelope
initializer success test as the discriminator.

### 2. Mounted replacement sessions can never provide required Raster fuel

`ActiveArtifactStoreReplacement::new` and `session_params` hard-code
`fuel_per_step: 64` ([plugin runtime](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs):15011-15028).
The Raster initializer uses that same `StepContext` but requires 4096 fuel
before each control allocation and 16384 fuel before a fixed map-page
allocation ([retained codec](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs):1637-1649,1980-2016).
`raster_reserve_unit` yields whenever remaining fuel is below the requested
units, so production can never progress through a layer control allocation or
a page allocation. This is a permanent liveness stall, independent of the
preflight failure above.

Separate semantic work fuel from byte admission credit. A bounded allocation
must consume a small fixed semantic unit after an independent byte-credit
check, and a mounted low-fuel test must drive a layer and a second map page to
completion.

### 3. `RasterOwnedMap` permits populated ordinary Drop and key destruction

`RasterOwnedMap` owns populated boxed pages but has no `Drop` guard; normal
destruction can release every page and all nested entries outside the one-page
retirement cursor ([owned map](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs):50-54,210-216).
Its public `remove` also calls `remove_entry` and discards the returned key,
silently dropping that exact owner ([same file](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️component.rs):176-189).
This violates the retained route's exact owner handback and forbids no
populated Drop guarantee. Keep map ownership behind a fail-closed terminal
authority, remove/privatize destructive APIs that discard keys, and require
the pair-returning route to hand both owners to bounded retirement.

### 4. Retirement-stack pages are allocated without a matching close-step credit

When the stack grows, `RasterOwnedRetirement::advance` verifies only that the
grant is at least 4096, allocates `Box<RasterRetirementFramePage>`, and returns
`Pending { released_items: 0, released_bytes: 0 }`
([retained codec](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs):495-503).
The subsequent frame push reports one item and zero bytes
([same file](../../../../../../../../✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs):509-513), while later page Drop reports 4096 bytes. The actual page backing is thus created on an unreported step and only retroactively treated as released. Make the allocation an explicit admitted ownership action with symmetric allocation/retirement credit and a fixture asserting exact ledger transitions.

## Other Checks

- The prior raw whole-buffer caller census remains structurally improved; this audit did not find a new Raster raw caller.
- The fixed-page `RasterOwnedMapPageBacking` release path and the completed-candidate disposer are present, but cannot compensate for the blockers above.
- No permanent verifier, formatter, or build command was run: this report is a source-only audit and the live Rust success path is already disproved by blocker 1.

## Required Re-audit Conditions

Repair all four blockers, extend the permanent verifier with mutations for the
impossible control aggregate, mounted low-fuel liveness, populated-map ordinary
destruction/key handback, and stack-page allocation credit, then execute the
focused Rust initializer fixture before requesting source acceptance.
