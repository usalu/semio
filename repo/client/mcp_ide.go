// #region 🧲Header
// Per-IDE MCP server kind, native descriptions, plan/spec resolution, and hook dispatch.
// #endregion 🧲Header

// #region 🤸Preamble
// Package client — MCP + hook helpers shared by repo/client/mcp and repo/client/mcp/{cursor,kiro,...}.

package client

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
)

// #endregion 🤸Preamble

// #region 🪪McpClientKind
// 🪪McpClientKind selects IDE-native MCP surface (descriptions, optional plan/spec fields).

type McpClientKind string

const (
	McpClientGeneric McpClientKind = "generic"
	McpClientCursor  McpClientKind = "cursor"
	McpClientKiro    McpClientKind = "kiro"
	McpClientCopilot McpClientKind = "copilot"
	McpClientClaude  McpClientKind = "claude"
	McpClientCodex   McpClientKind = "codex"
)

// HookClientForMcpKind maps an MCP entry binary to the hook client id used by ResolveHookEvent.
func HookClientForMcpKind(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "cursor-chat"
	case McpClientKiro:
		return "kiro-cli"
	case McpClientCopilot:
		return "copilot-chat"
	case McpClientClaude:
		return "claude-code"
	case McpClientCodex:
		return "codex"
	default:
		return ""
	}
}

// McpKindFromResolvedClient maps a validated ticket client slug to an MCP surface kind for plan/spec attachment.
func McpKindFromResolvedClient(client string) McpClientKind {
	switch strings.TrimSpace(client) {
	case "cursor-chat", "cursor":
		return McpClientCursor
	case "kiro-cli":
		return McpClientKiro
	case "copilot-chat":
		return McpClientCopilot
	case "claude-code":
		return McpClientClaude
	case "codex":
		return McpClientCodex
	default:
		return McpClientGeneric
	}
}

// McpServerName returns the MCP server identifier string for the given kind.
func McpServerName(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "repo-cursor"
	case McpClientKiro:
		return "repo-kiro"
	case McpClientCopilot:
		return "repo-copilot"
	case McpClientClaude:
		return "repo-claude"
	case McpClientCodex:
		return "repo-codex"
	default:
		return "repo"
	}
}

// #endregion 🪪McpClientKind

// #region 🗺️ResolvePlanSource
// 🗺️ResolvePlanSource resolves a plan or spec id to an absolute filesystem path (file or directory).

func ResolvePlanSource(kind McpClientKind, id string) (absPath string, isDir bool, err error) {
	id = strings.TrimSpace(id)
	if id == "" {
		return "", false, fmt.Errorf("plan or spec id is empty")
	}
	root := strings.TrimSpace(rootDir)
	if root == "" {
		return "", false, fmt.Errorf("repository root is not set")
	}
	switch kind {
	case McpClientCursor:
		pattern := filepath.Join(root, ".cursor", "plans", "*_"+id+".plan.md")
		matches, gerr := filepath.Glob(pattern)
		if gerr != nil {
			return "", false, gerr
		}
		if len(matches) == 0 {
			return "", false, fmt.Errorf("no Cursor plan matches id %q (glob %s)", id, pattern)
		}
		if len(matches) > 1 {
			return "", false, fmt.Errorf("ambiguous Cursor plan id %q: %d matches", id, len(matches))
		}
		return filepath.Clean(matches[0]), false, nil
	case McpClientKiro:
		dir := filepath.Join(root, ".kiro", "specs", id)
		st, statErr := os.Stat(dir)
		if statErr != nil {
			return "", false, fmt.Errorf("Kiro spec %q: %w", id, statErr)
		}
		if !st.IsDir() {
			return "", false, fmt.Errorf("Kiro spec %q is not a directory", id)
		}
		return filepath.Clean(dir), true, nil
	case McpClientCopilot, McpClientClaude, McpClientCodex:
		home, herr := os.UserHomeDir()
		if herr != nil {
			return "", false, herr
		}
		repoBase := filepath.Base(filepath.Clean(root))
		var p string
		switch kind {
		case McpClientCopilot:
			p = filepath.Join(home, ".copilot", "projects", repoBase, "memory", id+".md")
		case McpClientClaude:
			p = filepath.Join(home, ".claude", "plans", id+".md")
		case McpClientCodex:
			p = filepath.Join(home, ".codex", "memory", repoBase, id+".md")
		}
		st, statErr := os.Stat(p)
		if statErr != nil {
			return "", false, fmt.Errorf("plan file for id %q: %w", id, statErr)
		}
		if st.IsDir() {
			return "", false, fmt.Errorf("expected file at %s", p)
		}
		return filepath.Clean(p), false, nil
	default:
		return "", false, fmt.Errorf("plan or spec attachment is not supported for mcp kind %q", kind)
	}
}

