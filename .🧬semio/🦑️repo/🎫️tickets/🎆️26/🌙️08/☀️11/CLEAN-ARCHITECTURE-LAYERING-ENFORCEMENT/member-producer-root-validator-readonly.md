# Member Producer Root Validator Read-Only Audit

Static source observation only. No Root import/initialization, test run, source mutation, native/resource/frontend action, or runtime correctness claim. Concurrent edits may invalidate these hashes.

Root `toolJobMemberStoreOwnerExact` (1886; invocation 6483, export 25658) necessarily rejects current sources: its leading extraction requires removed `MemberStoreOwner<Mutation>` and `install_member_store_owners_exact`. `store` at 6333 is `policyReadOsStoreSource(root)`. That assembles only nineteen hard-coded paths (16946 onward), omitting initialization owners, member producer schema, initial open operation/catalog, and canonical tests. `semio` at 6349 reads only artifact root; the producer defining macro now lives in the v1 base contract. A rename-only repair would still fail or accidentally accept source copied into unrelated text.

Stale requirements: MemberStoreOwner trait/default method, MemberStoreOwners struct, ArtifactPack + MemberStoreOwner bounds/count, two P::member_store_owners installation occurrences, old create/open call signatures, old Some(owners.field) installation assignments, semio_subset_table!(member_owners), old snapshot trait impl, dsl::MemberStoreOwners::new, SemioOwnedValueRetirementFactory/SemioMutationRetirementFactory and SemioStoreClosePhase/SemioStoreOwnedDisposer names. Those Semio implementations are absent from current stdio Rust. The blocked_disposer test name is stale: canonical root test is `member_close_rejects_missing_owner_and_preserves_the_installed_disposer_until_terminal` (10195).

Keep the original obligations for ArtifactOwnedValueRetirementFactory, ArtifactStoreCloseView extraction lanes, ManuallyDrop envelope/current, terminal-empty Drop guards, edit-ledger ownership/capacity/no map replacement, ArtifactStoreOwnedDisposer terminal predicate, child factory rejection, exact create/open wrapper bypass refusal, and no ad hoc snapshot installation. Bind each to its actual defining file/declared tests rather than aggregate text. Do not delete those checks to obtain green.

Concrete current binding: read the exact producer schema: `MemberStoreProducer<P,M>:Copy+Send+Sync+'static`, associated `SnapshotOpen:MemberSnapshotOpenOperation<Snapshot=P>`, mandatory declaration-only birth-demand and grant-consuming owners methods returning DocumentStoreOwners and original progress/admission-error. Read DocumentStoreOwners defining initialization owners source: four DocumentCatalogOwner wrappers, constructor completeness/tickets and bounded uninstalled-owner close. Read InitialMemberStoreOpen operation: C bound, retained producer:C, ManuallyDrop<Option<C::SnapshotOpen>>, real source quote summed into request birth bytes, producer.member_store_owners(source_grant), progress.fits(source_grant) AND exact retained_capacity_bytes equality, constructor gate, original owners retained and transferred once into hydration. Read exact installer root block: rejects incomplete constructor and existing catalog; transfers original factory tickets and each original field via take(), including disposer. Bind cold create/open to their C parameter and install_funded_member_store_owners; they no longer recover catalog from P.

Semio binding: defining base macro impl is MemberStoreProducer<$snapshot,$mutation> for SemioMemberStoreProducer<$snapshot,$mutation>, associated SnapshotOpen=$open; real bounded birth-demand and bounded owners(grant) calls. Artifact root table contains explicit producer type and value per arm. Check macro declaration plus actual table selections and selected definitions, without accepting arbitrary string mentions, comments, fixture text or alternate owner fallback.

Existing verification mounts retained: catalog portable test at composition/open/operation/catalog/tests/🟦️.ts uses bun:test, Ajv and fast-json-patch, producer schema/fixture refusal oracle, original ingress capacity and independently funded ticket phases, and exact source assertions. Canonical root Rust tests contain wrapper bypass (10174), installed-disposer-until-terminal (10195), empty-genesis/dialect rejection (10217). No test was run in this read-only task. Root should update its guard consumers and use its existing typed/portable command mount, not import the permanent script to evaluate the guard.

## SHA-256 Observations

| Source | SHA-256 |
| --- | --- |
| 📜️script.ts | 67021eebc98c2c60b43a0a23e9425d11a57713e4a554c0a43d7576fa35a0e5fe |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 8d875a428ebaca6702d7bcc03b80e4907b43bd29ee4d511216ec9f38ad794c60 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs | 96a5755854ee1d041505401d22b1426aac9ec472d03eccee4f00224ed100a6c8 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs | 46922b4ec47260939b73ed02c0ee7154067830d825a6242d3557df7e00fb86b3 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs | 341faf811bc518f31056b69b47250af077fd3dc86e7a9ba2bd68e00e382be6be |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/📦️catalog/🧪️tests/🟦️.ts | 75a31abf654a43bfe7fc30e770bbe0cfb1e0e9576150761eb3bd5c90dba590c1 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | e183381255014f3c1d170614da2dda253103d65fb1fdd32242b603efa6e532f0 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs | eca0baf3410b8b81fd66856f87ec8dba2d85cbd726a83c5d8d207540aa387b84 |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs | 51999a3bd2ff3ed1d474e27351eebe2a915bf48eb296e007b6be3db1e55ddee1 |
