"""Group agent-client: `editor` means an agent client (Copilot, Cursor, Windsurf, Claude Code, Droid, Codex, Antigravity, Kiro) only here; IDE text editors keep their name.
Fail-closed codemod: explicit file allowlist, ordered literal rules, directory moves guarded by sha256, every leftover `editor` token printed for review."""
import re
from blib import *

g = "agent-client"
P = M + "/🧩️providers"
H = M + "/🪝️hooks"
CL = M + "/💻️client/⌨️cli"
FILES = [
    P + "/README.md", P + "/📦️packages/🐹️go/🎆️editors/🐹️.go", P + "/📦️packages/🐹️go/🐹️.go",
    P + "/📦️packages/🦀️rust/🎆️editor-providers/🦀️.rs", P + "/📦️packages/🦀️rust/Cargo.toml", P + "/📦️packages/🦀️rust/🎖️registry-and-git-hooks/🦀️.rs",
    P + "/📦️packages/🦀️rust/📐️provider-model/🦀️.rs", P + "/📦️packages/🦀️rust/🔭️provider-interfaces/🦀️.rs", P + "/📦️packages/🦀️rust/🦀️.rs",
    P + "/🔮️oracles/🔣️.json", P + "/🧬️schema/🔣️.json", P + "/🧪️tests/🔬️unit/🐹️.go", P + "/🧪️tests/🔬️unit/🦀️.rs",
    P + "/🧪️tests/🧫️fixtures/🪝️editor-hook-output-format/🪝️hook-outputs.json", P + "/🧪️tests/🪝️editor-hook-output-format/🐹️.go",
    P + "/🧪️tests/🪝️editor-hook-output-format/🥒️.feature", P + "/🧪️tests/🪝️editor-hook-output-format/🦀️.rs", P + "/🧪️tests/🪪️mcp-client-kind-parse/🥒️.feature",
    H + "/README.md", H + "/📦️packages/🐹️go/🔶️dispatch/🐹️.go", H + "/📦️packages/🐹️go/🚦️run/🐹️.go", H + "/📦️packages/🦀️rust/🔶️dispatch-and-formatting/🦀️.rs",
    H + "/📦️packages/🦀️rust/🦀️.rs", H + "/🔮️oracles/🔣️.json", H + "/🧬️schema/🔣️.json",
    CL + "/🎆️editor-providers/🐹️.go", CL + "/📣️hook-event-resolvers/🐹️.go", CL + "/🔒️git-devcontainer-providers/🐹️.go", CL + "/🔭️provider-interfaces/🐹️.go",
    CL + "/🕹️hook-command/🐹️.go", CL + "/🧪️tests/🧜️mermaid-and-providers/🐹️.go", M + "/⌨️cli/🔌️mcp-verb/🦀️.rs",
    CL + "/🧪️tests/⚙️configure-and-vscode-hooks/🐹️.go", P + "/📦️packages/🐹️go/🪝️hooks/🐹️.go", H + "/🧪️tests/🔶️run/🐹️.go",
    H + "/📦️packages/🐹️go/🐹️.go", H + "/📦️packages/🦀️rust/🎯️hook-model/🦀️.rs", M + "/🔌️mcp/🧬️schema/🔣️.json", M + "/🔌️mcp/📦️packages/🦀️rust/🗄️repository/🦀️.rs",
    M + "/⌨️cli/📦️packages/🐹️go/🗣️mcp-description/🐹️.go", M + "/⌨️cli/📦️packages/🐹️go/🏭️mcp-factory/🐹️.go", CL + "/💭️mcp-descriptions/🐹️.go", CL + "/🛰️mcp-server-factory/🐹️.go",
    M + "/🔌️mcp/🧪️tests/📋️capability-listing/🥒️.feature",
]
RULES = [
    ("EditorProviders", "AgentClientProviders"), ("EditorProvider", "AgentClientProvider"), ("editor_providers", "agent_client_providers"), ("editor_provider", "agent_client_provider"),
    ("editorProvider", "agentClientProvider"), ("editor-providers", "agent-client-providers"), ("perEditor", "perAgentClient"), ("per_editor", "per_agent_client"),
    ("everyEditor", "everyAgentClient"), ("every_editor", "every_agent_client"), ("every-editor", "every-agent-client"), ("editor-hook-output", "agent-client-hook-output"),
    ("EditorForClient", "AgentClientFor"), ("editor_for_client", "agent_client_for"), ("EditorClient", "AgentClient"), ("EditorKind", "AgentClientKind"),
    ("🎆️editors", "🎆️agent-clients"), ('"editors"', '"agentClients"'), ("file.Editors", "file.AgentClients"), ("Editors []string", "AgentClients []string"),
    ("let editors =", "let agent_clients ="), ("&editors", "&agent_clients"), ("for editor in", "for client in"), ("get_agent_client_provider(editor)", "get_agent_client_provider(client)"),
    ("for _, kind := range file.AgentClients", "for _, kind := range file.AgentClients"),
    ("no editor provider for {editor}", "no agent-client provider for {client}"), ("(editor.clone()", "(client.clone()"),
    # prose
    ("the eight editor providers", "the eight agent-client providers"), ("eight editor providers", "eight agent-client providers"), ("All eight editor providers", "All eight agent-client providers"),
    ("editor providers", "agent-client providers"), ("Editor providers", "Agent-client providers"), ("editor provider", "agent-client provider"), ("Editor provider", "Agent-client provider"),
    ("Editor hook event resolvers", "Agent-client hook event resolvers"), ("the editor hook output case", "the agent-client hook output case"), ("editor hook output case", "agent-client hook output case"),
    ("the eight editors", "the eight agent clients"), ("eight editors", "eight agent clients"), ("Seven of the eight editors", "Seven of the eight agent clients"),
    ("the same eight editors", "the same eight agent clients"), ("Every editor", "Every agent client"), ("every editor", "every agent client"), ("each editor", "each agent client"),
    ("Editor and agent integrations", "Agent-client integrations"), ("editor/agent operations (VSCode/Copilot", "agent-client operations (VS Code/Copilot"),
    ("the editor does not surface", "the agent client does not surface"), ("reaches the editor", "reaches the agent client"), ("an editor", "an agent client"), ("non-VS-Code editor", "non-VS-Code agent client"),
    ("every non-VS-Code editor", "every non-VS-Code agent client"), ("Editor identity, not MCP protocol", "Agent-client identity, not MCP protocol"), ("editor identity, not MCP protocol", "agent-client identity, not MCP protocol"),
    ("The formatting half of an editor", "The formatting half of an agent client"), ("The editor surface the profile stands for", "The agent-client surface the profile stands for"),
    ("and the editors —", "and the agent clients —"), ("every editor reads back", "every agent client reads back"), ("every non-VS-Code editor reads", "every non-VS-Code agent client reads"),
    ("the editor providers", "the agent-client providers"), ("the editor client slugs", "the agent-client slugs"), ("The eight editor client slugs", "The eight agent-client slugs"),
    ("which editors are asked", "which agent clients are asked"), ("the neutral lifecycle event slug every editor's native event", "the neutral lifecycle event slug every agent client's native event"),
    ("editor's native", "agent client's native"), ("the payload the editor providers marshal", "the payload the agent-client providers marshal"), ("its editor providers format", "its agent-client providers format"),
    ("the editor providers marshal", "the agent-client providers marshal"), ("sandbox and editor provider traits", "sandbox and agent-client provider traits"), ("sandbox and editor provider", "sandbox and agent-client provider"),
    ("eight editors:", "eight agent clients:"), ("contract with eight editors", "contract with eight agent clients"), ("same eight IDEs", "same eight agent clients"),
    ("native hook events onto one neutral event vocabulary", "native hook events onto one neutral event vocabulary"),
    ("sandbox (devcontainer), the eight editor\n", "sandbox (devcontainer), the eight agent-client\n"),
    ("Every other editor", "Every other agent client"),
    ("FormatVSCodeHookOutput", "FormatCopilotHookOutput"), ("formatVSCodeHookOutput", "formatCopilotHookOutput"), ("vsCodeEventFromHookEvent", "copilotEventFromHookEvent"),
    ("format_vscode_hook_output", "format_copilot_hook_output"), ("VsCodeHookOutput", "CopilotHookOutput"),
    ("the native events each IDE emits", "the native events each agent client emits"), ("the native payload each IDE writes", "the native payload each agent client writes"),
    ("IDE-native hook event name", "agent-client-native hook event name"), ("when the IDE repeats it", "when the agent client repeats it"), ("as the IDE passed them", "as the agent client passed them"),
    ("the IDE reported", "the agent client reported"), ("the IDE wrote to stdin", "the agent client wrote to stdin"), ("Per-IDE surface", "Per-agent-client surface"), ("per-IDE surface", "per-agent-client surface"),
    ("Which IDE-native MCP surface", "Which agent-client MCP surface"), ("per-IDE descriptions", "per-agent-client descriptions"), ("eight IDEs' native hook events", "eight agent clients' native hook events"),
    ("hook runs for the IDE kind", "hook runs for the agent-client kind"), ("must include generic + each IDE kind used", "must include generic + each agent-client kind used"),
    ("A repo MCP entry point, a hook client slug and an agent-client provider all name the same eight IDEs", "A repo MCP entry point, a hook client slug and an agent-client provider all name the same eight agent clients"),
]
LEFT = []
EDITED = {e["path"] for e in PLAN["edits"]}
for f in FILES:
    t = read(f)
    n = t
    for a, b in RULES:
        n = n.replace(a, b)
    if f in EDITED or f.endswith("Cargo.toml"):
        cur = t
        for a, b in RULES:
            if a != b and a in cur:
                ED(g, f, a, b, (cur.count(a),), "agent-client-edit"); cur = cur.replace(a, b)
        for i, line in enumerate(n.split("\n")):
            if re.search("editor", line, re.I): LEFT.append(f"{f.replace(M + '/', 'M/')}:{i + 1}: {line.strip()[:160]}")
        continue
    for i, line in enumerate(n.split("\n")):
        if re.search("editor", line, re.I): LEFT.append(f"{f.replace(M + '/', 'M/')}:{i + 1}: {line.strip()[:160]}")
    if n != t and f.endswith(".go"):
        import subprocess
        run = lambda s: subprocess.run(["gofmt"], input=s, capture_output=True, text=True)
        before, after = run(t), run(n)
        if after.returncode != 0: raise SystemExit("gofmt rejects the codemod result of " + f + ": " + after.stderr[:300])
        if before.returncode == 0 and before.stdout == t: n = after.stdout
        else: print("[agent-client] original not gofmt-clean, left unformatted:", f)
    if n != t: TEXTWRITE(g, f, n, "agent-client-text")
