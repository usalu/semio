"""Group code: no plan is read from .cursor/plans; Cursor carries no plan attachment; the plan is a ticket-owned file."""
import os, re
from blib import *

g = "code"
T = M + "/🎫️tickets"
CLI = M + "/💻️client/⌨️cli"
GOCLI = M + "/⌨️cli/📦️packages/🐹️go"

# ---- tickets Go: plan-source
P = T + "/📦️packages/🐹️go/🗺️plan-source/🐹️.go"
ED(g, P, '''	case providers.McpClientCursor:
		directory := JoinPath(root, ".cursor/plans")
		suffix := "_" + id + ".plan.md"
		entries, _ := store.Entries(directory)
		matches := make([]string, 0)
		for _, entry := range entries {
			if entry.Kind == NodeFile && strings.HasSuffix(entry.Name, suffix) {
				matches = append(matches, JoinPath(directory, entry.Name))
			}
		}
		switch len(matches) {
		case 0:
			return PlanSource{}, errTicketNotFound(fmt.Sprintf("no Cursor plan matches id %q (glob %s/*%s)", id, directory, suffix))
		case 1:
			return PlanSource{Path: matches[0]}, nil
		}
		return PlanSource{}, errConflict(fmt.Sprintf("ambiguous Cursor plan id %q: %d matches", id, len(matches)))
''', "")
ED(g, P, "// 🗺️ResolvePlanSource resolves a plan or spec id to a store path, per IDE.", "// 🗺️ResolvePlanSource resolves a plan or spec id to a store path, per agent client. A ticket owns the `*.plan.md` files of its own folder; no plan is read from a client directory inside the repository.")

# ---- tickets Rust: documents
R = T + "/📦️packages/🦀️rust/💾️documents/🦀️.rs"
ED(g, R, '''        McpClientKind::Cursor => {
            let directory = join_path(root, ".cursor/plans");
            let suffix = format!("_{id}.plan.md");
            let matches: Vec<String> = store
                .entries(&directory)
                .unwrap_or_default()
                .into_iter()
                .filter(|entry| entry.kind == NodeKind::File && entry.name.ends_with(&suffix))
                .map(|entry| join_path(&directory, &entry.name))
                .collect();
            match matches.len() {
                0 => Err(TicketError::not_found(format!("no Cursor plan matches id {id:?} (glob {directory}/*{suffix})"))),
                1 => Ok(PlanSource { path: matches[0].clone(), is_directory: false }),
                count => Err(TicketError::conflict(format!("ambiguous Cursor plan id {id:?}: {count} matches"))),
            }
        }
''', "")
ED(g, R, "        McpClientKind::Generic => Err(TicketError::invalid(", "        McpClientKind::Cursor | McpClientKind::Generic => Err(TicketError::invalid(")
ED(g, R, '''/// 🗺️ Resolves a plan or spec id to a store path, per IDE.
///
/// Cursor globs `<root>/.cursor/plans/*_<id>.plan.md` and refuses an ambiguous match; Kiro requires
/// the directory `<root>/.kiro/specs/<id>`; Copilot, Claude and Codex each name one file under the
/// contributor's home directory. The generic surface carries no plan attachment at all.''',
'''/// 🗺️ Resolves a plan or spec id to a store path, per agent client.
///
/// Kiro requires the directory `<root>/.kiro/specs/<id>`; Copilot, Claude and Codex each name one
/// file under the contributor's home directory. Cursor and the generic surface carry no plan
/// attachment: a ticket owns the `*.plan.md` files of its own folder and no plan is read from a
/// client directory inside the repository.''')

# ---- client cli: plan-resolution
CP = CLI + "/🗺️plan-resolution/🐹️.go"
ED(g, CP, '''	case McpClientCursor:
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
''', "")

# ---- MCP surfaces: plan_id exists only in the copilot, claude and codex profiles
for f, q in ((GOCLI + "/🏭️mcp-factory/🐹️.go", "providers."), (CLI + "/🛰️mcp-server-factory/🐹️.go", "")):
    ED(g, f, f"	case {q}McpClientCursor, {q}McpClientCopilot, {q}McpClientClaude, {q}McpClientCodex:\n", f"	case {q}McpClientCopilot, {q}McpClientClaude, {q}McpClientCodex:\n", (2,))
