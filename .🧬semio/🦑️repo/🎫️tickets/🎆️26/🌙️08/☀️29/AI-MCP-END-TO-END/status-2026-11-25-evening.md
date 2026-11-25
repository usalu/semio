# 🌉️ Status 2026-11-25 (coordinator)

Verified on a rebuilt server binary over raw stdio: discovery, resources, prompts, context, search, probe artifacts, typed errors, missing-model details, `action_prepare` no longer crashes, gateway tools refused with a typed error. `initialize` 0.14 s, `tools/list` 0.73 s (8.65 s before).

Not verified: the Rust unit tests of `semio-os-mcp` (source boundary, see the agent report), the on-demand model build and hot swap through a live launcher, a real Claude Code client.

Open blocker: `artifact_create s.note.note` exceeds the operation budget (`note-create-item-budget.md`); all typed plugin actions, history, transactions, inference and export depend on it.
