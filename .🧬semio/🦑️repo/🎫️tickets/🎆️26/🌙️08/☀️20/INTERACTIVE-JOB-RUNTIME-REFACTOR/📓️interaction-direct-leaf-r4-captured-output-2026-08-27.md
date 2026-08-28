# Interaction Direct Leaf R4 Captured Output

Canonical current-project exact-filter run exited 1 (Cargo101). Rust compiled dependencies and rejected the plugin test crate with 89 errors; no test executed. Tool output was truncated at source to 10,000 tokens from 129,370 tokens, so this is the exact available excerpt, not a complete log.

```text
Warning: truncated output (original token count: 129370)
Total output lines: 8081


> nx run @semio-tech/framework-plugin:test --args=local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned -- --nocapture

> bun 📜️script.ts test local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned -- --nocapture

   Compiling semio-framework-async v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust)
   Compiling semio-framework-job v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust)
warning: unused import: `OnceLock`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:41:22
   |
41 | use std::sync::{Arc, OnceLock};
   |                      ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::time::Instant`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:43:5
   |
43 | use std::time::Instant;
   |     ^^^^^^^^^^^^^^^^^^

warning: `semio-framework-job` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-job` to apply 2 suggestions)
   Compiling semio-framework-replication v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust)
warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/./../../📡️wire/🦀️component.rs:149:51
    |
149 |         self.backing.as_ref().map_or(0, |backing| std::mem::size_of_val(backing.as_ref()))
    |                                                   ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
149 -         self.backing.as_ref().map_or(0, |backing| std::mem::size_of_val(backing.as_ref()))
149 +         self.backing.as_ref().map_or(0, |backing| size_of_val(backing.as_ref()))
    |

warning: method `push` is never used
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🦀️component.rs:142:8
    |
109 | impl<T> MutationDagFixedSlots<T> {
    | -------------------------------- method in this implementation
...
142 |     fn push(&mut self, value: T) -> Result<(), T> {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-replication` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-replication` to apply 1 suggestion)
   Compiling semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:16:6
   |
16 | impl std::fmt::Debug for UiTypedRetirementCursor {
   |      ^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
   |
16 - impl std::fmt::Debug for UiTypedRetirementCursor {
16 + impl fmt::Debug for UiTypedRetirementCursor {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:35
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:63
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                                               ^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result {
   |

warning: unused variable: `value`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:61:57
    |
 61 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    |
help: `value` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:61:57
    |
 61 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
    = note: this warning originates in the macro `typed_fields` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:61:96
    |
 61 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    |
help: `bytes` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:61:96
    |
 61 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:63:29
    |
 63 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:63:29
    |
 63 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:21
    |
 64 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:21
    |
 64 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
196 | typed_fields!(SeparatorProps {});
    | -------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `semio-framework-ui-contract` (lib) generated 7 warnings (run `cargo fix --lib -p semio-framework-ui-contract` to apply 3 suggestions)
   Compiling semio-framework-pack v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust)
warning: fields `header_len` and `stored_len` are never read
   --> 🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/../../📐️format/🦀️component.rs:178:5
    |
176 | struct EncodedSegment {
    |        -------------- fields in this struct
177 |     bytes: Vec<u8>,
178 |     header_len: usize,
    |     ^^^^^^^^^^
179 |     stored_len: usize,
    |     ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-pack` (lib) generated 1 warning
   Compiling semio-framework-os-kernel-dsl-derive v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust)
warning: field `source_path` is never read
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:48:5
   |
