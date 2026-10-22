#!/usr/bin/env python3
"""Programmatic edits shared by the overlay and the apply tool: each function takes the text of a file as it is at apply time (after the sets applied before this one) and returns the new text, or the same text when the edit is already present. They survive other sets changing the surrounding lines because they locate JSON structure, not line text."""
import json, re


def _braced(text, start):
    """End index (exclusive) of the object that opens at text[start] == '{'."""
    depth, i, in_string = 0, start, False
    while i < len(text):
        c = text[i]
        if in_string:
            if c == "\\":
                i += 1
            elif c == '"':
                in_string = False
        elif c == '"':
            in_string = True
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise ValueError("unbalanced object")


FLAGS = [
    ("--agent", "required", "name", ["session"], "session open: the agent name that identifies the session (default: the agent of the calling client).", "session open: der Agent-Name, der die Sitzung identifiziert (Standard: der Agent des aufrufenden Clients)."),
    ("--client", "required", "name", ["session"], "session open: the name of the agent client, used to name the agent of an MCP connection when no client variable identifies it.", "session open: der Name des Agent-Clients, mit dem der Agent einer MCP-Verbindung benannt wird, wenn keine Client-Variable ihn identifiziert."),
    ("--client-pid", "required", "pid", ["session"], "session open: the process id of the agent client of a connection (part of the connection's identity).", "session open: die Prozess-ID des Agent-Clients einer Verbindung (Teil der Identität der Verbindung)."),
    ("--owner-pid", "required", "pid", ["session"], "session open: a process whose end ends the session (default: the nearest agent-client ancestor).", "session open: ein Prozess, dessen Ende die Sitzung beendet (Standard: der nächste Agent-Client-Vorfahre)."),
    ("--weight", "required", "n", ["session"], "session open: the weight of the session in the budget split between live sessions, 1 to 16 (default: 1).", "session open: das Gewicht der Sitzung bei der Aufteilung des Budgets zwischen aktiven Sitzungen, 1 bis 16 (Standard: 1)."),
    ("--export", "none", None, ["session"], "session open: also print the environment line NAME=value that attaches child processes to the session.", "session open: gibt zusätzlich die Umgebungszeile NAME=Wert aus, die Kindprozesse an die Sitzung bindet."),
    ("--dead", "none", None, ["session"], "session close: close every gone session instead of one.", "session close: schließt jede beendete Sitzung statt einer einzelnen."),
    ("--force", "none", None, ["session"], "session close: close a session although its owner process is still alive.", "session close: schließt eine Sitzung, obwohl ihr Eigentümerprozess noch lebt."),
]


def _flag_text(name, value, value_name, operations, en, de):
    rows = [f'      "name": {json.dumps(name)},', f'      "value": {json.dumps(value)},']
    if value_name:
        rows.append(f'      "valueName": {json.dumps(value_name)},')
    rows.append(f'      "operations": {json.dumps(operations)},')
    rows.append(f'      "description": {{ "en": {json.dumps(en, ensure_ascii=False)}, "de": {json.dumps(de, ensure_ascii=False)} }}')
    return "    {\n" + "\n".join(rows) + "\n    }"


def _flag_span(text, name):
    m = re.search(r'\{\s*"name":\s*' + re.escape(json.dumps(name)), text)
    if not m:
        return None
    start = m.start()
    return start, _braced(text, start)


def _add_operation_to_flag(text, name, operation):
    span = _flag_span(text, name)
    if span is None:
        raise ValueError("flag missing: " + name)
    block = text[span[0]:span[1]]
    m = re.search(r'"operations":\s*\[([^\]]*)\]', block)
    names = json.loads("[" + m.group(1) + "]")
    if operation in names:
        return text
    names.append(operation)
    replaced = block[:m.start()] + '"operations": [' + ", ".join(json.dumps(n) for n in names) + "]" + block[m.end():]
    return text[:span[0]] + replaced + text[span[1]:]


