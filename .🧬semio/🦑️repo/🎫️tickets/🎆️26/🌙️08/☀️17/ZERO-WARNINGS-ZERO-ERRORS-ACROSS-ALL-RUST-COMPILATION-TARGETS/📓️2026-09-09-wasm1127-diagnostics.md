# WASI 1127 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":1,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T13:56:55.164Z","finishedAt":"2026-09-09T14:33:46.890Z","packages":160}

## E0063 — missing field `child_content_generation` in initializer of `component::app::ArtifactStoreReplacementAdmissionTarget<'_, A, M>`

error[E0063]: missing field `child_content_generation` in initializer of `component::app::ArtifactStoreReplacementAdmissionTarget<'_, A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18664:30
      |
18664 | ...   let mut target = ArtifactStoreReplacementAdmissionTarget::<A, M> { jobs: &mut self.store_replacement_jobs, operation: handl...
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `child_content_generation`


