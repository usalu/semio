# WASI 944 Diagnostics

error: using `chunks_exact` with a constant chunk size
   --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🦀️.rs:754:44
    |
754 |     for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
    |                                            ^^^^^^^^^^^^^^^ help: consider using `as_chunks` instead: `as_chunks::<2>().0.iter()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#chunks_exact_to_as_chunks
    = note: `-D clippy::chunks-exact-to-as-chunks` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::chunks_exact_to_as_chunks)]`


error: using `chunks_exact` with a constant chunk size
   --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🦀️.rs:854:28
    |
854 |     let mut chunks = bytes.chunks_exact(3);
    |                            ^^^^^^^^^^^^^^^
    |
help: consider using `as_chunks::<3>()` instead
   --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🦀️.rs:854:28
    |
854 |     let mut chunks = bytes.chunks_exact(3);
    |                            ^^^^^^^^^^^^^^^
    = note: you can access the chunks using `chunks.0.iter()`, and the remainder using `chunks.1`
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#chunks_exact_to_as_chunks


error: using `chunks_exact` with a constant chunk size
   --> ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/../../🦀️.rs:887:21
    |
887 |     hash.as_bytes().chunks_exact(2).map(|pair| nibble(pair[0]) << 4 | nibble(pair[1])).collect()
    |                     ^^^^^^^^^^^^^^^ help: consider using `as_chunks` instead: `as_chunks::<2>().0.iter()`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#chunks_exact_to_as_chunks


error: empty line after doc comment
   --> ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/../../../../🖥️app-surface/🦀️.rs:303:1
    |
303 | / /// is the manifest action id the command was declared under, which the command log labels the edit with.
304 | |
    | |_^
...
308 |   pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    |   ---------------------- the comment documents this function
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_doc_comments
    = note: `-D clippy::empty-line-after-doc-comments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::empty_line_after_doc_comments)]`
    = help: if the empty line is unintentional, remove it
help: if the documentation should include the empty line include it in the comment
    |
304 | ///
    |


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/../../../../🖥️app-surface/🦀️.rs:407:1
    |
407 | / pub fn norm_retained_reduce<A: NormRetainedEditor>(
408 | |     command: &A::Command,
409 | |     snapshot: &A::Snapshot,
410 | |     config: &A::Config,
...   |
415 | |     operation: &semio_framework_plugin::AppOperationContext,
416 | | ) -> NormRetainedCommandResult<A::Mutation> {
    | |___________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:158:1
    |
158 | / fn space_index_retained_reduce(
159 | |     command: &SpaceIndexCommand,
160 | |     snapshot: &SSpaceSnapshot,
161 | |     config: &SpaceIndexConfig,
...   |
166 | |     operation: &semio_framework_plugin::AppOperationContext,
167 | | ) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation, NoDraftMutation>, Fault> {
    | |___________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`

