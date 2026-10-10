# Full Kernel Receiving Compiler Evidence

Exact registered full Kernel child26251/session49758 terminated exit1. Actual compiler emitted nine diagnostics and no law assertions executed. No sealed peer or erased read missing-method failure was reached. Original build600000 and full mutation-testing/all-targets/no-fail-fast/ignore-default-filter policy remained unchanged.

The current original operation import was already repaired by its Media owner to crate::Mutation. Removed only the eight redundant cross-domain history retirement registrations; canonical SPR field-aware retirement derives remain authoritative. The next exact full run is required before either Store producer.

```text
error[E0432]: unresolved import `crate::os_store::Mutation`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📦️codec/🎮️operation/🦀️.rs:2:105
   |
 2 | use crate::os_store::{DocumentStoreOwners, DocumentStoreOwnersAdmissionError, ErasedSnapshotRetirement, Mutation};
   |                                                                                                         ^^^^^^^^ no `Mutation` in `os_store`
   |
   = help: consider importing one of these traits instead:
           crate::Mutation
           protocol::Mutation
note: variant `crate::os_store::component::canonical_edit::CanonicalEditNode::Mutation` exists but is inaccessible
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:71:5
   |
71 |     Mutation(&'a M),
   |     ^^^^^^^^^^^^^^^ not accessible
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::HistoryLog`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:416:1
    |
416 | semio_framework_value::artifact_retire_struct!(crate::os_spr::HistoryLog { doc_id, schema, edits, transitions, composition, conflicts, viewer_line, viewer_checkpoint });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::HistoryLog`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:36:44
    |
 36 | #[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned)]
    |                                            ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::HistoryComposition`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:417:1
    |
417 | semio_framework_value::artifact_retire_struct!(crate::os_spr::HistoryComposition { owner, dialect });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::HistoryComposition`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:78:44
    |
 78 | #[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
    |                                            ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::HistoryTransitionRecord`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:418:1
    |
418 | semio_framework_value::artifact_retire_struct!(crate::os_spr::HistoryTransitionRecord { id, actor, hlt, dependencies, observed, payload });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::HistoryTransitionRecord`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:64:35
    |
 64 | #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
    |                                   ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `HistoryConflict`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:419:1
    |
419 | semio_framework_value::artifact_retire_struct!(crate::os_spr::history::HistoryConflict { id, kind, status, actors, hlt, edit_ids, envelopes, messages });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `HistoryConflict`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:94:35
    |
 94 | #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
    |                                   ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `HistoryMessage`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:420:1
    |
420 | semio_framework_value::artifact_retire_struct!(crate::os_spr::history::HistoryMessage { level, code, message, target, op_index });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `HistoryMessage`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:113:35
    |
113 | #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
    |                                   ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::HistoryEdit`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:421:1
    |
421 | semio_framework_value::artifact_retire_struct!(crate::os_spr::HistoryEdit { id, actor, line, started_at, finished_at, verb, ops, inverse, meta, lane });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::HistoryEdit`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:122:35
    |
122 | #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
    |                                   ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::OpPayload`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:422:1
    |
422 | semio_framework_value::artifact_retire_struct!(crate::os_spr::OpPayload { text, binary });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::OpPayload`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:158:35
    |
158 | #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
    |                                   ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```text
error[E0119]: conflicting implementations of trait `semio_framework_value::retirement::RetireOwned` for type `os_spr::history::HistoryOpMeta`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:423:1
    |
423 | semio_framework_value::artifact_retire_struct!(crate::os_spr::HistoryOpMeta { op_id, dependencies, base_version, author_id, hlt, undo_policy, payload_hash, group_id, origin, messages, transaction });
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `os_spr::history::HistoryOpMeta`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/📜️history/🦀️.rs:164:44
    |
164 | #[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned)]
    |                                            ---------------------------------- first implementation here
    |
    = note: this error originates in the macro `semio_framework_value::artifact_retire_struct` (in Nightly builds, run with -Z macro-backtrace for more info)
```