// planClientTag returns the value stored in ticket.json for the plan attachment.
func planClientTag(kind McpClientKind) string {
	switch kind {
	case McpClientCursor:
		return "cursor"
	case McpClientKiro:
		return "kiro"
	case McpClientCopilot:
		return "copilot"
	case McpClientClaude:
		return "claude"
	case McpClientCodex:
		return "codex"
	default:
		return string(kind)
	}
}

// ApplyTicketPlanFromIDs resolves plan_id or spec_id and attaches it to the ticket before save.
func ApplyTicketPlanFromIDs(ticket *Ticket, kind McpClientKind, planID, specID string) error {
	planID = strings.TrimSpace(planID)
	specID = strings.TrimSpace(specID)
	if planID == "" && specID == "" {
		return nil
	}
	if planID != "" && specID != "" {
		return fmt.Errorf("pass only one of plan_id or spec_id")
	}
	var resolveKind McpClientKind
	var id string
	switch kind {
	case McpClientKiro:
		if planID != "" {
			return fmt.Errorf("use spec_id for Kiro, not plan_id")
		}
		resolveKind = McpClientKiro
		id = specID
	default:
		if specID != "" {
			return fmt.Errorf("use plan_id for this client, not spec_id")
		}
		resolveKind = kind
		id = planID
	}
	src, isDir, err := ResolvePlanSource(resolveKind, id)
	if err != nil {
		return err
	}
	ticket.Plan = &TicketPlan{
		Client: planClientTag(kind),
		ID:     id,
		Source: src,
	}
	_ = isDir // stored implicitly: directories use trailing handling in move
	return nil
}

// #endregion 🗺️ResolvePlanSource

// #region 📦MoveTicketPlan
// 📦moveTicketPlanIntoFolder moves the attached plan/spec from Source into the ticket folder on close.

func moveTicketPlanIntoFolder(ticket *Ticket) error {
	if ticket == nil || ticket.Plan == nil || strings.TrimSpace(ticket.Plan.Source) == "" {
		return nil
	}
	src := filepath.Clean(ticket.Plan.Source)
	if ticket.FolderPath == "" {
		return fmt.Errorf("ticket folder path is empty")
	}
	destName := filepath.Base(src)
	dest := filepath.Join(ticket.FolderPath, destName)
	if _, err := os.Stat(src); os.IsNotExist(err) {
		if _, err2 := os.Stat(dest); err2 == nil {
			ticket.Plan.Local = destName
			ticket.Plan.Source = ""
			return nil
		}
		return fmt.Errorf("plan source %q missing and destination %q not found", src, dest)
	}
	st, err := os.Stat(src)
	if err != nil {
		return fmt.Errorf("plan source missing: %w", err)
	}
	if st.IsDir() {
		if err := os.Rename(src, dest); err != nil {
			if err := copyDirTree(src, dest); err != nil {
				return fmt.Errorf("copy spec directory: %w", err)
			}
			if err := os.RemoveAll(src); err != nil {
				return fmt.Errorf("remove spec source after copy: %w", err)
			}
		}
	} else {
		if err := MoveFile(src, dest); err != nil {
			return fmt.Errorf("move plan file: %w", err)
		}
	}
	ticket.Plan.Local = destName
	ticket.Plan.Source = ""
	return nil
}

func copyDirTree(srcRoot, dstRoot string) error {
	return filepath.WalkDir(srcRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(srcRoot, path)
		if err != nil {
			return err
		}
		if rel == "." {
			return os.MkdirAll(dstRoot, 0o755)
		}
		target := filepath.Join(dstRoot, rel)
		if d.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
			return err
		}
		return CopyFile(path, target)
	})
}

// #endregion 📦MoveTicketPlan

// #region 🗣️McpDescriptions
// 🗣️mcpDesc picks a when-to-use string for tools, prompts, and resource titles (no cross-IDE leakage).

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

