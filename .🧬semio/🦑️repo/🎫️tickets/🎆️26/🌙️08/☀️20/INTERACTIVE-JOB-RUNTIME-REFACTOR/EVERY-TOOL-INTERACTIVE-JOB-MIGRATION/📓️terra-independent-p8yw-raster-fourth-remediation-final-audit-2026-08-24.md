# P8yw Raster Fourth Remediation Independent Final Audit

## Verdict

**RED — not accepted.** The fourth remediation fixes the prior empty-bounds and semantic-fuel source defects, and the structural verifier recognizes those repairs. It still fails the required exact-owner/saturation contract in a concrete live source path: a saturated standalone control allocation panics while its input is in `ManuallyDrop`, losing the exact owner. In addition, the public populated `RasterOwnedMap` DSL materializer still performs an uncredited whole-map clone/materialization loop. No production source or verifier was changed in this audit.

## Scope and Prior Evidence Read

This audit read the root `AGENTS.md`, the prior final audit `📓️terra-independent-p8yw-raster-third-remediation-final-audit-2026-08-23.md`, and the implementation handback `📓️p8yw-raster-retained-envelope-ingress-2026-08-23.md`, then inspected the current working-tree diff and current Raster sources. The relevant current diff is eight Raster Rust files, including the retained binary codec and the owned-map implementation.

## Blocking Findings

### 1. Standalone control saturation panics and loses the exact input owner

`RasterOwnedRetirement::new` wraps its supplied owner in `ManuallyDrop` and then calls `RasterStandaloneControlCredit::try_claim().expect(...)` before transferring that owner into the cursor. At process saturation, `try_claim` returns `Err("raster-store.standalone-control-capacity")`; `expect` panics while the `ManuallyDrop<RasterRetirementOwner>` cannot drop or enter a bounded return path.

- `.../🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:199-213` creates the `ManuallyDrop` owner, panics at line 202, and takes it only at line 203.
- The same loss exists in `RasterSnapshotRetirementFactory::retire`: `ManuallyDrop<Arc<RasterSnapshot>>` is created at line 802, the saturation panic is line 803, and the take is only line 804.
- Both factories are reachable by retained close routes: `retire_owned` allocates at lines 725-728 and 811-814, and the initializer creates a previous-snapshot retirement at lines 3732-3737.

This violates the required max/+1, fault/panic, exact-producer-identity, and lost-handle conditions. A 833rd concurrent standalone retained authority (`RASTER_NON_STACK_CONTROL_BACKINGS * ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY = 13 * 64 = 832`) does not yield a retained fault/rejection with the original owner; it panics before the owner is handed back. The control counters are consequently not a real one-owner reservation/return proof at this seam.

The mounted initializer's separate 13-credit reservation does not cure this: it is claimed at lines 3597-3606 and released one arbitrary credit per turn at 3517-3524 / 3873-3889, while each concrete standalone/Arc retirement claims a different `RASTER_STANDALONE_PROCESS_CONTROLS` credit. No capability ties a specific reserved initialization slot to a concrete Box or Arc control.

Required repair: make construction fallible and retain/return the exact owner on saturation; do not use `expect` between `ManuallyDrop` capture and transfer. Thread a typed claimed credit into every concrete Box/Arc control owner and add max/max+1 tests for both `retire_owned` and `SnapshotRetirementFactory::retire`, covering fault, panic containment, and terminal-empty counters.

### 2. Populated `RasterOwnedMap` DSL materialization is still a public, uncredited loop

The claimed fail-closed map materialization is incomplete. Although populated `Clone`, serde decode, and DSL `from_value` refuse owners, public `DslField::to_value` allocates a vector and loops over every entry, cloning the key and recursively materializing the value:

```rust
let mut entries = Vec::with_capacity(self.length);
for (key, value) in self {
    entries.push((key.clone(), value.to_value()));
}
```