open(os.path.join(HERE, "staged", "agent-client-leftovers.txt"), "w", encoding="utf-8").write("\n".join(LEFT) + "\n")
print("[agent-client] files rewritten, leftover `editor` lines:", len(LEFT))

MOVE(g, P + "/📦️packages/🐹️go/🎆️editors/🐹️.go", P + "/📦️packages/🐹️go/🎆️agent-clients/🐹️.go", "agent-client-move")
MOVE(g, P + "/📦️packages/🦀️rust/🎆️editor-providers/🦀️.rs", P + "/📦️packages/🦀️rust/🎆️agent-client-providers/🦀️.rs", "agent-client-move")
MOVE(g, CL + "/🎆️editor-providers/🐹️.go", CL + "/🎆️agent-client-providers/🐹️.go", "agent-client-move")
for name in ("🐹️.go", "🥒️.feature", "🦀️.rs"):
    MOVE(g, P + "/🧪️tests/🪝️editor-hook-output-format/" + name, P + "/🧪️tests/🪝️agent-client-hook-output-format/" + name, "agent-client-move")
MOVE(g, P + "/🧪️tests/🧫️fixtures/🪝️editor-hook-output-format/🪝️hook-outputs.json", P + "/🧪️tests/🧫️fixtures/🪝️agent-client-hook-output-format/🪝️hook-outputs.json", "agent-client-move")
for d in (P + "/📦️packages/🐹️go/🎆️editors", P + "/📦️packages/🦀️rust/🎆️editor-providers", CL + "/🎆️editor-providers", P + "/🧪️tests/🪝️editor-hook-output-format", P + "/🧪️tests/🧫️fixtures/🪝️editor-hook-output-format"):
    RMDIR(g, d)
TX = L + "/🔣️taxonomy.json"
ED(g, TX, '        "🎆️editor-providers",\n', '        "🎆️agent-client-providers",\n')
ED(g, TX, '        "🎆️editors",\n', '        "🎆️agent-clients",\n')
