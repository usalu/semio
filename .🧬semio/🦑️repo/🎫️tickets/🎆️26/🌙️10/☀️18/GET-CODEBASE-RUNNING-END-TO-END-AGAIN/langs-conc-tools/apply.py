#!/usr/bin/env python3
# 🚀 Applies the langs-conc set (pinned native toolchains, isolated Python environments, .NET SDK pin and out-of-tree artifacts, pinned CMake and Ninja, Go alignment, toolchain identity keys, hermetic toolchain classes) all or nothing: creates the new members and edits every existing file at an anchor that must occur exactly once in the CURRENT text, else the whole set is refused as drift. Dry run by default; --apply writes (backups under C/🗑️generated/langs-conc/); --undo restores while nothing moved. Environment: LC_ROOT redirects every path to a verification overlay (the live checkout needs --apply).
import hashlib, json, os, platform, shutil, subprocess, sys, tarfile, tempfile, urllib.request, zipfile
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import layout
from layout import ROOT, REAL, NEW
import overlay as ov
import patches
STAGED = os.environ.get("LC_STAGED") or os.path.join(HERE, "staged")
OVERLAY = ROOT != REAL
BACKUP = (os.path.normpath(HERE + "/../🗑️generated/langs-conc/apply-backup") if "🧬semio" in HERE else HERE + "/backup") + ("-overlay-" + hashlib.sha256(ROOT.encode()).hexdigest()[:8] if OVERLAY else "")
def sha(data): return hashlib.sha256(data).hexdigest()
def rel(path): return os.path.relpath(path, ROOT)
def read(path):
    try: return open(path, "rb").read()
    except FileNotFoundError: return None
def plan():
    refused, ops = [], []
    for flat, dest in NEW.items():
        data = open(STAGED + "/" + flat, "rb").read(); current = read(dest)
        if current != data: ops.append({"path": dest, "before": current, "after": data})
    chains = {}
    for path, edit in patches.edits(): chains.setdefault(path, []).append(edit)
    for path, fns in chains.items():
        current = read(path)
        if current is None: refused.append("missing: " + rel(path)); continue
        try:
            text, changed = current.decode("utf8"), False
            for edit in fns:
                text, c = edit(text); changed = changed or c
        except SystemExit as error: refused.append("%s: %s" % (rel(path), error)); continue
        if changed: ops.append({"path": path, "before": current, "after": text.encode("utf8")})
    for path, data in patches.writes():
        current = read(path)
        if current != data: ops.append({"path": path, "before": current, "after": data})
    return ops, refused
def replace(path, data):
    if OVERLAY: return ov.write(ROOT, path, data)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".lc-")
    with os.fdopen(handle, "wb") as out: out.write(data)
    os.chmod(temporary, os.stat(path).st_mode & 0o7777 if os.path.exists(path) else 0o644)
    os.replace(temporary, path)
def remove(path):
    if OVERLAY: return ov.remove(ROOT, path)
    os.path.exists(path) and os.remove(path)
def restore(done):
    for op in reversed(done):
        if op["before"] is None: remove(op["path"])
        else: replace(op["path"], op["before"])
def undo():
    manifest = json.load(open(BACKUP + "/manifest.json"))
    for row in manifest:
        current = read(row["path"])
        if current is None or sha(current) != row["after"]: sys.exit("REFUSED: changed since apply: " + rel(row["path"]))
    for row in reversed(manifest):
        if row["backup"] is None: remove(row["path"])
        else: replace(row["path"], open(BACKUP + "/" + row["backup"], "rb").read())
    print("undone", len(manifest), "files")
def host_name():
    system = {"darwin": "darwin", "linux": "linux", "win32": "win32"}[sys.platform if sys.platform in ("darwin", "win32") else "linux"]
    machine = platform.machine().lower()
    return system + "-" + ("arm64" if machine in ("arm64", "aarch64") else "x64")
def sha_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""): digest.update(chunk)
    return digest.hexdigest()
def go_pin():
    policy = json.load(open(STAGED + "/toolchains-policy.json", encoding="utf8"))
    host, spec = host_name(), policy["go"]["tool"]
    row = next(item for item in spec["distributions"] if item["host"] == host)
    tools = os.path.join(ROOT, *policy["roots"]["tools"].split("/"))
    directory = os.path.join(tools, "native", "go", spec["version"], host + "-" + row["sha256"][:16])
    return policy, spec, row, directory
def go_provisioned():
    policy, spec, row, directory = go_pin()
    try: receipt = json.load(open(os.path.join(directory, ".toolchain.json"), encoding="utf8"))
    except (OSError, ValueError): return False
    if receipt.get("owner") != "workspace:native-tools" or receipt.get("identity") != "go:%s:%s:%s" % (spec["version"], host_name(), row["sha256"]): return False
    base = "" if row["binaryDirectory"] == "." else row["binaryDirectory"] + "/"
    return all(receipt.get("files", {}).get(base + name) == (sha_file(os.path.join(directory, *(base + name).split("/"))) if os.path.isfile(os.path.join(directory, *(base + name).split("/"))) else None) for name in row["binaries"])
