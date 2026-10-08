# Flow6 Actual Full Receiving Refusal

Registered Nx/Bun1/Cargo101 under the original full owning controls. No runtime target or whole OS acceptance. Producer exact; four Store source files advanced during this cut, preserved with full captured/terminal/current bodies. GeneralUI is the first actual compiler boundary: prepared scene, mounted layout and host close jobs still implement the old two-scalar job close signature and receipt fields.

Initial report write physically refused ENOSPC; terminal/admission/compiler evidence was already saved. The unchanged read-only facts were then saved when filesystem availability returned; no Native source/target deletion occurred.

{
  "code": 101,
  "reason": "exit",
  "outer": 1,
  "exact": false,
  "producer": true,
  "sources": 1023,
  "owning": false,
  "whole": false,
  "errorBlocks": 585,
  "changed": [
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧪️tests/🔬️unit/🦀️.rs",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
  ]
}

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3417:19
     |
3417 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
     |
3417 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1528:19
     |
1528 |     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
     |
1528 -     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:222:19
    |
222 |     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
    |
222 -     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3419:76
     |
3419 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                            ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3419:95
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3419:95
     |
3419 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                               ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3422:13
```

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3422:13
     |
3422 |             semio_framework_job::InteractiveJobCloseStep::Complete
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
     |
help: you might have meant to create a new value of the struct
     |
3422 |             semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ }
     |                                                                    +++++++++++++++++++++++++
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3424:69
     |
3424 |             semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |                                                                     ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3424:88
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:3424:88
     |
3424 |             semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |                                                                                        ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1530:76
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1530:76
     |
1530 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                            ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1530:95
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1530:95
     |
1530 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                               ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1533:13
```

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1533:13
     |
1533 |             semio_framework_job::InteractiveJobCloseStep::Complete
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
     |
help: you might have meant to create a new value of the struct
     |
1533 |             semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ }
     |                                                                    +++++++++++++++++++++++++
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1535:69
     |
1535 |             semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |                                                                     ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1535:88
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:1535:88
     |
1535 |             semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
     |                                                                                        ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1475:38
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1475:38
     |
1475 |                     let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
     |                                      ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
     |                                                 |
     |                                                 expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2778:12
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1486:37
     |
1486 |                     let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
     |                                     ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
     |                                                |
     |                                                expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2422:12
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2142:30
     |
2142 |             let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
     |                              ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
     |                                         |
     |                                         expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2778:12
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2155:33
     |
2155 |                 let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
     |                                 ^^^^^^^^^^ -  ------------------------------------------- unexpected argument #2 of type `usize`
     |                                            |
     |                                            expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️.rs:2422:12
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:225:80
    |
225 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
    |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
    |
    = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:225:99
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:225:99
    |
225 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
    |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
    |
    = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:228:76
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:228:76
    |
228 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    |                                                                            ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
    |
    = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:228:95
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:228:95
    |
228 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
    |                                                                                               ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
    |
    = note: available fields are: `progress`

error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:230:9
```

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:230:9
    |
230 |         semio_framework_job::InteractiveJobCloseStep::Complete
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
    |
help: you might have meant to create a new value of the struct
    |
230 |         semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ }
    |                                                                +++++++++++++++++++++++++
```

```text
error[E0252]: the name `RetainedCloneBirthDemand` is defined multiple times
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:9:208
  |
9 | ...r, RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneStep, RetainedCloneBirthDemand, adm...
  |       ------------------------                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^--
  |       |                                                                                                                              |
  |       |                                                                                                                              `RetainedCloneBirthDemand` reimported here
  |       previous import of the type `RetainedCloneBirthDemand` here                                                                    help: remove unnecessary import
  |
  = note: `RetainedCloneBirthDemand` must be defined only once in the type namespace of this module
```

```text
error[E0252]: the name `ValueError` is defined multiple times
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:12:29
   |
10 | use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, shared::Shared...
   |                             ---------- previous import of the type `ValueError` here
11 | use std::{marker::PhantomData, mem::size_of, sync::Arc};
12 | use semio_framework_value::{ValueError,ValueRefusalKind,retirement::{RetireOwned,shared::SharedControlledRetirement}};
   |                             ^^^^^^^^^^-
   |                             |
   |                             `ValueError` reimported here
```

```text
error[E0252]: the name `ValueRefusalKind` is defined multiple times
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:12:40
   |
10 | use semio_framework_value::{ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, shared::Shared...
   |                                         ---------------- previous import of the type `ValueRefusalKind` here
11 | use std::{marker::PhantomData, mem::size_of, sync::Arc};
12 | use semio_framework_value::{ValueError,ValueRefusalKind,retirement::{RetireOwned,shared::SharedControlledRetirement}};
   |                                        ^^^^^^^^^^^^^^^^-
   |                                        |
   |                                        `ValueRefusalKind` reimported here
```

```text
error[E0252]: the name `RetireOwned` is defined multiple times
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:12:70
   |
10 | ...ror, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement, shared::SharedControlledRetirement}, FactoryAu...
   |                                        ----------- previous import of the trait `RetireOwned` here
11 | ...:size_of, sync::Arc};
12 | ...ror,ValueRefusalKind,retirement::{RetireOwned,shared::SharedControlledRetirement}};
   |                                      ^^^^^^^^^^^-
   |                                      |
   |                                      `RetireOwned` reimported here
```

```text
error[E0252]: the name `SharedControlledRetirement` is defined multiple times
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:12:82
   |
10 | ...retirement::{RetireOwned, controlled::ControlledRetirement, shared::SharedControlledRetirement}, FactoryAuthority};
   |                                                                ---------------------------------- previous import of the type `SharedControlledRetirement` here
11 | ...
12 | ...tirement::{RetireOwned,shared::SharedControlledRetirement}};
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SharedControlledRetirement` reimported here
   |
   = note: `SharedControlledRetirement` must be defined only once in the type namespace of this module
```

```text
error[E0432]: unresolved import `semio_framework_value::SnapshotRetirementStep`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27:123
   |
27 | ...ment, SnapshotRetirementFactory, SnapshotRetirementStep};
   |                                     ^^^^^^^^^^^^^^^^^^^^^^ no `SnapshotRetirementStep` in the root
   |
help: a similar name exists in the module
   |
27 - pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
27 + pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementFactory};
```

```text
error[E0432]: unresolved import `super::SnapshotRetirementStep`
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:6:180
  |
6 | ...est, MemberOpenStep, SnapshotRetirementStep};
  |                         ^^^^^^^^^^^^^^^^^^^^^^ no `SnapshotRetirementStep` in `os_store::component::member_open`
  |
  = help: consider importing this unresolved item through its public re-export instead:
          crate::SnapshotRetirementStep

error[E0201]: duplicate definitions with name `begin_demand`:
```

```text
error[E0201]: duplicate definitions with name `begin_demand`:
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:89:5
      |
   85 | /     fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
   86 | |         Ok(RetainedCloneBirthDemand{capacity_bytes:size_of::<RetainedClonePreparation<P,M,E>>(),depth:1})
   87 | |     }
      | |_____- previous definition here
   88 |
   89 | /     fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
   90 | |         if !M::controlled_retirement_supported() || !P::controlled_retirement_supported() {
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:80:5
   |
80 |     fn next_close_byte_demand(&self) -> usize { self.request.as_ref().map_or(0, MemberOpenRequest::next_close_byte_demand) }
   |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |  |
   |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
   |     not a member of trait `ErasedSnapshotRetirement`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:233:5
    |
233 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
234 | |         if let Some(active) = self.active.as_ref() { return active.next_close_byte_demand(); }
235 | |         if self.snapshot.is_some() { return 1; }
236 | |         self.request.as_ref().map_or(usize::from(!self.terminal), MemberOpenRequest::next_close_byte_demand)
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:868:5
    |
868 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
869 | |         if let Some(active) = self.active.as_ref() { return active.next_close_byte_demand(); }
870 | |         if self.genesis_request.is_some() || self.genesis_pack.is_some() { return 1; }
871 | |         if let Some(hydration) = self.hydration.as_ref() { return hydration.next_close_byte_demand(); }
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:561:5
    |
561 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
562 | |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
563 | |         if let Some(runtime) = self.runtime.as_ref() { return runtime.next_close_byte_demand(); }
564 | |         if self.pending_edit.is_some() || self.envelope.is_some() || self.initial.is_some() || self.history.is_some() { return 1; }
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:994:5
     |
 994 |       fn next_close_byte_demand(&self) -> usize {
     |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
     |  _____|
     | |
 995 | |         if self.terminal { return 0; }
 996 | |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
 997 | |         if let Some(job) = self.fold_job.as_ref() { return job.next_close_byte_demand(); }
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:578:5
    |
578 |       fn next_close_byte_demand(&self) -> usize {
    |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
    |  _____|
    | |
579 | |         if self.terminal { return 0; }
580 | |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
581 | |         if let Some(job) = self.fold_job.as_ref() { return job.next_close_byte_demand(); }
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1339:5
     |
1339 |       fn next_close_byte_demand(&self) -> usize {
     |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
     |  _____|
     | |
1340 | |         self.active.as_ref().map_or_else(|| self.dag.as_ref().map_or(0, |dag| if dag.terminal_is_empty() { dag.next_backing_rele...
1341 | |     }
     | |_____^ not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1496:5
     |
1496 |     fn next_close_byte_demand(&self) -> usize { self.active.as_ref().or(self.strings.as_ref()).map_or_else(|| self.rows.byte_demand(), artifact_retirement_box_byte_demand) }
     |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |     |  |
     |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
     |     not a member of trait `ErasedSnapshotRetirement`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1583:5
     |
1583 |       fn next_close_byte_demand(&self) -> usize {
     |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
     |  _____|
     | |
1584 | |         self.active.as_ref().map_or(usize::from(self.edit.is_some()), artifact_retirement_box_byte_demand)
1585 | |     }
     | |_____^ not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1682:5
     |
1682 |       fn next_close_byte_demand(&self) -> usize {
     |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
     |  _____|
     | |
1683 | |         self.active.as_ref().map_or(usize::from(self.vcs.is_some()), artifact_retirement_box_byte_demand)
1684 | |     }
     | |_____^ not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1932:5
     |
1932 |       fn next_close_byte_demand(&self) -> usize {
     |       ^  ---------------------- help: there is an associated function with a similar name: `next_copy_byte_demand`
     |  _____|
     | |
1933 | |         self.active.as_ref().map_or_else(|| self.metadata.as_ref().map_or(usize::from(self.envelope.is_some()), ErasedSnapshotRe...
1934 | |     }
     | |_____^ not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1966:5
     |
1966 |     fn next_close_byte_demand(&self) -> usize { self.snapshot.as_ref().or(self.pack.as_ref()).map_or(0, artifact_retirement_box_byte_demand) }
     |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |     |  |
     |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
     |     not a member of trait `ErasedSnapshotRetirement`

error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
```

```text
error[E0407]: method `next_close_byte_demand` is not a member of trait `ErasedSnapshotRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2774:5
     |
2774 |     fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| self.owners.as_ref().map_or(0, DocumentStoreOwners::next_close_byte_demand), artifact_retirement_box_byte_demand) }
     |     ^^^----------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |     |  |
     |     |  help: there is an associated function with a similar name: `next_copy_byte_demand`
     |     not a member of trait `ErasedSnapshotRetirement`

warning: unused variable: `width`
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:65:59
     |
  65 |         if self.index.is_some() { return crate::os_store::artifact_retirement_owner_demands(&self.index, body); }
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:66:26
     |
  66 |         crate::os_store::artifact_retirement_owner_demands(&self.input, body)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:72:41
     |
  72 |             let step = crate::os_store::artifact_retirement_owner_close(&mut self.index, grant)?;
     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:75:26
     |
  75 |         crate::os_store::artifact_retirement_owner_close(&mut self.input, grant)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:436:26
     |
 436 |         crate::os_store::artifact_retirement_owner_demands(&self.owners, body)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:470:26
     |
 470 |         crate::os_store::artifact_retirement_owner_close(&mut self.owners, grant)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:531:26
     |
 531 |         crate::os_store::artifact_retirement_owner_close(&mut self.owners, grant)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:534:88
     |
 534 |     fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.own...
     |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:535:105
     |
 535 | ...esult<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.owners, body)?.capacity_bytes) }
     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:536:91
     |
 536 |     fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self....
     |                                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs:537:84
     |
 537 |     fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.owners,...
     |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:60:22
     |
  60 |     crate::os_store::artifact_retirement_owner_close(input, grant)
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:182:106
     |
 182 | ...tirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:244:106
     |
 244 | ...tirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:283:106
     |
 283 | ...tirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.owner, body) }
     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs:334:106
     |
 334 | ...tirementDemand, ValueError> { crate::os_store::artifact_retirement_owner_demands(&self.input, body) }
     |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_close` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:196:22
     |
 196 |     crate::os_store::artifact_retirement_owner_close(request, grant)
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:1
     |
1386 | pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
     | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_close_step` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:223:26
     |
 223 |         crate::os_store::artifact_retirement_owner_demands(&self.request, body)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_owner_demands` in module `crate::os_store`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs:246:26
     |
 246 |         crate::os_store::artifact_retirement_owner_demands(&self.request, body)
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:203:68
    |
203 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(snapshot));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:631:80
    |
631 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement((history, auxiliary)));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:631:80
    |
631 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement((history, auxiliary)));
    |                                                                                ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:650:80
    |
650 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement(auxiliary));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:650:80
    |
650 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement(auxiliary));
    |                                                                                ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:783:111
    |
783 | ...o_framework_value::retirement::owned_retirement(pack)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:783:111
    |
783 | ...o_framework_value::retirement::owned_retirement(pack)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_...
    |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:815:68
    |
815 |             *self.active = Some(semio_framework_value::retirement::owned_retirement((history, auxiliary)));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:815:68
    |
815 |             *self.active = Some(semio_framework_value::retirement::owned_retirement((history, auxiliary)));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find type `SnapshotRetirementStep` in this scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:91:84
   |
91 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::Va...
```

```text
error[E0425]: cannot find type `SnapshotRetirementStep` in this scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:91:84
   |
91 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::Va...
   |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: you might be missing a type parameter
   |
88 | pub trait MemberOpenOperation<SnapshotRetirementStep> {
   |                              ++++++++++++++++++++++++
```

```text
error[E0425]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:497:84
    |
497 | ...: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
    |                                             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: you might be missing a type parameter
    |
492 | impl<P, M, SnapshotRetirementStep> ErasedSnapshotRetirement for MemberStoreOpenRetained<P, M>
    |          ++++++++++++++++++++++++
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:533:68
    |
533 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(history));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:562:68
     |
 562 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:562:68
     |
 562 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
     |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:19:25
     |
  19 |     let demand = super::artifact_retirement_box_byte_demand(active.as_ref().unwrap());
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:195:224
     |
 195 | ...ap(|owner| (owner.terminal_is_empty(), super::artifact_retirement_box_byte_demand(owner))), self.runtime.as_ref().map(|owner| (...
     |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0422]: cannot find struct, variant or union type `Change` in module `crate::os_store`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:616:51
    |
616 | ...   let change = crate::os_store::Change { id: std::mem::take(&mut change.id), edit_ids: std::mem::take(&mut change.edit_ids), de...
    |                                     ^^^^^^ not found in `crate::os_store`
    |
help: consider importing this struct through its public re-export
    |
  3 + use crate::Change;
    |
```

```text
error[E0422]: cannot find struct, variant or union type `Checkpoint` in module `crate::os_store`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:634:59
    |
634 | ...   let checkpoint = crate::os_store::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.cha...
    |                                         ^^^^^^^^^^ not found in `crate::os_store`
    |
note: variant `crate::space_history::io::sqlite::snapshot::preflight::borrowed::Node::Checkpoint` exists but is inaccessible
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs:12:68
    |
 12 |  Root(&'a SpaceHistorySnapshot),Checkpoints(&'a [SpaceCheckpoint]),Checkpoint(&'a SpaceCheckpoint),
```

```text
error[E0422]: cannot find struct, variant or union type `Author` in module `crate::os_store`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:641:156
    |
641 | ...ains indexed").authors.push(crate::os_store::Author { id: std::mem::take(&mut author.id), name: std::mem::take(&mut author.name)...
    |                                                 ^^^^^^ not found in `crate::os_store`
    |
note: these variants exist but are inaccessible
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/📇️directory/🚪️io/📝️text/🦀️.rs:26:5
    |
 26 |     Author {
```

```text
error[E0422]: cannot find struct, variant or union type `Alternative` in module `crate::os_store`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:658:56
    |
658 | ...   let alternative = crate::os_store::Alternative { id: std::mem::take(&mut alternative.id), name: std::mem::take(&mut alternati...
    |                                          ^^^^^^^^^^^ not found in `crate::os_store`
    |
note: variant `crate::space_history::io::sqlite::snapshot::preflight::borrowed::Node::Alternative` exists but is inaccessible
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs:15:39
    |
 15 |  Alternatives(&'a [SpaceAlternative]),Alternative(&'a SpaceAlternative),
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:698:76
    |
698 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(previous));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:763:80
    |
763 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement((entry, previous)));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:763:80
    |
763 |                         *self.active = Some(semio_framework_value::retirement::owned_retirement((entry, previous)));
    |                                                                                ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:853:85
    |
853 |                     } else { *self.active = Some(semio_framework_value::retirement::owned_retirement(retired_strings)); }
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:853:85
    |
853 |                     } else { *self.active = Some(semio_framework_value::retirement::owned_retirement(retired_strings)); }
    |                                                                                     ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:858:76
    |
858 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(self.pending_target.take().expect("finished decoded t...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:858:76
    |
858 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(self.pending_target.take().expect("finished decoded t...
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:891:76
    |
891 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold alia...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:891:76
    |
891 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold alia...
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:896:76
    |
896 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:896:76
    |
896 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:900:115
    |
900 | ...o_framework_value::retirement::owned_retirement(pins)); cx.consume_fuel(1); return PersistedDocumentHydrationStep::Pending(self....
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:900:115
    |
900 | ...o_framework_value::retirement::owned_retirement(pins)); cx.consume_fuel(1); return PersistedDocumentHydrationStep::Pending(self....
    |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:902:76
    |
902 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:902:76
    |
902 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:907:76
    |
907 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:907:76
    |
907 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:996:68
     |
 996 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:996:68
     |
 996 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
     |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1024:124
     |
1024 | ...o_framework_value::retirement::owned_retirement(conflicts)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, rel...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1026:68
     |
1026 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(transitions));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1026:68
     |
1026 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(transitions));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1033:115
     |
1033 | ...o_framework_value::retirement::owned_retirement(target)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, releas...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1033:115
     |
1033 | ...o_framework_value::retirement::owned_retirement(target)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, releas...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1034:116
     |
1034 | ...o_framework_value::retirement::owned_retirement(address)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, relea...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1034:116
     |
1034 | ...o_framework_value::retirement::owned_retirement(address)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, relea...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1035:114
     |
1035 | ...o_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(source).expect("target decoder closes before its sou...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1035:114
     |
1035 | ...o_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(source).expect("target decoder closes before its sou...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1037:68
     |
1037 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(ids));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1037:68
     |
1037 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(ids));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1041:100
     |
1041 | ...o_framework_value::retirement::owned_retirement(id)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_b...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1041:100
     |
1041 | ...o_framework_value::retirement::owned_retirement(id)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_b...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1081:68
     |
1081 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold ali...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1081:68
     |
1081 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold ali...
     |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1085:68
     |
1085 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1085:68
     |
1085 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1088:107
     |
1088 | ...o_framework_value::retirement::owned_retirement(pins)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1088:107
     |
1088 | ...o_framework_value::retirement::owned_retirement(pins)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1090:68
     |
1090 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1090:68
     |
1090 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1094:68
     |
1094 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1094:68
     |
1094 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1110:68
     |
1110 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(expected));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1110:68
     |
1110 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(expected));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1114:68
     |
1114 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(owner));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1114:68
     |
1114 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(owner));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1118:68
     |
1118 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.actor.0)));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1118:68
     |