ED(g, M + "/💻️client/🔌️mcp/🗄️repository/🐹️.go", "	case providers.McpClientCursor, providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:\n", "	case providers.McpClientCopilot, providers.McpClientClaude, providers.McpClientCodex:\n")
ED(g, M + "/🔌️mcp/📦️packages/🦀️rust/🗄️repository/🦀️.rs", "        Profile::Cursor | Profile::Copilot | Profile::Claude | Profile::Codex => {\n            open.push((\"plan_id\"", "        Profile::Copilot | Profile::Claude | Profile::Codex => {\n            open.push((\"plan_id\"")
ED(g, M + "/🔌️mcp/📦️packages/🦀️rust/🗄️repository/🦀️.rs", "        Profile::Generic => {}\n    }\n    let tool = |name: &str, schema: Schema|", "        Profile::Cursor | Profile::Generic => {}\n    }\n    let tool = |name: &str, schema: Schema|")
ED(g, M + "/🔌️mcp/🧪️tests/🔬️unit/🦀️.rs", "matches!(profile, Profile::Cursor | Profile::Copilot | Profile::Claude | Profile::Codex)", "matches!(profile, Profile::Copilot | Profile::Claude | Profile::Codex)")
ED(g, M + "/💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go", '} else if profile.Kind == "generic" {', '} else if profile.Kind == "generic" || profile.Kind == "cursor" {')
ED(g, M + "/🔌️mcp/🧪️tests/🧫️fixtures/📋️surface.json", '"slug": "cursor",\n      "extraTicketArgument": "plan_id"', '"slug": "cursor",\n      "extraTicketArgument": null')
ED(g, M + "/🔌️mcp/🧬️schema/🔣️.json", "`plan_id` exists only in the cursor, copilot, claude and codex profiles", "`plan_id` exists only in the copilot, claude and codex profiles")
ED(g, M + "/🔌️mcp/🧪️tests/📋️capability-listing/🥒️.feature", "An IDE profile adds only its own plan or spec argument", "An agent-client profile adds only its own plan or spec argument")
ED(g, M + "/🔌️mcp/🧪️tests/📋️capability-listing/🥒️.feature", "only the plan profiles carry `plan_id`, and the generic profile carries neither", "only the copilot, claude and codex profiles carry `plan_id`, and the generic and cursor profiles carry neither")

# ---- MCP descriptions: Cursor has no plan attachment
CURSOR_DESC = 'Set when a `.cursor/plans/*_<id>.plan.md` file exists and must be archived into the ticket on close.'
ED(g, GOCLI + "/🗣️mcp-description/🐹️.go", f'		providers.McpClientCursor:  "{CURSOR_DESC}",\n', "")
ED(g, CLI + "/💭️mcp-descriptions/🐹️.go", f'		McpClientCursor:  "{CURSOR_DESC}",\n', "")
ED(g, M + "/🔌️mcp/🖼️assets/🔣️descriptions.json", f'      "cursor": "{CURSOR_DESC}",\n', "")

# ---- schemas and comments: the plan is a ticket-owned file
TS = T + "/🧬️schema/🔣️.json"
ED(g, TS, '"description": "🪪️ Which IDE surface a plan or spec id is resolved against."', '"description": "🪪️ Which agent client a plan or spec id is resolved against. Cursor and generic carry no plan attachment."')
ED(g, TS, '"description": "🗺️ The plan or spec bound to a ticket. `source` is the absolute path while the ticket is open; the close moves the file into the ticket folder and swaps `source` for `local`.",',
   '"description": "🗺️ The primary plan or spec of a ticket. A ticket owns every `<name>_<id>.plan.md` (or `<name>.plan.md`) file of its own folder and no plan lives in a client directory of the repository. `source` is the absolute path of a client-owned file (a home-directory plan or a Kiro spec directory) while the ticket is open; the close moves it into the ticket folder and swaps `source` for `local`.",')
