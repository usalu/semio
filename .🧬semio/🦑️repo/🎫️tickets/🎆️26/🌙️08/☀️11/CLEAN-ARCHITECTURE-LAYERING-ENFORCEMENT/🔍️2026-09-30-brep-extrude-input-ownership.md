# BREP Extrusion Cached Input Ownership

Read-only source audit of existing concrete BREP transform, extrusion and retirement behavior. No native job started, implementation/test edit, Git mutation or runtime pass claimed. Agent-owned native regression, app174/fresh-oracle comparison, app gate and root proof remain pending.

All paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/`.

## Conclusion

Operation-local identity deep copies are the appropriate boundary before passing a cached profile to the destructive prism builder. `transform_face` and `transform_wire` allocate independent reachable topology inside the same concrete BREP owner; they do not clear Session state, replace geometry authority, or reset cache. They preserve source geometry/topology entries and original persistent labels. They mint new generated labels for the copied entities rather than preserving the same labels or recording an explicit old→new lineage relation. Do not describe this as stronger historical correspondence than the actual recorder provides.

## Exact Source Evidence

- `⚙️engine/🦀️.rs:983–994` currently obtains a planar face from a cached wire and passes the obtained face ID directly to `extrude_face`; direct face extrusion passes `face_id(face)` directly. `planar_face_from_wire_sync:933–939` only clones the Wire vectors, leaving their arena IDs shared.
- `🔺️diff/➡️sweep/🦀️.rs:45–53` explicitly treats supplied profile as the bottom solid cap. `🔺️diff/➡️sweep/🧮️core/🦀️.rs:217–241` changes the supplied bottom face `flipped` flag in place when normal/travel require it, records that source label modified, then transforms it for the top. Consequently a cached profile is not a read-only value at this call boundary.
- `🔺️diff/🔁️transform/🦀️.rs:221–237` starts a fresh CopyCtx per transform. Face copy165–178 clones surface, outer loop and every inner loop/hole, preserves flipped under identity, and patches only the newly allocated loops to the newly allocated face.
- Copy loop139–157 carries original ring order, coedge direction and parameter ranges; pcurves87–94 are cloned without resampling. Edge120–130 retains curve range and endpoint incidence. CopyCtx64–70 preserves shared edge/vertex/curve references consistently within the copied graph. Source arena entries are only read/cloned; mutations target fresh IDs.
- Wire copy232–237 clones all member edges and endpoint vertices with the same remap, preserving member forward flags, vertex order and closedness. Wires do not own face pcurves; those are retained by the face-copy path.
- Euler `🔺️diff/🔺️euler/🦀️.rs:44,52,77,85,93` records fresh generated entity labels. No original-source deletion or modification occurs in the identity copy. Copying a face is not tessellation reconstruction, so analytic/NURBS curve and surface supports and holes survive the copy.

## Ownership and Retirement Implications

Copies live in the existing Body arenas; the returned solid reaches copied bottom/top/lateral topology. Use raw copied FaceId/Wire inside the operation rather than registering a scratch geometry handle merely to pass it onward. An extra live scratch handle would retain its profile until disposal/terminal closure. Existing `extrude_wire_sync` already registers an intermediate planar-face handle; the repair should avoid introducing another unnecessary registry lifetime.

`⚙️engine/🦀️.rs:1749–1755` removes live handles and synchronously compacts unreachable arenas on disposal. Terminal `⚙️engine/🧹️retirement/🦀️.rs:78–85` detaches all Body arenas and live handles into explicit payload/frontier retirement, including face inner-loop vectors and curved geometric supports. Operation-local copied allocations remain owned by that same concrete BREP family and follow these existing cleanup paths. A failed construction may leave unreachable arena allocations until existing compaction/terminal retirement; do not claim atomic rollback that the implementation does not provide.

The transform copy loops and prism construction are synchronous and proportional to topology. This audit establishes source immutability/ownership, not bounded latency, cancellation responsiveness or a deadline. No invented timing limit or cache reset is needed for the ownership correction.

## Concrete Regression Expectations

The execution agent's native law should compare source face flipped state, original curve/vertex IDs and repeated downstream results before/after multiple slider extrusions against fresh evaluation. At least one face-with-hole/curved-support case should verify inner-loop and pcurve preservation if not already covered by existing transform laws. Preserve cached upstream authority; a Session reset would conceal rather than fix aliasing. Track copied output disposal and terminal retirement with the existing owner path. Final acceptance still requires actual fresh native/app/root receipts.

Findings sent directly to execution agent `finish_incremental_composition`; no source edits were made by this audit.
