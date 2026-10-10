# Full Plugin Receiving Native6

Registered unchanged full native gate PID29840/session16187 completed terminal1. Cargo reports2338lib-test errors and202lib errors. No Plugin test ran. The complete machine contexts are retained under `🗑️generated/p/plugin-receiving-red6-errors.json`. Metadata, peer, and pure IO test-first errors below remain genuine compiler RED rather than acceptance.

## error[E0308]: mismatched types

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:435

```text
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:435
    |
 39 | ...e(&mut self.owner.child_id), Some(&mut self.actor.0), Some(&mut self.key.owner), Some(&mut self.key.slot), Some(&mut self.key.ch...
    |                                 ---- ^^^^^^^^^^^^^^^^^ expected `&mut String`, found `&mut SharedUtf8`
    |                                 |
    |                                 arguments to this enum variant are incorrect
    |
    = note: expected mutable reference `&mut std::string::String`
               found mutable reference `&mut SharedUtf8`
help: the type constructed contains `&mut SharedUtf8` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:39:430
    |
 39 | ...ome(&mut self.owner.child_id), Some(&mut self.actor.0), Some(&mut self.key.owner), Some(&mut self.key.slot), Some(&mut self.key....
    |                                   ^^^^^-----------------^
    |                                        |
    |                                        this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^

```

## error[E0308]: mismatched types

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:395

```text
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:395
    |
 42 | ...slot), Some(&self.owner.child_id), Some(&self.actor.0), Some(&self.key.owner), Some(&self.key.slot), Some(&self.key.child_id), S...
    |                                       ---- ^^^^^^^^^^^^^ expected `&String`, found `&SharedUtf8`
    |                                       |
    |                                       arguments to this enum variant are incorrect
    |
    = note: expected reference `&std::string::String`
               found reference `&SharedUtf8`
help: the type constructed contains `&SharedUtf8` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:42:390
    |
 42 | ...ot), Some(&self.owner.child_id), Some(&self.actor.0), Some(&self.key.owner), Some(&self.key.slot), Some(&self.key.child_id), Som...
    |                                     ^^^^^-------------^
    |                                          |
    |                                          this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^

```

## error[E0308]: mismatched types

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:155:452

```text
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🦀️.rs:155:452
    |
155 | ... slot, child_id: child }, actor: protocol::ActorId(actor), key: MemberKey { owner: key_owner, slot: key_slot, child_id: key_chil...
    |                                     ----------------- ^^^^^ expected `SharedUtf8`, found `String`
    |                                     |
    |                                     arguments to this struct are incorrect
    |
note: tuple struct defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🆔️ids/🦀️.rs:20:12
    |
 20 | pub struct ActorId(pub semio_framework_value::SharedUtf8);
    |            ^^^^^^^
help: call `Into::into` on this expression to convert `std::string::String` into `SharedUtf8`
    |
155 |             Ok(Some(PrivateChildMemberMetadata { parts: ManuallyDrop::new(Some(PrivateChildMemberMetadataParts { expected: ArtifactRef { artifact_id: id, dialect: ArtifactDialect { artifact_kind: kind, standard, subset } }, owner: store::OwnerRef { parent: ArtifactRef { artifact_id: parent, dialect: ArtifactDialect { artifact_kind: parent_kind, standard: parent_standard, subset: parent_subset } }, slot, child_id: child }, actor: protocol::ActorId(actor.into()), key: MemberKey { owner: key_owner, slot: key_slot, child_id: key_child }, prepared_identity: PreparedChildContentIdentity { key: MemberKey { owner: prepared_owner, slot: prepared_slot, child_id: prepared_child }, reference: ArtifactRef { artifact_id: prepared_id, dialect: ArtifactDialect { artifact_kind: prepared_kind, standard: prepared_standard, subset: prepared_subset } } }, publication_actor, transaction, group_id, registry_owner: store::OwnerRef { parent: ArtifactRef { artifact_id: registry_parent, dialect: ArtifactDialect { artifact_kind: registry_kind, standard: registry_standard, subset: registry_subset } }, slot: registry_slot, child_id: registry_child } })) }))
    |                                                                                                                                                                                                                                                                                                                                                                                                                                                                         +++++++

```

## error[E0425]: cannot find function `borrow_presence_metadata` in crate `protocol`

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12138:59

```text
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12138:59
      |
12138 |                 let (result, heap) = observe(|| protocol::borrow_presence_metadata(&bytes));
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `protocol`

```

## error[E0425]: cannot find function `borrow_presence_metadata` in crate `protocol`

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12155:53

```text
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12155:53
      |
12155 |             let (view, heap) = observe(|| protocol::borrow_presence_metadata(&encoded).unwrap());
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `protocol`

```

## error[E0425]: cannot find function `borrow_presence_metadata` in crate `protocol`

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12169:54

```text
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12169:54
      |
12169 |             let (error, heap) = observe(|| protocol::borrow_presence_metadata(&invalid));
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `protocol`

```

## error[E0308]: mismatched types

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:253:60

```text
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧩️composition/🪪️metadata/🧪️tests/🔬️unit/🦀️.rs:253:60
    |
253 |     let actual_actor: &semio_framework_value::SharedUtf8 = &parts.actor;
    |                       ----------------------------------   ^^^^^^^^^^^^ expected `&SharedUtf8`, found `&String`
    |                       |
    |                       expected due to this
    |
    = note: expected reference `&SharedUtf8`
               found reference `&std::string::String`

```

## error[E0433]: cannot find type `PeerRosterFault` in this scope

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12051:51

```text
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12051:51
      |
12051 |             let (mut retained, heap) = observe(|| PeerRosterFault::new(Some(fault)));
      |                                                   ^^^^^^^^^^^^^^^ use of undeclared type `PeerRosterFault`

```