ED(g, M + "/📐️model/📦️packages/🐹️go/🎫️ticket/🐹️.go", "// 🧷️TicketPlan records an IDE-native plan or spec file to archive into the ticket folder on close.", "// 🧷️TicketPlan records a client-native plan or spec file to archive into the ticket folder on close.")
ED(g, M + "/📐️model/📦️packages/🦀️rust/🎫️work-aggregates/🦀️.rs", "/// 🧷️ An IDE-native plan or spec file archived into the ticket folder on close.", "/// 🧷️ A client-native plan or spec file archived into the ticket folder on close.")
ED(g, M + "/📐️model/🧬️schema/🔣️.json", '"$comment": "🧷️ An IDE-native plan or spec file archived into the ticket folder on close."', '"$comment": "🧷️ A client-native plan or spec file archived into the ticket folder on close."')

import json as _j
POST_PLAN = [e["dest"] for e in _j.load(open(os.path.join(HERE, "staged", "plans-map.json"), encoding="utf-8"))["entries"] if e["src"].endswith("post_plan_to_issue_c256e28e.plan.md")][0]
# ---- tests: Cursor refuses a plan attachment; the live GitHub test reads the plan from its ticket
TU = T + "/🧪️tests/🔬️unit/🐹️.go"
FUNC(g, TU, "TestResolvePlanSourceCursorPlanID", '''func TestResolvePlanSourceRefusesCursorPlanID(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	if err := os.WriteFile(filepath.Join(planDir, "kit_store_"+id+".plan.md"), []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	sandbox.BindRoot(t, tmp)

	if got, err := ResolvePlanSource(NewFileTicketStore(), HostPlanRoots(), providers.McpClientCursor, id); err == nil {
		t.Fatalf("a client directory of the repository must not resolve a plan, got %q", got.Path)
	}
}''')
FUNC(g, TU, "TestApplyTicketPlanFromIDsCursor", '''func TestApplyTicketPlanFromIDsRefusesCursor(t *testing.T) {
	tmp := t.TempDir()
	sandbox.BindRoot(t, tmp)

	ticket := &model.Ticket{}
	if err := ApplyTicketPlanFromIDs(NewFileTicketStore(), HostPlanRoots(), ticket, providers.McpClientCursor, "fe75d494", ""); err == nil {
		t.Fatal("cursor carries no plan attachment")
	}
	if ticket.Plan != nil {
		t.Fatalf("plan: %+v", ticket.Plan)
	}
}''')
LIVE_OLD = 'filepath.Join(workspace.GetRootDir(), ".cursor", "plans", "post_plan_to_issue_c256e28e.plan.md")'
LIVE_NEW = 'filepath.Join(workspace.GetRootDir(), filepath.FromSlash("' + POST_PLAN + '"))'
ED(g, TU, LIVE_OLD, LIVE_NEW)

TC = CLI + "/🧪️tests/🗒️ticket-plan/🐹️.go"
FUNC(g, TC, "TestResolvePlanSourceCursorPlanID", '''func TestResolvePlanSourceRefusesCursorPlanID(t *testing.T) {
	tmp := t.TempDir()
	planDir := filepath.Join(tmp, ".cursor", "plans")
	if err := os.MkdirAll(planDir, 0o755); err != nil {
		t.Fatal(err)
	}
	id := "fe75d494"
	if err := os.WriteFile(filepath.Join(planDir, "kit_store_"+id+".plan.md"), []byte("plan"), 0o644); err != nil {
		t.Fatal(err)
	}
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	if got, _, err := ResolvePlanSource(McpClientCursor, id); err == nil {
		t.Fatalf("a client directory of the repository must not resolve a plan, got %q", got)
	}
}''')
FUNC(g, TC, "TestApplyTicketPlanFromIDsCursor", '''func TestApplyTicketPlanFromIDsRefusesCursor(t *testing.T) {
	tmp := t.TempDir()
	oldRoot := rootDir
	rootDir = tmp
	defer func() { rootDir = oldRoot }()

	ticket := &Ticket{}
	if err := ApplyTicketPlanFromIDs(ticket, McpClientCursor, "fe75d494", ""); err == nil {
		t.Fatal("cursor carries no plan attachment")
	}
	if ticket.Plan != nil {
		t.Fatalf("plan: %+v", ticket.Plan)
	}
}''')
ED(g, TC, 'filepath.Join(rootDir, ".cursor", "plans", "post_plan_to_issue_c256e28e.plan.md")', 'filepath.Join(rootDir, filepath.FromSlash("' + POST_PLAN + '"))')

