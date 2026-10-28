"""📦️ quietfc-g staged3 (quietfc round 3: partial growth keeps only the jobs the memory carries): flat whole-file batch, `files/fNNN` + `base/fNNN` + `manifest.json`.
Usage: python3 -I stage.py check|apply|revert|rebase [--root DIR]
check: pending / applied / base-changed per file. apply: all-or-nothing, refuses (exit 2) on base-changed, backs the pending files up to `backup/`. revert: puts the backups back, byte-exact. rebase: for every base-changed file a 3-way merge (`git merge-file`: live, base/, files/) into `files/`, `base/` := live and the manifest hashes refreshed; refuses (exit 2) on a conflict and writes nothing."""
import hashlib, json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
MANIFEST = os.path.join(HERE, "manifest.json")
REPO = "/Users/ueli/Documents/semio"


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".quietfc-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def states(root, rows):
    out = {}
    for rel, row in rows.items():
        now = digest(os.path.join(root, rel))
        out[rel] = "applied" if now == row["new"] else "pending" if now == row["base"] else "base-changed"
    return out


def rebase(root, rows):
    merged, conflicts = {}, []
    with tempfile.TemporaryDirectory() as scratch:
        for rel, row in rows.items():
            live = os.path.join(root, rel)
            if digest(live) in (row["base"], row["new"]) or not os.path.isfile(live):
                continue
            current = os.path.join(scratch, row["file"] + ".cur")
            shutil.copyfile(live, current)
            result = subprocess.run(["git", "merge-file", "-p", current, os.path.join(HERE, "base", row["file"]), os.path.join(HERE, "files", row["file"])], capture_output=True)
            if result.returncode != 0:
                conflicts.append(rel)
            else:
                merged[rel] = (result.stdout, open(live, "rb").read())
    if conflicts:
        print("REFUSED, conflicts in:\n" + "\n".join(conflicts))
        sys.exit(2)
    for rel, (data, base) in merged.items():
        row = rows[rel]
        replace(os.path.join(HERE, "files", row["file"]), data)
        replace(os.path.join(HERE, "base", row["file"]), base)
        row["base"], row["new"] = hashlib.sha256(base).hexdigest(), hashlib.sha256(data).hexdigest()
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("rebased", len(merged), "files")


def main():
    mode, root = sys.argv[1], arg("--root", REPO)
    backups = arg("--backup", os.path.join(HERE, "backup"))
    rows = json.load(open(MANIFEST, encoding="utf-8"))
    if mode == "rebase":
        return rebase(root, rows)
    state = states(root, rows)
    if mode == "check":
        for rel, value in state.items():
            print(value, rel)
        return
    if mode == "apply":
        bad = [rel for rel, value in state.items() if value == "base-changed"]
        if bad:
            print("REFUSED, a peer edited these since staging (run `stage.py rebase`):\n" + "\n".join(bad))
            sys.exit(2)
        for rel, value in state.items():
            if value == "applied":
                continue
            target, backup = os.path.join(root, rel), os.path.join(backups, rows[rel]["file"])
            if os.path.exists(target):
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                shutil.copy2(target, backup)
            replace(target, open(os.path.join(HERE, "files", rows[rel]["file"]), "rb").read())
        print("applied", sum(1 for v in state.values() if v != "applied"), "files")
        return
    if mode == "revert":
        for rel, value in state.items():
            if value != "applied":
                continue
            target, backup = os.path.join(root, rel), os.path.join(backups, rows[rel]["file"])
            if os.path.exists(backup):
                replace(target, open(backup, "rb").read())
            elif rows[rel]["base"] is None:
                os.remove(target)
        print("reverted")
        return
    sys.exit("usage: stage.py check|apply|revert|rebase")


main()
