# Receiving Outcome Port Map

Read-only current source map; no edits or compiler jobs.

## Implementors confirmed

1. Store 📦️codec/📸️native-snapshot/🫴️receiving/🦀️.rs:119 blanket OriginalArtifactSnapshotReceiving<T> for OriginalSnapshotRecordReceiving<T>. Trait step currently returns Result<RetainedCloneProgress,ValueError>; adapter selects inherent step.result.map_err(flat). This discards actual work. flat selects primary.or(quote_refusal).or(allocation_refusal), abandoning other distinct failure owners.
2. IFC4 ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/owned.rs:95 IfcOriginalPackReceiving. step:111 is handwritten. Phase1 takes record.step(...).result and repeats primary.or(quote_refusal).or(allocation_refusal). It independently loses work and causes; updating only the blanket adapter is insufficient.

Scoped source search confirmed these two actual implementations. IFC owned-tests.rs imports the trait at :89 and :162 and exercises custom receiver calls.

## Required direct signature

Change OriginalArtifactSnapshotReceiving::step to the same actual type as the existing inherent port:
RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary>.

Blanket implementation should delegate the complete inherent outcome directly; remove flat only once no remaining callers require it. Do not reconstruct work from copied_items: an attempted semantic action can have work without copied items.

IFC must retain child outcome intact, wrap its own ValueError into OriginalReceivingPrimary without selecting one cause, and report actual work for admission/take/graph actions. Its graph.step currently returns Result too, so a truthful child work port is needed there rather than a guessed constant getter. Keep refusal-owned fields alive through paid closure.

## Propagation boundary

Store receiving/📜️pair/🦀️.rs:73 phase1 is the direct trait-based consumer: cursor.step(...).map_err(Into::into). Pair step itself currently returns Result<progress,OriginalArtifactPairFailure>. Its cause enum holds Value or History only, with no OriginalReceivingPrimary variant. Therefore add complete native cause custody and propagate actual work through pair step, including error outcomes; otherwise the new trait merely moves the loss outward.

Plugin 🪟️window/🎚️config/📐️owner-contract-and-mutation/🛬️receiving/🦀️.rs:8 WindowConfigNativePairReceiving::step repeats the pair Result signature, and its implementation delegates the pair. Port this erased boundary and the actual window pair load caller together. Existing snapshot close/take ports are distinct; do not pretend successful step transfers or closes their owners.

Schema-first regression should force one child attempt with work>0 but zero copied_items, plus a failure holding primary, quote_refusal and allocation_refusal simultaneously. Assert all owners survive trait→pair→window forwarding and close under the same original recipient, not only the first formatted diagnostic.