# ---- library: no cursor-plan-snapshot population, the commit bullets cover ticket plans
BUL = L + "/🎛️commands/📝️commit/🔸️bullets/🟦️.ts"
ED(g, BUL, '{ id: ".cursor/plans", match: (p: string) => p.includes(".cursor/plans"), keywords: ["plan"] },', '{ id: "ticket plans", match: (p: string) => p.includes("🎫️tickets/") && p.endsWith(".plan.md"), keywords: ["plan"] },')
ED(g, BUL, "(include .cursor/plans, product code", "(include ticket plans, product code")
MC = L + "/🧪️tests/🔬️workspace-contract/🔬️micro-commit/🔹️buildmicrocommitmetrics-merges-uloc-git/🟦️.ts"
ED(g, MC, 'shouldSkipPathForUloc(root, ".cursor/plans/foo.plan.md")', 'shouldSkipPathForUloc(root, ".claude/plans/foo.plan.md")')
ED(g, MC, 'flags missing cursor-plans and product coverage', 'flags missing ticket-plans and product coverage')
ED(g, MC, 'const staged = [".cursor/plans/brush_fix_cfd8a931.plan.md",', 'const staged = [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️02/BRUSH-FILL-COLLISION/brush_fix_cfd8a931.plan.md",')
ED(g, MC, '.toContain(".cursor/plans");', '.toContain("ticket plans");')
ED(g, "/".join([".agents/skills/micro-commit/SKILL.md"]), "  - **`.cursor/plans/`** — Cursor plans are **high impact**; summarize what the plan decides or enables (read the diff, not the filename alone).", "  - **ticket `*.plan.md` files** — plans are **high impact**; summarize what the plan decides or enables (read the diff, not the filename alone).")
ED(g, ".agents/skills/micro-commit/SKILL.md", "A new **`.cursor/plans/*.plan.md`** often belongs", "A new **ticket `*.plan.md`** often belongs")
ED(g, ".agents/skills/micro-commit/SKILL.md", "(especially `.cursor/plans/` or repo infra)", "(especially ticket plans or repo infra)")

TX = L + "/🔣️taxonomy.json"
ED(g, TX, '"historicalDocumentEvidence": "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded"', '"historicalDocumentEvidence": "ticket-report-workspace-and-dev-prompt-log-whole-document-excluded"')
txt = read(TX)
i = txt.index('    "cursor-plan-snapshot": {')
j = txt.index('    },\n', i) + len('    },\n')
ED(g, TX, txt[i:j], "")
import json as _json
_m = _json.load(open(os.path.join(HERE, "staged", "plans-map.json"), encoding="utf-8"))
_dest = {e["src"]: e["dest"] for e in _m["entries"] if "dest" in e}
ED(g, TX, '"path": ".cursor/plans/puzzle_5d_design_parity_61aab236.plan.md"', '"path": ' + _json.dumps(_dest[".cursor/plans/puzzle_5d_design_parity_61aab236.plan.md"], ensure_ascii=False))

TD = L + "/🔍️discovery/📑️taxonomy-document/🟦️.ts"
ED(g, TD, '["ticket-report", "ticket-workspace", "cursor-plan-snapshot", "dev-prompt-log"] as const;', '["ticket-report", "ticket-workspace", "dev-prompt-log"] as const;')
ED(g, TD, "the exact, closed, four-population historical-document-evidence grammar", "the exact, closed, three-population historical-document-evidence grammar")
ED(g, TD, 'exactly ordered "ticket-report", "ticket-workspace", "cursor-plan-snapshot" and "dev-prompt-log" populations.', 'exactly ordered "ticket-report", "ticket-workspace" and "dev-prompt-log" populations.')
RX(g, TD, r'  const cursorPlanSnapshot = .*\n  if \(object\(cursorPlanSnapshot\).*\n', "")
ED(g, TD, 'historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded";', 'historicalDocumentEvidence: "ticket-report-workspace-and-dev-prompt-log-whole-document-excluded";')
ED(g, L + "/🔍️discovery/📎️audit-leaf-and-reference-policies/🟦️.ts", 'historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded"', 'historicalDocumentEvidence: "ticket-report-workspace-and-dev-prompt-log-whole-document-excluded"')

