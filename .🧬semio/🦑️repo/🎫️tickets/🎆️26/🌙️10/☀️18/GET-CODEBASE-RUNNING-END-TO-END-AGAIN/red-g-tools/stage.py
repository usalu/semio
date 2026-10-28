"""🧷 red-g staging tool: anchored text edits on library/process/launcher files, handed to gate-g as one compile-atomic batch.

usage: python3 -I stage.py make   <batch>                 build <batch>/edits.json from <batch>/spec.json against the LIVE tree (base hashes, uniqueness, result hashes)
       python3 -I stage.py check  <batch> [--root DIR]    one line per file: base | applied | DRIFT-ANCHORED (anchors still unique, apply --rebase works) | DRIFT-BROKEN | MISSING
       python3 -I stage.py apply  <batch> [--root DIR] [--rebase]   all-or-nothing; refuses on DRIFT unless --rebase and every anchor is unique
       python3 -I stage.py revert <batch> [--root DIR]    restores the saved base bytes of an applied file (reverse replacement for a drifted one), all-or-nothing

spec.json = {"files": {"<repo-relative path>": [{"old": "...", "new": "..."}]}, "new": {"<repo-relative path>": "<file under <batch>/files/>"}}
Every `old` must occur exactly `count` times (default 1) in the base text; a new file has no base (check says `absent` before, `applied` after).
"""
import hashlib, json, os, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = os.path.abspath(os.path.join(HERE, *[".."] * 8))


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def write_atomic(path, data: bytes):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".red-g-")
    with os.fdopen(handle, "wb") as out:
        out.write(data)
    try:
        os.chmod(temporary, os.stat(path).st_mode & 0o7777)
    except FileNotFoundError:
        os.chmod(temporary, 0o644)
    os.replace(temporary, path)


def forward(text, edits):
    for edit in edits:
        count = edit.get("count", 1)
        if text.count(edit["old"]) != count:
            return None, edit["old"][:80]
        text = text.replace(edit["old"], edit["new"])
    return text, None


def backward(text, edits):
    for edit in reversed(edits):
        count = edit.get("count", 1)
        if text.count(edit["new"]) != count:
            return None, edit["new"][:80]
        text = text.replace(edit["new"], edit["old"])
    return text, None


def load(batch):
    with open(os.path.join(HERE, "staged", batch, "edits.json"), encoding="utf8") as handle:
        return json.load(handle)


def state(root, batch, entry):
    path = os.path.join(root, entry["file"])
    if not os.path.exists(path):
        return "absent" if entry.get("created") else "MISSING", None
    data = read(path)
    digest = sha(data)
    if digest == entry["applied"]:
        return "applied", data
    if digest == entry["base"]:
        return "base", data
    text = data.decode("utf8")
    if entry.get("created"):
        return "DRIFT-BROKEN", data
    ok, _ = forward(text, entry["edits"])
    if ok is not None:
        return "DRIFT-ANCHORED", data
    ok, _ = backward(text, entry["edits"])
    return ("DRIFT-APPLIED" if ok is not None else "DRIFT-BROKEN"), data


def make(batch):
    folder = os.path.join(HERE, "staged", batch)
    with open(os.path.join(folder, "spec.json"), encoding="utf8") as handle:
        spec = json.load(handle)
    out = {"batch": batch, "files": []}
    for file, edits in spec.get("files", {}).items():
        data = read(os.path.join(LIVE, file))
        result, bad = forward(data.decode("utf8"), edits)
        if result is None:
            sys.exit(f"anchor not unique or missing in {file}: {bad!r}")
        keep = os.path.join(folder, "base", sha(data)[:16])
        os.makedirs(os.path.dirname(keep), exist_ok=True)
        with open(keep, "wb") as handle:
            handle.write(data)
        out["files"].append({"file": file, "base": sha(data), "applied": sha(result.encode("utf8")), "edits": edits, "keep": sha(data)[:16]})
    for file, source in spec.get("new", {}).items():
        content = read(os.path.join(folder, "files", source))
        out["files"].append({"file": file, "created": True, "base": None, "applied": sha(content), "source": source})
    with open(os.path.join(folder, "edits.json"), "w", encoding="utf8") as handle:
        json.dump(out, handle, ensure_ascii=False, indent=1)
    print(f"{len(out['files'])} files staged for {batch}")


def run(command, batch, root, rebase):
    manifest = load(batch)
    rows = [(entry, *state(root, batch, entry)) for entry in manifest["files"]]
    if command == "check":
        for entry, status, _ in rows:
            print(status, entry["file"].replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "R/"))
        return 0
    plan = []
    for entry, status, data in rows:
        path = os.path.join(root, entry["file"])
        if command == "apply":
            if status == "applied":
                continue
            if entry.get("created"):
                if status != "absent":
                    sys.exit(f"refuse: {entry['file']} exists ({status})")
                plan.append((path, read(os.path.join(HERE, "staged", batch, "files", entry["source"]))))
                continue
            if status not in ("base", "DRIFT-ANCHORED") or (status == "DRIFT-ANCHORED" and not rebase):
                sys.exit(f"refuse: {entry['file']} is {status}")
            result, bad = forward(data.decode("utf8"), entry["edits"])
            if result is None:
                sys.exit(f"refuse: anchor {bad!r} not unique in {entry['file']}")
            plan.append((path, result.encode("utf8")))
        else:
            if status in ("base", "absent"):
                continue
            if entry.get("created"):
                if status != "applied":
                    sys.exit(f"refuse: {entry['file']} changed since apply")
                plan.append((path, None))
                continue
            if status == "applied":
                plan.append((path, read(os.path.join(HERE, "staged", batch, "base", entry["keep"]))))
                continue
            result, bad = backward(data.decode("utf8"), entry["edits"])
            if result is None:
                sys.exit(f"refuse: {entry['file']} is {status} and its new text {bad!r} is not unique")
            plan.append((path, result.encode("utf8")))
    for path, content in plan:
        if content is None:
            os.remove(path)
        else:
            write_atomic(path, content)
    print(f"{command}: {len(plan)} files changed")
    return 0


if __name__ == "__main__":
    arguments = sys.argv[1:]
    command, batch = arguments[0], arguments[1]
    root = LIVE
    if "--root" in arguments:
        root = os.path.abspath(arguments[arguments.index("--root") + 1])
    if command == "make":
        make(batch)
    else:
        sys.exit(run(command, batch, root, "--rebase" in arguments))
