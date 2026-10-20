#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the conc-cargo set (dry run by default).

  apply.py check                      anchors, bases and new files, no writes
  apply.py apply [--write] [--groups lease,lane,...] [--ignore-sync] [--rebase]
  apply.py revert [--write]           restore the backups taken by apply
  apply.py bases                      record the sha256 of every edited live file as the staging base
  apply.py mirror DIR                 symlink overlay of the live tree with the patched files as real files
"""
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if not os.path.isdir(os.path.join(ROOT, "🧰️framework")):
    ROOT = "/Users/ueli/Documents/semio"
ROOT = os.environ.get("CONC_CARGO_ROOT", ROOT)
STAGED = os.path.join(HERE, "staged")
sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..")))
from winpath import resolve, iter_logical
BACKUP = os.environ.get("CONC_CARGO_BACKUP", os.path.join(HERE, "backup"))
BASES = os.path.join(HERE, "bases.json")
sys.path.insert(0, HERE)
import patches  # noqa: E402


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def staged_files():
    return iter_logical(STAGED)


def group_of(relative):
    if relative in patches.REWRITES:
        return patches.REWRITES[relative]
    return "lane"


REQUIRES = {"lane": {"lease"}, "admit": {"lane"}, "registry": {"lease"}, "buildscript": {"lane"}, "broker": {"buildscript"}}


def closure(groups):
    groups = set(groups)
    while True:
        more = set().union(*[REQUIRES.get(g, set()) for g in groups]) - groups
        if not more:
            return groups
        groups |= more


def plan(groups, rebase, upgrade=False):
    groups = closure(groups)
    bases = json.load(open(BASES, encoding="utf-8")) if os.path.exists(BASES) else {}
    ledger_path = os.path.join(BACKUP, "applied.json")
    ledger = json.load(open(ledger_path, encoding="utf-8")) if os.path.exists(ledger_path) else {}
    names_path = os.path.join(BACKUP, "edits.json")
    done = json.load(open(names_path, encoding="utf-8")) if os.path.exists(names_path) else {}
    writes, notes, errors = {}, [], []
    for target, edits in patches.EDITS.items():
        selected = [row for row in edits if row[0] in groups]
        if not selected:
            continue
        path = os.path.join(ROOT, target)
        if not os.path.exists(path):
            errors.append("missing target " + target)
            continue
        text = read(path)
        pending = []
        result = text
        for group, name, old, new in selected:
            applied = name in done.get(target, []) or (old not in result and (new == "" or new in result)) or (old in new and result.count(new) >= 1)
            if applied:
                notes.append(("same", target, name))
            elif result.count(old) == 1:
                pending.append(name)
                result = result.replace(old, new, 1)
            else:
                errors.append("anchor %s found %d times in %s" % (name, result.count(old), target))
        if pending and bases.get(target) and bases[target] != sha(text) and ledger.get(target) != sha(text) and not rebase:
            errors.append("base changed since staging: " + target + " (use --rebase after reading the diff)")
            continue
        for name in pending:
            notes.append(("edit", target, name))
        if result != text:
            writes[target] = result
    for relative in staged_files():
        source = read(resolve(STAGED, relative))
        target = os.path.join(ROOT, relative)
        if relative not in patches.REWRITES and not (groups & {"lane", "admit"}):
            continue
        if relative in patches.REWRITES:
            if patches.REWRITES[relative] not in groups:
                continue
            if not os.path.exists(target):
                errors.append("rewrite target missing " + relative)
                continue
            current = read(target)
            if current == source:
                notes.append(("same", relative, "rewrite"))
            elif bases.get(relative) in (None, sha(current)) or ledger.get(relative) == sha(current) or rebase:
                writes[relative] = source
                notes.append(("rewrite", relative, ""))
            else:
                errors.append("base changed since staging: " + relative)
            continue
        if os.path.exists(target):
            if read(target) == source:
                notes.append(("same", relative, "new"))
            elif upgrade and relative.startswith(patches.UC + os.sep):
                writes[relative] = source
                notes.append(("upgrade", relative, ""))
            else:
                errors.append("new file exists with other content: " + relative + " (set 1 applied earlier? use --upgrade)")
        else:
            writes[relative] = source
            notes.append(("new", relative, ""))
    return writes, notes, errors


def running_sync():
    out = subprocess.run(["ps", "-Ao", "pid,command"], capture_output=True, text=True).stdout
    return [line for line in out.splitlines() if ("repo dependencies sync" in line or "repo dependencies lock" in line or "repo show projects" in line) and "apply.py" not in line]


def main(argv):
    command = argv[0] if argv else "check"
    flags = set(argv[1:])
    groups = set(patches.GROUPS)
    for index, value in enumerate(argv):
        if value == "--groups":
            groups = set(argv[index + 1].split(","))
    if command == "bases":
        rows = {}
        for target in list(patches.EDITS) + list(patches.REWRITES):
            path = os.path.join(ROOT, target)
            if os.path.exists(path):
                rows[target] = sha(read(path))
        json.dump(rows, open(BASES, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        print("recorded", len(rows), "bases")
        return 0
    if command in ("check", "apply"):
        writes, notes, errors = plan(groups, "--rebase" in flags, "--upgrade" in flags)
        for kind, target, name in notes:
            print("%-8s %s %s" % (kind, target, name))
        for error in errors:
            print("ERROR", error)
        print("%d write(s), %d error(s)" % (len(writes), len(errors)))
        if command == "check" or errors:
            return 1 if errors else 0
        busy = running_sync()
        if busy and "--ignore-sync" not in flags:
            print("REFUSED while these run (use --ignore-sync to override):")
            for line in busy:
                print("  ", line[:160])
            return 2
        if "--write" not in flags:
            print("dry run: nothing written (add --write)")
            return 0
        for relative, text in writes.items():
            path = os.path.join(ROOT, relative)
            if os.path.exists(path) and not os.path.exists(os.path.join(BACKUP, relative + ".absent")):
                backup = os.path.join(BACKUP, relative)
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                if not os.path.exists(backup):
                    shutil.copy2(path, backup)
            else:
                marker = os.path.join(BACKUP, relative + ".absent")
                os.makedirs(os.path.dirname(marker), exist_ok=True)
                open(marker, "w").close()
        for relative, text in writes.items():
            path = os.path.join(ROOT, relative)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            temporary = path + ".conc-cargo-tmp"
            with open(temporary, "w", encoding="utf-8") as handle:
                handle.write(text)
            os.replace(temporary, path)
        names_path = os.path.join(BACKUP, "edits.json")
        done = json.load(open(names_path, encoding="utf-8")) if os.path.exists(names_path) else {}
        for target, edits in patches.EDITS.items():
            for group, name, old, new in edits:
                if group in closure(groups) and target in writes and name not in done.setdefault(target, []):
                    done[target].append(name)
        json.dump(done, open(names_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        ledger = os.path.join(BACKUP, "applied.json")
        recorded = json.load(open(ledger, encoding="utf-8")) if os.path.exists(ledger) else {}
        recorded.update({relative: sha(text) for relative, text in writes.items()})
        json.dump(recorded, open(ledger, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        print("applied", len(writes), "file(s); backups in", BACKUP)
        return 0
    if command == "revert":
        ledger = os.path.join(BACKUP, "applied.json")
        recorded = json.load(open(ledger, encoding="utf-8")) if os.path.exists(ledger) else {}
        count, refused = 0, []
        for base, _, files in os.walk(BACKUP):
            for name in files:
                if name in ("applied.json", "edits.json"):
                    continue
                path = os.path.join(base, name)
                relative = os.path.relpath(path, BACKUP)
                absent = relative.endswith(".absent")
                live = relative[: -len(".absent")] if absent else relative
                target = os.path.join(ROOT, live)
                if os.path.exists(target) and recorded.get(live) and sha(read(target)) != recorded[live] and "--force" not in flags:
                    refused.append(live)
                    continue
                print("remove" if absent else "restore", live)
                if "--write" in flags:
                    if absent:
                        if os.path.exists(target):
                            os.remove(target)
                            directory = os.path.dirname(target)
                            while directory.startswith(ROOT) and directory != ROOT and not os.listdir(directory):
                                os.rmdir(directory)
                                directory = os.path.dirname(directory)
                    else:
                        shutil.copy2(path, target)
                count += 1
        for live in refused:
            print("REFUSED (changed since apply, use --force):", live)
        if "--write" in flags and not refused:
            shutil.rmtree(BACKUP, ignore_errors=True)
        print(count, "file(s)" + ("" if "--write" in flags else " (dry run, add --write)"), len(refused), "refused")
        return 1 if refused else 0
    if command == "mirror":
        overlay = argv[1]
        writes, _, errors = plan(set(patches.GROUPS), True)
        if errors:
            print("\n".join(errors))
            return 1
        shutil.rmtree(overlay, ignore_errors=True)
        os.makedirs(overlay)

        def mirror(directory, children):
            wanted = {}
            for child in children:
                head, _, rest = child.partition(os.sep)
                wanted.setdefault(head, []).append(rest)
            source, destination = os.path.join(ROOT, directory), os.path.join(overlay, directory)
            os.makedirs(destination, exist_ok=True)
            if os.path.isdir(source):
                for entry in os.listdir(source):
                    if entry not in wanted and not os.path.lexists(os.path.join(destination, entry)):
                        os.symlink(os.path.join(source, entry), os.path.join(destination, entry))
            for head, rests in wanted.items():
                if rests == [""]:
                    with open(os.path.join(destination, head), "w", encoding="utf-8") as handle:
                        handle.write(writes[os.path.join(directory, head)])
                else:
                    mirror(os.path.join(directory, head), [rest for rest in rests if rest])

        mirror("", list(writes))
        for entry in ("node_modules", "package.json", "tsconfig.json", "bunfig.toml"):
            if os.path.exists(os.path.join(ROOT, entry)) and not os.path.lexists(os.path.join(overlay, entry)):
                os.symlink(os.path.join(ROOT, entry), os.path.join(overlay, entry))
        print("mirror", overlay, len(writes), "file(s)")
        return 0
    print(__doc__)
    return 1


sys.exit(main(sys.argv[1:]))
