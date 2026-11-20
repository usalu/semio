# Native Failure Audit

The recorded `🗑️generated/native-offload-one.log` ends with Cargo exit 101. Its seven compiler errors all report E0277 for `AdmittedArtifactGenesis<A::Snapshot>: RetireOwned` in `semio-plugin-decl-app` lifted retirement. The log reports no distinct compiler error in its terminal diagnostic. Later output contains warnings and progress from other jobs, not a successful completion.

Current `🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs:114` implements `RetireOwned` for `AdmittedArtifactGenesis<P>` with `P: RetireOwned + Sync`. `📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:337` requires both bounds on `ArtifactApp::Snapshot`. These bounds satisfy the logged obligation on source inspection. The genesis source modification time was 06:13:28 local; the log continued to receive output long after its compiler error, so its final modification time alone does not establish what source the failing compilation read.

A narrow verification is `repo cargo --store=shared --configuration=prototype -- check -p semio-plugin-decl-app --lib`; use the same feature selection as the original native build if it differs. This checks the original failing package before repeating the expensive native end-to-end test. The next compiler error cannot be predicted from this log. No build was started during this audit because the host reported heavy swapping and insufficient memory.
