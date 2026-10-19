#!/usr/bin/env python3
import hashlib, json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
BASE = os.path.join(STAGED, "base")


def E(code):
    return chr(code) + "️"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = find_root()
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BAZEL = os.path.join(LIBRARY, E(0x1F3D7) + "build", E(0x1F527) + "tools", E(0x1F3D7) + "bazel")
TESTS = os.path.join(BAZEL, E(0x1F9EA) + "tests")
SCHEMA = E(0x1F9EC) + "schema"
TS = E(0x1F7E6) + ".ts"
JSON_ = E(0x1F523) + ".json"
SCHEMA_JSON = E(0x1F9EC) + "schema.json"


def tests(directory, name):
    return os.path.join(TESTS, E(0x1F4E5) + directory, name)


TARGET = {
    "script.ts": os.path.join(BAZEL, E(0x1F4DC) + "script.ts"),
    "schema.json": os.path.join(BAZEL, SCHEMA, JSON_),
    "tests-main.ts": os.path.join(TESTS, TS),
    "tests-actions.ts": tests("actions", TS),
    "tests-fingerprint.ts": tests("fingerprint", TS),
}

NEW = {
    "sources-test.ts": tests("sources", TS),
    "sources-fixture.json": tests("sources", JSON_),
    "sources-schema.json": tests("sources", SCHEMA_JSON),
    "workspace-test.ts": tests("workspace", TS),
    "workspace-fixture.json": tests("workspace", JSON_),
    "workspace-schema.json": tests("workspace", SCHEMA_JSON),
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def absolute(relative):
    return os.path.join(ROOT, relative)


def bases_path():
    return os.path.join(HERE, "bases.json")


def staged(name):
    return read(os.path.join(STAGED, name))


def cmd_bases():
    data = {name: sha(read(absolute(path))) for name, path in TARGET.items()}
    json.dump(data, open(bases_path(), "w"), indent=1)
    print(json.dumps(data, indent=1))


def rebased(name):
    with tempfile.TemporaryDirectory() as scratch:
        diff = subprocess.run(["diff", "-u", os.path.join(BASE, name), os.path.join(STAGED, name)], capture_output=True)
        if diff.returncode == 0:
            return read(absolute(TARGET[name]))
        patch_file = os.path.join(scratch, "change.diff")
        with open(patch_file, "wb") as handle:
            handle.write(diff.stdout)
        out = os.path.join(scratch, "out")
        result = subprocess.run(["patch", "-s", "--no-backup-if-mismatch", "-o", out, absolute(TARGET[name]), patch_file], capture_output=True, text=True)
        if result.returncode != 0:
            raise SystemExit("cannot rebase %s onto the current file: %s" % (name, (result.stdout + result.stderr)[:300]))
        return read(out)


def result_texts(strict):
    bases = json.load(open(bases_path()))
    out = {}
    for name, path in TARGET.items():
        current = read(absolute(path))
        if sha(current) == bases[name]:
            out[name] = staged(name)
        elif strict:
            raise SystemExit("target changed since staging: %s (rerun with --rebase to apply the staged diff onto the current file)" % path)
        else:
            out[name] = rebased(name)
    return out


def sync_running():
    listing = subprocess.run(["ps", "-ww", "-eo", "pid,command"], capture_output=True, text=True).stdout
    found = []
    for line in listing.splitlines()[1:]:
        pid, _, command = line.strip().partition(" ")
        words = command.split()
        if not words or str(os.getpid()) == pid:
            continue
        executable = os.path.basename(words[0])
        if executable == "bun" and "dependencies sync" in command:
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def cmd_check(strict):
    for name, data in result_texts(strict).items():
        print("ok", name, len(data))
    for name, path in NEW.items():
        print("new", name, "EXISTS" if os.path.exists(absolute(path)) else "absent")


def cmd_apply(args):
    strict = "--rebase" not in args
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        raise SystemExit(2)
    for name, path in NEW.items():
        if os.path.exists(absolute(path)) and read(absolute(path)) != staged(name) and "--overwrite-new" not in args:
            raise SystemExit("refusing: new target already exists with different bytes: " + path)
    texts = result_texts(strict)
    if "--dry-run" in args:
        for name, data in texts.items():
            print("would patch" if read(absolute(TARGET[name])) != data else "unchanged", name)
        for name, path in NEW.items():
            print("would write", name, path)
        return
    for name, data in texts.items():
        path = absolute(TARGET[name])
        if read(path) == data:
            print("unchanged", name)
            continue
        temporary = path + ".bzlstage.tmp"
        with open(temporary, "wb") as handle:
            handle.write(data)
        os.replace(temporary, path)
        print("patched", name)
    for name, path in NEW.items():
        target = absolute(path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        data = staged(name)
        if not os.path.exists(target) or read(target) != data:
            with open(target, "wb") as handle:
                handle.write(data)
            print("wrote", name)


def overlay(real, mirror, entries):
    os.makedirs(mirror, exist_ok=True)
    children = {}
    for parts, payload in entries:
        children.setdefault(parts[0], []).append((parts[1:], payload))
    if os.path.isdir(real):
        for name in os.listdir(real):
            if name in children:
                continue
            link = os.path.join(mirror, name)
            if not os.path.lexists(link):
                os.symlink(os.path.join(real, name), link)
    for name, rest in children.items():
        if len(rest) == 1 and not rest[0][0]:
            with open(os.path.join(mirror, name), "wb") as handle:
                handle.write(rest[0][1])
        else:
            overlay(os.path.join(real, name), os.path.join(mirror, name), rest)


def cmd_mirror(args):
    mirror = os.path.abspath(args[0])
    entries = []
    baseline = "--base" in args
    for name, data in (({name: read(os.path.join(BASE, name)) for name in TARGET} if baseline else result_texts("--rebase" not in args))).items():
        entries.append((TARGET[name].split(os.sep), data))
    for name, path in ({} if baseline else NEW).items():
        entries.append((path.split(os.sep), staged(name)))
    patched = {tuple(parts) for parts, _ in entries}
    copies = [args[i + 1] for i, value in enumerate(args) if value == "--copy" and i + 1 < len(args)]
    for relative in copies:
        for directory, directories, filenames in os.walk(absolute(relative)):
            directories[:] = [name for name in directories if name != "node_modules"]
            for filename in filenames:
                full = os.path.join(directory, filename)
                parts = tuple(os.path.relpath(full, ROOT).split(os.sep))
                if os.path.islink(full) or parts in patched:
                    continue
                patched.add(parts)
                entries.append((list(parts), read(full)))
    lists = [args[i + 1] for i, value in enumerate(args) if value == "--copy-list" and i + 1 < len(args)]
    for listing in lists:
        for relative in open(listing, encoding="utf8").read().split("\n"):
            full = absolute(relative)
            parts = tuple(relative.split("/"))
            if relative and os.path.isfile(full) and not os.path.islink(full) and parts not in patched:
                patched.add(parts)
                entries.append((list(parts), read(full)))
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries)
    print("mirror", mirror)


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "bases":
        cmd_bases()
    elif command == "check":
        cmd_check("--rebase" not in sys.argv)
    elif command == "apply":
        cmd_apply(sys.argv[2:])
    elif command == "mirror":
        cmd_mirror(sys.argv[2:])
    else:
        raise SystemExit("usage: stage.py bases|check [--rebase]|apply [--dry-run] [--rebase] [--ignore-sync] [--overwrite-new]|mirror DIR [--base] [--rebase] [--copy RELATIVE_DIR]...")
