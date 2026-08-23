# P8yw Raster Second Remediation Re-audit — 2026-08-23

## Verdict

RED. The second repair closes the five originally named mechanisms at a superficial source level,
but the replacement still lacks an exact allocation authority and its fixed retirement stack is
not large enough for the admitted mixed layer-plus-DSL shape. No Cargo, Nx, Wasm, browser, runtime,
or network gate was run because overlapping Rust source packets remain active.

## Evidence Preserved

- Raster remains absent from the raw whole-buffer caller census; the structural count is one shared
  definition plus eleven live callers.
- Whole recursive serde encode/clone and monolithic diff/apply calls are gone from the retained
  initializer.
- Typed mutation digest/candidate cursors and a completed-store disposer are present.
- `String`/`Vec` retirement uses observed capacity, and recursive owner traversal is iterative.
- The public initializer and its Raster implementation fail closed on premature Drop.

Those improvements remain useful and must not be reverted.

## Blocking Findings

### 1. Standard-map backing allocation is neither admitted nor retired exactly

The bounds authorities count a `BTreeMap` only as `size_of_val(&map)` plus semantic key/value
owners. They do not account the map's allocated nodes or control backing. Candidate construction
then calls `BTreeMap::insert` after reserving fuel derived from the key length, with no owned fixed
node/page authority and no exact allocation credit. The relevant snapshot path is
`component.rs:1563-1595` and `:1696-1723`; Adjustment parameters use the same standard-map
construction at `:1286` and the parameter insert path.

Retirement calls `pop_last` on populated asset and parameter maps (`:168-174`, `:219-224`). A
successful removal may merge or free implementation-owned nodes during a step reported as a
zero-byte `Push`. The empty-map fixture proves only that a newly empty shell has no allocation; it
does not prove that the preceding `pop_last` released no backing.

This violates both the exact byte ledger and the rule forbidding reliance on external
implementation details. Replace these retained maps with an owned fixed/page authority, or define
an owned explicit semantic-entry plus conservative fixed backing contract whose allocation and
one-grant retirement are independently proved. Add cap/+1 and formerly-populated-map retirement
mutations; an empty fresh map is not a discriminator.

### 2. Requested capacities are treated as actual candidate allocation capacities

`raster_clone_owned_string` preflights the source capacity, calls `try_reserve_exact`, and then
returns the new string without checking or admitting the allocation's observed capacity. Layer and
DSL skeletons similarly use `Vec::with_capacity`/`try_reserve_exact` while the simultaneous
candidate totals were calculated from the requested source capacity (`:750-810`, `:1100-1110`,
`:1254-1287`, `:1641-1676`). Rust promises at least the requested capacity, not byte-for-byte
allocator equality. A retained candidate may therefore own more bytes than its preflight ledger,
even though retirement later truthfully observes the larger capacity.

Use fixed-page owned containers, an owned allocator contract, or an allocate-inspect-admit protocol
that retains and incrementally retires the exact rejected allocation. Add a discriminator whose
allocator returns a larger capacity than requested; comparing ordinary source/candidate values is
not an ownership proof.

### 3. Box and Arc allocation backings disappear outside the reported byte ledger

CreateLayer retirement moves `*layer` out of `Box<RasterLayerNode>` and deallocates the Box backing
while returning a zero-byte `Push` (`:51-55` and the Create branch in `frame_action`). Clone and
retirement authorities also allocate multiple Box control owners without an explicit fixed backing
credit. `RasterSnapshotRootRetirement` uses `Arc::try_unwrap` and reports one item/zero bytes when
the Arc allocation/control backing can be released (`:449-495`). These are real allocation owners,
not scalar shells.

Move these behind owned fixed control arenas or add explicit conservative fixed backing credits and
matching close evidence. Verifier mutations must reject unreported Box/Arc deallocation and
cap-plus-one control-owner creation.

### 4. The iterative stack does not cover the admitted combined depth

The fixed stack capacity is `RASTER_MAXIMUM_NESTED_DEPTH * 2 + 8` (`:25-26`). Layer bounds and DSL
bounds each independently admit depths up to the same 128 limit. A maximally nested Group chain can
end in an Adjustment whose parameter is a maximally nested object/array chain. Retirement retains
the parent layer frames while the DSL subtree is active; object entries add both `ValueEntry` and
`Value` frames. The required simultaneous depth is therefore the retained layer depth plus roughly
twice the DSL depth plus wrappers, which can exceed 264.

When this occurs, `advance` returns an error at the stack push (`:404-410`) after the owner was
already admitted. Terminal retirement then cannot reach empty, so the fail-closed Drop assertion
turns this into a lifecycle failure rather than a safe rejection.

Either enforce one combined admitted depth budget shared by layer and DSL traversal or size/prove
the fixed stack from the exact combined frame formula. Add a hostile Group-to-Adjustment-to-object
fixture at maximum and maximum-plus-one combined depth. Separate deep-layer and deep-value fixtures
do not discriminate this failure.

## Acceptance Conditions

Repair all four findings without reintroducing whole recursive work. Extend the permanent verifier
for populated map backing, allocator over-capacity, Box/Arc control backing, and combined-depth
failure. Then rerun the narrow source gates and return the packet for another independent audit.
The eventual serialized native/Wasm/browser matrix remains mandatory even after source acceptance.
