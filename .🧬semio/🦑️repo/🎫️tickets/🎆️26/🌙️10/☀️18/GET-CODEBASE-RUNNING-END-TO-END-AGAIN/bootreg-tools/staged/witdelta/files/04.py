"""🐍 Independent oracle for the Cargo preparation witness: decides from the witness file alone whether a launcher may skip the preparation child.
usage: python3 -I 🐍️.py <schema.json> <storage.json> <root> <manifest> <environment.json> <package>...   prints true|false"""
import hashlib, json, os, stat, sys
schema, storage, root, manifest, environment, packages = json.load(open(sys.argv[1], encoding="utf8")), json.load(open(sys.argv[2], encoding="utf8")), sys.argv[3], sys.argv[4], json.load(open(sys.argv[5], encoding="utf8")), sys.argv[6:]
protocol = schema["definitions"]["Protocol"]["const"]
def sha(data): return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf8")).hexdigest()
def dumps(value): return json.dumps(value, separators=(",", ":"), ensure_ascii=False)
def kind(path):
    try: mode = os.lstat(path).st_mode
    except FileNotFoundError: return "absent"
    return "symlink" if stat.S_ISLNK(mode) else "file" if stat.S_ISREG(mode) else "directory" if stat.S_ISDIR(mode) else "other"
def under_directories(path):
    rel = os.path.relpath(path, root)
    if rel == ".." or rel.startswith(".." + os.sep) or os.path.isabs(rel): return False
    current = root
    if kind(current) != "directory": return False
    for part in [p for p in rel.split(os.sep) if p and p != "."]:
        current = os.path.join(current, part)
        if kind(current) != "directory": return False
    return True
def stat_text(path):
    s = os.lstat(path)
    return ":".join(str(v) for v in (s.st_dev, s.st_ino, s.st_size, s.st_mtime_ns, s.st_ctime_ns))
def names_of(path):
    return {e.name for e in os.scandir(path) if e.is_dir(follow_symlinks=False) or (e.is_file(follow_symlinks=False) and e.name == protocol["listedFile"])}
def skipped(name): return name.startswith(".") or name in protocol["skippedNames"]
def manifest_free(path, name):
    if name == protocol["listedFile"]: return False
    if skipped(name): return True
    seen = 0
    for top, dirs, files in os.walk(path, followlinks=False):
        seen += len(dirs) + len(files)
        if seen > protocol["maximumAppearedEntries"]: return False
        dirs[:] = [d for d in dirs if not skipped(d) and not os.path.islink(os.path.join(top, d))]
        if protocol["listedFile"] in files and not os.path.islink(os.path.join(top, protocol["listedFile"])): return False
    return True
vanished = []
def inside(path): return any(path == p or path.startswith(p + os.sep) for p in vanished)
def replay(row):
    path, op = row["path"], row["operation"]
    if not row["stamp"]: return False
    if inside(path): return not (op == "text" or (op == "presence" and row["stamp"] == "true") or (op == "state" and row["stamp"] == "file"))
    if op == "list":
        if not under_directories(path): return False
        if row.get("stat") == stat_text(path): return True
        now = names_of(path)
        before = row.get("entries")
        if before is None: return sha(dumps(sorted(now))) == row["stamp"]
        if sha(dumps(before)) != row["stamp"]: return False
        if set(before) == now: return True
        vanished.extend(os.path.join(path, n) for n in set(before) - now)
        return all(manifest_free(os.path.join(path, n), n) for n in now - set(before))
    if not under_directories(root if path == root else os.path.dirname(path)): return False
    current = kind(path)
    if op == "state": return current != "absent" and current == row["stamp"]
    if op == "presence": return current != "symlink" and str(current != "absent").lower() == row["stamp"]
    if current != "file": return False
    return row["stamp"] == (row["stamp"] if row.get("stat") == stat_text(path) else sha(open(path, "rb").read().decode("utf8", "replace")))
def compiler_environment(env):
    rule = protocol["compilerEnvironment"]
    rows = [(k, v) for k, v in env.items() if (k.startswith(rule["prefix"]) and not any(k.startswith(p) for p in rule["excludedPrefixes"]) and k not in rule["excludedNames"]) or k in rule["names"]]
    return dict(sorted(rows))
def global_configuration(env):
    home = env.get("CARGO_HOME") or os.path.join(os.path.expanduser("~"), ".cargo")
    out = []
    for name in protocol["globalConfiguration"]:
        path = os.path.join(home, name)
        out.append({"path": path, "sha256": sha(open(path, "rb").read()) if os.path.exists(path) else None})
    return out
def file_sha(path):
    return sha(open(path, "rb").read()) if os.path.lexists(path) else None
def holds():
    key = sha(dumps([root, manifest, sorted(packages)]))
    try:
        witness = json.load(open(os.path.join(storage["directory"], key + protocol["witnessSuffix"]), encoding="utf8"))
        receipt = json.load(open(os.path.join(storage["directory"], key + protocol["receiptSuffix"]), encoding="utf8"))
    except (OSError, ValueError): return False
    hashed = witness["hashed"]
    if receipt != dict(witness["receipt"], version=1): return False
    if sha(dumps(hashed)) != receipt["inputsSha256"] or hashed["recipes"]: return False
    if dumps(hashed["compilerEnvironment"]) != dumps(compiler_environment(environment)) or dumps(hashed["globalConfiguration"]) != dumps(global_configuration(environment)): return False
    if file_sha(witness["lock"]) != receipt["lockSha256"]: return False
    if any(file_sha(s["path"]) != s["sha256"] for s in hashed["sources"]): return False
    if any(file_sha(i["path"]) != i["sha256"] for i in hashed["inputs"] + hashed["configuration"]): return False
    for tool in witness["tools"]:
        s = os.lstat(tool["path"])
        if ":".join(["file" if stat.S_ISREG(s.st_mode) else "other", str(s.st_dev), str(s.st_ino), str(s.st_size), str(s.st_mtime_ns), str(s.st_ctime_ns)]) != tool["stamp"]: return False
    return all(replay(row) for row in witness["observations"])
print("true" if holds() else "false")
