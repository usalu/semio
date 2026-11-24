# Preparation Law Tests: bmp, epw, semio

Ran the `preparation_law` test target of each artifact crate with native cargo, same env as `stale-cargo.sh` (build dir `🗑️generated/native/build-domain`). Command per crate: `cargo test --manifest-path <crate>/📦️packages/🦀️rust/Cargo.toml --test preparation_law`. Driver: `🗑️generated/prep-run.sh`.

| Crate | Passed | Failed | Result |
| --- | --- | --- | --- |
| `semio-s-artifact-stdio-bmp` | 2 | 0 | exit 0 |
| `semio-s-artifact-stdio-epw` | 1 | 0 | exit 0 |
| `semio-s-artifact-stdio-semio` | 0 | 0 | exit 101, build failed, no tests ran |

## Failures

### semio: compile error in a dependency, no test ran

`semio-plugin-composition-publication` (lib) does not compile, so `semio-s-artifact-stdio-semio` never reached its tests. 3 errors, all `E0599 no method named digest_witness`:

- First error: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/📦️packages/🦀️rust/../../🦀️.rs:54:56`
  `Self::Artifact(publication) => publication.digest_witness(),`
  Receiver type is `&ArtifactStoreBatchPublication<<A as ArtifactApp>::Snapshot, <A as ArtifactApp>::Mutation>`.
- Also `🦀️.rs:55:54` (Config receiver) and a third at Draft/DraftMutation receiver.

This is not caused by the `preparation_law` helper. The method `digest_witness` is missing for `ArtifactStoreBatchPublication`, so it needs to be defined or imported in the publication crate.

## Notes

- bmp tests: `publication_preparation_publishes_the_post_snapshot_and_a_restoring_inverse`, `publication_preparation_turns_are_bounded_by_pages_and_paged_paints_restore`, both ok.
- epw test: `macro_installed_preparation_publishes_the_post_snapshot_and_a_restoring_inverse`, ok.
- Warnings in the logs were not reviewed.
- First attempt of `prep-run.sh` did nothing because `$G` was undefined in the script. Fixed and rerun. Not related to the tests.
- No source files were edited.
