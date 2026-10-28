#!/usr/bin/env python3
"""🔬 depinfo.py <tag> <target|ALL> <cargo-log.jsonl>... : the COMPILE-time leg. For the crates a target's cargo legs test it takes the path-dependency closure from the Cargo.toml files themselves (normal + build + dev dependencies of the tested crate, normal + build below; `workspace = true` resolved through [workspace.dependencies]; a plain regex reader, independent of the precision engine) and reads the rustc dep-info (`.d`, written by rustc) of every unit of those crates from the cargo build log: every workspace file rustc read (mod files, include_str!/include_bytes!) must be declared by the target. Streams the logs, RSS guard 600 MB; writes evidence/<tag>-<target>-depinfo.json."""
import json, os, re, resource, sys
W = "/Users/ueli/Documents/semio"
C = W + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
K, G = C + "/keys4-g-tools", C + "/🗑️generated/keys4-g"
FW = W + "/🧰️framework"
tag, which, logs = sys.argv[1], sys.argv[2], sys.argv[3:]
CRATES = json.load(open(K + "/crates-of-target.json", encoding="utf-8"))
names = list(CRATES) if which == "ALL" else which.split(",")
def read(path): return open(path, encoding="utf-8").read()
workspace_text = read(FW + "/Cargo.toml")
workspace_paths = {}
section = ""
for line in workspace_text.split("\n"):
    h = re.match(r"^\s*\[([^\]]+)\]", line)
    if h: section = h.group(1).strip(); continue
    if section == "workspace.dependencies":
        m = re.match(r'^\s*([A-Za-z0-9_\-]+)\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"', line)
        if m: workspace_paths[m.group(1)] = os.path.normpath(os.path.join(FW, m.group(2)))
    elif section.startswith("workspace.dependencies."):
        m = re.match(r'^\s*path\s*=\s*"([^"]+)"', line)
        if m: workspace_paths[section[len("workspace.dependencies."):]] = os.path.normpath(os.path.join(FW, m.group(1)))
manifests, package_of = {}, {}
for base, dirs, files in os.walk(FW):
    dirs[:] = [d for d in dirs if d not in ("node_modules", "target", ".cache", ".git")]
    if "Cargo.toml" in files:
        text = read(os.path.join(base, "Cargo.toml"))
        m = re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"', text, re.M | re.S)
        if m: package_of[m.group(1)] = base; manifests[base] = text
dir_name = {d: n for n, d in package_of.items()}
def enabled_features(text):
    features = {}
    block = re.search(r'^\[features\]\s*\n((?:.*\n)*?)(?=^\[|\Z)', text, re.M)
    if block:
        for m in re.finditer(r'^([A-Za-z0-9_\-]+)\s*=\s*\[([^\]]*)\]', block.group(1), re.M | re.S):
            features[m.group(1)] = re.findall(r'"([^"]+)"', m.group(2))
    on, queue = set(), ["default"]
    while queue:
        f = queue.pop()
        if f in on: continue
        on.add(f); queue += features.get(f, [])
    return on
def deps(base, with_dev):
    text = manifests[base]; found = []; on = enabled_features(text)
    def active(key, rest):
        if not re.search(r"optional\s*=\s*true", rest): return True
        return any(f == key or f == "dep:" + key or f.startswith(key + "/") or f.startswith("dep:" + key + "/") for f in on)
    section = ""
    for line in text.split("\n"):
        h = re.match(r"^\s*\[([^\]]+)\]", line)
        if h: section = h.group(1); continue
        if "dependencies" not in section: continue
        if "dev-dependencies" in section and not with_dev: continue
        sub = re.match(r"^dependencies\.([^\]]+)$|^.*dependencies\.(.+)$", section)
        m = re.match(r'^\s*([A-Za-z0-9_\-]+)\s*=\s*(.*)$', line)
        if not m: continue
        key, rest = m.group(1), m.group(2)
        p = re.search(r'path\s*=\s*"([^"]+)"', rest)
        if p and active(key, rest): found.append(os.path.normpath(os.path.join(base, p.group(1))))
        elif re.search(r"workspace\s*=\s*true", rest) and key in workspace_paths and active(key, rest): found.append(workspace_paths[key])
        elif re.search(r"workspace\s*=\s*true", rest):
            for n, d in workspace_paths.items(): pass
    for m in re.finditer(r'^\[(?:[^\]]*dependencies)\.([^\]]+)\]\s*\n((?:.*\n)*?)(?=\[|\Z)', text, re.M):
        body = m.group(2)
        p = re.search(r'^path\s*=\s*"([^"]+)"', body, re.M)
        if p: found.append(os.path.normpath(os.path.join(base, p.group(1))))
        elif re.search(r"workspace\s*=\s*true", body) and m.group(1) in workspace_paths: found.append(workspace_paths[m.group(1)])
    return [d for d in found if d in manifests]
