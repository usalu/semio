# Named Static Pack Producer: Owned Intake Viability

Read-only inspection; no production, manifest, native job, or driver change.

A shared implementation can avoid per-app command variants, but it cannot be dropped into the ordinary retained command worker with a different generic command. The clean existing boundary is a genuine framework-owned source producer followed by original emission, or a command-independent reserved producer using the same registration and publication gates. The producer still specializes its registration on the concrete ArtifactApp owner and validates the app-declared static catalog; specialization is ownership, not duplicated behavior.

## Current Command Coupling

All paths below are relative to framework OS plugin root: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin.

- 📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:364 declares Command as an app-associated closed type with OpBinary + RetireOwned + Send + Sync.
- 🧵️retained-command/🔖️checkpoint-and-work-contract/🦀️.rs:94,161–182: ArtifactCommandInputs contains &A::Command; ArtifactCommandWork receives mutable A::Command, may decode only A::Command, and its extent/step take that same command type.
- 🎮️command/🎟️typed/🦀️.rs:23,48–50: OriginalTypedCommandSource stores Box<A::Command>; capture_command requires both the actual typed owner and the sealed original wire. produce_action accepts ArtifactOriginalCommandProducerFactory<A::Command> and pins action/factory identity, original caller loan, command, and wire across retries.
- 📜️declaration/👷️job/🎮️command/🎯️action/🦀️.rs:8–22: both producer factory and producer are generic over the concrete C and hand off Box<C> together with retained wire.
- 📜️declaration/👷️job/🎟️request/🔒️command/🦀️.rs:7–29: ArtifactOriginalToolCommand<C> stores original source binding, exact wire byte count, page count, first-page identity, and boxed command. admitted_encoded_bytes refuses changed wire/source. A fabricated command or unrelated fresh wire is not an acceptable bridge.

A shared Source command type could become A::Command only through an explicit common app-command design. Existing arbitrary app commands cannot receive it by pretending that a framework worker is independent of A::Command.

## Existing Command-Independent Ports

- 📜️declaration/👷️job/🧵️reserved-tool-job/🦀️.rs:43–85: ArtifactReservedToolInput is Action { args, interaction, hover } or Media { port, media }. ArtifactReservedToolJobRequest<A> owns this input, same source binding, retained grant, exact parent revision/controller/tool/schema/contract, SnapshotRead, context, and completion. It has no A::Command field.
- 📜️declaration/👷️job/🎟️request/🏭️reserved/🦀️.rs:7–28: ArtifactReservedWork<A> steps against that request. admit_reserved_work consumes the original request only after checking original parent/source binding and actual grant; its persistent InteractiveJob revalidates the source each turn.
- 👷️job/🏛️reserved/🦀️.rs:190–230: existing framework reserved factories specialize on A, use ArtifactReservedToolJob payload/job, and declare exact DOCUMENT_SCHEMA, owner witness and publication lanes. They do not require an app command variant.
- 📜️declaration/👷️job/🧾️bounded-proof-and-factory-registry/🦀️.rs:76–154: registration enforces concrete Owner=A, exact controller/tool bijection, matching document schema, migrated classification, and complete lane contracts. These checks must remain for a shared source loader.

Thus a new framework source route can share one implementation and use per-app declarative catalog data, without nine custom command variants. It still needs its own real registered route/factory/source custody, not a borrowed registration for an unrelated verb.

## Original Emission Is Already the Correct Typed Result Boundary

- 👷️job/📨️emission/🎟️capture/🦀️.rs:12: OriginalEmissionInput<A> is verb + ActionMeta + Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>; no command field.
- Same file:52–59,73–93: admission/capture/handoff retain the actual caller source loan, source binding, decoder ledger/recipient and original input until funded handoff or paid closure. There is no requirement to invent an owned completion.
- 🧵️instance/📨️dispatch/📥️staging/📤️emit/🎟️original/🦀️.rs:14–69: dispatch_original_emission admits the actual lease, checks live instance, registered app/framework publication owner, allowed lanes and captured Store frontier, captures Window/child authority, and mounts only into the exact reserved slot. It subsequently closes the capture under original retirement.
- 🧵️retained-command/🔄️full-operation/📥️emission/🦀️.rs:9–25: direct emissions explicitly borrow original authoring authority; source + decoder + issued lease are validated. Mounted state has no owned completion or owned authoring identity.
- 📜️declaration/📨️emission/🦀️.rs:58–78: Emit already owns an effects vector; a LoadDocument effect does not require constructing an app mutation or invoking A::handle.
- 📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs:51–65: LoadDocument(pack,spr) is the established whole-document genesis path, with no mutation/history row and live identity stamping. The current helper cold-encodes pack and allocates vectors, so calling it outside a measured retained source step would not implement the required funded loader.

A static Pack producer can therefore borrow the same genuine initiating caller while copying static bytes into the actual owned effect payload under quoted copy/capacity/work, then donate one OriginalEmissionInput<A> to this existing intake. The source needs persistent name/byte/field cursors and original cancellation/retirement custody. Static storage removes source backing allocation; it does not make destination Vec/String/Emit allocations or copied bytes free. No larger ceilings or replacement caller are necessary.

## Existing Action Ingress and the Boundary That Must Change

- 🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:138–195 retains ActionIngressSource, metadata, revision checks, typing settlement and source loans before reserved/app dispatch. This action ingress can carry a named static asset selector without changing app-command wire types.
- Same file:75–78 already forwards a retained OriginalEmissionSource to dispatch_original_emission.
- Same file:191–203 routes true framework-reserved actions before app command production. The catalogue fallback is selected only when the app did not declare setActiveExample.
- 📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:309 exposes catalogue_example_document as an owned String result.
- 🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs:404–408 currently parses the whole catalogue document, cold-encodes Pack, builds empty SPR and dispatches an Emit in one async call. It is not an already bounded static-Pack producer. Reusing it unchanged would preserve the exact gap being investigated.

Recommended source boundary: one schema-first named static Pack declaration + one framework-owned, source-bound retained producer, routed before app command construction and preserving existing typing/revision/registration checks. Output remains the genuine original emission intake. Keep destination typed decode/publication validation; accepting static bytes is not permission to skip schema or subset checks. Runtime and Light own those adjacent audits.

## Live Native Driver

Session86185 / Python PID1905011 is preserved unchanged. Latest native-peer-work-coherent log was still capacity queued at1452 seconds with two earlier requests; no source capture/compiler started for this job then. Peer metadata3 and cold metadata1 are actual native GREEN; full lifecycle, fresh core query compile and subsequent page/editor/SDK jobs remain pending.


Live follow-up: the unchanged native request subsequently passed capacity and entered source.served/resource-directory-admission, then foreign-source admission. No native lifecycle result yet.


Later native result: semantic compilation stopped in ActionMeta's existing retained-field copier because Arc<ViewModel> does not implement RetainedClone. No lifecycle test executed. Parent has the exact source conflict and existing funded routing-alias port; no read-only task production changes were made.
