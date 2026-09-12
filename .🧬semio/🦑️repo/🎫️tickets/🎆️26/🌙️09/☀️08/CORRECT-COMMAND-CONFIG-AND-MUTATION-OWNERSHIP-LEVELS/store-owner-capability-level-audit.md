# Store Owner Capability-Level Audit

## Decision

`DocumentStoreOwners<P, M>` must become the domain-neutral, required
`StoreLifecycleOwners<P, M>` catalog. It owns only the four lifecycle authorities that an
initialized `ArtifactStore` retains for its lifetime:

1. `SnapshotRetirementFactory<P>`;
2. initial-snapshot retirement;
3. mutation retirement; and
4. the store disposer.

It must **not** acquire a persisted-document decoder. A decoder is needed only while an
owned persisted document candidate is being admitted and hydrated. Keeping it in the
live-store catalog would give every config, draft, interaction, fixture, and support store a
document-load capability it neither requests nor retains.

The required capability belongs in a distinct
`PersistedDocumentLoadOwners<P, M>`, consumed by the common document hydrator before it
creates an `ArtifactStore`. It composes:

```text
PersistedDocumentLoadOwners<P, M>
├── lifecycle: StoreLifecycleOwners<P, M>
└── snapshot_decoder: Arc<dyn PersistedDocumentSnapshotDecodeFactory<P>>
```

There is no optional decoder field, rejecting dummy decoder, synchronous `P::decode_pack`
fallback, alias, or retained decoder on the resulting store. Absence of this composite is an
admission failure before the owned pack is transferred out of its caller.

The proposed names are final replacement names. Remove `DocumentStoreOwners`,
`install_document_store_owners_exact`, and their old builder names in the same migration;
do not leave aliases or compatibility entry points.

## Evidence and Classification

