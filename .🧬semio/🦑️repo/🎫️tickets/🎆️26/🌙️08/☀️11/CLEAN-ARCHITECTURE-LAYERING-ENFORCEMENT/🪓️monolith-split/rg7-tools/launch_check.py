import json, os, re, subprocess, sys, collections
root = os.getcwd()
files = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-co", "--exclude-standard", "-z", "--", "**/📋️project.json", "📋️project.json"], capture_output=True).stdout.decode().split("\0")
projects = {}
for f in files:
    if not f:
        continue
    try:
        p = json.load(open(f, encoding="utf8"))
    except Exception as e:
        continue
    if isinstance(p.get("name"), str):
        projects[p["name"]] = set((p.get("targets") or {}).keys())
owners = json.load(open("📋️build-owners.json", encoding="utf8"))["owners"]
cargo_only = {o["name"] for o in owners if o["manifest"].endswith("Cargo.toml")}
def load(path):
    t = open(path, encoding="utf8").read()
    try:
        return json.loads(t)
    except Exception:
        out = []; i = 0; instr = False
        while i < len(t):
            ch = t[i]
            if instr:
                out.append(ch)
                if ch == "\\": out.append(t[i + 1]); i += 1
                elif ch == '"': instr = False
            elif ch == '"': instr = True; out.append(ch)
            elif t.startswith("//", i):
                while i < len(t) and t[i] != "\n": i += 1
                continue
            elif t.startswith("/*", i):
                i = t.index("*/", i) + 1
            else: out.append(ch)
            i += 1
        return json.loads(re.sub(r",(\s*[}\]])", r"\1", "".join(out)))
bad = collections.defaultdict(list)
for path in [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]:
    d = load(path)
    cfgs = [c for c in d["configurations"] if isinstance(c, dict)]
    print(path, len(d["configurations"]) - len(cfgs), "generated placeholders skipped;", end=" ")
    names = [c.get("name") for c in cfgs]
    dup = [n for n, k in collections.Counter(names).items() if k > 1]
    print(path, "parses;", len(cfgs), "configs;", len(dup), "duplicate names")
    for c in cfgs:
        cmd = c.get("command", "")
        m = re.match(r"bun repo run (@?[^\s:]+(?:/[^\s:]+)?):([^\s]+)", cmd)
        if m:
            pr, tg = m.groups()
            if pr in projects:
                if tg not in projects[pr]: bad[path].append(("missing-target", c["name"], pr, tg))
            elif pr in cargo_only: pass
            else: bad[path].append(("missing-project", c["name"], pr, tg))
            continue
        m = re.match(r"bun repo exec --projects=([^\s]+)", cmd)
        if m:
            for pr in m.group(1).split(","):
                if pr not in projects and pr not in cargo_only: bad[path].append(("missing-exec-project", c["name"], pr, ""))
        for q in re.findall(r'\$\{workspaceFolder\}/([^"\s]+)', cmd):
            if not os.path.exists(q): bad[path].append(("missing-path", c["name"], q, ""))
        m = re.match(r"bun (\.🧬semio/[^\s]+|[^\s\"$]+📜️script\.ts)", cmd)
        if m and not os.path.exists(m.group(1)): bad[path].append(("missing-path", c["name"], m.group(1), ""))
for path, rows in bad.items():
    cnt = collections.Counter(r[0] for r in rows)
    print(path, dict(cnt))
    pk = collections.Counter((r[0], r[2]) for r in rows)
    for (k, v), n in pk.most_common(40): print("  ", n, k, v[:200])
