# WASI 1009 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":8,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T10:37:03.299Z","finishedAt":"2026-09-09T10:38:07.365Z","packages":160}

## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:30
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be sent between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement::<P>::new`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1522:9
     |
1522 | impl<P: Send + Sync + 'static> ReturnedSnapshotReadRetirement<P> {
     |         ^^^^ required by this bound in `ReturnedSnapshotReadRetirement::<P>::new`
1523 |     fn new(alias: Arc<P>, owned_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
     |        --- required by a bound in this associated function
help: consider further restricting type parameter `P` with trait `Send`
     |
4729 | impl<P: Clone + std::marker::Send, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:30
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be shared between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement::<P>::new`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1522:16
     |
1522 | impl<P: Send + Sync + 'static> ReturnedSnapshotReadRetirement<P> {
     |                ^^^^ required by this bound in `ReturnedSnapshotReadRetirement::<P>::new`
1523 |     fn new(alias: Arc<P>, owned_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
     |        --- required by a bound in this associated function
help: consider further restricting type parameter `P` with trait `Sync`
     |
4729 | impl<P: Clone + std::marker::Sync, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:30
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be sent between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:42
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                          ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Send`
     |
4729 | impl<P: Clone + std::marker::Send, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:30
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be shared between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:49
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                                 ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Sync`
     |
4729 | impl<P: Clone + std::marker::Sync, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:21
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                     ^^^ `P` cannot be sent between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:42
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                          ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Send`
     |
4729 | impl<P: Clone + std::marker::Send, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:21
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                     ^^^ `P` cannot be shared between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:49
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                                 ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Sync`
     |
4729 | impl<P: Clone + std::marker::Sync, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:21
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be sent between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:42
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                          ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Send`
     |
4729 | impl<P: Clone + std::marker::Send, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:4959:21
     |
4959 |                     Box::new(ReturnedSnapshotReadRetirement::new(previous, factory.clone()))
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `P` cannot be shared between threads safely
     |
note: required by a bound in `os_store::component::ReturnedSnapshotReadRetirement`
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:1516:49
     |
1516 | struct ReturnedSnapshotReadRetirement<P: Send + Sync + 'static> {
     |                                                 ^^^^ required by this bound in `ReturnedSnapshotReadRetirement`
help: consider further restricting type parameter `P` with trait `Sync`
     |
4729 | impl<P: Clone + std::marker::Sync, Mutation: self::Mutation<P>> TransientStore<P, Mutation> {
     |               +++++++++++++++++++