HT = L + "/🧪️tests/📜️historical-document-evidence/🟦️.ts"
ED(g, HT, '["ticket-report", "ticket-workspace", "cursor-plan-snapshot", "dev-prompt-log"]);', '["ticket-report", "ticket-workspace", "dev-prompt-log"]);')
ED(g, HT, 'toBe("ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded");', 'toBe("ticket-report-workspace-and-dev-prompt-log-whole-document-excluded");')
ED(g, HT, 'delete value["cursor-plan-snapshot"]; },', 'delete value["ticket-report"]; },')
ED(g, HT, 'structuredClone(value["cursor-plan-snapshot"])', 'structuredClone(value["ticket-report"])')
ED(g, HT, 'delete value["cursor-plan-snapshot"].reason;', 'delete value["dev-prompt-log"].reason;')
ED(g, HT, 'value["cursor-plan-snapshot"].extra = true;', 'value["dev-prompt-log"].extra = true;')
ED(g, HT, 'value["cursor-plan-snapshot"].leafPattern = "(";', 'value["ticket-workspace"].leafPattern = "(";')
ED(g, HT, '''    [".cursor/plans/2d_references_on_grid_ff798114.plan.md", true, "Cursor plan snapshot"],
    [".cursor/plans/not-a-plan.md", false, "a .cursor/plans file that is not a *.plan.md snapshot stays live"],''',
'''    [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️06/INFINITE-WORLD-2D-REFERENCES/2d_references_on_grid_ff798114.plan.md", true, "a ticket-owned plan is ticket workspace"],
    [".cursor/plans/2d_references_on_grid_ff798114.plan.md", false, "a client plan directory is no historical population: it stays live"],''')
ED(g, HT, 'const cursorPlan = ".cursor/plans/histref_fixture_test_deadbeef.plan.md";', 'const ticketPlan = `${closedTicketDir}/histref_fixture_test_deadbeef.plan.md`;')
ED(g, HT, 'put(cursorPlan, prose(".cursor/plans"));', 'put(ticketPlan, prose(closedTicketDir));')
ED(g, HT, 'expect(blocked.has(cursorPlan), "Cursor plan snapshot must not block the move").toBe(false);', 'expect(blocked.has(ticketPlan), "a ticket-owned plan must not block the move").toBe(false);')
ED(g, HT, 'const cursorPlanPath = ".cursor/plans/2d_references_on_grid_ff798114.plan.md";', 'const ticketPlanPath = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️06/☀️06/INFINITE-WORLD-2D-REFERENCES/2d_references_on_grid_ff798114.plan.md";')
ED(g, HT, 'historicalDocumentEvidence(cursorPlanPath, taxonomy, root), "before: no population means a Cursor plan snapshot is a live reference too"', 'historicalDocumentEvidence(ticketPlanPath, taxonomy, root), "before: no population means a ticket-owned plan is a live reference too"')
ED(g, HT, 'historicalDocumentEvidence(cursorPlanPath, real, root)', 'historicalDocumentEvidence(ticketPlanPath, real, root)')

FM = M + "/🏠️workspace/📦️packages/🐹️go/🎨️formatter/🐹️.go"
ED(g, FM, '''		if part == ".cursor" && index+1 < len(parts) && parts[index+1] != "plans" {
			return true
		}
		switch part {
		case ".vscode",''', '''		switch part {
		case ".cursor", ".vscode",''')
ED(g, FM, "	for index, part := range parts {\n		switch part {\n		case \".cursor\",", "	for _, part := range parts {\n		switch part {\n		case \".cursor\",")
ED(g, FM, "// 🛡️ IsIdeConfigurationPath identifies editor-owned configuration from dashboard authority.", "// 🛡️ IsIdeConfigurationPath identifies IDE-owned configuration from dashboard authority; the `.cursor` directory has no authority at all, not even for plans.")