SESSION_OPERATION = {
    "summary": {
        "en": "Show, open and close agent sessions: one identity per agent working in this worktree, with its cargo lane, Bazel slot preference, Nx daemon namespace, test sandbox root, lease owner tag, budget share and capability.",
        "de": "Zeigt, öffnet und schließt Agent-Sitzungen: eine Identität je Agent, der in diesem Arbeitsbaum arbeitet, mit Cargo-Bahn, Bazel-Slot-Präferenz, Nx-Daemon-Namensraum, Test-Sandbox-Wurzel, Lease-Eigentümer-Marke, Budgetanteil und Berechtigung.",
    },
    "subcommands": [
        ("open", "session open [--agent=<name>] [--client=<name>] [--client-pid=<pid>] [--owner-pid=<pid>] [--weight=<n>] [--export] [--json]", "Open the session of the calling agent, or attach to it when it exists. The agent is named by --agent, else by the client's own session variable, else by the connection (--client), else by the nearest agent-client ancestor process, else the default session.", "Öffnet die Sitzung des aufrufenden Agents oder bindet an sie, wenn sie existiert. Der Agent wird durch --agent benannt, sonst durch die eigene Sitzungsvariable des Clients, sonst durch die Verbindung (--client), sonst durch den nächsten Agent-Client-Vorfahrenprozess, sonst die Standard-Sitzung."),
        ("status", "session status [<id>] [--json]", "Print one session (default: the current one): identity, state, owner process, every derived resource name, budget share, capability and what it holds right now (leases, capacity grants, test sandboxes).", "Gibt eine Sitzung aus (Standard: die aktuelle): Identität, Zustand, Eigentümerprozess, jeden abgeleiteten Ressourcennamen, Budgetanteil, Berechtigung und was sie gerade hält (Leases, Kapazitätszusagen, Test-Sandboxes)."),
        ("close", "session close <id> [--force] [--json] | session close --dead [--json]", "Close a session and release its test sandboxes and temp root, or close every gone session. A live session is refused unless --force is given.", "Schließt eine Sitzung und gibt ihre Test-Sandboxes und ihre Temp-Wurzel frei oder schließt jede beendete Sitzung. Eine aktive Sitzung wird ohne --force abgelehnt."),
        ("list", "session list [--all] [--json]", "List the sessions of this worktree (every worktree with --all) with agent, client, state, budget share and last use.", "Listet die Sitzungen dieses Arbeitsbaums (mit --all jedes Arbeitsbaums) mit Agent, Client, Zustand, Budgetanteil und letzter Nutzung auf."),
    ],
    "flags": ["--json", "--all", "--agent", "--client", "--client-pid", "--owner-pid", "--weight", "--export", "--dead", "--force"],
    "examples": [
        ("repo session status", "See which agent session this command belongs to and what it holds.", "Zeigt, zu welcher Agent-Sitzung dieser Befehl gehört und was sie hält."),
        ("repo session list --all --json", "Machine-readable list of every agent session of every worktree on this host.", "Maschinenlesbare Liste jeder Agent-Sitzung jedes Arbeitsbaums dieses Rechners."),
        ("repo session close --dead", "Release the sandboxes and temp roots of every session whose process tree is gone.", "Gibt die Sandboxes und Temp-Wurzeln jeder Sitzung frei, deren Prozessbaum beendet ist."),
    ],
    "notes": [
        ("Every repo command attaches to its session at the door and passes only its address (SEMIO_AGENT_SESSION) to child processes; every tool adapter reads the session from that one API. Sessions of a gone process tree are collected automatically at most once a minute.", "Jeder repo-Befehl bindet sich an der Tür an seine Sitzung und gibt nur deren Adresse (SEMIO_AGENT_SESSION) an Kindprozesse weiter; jeder Werkzeug-Adapter liest die Sitzung aus dieser einen Schnittstelle. Sitzungen eines beendeten Prozessbaums werden automatisch höchstens einmal pro Minute aufgeräumt."),
        ("The budget share of a session is its weight divided by the weights of all live sessions; the machine admission queue serves the session with the least reserved memory per weight first.", "Der Budgetanteil einer Sitzung ist ihr Gewicht geteilt durch die Gewichte aller aktiven Sitzungen; die Zulassungswarteschlange des Rechners bedient zuerst die Sitzung mit dem wenigsten reservierten Speicher je Gewicht."),
    ],
}