1118 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.actor.0)));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1122:68
     |
1122 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(schema));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1122:68
     |
1122 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(schema));
     |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:304:76
    |
304 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(source));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:304:76
    |
304 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(source));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:308:76
    |
308 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(source));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:308:76
    |
308 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(source));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:312:76
    |
312 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: None, ..source }));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:312:76
    |
312 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: None, ..source }));
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:316:76
    |
316 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: Some(metadata), .....
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:316:76
    |
316 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: Some(metadata), .....
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:382:76
    |
382 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(payload));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:382:76
    |
382 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(payload));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:545:76
    |
545 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("config fo...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:545:76
    |
545 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("config fo...
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:549:76
    |
549 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:549:76
    |
549 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:553:76
    |
553 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:553:76
    |
553 |                     *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
    |                                                                            ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:580:68
     |
 580 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in module `super`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:580:68
     |
 580 |         if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_byte_demand(active); }
     |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
    ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1375:1
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:615:68
    |
615 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(transitions));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:619:68
    |
619 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:619:68
    |
619 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:623:68
    |
623 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:623:68
    |
623 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:627:68
    |
627 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:627:68
    |
627 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:631:68
    |
631 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:631:68
    |
631 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:635:68
    |
635 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:635:68
    |
635 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:639:68
    |
639 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:639:68
    |
639 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:643:68
    |
643 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:643:68
    |
643 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(value));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:682:68
    |
682 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("config fo...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:682:68
    |
682 | ...   *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("config fo...
    |                                                              ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:686:68
    |
686 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:686:68
    |
686 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:690:68
    |
690 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(expected_id));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:690:68
    |
690 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(expected_id));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:694:68
    |
694 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.actor.0)));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:694:68
    |
694 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.actor.0)));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:698:68
    |
698 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(schema));
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:698:68
    |
698 |             *self.active = Some(semio_framework_value::retirement::owned_retirement(schema));
    |                                                                    ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1340:162
     |
1340 |         self.active.as_ref().map_or_else(|| self.dag.as_ref().map_or(0, |dag| if dag.terminal_is_empty() { dag.next_backing_release_byte_demand() } else { 1 }), artifact_retirement_box_byte_demand)
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1340:162
     |
1340 |         self.active.as_ref().map_or_else(|| self.dag.as_ref().map_or(0, |dag| if dag.terminal_is_empty() { dag.next_backing_release_byte_demand() } else { 1 }), artifact_retirement_box_byte_demand)
     |                                                                                                                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1483:160
     |
1483 | ...o_framework_value::retirement::owned_retirement(strings)); self }
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1492:111
     |
1492 | ...o_framework_value::retirement::owned_retirement(identity)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, rele...
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1492:111
     |
1492 | ...o_framework_value::retirement::owned_retirement(identity)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, rele...
     |                                   ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1496:136
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1496:136
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
1496 |     fn next_close_byte_demand(&self) -> usize { self.active.as_ref().or(self.strings.as_ref()).map_or_else(|| self.rows.byte_demand(), artifact_retirement_box_byte_demand) }
     |                                                                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1584:71
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
1584 |         self.active.as_ref().map_or(usize::from(self.edit.is_some()), artifact_retirement_box_byte_demand)
     |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1683:70
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
1683 |         self.active.as_ref().map_or(usize::from(self.vcs.is_some()), artifact_retirement_box_byte_demand)
     |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1933:164
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
1933 |         self.active.as_ref().map_or_else(|| self.metadata.as_ref().map_or(usize::from(self.envelope.is_some()), ErasedSnapshotRetirement::next_close_byte_demand), artifact_retirement_box_byte_demand)
     |                                                                                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `shared_lease_retirement` in module `semio_framework_value::retirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1956:144
     |
1956 | ...amework_value::retirement::shared_lease_retirement(pack)) }
     |                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1966:105
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1966:105
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
1966 |     fn next_close_byte_demand(&self) -> usize { self.snapshot.as_ref().or(self.pack.as_ref()).map_or(0, artifact_retirement_box_byte_demand) }
     |                                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2122:79
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
2122 |     fn next_close_byte_demand(&self) -> usize { self.owners.front().map_or(0, artifact_retirement_box_byte_demand) }
     |                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2365:243
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
2365 |         self.active.as_ref().map_or_else(|| if self.phase == ArtifactStoreCursorDisposerPhase::Displaced { store.displaced_retirements.next_close_byte_demand() } else { usize::from(self.phase != ArtifactStoreCursorDisposerPhase::Complete) }, artifact_retirement_box_byte_demand)
     |                                                                                                                                                                                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2774:162
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
2774 |     fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| self.owners.as_ref().map_or(0, DocumentStoreOwners::next_close_byte_demand), artifact_retirement_box_byte_demand) }
     |                                                                                                                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find value `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9279:54
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
9279 |         Ok(self.active_retirement.as_ref().map_or(0, artifact_retirement_box_byte_demand))
     |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `owned_retirement` in module `semio_framework_value::retirement`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10775:71
      |
10775 |             self.retirement = Some(semio_framework_value::retirement::owned_retirement(capture.bytes));
      |                                                                       ^^^^^^^^^^^^^^^^ not found in `semio_framework_value::retirement`

error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21551:56
      |
21551 |         self.apply_command(vec![mutation], lane, None, identity).await?;
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21551:56
      |
21551 |         self.apply_command(vec![mutation], lane, None, identity).await?;
      |                                                        ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23126:114
      |
23126 |             let (transition, dependencies, checkpoint_id) = self.pending_checkpoint_transition(None, Vec::new(), identity).await?;
      |                                                                                                                  ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23133:30
      |
23133 |         let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
      |                              ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27608:94
      |
27608 |         self.dispatch(ArtifactCommand::CommitCheckpoint { message: Some(message), authors }, identity).await?;
      |                                                                                              ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27624:121
      |
27624 | ... alternative_id: alternative_id.to_string() }, identity).await.map(|_| ());
      |                                                   ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27627:105
      |
27627 |         self.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }, identity).await.map(|_| ())
      |                                                                                                         ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27631:68
      |
27631 |         self.dispatch(ArtifactCommand::CreateAlternative { name }, identity).await?;
      |                                                                    ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27664:46
      |
27664 |         self.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ())
      |                                              ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error[E0425]: cannot find value `identity` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27668:46
      |
27668 |         self.dispatch(ArtifactCommand::Redo, identity).await.map(|_| ())
      |                                              ^^^^^^^^ not found in this scope
      |
