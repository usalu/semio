# Settled Extrusion and Registration Audit

Read-only audit against the six exact paths in `🔁️2026-09-30-retained-extrusion-ownership.md`. No source/test edits, compilation jobs, Git mutations, or new runtime tests. App276, original slider174 integration and final root aggregate are separate pending proofs.

## Actionable Report Issue

At audit time two of the six report SHA256 values were stale. Current retained fixture hash is `66e7c2af229fabae8d5b21efc13e180801176a648c550a17aad8b217aad2441f`; current Session script hash is `39fd549e63c21ce57b250978cb0f6af0cff2d5a06a3a729656efc3abe611222d`. Other four reported values match source. Execution owner was notified to refresh these after its registration/schema finalization. This is source provenance drift, not an identified production regression.

No additional actionable implementation defect identified in settled extrusion scope.

## Production Ownership

Concrete BREP engine `.../🧊️brep/🧬️schema/⚙️engine/🦀️.rs:933–940` identity-copies wire topology before planar-face construction. `extrude_wire_sync:985–996` copies wire topology, builds its raw FaceId and prism directly, and registers only final solid. `extrude_sync:999–1004` identity-copies the source face before destructive extrusion. No registered scratch face/wire is introduced, public API alias added, geometry authority substituted, Session reset or cache disable performed.

Existing identity transform helpers deep-copy topology, geometric supports, face inner loops and pcurves; source arena entries and labels remain unchanged. Copied labels are fresh/generated, not explicit source→copy historical correspondence. Raw copied allocations remain owned by the same Body family. Existing retain/dispose compaction and terminal frontier retirement continue to own their cleanup. Source/reference semantics are stronger evidence than simply obtaining plausible mesh volumes.

## Regression and Third-party Coverage

The new native law at `.../🧊️brep/🧪️tests/📦️extrude-orientation/🦀️.rs:299–339` uses retained rectangle-wire, hexagon-wire and rectangle-face profiles across11distances. It asserts actual `validate_sync` success for every repeated output (the corrected check that caught RED2), closed oriented soup edge invariants, signed volume versus declared closed form, independent Parry3d integration, exact unchanged earlier output meshes, one registered profile after output retirement, and zero registered handles after empty retain. Retain2214–2216 runs existing reachable topology compaction; registry zero alone is not an independently inspected terminal payload receipt.

AJV twin validates all three cases and two hostile cases and independently recomputes rectangle/polygon areas. Its8counted assertions are not equivalent to all native topology assertions; both comparisons matter. All13orientation laws include the new repeated-retention law, independently of original composition356/app276.

## Correct Execution Registration

Session owning `.../🌊️session/📦️packages/🦀️rust/📜️script.ts:17–22` unconditionally runs existing Session native groups, AJV twin, and exact BREP law through `runExactCargoLaws` with package `semio-s-artifact-stdio-semio`, target kind `test`, target name `brep_extrude_orientation`, and exact function `retained_profiles_survive_every_repeated_extrusion_without_topology_aliases`. Cargo manifest119–120 mounts that test binary to the actual integration source. It then rebuilds Session WASM and runs browser laws. Unknown canonical arguments fail; no oracle-only or empty-success path is present in this owner.

Session project canonical target is `cache:false` and calls its permanent script. Root script8959–8963 discovers owner contributions using `nx run-many -t canonical-architecture --all --exclude workspace --skip-nx-cache`. Static wiring therefore continues to include Session; actual final root discovery/completion remains its own runtime receipt, not claimed here.

## Actual Receipt Available

Read existing `🗑️generated/retained-extrusion-green2.log`: uncached Nx test succeeded,13tests/13passed/0skipped, and the exact new retained-profile law passed1/1 in0.02s. The log retains Nextest artifacts at `retained-extrusion-green2/semio-nextest-vSLz9B`. This is actual settled artifact-suite evidence, not a fresh run by this reviewer and not proof that Session canonical or complete original app/root aggregate has finished. Earlier misleading green-named RED receipt is excluded.
