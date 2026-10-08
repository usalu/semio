# During-Compilation Source Evidence Requirement

The observer's post-compilation source hash is not by itself proof of the bytes rustc consumed if a shared source changed during compilation. Current Runtime work will bind genuine dep-info compiler checksums to captured/current content through the existing owned hash implementation and independent oracle, while preserving raw compiler records. Actual proc-macro read snapshots need their original read content/roster and exact caller/producer unit context as well.

This strengthens evidence currentness; it is not a fixture finding, an assertion of source authorship, a feature/flag change or an instruction to stop concurrent workers. Current corrected fourguest receipts remain valid historical completed invocations but refuse current reuse where their inputs/artifacts changed. No weakened consumer, post-hoc hash substitution, ticket-to-production proof promotion or clean full runtime/publication result is claimed.