def catalog(text):
    """Adds the `session` operation, its flags and its entries in the shared flags `--json` and `--all` to the operations catalogue."""
    if '"name": "session"' in text and '"name": "--weight"' in text:
        return text
    marker = re.search(r'\n  \],\n  "operations": \[', text)
    if marker is None:
        raise ValueError("catalogue flags array not found")
    existing = {m.group(1) for m in re.finditer(r'\{\s*"name":\s*"(--[a-z-]+)"', text[:marker.start()])}
    out = text
    for name, value, value_name, operations, en, de in FLAGS:
        if name in existing:
            out = _add_operation_to_flag(out, name, "session")
        else:
            marker = re.search(r'\n  \],\n  "operations": \[', out)
            out = out[:marker.start()] + ",\n" + _flag_text(name, value, value_name, operations, en, de) + out[marker.start():]
    for name in ("--json", "--all"):
        out = _add_operation_to_flag(out, name, "session")
    base = re.search(r'\{\s*"name":\s*"leases"', out)
    ref = json.loads(out[base.start():_braced(out, base.start())])
    row = {"name": "session", "requires": ref["requires"], "synopsis": "session open|status|close|list", "summary": SESSION_OPERATION["summary"], **({"implementations": ref["implementations"]} if "implementations" in ref else {}), "dryRun": False, "progress": False, "cancellable": False,
           "subcommands": [{"name": n, "synopsis": s, "summary": {"en": en, "de": de}} for n, s, en, de in SESSION_OPERATION["subcommands"]],
           "flags": SESSION_OPERATION["flags"],
           "examples": [{"command": c, "description": {"en": en, "de": de}} for c, en, de in SESSION_OPERATION["examples"]],
           "notes": [{"en": en, "de": de} for en, de in SESSION_OPERATION["notes"]]}
    body = json.dumps(row, ensure_ascii=False, indent=2)
    body = "\n".join(("    " + line) if line else line for line in body.split("\n"))
    tail = re.search(r'\n  \]\n\}\n?$', out)
    if tail is None:
        raise ValueError("catalogue operations array end not found")
    return out[:tail.start()] + ",\n" + body + out[tail.start():]


def operations_schema(text):
    """Adds `session` to the closed operation name enum of the catalogue schema."""
    m = re.search(r'("enum":\s*\[)((?:"[a-z-]+",\s*)*"leases"[^\]]*)(\])', text)
    if m is None:
        raise ValueError("operation enum not found")
    names = json.loads("[" + m.group(2) + "]")
    if "session" in names:
        return text
    names.append("session")
    return text[:m.start(2)] + ", ".join(json.dumps(n) for n in names) + text[m.end(2):]


SURFACE_TOOLS = [
    '    { "operation": "session", "subcommand": "open", "kind": "maintain", "flags": ["--weight", "--json"], "bind": { "--json": true } }',
    '    { "operation": "session", "subcommand": "status", "kind": "query", "flags": ["--json"], "bind": { "--json": true }, "positional": [{ "name": "id", "optional": true, "pattern": "^[a-z][a-z0-9]{0,11}$", "description": { "en": "Agent session identifier (default: the session of this call).", "de": "Kennung der Agent-Sitzung (Standard: die Sitzung dieses Aufrufs)." } }] }',
    '    { "operation": "session", "subcommand": "list", "kind": "query", "flags": ["--all", "--json"], "bind": { "--json": true } }',
    '    { "operation": "session", "subcommand": "close", "kind": "maintain", "flags": ["--dead", "--force", "--json"], "bind": { "--json": true }, "positional": [{ "name": "id", "optional": true, "pattern": "^[a-z][a-z0-9]{0,11}$", "description": { "en": "Agent session identifier to close; leave empty together with dead=true.", "de": "Kennung der zu schließenden Agent-Sitzung; zusammen mit dead=true leer lassen." } }] }',
]


