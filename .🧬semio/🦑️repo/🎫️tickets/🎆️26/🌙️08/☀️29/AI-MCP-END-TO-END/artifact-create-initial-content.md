# artifact_create `initial` content

Verdict: by design, not a bug. `initial` is persisted.

Evidence (journey-space-2 and a fresh run):
- `artifact_create_handler` (`🗿️artifact/🦀️.rs`) passes `initial` (default `{}`) to `create_probe_artifact`, which seeds the genesis envelope with `ProbeSnapshot::default()` (`null`) and commits `ProbeMutation::SetValue(initial)` as the first edit (`🏠️workspace/🛂️gateway-backend/🔬️probe-documents/🦀️.rs`).
- The folder log `.semio/events.semio` holds `+{"note":"created by the semio MCP journey"}` with inverse `null`.
- Decoding the snapshot's `sprBase64` shows the edit journal entry `{"note":"created by the semio MCP journey"}`. The pack (`bnVsbA==` = `null`, 4 bytes) is the genesis value; the spr (457 bytes) is the journal that carries the initial value.
- Declared contract: `ArtifactCreateInput.initial` is an untyped value; output reports revision cursor 1. The e2e schema only requires `packBytes >= 1`. Nothing declares the pack as the current value.

Changes: README section "Created content lives in the journal", fixture `🧪️tests/🧫️fixtures/🌱️create-initial-content-law.json`, Rust quick test `artifact_create_commits_initial_as_the_first_journal_edit`.
