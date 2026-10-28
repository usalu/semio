"""Staging engine of disk-g: anchored text patches and new files for the shared build library, applied by gate-g (never live by this lane).
Usage: python3 -I stage.py <check|apply|revert|bases|newonly> <batch> [--root <checkout>] [--backup-dir <dir>] [--force] [--tolerate-missing]
A batch is `batches/<batch>.py` exporting EDITS [(rel, [(old, new), ...])], NEW [(rel, file name in staged/<batch>/files)] and optional DELETE [rel] and PRECONDITION (callable root -> error text or None).
check: every edited file is `base` (each old anchor occurs exactly once) or `applied` (each new text present, each old absent); exit 0 when all files are consistently base or applied.
apply: backs the originals up once to backup/<batch>/ and writes. revert: restores them. bases: writes staged/<batch>/bases.json (sha256 of every edited file now). newonly: writes only the new files (the laws of a batch against the unedited tree: they must fail)."""
import hashlib, importlib.util, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
MARKER, AGENTS = "📋️project.json", "AGENTS.md"


def find_root():
    probe = HERE
    while True:
        if os.path.isfile(os.path.join(probe, MARKER)) and os.path.isfile(os.path.join(probe, AGENTS)):
            return probe
        parent = os.path.dirname(probe)
        if parent == probe:
            raise SystemExit("stage: checkout root not found (pass --root)")
        probe = parent


