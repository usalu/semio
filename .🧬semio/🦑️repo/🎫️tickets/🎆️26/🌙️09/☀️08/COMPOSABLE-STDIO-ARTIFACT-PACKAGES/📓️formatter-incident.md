# Formatter Incident

During the multi-artifact acceptance pass, I invoked `cargo fmt -- <GIS Map testkit path>` intending to format one Rust source. Cargo interpreted the trailing path as a rustfmt argument while still expanding the workspace package set. The process ran for about one minute before I observed the workspace-wide expansion and interrupted it; the command exited 130.

I did not revert the resulting changes. The workspace already contained concurrent edits across thousands of Rust files, so a broad revert or checkout could have destroyed other agents' work and there was no clean attribution baseline for separating pre-existing semantic edits from formatter output.

All subsequent scoped formatting uses the `rustfmt` executable with an exact file path. Acceptance review treats unexpected whitespace-only changes outside the owned ledger as possible formatter spillover and does not claim them as semantic extraction work. The owned source ledger remains based on files deliberately created, updated, or removed for the artifact extraction and its required test prerequisites.