### The current catalog is a lifecycle catalog

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2156-2193` defines
`DocumentStoreOwners`. Its constructor requires exactly the four lifecycle authorities above.
`ArtifactStore::from_initialized_runtime_with_owners` (`:14888`) and
`install_document_store_owners_exact` (`:15252`) move those owners into a generic
`ArtifactStore`; neither checks an artifact reference, owner reference, dialect, schema, or
pack. This proves that the type is store lifecycle, not document archive identity.

The catalog also currently carries two optional publication fields. Those fields are not
lifecycle:

| Current field | Actual capability | Proper boundary |
| --- | --- | --- |
| `one_item_preparation` | Typed mutation staging against an exact store base. The plugin runtime already obtains these independently from `ArtifactApp` and supplies them to `begin_apply_batch`. | Generic store publication module, separate from lifecycle. |
| `one_item_wire_preparation` | Decodes/prepares an owned `schema + bytes` mutation payload, then stages it against a particular store. | Generic encoded-mutation publication module, with composition routing outside the decoder. |

The only production references to `.with_one_item_preparation` and
`.with_one_item_wire_preparation` are their definitions; the installers are exercised by the
store unit fixture. They therefore do not justify retaining either optional field in the
87-source lifecycle catalog.

### Wire preparation is not a member-relationship factory

`MemberStoreOneItemWireRequest` (`store/🦀️.rs:13955-13974`) carries:

```text
operation, expected_generation, expected_revision, actor, group_id,
schema, bytes, description
```

It has no `ArtifactRef`, `OwnerRef`, child slot, child id, parent id, or dialect. A repository
search found no non-test caller of `begin_one_item_wire_publication` or its request type.
The generic `ArtifactStore<P, M>` blanket implementation itself implements `SpaceMember`
(`:18976`), so the method name does not establish a member relationship.

`begin_one_item_wire_publication` does establish an important *different* identity:
it passes the request's operation/generation/revision to the store's primary-history batch
admission and retains `self.snapshot_read_leases` in the resulting publication (`:19005-19020`).
Advance, prepare, abort, and close reject a publication whose retained lease is not the same
store. This is exact-store authority, not parent/member identity. The decoder is therefore a
generic encoded-mutation capability; the composition adapter owns any child selection and
member relationship before it calls the generic capability.

This audit found no wrong-owner runtime path: ordinary config owners do not install a wire
factory, `artifact_ref()` is `None` when an envelope has no dialect, and no registry/archive
caller admits a config store as a child from the evidence inspected. The present
`SpaceMember` availability is an over-broad mixed trait surface, not a demonstrated archive or
cross-store safety failure.

### `HistoryLane::Document` is primary history, not the Artifact publication target

The core definition at `store/🦀️.rs:2435-2467` says `HistoryLane::Document` is a store's
ordinary main undo/redo history. `Interaction` is a replayable side history. The separate
`ArtifactToolPublicationLane` chooses *which* store receives a mutation.

Consequently these active calls are correct in intent:

| Call path | Target selected elsewhere | History used | Finding |
| --- | --- | --- | --- |
| `plugin/🦀️.rs:25072` | artifact store | `Document` | ordinary primary history |
| `plugin/🦀️.rs:25097` | config store | `Document` | config's own ordinary primary history |
| `plugin/🦀️.rs:25126` | draft store | `Document` | draft's own ordinary primary history |
| `plugin/🪟️window/🎚️config/🦀️.rs:420` | exact window partition | `Document` | that partition's own ordinary primary history |

The configuration factories intentionally require that value, including the bounded factory
at `plugin/🦀️.rs:13788-14005`. Do not add Config, Draft, or Window variants to
`HistoryLane`, and do not redirect these calls. A future neutral rename from `Document` to
`Primary` is reasonable only as one complete semantic rename of the generic ordinary-history
axis; it is independent of this capability migration and must not introduce an alias.

### Config, draft, interaction, and window configuration have distinct persistence facts

| Context | Current lifecycle construction | Actual persisted load path | Required migration conclusion |
| --- | --- | --- | --- |
| Root document | `ArtifactApp::build_document_store_owners`; archive load calls common retained hydrator at `plugin/🦀️.rs:21344-21358` | root archive has exact `ArtifactRef`, optional `OwnerRef`, dialect, schema, decoded SPR, and owned pack | Require `PersistedDocumentLoadOwners`; obtain it before `mem::take(parent_pack)`. |
| Composed member | `MemberStoreOwner::member_store_owners`, `create_member_store`, `open_member_store`, and `InitialMemberStoreOpen` | `open_member_store` validates expected artifact/owner/dialect/schema; initial open obtains a typed snapshot through `SnapshotOpen`, then calls the common hydrator from `:508` | Feed a decoder-produced retained snapshot into the same document load path. A public `from_initial(P)` bypass is not acceptable. |
| App config | `bounded_config_store_owners`, `NoConfig` owner factory, and app builders | `hydrate_config_lane`, `load_config_pack`, and `AppCommand::LoadConfig` all reach `parse_document_pack` then reset (`plugin/🦀️.rs:27591`, `:27839`, `:35213`) | Preserve a real `PersistedConfigLoadOwners<C, M>` and config decoder at the config persistence boundary. |
| Draft | `no_draft_store_owners` uses the same lifecycle shape | `hydrate_draft_lane` parses pack+SPR then resets (`plugin/🦀️.rs:27597`) | Preserve a separate `PersistedDraftLoadOwners<D, M>` and decoder. Do not inherit document identity. |
| Window config | `WindowConfigOwner::build_store_owners`; one partition per window id | `TypedWindowConfigStoreOwner::load` parses then checks schema (`window/config/🦀️.rs:457-470`) | Require `PersistedWindowConfigLoadOwners<O>` with exact window id, kind, and schema. Validate the expected generated envelope id `window-config:{kind}:{window_id}`, not schema alone. |
| Interaction | `interaction_store_owners()` creates only lifecycle owners | no persisted pack decoder call was found | Lifecycle-only. Do not add a persisted decoder merely because its backing algebra is `ArtifactStore`. |

`NoConfig` is not evidence that persistence is absent. It implements `ArtifactPack`: an empty
pack is a valid real zero-state decode and nonempty bytes are rejected
(`plugin/🦀️.rs:9411-9460`). `NoDraft` is presently an alias of it. Since config/draft host
load routes exist, their replacement must have a real zero-state decoder at the respective
config or draft boundary, not a blanket rejection factory. The later no-alias migration should
give the zero-draft state its own nominal type if the two capabilities remain separate.

## Target Type and Folder Boundary

Keep the generic store algebra in `🏪️store`; put document persistence in the document
subtree and lane-specific persistence beside its owning plugin lane.

```text
🏪️store/
├── 🧱️lifecycle/🦀️.rs
│   └── StoreLifecycleOwners<P, M>
├── 📬️mutation/🦀️.rs
│   ├── TypedMutationPreparationFactory<P, M>
│   ├── EncodedMutation { schema, bytes }
│   └── EncodedMutationPreparationFactory<P, M>
├── 🧾document/
│   ├── 🎒️pack/💧️decode/🦀️.rs
│   │   ├── PersistedDocumentDecodeRequest
│   │   ├── PersistedDocumentSnapshotDecodeFactory<P>
│   │   └── retained, cancellable decode operation
│   └── 💧️load/🦀️.rs
│       └── PersistedDocumentLoadOwners<P, M>
└── 🧩️composition/
    ├── 📬️publication/🦀️.rs
    │   └── member selection plus exact-store lease adapter around encoded preparation
    └── 🚪️open/…
        └── member identity validation and retained decoder input

