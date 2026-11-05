# Shared Runtime Ticket Discovery

The connected repo://tickets MCP resource returned empty text, consistent with its previously observed wrong workspace root. A read-only local repo mcp request used the actual workspace but the all-ticket resource refused with code -32001, response too large. No response cap was raised and that broad request was not repeated. The owned helper closed its local MCP process in finally; its script is retained as input and temporary stderr is under generated.

A bounded read-only metadata search through repo tools found the existing open umbrella tickets Interactive Job Runtime Refactor (2026/08/20) and Microkernel Pooled Actor Plugin Runtime (2026/08/17). The former declares persistent bounded cancellable observable operations; the latter declares the actual actor/plugin runtime scope. These are related scope records. Their open status does not prove an active owner is currently implementing the specific missing general OriginalCore export/import execution.

Current source evidence, independently reviewed, shows that phased OriginalCore instances are refused by the existing legacy begin_owned_operation path used for context, kernel poll and job operations. The funded scalar execution owner does not yet provide the general Rust actor opcode/import machinery. See context-vm-proposal.md and actor-turn-custody.md. A wrapper around the old result is insufficient, and a legacy-instance substitution would not prove this gate.

The Puzzle fleet continues the domain-owned serializer, Source ownership and native/browser gates while preserving the actual actor prerequisite as open. No new umbrella ticket or goal was opened, closed or reopened by this discovery.
