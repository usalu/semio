# PDF Shared Actor Compiler Frontier Readback

Original whole-PDF replay26385 ended exit1 before Nextest on ten OSKernel compiler errors caused by the current explicit ActorId API. The actual context file is generated physical-resumed-pdf-current-os-kernel-ten-compiler-contexts.json. Six ArtifactStore::new callers require an actor; two retained initialization callers require an actor; two retained finalization paths still pass persisted-author Option<String> to a typed ActorId setter. No API restoration, actor fabrication, or production edit was performed in this lane.

A later current-source readback found the six ArtifactStore callers already changing externally: member create/open forward their explicit caller actor, while replay/apply codec temporary stores use the existing LOCAL_ACTOR_ID authority. At that readback hydration and retained-config initialization/finalization still had four stale callers; their retained structs did not yet contain an explicit actor field. These are current actor-propagation work for the shared authority owner, not valid locations for this lane to infer a loader actor from persisted history authors. Root and Immutable were notified.

Prepared original PDF, VDI, five remaining Norm, and CSV/JSON Native commands are retained serially. Their shared compiler prerequisite must join before runtime qualification can be credited.