45 | struct MutationAggregateSourceAuthority {
   |        -------------------------------- field in this struct
...
48 |     source_path: PathBuf,
   |     ^^^^^^^^^^^
   |
   = note: `MutationAggregateSourceAuthority` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-os-kernel-dsl-derive` (lib) generated 1 warning
   Compiling semio-framework-os-kernel v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust)
warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1765:13
     |
1765 |     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
     |
1765 -     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
1765 +     marker: PhantomData<fn() -> (P, Mutation)>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1770:119
     |
1770 | ...active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1770 -         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
1770 +         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
     |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:167:43
    |
167 | impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
    |                                           ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
167 - impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
167 + impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> PresenceStore<P, M> {
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5868:49
     |
5868 | struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5868 - struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
5868 + struct ArtifactRepositoryHistoryEntryDecoder<T>(PhantomData<T>);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5872:14
     |
5872 |         Self(std::marker::PhantomData)
     |              ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5872 -         Self(std::marker::PhantomData)
5872 +         Self(PhantomData)
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5957:14
     |
5957 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5957 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
5957 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5958:23
     |
5958 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5958 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
5958 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5976:18
     |
5976 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5976 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
5976 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5977:27
     |
5977 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5977 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
5977 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6171:14
     |
6171 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6171 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6171 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6172:23
     |
6172 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6172 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6172 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6173:25
     |
6173 |     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6173 -     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6173 +     retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6191:18
     |
6191 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6191 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6191 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6192:27
     |
6192 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6192 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6192 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6193:29
     |
6193 |         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6193 -         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6193 +         retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/…119370 tokens truncated…           `SpaceHistoryMutation` implements `dsl::Mutation<SpaceHistorySnapshot>`
              `Std1AnyMutation` implements `dsl::Mutation<Std1AnySnapshot>`
              `Std1StrictMutation` implements `dsl::Mutation<Std1StrictSnapshot>`
              `Std2AnyMutation` implements `dsl::Mutation<Std2AnySnapshot>`
              `WorkflowMutation` implements `dsl::Mutation<WorkflowSnapshot>`
            and 2 others
note: required by a bound in `dsl::MutationKind::inverse`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:207:9
    |
207 |     Op: Mutation<P>,
    |         ^^^^^^^^^^^ required by this bound in `MutationKind::inverse`
...
215 |     fn inverse(&self, base: &P) -> Vec<Op>;
    |        ------- required by a bound in this associated function
    = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8.long-type-4405478583205909084.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `diff` found for enum `transient::mutations::PublicationTransientMutation` in the current scope
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:56:38
    |
 56 |         assert_eq!(transient_inverse.diff(&transient_after).diff().apply(&transient_after).unwrap(), transient_before);
    |                                      ^^^^ method not found in `transient::mutations::PublicationTransientMutation`
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs:10:1
    |
 10 | pub enum PublicationTransientMutation {
    | ------------------------------------- method `diff` not found for this enum
    |
note: the method `diff` exists on the type `change_publication_transient::ChangePublicationTransient`
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/📡️spr/🎮️command/🦀️component.rs:211:5
    |
211 |     fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `diff`, perhaps you need to implement one of them:
            candidate #1: `MutationKind`
            candidate #2: `dsl::Mutation`

error[E0277]: the trait bound `PublicationPresenceMutation: Mutation<PublicationPresence>` is not satisfied
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:58:21
   |
58 | ...   assert_eq!(<PublicationPresenceMutation as Mutation<PublicationPresence>>::DESCRIPTORS, &[ChangePublicationPresence::DESCRIPTO...
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `dsl::Mutation<presence::PublicationPresence>` is not implemented for `presence::mutations::PublicationPresenceMutation`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs:10:1
   |
10 | pub enum PublicationPresenceMutation {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `dsl::Mutation<P>`:
             `NoPresenceMutation` implements `dsl::Mutation<NoPresence>`
             `NoTransientMutation` implements `dsl::Mutation<NoTransient>`
             `RunMutation` implements `dsl::Mutation<RunArtifact>`
             `SpaceHistoryMutation` implements `dsl::Mutation<SpaceHistorySnapshot>`
             `Std1AnyMutation` implements `dsl::Mutation<Std1AnySnapshot>`
             `Std1StrictMutation` implements `dsl::Mutation<Std1StrictSnapshot>`
             `Std2AnyMutation` implements `dsl::Mutation<Std2AnySnapshot>`
             `WorkflowMutation` implements `dsl::Mutation<WorkflowSnapshot>`
           and 2 others
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8.long-type-6079630089535308001.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no associated function or constant named `DESCRIPTOR` found for struct `change_publication_presence::ChangePublicationPresence` in the current scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:58:126
   |
58 | ...esence::DESCRIPTOR]);
   |            ^^^^^^^^^^ associated function or constant not found in `change_publication_presence::ChangePublicationPresence`
   |
  ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:9:1
   |
 9 | pub struct ChangePublicationPresence {
   | ------------------------------------ associated function or constant `DESCRIPTOR` not found for this struct
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `DESCRIPTOR`, perhaps you need to implement it:
           candidate #1: `MutationLeaf`

error[E0277]: the trait bound `PublicationTransientMutation: Mutation<PublicationTransient>` is not satisfied
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:59:21
   |
59 | ...   assert_eq!(<PublicationTransientMutation as Mutation<PublicationTransient>>::DESCRIPTORS, &[ChangePublicationTransient::DESCRI...
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `dsl::Mutation<transient::PublicationTransient>` is not implemented for `transient::mutations::PublicationTransientMutation`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs:10:1
   |
10 | pub enum PublicationTransientMutation {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `dsl::Mutation<P>`:
             `NoPresenceMutation` implements `dsl::Mutation<NoPresence>`
             `NoTransientMutation` implements `dsl::Mutation<NoTransient>`
             `RunMutation` implements `dsl::Mutation<RunArtifact>`
             `SpaceHistoryMutation` implements `dsl::Mutation<SpaceHistorySnapshot>`
             `Std1AnyMutation` implements `dsl::Mutation<Std1AnySnapshot>`
             `Std1StrictMutation` implements `dsl::Mutation<Std1StrictSnapshot>`
             `Std2AnyMutation` implements `dsl::Mutation<Std2AnySnapshot>`
             `WorkflowMutation` implements `dsl::Mutation<WorkflowSnapshot>`
           and 2 others
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8.long-type-4405478583205909084.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no associated function or constant named `DESCRIPTOR` found for struct `change_publication_transient::ChangePublicationTransient` in the current scope
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:59:129
   |
59 | ...sient::DESCRIPTOR]);
   |           ^^^^^^^^^^ associated function or constant not found in `change_publication_transient::ChangePublicationTransient`
   |
  ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:9:1
   |
 9 | pub struct ChangePublicationTransient {
   | ------------------------------------- associated function or constant `DESCRIPTOR` not found for this struct
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `DESCRIPTOR`, perhaps you need to implement it:
           candidate #1: `MutationLeaf`

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:68:124
    |
 68 | ...presence.print_op()).unwrap(), PublicationPresenceMutation::from(presence.clone()));
    |                                   --------------------------------- ^^^^^^^^^^^^^^^^ expected `PublicationPresenceMutation`, found `ChangePublicationPresence`
    |                                   |
    |                                   arguments to this function are incorrect
    |
note: associated function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/convert/mod.rs:594:8
    |
594 |     fn from(value: T) -> Self;
    |        ^^^^
help: try wrapping the expression in `component::publication_fixture::presence::mutations::PublicationPresenceMutation::ChangePublicationPresence`
    |
 68 |         assert_eq!(PublicationPresenceMutation::parse_op(&presence.print_op()).unwrap(), PublicationPresenceMutation::from(component::publication_fixture::presence::mutations::PublicationPresenceMutation::ChangePublicationPresence(presence.clone())));
    |                                                                                                                            ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++                +

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:69:127
    |
 69 | ...ransient.print_op()).unwrap(), PublicationTransientMutation::from(transient.clone()));
    |                                   ---------------------------------- ^^^^^^^^^^^^^^^^^ expected `PublicationTransientMutation`, found `ChangePublicationTransient`
    |                                   |
    |                                   arguments to this function are incorrect
    |
note: associated function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/convert/mod.rs:594:8
    |
594 |     fn from(value: T) -> Self;
    |        ^^^^
help: try wrapping the expression in `component::publication_fixture::transient::mutations::PublicationTransientMutation::ChangePublicationTransient`
    |
 69 |         assert_eq!(PublicationTransientMutation::parse_op(&transient.print_op()).unwrap(), PublicationTransientMutation::from(component::publication_fixture::transient::mutations::PublicationTransientMutation::ChangePublicationTransient(transient.clone())));
    |                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++                 +

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:72:135
    |
 72 | ...ncode_op().unwrap()).unwrap(), PublicationPresenceMutation::from(presence.clone()));
    |                                   --------------------------------- ^^^^^^^^^^^^^^^^ expected `PublicationPresenceMutation`, found `ChangePublicationPresence`
    |                                   |
    |                                   arguments to this function are incorrect
    |
note: associated function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/convert/mod.rs:594:8
    |
594 |     fn from(value: T) -> Self;
    |        ^^^^
help: try wrapping the expression in `component::publication_fixture::presence::mutations::PublicationPresenceMutation::ChangePublicationPresence`
    |
 72 |         assert_eq!(PublicationPresenceMutation::decode_op(&presence.encode_op().unwrap()).unwrap(), PublicationPresenceMutation::from(component::publication_fixture::presence::mutations::PublicationPresenceMutation::ChangePublicationPresence(presence.clone())));
    |                                                                                                                                       ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++                +

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🦀️.rs:73:138
    |
 73 | ...ncode_op().unwrap()).unwrap(), PublicationTransientMutation::from(transient.clone()));
    |                                   ---------------------------------- ^^^^^^^^^^^^^^^^^ expected `PublicationTransientMutation`, found `ChangePublicationTransient`
    |                                   |
    |                                   arguments to this function are incorrect
    |
note: associated function defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/convert/mod.rs:594:8
    |
594 |     fn from(value: T) -> Self;
    |        ^^^^
help: try wrapping the expression in `component::publication_fixture::transient::mutations::PublicationTransientMutation::ChangePublicationTransient`
    |
 73 |         assert_eq!(PublicationTransientMutation::decode_op(&transient.encode_op().unwrap()).unwrap(), PublicationTransientMutation::from(component::publication_fixture::transient::mutations::PublicationTransientMutation::ChangePublicationTransient(transient.clone())));
    |                                                                                                                                          +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++                 +

warning: unused variable: `parent_document_id`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:19585:17
      |
19585 |             let parent_document_id = self.store.envelope().id.clone();
      |                 ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_parent_document_id`
      |
      = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `restart_command`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37110:21
      |
37110 |                 let restart_command = restart_command.clone();
      |                     ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_restart_command`

warning: unused variable: `envelope_seq`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31760:22
      |
31760 |         if let Some((envelope_seq, mut owner)) = command {
      |                      ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_envelope_seq`

warning: unused variable: `actor`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10104:26
      |
10104 |                     let (actor, pack) = self.packs[self.packs_len - 1].as_ref().expect("retained app-typed presence pack");
      |                          ^^^^^ help: if this is intentional, prefix it with an underscore: `_actor`

Some errors have detailed explanations: E0046, E0277, E0308, E0433, E0599.
For more information about an error, try `rustc --explain E0046`.
warning: `semio-framework-plugin` (lib test) generated 480 warnings
error: could not compile `semio-framework-plugin` (lib test) due to 89 previous errors; 483 warnings emitted
1741 |  * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
1742 |  * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
1743 |  */
1744 | export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
1745 |   const status = runCmdInternal(cmd, args, opts);
1746 |   if (status !== 0) throw new Error(`${cmd} ${args.join(" ")} exited with status ${status}`);
                                     ^
error: cargo test --manifest-path Cargo.toml --lib local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned -- --nocapture exited with status 101
      at runCmd (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1746:31)
      at runCargo (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:2693:3)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:14:11)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1048:71)
      at runBundleScriptMain (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1078:16)

Bun v1.3.14 (macOS arm64)
Warning: command "bun 📜️script.ts test local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```
