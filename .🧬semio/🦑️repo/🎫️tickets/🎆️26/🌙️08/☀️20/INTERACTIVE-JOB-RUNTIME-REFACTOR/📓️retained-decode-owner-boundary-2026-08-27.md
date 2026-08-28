# Retained Decode Owner Boundary

The Mutation proposal at `SEMANTIC-MUTATIONS-OVERHAUL/🧪️flow-decode-ownership-45/📓️partial-decode-proposal.md` was read as a source/design audit, not native partial-decode evidence. Its raw-frame, partial-field, outer EOF and intrinsic conversion ownership problem is distinct from Kernel return-output framing. Reuse the existing Store authorities before designing another transaction API.

## Shared Store Region Is Not Reserved Here

This lane has no active or reserved edits to these existing definitions in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`:

- `OwnedSchemaRecordCursor` around 5402: fixed schema plus owned token/page cursor; `try_new` returns the original token cursor on schema rejection, and `step(StepContext)` returns schema-token/terminal state.
- `ArtifactEnvelopeMutationFieldAuthority` around 5813: token acceptance, publication into an exact reserved target, bounded close and terminal emptiness.
- `ArtifactEnvelopeMutationFieldTarget` around 5826: reserve/publish/cancel of the exact mutation field reservation.
- `ArtifactEnvelopeOwnedFieldCatalog` around 5840: mandatory snapshot/mutation/VCS/conflict and history-entry factories bound to operation, generation and path.
- `ArtifactEnvelopeDecodeAuthority` around 7421: the record and field lease stay in the job's structural fields; `try_new` returns the original record and field decoder on admission refusal. The field registry/ticket and pending token are retained across release steps.

Existing caller admission is in Plugin `VcsArtifactApp::begin_artifact_envelope_ingress`, `preflight_artifact_envelope_ingress_page`, `construct_and_admit_artifact_envelope_ingress_page`, `admit_artifact_envelope_ingress_page`, and `seal_artifact_envelope_ingress` around 19936–20000. It uses `ArtifactEnvelopeDecodeOperationHandle` with operation/generation and separate exact ingress/decode/replacement slots. This is not an AppCommand witness. Source inspection does not establish that every domain catalog or all cancellation/unwind cases already satisfy the proposed mutation change.

## Reserved Command And Interaction Region

Ordinary AppCommand input remains this lane's pending integration in `📡️spr/🧵️channel/🦀️component.rs`: `PagedCommand`, `PagedCommandReader`, `CommandEnvelope`, `CommandDriverRegistry`, `PagedAppCommandDecodeCursor` around 1184 and `DecodedAppCommandOwner` around 1190. The outer `PluginCommandIngress` state at Plugin around 31005 owns Encoded/Decoding/Decoded and closing states; `step(self)` returns the next owner or terminal result. It is a route-framing owner, not a complete generic mutation decoder or an unwind-safe structural-owner proof for every current branch. Its currently rejected ordinary tags 5, 10–14 and 17–26 still need route-specific retained implementations; Presence 28 has a separate reserved owner, and local query 29 uses its fixed scalar protocol.

The action-bus `ToolWireAdmission` and `RetainedToolWireInput` in `🎯️action-bus/🦀️component.rs` are factory/contract and raw page authorities. `plugin/🧵️retained-command/🦀️component.rs::ArtifactRetainedCommandJob::from_wire` carries those pages together with an already constructed typed command payload and work/context owners. These are not proof that arbitrary `DeserializeOwned` field conversion or outer EOF cleanup is retained. Do not borrow their factory witness to authorize a different mutation/frame decoder.

Native InteractionConfigMutation/SetInteractionState and the retained local-interaction root/update remain owned here. Their future Store publication is not a reservation on the shared envelope decoder/catalog region. The tutorial TypeScript handoff is recorded separately. Kernel return headers, borrowed SendMessage encoding, native return-origin admission and paged source retirement are also owned here; none calls for another mutation input ABI.

No shared production decoder, derive, trait, catalog, authority or caller was modified by this audit. Mutation's shared schema/API work can proceed under coordinator ownership; its exact production cutover and caller adoption need a coherent boundary, preserving original owners on refusal/error/unwind. No cold seed wrapper, global cleanup sink, compatibility decoder, or new inferred authority is accepted.