Exact evidence is `.../🗿️artifacts/🖨️raster/🦀️component.rs:347-366`, especially lines 352-357. A populated 64-entry map therefore crosses a public ordinary DSL route with no retained page cursor, no semantic fuel, no process/map-page credit, and potentially recursive `DslValue` materialization. This is the explicitly requested populated Clone/serde/**Dsl materialization** fail-closed/no-uncredited-loop property, so it independently blocks GREEN.

Required repair: make populated `to_value` fail closed or replace this public route with a retained, page-credited materialization authority. Add a hostile populated-map `to_value` fixture and verifier mutation that reintroduces the loop/key clone/value materialization.

## Evidence for Requested Properties

| Property | Independent result | Exact evidence |
|---|---|---|
| Non-stack control reservation/return | **RED** | Standalone and Arc seams panic before exact handoff at retained codec lines 199-203 and 800-805. `held_items`/`held_bytes` exist only on the separately claimed standalone credit (48-80), while initializer `remaining` credits (83-121) are not tied to the later concrete owners. |
| Retirement stack page CAS/allocate/return | Structurally **PASS**, execution unrun | Claim CAS at 216-230; allocation only after held pending credit at 624-642; empty-page return in the same 4,096-byte report at 609-617. The permanent test is at 4355-4376. This does not repair blocker 1. |
| Empty bounds and mounted 64-fuel second page | Structurally **PASS**, execution unrun | Payload/control totals are separated at 1080-1175; fuel consumes one unit at 1063-1069. The fixture has 64 fuel, nine assets (second page), candidate close, and process-zero assertion at 4146-4208. |
| Owned map ordinary Drop, pair handback, clone/serde/DSL | **RED** | Drop refuses populated maps at owned-map lines 250-254; removal returns a guarded pair at 212-220; Clone and serde decode refuse populated maps at 257-261 and 327-344. But populated `to_value` remains uncredited at 352-357. |
| Saturation, max/+1, cancellation/fault/panic, identity/freshness | **RED** | The max/+1 standalone path is a panic/lost `ManuallyDrop` owner (blocker 1). Existing tests cover map capacity and ordinary-drop refusal, not saturated `RasterOwnedRetirement::new` or `RasterSnapshotRetirementFactory::retire`. Operation/generation checks exist at 3549-3555, but cannot compensate for an owner lost before a retained authority exists. |
| Permanent verifier fixtures and hostile mutations | **RED** | The verifier self-test passes, but `toolJobRasterEnvelopeCallerRetainedExact` only checks textual presence for the claimed seams (`📜️script.ts:1826-1874`). It does not require a saturated-standalone/Arc fixture nor reject the owned-map `to_value` loop. Thus it accepts both remaining counterexamples. |

## Positive Checks Preserved

- `RasterOwnedMap` ordinary populated Drop is fail-closed; pair removal has an exact take-once wrapper (`.../🗿️artifacts/🖨️raster/🦀️component.rs:50-72, 212-254`).
- Stack page state uses checked CAS claim, allocation after claim, and return-on-empty-page transitions (`.../💾️binary/🦀️component.rs:216-243, 605-652`).
- Fixed control bytes are separated from payload preflight totals (`.../💾️binary/🦀️component.rs:1080-1182`), resolving the prior impossible empty-bounds arithmetic at source level.
- `raster_reserve_unit` uses semantic fuel `1`, not byte-valued fuel (`.../💾️binary/🦀️component.rs:1063-1069`).

## Gates Run

| Gate | Result |
|---|---|
| Scoped eight-file `rustfmt --check --edition 2021` | **PASS** |
| Scoped Raster `git diff --check` | **PASS** |
| Permanent verifier self-test | **PASS** — `self-tests=328 clean` |
| Live `verify interactivity tool-jobs --format json` | Expected global **RED** — 884 remaining commands and unrelated global failure classes; no emitted Raster-specific predicate failure. It cannot validate the two blockers above. |
| Cargo, Nx, Wasm, browser, runtime, network, broad builds | **Not run by instruction** |

## Re-audit Conditions

1. Replace saturating `expect` constructors with an exact-owner retained rejection/close route for both standalone and Arc-root factories; bind each physical control allocation to one typed credit and prove held/returned counters at terminal-empty.
2. Remove or retain-cursorize populated `RasterOwnedMap::to_value`.
3. Extend permanent verifier self-tests and live predicates with hostile mutations for each repair, then rerun the scoped formatter, self-test, and live source check. Rust execution remains a later serialized gate.