def surface(text):
    """Adds the four session tools to the authored build surface and reduces the agent identity to the one environment variable that carries a session address."""
    if '"operation": "session"' in text:
        return text
    out = text
    m = re.search(r'"environment":\s*\{\s*"agent":\s*"SEMIO_AGENT_ID",\s*"session":\s*"SEMIO_SESSION",\s*"lane":\s*"SEMIO_CARGO_LANE"\s*\},\s*"connectionSources":\s*\[[^\]]*\]', out)
    if m is None:
        raise ValueError("identity environment not found")
    out = out[:m.start()] + '"environment": { "session": "SEMIO_AGENT_SESSION" }' + out[m.end():]
    out = re.sub(r'("environment": \{ "session": "SEMIO_AGENT_SESSION" \})\s*\n(\s*\})', r'\1\n\2', out, count=1)
    anchor = out.find('"--violations":', out.find('"flagTypes"'))
    if anchor < 0:
        raise ValueError("flag types not found")
    end = _braced(out, out.index("{", anchor))
    out = out[:end] + ',\n    "--weight": { "type": "integer", "minimum": 1, "maximum": 16 }' + out[end:]
    tail = re.search(r'\n  \],\n  "excluded"', out)
    if tail is None:
        raise ValueError("tools array end not found")
    return out[:tail.start()] + ",\n" + ",\n".join(SURFACE_TOOLS) + out[tail.start():]


def surface_schema(text):
    """Reduces the identity of the surface schema to the session variable."""
    old = '''          "required": ["argument", "pattern", "description", "environment", "connectionSources"],
          "properties": {
            "argument": { "type": "string", "pattern": "^[a-z][a-z0-9_]*$" },
            "pattern": { "type": "string", "minLength": 1 },
            "description": { "$ref": "#/$defs/Text" },
            "environment": {
              "type": "object",
              "additionalProperties": false,
              "required": ["agent", "session", "lane"],
              "properties": { "agent": { "type": "string" }, "session": { "type": "string" }, "lane": { "type": "string" } }
            },
            "connectionSources": { "type": "array", "items": { "type": "string", "pattern": "^[A-Z][A-Z0-9_]*$" } }
          }'''
    new = '''          "required": ["argument", "pattern", "description", "environment"],
          "properties": {
            "argument": { "type": "string", "pattern": "^[a-z][a-z0-9_]*$" },
            "pattern": { "type": "string", "minLength": 1 },
            "description": { "$ref": "#/$defs/Text" },
            "environment": {
              "type": "object",
              "additionalProperties": false,
              "required": ["session"],
              "properties": { "session": { "type": "string", "pattern": "^[A-Z][A-Z0-9_]*$" } }
            }
          }'''
    if old not in text:
        if new in text:
            return text
        raise ValueError("identity schema not found")
    return text.replace(old, new)


def entrypoint_fixture(text):
    """Adds the build verb `session` and its routing vector to the shared entrypoint fixture of the Go and Rust launchers (text surgery: other sets add rows in their own layout)."""
    document = json.loads(text)
    if "session" in document["buildVerbs"]:
        return text
    out = text
    verbs = re.search(r'("buildVerbs":\s*\[)(.*?)(\n(\s*)\])', out, re.S)
    if verbs is None:
        raise ValueError("buildVerbs not found")
    last_indent = re.findall(r'\n(\s*)"[^"]+"\s*,?\s*$', verbs.group(2).rstrip() and verbs.group(2), re.M)
    indent = last_indent[-1] if last_indent else "    "
    out = out[:verbs.end(2)] + ",\n" + indent + '"session"' + out[verbs.end(2):]
    anchor = re.search(r'\{\s*"id":\s*"remote-execution"', out)
    if anchor is None:
        raise ValueError("remote-execution vector not found")
    end = _braced(out, anchor.start())
    column = len(out[:anchor.start()]) - len(out[:anchor.start()].rstrip(" ")) if out[:anchor.start()].endswith(" ") else 0
    pad = " " * (anchor.start() - out.rfind("\n", 0, anchor.start()) - 1)
    body = json.dumps({"id": "session", "argv": ["session", "--opaque=value"], "kind": "build", "arguments": ["session", "--opaque=value"]}, indent=2, ensure_ascii=False)
    body = "\n".join((pad + line) if index else line for index, line in enumerate(body.split("\n")))
    out = out[:end] + ",\n" + pad + body + out[end:]
    json.loads(out)
    return out


