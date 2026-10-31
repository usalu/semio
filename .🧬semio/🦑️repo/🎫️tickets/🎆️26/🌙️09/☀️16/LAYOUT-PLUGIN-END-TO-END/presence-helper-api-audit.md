# Shared Presence Helper API Audit

Read-only bounded follow-up to Root's actual Layout15 original74693 terminal compiler failure (outer EXIT1, inner101); no test binary ran. Root/Rust own diagnosis and actual verification. Rust confirms the field reads were already present at his first fresh read; this audit does not attribute their authorship. This audit changed no Source, schema, policy, runtime, cache or prepared setup input. Setup7 remains unlaunched.

## Bounded Scope And Findings

Read all nine current Rust files under the shared plugin artifact-app-laws helper tree, including the untracked presence helper that read-only Git grep alone does not enumerate. Followed the concrete PeerPresenceRoot declaration, roster storage field, and current peer publication owner only; no repository-wide census.

| Actual definition or use | Finding |
| --- | --- |
| plugin/📜️declaration/👁️view/🫂️peer-presence/🦀️.rs:36–39 | PeerPresenceRoot has a fixed64-slot entries array and public len:usize representing occupied peers. Fixed backing length is not logical cardinality. |
| Same declaration:91–98 | len() is available only under the defining crate's cfg(test); is_empty() is public. A consuming testkit helper compiled as a dependency cannot rely on the defining crate's cfg(test) method. Do not add a compatibility method or change the peer owner's API to accommodate the caller. |
| plugin/🧵️instance/🏪️stores/📥️staging/🧱️components/🦀️.rs:140 | The concrete roster retains ManuallyDrop<Arc<PeerPresenceRoot>>; reads of its public len field preserve the existing actual root. No replacement collection or clone is required. |
| artifact-app-laws/👥️presence-round-trip-law/🦀️.rs:39,44 | Exactly two helper assertions observe live metadata cardinality. Current Source already reads peer.roster.peer_presence.len, as already present at the first fresh read, expecting1 after the one-peer roster and0 after departure. These are the only stale-method candidates found; no additional patch is proposed. |
| Same helper:7–12 | peers.len() is a slice length; pages.len() is a Vec length used by the existing cursor. They are not PeerPresenceRoot method calls and should remain unchanged. |
| Same helper:33,36–38,43 | presence_store.peers_root() is the separate app-typed presence root. Its public is_empty()/peers() observations remain valid; do not conflate that namespace with live peer metadata. |

The helper publishes exactly one PresencePeer and then an empty roster, so expected cardinalities1→0 remain coherent with the actual64-slot metadata root. The actor must satisfy the existing nonempty/256-byte identity predicate in the real owner; duplicate and capacity faults remain enforced by that owner. No slot, byte, page, deadline or turn allowance needs increasing. The typed root guard is explicitly dropped before departure, and document-pack equality assertions remain in place.

All other matching roster mentions within the nine-file helper tree concern declaration/member rosters or reexports, not this PeerPresenceRoot cardinality API. There is no further same-type .len() call to repair in this bounded scope.

## Candidate Disposition And Evidence Limit

The concrete candidate set is exactly the two already-corrected field reads above. Preserve the ongoing peer type refactor and Rust's source ownership; no legacy method, alias or broader cleanup is justified. This is Source/API review only and does not establish compilation, test execution, full Layout15 success or UI behavior. The canonical full Layout15 remains mandatory.

The generated metadata receipt records current helper/typedef hashes and bounded file count without copying Source into the ticket.

Attribution correction: the live helper SHA68520ee8644a5ffd0ad1e4a268b52abc795dc4d5edc9cf9518f8ab4ad3f9f1c0 already contained field reads when both readers first inspected it. Rust made no Source edit. The failed Native captured older method-call bytes; preserve that distinction and the peer-authored current hunk. No broader unselected SDK contract test was inspected or changed by this audit.
