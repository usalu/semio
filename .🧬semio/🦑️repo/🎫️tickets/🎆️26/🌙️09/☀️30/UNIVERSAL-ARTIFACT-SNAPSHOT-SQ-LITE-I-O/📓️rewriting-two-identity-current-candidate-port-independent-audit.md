# Current Two-Identity Candidate Port Audit

Read-only review of `📥️inputs/rewriting-two-identity-owned-composition-pairs.json` (PreparedUnMounted, five full-file pairs). No production writes or owning tests.

The proposed dictionary hash/search/deletion repair consistently uses `(owner.slot, owner.child_id)` logical membership; OwnerRef stays logical. Genesis/history/open ArtifactRef and envelope document IDs use declared actual target. Public restore validation retains the full slot + logical child ID + target ID + kind/standard/subset triple. Closure probing uses target identity and still checks OwnerRef.parent/slot/logical child ID against the complete declared child fields. Pins/cascade locate actual targets then return logical dictionary keys. No global logical-to-target substitution was found in the inspected changed regions.

## Concrete remaining duplicate-endpoint gap

Current Store root lines 3646–3661 admit a many-slot projection with two distinct logical child IDs referencing the same actual target: duplicate checking only compares `prior.child_id`; singular slots reject repeats but many slots do not. Candidate Plugin `admit_child_member` resolves each logical key then calls `CompositionGraph::admit_owns` with target ID. Current Store lines 26899–26907 treats an already-owned target under the same parent and slot as idempotent success. The candidate logical member dictionary can therefore admit both logical entries for that one physical document, while candidate `CompositionGraph::sync_member` explicitly rejects duplicate targets later. Refuse the duplicate actual target before publishing the second member, or otherwise establish one coherent endpoint uniqueness rule across projection/admission/sync. Preserve legal distinct logical and target values.

This is a concrete candidate source invariant gap, not an executed failure. Global typed read/frame and ChildContentEntry target authority remain separate unmounted work. Existing control-character restrictions in closure/open remain their own preexisting domain boundary, outside this identity patch.

## Current before anchors

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`: full before matches current = `true`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: full before matches current = `true`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🦀️.rs`: full before matches current = `true`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs`: full before matches current = `true`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: full before matches current = `true`.

## Fourteen-path refresh

Fresh candidate now includes duplicate actual target refusal in ChildRestoreProjection before member registration. It checks prior.child_id OR prior.artifact_id, aligning projection/admission with graph sync. The reported many-slot duplicate-target hole is therefore closed in the held candidate, not yet runtime proven. The added neutral projection case has two distinct logical keys sharing one actual target and requires refusal; the independent AJV uniqueness oracle checks logical IDs and actual targets separately.

ToolRun child_id and TimeTravel member-store keys correctly retain owner.child_id. The child-frame typed read now uses the same hand-authored contract.childId as the logical parent/member key, while its actual envelope target stays contract.target.artifactId. No identity equalization is proposed. MemberOpen neutral fixture replaces the former false unequal-owner negative with an explicit empty logical-owner negative and adds a valid distinct logical/target case; all nine other previous cases are semantically unchanged. Independent structural oracle checks nonempty/control-free owner keys rather than equality to document ID. Persisted history owner mismatch laws stay separate. No fixture outputs were computed from production.

Archive closure Source consumer still awaits High's coherent full-parent-projection pairing at this readback; no verdict is inferred for that unfinished port. Global typed capture/read/frame remains unmounted and separate.

## Fifteenth archive Source port

Reviewed the new declared ArchiveMembership rows against the actual two-member channel archive fixture: root-1/children/child-1→child-1 and child-1/nested/grandchild-1→grandchild-1 match exact current authored identities. The Source oracle now separately admits a distinct logical key with its independently authored declared relationship and rejects that same archive against the original declaration. Every prior invalid ordinal/owner child ID/owner parent mutation remains refused against declared membership; graphlib retains actual-coordinate cycle and reachability checks. This Source helper covers identity/topology, not full dialect/schema matching; current candidate Rust closure separately checks full dialect and OwnerRef triple. No new blocker found in this exact changed port. Global frame/read remains separate.


## Eighteen-Case TypeScript Closure Readback

Read the complete proposed closure producer and independent Graphlib consumer in the current eighteen-path capsule. Parsed the before/after neutral fixture as JSON: all original sixteen case objects are exactly unchanged; the two new cases separately demand accepted distinct logical key/address and refusal of two logical keys owning one addressed target. This was a read-only fixture comparison, not Source execution.

The proposed producer indexes addressed `member.reference.artifactId` (lines 55–58), retrieves by `edge.target.artifactId` (70), and retains exact full addressed reference, parent reference, slot and logical `owner.childId` matching (73). Its new duplicate scan (69) rejects either duplicate logical identity or duplicate addressed target within a parent. The scan remains bounded by the preceding 64-child bound (66). The seen-set rejects repeated addressed members across parents (74). No blanket target-to-logical replacement is present in this inspected port.

The independent consumer still uses Graphlib addressed-coordinate cycle and complete reachability checks (17–25). Its declared/actual relation tuples now compare `[slot, logical childId, full target reference]` against `[owner.slot, owner.childId, full member.reference]` (27–31). Thus accepting the new legal distinct case does not relax full ownership or dialect equality. The two new examples are authored inputs rather than producer-derived expectations. The strict canonical schema validator join is explicit (4,60).

No concrete new blocker was found in these three added paths. Heap Maps/Sets and mutable continuation inputs retain their existing scope; this inspection makes no full allocator or generic immutable-capture claim. High reports the unchanged Source71 baseline reached 70 pass/1 failure at the distinct case before production activation. Production identity ports and global typed child/read/frame authority remain separate runtime gates.