def provision_go():
    # 🐹 Provisions the repository's pinned Go into the tools partition exactly like the TypeScript preparer (same directory, same receipt): pinned size and SHA-256, the host's unpacker, one atomic rename; the Go on PATH is never consulted and never modified.
    policy, spec, row, directory = go_pin()
    if go_provisioned(): return directory
    parent = os.path.dirname(directory); os.makedirs(parent, exist_ok=True)
    temporary = tempfile.mkdtemp(prefix=".prepare-", dir=parent)
    try:
        archive = os.path.join(temporary, row["archive"])
        print("downloading the pinned Go %s for %s (%d bytes)" % (spec["version"], host_name(), row["bytes"]))
        digest, size = hashlib.sha256(), 0
        with urllib.request.urlopen(spec["release"] + "/" + row["archive"], timeout=120) as response, open(archive, "wb") as out:
            for chunk in iter(lambda: response.read(1 << 20), b""):
                size += len(chunk)
                if size > row["bytes"]: raise RuntimeError("archive exceeds its pinned size")
                digest.update(chunk); out.write(chunk)
        if size != row["bytes"] or digest.hexdigest() != row["sha256"]: raise RuntimeError("archive does not match its pinned size and SHA-256")
        extracted = os.path.join(temporary, "extracted"); os.makedirs(extracted)
        if row["format"] == "zip":
            with zipfile.ZipFile(archive) as bundle: bundle.extractall(extracted)
        else:
            with tarfile.open(archive, "r:gz") as bundle:
                for member in bundle.getmembers():
                    if not (member.isreg() or member.isdir()) or member.name.startswith("/") or ".." in member.name.split("/"): raise RuntimeError("unsafe archive member: " + member.name)
                bundle.extractall(extracted)
        payload = os.path.join(extracted, row["prefix"]) if row["prefix"] else extracted
        base = "" if row["binaryDirectory"] == "." else row["binaryDirectory"] + "/"
        files = {}
        for name in row["binaries"]:
            path = os.path.join(payload, *(base + name).split("/"))
            if not os.path.isfile(path): raise RuntimeError("archive holds no " + name)
            if os.name != "nt": os.chmod(path, 0o755)
            files[base + name] = sha_file(path)
        reported = subprocess.run([os.path.join(payload, *(base + row["binaries"][0]).split("/"))] + spec["versionArguments"], capture_output=True, text=True, timeout=60).stdout
        if spec["version"] not in reported: raise RuntimeError("version mismatch: " + reported.strip())
        open(os.path.join(payload, ".toolchain.json"), "w", encoding="utf8").write(json.dumps({"owner": "workspace:native-tools", "identity": "go:%s:%s:%s" % (spec["version"], host_name(), row["sha256"]), "files": files}) + "\n")
        try: os.rename(payload, directory)
        except OSError:
            if not os.path.isdir(directory): raise
    finally:
        shutil.rmtree(temporary, ignore_errors=True)
    if not go_provisioned(): raise RuntimeError("the provisioned Go differs from its receipt")
    return directory
def host_report():
    policy, spec, row, directory = go_pin()
    print("  pinned Go %-10s %s" % (spec["version"], "provisioned at " + os.path.relpath(directory, ROOT) if go_provisioned() else "NOT PROVISIONED (the live apply provisions it from the pinned archive; nothing on PATH is used)"))
def main():
    if "--undo" in sys.argv: return undo()
    if "--check-host" in sys.argv: print("Repository toolchains (informational; the Go on PATH is never consulted):"); host_report(); return
    if OVERLAY: ov.base(ROOT); ov.copies(ROOT); ov.node_modules(ROOT); ov.isolated_git(ROOT)
    ops, refused = plan()
    created = sum(1 for op in ops if op["before"] is None)
    print("langs-conc set: %d file operation(s): %d created, %d edited%s" % (len(ops), created, len(ops) - created, " [overlay " + ROOT + "]" if OVERLAY else ""))
    if "--list" in sys.argv:
        for op in ops: print("  ", "create" if op["before"] is None else "edit  ", rel(op["path"]))
    if refused:
        print("REFUSED (nothing written):"); [print("  -", row) for row in refused]; sys.exit(1)
    if "--apply" not in sys.argv: return print("dry run: nothing written; rerun with --apply (add --list to name every file, --check-host to compare the host toolchains with the pins)")
    if not OVERLAY and "--coordinator-go" not in sys.argv: sys.exit("REFUSED (nothing written): applying to the live checkout needs the coordinator's GO; pass --coordinator-go together with --apply.")
    if not OVERLAY and not go_provisioned():
        try: print("pinned Go provisioned:", os.path.relpath(provision_go(), ROOT))
        except Exception as error: sys.exit("REFUSED (nothing written): the pinned Go could not be provisioned: %s" % error)
    os.makedirs(BACKUP, exist_ok=True)
    manifest, done = [], []
    try:
        for op in ops:
            if read(op["path"]) != op["before"]: raise RuntimeError("changed during apply: " + rel(op["path"]))
        for index, op in enumerate(ops):
            name = None if op["before"] is None else "%03d.bak" % index
            if name: open(BACKUP + "/" + name, "wb").write(op["before"])
            replace(op["path"], op["after"]); done.append(op)
            manifest.append({"path": op["path"], "backup": name, "after": sha(op["after"])})
        json.dump(manifest, open(BACKUP + "/manifest.json", "w"), ensure_ascii=False, indent=1)
    except Exception as error:
        restore(done); sys.exit("REFUSED, all changes rolled back: %s" % error)
    print("applied.")
main()
