# WASI 1111 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":6,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T13:19:03.262Z","finishedAt":"2026-09-09T13:34:39.167Z","packages":160}

## dead_code — method `entry_by_ordinal` is never used

error: method `entry_by_ordinal` is never used
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:7483:12
     |
7365 |     impl<M> ChildMemberRegistry<M> {
     |     ------------------------------ method in this implementation
...
7483 |         fn entry_by_ordinal(&self, ordinal: usize) -> Option<&ChildMemberEntry<M>> {
     |            ^^^^^^^^^^^^^^^^
     |
     = note: `-D dead-code` implied by `-D warnings`
     = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`


## dead_code — struct `DocumentClosureSourceView` is never constructed

error: struct `DocumentClosureSourceView` is never constructed
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:7527:12
     |
7527 |     struct DocumentClosureSourceView<'a, P, M> {
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — associated function `new` is never used

error: associated function `new` is never used
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:7535:12
     |
7534 |     impl<'a, P, M> DocumentClosureSourceView<'a, P, M> {
     |     -------------------------------------------------- associated function in this implementation
7535 |         fn new(generation: u64, root_reference: &'a ArtifactRef, root_snapshot: &'a P, members: &'a ChildMemberRegistry<M>) -> Self {
     |            ^^^


## clippy::manual_is_multiple_of — manual implementation of `.is_multiple_of()`

error: manual implementation of `.is_multiple_of()`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:128:8
    |
128 |     if turn_seq % 256 == 0 {
    |        ^^^^^^^^^^^^^^^^^^^ help: replace with: `turn_seq.is_multiple_of(256)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of
    = note: `-D clippy::manual-is-multiple-of` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::manual_is_multiple_of)]`


## clippy::manual_is_multiple_of — manual implementation of `.is_multiple_of()`

error: manual implementation of `.is_multiple_of()`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🔄️turn/🦀️.rs:854:8
    |
854 |     if turn_seq % 256 == 0 {
    |        ^^^^^^^^^^^^^^^^^^^ help: replace with: `turn_seq.is_multiple_of(256)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_is_multiple_of


## clippy::filter_map_bool_then — usage of `bool::then` in `filter_map`

error: usage of `bool::then` in `filter_map`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:7472:18
     |
7472 | ...  .filter_map(move |(index, slot)| (occupied[index / 64] & (1 << (index % 64)) != 0).then(|| unsafe { slot.assume_init_ref() }))
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `filter` then `map` instead: `filter(|&(index, slot)| (occupied[index / 64] & (1 << (index % 64)) != 0)).map(|(index, slot)| unsafe { slot.assume_init_ref() })`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#filter_map_bool_then
     = note: `-D clippy::filter-map-bool-then` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::filter_map_bool_then)]`


