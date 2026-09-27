#!/usr/bin/env python3
"""🪟️ Z4: static Windows audit of every Nx `run-commands` target and every launch row: flags constructs that
cmd.exe/PowerShell (Nx runs `run-commands` through cmd.exe on win32) do not share with POSIX shells, and resolves each
target's static dependsOn closure from the launch rows of the four outcomes. usage: windows-target-scan.py <repo>"""
import json, re, subprocess, sys
from collections import defaultdict

repo = sys.argv[1]
files = subprocess.run(["git", "-C", repo, "ls-files", "-co", "--exclude-standard", "--", "*📋️project.json"], capture_output=True, text=True, check=True).stdout.split("\n")
files = [f for f in files if f and not f.startswith(".🧬semio/") and not f.startswith(".tmp")]
RULES = [
    ("chain-operator", re.compile(r"&&|\|\||(?<![\w-]);(?!\w)")),
    ("pipe-or-redirect", re.compile(r"(?<![|])\|(?![|])|(?<![-=])>|<(?![-=])|2>&1|/dev/null")),
    ("posix-expansion", re.compile(r"\$\(|`|\$[A-Za-z_{]")),
    ("cmd-expansion", re.compile(r"%[A-Za-z_]+%")),
    ("env-prefix", re.compile(r"(^|\s)[A-Z][A-Z0-9_]*=[^\s]*\s+\S")),
    ("single-quote-arg", re.compile(r"(^|\s)'[^']*'")),
    ("posix-shell", re.compile(r"(^|\s)(sh|bash|zsh)\s|\.sh(\s|$|\")")),
    ("posix-tool", re.compile(r"^(rm|cp|mv|mkdir|cat|chmod|ln|which|touch|sed|awk|grep|find|xargs|tar|curl|kill|pkill|lsof|ps|env|true|false|test|sleep|echo|export)\s")),
    ("python3", re.compile(r"(^|\s)python3\s")),
    ("posix-abs-path", re.compile(r"(^|[\s\"'=])/(usr|tmp|bin|opt|etc|var|Users|home)/")),
    ("backslash-path", re.compile(r"\\\\|[A-Za-z]\\")),
    ("npx-bunx", re.compile(r"(^|\s)(npx|bunx)\s")),
    ("not-bun", re.compile(r"^(?!bun\s)")),
]
targets = {}
projects = {}
for f in files:
    try: p = json.load(open(f"{repo}/{f}", encoding="utf-8"))
    except Exception as e: print(f"UNPARSEABLE {f}: {e}"); continue
    name = p.get("name") or f
    projects[name] = f
    for t, spec in (p.get("targets") or {}).items():
        opts = spec.get("options") or {}
        cmds = []
        if "command" in opts: cmds.append(opts["command"])
        for c in opts.get("commands") or []: cmds.append(c if isinstance(c, str) else c.get("command", ""))
        deps = []
        for d in spec.get("dependsOn") or []:
            if isinstance(d, str): deps.append(d if ":" in d else f"{name}:{d.lstrip('^')}")
            elif isinstance(d, dict) and "target" in d:
                for proj in (d.get("projects") if isinstance(d.get("projects"), list) else [d.get("projects") or name]): deps.append(f"{proj}:{d['target']}")
        targets[f"{name}:{t}"] = {"file": f, "executor": spec.get("executor"), "commands": cmds, "cwd": opts.get("cwd"), "deps": deps, "env": opts.get("env")}
launch = open(f"{repo}/.vscode/launch.json", encoding="utf-8").read()
launch = json.loads(re.sub(r"^\s*//.*$", "", launch, flags=re.M))["configurations"]
OUTCOME = re.compile(r"🪐️os-s|🗄️os-hub|🌉️os-mcp|🤝️|🔐️hub-auth|🧊️wgpu|🧑‍🤝‍🧑|🪐️s|os-hub|os-mcp|serve|🚀️")
roots = []
for row in launch:
    m = re.match(r"^bun (?:x )?nx run (\S+)", row.get("command", ""))
    if m and OUTCOME.search(row["name"] + row["command"]): roots.append((row["name"], m.group(1)))
closure = set()
def walk(t):
    if t in closure: return
    closure.add(t)
    for d in targets.get(t, {}).get("deps", []): walk(d)
for _, t in roots: walk(t)
findings = defaultdict(list)
for key, spec in targets.items():
    if spec["executor"] not in ("nx:run-commands", None): continue
    for cmd in spec["commands"]:
        for rule, rx in RULES:
            if rx.search(cmd): findings[rule].append((key in closure, key, cmd))
    if spec["env"]: findings["options-env"].append((key in closure, key, json.dumps(spec["env"], ensure_ascii=False)))
print(f"projects={len(projects)} targets={len(targets)} outcome-launch-rows={len(roots)} outcome-closure-targets={len(closure)} unresolved-in-closure={len([t for t in closure if t not in targets])}")
for rule, rows in sorted(findings.items()):
    inside = [r for r in rows if r[0]]
    print(f"\n## {rule}: {len(rows)} target(s), {len(inside)} in the outcome closure")
    for flag, key, cmd in sorted(rows, key=lambda r: (not r[0], r[1]))[:60]:
        print(f"  {'*' if flag else ' '} {key} :: {cmd[:220]}")
print("\n## launch rows not starting with `bun nx run` / `bun x nx run`")
for row in launch:
    if not re.match(r"^bun (x )?nx run", row.get("command", "")): print(f"  {row['name']} :: {row['command']}")