note: these functions exist but are inaccessible
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:17:1
      |
   17 | fn identity(prefix:&str,length:usize,control:&mut NativeEncodeControl<'_>,project:impl FnOnce(&mut IdentityInput<'_,'_>)->Result<(),ValueError>)->Result<String,ValueError>{
```

```text
error: could not compile `semio-framework-ui` (lib) due to 22 previous errors; 40 warnings emitted
warning: build failed, waiting for other jobs to finish...
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30339:23
      |
30339 | ...   fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_val...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30339:23
      |
30339 | ...   fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_val...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
30339 -         fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30338:5
      |
30338 |     impl<T: Send> ErasedSnapshotRetirement for RoundTripValueRetirement<T> {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30358:25
      |
30358 |         fn retire_owned(&self, value: T) -> Box<dyn ErasedSnapshotRetirement> {
      |                         ^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire_owned` from trait: `fn(&Self, T, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
help: add the missing parameter from the trait
      |
30358 |         fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0046]: not all trait items implemented, missing: `retirement_birth_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30357:5
      |
30357 |     impl<T: Send + 'static> ArtifactOwnedValueRetirementFactory<T> for RoundTripValueRetirementFactory {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `retirement_birth_bytes` in implementation
      |
      = help: implement the missing item: `fn retirement_birth_bytes(&self, _: &T) -> usize { todo!() }`

error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28665:19
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28665:19
      |
28665 |     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
28665 -     fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28664:1
      |
28664 | impl<T: Send> ErasedSnapshotRetirement for SpaceHistoryOwnedRetirement<T> {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28686:15
      |
28686 |     fn retire(&self, snapshot: Arc<SpaceHistorySnapshot>) -> Box<dyn ErasedSnapshotRetirement> {
      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire` from trait: `fn(&Self, std::sync::Arc<P>, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
help: add the missing parameter from the trait
      |
28686 |     fn retire(&self, snapshot: Arc<SpaceHistorySnapshot>, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28695:21
      |
28695 |     fn retire_owned(&self, value: SpaceHistorySnapshot) -> Box<dyn ErasedSnapshotRetirement> {
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire_owned` from trait: `fn(&Self, T, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
help: add the missing parameter from the trait
      |
28695 |     fn retire_owned(&self, value: SpaceHistorySnapshot, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0046]: not all trait items implemented, missing: `retirement_birth_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28694:1
      |
28694 | impl ArtifactOwnedValueRetirementFactory<SpaceHistorySnapshot> for SpaceHistoryOwnedValueRetirementFactory {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `retirement_birth_bytes` in implementation
      |
      = help: implement the missing item: `fn retirement_birth_bytes(&self, _: &os_store::component::SpaceHistorySnapshot) -> usize { todo!() }`

error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28701:21
```

```text
error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28701:21
      |
28701 |     fn retire_owned(&self, value: SpaceHistoryMutation) -> Box<dyn ErasedSnapshotRetirement> {
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire_owned` from trait: `fn(&Self, T, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
help: add the missing parameter from the trait
      |
28701 |     fn retire_owned(&self, value: SpaceHistoryMutation, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0046]: not all trait items implemented, missing: `retirement_birth_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28700:1
      |
28700 | impl ArtifactOwnedValueRetirementFactory<SpaceHistoryMutation> for SpaceHistoryOwnedValueRetirementFactory {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `retirement_birth_bytes` in implementation
      |
      = help: implement the missing item: `fn retirement_birth_bytes(&self, _: &space_history_mutations::SpaceHistoryMutation) -> usize { todo!() }`

error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30329:19
```

```text
error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30329:19
      |
30329 |         fn retire(&self, snapshot: Arc<P>) -> Box<dyn ErasedSnapshotRetirement> {
      |                   ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire` from trait: `fn(&Self, std::sync::Arc<P>, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
help: add the missing parameter from the trait
      |
30329 |         fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11724:19
      |
11724 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
11724 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11723:1
      |
11723 | impl<T: Send + 'static> ErasedSnapshotRetirement for BoundedArtifactValueRetirement<T> {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `retire_owned` has 2 parameters but the declaration in trait `retire_owned` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11763:21
      |
11763 |     fn retire_owned(&self, value: T) -> Box<dyn ErasedSnapshotRetirement> {
      |                     ^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire_owned` from trait: `fn(&Self, T, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
help: add the missing parameter from the trait
      |
11763 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0046]: not all trait items implemented, missing: `retirement_birth_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11762:1
      |
11762 | impl<T: Send + 'static> ArtifactOwnedValueRetirementFactory<T> for BoundedArtifactRetirementFactory<T> {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `retirement_birth_bytes` in implementation
      |
      = help: implement the missing item: `fn retirement_birth_bytes(&self, _: &T) -> usize { todo!() }`

error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11771:15
```

```text
error[E0050]: method `retire` has 2 parameters but the declaration in trait `retire` has 3
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11771:15
      |
11771 |     fn retire(&self, snapshot: Arc<T>) -> Box<dyn ErasedSnapshotRetirement> {
      |               ^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters, found 2
      |
      = note: `retire` from trait: `fn(&Self, std::sync::Arc<P>, RetainedCloneGrant) -> Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
help: add the missing parameter from the trait
      |
11771 |     fn retire(&self, snapshot: Arc<T>, grant: RetainedCloneGrant) -> Box<dyn ErasedSnapshotRetirement> {
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8872:19
     |
8872 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
8872 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8867:1
     |
8867 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>
8868 | | where
8869 | |     P: Send,
8870 | |     Mutation: Send,
     | |___________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_job::InteractiveJob::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9892:19
     |
9892 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> InteractiveJobCloseStep`
help: remove the extra parameter to match the trait
     |
9892 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10016:19
      |
10016 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
10016 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10008:1
      |
10008 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected<P, Mutation>
10009 | | where
10010 | |     P: Send,
10011 | |     Mutation: Send,
      | |___________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10115:19
      |
10115 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
10115 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10110:1
      |
10110 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation>
10111 | | where
10112 | |     P: Send,
10113 | |     Mutation: Send,
      | |___________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:173:19
    |
173 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::V...
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
    |
173 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:172:1
    |
172 | impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for PresenceStoreRetirement<P> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
    = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2775:19
     |
2775 |     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
2775 -     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2771:1
     |
2771 | / impl<P, M> ErasedSnapshotRetirement for UninstalledDocumentEnvelopeRetirement<P, M>
2772 | | where P: Clone + ToValue + FromValue + Send + Sync + 'static, M: Clone + ToValue + FromValue + self::Mutation<P> + Send + 'static,
     | |__________________________________________________________________________________________________________________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1486:19
     |
1486 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1486 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1485:1
     |
1485 | impl<Mu: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreOperationRowsRetirement<Mu> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1558:19
     |
1558 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1558 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1557:1
     |
1557 | impl<Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreDecodedEditRetirement<Mutation> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1605:19
     |
1605 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1605 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1600:1
     |
1600 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactStoreVcsRetirement<P, Mutation>
1601 | | where
1602 | |     P: Send + Sync + 'static,
1603 | |     Mutation: Send + 'static,
     | |_____________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1766:19
     |
1766 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1766 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1756:1
     |
1756 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactStoreEnvelopeRetirement<P, Mutation>
1757 | | where
1758 | |     P: Send + Sync + 'static,
1759 | |     Mutation: Send + 'static,
     | |_____________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1961:19
     |
1961 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1961 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1960:1
     |
1960 | impl ErasedSnapshotRetirement for ArtifactGenesisRetirement {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1142:19
     |
1142 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1142 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1141:1
     |
1141 | impl ErasedSnapshotRetirement for ArtifactStoreEditRetirement {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1236:19
     |
1236 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1236 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1235:1
     |
1235 | impl ErasedSnapshotRetirement for ArtifactStoreHistoryMetadataRetirement {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1309:19
     |
1309 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1309 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1308:1
     |
1308 | impl ErasedSnapshotRetirement for ArtifactStoreMutationDagRetirement {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
     |
     = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
     = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18459:19
      |
18459 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
18459 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18454:1
      |
18454 | / impl<P, Mutation> ErasedSnapshotRetirement for ArtifactStoreBatchPublication<P, Mutation>
18455 | | where
18456 | |     P: Send + Sync + 'static,
18457 | |     Mutation: Send + 'static,
      | |_____________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18624:19
      |
18624 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
18624 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18623:1
      |
18623 | impl ErasedSnapshotRetirement for ArtifactStorePendingReportRetirement {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:586:19
    |
586 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::V...
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
    |
586 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:573:1
    |
573 | / impl<P, M> ErasedSnapshotRetirement for RetainedConfigStoreHydration<P, M>
574 | | where
575 | |     P: Clone + ToValue + FromValue + Send + Sync + 'static,
576 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
    | |______________________________________________________________________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
    = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1005:19
     |
1005 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::...
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
     |
     = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
     |
1005 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:989:1
    |
989 | / impl<P, M> ErasedSnapshotRetirement for RetainedPersistedDocumentHydration<P, M>
990 | | where
991 | |     P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
992 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
    | |______________________________________________________________________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
    = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:497:19
    |
497 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::V...
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
    |
497 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:492:1
    |
492 | / impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained<P, M>
493 | | where
494 | |     P: Clone + super::ToValue + super::FromValue + Send + Sync + 'static,
495 | |     M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
    | |_______________________________________________________________________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
    = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:766:19
    |
766 |     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
    |
766 -     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:760:1
    |
760 | / impl<F, P, M> ErasedSnapshotRetirement for InitialMemberStoreOpen<F, P, M>
761 | | where
762 | |     F: MemberFactory + 'static,
763 | |     P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactComposition...
764 | |     M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
    | |______________________________________________________________________________________^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:59:19
   |
59 |     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
   |
   = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
   |
59 -     fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:58:1
   |
58 | impl<P: Send> ErasedSnapshotRetirement for UnsupportedMemberSnapshotOpen<P> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
   |
   = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
   = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
   = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
   = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:181:19
    |
181 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::V...
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
    |
    = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
    |
181 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:180:1
    |
180 | impl<P: semio_framework_value::retirement::RetireOwned> ErasedSnapshotRetirement for PackMemberSnapshotOpen<P> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
    |
    = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
    = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27707:14
      |
27707 |         self.dispatch_binary(cmd_bytes).await
      |              ^^^^^^^^^^^^^^^----------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23645:18
      |
23645 |     pub async fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandRe...
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27716:27
      |
27716 |         let result = self.dispatch_binary(cmd_bytes).await;
      |                           ^^^^^^^^^^^^^^^----------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:23645:18
      |
23645 |     pub async fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandRe...
```

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28848:29
      |
28848 |         let checkpoint_id = content_addressed_entity_id("space-checkpoint", &space_checkpoint_payload).await;
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------------------------- argument #3 of type `&mut NativeEncodeControl<'_>` is missing
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:51:8
      |
   51 | pub fn content_addressed_entity_id(prefix:&str,payload:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let...
```

```text
error[E0277]: `Result<std::string::String, semio_framework_value::ValueError>` is not a future
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28848:104
      |
28848 |         let checkpoint_id = content_addressed_entity_id("space-checkpoint", &space_checkpoint_payload).await;
      |                             -------------------------------------------------------------------------- ^^^^^ `Result<std::string::String, semio_framework_value::ValueError>` is not a future
      |                             |
      |                             this call returns `Result<std::string::String, semio_framework_value::ValueError>`
      |
      = help: the trait `Future` is not implemented for `Result<std::string::String, semio_framework_value::ValueError>`
      = note: Result<std::string::String, semio_framework_value::ValueError> must be a future or must implement `IntoFuture` to be awaited
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28851:19
      |
28851 | ...ta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint })], transaction: None }).aw...
      |       ^^^^^^^^-------------------------------------------------------------------------------------------------------------------------------------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28852:19
      |
28852 |         self.meta.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await?;
      |                   ^^^^^^^^-------------------------------------------------------------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28863:30
      |
28863 |         let alternative_id = content_addressed_entity_id("space-alternative", &space_alternative_payload).await;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------------------------- argument #3 of type `&mut NativeEncodeControl<'_>` is missing
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:51:8
      |
   51 | pub fn content_addressed_entity_id(prefix:&str,payload:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let...
```

```text
error[E0277]: `Result<std::string::String, semio_framework_value::ValueError>` is not a future
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28863:107
      |
28863 |         let alternative_id = content_addressed_entity_id("space-alternative", &space_alternative_payload).await;
      |                              ---------------------------------------------------------------------------- ^^^^^ `Result<std::string::String, semio_framework_value::ValueError>` is not a future
      |                              |
      |                              this call returns `Result<std::string::String, semio_framework_value::ValueError>`
      |
      = help: the trait `Future` is not implemented for `Result<std::string::String, semio_framework_value::ValueError>`
      = note: Result<std::string::String, semio_framework_value::ValueError> must be a future or must implement `IntoFuture` to be awaited
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28865:19
      |
28865 | ...ta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative })], transaction: None }).aw...
      |       ^^^^^^^^----------------------------------------------------------------------------------------------------------------------------------------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:28887:19
      |
28887 | ...ta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: alternative_id.to_string() })], transaction: None }).aw...
      |       ^^^^^^^^------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29358:5
      |
29358 |     content_addressed_entity_id("child", &payload).await
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------- argument #3 of type `&mut NativeEncodeControl<'_>` is missing
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:51:8
      |
   51 | pub fn content_addressed_entity_id(prefix:&str,payload:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let...
```

```text
error[E0277]: `Result<std::string::String, semio_framework_value::ValueError>` is not a future
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29358:52
      |
29358 |     content_addressed_entity_id("child", &payload).await
      |     ---------------------------------------------- ^^^^^ `Result<std::string::String, semio_framework_value::ValueError>` is not a future
      |     |
      |     this call returns `Result<std::string::String, semio_framework_value::ValueError>`
      |
      = help: the trait `Future` is not implemented for `Result<std::string::String, semio_framework_value::ValueError>`
      = note: Result<std::string::String, semio_framework_value::ValueError> must be a future or must implement `IntoFuture` to be awaited
```

```text
error[E0061]: this function takes 3 arguments but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29378:5
      |
29378 |     content_addressed_entity_id("invocation", &payload).await
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------ argument #3 of type `&mut NativeEncodeControl<'_>` is missing
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🦀️.rs:51:8
      |
   51 | pub fn content_addressed_entity_id(prefix:&str,payload:&[u8],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{let...
```

```text
error[E0277]: `Result<std::string::String, semio_framework_value::ValueError>` is not a future
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29378:57
      |
29378 |     content_addressed_entity_id("invocation", &payload).await
      |     --------------------------------------------------- ^^^^^ `Result<std::string::String, semio_framework_value::ValueError>` is not a future
      |     |
      |     this call returns `Result<std::string::String, semio_framework_value::ValueError>`
      |
      = help: the trait `Future` is not implemented for `Result<std::string::String, semio_framework_value::ValueError>`
      = note: Result<std::string::String, semio_framework_value::ValueError> must be a future or must implement `IntoFuture` to be awaited
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29988:15
      |
29988 |         store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("apply");
      |               ^^^^^^^^-------------------------------------------------------------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29990:15
      |
29990 |         store.dispatch(ArtifactCommand::Undo).await.expect("undo");
      |               ^^^^^^^^----------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:29992:15
      |
29992 |         store.dispatch(ArtifactCommand::Redo).await.expect("redo");
      |               ^^^^^^^^----------------------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20812:18
      |
20812 |     pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Co...
```

```text
error[E0050]: method `close_step` has 3 parameters but the declaration in trait `semio_framework_value::ErasedSnapshotRetirement::close_step` has 2
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26942:19
      |
26942 |     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value:...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
      |
      = note: `close_step` from trait: `fn(&mut Self, RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>`
help: remove the extra parameter to match the trait
      |
26942 -     fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
```

```text
error[E0046]: not all trait items implemented, missing: `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26941:1
      |
26941 | impl ErasedSnapshotRetirement for ArtifactStoreBackboneRetirement {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `next_copy_byte_demand`, `next_capacity_byte_demand`, `next_release_byte_demand`, `next_depth_demand` in implementation
      |
      = help: implement the missing item: `fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
      = help: implement the missing item: `fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { todo!() }`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:30279:13
      |
30279 | ...   ErasedSnapshotRetirement::close_step(&mut retirement, 1, 4_096).expect("parsed document retires within its exact grant");
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                  -  ----- unexpected argument #3 of type `{integer}`
      |                                                             |
      |                                                             expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs:138:22
    |
138 |             if grant.maximum_bytes < demand { return Ok(GroupOwnsStep::Blocked); }
    |                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
138 -             if grant.maximum_bytes < demand { return Ok(GroupOwnsStep::Blocked); }
138 +             if grant.maximum_items < demand { return Ok(GroupOwnsStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs:166:22
    |
166 |             if grant.maximum_bytes < demand { return Ok(GroupOwnsStep::Blocked); }
    |                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
166 -             if grant.maximum_bytes < demand { return Ok(GroupOwnsStep::Blocked); }
166 +             if grant.maximum_items < demand { return Ok(GroupOwnsStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs:192:22
    |
192 |             if grant.maximum_bytes < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
    |                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
192 -             if grant.maximum_bytes < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
192 +             if grant.maximum_items < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs:200:18
    |
200 |         if grant.maximum_bytes < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
    |                  ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
200 -         if grant.maximum_bytes < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
200 +         if grant.maximum_items < preparation.next_byte_demand(self) { return Ok(GroupOwnsStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs:234:18
    |
234 |         if grant.maximum_bytes < demand { return SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }; }
    |                  ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
234 -         if grant.maximum_bytes < demand { return SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }; }
234 +         if grant.maximum_items < demand { return SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }; }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27411:18
      |
27411 |         if grant.maximum_bytes < birth_bytes { return Ok(None); }
      |                  ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27411 -         if grant.maximum_bytes < birth_bytes { return Ok(None); }
27411 +         if grant.maximum_items < birth_bytes { return Ok(None); }
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27439:135
      |
27439 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27439:156
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27439:156
      |
27439 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27439 -         self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())
27439 +         self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items }).map_err(|error| error.to_string())
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27469:141
      |
27469 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())? {
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27469:162
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27469:162
      |
27469 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())? {
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27469 -         match self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| error.to_string())? {
27469 +         match self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items }).map_err(|error| error.to_string())? {
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27523:103
      |
27523 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes })
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27523:124
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27523:124
      |
