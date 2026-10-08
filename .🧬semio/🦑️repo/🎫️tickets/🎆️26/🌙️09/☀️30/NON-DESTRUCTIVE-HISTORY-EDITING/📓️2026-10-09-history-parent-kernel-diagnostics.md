# Current History Parent Kernel Diagnostics

Native64483 ended before selected assertions. Compiler summary:1911 errors,1342 warnings. Current direct history parent located blocks: 1.

Only direct planning owners and the exact Store main retirement section are selected. Located error absence is not a native or whole typing certificate; earlier unresolved types can suppress later checking.

```text
error[E0599]: no associated function or constant named `operation_preparation_progress` found for struct `os_store::component::EditReplay<_, _>` in the current scope
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:151:118
      |
  151 |         self.replay.as_ref().map_or_else(replay_preparation::ArtifactReplayPreparationProgress::default, EditReplay::operation_preparation_progress)
      |                                                                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `os_store::component::EditReplay<_, _>`
...
25751 | pub struct EditReplay<P, Mutation: self::Mutation<P>> {
      | ----------------------------------------------------- associated function or constant `operation_preparation_progress` not found for this struct
      |
help: the function `operation_preparation_progress` is implemented on `os_store::component::ReplayOwnedState<_, _>`
      |
  151 -         self.replay.as_ref().map_or_else(replay_preparation::ArtifactReplayPreparationProgress::default, EditReplay::operation_preparation_progress)
  151 +         self.replay.as_ref().map_or_else(replay_preparation::ArtifactReplayPreparationProgress::default, os_store::component::ReplayOwnedState<_, _>::operation_preparation_progress)
      |
```