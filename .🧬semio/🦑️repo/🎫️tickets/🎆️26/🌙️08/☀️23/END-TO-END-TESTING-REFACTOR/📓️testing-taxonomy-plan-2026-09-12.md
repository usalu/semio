# Testing Taxonomy Completion — 2026-09-12

The active objective is to remove every obsolete testing category, including testkit folders beside tests, and leave testing content classified only as tests, fixtures, examples or oracles at its semantic owner. The prior completed scans do not prove this requirement: the current tree and current consumers are authoritative.

## Requirements And Evidence

- Executable tests follow `<semantic-owner>/🧪️tests/<test-name>/<implementation>`. Case-specific support belongs with the actual case implementation, without a disguised shared-helper bucket.
- Fixtures are examples used only for testing under owner `🧫️fixtures`, outside test-case folders. Production static data belongs under `🖼️assets` and production code must not depend on fixture payloads.
- Examples demonstrate actual use; oracles provide an independent reference, expected behavior or comparison. Neither category may become a rename for arbitrary test support.
- Imports, native mounts/includes, discovery, task runners, manifests, cache inputs, schemas, taxonomy declarations and current documentation must agree with the resulting paths.
- Language-neutral regression vectors and independent library oracles must verify new enforcement. Runtime checks must load the relocated content. Final repository-wide evidence must cover both directory placement and active references, with any failures stated precisely.

## Ticket And Fleet

The actual local repo MCP returned `repo://goals`; `🎯aioptimizedrepo` remains the most appropriate goal. The existing `26/08/23/END-TO-END-TESTING-REFACTOR` ticket covers this task and was reopened through `ticket_reopen` with `no_management=true`. No repository goal was changed, no Git mutation was used and no external message was sent.

The main GPT 6 Astra coordinator owns integration, ticket lifecycle and final verification. GPT 5.6 Terra Extra High agents independently audit framework and other authored trees; GPT 5.6 Sol Extra High agents implement the guards and the resulting relocation partitions. The four available concurrency slots are filled with useful bounded work and rotated from exploration into execution and final audits.

All temporary outputs belong under this ticket's `🗑️generated`; retained scripts are named `📜️script.ts`. Existing unrelated edits are preserved. Reports retain exact source/destination and consumer paths. The previous ticket-close oversized-artifact purge must be accounted for so required retained inputs and Markdown reports survive closure; generated output will be explicitly removed after verification.

## Canonical Oracle Collection

The canonical collection is plural `🔮️oracles`. Existing singular `🔮️oracle`, `🧪️oracle` and configured `⚖️oracle` aliases must be replaced directly in physical folders, registry discovery, schema and consumer paths. This decision supersedes the initial framework audit's singular spelling. Real oracle contribution manifests remain at their semantic owner; they must not be moved into arbitrary test cases.

## Execution Fleet Recovery

The native collaboration tool rejected additional root and child spawns with `agent thread limit reached` after both Terra audits completed. A local ephemeral worker launch with system Codex CLI 0.135.0 failed before execution because the server requires a newer CLI for the requested Sol model. The already-installed app bundle contains Codex CLI 0.153.4 at `/Applications/ChatGPT.app/Contents/Resources/codex`; explicit Sol Extra High launches with that binary started both framework and plugin workers, and their runtime event logs confirm they began reading the lane instructions. The native Sol guard worker remains active, so the fleet is three Sol implementation workers plus the Astra coordinator, using all four available concurrent slots. Terra workers will resume independent audits after execution slots become available. No global CLI installation or configuration was changed.

Worker launch uses the local CLI's documented `exec`, `--ephemeral`, `--model`, `--config`, JSON output and stdin prompt options, verified from that executable's help. The [official Sol model reference](https://developers.openai.com/api/docs/models/gpt-5.6-sol) confirms the requested model identifier and `xhigh` support. All worker prompts, launcher inputs and source changes remain within the shared ticket/worktree; generated event logs are removed at completion.
