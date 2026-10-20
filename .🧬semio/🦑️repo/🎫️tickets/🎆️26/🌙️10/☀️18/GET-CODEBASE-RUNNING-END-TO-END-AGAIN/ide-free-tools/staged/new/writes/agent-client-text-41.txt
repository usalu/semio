// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP descriptions.

// #endregion 🧲️Header

package client



func mcpDesc(kind McpClientKind, key string) string {
	if kind == "" {
		kind = McpClientGeneric
	}
	if m, ok := mcpDescriptionTable[key]; ok {
		if s, ok2 := m[kind]; ok2 && s != "" {
			return s
		}
		if s, ok3 := m[McpClientGeneric]; ok3 {
			return s
		}
	}
	return ""
}

// 🔌️mcpDescriptionTable: keys = stable ids; inner map must include generic + each agent-client kind used.
var mcpDescriptionTable = map[string]map[McpClientKind]string{
	"prompt_enhance": {
		McpClientGeneric: "Use when you are about to add behavior and must align the change with repository rules before editing.",
		McpClientCursor:  "Use when the Cursor agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientKiro:    "Use when the Kiro agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientCopilot: "Use when the Copilot agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientClaude:  "Use when Claude Code is about to add behavior and must align the change with repository rules before editing.",
		McpClientCodex:   "Use when Codex is about to add behavior and must align the change with repository rules before editing.",
	},
	"prompt_refactor": {
		McpClientGeneric: "Use when a refactor is required and you must not regress behavior or tests.",
		McpClientCursor:  "Use when the Cursor agent must refactor without regressing behavior or tests.",
		McpClientKiro:    "Use when the Kiro agent must refactor without regressing behavior or tests.",
		McpClientCopilot: "Use when the Copilot agent must refactor without regressing behavior or tests.",
		McpClientClaude:  "Use when Claude Code must refactor without regressing behavior or tests.",
		McpClientCodex:   "Use when Codex must refactor without regressing behavior or tests.",
	},
	"prompt_test": {
		McpClientGeneric: "Use when tests must be extended before you claim coverage for new paths.",
		McpClientCursor:  "Use when the Cursor agent must extend tests before claiming coverage for new paths.",
		McpClientKiro:    "Use when the Kiro agent must extend tests before claiming coverage for new paths.",
		McpClientCopilot: "Use when the Copilot agent must extend tests before claiming coverage for new paths.",
		McpClientClaude:  "Use when Claude Code must extend tests before claiming coverage for new paths.",
		McpClientCodex:   "Use when Codex must extend tests before claiming coverage for new paths.",
	},
	"prompt_comply": {
		McpClientGeneric: "Use when the tree is red and you must converge implementation to passing tests without deleting assertions.",
		McpClientCursor:  "Use when the Cursor agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientKiro:    "Use when the Kiro agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientCopilot: "Use when the Copilot agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientClaude:  "Use when Claude Code must converge a red workspace to passing tests without deleting assertions.",
		McpClientCodex:   "Use when Codex must converge a red workspace to passing tests without deleting assertions.",
	},
	"res_root": {
		McpClientGeneric: "Fetch when the session anchor for the repository root is required before any scoped read.",
		McpClientCursor:  "Fetch when the Cursor session anchor for the repository root is required before any scoped read.",
		McpClientKiro:    "Fetch when the Kiro session anchor for the repository root is required before any scoped read.",
		McpClientCopilot: "Fetch when the Copilot session anchor for the repository root is required before any scoped read.",
		McpClientClaude:  "Fetch when Claude Code needs the repository root anchor before any scoped read.",
		McpClientCodex:   "Fetch when Codex needs the repository root anchor before any scoped read.",
	},
	"res_bundles": {
		McpClientGeneric: "Fetch when bundle inventory is required before choosing a technology subtree.",
		McpClientCursor:  "Fetch when bundle inventory is required before the Cursor agent chooses a technology subtree.",
		McpClientKiro:    "Fetch when bundle inventory is required before the Kiro agent chooses a technology subtree.",
		McpClientCopilot: "Fetch when bundle inventory is required before the Copilot agent chooses a technology subtree.",
		McpClientClaude:  "Fetch when bundle inventory is required before Claude Code chooses a technology subtree.",
		McpClientCodex:   "Fetch when bundle inventory is required before Codex chooses a technology subtree.",
	},
	"res_bundle_one": {
		McpClientGeneric: "Fetch when a single bundle context gate is required before editing that bundle.",
		McpClientCursor:  "Fetch when the Cursor agent needs one bundle context gate before editing that bundle.",
		McpClientKiro:    "Fetch when the Kiro agent needs one bundle context gate before editing that bundle.",
		McpClientCopilot: "Fetch when the Copilot agent needs one bundle context gate before editing that bundle.",
		McpClientClaude:  "Fetch when Claude Code needs one bundle context gate before editing that bundle.",
		McpClientCodex:   "Fetch when Codex needs one bundle context gate before editing that bundle.",
	},
	"res_folders": {
		McpClientGeneric: "Fetch when folder enumeration is required before path-sensitive edits.",
		McpClientCursor:  "Fetch when folder enumeration is required before the Cursor agent performs path-sensitive edits.",
		McpClientKiro:    "Fetch when folder enumeration is required before the Kiro agent performs path-sensitive edits.",
		McpClientCopilot: "Fetch when folder enumeration is required before the Copilot agent performs path-sensitive edits.",
		McpClientClaude:  "Fetch when folder enumeration is required before Claude Code performs path-sensitive edits.",
		McpClientCodex:   "Fetch when folder enumeration is required before Codex performs path-sensitive edits.",
	},
	"res_folder_one": {
		McpClientGeneric: "Fetch when one folder subtree must be confirmed before moves or renames under it.",
		McpClientCursor:  "Fetch when the Cursor agent must confirm one folder subtree before moves or renames under it.",
		McpClientKiro:    "Fetch when the Kiro agent must confirm one folder subtree before moves or renames under it.",
		McpClientCopilot: "Fetch when the Copilot agent must confirm one folder subtree before moves or renames under it.",
		McpClientClaude:  "Fetch when Claude Code must confirm one folder subtree before moves or renames under it.",
		McpClientCodex:   "Fetch when Codex must confirm one folder subtree before moves or renames under it.",
	},
	"res_files": {
		McpClientGeneric: "Fetch when file listing is required before batch reads across a scope.",
		McpClientCursor:  "Fetch when file listing is required before the Cursor agent batch-reads a scope.",
		McpClientKiro:    "Fetch when file listing is required before the Kiro agent batch-reads a scope.",
		McpClientCopilot: "Fetch when file listing is required before the Copilot agent batch-reads a scope.",
		McpClientClaude:  "Fetch when file listing is required before Claude Code batch-reads a scope.",
		McpClientCodex:   "Fetch when file listing is required before Codex batch-reads a scope.",
	},
	"res_file_one": {
		McpClientGeneric: "Fetch when a single file identity must be verified before a targeted edit.",
		McpClientCursor:  "Fetch when the Cursor agent must verify a single file identity before a targeted edit.",
		McpClientKiro:    "Fetch when the Kiro agent must verify a single file identity before a targeted edit.",
		McpClientCopilot: "Fetch when the Copilot agent must verify a single file identity before a targeted edit.",
		McpClientClaude:  "Fetch when Claude Code must verify a single file identity before a targeted edit.",
		McpClientCodex:   "Fetch when Codex must verify a single file identity before a targeted edit.",
	},
	"res_sections": {
		McpClientGeneric: "Fetch when section discovery is required before region-scoped refactors.",
		McpClientCursor:  "Fetch when section discovery is required before the Cursor agent runs region-scoped refactors.",
		McpClientKiro:    "Fetch when section discovery is required before the Kiro agent runs region-scoped refactors.",
		McpClientCopilot: "Fetch when section discovery is required before the Copilot agent runs region-scoped refactors.",
		McpClientClaude:  "Fetch when section discovery is required before Claude Code runs region-scoped refactors.",
		McpClientCodex:   "Fetch when section discovery is required before Codex runs region-scoped refactors.",
	},
	"res_section_one": {
		McpClientGeneric: "Fetch when one section boundary must be loaded before a surgical edit inside that section.",
		McpClientCursor:  "Fetch when the Cursor agent must load one section boundary before a surgical edit inside that section.",
		McpClientKiro:    "Fetch when the Kiro agent must load one section boundary before a surgical edit inside that section.",
		McpClientCopilot: "Fetch when the Copilot agent must load one section boundary before a surgical edit inside that section.",
		McpClientClaude:  "Fetch when Claude Code must load one section boundary before a surgical edit inside that section.",
		McpClientCodex:   "Fetch when Codex must load one section boundary before a surgical edit inside that section.",
	},
	"res_definitions": {
		McpClientGeneric: "Fetch when definition discovery is required before symbol-level work.",
		McpClientCursor:  "Fetch when definition discovery is required before the Cursor agent performs symbol-level work.",
		McpClientKiro:    "Fetch when definition discovery is required before the Kiro agent performs symbol-level work.",
		McpClientCopilot: "Fetch when definition discovery is required before the Copilot agent performs symbol-level work.",
		McpClientClaude:  "Fetch when definition discovery is required before Claude Code performs symbol-level work.",
		McpClientCodex:   "Fetch when definition discovery is required before Codex performs symbol-level work.",
	},
	"res_definition_one": {
		McpClientGeneric: "Fetch when one definition record must be confirmed before renaming or extracting it.",
		McpClientCursor:  "Fetch when the Cursor agent must confirm one definition record before renaming or extracting it.",
		McpClientKiro:    "Fetch when the Kiro agent must confirm one definition record before renaming or extracting it.",
		McpClientCopilot: "Fetch when the Copilot agent must confirm one definition record before renaming or extracting it.",
		McpClientClaude:  "Fetch when Claude Code must confirm one definition record before renaming or extracting it.",
		McpClientCodex:   "Fetch when Codex must confirm one definition record before renaming or extracting it.",
	},
	"res_tickets": {
		McpClientGeneric: "Fetch when ticket inventory is required before choosing where to attach work.",
		McpClientCursor:  "Fetch when ticket inventory is required before the Cursor agent chooses where to attach work.",
		McpClientKiro:    "Fetch when ticket inventory is required before the Kiro agent chooses where to attach work.",
		McpClientCopilot: "Fetch when ticket inventory is required before the Copilot agent chooses where to attach work.",
		McpClientClaude:  "Fetch when ticket inventory is required before Claude Code chooses where to attach work.",
		McpClientCodex:   "Fetch when ticket inventory is required before Codex chooses where to attach work.",
	},
	"res_ticket_one": {
		McpClientGeneric: "Fetch when one ticket record must be read before closing or reopening that ticket.",
		McpClientCursor:  "Fetch when the Cursor agent must read one ticket record before closing or reopening that ticket.",
		McpClientKiro:    "Fetch when the Kiro agent must read one ticket record before closing or reopening that ticket.",
		McpClientCopilot: "Fetch when the Copilot agent must read one ticket record before closing or reopening that ticket.",
		McpClientClaude:  "Fetch when Claude Code must read one ticket record before closing or reopening that ticket.",
		McpClientCodex:   "Fetch when Codex must read one ticket record before closing or reopening that ticket.",
	},
	"res_goals": {
		McpClientGeneric: "Fetch when goal document is required before opening or linking a ticket.",
		McpClientCursor:  "Fetch when goal document is required before the Cursor agent opens or links a ticket.",
		McpClientKiro:    "Fetch when goal document is required before the Kiro agent opens or links a ticket.",
		McpClientCopilot: "Fetch when goal document is required before the Copilot agent opens or links a ticket.",
		McpClientClaude:  "Fetch when goal document is required before Claude Code opens or links a ticket.",
		McpClientCodex:   "Fetch when goal document is required before Codex opens or links a ticket.",
	},
	"res_goal_one": {
		McpClientGeneric: "Fetch when one goal record must be confirmed before milestone or ticket association.",
		McpClientCursor:  "Fetch when the Cursor agent must confirm one goal record before milestone or ticket association.",
		McpClientKiro:    "Fetch when the Kiro agent must confirm one goal record before milestone or ticket association.",
		McpClientCopilot: "Fetch when the Copilot agent must confirm one goal record before milestone or ticket association.",
		McpClientClaude:  "Fetch when Claude Code must confirm one goal record before milestone or ticket association.",
		McpClientCodex:   "Fetch when Codex must confirm one goal record before milestone or ticket association.",
	},
	"res_policies": {
		McpClientGeneric: "Fetch when policy inventory is required before an audit or autofix pass.",
		McpClientCursor:  "Fetch when policy inventory is required before the Cursor agent starts an audit or autofix pass.",
		McpClientKiro:    "Fetch when policy inventory is required before the Kiro agent starts an audit or autofix pass.",
		McpClientCopilot: "Fetch when policy inventory is required before the Copilot agent starts an audit or autofix pass.",
		McpClientClaude:  "Fetch when policy inventory is required before Claude Code starts an audit or autofix pass.",
		McpClientCodex:   "Fetch when policy inventory is required before Codex starts an audit or autofix pass.",
	},
	"res_policy_one": {
		McpClientGeneric: "Fetch when one policy scope must be verified before interpreting breaches.",
		McpClientCursor:  "Fetch when the Cursor agent must verify one policy scope before interpreting breaches.",
		McpClientKiro:    "Fetch when the Kiro agent must verify one policy scope before interpreting breaches.",
		McpClientCopilot: "Fetch when the Copilot agent must verify one policy scope before interpreting breaches.",
		McpClientClaude:  "Fetch when Claude Code must verify one policy scope before interpreting breaches.",
		McpClientCodex:   "Fetch when Codex must verify one policy scope before interpreting breaches.",
	},
	"res_statutes": {
		McpClientGeneric: "Fetch when breach-kind listing is required before triage ordering.",
		McpClientCursor:  "Fetch when breach-kind listing is required before the Cursor agent orders triage.",
		McpClientKiro:    "Fetch when breach-kind listing is required before the Kiro agent orders triage.",
		McpClientCopilot: "Fetch when breach-kind listing is required before the Copilot agent orders triage.",
		McpClientClaude:  "Fetch when breach-kind listing is required before Claude Code orders triage.",
		McpClientCodex:   "Fetch when breach-kind listing is required before Codex orders triage.",
	},
	"res_statute_one": {
		McpClientGeneric: "Fetch when one breach-kind record must be read before applying a fix strategy.",
		McpClientCursor:  "Fetch when the Cursor agent must read one breach-kind record before applying a fix strategy.",
		McpClientKiro:    "Fetch when the Kiro agent must read one breach-kind record before applying a fix strategy.",
		McpClientCopilot: "Fetch when the Copilot agent must read one breach-kind record before applying a fix strategy.",
		McpClientClaude:  "Fetch when Claude Code must read one breach-kind record before applying a fix strategy.",
		McpClientCodex:   "Fetch when Codex must read one breach-kind record before applying a fix strategy.",
	},
	"res_contributors": {
		McpClientGeneric: "Fetch when contributor listing is required before attributing sessions or checkpoints.",
		McpClientCursor:  "Fetch when contributor listing is required before the Cursor agent attributes sessions or checkpoints.",
		McpClientKiro:    "Fetch when contributor listing is required before the Kiro agent attributes sessions or checkpoints.",
		McpClientCopilot: "Fetch when contributor listing is required before the Copilot agent attributes sessions or checkpoints.",
		McpClientClaude:  "Fetch when contributor listing is required before Claude Code attributes sessions or checkpoints.",
		McpClientCodex:   "Fetch when contributor listing is required before Codex attributes sessions or checkpoints.",
	},
	"res_contributor_one": {
		McpClientGeneric: "Fetch when one contributor record must be read before ownership-sensitive edits.",
		McpClientCursor:  "Fetch when the Cursor agent must read one contributor record before ownership-sensitive edits.",
		McpClientKiro:    "Fetch when the Kiro agent must read one contributor record before ownership-sensitive edits.",
		McpClientCopilot: "Fetch when the Copilot agent must read one contributor record before ownership-sensitive edits.",
		McpClientClaude:  "Fetch when Claude Code must read one contributor record before ownership-sensitive edits.",
		McpClientCodex:   "Fetch when Codex must read one contributor record before ownership-sensitive edits.",
	},
	"res_checkpoints": {
		McpClientGeneric: "Fetch when checkpoint inventory is required before comparing versions.",
		McpClientCursor:  "Fetch when checkpoint inventory is required before the Cursor agent compares versions.",
		McpClientKiro:    "Fetch when checkpoint inventory is required before the Kiro agent compares versions.",
		McpClientCopilot: "Fetch when checkpoint inventory is required before the Copilot agent compares versions.",
		McpClientClaude:  "Fetch when checkpoint inventory is required before Claude Code compares versions.",
		McpClientCodex:   "Fetch when checkpoint inventory is required before Codex compares versions.",
	},
	"res_checkpoint_one": {
		McpClientGeneric: "Fetch when one checkpoint record must be read before tying hooks or tickets to history.",
		McpClientCursor:  "Fetch when the Cursor agent must read one checkpoint record before tying hooks or tickets to history.",
		McpClientKiro:    "Fetch when the Kiro agent must read one checkpoint record before tying hooks or tickets to history.",
		McpClientCopilot: "Fetch when the Copilot agent must read one checkpoint record before tying hooks or tickets to history.",
		McpClientClaude:  "Fetch when Claude Code must read one checkpoint record before tying hooks or tickets to history.",
		McpClientCodex:   "Fetch when Codex must read one checkpoint record before tying hooks or tickets to history.",
	},
	"tool_ticket_open": {
		McpClientGeneric: "Use at the start of any tracked task that will touch the repository and needs a durable workspace folder.",
		McpClientCursor:  "Use at the start of a Cursor agent task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		McpClientKiro:    "Use at the start of a Kiro agent task that will touch the repository and needs a durable workspace folder; use when a spec id should be bound for later archival on close.",
		McpClientCopilot: "Use at the start of a Copilot agent task that will touch the repository and needs a durable workspace folder; use when a memory plan id should be bound for later archival on close.",
		McpClientClaude:  "Use at the start of a Claude Code task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		McpClientCodex:   "Use at the start of a Codex task that will touch the repository and needs a durable workspace folder; use when a memory id should be bound for later archival on close.",
	},
	"tool_ticket_close": {
		McpClientGeneric: "Use only when the tracked task is finished, the summary is final, and the touched paths list is complete. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		McpClientCursor:  "Use only when the Cursor agent task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		McpClientKiro:    "Use only when the Kiro agent task is finished, the summary is final, the touched paths list is complete, and any bound spec should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		McpClientCopilot: "Use only when the Copilot agent task is finished, the summary is final, the touched paths list is complete, and any bound memory plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		McpClientClaude:  "Use only when the Claude Code task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
		McpClientCodex:   "Use only when the Codex task is finished, the summary is final, the touched paths list is complete, and any bound memory should be archived into the ticket folder. Oversized ticket-folder artifacts (files above 5 MiB, folders above 10 MiB) are deleted automatically on close.",
	},
	"tool_ticket_reopen": {
		McpClientGeneric: "Use when work must continue on a closed ticket before any new edits land.",
		McpClientCursor:  "Use when the Cursor agent must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		McpClientKiro:    "Use when the Kiro agent must continue a closed ticket before new edits; use when rebinding a spec id for archival on the next close.",
		McpClientCopilot: "Use when the Copilot agent must continue a closed ticket before new edits; use when rebinding a memory plan id for archival on the next close.",
		McpClientClaude:  "Use when Claude Code must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		McpClientCodex:   "Use when Codex must continue a closed ticket before new edits; use when rebinding a memory id for archival on the next close.",
	},
	"tool_section_move": {
		McpClientGeneric: "Use when a region rename is required and the surrounding file context is already loaded.",
		McpClientCursor:  "Use when the Cursor agent must rename a region and the surrounding file context is already loaded.",
		McpClientKiro:    "Use when the Kiro agent must rename a region and the surrounding file context is already loaded.",
		McpClientCopilot: "Use when the Copilot agent must rename a region and the surrounding file context is already loaded.",
		McpClientClaude:  "Use when Claude Code must rename a region and the surrounding file context is already loaded.",
		McpClientCodex:   "Use when Codex must rename a region and the surrounding file context is already loaded.",
	},
	"tool_file_integrate": {
		McpClientGeneric: "Use when two files must be merged at a named region boundary without losing section markers.",
		McpClientCursor:  "Use when the Cursor agent must merge two files at a named region boundary without losing section markers.",
		McpClientKiro:    "Use when the Kiro agent must merge two files at a named region boundary without losing section markers.",
		McpClientCopilot: "Use when the Copilot agent must merge two files at a named region boundary without losing section markers.",
		McpClientClaude:  "Use when Claude Code must merge two files at a named region boundary without losing section markers.",
		McpClientCodex:   "Use when Codex must merge two files at a named region boundary without losing section markers.",
	},
	"tool_section_extract": {
		McpClientGeneric: "Use when a region must be split out into its own file before shrinking the original module.",
		McpClientCursor:  "Use when the Cursor agent must split a region into its own file before shrinking the original module.",
		McpClientKiro:    "Use when the Kiro agent must split a region into its own file before shrinking the original module.",
		McpClientCopilot: "Use when the Copilot agent must split a region into its own file before shrinking the original module.",
		McpClientClaude:  "Use when Claude Code must split a region into its own file before shrinking the original module.",
		McpClientCodex:   "Use when Codex must split a region into its own file before shrinking the original module.",
	},
	"arg_plan_id": {
		McpClientCopilot: "Set when a Copilot project memory file for this id exists and must be archived into the ticket on close.",
		McpClientClaude:  "Set when a `.claude/plan/<id>.md` file exists and must be archived into the ticket on close.",
		McpClientCodex:   "Set when a `.codex/memory/<repo>/<id>.md` file exists and must be archived into the ticket on close.",
	},
	"arg_spec_id": {
		McpClientKiro: "Set when a `.kiro/specs/<id>/` directory exists and must be archived into the ticket on close.",
	},
}