def load(batch):
    spec = importlib.util.spec_from_file_location("batch_" + batch, os.path.join(HERE, "batches", batch + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def file_state(text, patches):
    """applied: every non-empty new text is present and every deleted anchor is gone; base: every old anchor occurs exactly once."""
    if all((old not in text or old in new) and (new == "" or new in text) for old, new in patches):
        return "applied"
    if all(text.count(old) == 1 for old, _ in patches):
        return "base"
    return "drift: anchors " + ",".join(str(text.count(old)) for old, _ in patches)


def apply_patches(text, patches):
    for old, new in patches:
        if text.count(old) != 1:
            raise SystemExit("stage: anchor not unique/present: " + old[:90])
        text = text.replace(old, new, 1)
    return text


def main(argv):
    if len(argv) < 3 or argv[1] not in ("check", "apply", "revert", "bases", "newonly"):
        raise SystemExit(__doc__)
    operation, batch = argv[1], argv[2]
    root = os.path.abspath(argv[argv.index("--root") + 1]) if "--root" in argv else find_root()
    force = "--force" in argv
    tolerate = "--tolerate-missing" in argv
    module = load(batch)
    files = os.path.join(HERE, "staged", batch, "files")
    backup = os.path.join(os.path.abspath(argv[argv.index("--backup-dir") + 1]) if "--backup-dir" in argv else os.path.join(HERE, "backup"), batch)
    index_file = os.path.join(backup, "index.json")
    edits, new_files, deletes = module.EDITS, module.NEW, getattr(module, "DELETE", [])
    precondition = getattr(module, "PRECONDITION", None)
    if operation == "bases":
        rows = {rel: sha(read(os.path.join(root, rel))) for rel, _ in edits if os.path.isfile(os.path.join(root, rel))}
        for rel, _ in new_files:
            rows[rel] = "absent" if not os.path.exists(os.path.join(root, rel)) else sha(read(os.path.join(root, rel)))
        os.makedirs(os.path.join(HERE, "staged", batch), exist_ok=True)
        with open(os.path.join(HERE, "staged", batch, "bases.json"), "w", encoding="utf8") as handle:
            json.dump(rows, handle, ensure_ascii=False, indent=1, sort_keys=True)
        print("bases:", len(rows), "files")
        return 0
    if operation == "newonly":
        for rel, name in new_files:
            path = os.path.join(root, rel)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            shutil.copyfile(os.path.join(files, name), path)
        print("newonly:", len(new_files), "files")
        return 0
    if operation == "check":
        verdicts = {}
        for rel, patches in edits:
            path = os.path.join(root, rel)
            if not os.path.isfile(path):
                verdicts[rel] = "missing file"
                continue
            text = read(path).decode("utf8")
            verdicts[rel] = file_state(text, patches)
        for rel, name in new_files:
            path = os.path.join(root, rel)
            want = read(os.path.join(files, name))
            verdicts[rel] = "base" if not os.path.exists(path) else "applied" if read(path) == want else "drift: exists with other bytes"
        for rel in deletes:
            verdicts[rel] = "base" if os.path.exists(os.path.join(root, rel)) else "applied"
        for rel, verdict in verdicts.items():
            print(verdict.split(":")[0].upper().ljust(8), rel[-110:], ("" if ":" not in verdict else verdict))
        problem = precondition(root) if precondition else None
        if problem:
            print("PRECONDITION", problem)
        kinds = {verdict.split(":")[0] for verdict in verdicts.values()}
        print("summary:", ", ".join(sorted(kinds)), "(%d files)" % len(verdicts))
        return 0 if kinds <= {"base"} and not problem or kinds == {"applied"} else 1
    if operation == "apply":
        problem = precondition(root) if precondition else None
        if problem and not force:
            print("PRECONDITION", problem)
            return 3
        if not os.path.isfile(index_file):
            shutil.rmtree(backup, ignore_errors=True)
        os.makedirs(backup, exist_ok=True)
        recorded = json.load(open(index_file, encoding="utf8")) if os.path.isfile(index_file) else {}
        plan = []
        for rel, patches in edits:
            path = os.path.join(root, rel)
            if tolerate and not os.path.isfile(path):
                print("skipped (not in this tree):", rel[-80:])
                continue
            text = read(path).decode("utf8")
            if file_state(text, patches) == "applied":
                continue
            plan.append((rel, path, text, apply_patches(text, patches)))
        for rel, name in new_files:
            path = os.path.join(root, rel)
            if os.path.exists(path) and read(path) != read(os.path.join(files, name)):
                raise SystemExit("stage: new file exists with other bytes: " + rel)
        for rel, path, text, result in plan:
            slot = recorded.setdefault(rel, {"backup": "%03d.orig" % len(recorded), "base": sha(text.encode("utf8"))})
            if not os.path.exists(os.path.join(backup, slot["backup"])):
                with open(os.path.join(backup, slot["backup"]), "wb") as handle:
                    handle.write(text.encode("utf8"))
            slot["applied"] = sha(result.encode("utf8"))
        for rel, name in new_files:
            recorded.setdefault(rel, {"backup": None, "base": "absent", "applied": sha(read(os.path.join(files, name)))})
        for rel in deletes:
            path = os.path.join(root, rel)
            if os.path.exists(path):
                slot = recorded.setdefault(rel, {"backup": "%03d.orig" % len(recorded), "base": sha(read(path)), "applied": "deleted"})
                shutil.copyfile(path, os.path.join(backup, slot["backup"]))
        with open(index_file, "w", encoding="utf8") as handle:
            json.dump(recorded, handle, ensure_ascii=False, indent=1)
        for rel, path, text, result in plan:
            temporary = path + ".stage.tmp"
            with open(temporary, "wb") as handle:
                handle.write(result.encode("utf8"))
            os.chmod(temporary, os.stat(path).st_mode & 0o7777)
            os.replace(temporary, path)
        for rel, name in new_files:
            path = os.path.join(root, rel)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            shutil.copyfile(os.path.join(files, name), path)
        for rel in deletes:
            path = os.path.join(root, rel)
            if os.path.exists(path):
                os.remove(path)
        print("applied: %d edited, %d new, %d deleted" % (len(plan), len(new_files), len(deletes)))
        return 0
    recorded = json.load(open(index_file, encoding="utf8")) if os.path.isfile(index_file) else None
    if recorded is None:
        raise SystemExit("stage: nothing recorded to revert for " + batch)
    for rel, slot in recorded.items():
        path = os.path.join(root, rel)
        if slot["backup"] is None:
            if os.path.exists(path):
                if sha(read(path)) != slot["applied"] and not force:
                    raise SystemExit("stage: new file changed since apply, not removed (use --force): " + rel)
                os.remove(path)
            continue
        original = read(os.path.join(backup, slot["backup"]))
        if slot["applied"] != "deleted" and os.path.exists(path) and sha(read(path)) != slot["applied"] and not force:
            raise SystemExit("stage: file changed since apply, not restored (use --force): " + rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as handle:
            handle.write(original)
    shutil.rmtree(backup, ignore_errors=True)
    print("reverted:", len(recorded), "files")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