def closure(root):
    start = package_of[root]; seen = set(); queue = [(start, True)]
    while queue:
        d, dev = queue.pop()
        if d in seen: continue
        seen.add(d)
        for n in deps(d, dev): queue.append((n, False))
    return {dir_name[d] for d in seen}
dinfos = {}
for log in logs:
    for line in open(log, encoding="utf-8"):
        try: row = json.loads(line)
        except Exception: continue
        if row.get("reason") != "compiler-artifact": continue
        pid = row["package_id"]; after = pid[pid.index("#") + 1:]
        name = after.split("@")[0] if "@" in after else pid[:pid.index("#")].split("/")[-1]
        for f in list(row.get("filenames", [])) + ([row["executable"]] if row.get("executable") else []):
            directory, base = os.path.dirname(f), os.path.basename(f)
            stem = re.sub(r"\.[^.\-]+$", "", re.sub(r"^lib", "", base))
            for cand in (os.path.join(directory, stem + ".d"), os.path.join(directory, base + ".d")):
                if os.path.exists(cand): dinfos.setdefault(name, set()).add(cand)
def parse(path):
    text = read(path); first = next((l for l in text.split("\n") if ": " in l), "")
    body = first[first.index(": ") + 2:]; out, cur, i = [], "", 0
    while i < len(body):
        c = body[i]
        if c == "\\" and i + 1 < len(body) and body[i + 1] == " ": cur += " "; i += 2; continue
        if c == " ":
            if cur: out.append(cur); cur = ""
        else: cur += c
        i += 1
    if cur: out.append(cur)
    return out
ambient = ("node_modules", ".cache", ".git", "target")
os.makedirs(K + "/evidence", exist_ok=True)
for target in names:
    declared = set(read(G + "/allow/" + target.replace("dash:", "") + ".allow").split("\n")) - {""}
    members = set()
    for root in CRATES.get(target, []):
        if root in package_of: members |= closure(root)
    reads, units, missing = set(), 0, []
    for member in sorted(members):
        infos = dinfos.get(member)
        if not infos: missing.append(member); continue
        for info in infos:
            units += 1; base = package_of[member]
            for entry in parse(info):
                absolute = entry if entry.startswith("/") else next((p for p in (os.path.join(FW, entry), os.path.join(base, entry), os.path.join(W, entry)) if os.path.exists(p)), os.path.join(W, entry))
                absolute = os.path.normpath(absolute)
                if not absolute.startswith(W + "/"): continue
                rel = absolute[len(W) + 1:]
                parts = rel.split("/")
                if any(p in ambient for p in parts) or parts[0].startswith(".🧬semio") and "cache" in rel: continue
                reads.add(rel)
    undeclared = sorted(p for p in reads if p not in declared)
    row = {"target": target, "crates": CRATES.get(target, []), "closureCrates": len(members), "depInfoUnits": units, "workspaceFilesRead": len(reads), "declared": len(declared), "undeclared": undeclared, "closureWithoutUnits": missing[:30]}
    json.dump(row, open(f"{K}/evidence/{tag}-{target.replace(':', '_')}-depinfo.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({"target": target, "crates": len(row["crates"]), "closure": len(members), "units": units, "read": len(reads), "undeclared": len(undeclared), "noUnits": len(missing), "sample": undeclared[:3]}, ensure_ascii=False))
assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 600 * 1048576, "RSS above 600 MB"