def surface_golden(text):
    """Adds the four session tools and their vectors to the language-neutral golden vectors of the build surface."""
    document = json.loads(text)
    if "session_open" in document["tools"]:
        return text
    names = sorted(set(document["tools"]) | {"session_open", "session_status", "session_list", "session_close"})
    document["tools"] = names
    document["vectors"] += [
        {"id": "session-open", "tool": "session_open", "arguments": {}, "valid": True, "argv": ["session", "open", "--json"], "environment": {}, "agent": ""},
        {"id": "session-open-weight", "tool": "session_open", "arguments": {"weight": 3, "agent": "planner"}, "valid": True, "argv": ["session", "open", "--json", "--weight=3"], "environment": {}, "agent": "planner"},
        {"id": "session-open-weight-too-heavy", "tool": "session_open", "arguments": {"weight": 17}, "valid": False},
        {"id": "session-status", "tool": "session_status", "arguments": {"id": "a3q5dv7obc"}, "valid": True, "argv": ["session", "status", "a3q5dv7obc", "--json"], "environment": {}, "agent": ""},
        {"id": "session-status-bad-id", "tool": "session_status", "arguments": {"id": "A-1"}, "valid": False},
        {"id": "session-list-all", "tool": "session_list", "arguments": {"all": True}, "valid": True, "argv": ["session", "list", "--json", "--all"], "environment": {}, "agent": ""},
        {"id": "session-close-dead", "tool": "session_close", "arguments": {"dead": True}, "valid": True, "argv": ["session", "close", "--json", "--dead"], "environment": {}, "agent": ""},
        {"id": "session-close-one", "tool": "session_close", "arguments": {"id": "a3q5dv7obc", "force": True}, "valid": True, "argv": ["session", "close", "a3q5dv7obc", "--json", "--force"], "environment": {}, "agent": ""},
    ]
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def process_project(text):
    """Registers the agent-session suites as test targets of the process framework project, next to the lease-order suites."""
    document = json.loads(text)
    if json.dumps(document, indent=2, ensure_ascii=False) + "\n" != text:
        raise ValueError("the process project manifest does not round-trip; edit it by hand")
    targets = document["targets"]
    if "test-agent-sessions" in targets:
        return text
    template = targets["test-resource-lease-order"]
    rebuilt = {}
    for name, target in targets.items():
        rebuilt[name] = target
        if name == "test-resource-lease-order-processes":
            for suite in ("agent-sessions", "agent-sessions-adapters"):
                copy = json.loads(json.dumps(template))
                copy["options"]["command"] = "bun ./📜️script.ts test " + suite
                rebuilt["test-" + suite] = copy
    document["targets"] = rebuilt
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def process_script(text):
    """Adds the two agent-session suites to the suite table of the process framework script."""
    anchor = re.search(r'( *)"resource-lease-order-processes": \{source:"[^"]+",budgetMs:TEST_LEVEL_BUDGET_MS\.quick\},\n', text)
    if anchor is None:
        raise ValueError("lease-order suite row not found")
    if '"agent-sessions"' in text:
        return text
    rows = anchor.group(1) + '"agent-sessions": {source:"🪪️agent-sessions/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n' + anchor.group(1) + '"agent-sessions-adapters": {source:"🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n'
    return text[:anchor.end()] + rows + text[anchor.end():]