🔌️plugin/
├── ⚙️config/🎒️pack/💧️decode/…
│   └── PersistedConfigLoadOwners<C, M>, ConfigLoadIdentity
├── 📝️draft/🎒️pack/💧️decode/…
│   └── PersistedDraftLoadOwners<D, M>, DraftLoadIdentity
└── 🪟️window/🎚️config/🎒️pack/💧️decode/…
    └── PersistedWindowConfigLoadOwners<O>, WindowConfigLoadIdentity
```

`PersistedDocumentDecodeRequest` must carry the exact typed document identity:
`ArtifactRef`, optional `OwnerRef`, schema, operation, generation, expiry, and the owned pack.
It must preserve that owner on admission failure and give cancellation/retirement a bounded
step. The document factory validates identity before yielding a typed candidate. A member
snapshot opener may be an implementation of the same retained decode protocol, but it must
return an opaque decoder-produced snapshot token rather than exposing a synchronous typed
snapshot bypass.

The config and draft identities are deliberately separate nominal types. They must contain the
identity their caller really has (app/lane/schema and operation context), not a fabricated
`ArtifactRef`. `WindowConfigLoadIdentity` contains the requested window id, registered kind,
schema, and the generated partition envelope id. This makes the missing current id/kind
validation explicit.

For the mixed `SpaceMember` surface, add an explicit document-composition marker bound to its
`ArtifactStore` implementation. `ArtifactCompositionFields` is structural and is implemented by
`NoConfig`, so it cannot be the marker. The marker must be implemented by root and member
document snapshot schemas used by `VcsArtifactApp`/`CompositionCoordinator`; it must not be
implemented by config, draft, window-config, or interaction state. Do not use
`MemberStoreOwner` as this bound: parent/root document stores also act as `SpaceMember` during
group dispatch. This is a compile-time boundary refinement that preserves generic store
composition and exact-store lease behavior.

## Precise Migration Inventory

The existing `document-store-owner-catalog-rename-ledger.md` records 87 Rust source files and
212 `DocumentStoreOwners` token occurrences. They are not 87 archive grammars. Migrate them by
capability category rather than with a blind text rename.

1. **Core lifecycle move.** In `store/🦀️.rs`, replace `DocumentStoreOwners` with
   `StoreLifecycleOwners`; make its constructor contain the four required lifecycle owners
   only. Rename `from_initialized_runtime_with_owners`'s parameter type and
   `install_document_store_owners_exact` to `install_store_lifecycle_owners_exact`. Move
   `retire_decoded_edit` and `retire_decoded_envelope` with the lifecycle retirement helper.
   Remove both `with_one_item_*` methods and optional stored fields.

2. **Generic publication move.** Move `ArtifactStoreOneItemPreparationFactory` and the former
   member-wire factory to `🏪️store/📬️mutation`. Rename the latter to
   `EncodedMutationPreparationFactory`, its input to `EncodedMutation`, and its request to a
   generic encoded primary-publication request. Preserve operation, generation, revision,
   actor, group id, primary-history semantics, and exact-store lease binding. The composition
   publication adapter owns member selection; it does not put parent/member data in a generic
   decoder request. The app runtime's independently constructed artifact/config/draft typed
   factories remain separate capabilities.

3. **Document load move.** Change
   `RetainedPersistedDocumentHydration` in
   `🏪️store/🧾document/📜️history/💧️hydration/🦀️.rs` to retain
   `PersistedDocumentLoadOwners`, an owned decode operation/token, and lifecycle owners only
   after decode completes. Replace its `ScanPack`/`P::decode_pack` path with the required
   retained factory. Replace public `from_initial` with a constructor accepting the opaque
   decoder result. Update the root archive call around `plugin/🦀️.rs:21344` to acquire load
   owners before transferring `parent_pack`.

4. **Composition open move.** Update `MemberStoreOwner`, `create_member_store`,
   `open_member_store`, `InitialMemberStoreOpen`, and the open-operation owner fields in
   `🏪️store/🧩️composition/🚪️open` to use lifecycle owners for a fresh initialized store and
   `PersistedDocumentLoadOwners` for persisted open. Preserve the current identity validation
   in `open_member_store`; remove its synchronous `parse_decoded_document_spr` path only after
   it enters the common retained load protocol.

5. **Artifact app and domain factories.** Rename every
   `build_document_store_owners`, `build_config_store_owners`, `build_draft_store_owners`,
   `bounded_config_store_owners`, `bounded_document_store_owners`, the 21 direct
   `DocumentStoreOwners::new` producer sites, and all corresponding imports to lifecycle names.
   This includes the plugin artifact schemas, DAG, database hash support, MCP probe, Flow, and
   the core support fixtures identified in the ledger. Each normal runtime construction returns
   `StoreLifecycleOwners`, never a decoder.

6. **Config/draft/window retained loads.** Add distinct app trait builders for config and draft
   load owners where those host loads are enabled. Route `hydrate_config_lane`,
   `load_config_pack`, `hydrate_draft_lane`, and `TypedWindowConfigStoreOwner::load` through
   their matching retained decoder. `WindowConfigOwner::build_store_owners` becomes a lifecycle
   builder and gains a separate window-load builder. Retain the independently supplied typed
   config publication factory; it is neither lifecycle nor document decoding.

7. **Interaction and support.** Rename interaction's catalog to lifecycle-only. Keep it out of
   every persisted decoder builder. Update source fixtures/tests/MCP/DB/DAG support catalogs to
   lifecycle names, except the recursive archive fixture which must provide a real document
   decoder to exercise the retained path. Remove every old import/name in the same patch.

8. **Type-surface refinement.** Add the document-composition marker to actual root/member
   document snapshots and require it for `SpaceMember`. Assert that config, draft,
   window-config, and interaction types cannot satisfy that trait. This is not a request to
   change their primary history or generic `ArtifactStore` algebra.

## Schema-First Laws Required Before Implementation

Add a neutral JSON Schema fixture, for example
`schema/store-owner-capability-level-laws.schema.json`, before Rust implementation. Every row
must name a route, requested identity, supplied capability set, owned-input state, expected
result, decoder invocation count, and resulting store/primary-history identity.

Required rows are:

| Route | Required proof |
| --- | --- |
| root document archive | exact artifact/dialect/schema/owner succeeds once; mismatches reject before decoder or pack transfer |
| composed member | exact parent/slot/child/dialect/schema succeeds; each mismatch rejects and cannot use a root-only load owner |
| config | active persisted config route uses its own real decoder; `NoConfig` accepts only its empty representation |
| draft | active persisted draft route uses a distinct nominal load owner and zero-draft decoder |
| window config | id, kind, schema, and generated envelope id all must match the registered partition |
| interaction | lifecycle installation works and no persisted-document decoder can be supplied or invoked |
| typed publication | Artifact, Config, Draft, and WindowConfig select different target stores while each records ordinary primary history |
| encoded publication | schema/bytes parsing has no parent/member identity; wrong lease, stale generation, and wrong target owner reject without cross-store advance |
| cancellation | each retained decoder returns/retirements the exact owned input under bounded grants and never falls back synchronously |

Validate those rows through three independent forms of evidence:

1. **Third-party schema law:** use the ticket's Bun validation script with a JSON Schema
   validator such as Ajv to validate every row and write normalized expected decisions. This
   remains test tooling, not a runtime dependency.
2. **Native law:** Rust tests execute the retained factories with counting decoders and exact
   owner pointers. They compare route decisions and ownership/cancellation observations to the
   neutral rows. Include an actual `NoConfig` empty-pack decode and a nonempty rejection.
3. **Compile law:** compile fixtures prove that a `StoreLifecycleOwners` cannot call document
   hydration; config/draft/window load owners cannot satisfy the document load parameter;
   `NoConfig` cannot implement/use `SpaceMember` without the document-composition marker; and
   no old catalog or installer name resolves. A generic encoded parser may compile for a lane
   only when that lane's controller supplies its exact store authority.

No source, Cargo, test, configuration, or git state was changed for this audit. The conclusions
come from static source and call-path inspection; no test suite was run.