27523 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27523 -         publication.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })
27523 +         publication.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items })
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27532:42
      |
27532 | ...   if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Err("prepared member r...
      |                                        ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27532 -         if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Err("prepared member read lease allocation is not funded".into()); }
27532 +         if !grant.permits_one() || grant.maximum_items < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return Err("prepared member read lease allocation is not funded".into()); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:27545:42
      |
27545 | ...   if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return reject(snapshot, "prep...
      |                                        ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
27545 -         if !grant.permits_one() || grant.maximum_bytes < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return reject(snapshot, "prepared read return whole lease allocation is not funded"); }
27545 +         if !grant.permits_one() || grant.maximum_items < ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { return reject(snapshot, "prepared read return whole lease allocation is not funded"); }
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:22345:62
      |
22345 |             self.displaced_retirements.push_reserved(factory.retire(Arc::new(unique)));
      |                                                              ^^^^^^------------------ argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:22345:54
      |
22345 |             self.displaced_retirements.push_reserved(factory.retire(Arc::new(unique)));
      |                                        ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                        |
      |                                        arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21126:97
      |
21126 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes };
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21126:118
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21126:118
      |
21126 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes };
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21126 -         let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes };
21126 +         let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items };
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21163:46
      |
21163 |                     let preparation = source.begin_next(request).map_err(VcsError::ValidationFailed)?;
      |                                              ^^^^^^^^^^--------- argument #2 of type `os_store::component::ArtifactStoreOneItemGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17788:8
      |