// mcpDescriptionTable: keys = stable ids; inner map must include generic + each IDE kind used.
var mcpDescriptionTable = map[string]map[McpClientKind]string{
	"prompt_enhance": {
		McpClientGeneric:  "Use when you are about to add behavior and must align the change with repository rules before editing.",
		McpClientCursor:   "Use when the Cursor agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientKiro:     "Use when the Kiro agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientCopilot:  "Use when the Copilot agent is about to add behavior and must align the change with repository rules before editing.",
		McpClientClaude:   "Use when Claude Code is about to add behavior and must align the change with repository rules before editing.",
		McpClientCodex:    "Use when Codex is about to add behavior and must align the change with repository rules before editing.",
	},
	"prompt_refactor": {
		McpClientGeneric:  "Use when a refactor is required and you must not regress behavior or tests.",
		McpClientCursor:   "Use when the Cursor agent must refactor without regressing behavior or tests.",
		McpClientKiro:     "Use when the Kiro agent must refactor without regressing behavior or tests.",
		McpClientCopilot:  "Use when the Copilot agent must refactor without regressing behavior or tests.",
		McpClientClaude:   "Use when Claude Code must refactor without regressing behavior or tests.",
		McpClientCodex:    "Use when Codex must refactor without regressing behavior or tests.",
	},
	"prompt_test": {
		McpClientGeneric:  "Use when tests must be extended before you claim coverage for new paths.",
		McpClientCursor:   "Use when the Cursor agent must extend tests before claiming coverage for new paths.",
		McpClientKiro:     "Use when the Kiro agent must extend tests before claiming coverage for new paths.",
		McpClientCopilot:  "Use when the Copilot agent must extend tests before claiming coverage for new paths.",
		McpClientClaude:   "Use when Claude Code must extend tests before claiming coverage for new paths.",
		McpClientCodex:    "Use when Codex must extend tests before claiming coverage for new paths.",
	},
	"prompt_comply": {
		McpClientGeneric:  "Use when the tree is red and you must converge implementation to passing tests without deleting assertions.",
		McpClientCursor:   "Use when the Cursor agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientKiro:     "Use when the Kiro agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientCopilot:  "Use when the Copilot agent must converge a red workspace to passing tests without deleting assertions.",
		McpClientClaude:   "Use when Claude Code must converge a red workspace to passing tests without deleting assertions.",
		McpClientCodex:    "Use when Codex must converge a red workspace to passing tests without deleting assertions.",
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
		McpClientGeneric: "Fetch when goal hierarchy is required before opening or linking a ticket.",
		McpClientCursor:  "Fetch when goal hierarchy is required before the Cursor agent opens or links a ticket.",
		McpClientKiro:    "Fetch when goal hierarchy is required before the Kiro agent opens or links a ticket.",
		McpClientCopilot: "Fetch when goal hierarchy is required before the Copilot agent opens or links a ticket.",
		McpClientClaude:  "Fetch when goal hierarchy is required before Claude Code opens or links a ticket.",
		McpClientCodex:   "Fetch when goal hierarchy is required before Codex opens or links a ticket.",
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
		McpClientGeneric:  "Use at the start of any tracked task that will touch the repository and needs a durable workspace folder.",
		McpClientCursor:   "Use at the start of a Cursor agent task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		McpClientKiro:     "Use at the start of a Kiro agent task that will touch the repository and needs a durable workspace folder; use when a spec id should be bound for later archival on close.",
		McpClientCopilot:  "Use at the start of a Copilot agent task that will touch the repository and needs a durable workspace folder; use when a memory plan id should be bound for later archival on close.",
		McpClientClaude:   "Use at the start of a Claude Code task that will touch the repository and needs a durable workspace folder; use when a plan id should be bound for later archival on close.",
		McpClientCodex:    "Use at the start of a Codex task that will touch the repository and needs a durable workspace folder; use when a memory id should be bound for later archival on close.",
	},
	"tool_ticket_close": {
		McpClientGeneric:  "Use only when the tracked task is finished, the summary is final, and the touched paths list is complete.",
		McpClientCursor:   "Use only when the Cursor agent task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder.",
		McpClientKiro:     "Use only when the Kiro agent task is finished, the summary is final, the touched paths list is complete, and any bound spec should be archived into the ticket folder.",
		McpClientCopilot:  "Use only when the Copilot agent task is finished, the summary is final, the touched paths list is complete, and any bound memory plan should be archived into the ticket folder.",
		McpClientClaude:   "Use only when the Claude Code task is finished, the summary is final, the touched paths list is complete, and any bound plan should be archived into the ticket folder.",
		McpClientCodex:    "Use only when the Codex task is finished, the summary is final, the touched paths list is complete, and any bound memory should be archived into the ticket folder.",
	},
	"tool_ticket_reopen": {
		McpClientGeneric:  "Use when work must continue on a closed ticket before any new edits land.",
		McpClientCursor:   "Use when the Cursor agent must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		McpClientKiro:     "Use when the Kiro agent must continue a closed ticket before new edits; use when rebinding a spec id for archival on the next close.",
		McpClientCopilot:  "Use when the Copilot agent must continue a closed ticket before new edits; use when rebinding a memory plan id for archival on the next close.",
		McpClientClaude:   "Use when Claude Code must continue a closed ticket before new edits; use when rebinding a plan id for archival on the next close.",
		McpClientCodex:    "Use when Codex must continue a closed ticket before new edits; use when rebinding a memory id for archival on the next close.",
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
	"tool_search": {
		McpClientGeneric: "Use when you lack authoritative paths and must narrow the workspace before opening files.",
		McpClientCursor:  "Use when the Cursor agent lacks authoritative paths and must narrow the workspace before opening files.",
		McpClientKiro:    "Use when the Kiro agent lacks authoritative paths and must narrow the workspace before opening files.",
		McpClientCopilot: "Use when the Copilot agent lacks authoritative paths and must narrow the workspace before opening files.",
		McpClientClaude:  "Use when Claude Code lacks authoritative paths and must narrow the workspace before opening files.",
		McpClientCodex:   "Use when Codex lacks authoritative paths and must narrow the workspace before opening files.",
	},
	"arg_plan_id": {
		McpClientCursor:  "Set when a `.cursor/plans/*_<id>.plan.md` file exists and must be archived into the ticket on close.",
		McpClientCopilot: "Set when a Copilot project memory file for this id exists and must be archived into the ticket on close.",
		McpClientClaude:  "Set when a `.claude/plans/<id>.md` file exists and must be archived into the ticket on close.",
		McpClientCodex:   "Set when a `.codex/memory/<repo>/<id>.md` file exists and must be archived into the ticket on close.",
	},
	"arg_spec_id": {
		McpClientKiro: "Set when a `.kiro/specs/<id>/` directory exists and must be archived into the ticket on close.",
	},
}

// #endregion 🗣️McpDescriptions

// #region 🦀McpServerFactory
// 🦀CreateMcpServer builds the MCP server for the given IDE kind (stdio).

func CreateMcpServer(kind McpClientKind) *server.MCPServer {
	if kind == "" {
		kind = McpClientGeneric
	}
	s := server.NewMCPServer(
		McpServerName(kind),
		"1.0.0",
		server.WithToolCapabilities(true),
		server.WithPromptCapabilities(true),
	)
	s.AddPrompt(
		mcp.NewPrompt("enhance",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_enhance")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Context the agent must honor while enhancing."), mcp.RequiredArgument()),
		),
		handleEnhancePrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("refactor",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_refactor")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Refactor constraints the agent must honor."), mcp.RequiredArgument()),
		),
		handleRefactorPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("test",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_test")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Coverage targets the agent must honor."), mcp.RequiredArgument()),
		),
		handleTestPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("comply",
			mcp.WithPromptDescription(mcpDesc(kind, "prompt_comply")),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("Compliance constraints the agent must honor."), mcp.RequiredArgument()),
		),
		handleComplyPrompt,
	)
	s.AddResource(
		mcp.NewResource("repo://", mcpDesc(kind, "res_root"), mcp.WithMIMEType("text/plain")),
		handleRepoResource,
	)
	s.AddResource(
		mcp.NewResource("repo://bundles", mcpDesc(kind, "res_bundles"), mcp.WithMIMEType("text/plain")),
		handleBundlesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://bundle/{id}", mcpDesc(kind, "res_bundle_one")),
		handleBundleResource,
	)
	s.AddResource(
		mcp.NewResource("repo://folders", mcpDesc(kind, "res_folders"), mcp.WithMIMEType("text/plain")),
		handleFoldersResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://folder/{id}", mcpDesc(kind, "res_folder_one")),
		handleFolderResource,
	)
	s.AddResource(
		mcp.NewResource("repo://files", mcpDesc(kind, "res_files"), mcp.WithMIMEType("text/plain")),
		handleFilesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://file/{id}", mcpDesc(kind, "res_file_one")),
		handleFileResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://sections/{id}", mcpDesc(kind, "res_sections")),
		handleSectionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://section/{id}", mcpDesc(kind, "res_section_one")),
		handleSectionResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://definitions/{id}", mcpDesc(kind, "res_definitions")),
		handleDefinitionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://definition/{id}", mcpDesc(kind, "res_definition_one")),
		handleDefinitionResource,
	)
	s.AddResource(
		mcp.NewResource("repo://tickets", mcpDesc(kind, "res_tickets"), mcp.WithMIMEType("text/plain")),
		handleTicketsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://ticket/{id}", mcpDesc(kind, "res_ticket_one")),
		handleTicketResource,
	)
	s.AddResource(
		mcp.NewResource("repo://goals", mcpDesc(kind, "res_goals"), mcp.WithMIMEType("text/plain")),
		handleGoalsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://goal/{id}", mcpDesc(kind, "res_goal_one")),
		handleGoalResource,
	)
	s.AddResource(
		mcp.NewResource("repo://policies", mcpDesc(kind, "res_policies"), mcp.WithMIMEType("text/plain")),
		handlePoliciesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://policy/{id}", mcpDesc(kind, "res_policy_one")),
		handlePolicyResource,
	)
	s.AddResource(
		mcp.NewResource("repo://statutes", mcpDesc(kind, "res_statutes"), mcp.WithMIMEType("text/plain")),
		handleStatutesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://statute/{id}", mcpDesc(kind, "res_statute_one")),
		handleStatuteResource,
	)
	s.AddResource(
		mcp.NewResource("repo://contributors", mcpDesc(kind, "res_contributors"), mcp.WithMIMEType("text/plain")),
		handleContributorsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://contributor/{id}", mcpDesc(kind, "res_contributor_one")),
		handleContributorResource,
	)
	s.AddResource(
		mcp.NewResource("repo://checkpoints", mcpDesc(kind, "res_checkpoints"), mcp.WithMIMEType("text/plain")),
		handleCheckpointsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("repo://checkpoint/{id}", mcpDesc(kind, "res_checkpoint_one")),
		handleCheckpointResource,
	)

	openOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_open")),
		mcp.WithString("emoji", mcp.Required(), mcp.Description("Single emoji representing the ticket (e.g. '🎫', '🐛', '✨').")),
		mcp.WithString("title", mcp.Required(), mcp.Description("Short title for the ticket. Shape is not enforced.")),
		mcp.WithString("prompt", mcp.Required(), mcp.Description("Full description of the task.")),
		mcp.WithString("goal", mcp.Description("Goal slug to associate the ticket with.")),
		mcp.WithString("client", mcp.Description("Agent client used for this ticket.")),
		mcp.WithString("llm", mcp.Description("LLM used for this ticket.")),
		mcp.WithBoolean("no_management", mcp.Description("Skip GitHub issue creation and management.")),
		mcp.WithString("draft", mcp.Description("Draft slug to seed the ticket workspace.")),
		mcp.WithString("parent", mcp.Description("Parent ticket slug for nested tickets.")),
		mcp.WithString("issue", mcp.Description("Existing GitHub issue URL to link instead of creating a new one.")),
	}
	switch kind {
	case McpClientCursor, McpClientCopilot, McpClientClaude, McpClientCodex:
		openOpts = append(openOpts, mcp.WithString("plan_id", mcp.Description(mcpDesc(kind, "arg_plan_id"))))
	case McpClientKiro:
		openOpts = append(openOpts, mcp.WithString("spec_id", mcp.Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(mcp.NewTool("ticket_open", openOpts...), newTicketOpenHandler(kind))

	closeOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_close")),
		mcp.WithString("summary", mcp.Required(), mcp.Description("Summary of the work done.")),
		mcp.WithArray("files", mcp.Description("Files created, updated, or removed during the ticket."), mcp.WithStringItems()),
		mcp.WithString("path", mcp.Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to close the latest open ticket.")),
		mcp.WithString("title", mcp.Description("Updated title for the ticket.")),
		mcp.WithBoolean("no_management", mcp.Description("Skip updating the GitHub issue.")),
	}
	s.AddTool(mcp.NewTool("ticket_close", closeOpts...), newTicketCloseHandler(kind))

	reopenOpts := []mcp.ToolOption{
		mcp.WithDescription(mcpDesc(kind, "tool_ticket_reopen")),
		mcp.WithString("path", mcp.Description("Ticket path as returned by ticket_open (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS'). Omit to reopen the latest closed ticket.")),
		mcp.WithString("prompt", mcp.Description("Updated or additional task description.")),
		mcp.WithString("llm", mcp.Description("LLM to use for the reopened ticket.")),
		mcp.WithString("client", mcp.Description("Agent client to use for the reopened ticket.")),
		mcp.WithString("title", mcp.Description("Updated title for the ticket.")),
		mcp.WithString("draft", mcp.Description("Draft slug to seed the ticket workspace.")),
		mcp.WithBoolean("no_management", mcp.Description("Skip updating the GitHub issue.")),
	}
	switch kind {
	case McpClientCursor, McpClientCopilot, McpClientClaude, McpClientCodex:
		reopenOpts = append(reopenOpts, mcp.WithString("plan_id", mcp.Description(mcpDesc(kind, "arg_plan_id"))))
	case McpClientKiro:
		reopenOpts = append(reopenOpts, mcp.WithString("spec_id", mcp.Description(mcpDesc(kind, "arg_spec_id"))))
	}
	s.AddTool(mcp.NewTool("ticket_reopen", reopenOpts...), newTicketReopenHandler(kind))

	s.AddTool(
		mcp.NewTool("section_move",
			mcp.WithDescription(mcpDesc(kind, "tool_section_move")),
			mcp.WithString("file", mcp.Required(), mcp.Description("Path to the file containing the section.")),
			mcp.WithString("old_name", mcp.Required(), mcp.Description("Current name of the section.")),
			mcp.WithString("new_name", mcp.Required(), mcp.Description("New name for the section.")),
		),
		sectionMove,
	)
	s.AddTool(
		mcp.NewTool("file_integrate",
			mcp.WithDescription(mcpDesc(kind, "tool_file_integrate")),
			mcp.WithString("source", mcp.Required(), mcp.Description("Path to the source file.")),
			mcp.WithString("target_section", mcp.Required(), mcp.Description("Name of the section in the target file to integrate into.")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Path to the target file.")),
			mcp.WithString("target_parent_section", mcp.Description("Name of the parent section in the target file.")),
		),
		sectionIntegrate,
	)
	s.AddTool(
		mcp.NewTool("section_extract",
			mcp.WithDescription(mcpDesc(kind, "tool_section_extract")),
			mcp.WithString("source_file", mcp.Required(), mcp.Description("Path to the source file.")),
			mcp.WithString("source_section", mcp.Required(), mcp.Description("Name of the section to extract.")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Path to the target file where the section will be written.")),
		),
		sectionExtract,
	)
	s.AddTool(
		mcp.NewTool("search",
			mcp.WithDescription(mcpDesc(kind, "tool_search")),
			mcp.WithString("query", mcp.Description("Space-separated keywords to search for.")),
		),
		mcpTree,
	)
	return s
}

// RunMcpServerFor starts the MCP stdio server for the given kind.
func RunMcpServerFor(kind McpClientKind) error {
	s := CreateMcpServer(kind)
	return server.ServeStdio(s)
}

// #endregion 🦀McpServerFactory

// #region 🪝TicketMcpHandlers
func newTicketOpenHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		return ticketOpenWithKind(ctx, request, kind)
	}
}

func newTicketCloseHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		_ = kind
		return ticketClose(ctx, request)
	}
}

func newTicketReopenHandler(kind McpClientKind) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		return ticketReopenWithKind(ctx, request, kind)
	}
}

// RunHookFor runs a hook for the IDE kind (client is implied; stdin is hook JSON payload).
func RunHookFor(kind McpClientKind, eventStr string, stdin []byte) error {
	client := HookClientForMcpKind(kind)
	if client == "" {
		return fmt.Errorf("hooks are not available for mcp kind %q", kind)
	}
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	repoRoot := findRepoRoot(cwd)
	SetRootDir(repoRoot)
	var input json.RawMessage
	if len(stdin) > 0 {
		input = json.RawMessage(stdin)
	}
	return runHookExecution(client, eventStr, "", "", "", "", repoRoot, input, false, os.Stdout, os.Stderr)
}

// RunMCPFor starts the MCP stdio server for the given IDE kind.
func RunMCPFor(kind McpClientKind) error {
	return RunMcpServerFor(kind)
}

// #endregion 🪝TicketMcpHandlers
