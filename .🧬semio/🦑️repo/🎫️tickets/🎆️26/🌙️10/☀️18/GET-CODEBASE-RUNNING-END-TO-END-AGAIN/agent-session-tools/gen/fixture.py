#!/usr/bin/env python3
"""Writes the language-agnostic fixture of the agent-session module (hand-stated expectations plus the reference derivation pinned): python3 fixture.py <module dir>."""
import json, os, subprocess, sys

mod = sys.argv[1]
old = json.load(open(sys.argv[2], encoding="utf8"))  # the unit-cache lane fixture the ancestry vectors move out of
CLAUDE = {"pid": 200, "started": "Mon Oct 20 10:00:00 2026", "command": "/Users/u/.local/bin/claude --resume"}
SHELL = {"pid": 300, "started": "Mon Oct 20 10:05:00 2026", "command": "/bin/zsh -c source /Users/u/.claude/shell-snapshots/s.sh && eval 'repo cargo'"}
identity = [
    {"name": "an explicit name wins over everything", "seed": {"agent": "reviewer", "environment": {"CLAUDE_CODE_SESSION_ID": "abc"}, "ancestry": [CLAUDE]}, "hand": {"agent": "reviewer", "client": "human", "origin": "explicit"}},
    {"name": "an explicit name with a client name", "seed": {"agent": "reviewer", "client": "claude-code", "environment": {}, "ancestry": []}, "hand": {"agent": "reviewer", "client": "claude", "origin": "explicit"}},
    {"name": "an explicit name with an unknown client is an mcp agent", "seed": {"agent": "w", "client": "inspector", "environment": {}, "ancestry": []}, "hand": {"agent": "w", "client": "mcp", "origin": "explicit"}},
    {"name": "an explicit name is sanitized", "seed": {"agent": "a b/c", "environment": {}, "ancestry": []}, "hand": {"agent": "a_b_c", "client": "human", "origin": "explicit"}},
    {"name": "the resident service names its agents", "seed": {"agent": "lib-a", "origin": "service", "environment": {}, "ancestry": []}, "hand": {"agent": "lib-a", "client": "service", "origin": "service"}},
    {"name": "the claude session variable", "seed": {"environment": {"CLAUDE_CODE_SESSION_ID": "5f2c-77"}, "ancestry": [SHELL]}, "hand": {"agent": "5f2c-77", "client": "claude", "origin": "environment"}},
    {"name": "the codex thread variable", "seed": {"environment": {"CODEX_THREAD_ID": "thread_9"}, "ancestry": []}, "hand": {"agent": "thread_9", "client": "codex", "origin": "environment"}},
    {"name": "the claude variable comes before the codex variable", "seed": {"environment": {"CODEX_THREAD_ID": "t", "CLAUDE_CODE_SESSION_ID": "c"}, "ancestry": []}, "hand": {"agent": "c", "client": "claude", "origin": "environment"}},
    {"name": "an empty variable is ignored", "seed": {"environment": {"CLAUDE_CODE_SESSION_ID": ""}, "ancestry": []}, "hand": {"agent": "default", "client": "human", "origin": "default"}},
    {"name": "a connection is named by its client and a digest", "seed": {"connection": {"client": "Claude Code", "clientProcess": 321, "process": 654}, "environment": {}, "ancestry": []}, "hand": {"client": "claude", "origin": "connection"}},
    {"name": "a connection with an unknown client", "seed": {"connection": {"client": "", "clientProcess": 1, "process": 2}, "environment": {}, "ancestry": []}, "hand": {"client": "mcp", "origin": "connection"}},
    {"name": "the nearest client process, not the shell that mentions the client in a path", "seed": {"environment": {}, "ancestry": [SHELL, CLAUDE, {"pid": 100, "started": "Mon Oct 20 09:00:00 2026", "command": "launchd"}]}, "hand": {"client": "claude", "origin": "process", "anchor": {"pid": 200, "started": "Mon Oct 20 10:00:00 2026"}}},
    {"name": "a restarted client is another agent", "seed": {"environment": {}, "ancestry": [{**CLAUDE, "started": "Mon Oct 20 11:00:00 2026"}]}, "hand": {"client": "claude", "origin": "process", "anchor": {"pid": 200, "started": "Mon Oct 20 11:00:00 2026"}}},
    {"name": "an interpreter running a client script", "seed": {"environment": {}, "ancestry": [{"pid": 77, "started": "t", "command": "node /usr/lib/node_modules/@anthropic-ai/claude-code/cli.js --x"}]}, "hand": {"client": "claude", "origin": "process"}},
    {"name": "no client anywhere is the default session", "seed": {"environment": {}, "ancestry": [SHELL]}, "hand": {"agent": "default", "client": "human", "origin": "default", "session": "d"}},
    {"name": "an owner process anchors an explicit session", "seed": {"agent": "x", "environment": {}, "ancestry": [], "ownerPid": 4242}, "hand": {"agent": "x", "anchor": {"pid": 4242, "started": ""}}},
]
client_process = [
    ["/home/u/.local/bin/claude --resume", "claude"], ["/Applications/Claude.app/Contents/MacOS/Claude", "claude"], ["/Applications/Cursor.app/Contents/Frameworks/Cursor Helper (Plugin).app/x", "cursor"],
    ["codex exec --json", "codex"], ["\"C:\\Users\\u\\AppData\\Local\\Programs\\claude\\claude.exe\" claude.exe", "claude"], ["windsurf --reuse-window", "windsurf"],
    ["node /usr/lib/node_modules/@anthropic-ai/claude-code/cli.js", "claude"], ["bun /home/u/tools/kiro/cli.ts", "kiro"], ["bun ./📜️script.ts claude", None],
    ["/bin/zsh -c source /Users/u/.claude/shell-snapshots/s.sh", None], ["-bash", None], ["sshd", None], ["", None], ["node", None], ["python3 -m http.server", None],
]
states = [
    ({"ageSeconds": 5, "anchorAlive": False, "isDefault": False}, "live"), ({"ageSeconds": 600, "anchorAlive": False, "isDefault": False}, "live"),
    ({"ageSeconds": 601, "anchorAlive": False, "isDefault": False}, "idle"), ({"ageSeconds": 3600, "anchorAlive": False, "isDefault": False}, "idle"),
    ({"ageSeconds": 3601, "anchorAlive": False, "isDefault": False}, "gone"), ({"ageSeconds": 3601, "anchorAlive": True, "isDefault": False}, "idle"),
    ({"ageSeconds": 86400, "anchorAlive": True, "isDefault": False}, "idle"), ({"ageSeconds": 86401, "anchorAlive": True, "isDefault": False}, "gone"),
    ({"ageSeconds": 999999, "anchorAlive": False, "isDefault": True}, "live"),
]
shares = [
    ([], {}), ([{"id": "a", "weight": 1}], {"a": 10000}), ([{"id": "a", "weight": 1}, {"id": "b", "weight": 1}], {"a": 5000, "b": 5000}),
    ([{"id": "a", "weight": 1}, {"id": "b", "weight": 1}, {"id": "c", "weight": 1}], {"a": 3334, "b": 3333, "c": 3333}),
    ([{"id": "x", "weight": 1}, {"id": "y", "weight": 2}], {"x": 3333, "y": 6667}), ([{"id": "p", "weight": 16}, {"id": "q", "weight": 1}, {"id": "r", "weight": 3}], None),
    ([{"id": "z", "weight": 7}, {"id": "y", "weight": 7}, {"id": "x", "weight": 7}, {"id": "w", "weight": 7}, {"id": "v", "weight": 7}, {"id": "u", "weight": 7}, {"id": "t", "weight": 7}], None),
]
fixture = {
    "version": 1,
    "identity": [{"name": r["name"], "seed": r["seed"], "hand": r["hand"]} for r in identity],
    "clientProcess": [{"command": c, "client": k} for c, k in client_process],
    "state": [{"facts": f, "state": s} for f, s in states],
    "shares": [{"rows": r, "shares": s} for r, s in shares],
    "stat": old["stat"], "linuxChain": old["linuxChain"], "windows": old["windows"],
    "psListing": {
        "text": "    1     0 Mon Oct 20 08:00:00 2026 /sbin/launchd\n  412     1 Mon Oct 20 08:01:00 2026 /Applications/Claude.app/Contents/MacOS/Claude\n  655   412 Mon Oct 20 09:00:00 2026 /bin/zsh -l\n  901   655 Wed Oct  7 10:06:00 2026 bun repo cargo build -p x\n  777     5 Mon Oct 20 09:30:00 2026 orphan --flag\n",
        "first": 901, "pids": [901, 655, 412], "orphanFirst": 777,
    },
}
path = os.path.join(mod, "🧪️tests", "🧫️fixtures", "🔣️.json")
os.makedirs(os.path.dirname(path), exist_ok=True)
oracle = os.path.join(mod, "🧪️tests", "🐍️oracle.py")
tmp = path + ".tmp"
json.dump(fixture, open(tmp, "w", encoding="utf8"), ensure_ascii=False)
out = json.loads(subprocess.run(["python3", oracle, tmp], capture_output=True, text=True, check=True).stdout)
for i, row in enumerate(fixture["shares"]):
    if row["shares"] is None:
        row["shares"] = out["shares"][i]
        assert sum(row["shares"].values()) == 10000, row
for row, expected in zip(fixture["identity"], out["identity"]):
    row["expected"] = expected
for row, expected in zip(fixture["clientProcess"], out["clientProcess"]):
    assert row["client"] == expected, (row, expected)
for row, expected in zip(fixture["state"], out["state"]):
    assert row["state"] == expected, (row, expected)
for row, expected in zip(fixture["shares"], out["shares"]):
    assert row["shares"] == expected, (row, expected)
for row in fixture["identity"]:
    e, h = row["expected"], row["hand"]
    for key, value in h.items():
        got = e.get({"agent": "agent", "client": "client", "origin": "origin", "session": "session", "anchor": "anchor"}[key])
        assert got == value, (row["name"], key, got, value)
os.remove(tmp)
with open(path, "w", encoding="utf8") as f:
    json.dump(fixture, f, ensure_ascii=False, indent=1)
    f.write("\n")
print("fixture", len(fixture["identity"]), "identity rows")