17788 |     fn begin_next(&mut self, request: ArtifactStoreBatchItemRequest<P>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn Arti...
```

```text
error[E0631]: type mismatch in function arguments
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21163:74
      |
21163 |                     let preparation = source.begin_next(request).map_err(VcsError::ValidationFailed)?;
      |                                                                  ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected due to this
      |                                                                  |
      |                                                                  required by a bound introduced by this call
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🦀️.rs:1549:5
      |
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21164:52
      |
21164 |                     publication.preparation = Some(preparation);
      |                                               ---- ^^^^^^^^^^^ expected `Box<_>`, found `(Box<_>, _)`
      |                                               |
      |                                               arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn os_store::component::ArtifactStoreOneItemPreparation<P, Mutation>>`
                  found tuple `(Box<dyn os_store::component::ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress)`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21192:42
      |
21192 |                         if bytes > grant.maximum_bytes {
      |                                          ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21192 -                         if bytes > grant.maximum_bytes {
21192 +                         if bytes > grant.maximum_items {
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21195:90
      |
21195 | ...   let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(...
      |                                                                        ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21195 -                         let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
21195 +                         let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_items).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21196:77
      |
21196 |                         if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
      |                                                                             ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21196 -                         if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
21196 +                         if !step.progressed || step.allocated_bytes > grant.maximum_items {
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21377:61
      |
21377 | ...   let (step, released) = spend_close_byte_grant(grant.maximum_bytes, |remaining| owner.close_step(ArtifactStoreOneItemGrant {...
      |                                                           ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21377 -         let (step, released) = spend_close_byte_grant(grant.maximum_bytes, |remaining| owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_bytes: remaining }))?;
21377 +         let (step, released) = spend_close_byte_grant(grant.maximum_items, |remaining| owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_bytes: remaining }))?;
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21377:169
      |
21377 | ...ant.maximum_items, maximum_bytes: remaining }))?;
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21419:105
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21419:105
      |
21419 | ...   staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| ...
      |                                                                                                   ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21419 -             staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
21419 +             staged.edit.inverse = semio_framework_value::list::PagedList::with_payload_page_bytes(grant.maximum_items).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21433:30
      |
21433 |             if bytes > grant.maximum_bytes {
      |                              ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21433 -             if bytes > grant.maximum_bytes {
21433 +             if bytes > grant.maximum_items {
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21436:127
      |
21436 | ...city_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footpr...
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21436 -             let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
21436 +             let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_items) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21436:226
      |
21436 | ...city_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into...
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21436 -             let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_bytes) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
21436 +             let step = if publication.transaction_open { inverse.reserve_capacity_one(publication.footprint.work_items, grant.maximum_bytes) } else { inverse.reserve_exact_capacity_one(publication.footprint.work_items, grant.maximum_items) }.map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21437:65
      |
21437 |             if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
      |                                                                 ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
21437 -             if !step.progressed || step.allocated_bytes > grant.maximum_bytes {
21437 +             if !step.progressed || step.allocated_bytes > grant.maximum_items {
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21499:66
      |
21499 |                 self.displaced_retirements.push_reserved(factory.retire(displaced));
      |                                                                  ^^^^^^----------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:21499:58
      |
21499 |                 self.displaced_retirements.push_reserved(factory.retire(displaced));
      |                                            ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                            |
      |                                            arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19513:47
      |
19513 |         Ok(owner.take().map(|mutation|factory.retire_owned(mutation)))
      |                                               ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
      |
   28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19513:12
      |
19513 |         Ok(owner.take().map(|mutation|factory.retire_owned(mutation)))
      |         -- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<Box<_>>`, found `Option<Result<(Box<_>, _), _>>`
      |         |
      |         arguments to this enum variant are incorrect
      |
      = note: expected enum `Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>`
                 found enum `Option<Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>>`
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19521:53
      |
19521 | ...kets.0[0] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
      |
   35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retai...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19521:53
      |
19521 | ...tickets.0[0] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
      |                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                   |
      |                   arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19572:111
      |
19572 |             self.displaced_retirements.push_reserved(factory.expect("non-tail current factory was validated").retire(previous));
      |                                                                                                               ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19572:54
      |
19572 |             self.displaced_retirements.push_reserved(factory.expect("non-tail current factory was validated").retire(previous));
      |                                        ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                        |
      |                                        arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19600:67
      |
19600 |         self.displaced_retirements.push_reserved(snapshot_factory.retire(previous_current));
      |                                                                   ^^^^^^------------------ argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19600:50
      |
19600 |         self.displaced_retirements.push_reserved(snapshot_factory.retire(previous_current));
      |                                    ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                    |
      |                                    arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19630:77
      |
19630 |                 self.displaced_retirements.push_reserved(authority.snapshot.retire(snapshot));
      |                                                                             ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19630:58
      |
19630 |                 self.displaced_retirements.push_reserved(authority.snapshot.retire(snapshot));
      |                                            ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                            |
      |                                            arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19637:69
      |
19637 |         self.displaced_retirements.push_reserved(authority.snapshot.retire(previous_current));
      |                                                                     ^^^^^^------------------ argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19637:50
      |
19637 |         self.displaced_retirements.push_reserved(authority.snapshot.retire(previous_current));
      |                                    ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                    |
      |                                    arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19891:118
      |
19891 | ...shot factory was validated").retire(snapshot));
      |                                 ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19891:54
      |
19891 | ..._retirements.push_reserved(factory.expect("nonshared tail snapshot factory was validated").retire(snapshot));
      |                 ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                 |
      |                 arguments to this method are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for struct `ManuallyDrop<FactoryChildTickets<6>>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19950:110
      |
19950 | ...ry_retirement_tickets.next_close_byte_demand(); }
      |                          ^^^^^^^^^^^^^^^^^^^^^^ method not found in `ManuallyDrop<FactoryChildTickets<6>>`
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
              candidate #1: `member_open::MemberOpenOperation`
              candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:19985:60
      |
19985 |                 let step = self.factory_retirement_tickets.close_step(maximum_items, maximum_bytes);
      |                                                            ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                                                       |
      |                                                                       expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:43:12
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20116:83
      |
20116 |         Ok(edit.inverse.pop().or_else(|| edit.forwards.pop()).map(|value| factory.retire_owned(value)))
      |                                                                                   ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
      |
   28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20116:12
      |
20116 |         Ok(edit.inverse.pop().or_else(|| edit.forwards.pop()).map(|value| factory.retire_owned(value)))
      |         -- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<Box<_>>`, found `Option<Result<(Box<_>, _), _>>`
      |         |
      |         arguments to this enum variant are incorrect
      |
      = note: expected enum `Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>`
                 found enum `Option<Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20255:36
      |
20255 |             return Ok(Some(factory.retire(entry.snapshot)));
      |                                    ^^^^^^---------------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20255:28
      |
20255 |             return Ok(Some(factory.retire(entry.snapshot)));
      |                       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                       |
      |                       arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20276:63
      |
20276 |         Ok(ArtifactStoreSnapshotRootClose::Retirement(factory.retire(snapshot)))
      |                                                               ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20276:55
      |
20276 |         Ok(ArtifactStoreSnapshotRootClose::Retirement(factory.retire(snapshot)))
      |            ------------------------------------------ ^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |            |
      |            arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20300:25
      |
20300 |         Ok(Some(factory.retire(snapshot)))
      |                         ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20300:17
      |
20300 |         Ok(Some(factory.retire(snapshot)))
      |            ---- ^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |            |
      |            arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18095:75
      |
18095 |             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
      |                                                                           ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18095 -             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
18095 +             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_items)?;
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18095:30
      |
18095 |             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
      |                              ^^^^^^^^^^ --------------------------  ------------------- unexpected argument #2
      |                                         |
      |                                         expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18110:42
      |
18110 |             self.retiring = Some(factory.retire_owned(value));
      |                                          ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
      |
   28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18110:34
      |
18110 |             self.retiring = Some(factory.retire_owned(value));
      |                             ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                             |
      |                             arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18119:42
      |
18119 |             self.retiring = Some(factory.retire(post));
      |                                          ^^^^^^------ argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
      |
   23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProg...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18119:34
      |
18119 |             self.retiring = Some(factory.retire(post));
      |                             ---- ^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                             |
      |                             arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn os_store::component::ArtifactStoreOneItemPreparation<P, Mutation>>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18204:149
      |
18204 | ...td::mem::size_of_val(owner.as_ref()) } else { owner.next_close_byte_demand() }; }
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
              candidate #1: `member_open::MemberOpenOperation`
              candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18206:201
      |
 1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
      | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
18206 |         if let Some(stage) = self.stage.as_ref() { return if stage.terminal_is_empty() { std::mem::size_of::<ArtifactStoreBatchStage<P, Mutation>>() } else { stage.retiring.as_ref().map_or(1, |owner| artifact_retirement_box_byte_demand(ow...
      |                                                                                                                                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18209:74
      |
 1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
      | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
18209 |         if let Some(owner) = self.authority_retirement.as_ref() { return artifact_retirement_box_byte_demand(owner); }
      |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: a function with a similar name exists
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18327:46
      |
18327 | ...   if grant.maximum_items == 0 || grant.maximum_bytes < self.next_close_byte_demand() { return Ok(SnapshotRetirementStep::Pend...
      |                                            ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18327 -         if grant.maximum_items == 0 || grant.maximum_bytes < self.next_close_byte_demand() { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
18327 +         if grant.maximum_items == 0 || grant.maximum_items < self.next_close_byte_demand() { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18329:112
      |
18329 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18329:133
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18329:133
      |
18329 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18329 -             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
18329 +             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18337:39
      |
18337 | ...   if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 ...
      |                                 ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18337 -             if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
18337 +             if released_bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18343:113
      |
18343 | ...imum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
      |                       ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18343:134
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18343:134
      |
18343 | ...grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18343 -             let step = source.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
18343 +             let step = source.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18351:39
      |
18351 | ...   if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 ...
      |                                 ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18351 -             if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
18351 +             if released_bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18364:39
      |
18364 | ...   if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 ...
      |                                 ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18364 -             if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
18364 +             if released_bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18380:50
      |
18380 |             let step = owner.close_step(1, grant.maximum_bytes)?;
      |                                                  ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18380 -             let step = owner.close_step(1, grant.maximum_bytes)?;
18380 +             let step = owner.close_step(1, grant.maximum_items)?;
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18380:30
      |
18380 |             let step = owner.close_step(1, grant.maximum_bytes)?;
      |                              ^^^^^^^^^^ -  ------------------- unexpected argument #2
      |                                         |
      |                                         expected `RetainedCloneGrant`, found integer
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18388:39
      |
18388 | ...   if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 ...
      |                                 ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
18388 -             if released_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
18388 +             if released_bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18393:56
      |
18393 |             self.authority_retirement = Some(authority.retire());
      |                                                        ^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17549:12
      |
17549 |     pub fn retire(self: Arc<Self>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSna...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18393:46
      |
18393 |             self.authority_retirement = Some(authority.retire());
      |                                         ---- ^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                                         |
      |                                         arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18460:100
      |
18460 |         ArtifactStoreBatchPublication::close_step(self, ArtifactStoreOneItemGrant { maximum_items, maximum_bytes })
      |                                                                                                    ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
      |
      = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18629:33
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18629:33
      |
18629 |             return match active.close_step(maximum_items, maximum_bytes)? {
      |                                 ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                            |
      |                                            expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17848:148
      |
17848 | ...nt { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_r...
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17848 -                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
17848 +                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_items.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17848:201
      |
17848 | ...mum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: sel...
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17848 -                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
17848 +                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_items, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17848:245
      |
17848 | ...rant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlle...
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17848 -                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_bytes, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
17848 +                 let step = retirement.step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.maximum_bytes.min(64), maximum_capacity_bytes: grant.maximum_bytes, maximum_release_bytes: grant.maximum_items, maximum_depth: self.issuer.as_ref().expect("original controlled source issuer").maximum_depth })?;
```

```text
error[E0609]: no field `maximum_depth` on type `&SourceRetirementIssuer<<A as ArtifactStoreBatchItemAuthority<P, Mutation>>::Input>`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17848:340
      |
17848 | ...ef().expect("original controlled source issuer").maximum_depth })?;
      |                                                     ^^^^^^^^^^^^^ unknown field
      |
      = note: available fields are: `birth_bytes`, `begin`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17853:30
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17853:30
      |
17853 |             if bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
      |                              ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17853 -             if bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
17853 +             if bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17859:43
      |
17859 | ...   if issuer.birth_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes...
      |                                     ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17859 -             if issuer.birth_bytes > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
17859 +             if issuer.birth_bytes > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17860:36
      |
17860 |             self.retirement = Some((issuer.begin)(std::mem::take(&mut self.inputs)));
      |                                    ^^^^^^^^^^^^^^---------------------------------- argument #2 of type `RetainedCloneGrant` is missing
      |
help: provide the argument
      |
17860 |             self.retirement = Some((issuer.begin)(std::mem::take(&mut self.inputs), /* RetainedCloneGrant */));
      |                                                                                   ++++++++++++++++++++++++++
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17860:36
      |
17860 |             self.retirement = Some((issuer.begin)(std::mem::take(&mut self.inputs)));
      |                               ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn BatchRetirement>`, found `Result<(Box<dyn BatchRetirement>, _), _>`
      |                               |
      |                               arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn BatchRetirement>`
                   found enum `Result<(Box<dyn BatchRetirement>, RetainedCloneProgress), (_, _)>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17866:32
      |
17866 | ...   if backing > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
      |                          ^^^^^^^^^^^^^ unknown field
      |
help: a field with a similar name exists
      |
17866 -             if backing > grant.maximum_bytes { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
17866 +             if backing > grant.maximum_items { return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17875:119
      |
17875 | ...tirement.as_ref()) } else { retirement.demands().map_or(usize::MAX, |(capacity, release)| capacity.max(release).max(retirement...
      |                                           ^^^^^^^-- argument #1 of type `usize` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs:9:8
      |
    9 |     fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17875:149
      |
17875 | ....demands().map_or(usize::MAX, |(capacity, release)| capacity.max(release).max(retirement.next_copy_byte_demand())) };
      |                                   ^^^^^^^^^^^^^^^^^^^
      |                                   |
      |                                   expected `RetirementDemand`, found `(_, _)`
      |                                   expected due to this
      |
      = note: expected struct `RetirementDemand`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:911:78
    |
911 |             return match active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)? {
    |                                                                              ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
911 -             return match active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)? {
911 +             return match active.close_step(grant.maximum_items.min(1), grant.maximum_items)? {
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:911:33
    |
911 |             return match active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)? {
    |                                 ^^^^^^^^^^ --------------------------  ------------------- unexpected argument #2
    |                                            |
    |                                            expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:912:134
    |
912 | ...if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released...
    |                                                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
912 -                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
912 +                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_items => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:925:56
    |
925 |             let released_bytes = bytes.len().min(grant.maximum_bytes);
    |                                                        ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
925 -             let released_bytes = bytes.len().min(grant.maximum_bytes);
925 +             let released_bytes = bytes.len().min(grant.maximum_items);
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:940:132
    |
940 | ...snapshot retirement authority").retire(post));
    |                                    ^^^^^^------ argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
    |
 23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:940:43
    |
940 | ...rement = Some(self.snapshot_retirement.as_ref().expect("sealer retains snapshot retirement authority").retire(post));
    |             ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |             |
    |             arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:948:53
      |
  948 |             self.active_retirement = Some(authority.retire());
      |                                                     ^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17549:12
      |
17549 |     pub fn retire(self: Arc<Self>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSna...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:948:43
    |
948 |             self.active_retirement = Some(authority.retire());
    |                                      ---- ^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                                      |
    |                                      arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:805:33
    |
805 |         let mut maximum = grant.maximum_bytes.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
    |                                 ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
805 -         let mut maximum = grant.maximum_bytes.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
805 +         let mut maximum = grant.maximum_items.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:63:53
   |
63 |             return match active.close_step(1, grant.maximum_bytes)? {
   |                                                     ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
63 -             return match active.close_step(1, grant.maximum_bytes)? {
63 +             return match active.close_step(1, grant.maximum_items)? {
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:63:33
   |
63 |             return match active.close_step(1, grant.maximum_bytes)? {
   |                                 ^^^^^^^^^^ -  ------------------- unexpected argument #2
   |                                            |
   |                                            expected `RetainedCloneGrant`, found integer
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:71:134
   |
71 | ... if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released...
   |                                                       ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
71 -                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
71 +                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_items => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:77:108
   |
77 |             self.active = Some(self.retirement.as_ref().expect("reader retains root retirement authority").retire(root));
   |                                                                                                            ^^^^^^------ argument #2 of type `RetainedCloneGrant` is missing
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
   |
23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:77:32
    |
 77 |             self.active = Some(self.retirement.as_ref().expect("reader retains root retirement authority").retire(root));
    |                           ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                           |
    |                           arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<T>)>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs:92:29
   |
92 |         let maximum = grant.maximum_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
   |                             ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
92 -         let maximum = grant.maximum_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
92 +         let maximum = grant.maximum_items.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn semio_framework_value::ErasedSnapshotRetirement>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16551:165
      |
16551 | ...::mem::size_of_val(active.as_ref()) } else { active.next_close_byte_demand() })
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
      = help: items from traits can only be used if the trait is implemented and in scope
      = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
              candidate #1: `member_open::MemberOpenOperation`
              candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16568:22
      |
16568 |         match active.close_step(maximum_items.min(1), maximum_bytes)? {
      |                      ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
      |                                 |
      |                                 expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16757:113
      |
16757 | ...ve = Some(current_factory.retire_owned(current)); }
      |                              ^^^^^^^^^^^^--------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
      |
   28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:16757:97
      |
16757 | ... = Some(current_factory.retire_owned(current)); }
      |       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |       |
      |       arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:12067:37
      |
12067 |                 let applied = store.dispatch_apply_exact(mutations).await;
      |                                     ^^^^^^^^^^^^^^^^^^^^----------- argument #2 of type `&mut EntityIdentityAuthority<'_>` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20828:14
      |
20828 |     async fn dispatch_apply_exact(&mut self, mutations: Vec<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<Comm...
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9912:80
     |
9912 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9912:99
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9912:99
     |
9912 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9921:84
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9921:84
     |
9921 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
     |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9921:100
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9921:100
     |
9921 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
     |                                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9934:80
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9934:80
     |
9934 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9934:99
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9934:99
     |
9934 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9942:76
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9942:76
     |
9942 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
     |                                                                            ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9942:95
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9942:95
     |
9942 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
     |                                                                                               ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9954:80
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9954:80
     |
9954 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9954:99
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9954:99
     |
9954 |                 return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
     |                                                                                                   ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9958:84
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9958:84
     |
9958 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
     |                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9958:100
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9958:100
     |
9958 |                     return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
     |                                                                                                    ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9966:76
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_items`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9966:76
     |
9966 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
     |                                                                            ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9966:95
```

```text
error[E0559]: variant `InteractiveJobCloseStep::Pending` has no field named `released_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9966:95
     |
9966 |             return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
     |                                                                                               ^^^^^^^^^^^^^^ `InteractiveJobCloseStep::Pending` does not have this field
     |
     = note: available fields are: `progress`

error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9973:71
```

```text
error[E0533]: expected value, found struct variant `semio_framework_job::InteractiveJobCloseStep::Complete`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9973:71
     |
9973 | ...ty::terminal_is_empty(self) { semio_framework_job::InteractiveJobCloseStep::Complete } else { semio_framework_job::InteractiveJ...
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a value
     |
help: you might have meant to create a new value of the struct
     |
9973 |         if ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self) { semio_framework_job::InteractiveJobCloseStep::Complete { progress: /* value */ } } else { semio_framework_job::InteractiveJobCloseStep::Blocked }
     |                                                                                                                              +++++++++++++++++++++++++
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10244:26
      |
10244 |         match retirement.close_step(maximum_items, maximum_bytes)? {
      |                          ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
      |                                     |
      |                                     expected `RetainedCloneGrant`, found `usize`
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10765:9
      |
10765 |         artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| Self::diagnostic("arti...
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                       -------------  ------------- unexpected argument #3 of type `usize`
      |                                                                  |
      |                                                                  expected `RetainedCloneGrant`, found `usize`
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10780:66
      |
10780 |             self.retirement = Some(self.initial_snapshot_factory.retire_owned(snapshot));
      |                                                                  ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
      |
   28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10780:36
      |
10780 |             self.retirement = Some(self.initial_snapshot_factory.retire_owned(snapshot));
      |                               ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
      |                               |
      |                               arguments to this enum variant are incorrect
      |
      = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                   found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:10904:26
      |
 1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
      | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
10904 |             let demand = artifact_retirement_box_byte_demand(retirement);
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11298:26
      |
 1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
      | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
11298 |             let demand = artifact_retirement_box_byte_demand(retirement);
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: a function with a similar name exists
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:11365:54
      |
11365 | ...e() { return artifact_retirement_box_close_step(&mut self.active_retirement, maximum_items, maximum_bytes).map_err(|_| Self::d...
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                              -------------  ------------- unexpected argument #3 of type `usize`
      |                                                                                 |
      |                                                                                 expected `RetainedCloneGrant`, found `usize`
      |
note: function defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8380:72
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
8380 |         if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
     |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8444:47
     |
8444 | ..._some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagno...
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                       -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                              |
     |                                                                              expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8446:61
     |
8446 |             *self.retirement = Some(self.retirement_factory.retire_owned(value));
     |                                                             ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8446:37
     |
8446 |             *self.retirement = Some(self.retirement_factory.retire_owned(value));
     |                                ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                |
     |                                arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, protocol::Edit<Mutation>)>`
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8547:72
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
8547 |         if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
     |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8586:47
     |
8586 | ..._some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagno...
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                       -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                              |
     |                                                                              expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8588:61
     |
8588 |             *self.retirement = Some(self.retirement_factory.retire_owned(value));
     |                                                             ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8588:37
     |
8588 |             *self.retirement = Some(self.retirement_factory.retire_owned(value));
     |                                ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                |
     |                                arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8916:19
     |
8916 |             match ErasedSnapshotRetirement::close_step(&mut retired, maximum_items.min(1), maximum_bytes) {
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^               --------------------  ------------- unexpected argument #3 of type `usize`
     |                                                                      |
     |                                                                      expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9402:76
     |
9402 | ...path; return artifact_retirement_box_close_step(&mut self.active_retirement, maximum_items, maximum_bytes).map_err(|_| OwnedSch...
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                              -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                                 |
     |                                                                                 expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9409:68
     |
9409 |             *self.active_retirement = Some(self.retirement_factory.retire_owned(value));
     |                                                                    ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9409:44
     |
9409 |             *self.active_retirement = Some(self.retirement_factory.retire_owned(value));
     |                                       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                       |
     |                                       arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9414:72
     |
9414 |                 *self.active_retirement = Some(self.retirement_factory.retire_owned(value));
     |                                                                        ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:9414:48
     |
9414 |                 *self.active_retirement = Some(self.retirement_factory.retire_owned(value));
     |                                           ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                           |
     |                                           arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, T)>`
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:7991:72
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
7991 |         if let Some(retirement) = self.retirement.as_ref() { return Ok(artifact_retirement_box_byte_demand(retirement)); }
     |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8171:59
     |
8171 |             *self.retirement = Some(self.mutation_factory.retire_owned(value));
     |                                                           ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8171:37
     |
8171 |             *self.retirement = Some(self.mutation_factory.retire_owned(value));
     |                                ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                |
     |                                arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8174:47
     |
8174 | ..._some() { return artifact_retirement_box_close_step(&mut self.retirement, maximum_items, maximum_bytes).map_err(|_| self.diagno...
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                       -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                              |
     |                                                                              expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8177:63
     |
8177 |                 *self.retirement = Some(self.mutation_factory.retire_owned(value));
     |                                                               ^^^^^^^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:8177:41
     |
8177 |                 *self.retirement = Some(self.mutation_factory.retire_owned(value));
     |                                    ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                    |
     |                                    arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5549:33
     |
5549 |             return match active.close_step(maximum_items, maximum_bytes)? {
     |                                 ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
     |                                            |
     |                                            expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5572:46
     |
5572 |             *self.active = Some(self.factory.retire(snapshot));
     |                                              ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
     |
  23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgr...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5572:33
     |
5572 |             *self.active = Some(self.factory.retire(snapshot));
     |                            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                            |
     |                            arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:17:28
   |
17 |         return match owner.close_step(1, maximum_bytes)? {
   |                            ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
   |                                       |
   |                                       expected `RetainedCloneGrant`, found integer
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:35:36
   |
35 |             *active = Some(factory.retire(root));
   |                                    ^^^^^^------ argument #2 of type `RetainedCloneGrant` is missing
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
   |
23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:35:28
    |
 35 |             *active = Some(factory.retire(root));
    |                       ---- ^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                       |
    |                       arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:91:31
   |
91 |             let step = active.close_step(1, maximum_bytes)?;
   |                               ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
   |                                          |
   |                                          expected `RetainedCloneGrant`, found integer
   |
note: method defined here
  --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:118:127
    |
118 | ...etains its installed factory").retire(local));
    |                                   ^^^^^^------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
    |
 23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs:118:39
    |
118 | ..._local = Some(self.local_factory.as_ref().expect("detached local root retains its installed factory").retire(local));
    |             ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |             |
    |             arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5836:81
     |
5836 |                 publication.displaced_root_retirement = Some(retirement_factory.retire(previous));
     |                                                                                 ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
     |
  23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgr...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5836:62
     |
5836 |                 publication.displaced_root_retirement = Some(retirement_factory.retire(previous));
     |                                                         ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                                         |
     |                                                         arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:6194:157
     |
6194 | ...ement factory".to_string())?.retire(previous)
     |                                 ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
     |
  23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgr...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:6194:21
     |
6194 | ...   publication.root_retirement_factory.as_ref().ok_or_else(|| "transient publication lost its local-root retirement factory".to_string())?.retire(previous)
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:6304:33
     |
6304 |             return match active.close_step(1, maximum_bytes)? {
     |                                 ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
     |                                            |
     |                                            expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5017:109
     |
5017 | ...ximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
     |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
     |
     = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5017:130
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5017:130
     |
5017 | ... grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
5017 -             let step = owner.advance(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
5017 +             let step = owner.advance(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items })?;
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5068:112
     |
5068 | ...ximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
     |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
     |
     = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5068:133
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5068:133
     |
5068 | ... grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
5068 -             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes })?;
5068 +             let step = owner.close_step(ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5080:144
     |
5080 | ...ement, factory, grant.maximum_items.min(1), grant.maximum_bytes)?;
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
5080 -             let step = advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, grant.maximum_items.min(1), grant.maximum_bytes)?;
5080 +             let step = advance_returned_snapshot_read(registry, &mut self.returned_read_retirement, factory, grant.maximum_items.min(1), grant.maximum_items)?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5086:75
     |
5086 |             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
     |                                                                           ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
5086 -             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
5086 +             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_items)?;
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5086:30
     |
5086 |             let step = owner.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
     |                              ^^^^^^^^^^ --------------------------  ------------------- unexpected argument #2
     |                                         |
     |                                         expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:5107:42
     |
5107 |             if scalar.len_utf8() > grant.maximum_bytes {
     |                                          ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
5107 -             if scalar.len_utf8() > grant.maximum_bytes {
5107 +             if scalar.len_utf8() > grant.maximum_items {
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1771:20
     |
1771 |             return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes);
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                         |
     |                                                                         expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1778:29
     |
1778 |             return metadata.close_step(maximum_items, maximum_bytes);
     |                             ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
     |                                        |
     |                                        expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1788:67
     |
1788 |                         *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                                                                   ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1788:45
     |
1788 |                         *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                                        ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                        |
     |                                        arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0782]: expected a type, found a trait
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1933:113
     |
1933 | ...ref().map_or(usize::from(self.envelope.is_some()), ErasedSnapshotRetirement::next_close_byte_demand), artifact_retirement_box_b...
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: you can add the `dyn` keyword if you want a trait object
     |
1933 |         self.active.as_ref().map_or_else(|| self.metadata.as_ref().map_or(usize::from(self.envelope.is_some()), <dyn ErasedSnapshotRetirement>::next_close_byte_demand), artifact_retirement_box_byte_demand)
     |                                                                                                                 ++++                         +
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1963:9
     |
1963 |         artifact_retirement_box_close_step(slot, maximum_items.min(1), maximum_bytes)
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^       --------------------  ------------- unexpected argument #3 of type `usize`
     |                                                  |
     |                                                  expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1986:28
     |
1986 |         return match owner.close_step(maximum_items.min(1), maximum_bytes)? {
     |                            ^^^^^^^^^^ --------------------  ------------- unexpected argument #2 of type `usize`
     |                                       |
     |                                       expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2105:21
     |
2105 |         match owner.close_step(maximum_items, maximum_bytes)? {
     |                     ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
     |                                |
     |                                expected `RetainedCloneGrant`, found `usize`
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2342:58
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
2342 |             if self.active.as_ref().is_some_and(|active| artifact_retirement_box_byte_demand(active) > remaining) {
     |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0425]: cannot find function `artifact_retirement_box_byte_demand` in this scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2343:295
     |
1375 | pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
     | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- similarly named function `artifact_retirement_box_demands` defined here
...
2343 |                 { use std::sync::atomic::{AtomicUsize, Ordering}; static N: AtomicUsize = AtomicUsize::new(0); if N.fetch_add(1, Ordering::Relaxed) < 3 { eprintln!("[DEBUG] store cursor needs funding: demand={} remaining={remaining} phase={:?} type={}", self.active.as_ref().map_or(0, |active| artifact_retirement_box_byte_demand(ac...
     |                                                                                                                                                                                                                                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: a function with a similar name exists
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2347:21
     |
2347 |             let r = artifact_retirement_box_close_step(&mut self.active, maximum_items.min(1), remaining);
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   --------------------  --------- unexpected argument #3 of type `usize`
     |                                                                          |
     |                                                                          expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2626:29
     |
2626 | ...ts = [Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(snapshot_retirement.clone())), Some(semio_frame...
     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2626:134
     |
2626 | ...)), Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(initial_snapshot_retirement.clone())), Some(semio...
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------------------------------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2626:247
     |
2626 | ...())), Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(mutation_retirement.clone())), None, None, None];
     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2627:460
     |
2627 | ...e, factory_retirement_tickets: semio_framework_value::FactoryChildTickets(tickets) }
     |                                   ------------------------------------------ ^^^^^^^ expected `[Option<Box<_>>; 6]`, found `[Option<Result<(Box<_>, _), _>>; 6]`
     |                                   |
     |                                   arguments to this struct are incorrect
     |
     = note: expected array `[std::option::Option<Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>>; 6]`
                found array `[Option<Result<(Box<dyn ErasedSnapshotRetirement>, _), _>>; 6]`
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2645:53
     |
2645 | ...kets.0[4] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2645:53
     |
2645 | ...tickets.0[4] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                   |
     |                   arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2652:53
     |
2652 | ...kets.0[3] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2652:53
     |
2652 | ...tickets.0[3] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                   |
     |                   arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2659:53
     |
2659 | ...kets.0[5] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:35:8
     |
  35 |     fn preborn_factory_retirement(self: Arc<Self>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, Retain...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2659:53
     |
2659 | ...tickets.0[5] = Some(semio_framework_value::FactoryRetirement::preborn_factory_retirement(factory.clone()));
     |                   ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                   |
     |                   arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2703:42
     |
2703 |         self.initial_snapshot_retirement.retire_owned(snapshot)
     |                                          ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2703:9
     |
2699 |     pub fn retire_initial_snapshot_owned(&self, snapshot: P) -> Box<dyn ErasedSnapshotRetirement>
     |                                                                 --------------------------------- expected `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>` because of return type
...
2703 |         self.initial_snapshot_retirement.retire_owned(snapshot)
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |
     = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for struct `FactoryChildTickets<N>` in the current scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2716:106
     |
2716 |         if !self.factory_retirement_tickets.terminal_is_empty() { return self.factory_retirement_tickets.next_close_byte_demand(); }
     |                                                                                                          ^^^^^^^^^^^^^^^^^^^^^^ method not found in `FactoryChildTickets<6>`
     |
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
             candidate #1: `member_open::MemberOpenOperation`
             candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2743:106
     |
2743 | ...lf.factory_retirement_tickets.close_step(1, maximum_bytes); }
     |                                  ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
     |                                             |
     |                                             expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🏭️factory/🦀️.rs:43:12
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2776:43
     |
2776 |         if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, items, bytes); }
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -----  ----- unexpected argument #3 of type `usize`
     |                                                                                                |
     |                                                                                                expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1314:20
     |
1314 |             return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes);
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                         |
     |                                                                         expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1488:43
     |
1488 |         if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, 1, maximum_bytes); }
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -  ------------- unexpected argument #3 of type `usize`
     |                                                                                                |
     |                                                                                                expected `RetainedCloneGrant`, found integer
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1489:79
     |
1489 | ...f.active = Some(self.factory.retire_owned(row)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes:...
     |                                 ^^^^^^^^^^^^----- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1489:66
     |
1489 | ...tive = Some(self.factory.retire_owned(row)); return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }...
     |           ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |           |
     |           arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mu)>`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1491:44
     |
1491 |         if self.strings.is_some() { return artifact_retirement_box_close_step(&mut self.strings, 1, maximum_bytes); }
     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                    -  ------------- unexpected argument #3 of type `usize`
     |                                                                                                  |
     |                                                                                                  expected `RetainedCloneGrant`, found integer
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1562:43
     |
1562 |         if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes); }
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                                                |
     |                                                                                                expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1567:55
     |
1567 |             *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                                                       ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1567:33
     |
1567 |             *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                            |
     |                            arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1609:43
     |
1609 |         if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, maximum_items, maximum_bytes); }
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -------------  ------------- unexpected argument #3 of type `usize`
     |                                                                                                |
     |                                                                                                expected `RetainedCloneGrant`, found `usize`
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1618:67
     |
1618 |                         *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                                                                   ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1618:45
     |
1618 |                         *self.active = Some(self.mutation_factory.retire_owned(mutation));
     |                                        ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                                        |
     |                                        arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, Mutation)>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:52:22
   |
52 |             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
   |                      ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
52 -             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
52 +             if grant.maximum_items < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:59:22
   |
59 |             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
   |                      ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
59 -             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
59 +             if grant.maximum_items < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:70:22
   |
70 |             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
   |                      ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
70 -             if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
70 +             if grant.maximum_items < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:89:18
   |
89 |         if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
   |                  ^^^^^^^^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
89 -         if grant.maximum_bytes < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
89 +         if grant.maximum_items < group.demand { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
```

```text
error[E0599]: no method named `last_key_value` found for struct `HistoryFoldIndex<K, V>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:103:62
    |
103 |                 let next = match revision.mutation_positions.last_key_value() {
    |                                                              ^^^^^^^^^^^^^^
    |
help: there is a method `first_key_value` with a similar name
    |
103 -                 let next = match revision.mutation_positions.last_key_value() {
103 +                 let next = match revision.mutation_positions.first_key_value() {
```

```text
error[E0107]: method takes 1 generic argument but 2 generic arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:104:84
    |
104 | ...ccumulator.mutation_positions.range::<MutationId, _>((Excluded(key), Unbounded)).next(),
    |                                  ^^^^^             --- help: remove the unnecessary generic argument
    |                                  |
    |                                  expected 1 generic argument
    |
note: method defined here, with 1 generic parameter: `R`
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:63:12
```

```text
error[E0277]: the trait bound `protocol::MutationId: RangeBounds<protocol::MutationId>` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:104:92
    |
104 | ...ositions.range::<MutationId, _>((Excluded(key), Unbounded)).next(),
    |             -----   ^^^^^^^^^^ the trait `RangeBounds<protocol::MutationId>` is not implemented for `protocol::MutationId`
    |             |
    |             required by a bound introduced by this call
    |
note: required by a bound in `HistoryFoldIndex::<K, V>::range`
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:63:20
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:104:107
    |
104 | ...on_positions.range::<MutationId, _>((Excluded(key), Unbounded)).next(),
    |                 ---------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `MutationId`, found `(Bound<_>, Bound<_>)`
    |                 |
    |                 arguments to this method are incorrect
    |
    = note: expected struct `protocol::MutationId`
                found tuple `(Bound<_>, Bound<_>)`
```

```text
error[E0599]: no method named `last_key_value` found for struct `HistoryFoldIndex<K, V>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:113:57
    |
113 |                 let next = match revision.indexed_edits.last_key_value() {
    |                                                         ^^^^^^^^^^^^^^
    |
help: there is a method `first_key_value` with a similar name
    |
113 -                 let next = match revision.indexed_edits.last_key_value() {
113 +                 let next = match revision.indexed_edits.first_key_value() {
```

```text
error[E0107]: method takes 1 generic argument but 2 generic arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:114:79
    |
114 |                     Some((key, _)) => self.revision_accumulator.indexed_edits.range::<[u8; 32], _>((Excluded(key), Unbounded)).next(),
    |                                                                               ^^^^^           --- help: remove the unnecessary generic argument
    |                                                                               |
    |                                                                               expected 1 generic argument
    |
note: method defined here, with 1 generic parameter: `R`
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:63:12
```

```text
error[E0277]: the trait bound `[u8; 32]: RangeBounds<[u8; 32]>` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:114:87
    |
114 |                     Some((key, _)) => self.revision_accumulator.indexed_edits.range::<[u8; 32], _>((Excluded(key), Unbounded)).next(),
    |                                                                               -----   ^^^^^^^^ the trait `RangeBounds<[u8; 32]>` is not implemented for `[u8; 32]`
    |                                                                               |
    |                                                                               required by a bound introduced by this call
    |
note: required by a bound in `HistoryFoldIndex::<K, V>::range`
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:63:20
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:114:100
    |
114 |                     Some((key, _)) => self.revision_accumulator.indexed_edits.range::<[u8; 32], _>((Excluded(key), Unbounded)).next(),
    |                                                                               -------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `[u8; 32]`, found `(Bound<_>, Bound<_>)`
    |                                                                               |
    |                                                                               arguments to this method are incorrect
    |
    = note: expected array `[u8; 32]`
               found tuple `(Bound<_>, Bound<_>)`
```

```text
error[E0599]: no method named `last_key_value` found for struct `HistoryFoldIndex<K, V>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:123:54
    |
123 |                 let next = match revision.unit_flags.last_key_value() {
    |                                                      ^^^^^^^^^^^^^^
    |
help: there is a method `first_key_value` with a similar name
    |
123 -                 let next = match revision.unit_flags.last_key_value() {
123 +                 let next = match revision.unit_flags.first_key_value() {
```

```text
error[E0107]: method takes 1 generic argument but 2 generic arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:124:76
    |
124 | ...ion_accumulator.unit_flags.range::<([u8; 32], usize), _>((Excluded(key), Unbounded)).next(),
    |                               ^^^^^                    --- help: remove the unnecessary generic argument
    |                               |
    |                               expected 1 generic argument
    |
note: method defined here, with 1 generic parameter: `R`
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🦀️.rs:63:12
```

```text
error[E0277]: the trait bound `([u8; 32], usize): RangeBounds<([u8; 32], usize)>` is not satisfied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:124:84
     |
 124 | ...t_flags.range::<([u8; 32], usize), _>((Excluded(key), Unbounded)).next(),
     |            -----   ^^^^^^^^^^^^^^^^^ the trait `RangeBounds<([u8; 32], usize)>` is not implemented for `([u8; 32], usize)`
     |            |
     |            required by a bound introduced by this call
     |
help: the following other types implement trait `RangeBounds<T>`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/range.rs:1187:1
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:124:107
    |
124 | ..._flags.range::<([u8; 32], usize), _>((Excluded(key), Unbounded)).next(),
    |                                          ^^^^^^^^^^^^^ expected `[u8; 32]`, found `Bound<_>`
    |
    = note: expected array `[u8; 32]`
                found enum `Bound<_>`

error[E0308]: mismatched types
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:124:122
    |
124 | ...e::<([u8; 32], usize), _>((Excluded(key), Unbounded)).next(),
    |                                              ^^^^^^^^^ expected `usize`, found `Bound<_>`
    |
    = note: expected type `usize`
               found enum `Bound<_>`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:163:42
    |
163 |         if !grant.permits_one() || grant.maximum_bytes < 4096 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
    |                                          ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
163 -         if !grant.permits_one() || grant.maximum_bytes < 4096 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
163 +         if !grant.permits_one() || grant.maximum_items < 4096 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:183:18
    |
183 |         if grant.maximum_bytes < group.demand { return Ok(SnapshotRetirementStep::Blocked); }
    |                  ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
183 -         if grant.maximum_bytes < group.demand { return Ok(SnapshotRetirementStep::Blocked); }
183 +         if grant.maximum_items < group.demand { return Ok(SnapshotRetirementStep::Blocked); }
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:827:101
    |
827 |         retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(current));
    |                                                                                                     ^^^^^^--------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
    |
 23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:827:84
    |
827 |         retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(current));
    |         ---------------------- arguments to this function are incorrect            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |
    = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:799:4
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:841:105
    |
841 |             retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(snapshot));
    |                                                                                                         ^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
    |
 23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:841:88
    |
841 |             retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(snapshot));
    |             ---------------------- arguments to this function are incorrect            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |
    = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:799:4
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:845:94
      |
  845 |         retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, authority.retire());
      |                                                                                              ^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17549:12
      |
17549 |     pub fn retire(self: Arc<Self>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSna...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:845:84
    |
845 |         retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, authority.retire());
    |         ---------------------- arguments to this function are incorrect            ^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |
    = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                 found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:799:4
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:954:97
    |
954 |     retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(post_snapshot));
    |                                                                                                 ^^^^^^--------------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:23:8
    |
 23 |     fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgre...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:954:80
    |
954 |     retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, snapshot_factory.retire(post_snapshot));
    |     ---------------------- arguments to this function are incorrect            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |
    = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<P>)>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:799:4
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:960:95
      |
  960 |     retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, seal.authority.retire());
      |                                                                                               ^^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
      |
note: method defined here
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17549:12
      |
17549 |     pub fn retire(self: Arc<Self>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSna...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:960:80
    |
960 |     retain_displaced_owner(&mut store.displaced_retirements, &mut reservation, seal.authority.retire());
    |     ---------------------- arguments to this function are incorrect            ^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |
    = note: expected struct `Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>`
                 found enum `Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), _>`
note: function defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:799:4
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1379:106
     |
1379 | ...ximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_mess...
     |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
     |
     = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1379:127
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1379:127
     |
1379 | ... grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error....
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
1379 -     match owner.close_step(super::ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_message()))? {
1379 +     match owner.close_step(super::ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.maximum_items }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_message()))? {
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2000:30
     |
2000 |                     if grant.maximum_bytes < decision_bytes {
     |                              ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
2000 -                     if grant.maximum_bytes < decision_bytes {
2000 +                     if grant.maximum_items < decision_bytes {
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2014:92
     |
2014 | ...{ maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(DurableOwnedGroupDecisionError::Codec)? {
     |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
     |
     = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2014:113
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2014:113
     |
2014 | ...temGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(DurableOwnedGroupDecisionError::Codec)? {
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
2014 -                 match journal.advance(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(DurableOwnedGroupDecisionError::Codec)? {
2014 +                 match journal.advance(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_items }).map_err(DurableOwnedGroupDecisionError::Codec)? {
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2114:99
     |
2114 | ...{ maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_mess...
     |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
     |
     = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2114:120
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2114:120
     |
2114 | ...temGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error....
     |                                                      ^^^^^^^^^^^^^ unknown field
     |
help: a field with a similar name exists
     |
2114 -                     match journal.close_step(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_message()))? {
2114 +                     match journal.close_step(super::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_items }).map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into_message()))? {
```

```text
error[E0277]: the trait bound `ArtifactGroupVisibility: Copy` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/♻️retirement/🦀️.rs:70:61
    |
 70 |     fn retirement(self) -> Box<dyn RetirementCursor> { leaf(self) }
    |                                                        ---- ^^^^ unsatisfied trait bound
    |                                                        |
    |                                                        required by a bound introduced by this call
    |
help: the trait `Copy` is not implemented for `ArtifactGroupVisibility`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🦀️.rs:574:1
```

```text
error[E0277]: the trait bound `ArtifactGroupVisibility: Copy` is not satisfied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/♻️retirement/🦀️.rs:71:116
    |
 71 |     fn retirement_birth_bytes(&self) -> Option<usize> { Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>()) }
    |                                                                                                                    ^^^^ unsatisfied trait bound
    |
help: the trait `Copy` is not implemented for `ArtifactGroupVisibility`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/../../🔨️modules/🌿️vcs/🦀️.rs:574:1
    |
574 | pub struct ArtifactGroupVisibility {
```

```text
error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:93:20
   |
93 |         let edit = self.edit.begin_demand()?;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `RetainedCloneBirthDemand`
   |
   = help: the nightly-only, unstable trait `Try` is not implemented for `RetainedCloneBirthDemand`

error[E0599]: no method named `snapshot_cursor_birth_demand` found for struct `std::sync::Arc<E>` in the current scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:94:31
```

```text
error[E0599]: no method named `snapshot_cursor_birth_demand` found for struct `std::sync::Arc<E>` in the current scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:94:31
   |
94 |         let clone = self.edit.snapshot_cursor_birth_demand()?;
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `std::sync::Arc<E>`

error[E0061]: this method takes 1 argument but 0 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:130:36
    |
130 |             edit_cursor: self.edit.begin(),
```

```text
error[E0061]: this method takes 1 argument but 0 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:130:36
    |
130 |             edit_cursor: self.edit.begin(),
    |                                    ^^^^^-- argument #1 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:51:8
    |
 51 |     fn begin(&self, grant: RetainedCloneGrant) -> Result<(Self::Cursor, RetainedCloneProgress), ValueError>;
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:130:26
    |
130 |             edit_cursor: self.edit.begin(),
    |                          ^^^^^^^^^^^^^^^^^ expected `Option<_>`, found `Result<(_, _), _>`
    |
    = note: expected enum `std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
               found enum `Result<(<E as RetainedCloneEdit<P, M>>::Cursor, RetainedCloneProgress), semio_framework_value::ValueError>`

error[E0063]: missing fields `edit`, `mutation_owner`, `pending_base` and 2 other fields in initializer of `RetainedClonePreparation<P, M, E>`
```

```text
error[E0063]: missing fields `edit`, `mutation_owner`, `pending_base` and 2 other fields in initializer of `RetainedClonePreparation<P, M, E>`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:125:22
    |
125 |         Ok((Box::new(RetainedClonePreparation::<P, M, E> {
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `edit`, `mutation_owner`, `pending_base` and 2 other fields

error[E0599]: no method named `terminal_is_empty` found for enum `std::option::Option<T>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:241:30
    |
241 |         if !self.edit_cursor.terminal_is_empty() { typed!(&self.edit_cursor); }
```

```text
error[E0599]: no method named `terminal_is_empty` found for enum `std::option::Option<T>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:241:30
    |
241 |         if !self.edit_cursor.terminal_is_empty() { typed!(&self.edit_cursor); }
    |                              ^^^^^^^^^^^^^^^^^ method not found in `std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
    |
note: the method `terminal_is_empty` exists on the type `<E as RetainedCloneEdit<P, M>>::Cursor`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:43:5
    |
 43 |     fn terminal_is_empty(&self) -> bool;
```

```text
error[E0599]: no method named `next_close_copy_byte_demand` found for reference `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:234:80
    |
234 | ...pr) => { return Ok(RetirementDemand { copy_bytes: $value.next_close_copy_byte_demand()?, capacity_bytes: $value.next_close_capac...
    |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
...
241 | ...cursor.terminal_is_empty() { typed!(&self.edit_cursor); }
    |                                 ------------------------- in this macro invocation
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0599]: no method named `next_close_capacity_byte_demand` found for reference `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:234:135
    |
234 | ...ementDemand { copy_bytes: $value.next_close_copy_byte_demand()?, capacity_bytes: $value.next_close_capacity_byte_demand(body)?, ...
    |                                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
...
241 | ...() { typed!(&self.edit_cursor); }
    |         ------------------------- in this macro invocation
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0599]: no method named `next_close_release_byte_demand` found for reference `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:234:197
    |
234 | ...entDemand { copy_bytes: $value.next_close_copy_byte_demand()?, capacity_bytes: $value.next_close_capacity_byte_demand(body)?, release_bytes: $value.next_close_release_byte_demand()?...
    |                                                                                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
...
241 | ... { typed!(&self.edit_cursor); }
    |       ------------------------- in this macro invocation
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0599]: no method named `next_close_depth_demand` found for reference `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:234:246
    |
234 | ...entDemand { copy_bytes: $value.next_close_copy_byte_demand()?, capacity_bytes: $value.next_close_capacity_byte_demand(body)?, release_bytes: $value.next_close_release_byte_demand()?, depth: $value.next_close_depth_demand()?...
    |                                                                                                                                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
...
241 | ... { typed!(&self.edit_cursor); }
    |       ------------------------- in this macro invocation
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:191:22
    |
191 |         match active.close_step(1, maximum_bytes) {
    |                      ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                 |
    |                                 expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0599]: no method named `next_step_byte_demand` found for reference `&HistoryFoldJob<'_, (HistoryFold, Vec<MutationEnvelope>, _, _)>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:210:71
    |
210 |             Phase::Fold => self.fold_job.as_ref().map_or(1, |job| job.next_step_byte_demand(1)),
    |                                                                       ^^^^^^^^^^^^^^^^^^^^^
    |
help: there is a method `next_copy_byte_demand` with a similar name, but with different arguments
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:14:5
    |
 14 |     fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
```

```text
error[E0599]: no method named `next_step_byte_demand` found for mutable reference `&mut HistoryFoldJob<'static, (HistoryFold, Vec<_>, _, _)>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:246:24
    |
246 | ...   if job.next_step_byte_demand(1) > crate::os_store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES { return self.reject(ConfigStoreHyd...
    |              ^^^^^^^^^^^^^^^^^^^^^
    |
help: there is a method `next_copy_byte_demand` with a similar name, but with different arguments
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:14:5
    |
 14 |     fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
```

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:247:34
    |
247 |                 let result = job.step(1, maximum_bytes, &mut || false);
    |                                  ^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                       |
    |                                       expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🦀️.rs:150:12
```

```text
error[E0599]: no method named `terminal_is_empty` found for enum `std::option::Option<T>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:433:30
    |
433 |         if !self.edit_cursor.terminal_is_empty() { return self.edit_cursor.close_step(grant); }
    |                              ^^^^^^^^^^^^^^^^^ method not found in `std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
    |
note: the method `terminal_is_empty` exists on the type `<E as RetainedCloneEdit<P, M>>::Cursor`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:43:5
    |
 43 |     fn terminal_is_empty(&self) -> bool;
```

```text
error[E0599]: no method named `close_step` found for enum `std::option::Option<T>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:433:76
    |
433 |         if !self.edit_cursor.terminal_is_empty() { return self.edit_cursor.close_step(grant); }
    |                                                                            ^^^^^^^^^^ method not found in `std::option::Option<<E as RetainedCloneEdit<P, M>>::Cursor>`
    |
note: the method `close_step` exists on the type `<E as RetainedCloneEdit<P, M>>::Cursor`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:38:5
    |
 38 |     fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>;
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:476:144
    |
476 | ...itial_snapshot_retirement.retire_owned(displaced));
    |                              ^^^^^^^^^^^^----------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:476:45
    |
476 | ... = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
    |       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |       |
    |       arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:168:78
    |
168 |     if progress.items > grant.maximum_items.min(1) || progress.bytes > grant.maximum_bytes {
    |                                                                              ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
168 -     if progress.items > grant.maximum_items.min(1) || progress.bytes > grant.maximum_bytes {
168 +     if progress.items > grant.maximum_items.min(1) || progress.bytes > grant.maximum_items {
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:484:140
    |
484 | ...itial_snapshot_retirement.retire_owned(validation));
    |                              ^^^^^^^^^^^^------------ argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:484:41
    |
484 | ... = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(validation));
    |       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |       |
    |       arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:259:67
    |
259 |         let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
    |                                                                   ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
    |
    = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:508:144
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:508:144
    |
508 | ...itial_snapshot_retirement.retire_owned(displaced));
    |                              ^^^^^^^^^^^^----------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔁️replay/🎮️operation/🦀️.rs:318:43
    |
318 | ...   if demand > grant.maximum_bytes { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
    |                         ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
318 -                         if demand > grant.maximum_bytes { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
318 +                         if demand > grant.maximum_items { return Err(VcsError::ValidationFailed("replay inverse page exceeds its body grant".into())); }
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:508:45
    |
508 | ... = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
    |       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |       |
    |       arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0631]: type mismatch in function arguments
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs:14:88
     |
  14 |  pub(crate) fn messages(&self)->&[MutationMessage]{self.messages.as_deref().map_or(&[],Vec::as_slice)}
     |                                                                             ------     ^^^^^^^^^^^^^
     |                                                                             |          |
     |                                                                             |          expected due to this
     |                                                                             |          found signature defined here
     |                                                                             required by a bound introduced by this call
     |
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:109:114
    |
109 | ...{ maximum_items: 1, maximum_bytes: grant.maximum_bytes })?;
    |                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
    |
    = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:109:135
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:109:135
    |
109 | ...temGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })?;
    |                                                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
109 -         let step = task.advance(base.as_ref(), &mut self.mutation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })?;
109 +         let step = task.advance(base.as_ref(), &mut self.mutation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_items })?;
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:118:83
    |
118 |                 || next.completed_bytes - self.checkpoint.completed_bytes > grant.maximum_bytes as u64
    |                                                                                   ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
118 -                 || next.completed_bytes - self.checkpoint.completed_bytes > grant.maximum_bytes as u64
118 +                 || next.completed_bytes - self.checkpoint.completed_bytes > grant.maximum_items as u64
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:169:57
    |
169 |             return match retirement.close_step(1, grant.maximum_bytes)? {
    |                                                         ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
169 -             return match retirement.close_step(1, grant.maximum_bytes)? {
169 +             return match retirement.close_step(1, grant.maximum_items)? {
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:169:37
    |
169 |             return match retirement.close_step(1, grant.maximum_bytes)? {
    |                                     ^^^^^^^^^^ -  ------------------- unexpected argument #2
    |                                                |
    |                                                expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:175:132
    |
175 | ...} if released_items > 1 || released_bytes > grant.maximum_bytes => {
    |                                                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
175 -                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_bytes => {
175 +                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_items => {
```

```text
error[E0560]: struct `os_store::component::ArtifactStoreOneItemGrant` has no field named `maximum_bytes`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:182:88
    |
182 |             return match task.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })? {
    |                                                                                        ^^^^^^^^^^^^^ `os_store::component::ArtifactStoreOneItemGrant` does not have this field
    |
    = note: available fields are: `maximum_copy_bytes`, `maximum_capacity_bytes`, `maximum_release_bytes`, `maximum_depth`

error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:182:109
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:182:109
    |
182 |             return match task.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })? {
    |                                                                                                             ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
182 -             return match task.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_bytes })? {
182 +             return match task.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_items })? {
```

```text
error[E0609]: no field `maximum_bytes` on type `os_store::component::ArtifactStoreOneItemGrant`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:188:132
    |
188 | ...} if released_items > 1 || released_bytes > grant.maximum_bytes => {
    |                                                      ^^^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists
    |
188 -                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_bytes => {
188 +                 SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > grant.maximum_items => {
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:195:62
    |
195 |             *self.retirement = Some(self.mutation_retirement.retire_owned(mutation));
    |                                                              ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&HistoryFoldJob<'_, (HistoryFold, Vec<MutationEnvelope>, _, _)>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:581:64
    |
581 |         if let Some(job) = self.fold_job.as_ref() { return job.next_close_byte_demand(); }
    |                                                                ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs:195:37
    |
195 |             *self.retirement = Some(self.mutation_retirement.retire_owned(mutation));
    |                                ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                                |
    |                                arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, M)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:595:33
    |
595 |             return match active.close_step(1, maximum_bytes)? {
    |                                 ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                            |
    |                                            expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:608:23
    |
608 |             match job.close_step(1, maximum_bytes)? {
    |                       ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                  |
    |                                  expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:666:68
    |
666 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(validation));
    |                                                                    ^^^^^^^^^^^^------------ argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs:666:33
    |
666 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(validation));
    |                            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                            |
    |                            arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:64:23
    |
 64 |         match request.close_step(items, bytes)? {
    |                       ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                  |
    |                                  expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:282:12
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:499:23
    |
499 |             return Ok(SnapshotRetirementStep::Complete);
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:505:43
     |
 505 |         if self.active.is_some() { return super::artifact_retirement_box_close_step(&mut self.active, 1, maximum_bytes); }
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:505:43
     |
 505 |         if self.active.is_some() { return super::artifact_retirement_box_close_step(&mut self.active, 1, maximum_bytes); }
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -  ------------- unexpected argument #3 of type `usize`
     |                                                                                                       |
     |                                                                                                       expected `RetainedCloneGrant`, found integer
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0599]: no associated function or constant named `next_close_byte_demand` found for struct `MemberOpenRequest` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:80:100
    |
 80 |     fn next_close_byte_demand(&self) -> usize { self.request.as_ref().map_or(0, MemberOpenRequest::next_close_byte_demand) }
    |                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `MemberOpenRequest`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:97:1
    |
 97 | pub struct MemberOpenRequest {
    | ---------------------------- associated function or constant `next_close_byte_demand` not found for this struct
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:529:68
    |
529 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
    |                                                                    ^^^^^^^^^^^^--------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:529:33
    |
529 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
    |                            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |                            |
    |                            arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:190:33
    |
190 |             return match active.close_step(1, maximum_bytes)? {
    |                                 ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                            |
    |                                            expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:537:34
    |
537 |             return match request.close_step(1, maximum_bytes)? {
    |                                  ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                             |
    |                                             expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:282:12
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:554:12
    |
554 |         Ok(SnapshotRetirementStep::Complete)
    |            ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:207:34
    |
207 |             return match request.close_step(1, maximum_bytes)? {
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:207:34
    |
207 |             return match request.close_step(1, maximum_bytes)? {
    |                                  ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                             |
    |                                             expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:282:12
```

```text
error[E0599]: no associated function or constant named `next_close_byte_demand` found for struct `MemberOpenRequest` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:565:175
    |
 97 | pub struct MemberOpenRequest {
    | ---------------------------- associated function or constant `next_close_byte_demand` not found for this struct
...
565 |         self.request.as_ref().map_or_else(|| self.owners.as_ref().map_or(usize::from(!self.terminal), super::DocumentStoreOwners::next_close_byte_demand), MemberOpenRequest::next_close_byte_demand)
    |                                                                                                                                                                               ^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `MemberOpenRequest`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:22:11
     |
  22 |     match super::artifact_retirement_box_close_step(active, 1, maximum_bytes) {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^         -  ------------- unexpected argument #3 of type `usize`
     |                                                             |
     |                                                             expected `RetainedCloneGrant`, found integer
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn semio_framework_value::ErasedSnapshotRetirement>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:234:68
    |
234 |         if let Some(active) = self.active.as_ref() { return active.next_close_byte_demand(); }
    |                                                                    ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no associated function or constant named `next_close_byte_demand` found for struct `MemberOpenRequest` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:236:86
    |
236 |         self.request.as_ref().map_or(usize::from(!self.terminal), MemberOpenRequest::next_close_byte_demand)
    |                                                                                      ^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `MemberOpenRequest`
    |
   ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:97:1
    |
 97 | pub struct MemberOpenRequest {
    | ---------------------------- associated function or constant `next_close_byte_demand` not found for this struct
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:272:23
    |
272 |         self.snapshot.close_step(maximum_items, maximum_bytes)
    |                       ^^^^^^^^^^ -------------  ------------- unexpected argument #2 of type `usize`
    |                                  |
    |                                  expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0599]: no method named `next_step_byte_demand` found for mutable reference `&mut HistoryFoldJob<'static, (HistoryFold, Vec<_>, _, _)>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:435:88
    |
435 | ...   let Some(maximum_bytes) = hydration_fold_byte_grant(logical_bytes, job.next_step_byte_demand(logical_bytes)) else { return se...
    |                                                                              ^^^^^^^^^^^^^^^^^^^^^
    |
help: there is a method `next_copy_byte_demand` with a similar name, but with different arguments
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:14:5
    |
 14 |     fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
```

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:436:34
    |
436 |                 let result = job.step(1, maximum_bytes, &mut || cx.should_yield());
    |                                  ^^^^ -  ------------- unexpected argument #2 of type `usize`
    |                                       |
    |                                       expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🦀️.rs:150:12
```

```text
error[E0599]: no method named `next_close_byte_demand` found for struct `UnsupportedMemberSnapshotOpen<P>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:278:63
    |
 30 | pub struct UnsupportedMemberSnapshotOpen<P> {
    | ------------------------------------------- method `next_close_byte_demand` not found for this struct
...
278 |     fn next_close_byte_demand(&self) -> usize { self.snapshot.next_close_byte_demand() }
    |                                                               ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:439:22
    |
439 |         match active.close_step(1, bytes) {
    |                      ^^^^^^^^^^ -  ----- unexpected argument #2 of type `usize`
    |                                 |
    |                                 expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 3 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:824:27
    |
824 |                 match job.step(1, bytes, &mut || cx.should_yield()) {
    |                           ^^^^ -  ----- unexpected argument #2 of type `usize`
    |                                |
    |                                expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🔀️transition/🔁️fold/🦀️.rs:150:12
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:739:31
    |
739 |                 match witness.close_step(1, bytes) {
    |                               ^^^^^^^^^^ -  ----- unexpected argument #2 of type `usize`
    |                                          |
    |                                          expected `RetainedCloneGrant`, found integer
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:770:33
    |
770 |             return match active.close_step(items, bytes)? {
    |                                 ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                            |
    |                                            expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:785:36
    |
785 |             return match hydration.close_step(items.min(1), bytes)? {
    |                                    ^^^^^^^^^^ ------------  ----- unexpected argument #2 of type `usize`
    |                                               |
    |                                               expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
842 |         close_field!(witness);
    |         --------------------- in this macro invocation
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&HistoryFoldJob<'_, (HistoryFold, Vec<MutationEnvelope>, _, _)>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:997:64
    |
997 |         if let Some(job) = self.fold_job.as_ref() { return job.next_close_byte_demand(); }
    |                                                                ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
843 |         close_field!(dictionary);
    |         ------------------------ in this macro invocation
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
844 |         close_field!(selected);
    |         ---------------------- in this macro invocation
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
845 |         close_field!(selection);
    |         ----------------------- in this macro invocation
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
846 |         close_field!(history);
    |         --------------------- in this macro invocation
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&protocol::HistoryFoldJob<'_, protocol::HistoryTransition>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:999:70
    |
999 |         if let Some(job) = self.target_decoder.as_ref() { return job.next_close_byte_demand(); }
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:833:38
    |
833 |                     let step = owner.close_step(items, bytes)?;
    |                                      ^^^^^^^^^^ -----  ----- unexpected argument #2 of type `usize`
    |                                                 |
    |                                                 expected `RetainedCloneGrant`, found `usize`
...
847 |         close_field!(snapshot_open);
    |         --------------------------- in this macro invocation
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1015:20
     |
1015 |             return super::artifact_retirement_box_close_step(&mut self.active, 1, maximum_bytes);
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                   -  ------------- unexpected argument #3 of type `usize`
     |                                                                                |
     |                                                                                expected `RetainedCloneGrant`, found integer
     |
note: function defined here
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1386:8
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1018:23
     |
1018 |             match job.close_step(1, maximum_bytes)? {
     |                       ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
     |                                  |
     |                                  expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:849:135
    |
849 | ...nitial_snapshot_retirement.retire_owned(snapshot));
    |                               ^^^^^^^^^^^^---------- argument #2 of type `RetainedCloneGrant` is missing
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
    |
 28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress...
```

```text
error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:849:33
    |
849 | ... = Some(self.owners.as_ref().expect("member-open owner catalog remains retained").initial_snapshot_retirement.retire_owned(snapshot));
    |       ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
    |       |
    |       arguments to this enum variant are incorrect
    |
    = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                 found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0061]: this method takes 1 argument but 2 arguments were supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1030:23
     |
1030 | ...   match job.close_step(1, maximum_bytes)? { SnapshotRetirementStep::Complete if job.terminal_is_empty() => { self.target_decod...
     |                 ^^^^^^^^^^ -  ------------- unexpected argument #2 of type `usize`
     |                            |
     |                            expected `RetainedCloneGrant`, found integer
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0061]: this method takes 2 arguments but 1 argument was supplied
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1077:68
     |
1077 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
     |                                                                    ^^^^^^^^^^^^--------- argument #2 of type `RetainedCloneGrant` is missing
     |
note: method defined here
    --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:28:8
     |
  28 |     fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgres...
```

```text
error[E0308]: mismatched types
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1077:33
     |
1077 |             *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
     |                            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Box<dyn ErasedSnapshotRetirement>`, found `Result<(Box<_>, _), _>`
     |                            |
     |                            arguments to this enum variant are incorrect
     |
     = note: expected struct `Box<dyn semio_framework_value::ErasedSnapshotRetirement>`
                  found enum `Result<(Box<(dyn semio_framework_value::ErasedSnapshotRetirement + 'static)>, RetainedCloneProgress), (semio_framework_value::ValueError, P)>`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&Box<dyn semio_framework_value::ErasedSnapshotRetirement>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:869:68
    |
869 |         if let Some(active) = self.active.as_ref() { return active.next_close_byte_demand(); }
    |                                                                    ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&persisted_document_hydration::RetainedPersistedDocumentHydration<P, M>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:871:77
    |
871 |         if let Some(hydration) = self.hydration.as_ref() { return hydration.next_close_byte_demand(); }
    |                                                                             ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&SelectedVerifiedMemberHistory<F>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:874:67
    |
874 |         if let Some(owner) = self.witness.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                   ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&SelectedMemberHistoryDictionary<F>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:875:70
    |
875 |         if let Some(owner) = self.dictionary.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&SelectedMemberHistoryInput<F>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:876:68
    |
876 |         if let Some(owner) = self.selected.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                    ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&MemberFactorySelection<F>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:877:69
    |
877 |         if let Some(owner) = self.selection.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                     ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&MemberHistoryVerification` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:878:67
    |
878 |         if let Some(owner) = self.history.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                   ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0599]: no method named `next_close_byte_demand` found for reference `&<P as os_store::component::MemberStoreOwner<M>>::SnapshotOpen` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:879:73
    |
879 |         if let Some(owner) = self.snapshot_open.as_ref() { return owner.next_close_byte_demand(); }
    |                                                                         ^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `next_close_byte_demand`, perhaps you need to implement one of them:
            candidate #1: `member_open::MemberOpenOperation`
            candidate #2: `os_store::component::ArtifactEnvelopeFieldDecoder`
```

```text
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:897:9
    |
897 |         ErasedSnapshotRetirement::close_step(self, maximum_items, maximum_bytes)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^       -------------  ------------- unexpected argument #3 of type `usize`
    |                                                    |
    |                                                    expected `RetainedCloneGrant`, found `usize`
    |
note: method defined here
   --> 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/../../♻️retirement/🧬️contract/🦀️.rs:12:8
```

```text
error[E0782]: expected a type, found a trait
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:903:49
    |
903 |     fn next_close_byte_demand(&self) -> usize { ErasedSnapshotRetirement::next_close_byte_demand(self) }
    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: you can add the `dyn` keyword if you want a trait object
    |
903 |     fn next_close_byte_demand(&self) -> usize { <dyn ErasedSnapshotRetirement>::next_close_byte_demand(self) }
    |                                                 ++++                         +
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:549:13
    |
549 | ...   SnapshotRetirementStep::Complete => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind...
    |       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:502:23
    |
502 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:502:23
    |
502 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:509:17
    |
509 |                 SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:509:17
    |
509 |                 SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:511:24
    |
511 |                     Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:511:24
    |
511 |                     Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    |                        ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:513:17
    |
513 | ...   SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Invar...
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:513:17
    |
513 | ...   SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Invar...
    |       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:514:17
    |
514 | ...   SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => {
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:514:17
    |
514 | ...   SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => {
    |       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:522:23
    |
522 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:522:23
    |
522 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:526:23
    |
526 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:526:23
    |
526 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:530:23
    |
530 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:530:23
    |
530 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:534:23
    |
534 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:534:23
    |
534 |             return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    |                       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:538:17
    |
538 |                 SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:538:17
    |
538 |                 SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:540:24
    |
540 |                     Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:540:24
    |
540 |                     Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    |                        ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:542:17
    |
542 | ...   SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Invar...
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:542:17
    |
542 | ...   SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Invar...
    |       ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:548:13
    |
548 |             SnapshotRetirementStep::Complete if owners.uninstalled_owners_terminal_is_empty() => {}
```

```text
error[E0433]: cannot find type `SnapshotRetirementStep` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:548:13
    |
548 |             SnapshotRetirementStep::Complete if owners.uninstalled_owners_terminal_is_empty() => {}
    |             ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `SnapshotRetirementStep`

Some errors have detailed explanations: E0046, E0050, E0061, E0063, E0107, E0201, E0252, E0277, E0308...
For more information about an error, try `rustc --explain E0046`.
warning: `semio-framework-os-kernel` (lib) generated 556 warnings
error: could not compile `semio-framework-os-kernel` (lib) due to 561 previous errors; 556 warnings emitted
```

```text
error: could not compile `semio-framework-os-kernel` (lib) due to 561 previous errors; 556 warnings emitted

```
