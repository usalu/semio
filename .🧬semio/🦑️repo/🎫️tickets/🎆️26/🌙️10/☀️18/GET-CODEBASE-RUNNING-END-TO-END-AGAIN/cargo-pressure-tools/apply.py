#!/usr/bin/env python3
import hashlib, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


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
J = lambda *parts: os.path.join(*parts)
FRAMEWORK, MODULES, PROCESS, ARTIFACTS, NATIVE = E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process", E(0x1F4E6) + "artifacts", E(0x1F3D7) + "native-build"
SOURCE, ADMISSION, READ = E(0x1F4E5) + "source", E(0x1F39F) + "admission", E(0x1F4D6) + "read"
SCHEMA, TESTS, TS, JSON_ = E(0x1F9EC) + "schema", E(0x1F9EA) + "tests", E(0x1F7E6) + ".ts", E(0x1F523) + ".json"
PRODUCTS, REPO, LIBRARY = E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F4DA) + "library"
CACHING, STORAGE, ENFORCEMENT, COMPILER = E(0x26A1) + "caching", E(0x1F4BE) + "storage", E(0x1F512) + "enforcement", E(0x1F3D7) + "compiler"
CAPACITY, RECOVERY, PRESSURE, RECHECK = E(0x1F4CF) + "capacity", E(0x267B) + "recovery", E(0x267B) + "pressure", E(0x1F501) + "recheck"

ADM = J(FRAMEWORK, MODULES, PROCESS, ARTIFACTS, NATIVE, SOURCE, ADMISSION)
CAP = J(FRAMEWORK, PRODUCTS, REPO, MODULES, LIBRARY, CACHING, STORAGE, ENFORCEMENT, COMPILER, CAPACITY)
TARGET = {
    "admission": J(ADM, TS),
    "admission-schema": J(ADM, SCHEMA, JSON_),
    "read": J(ADM, READ, TS),
    "feature": J(ADM, TESTS, E(0x1F952) + ".feature"),
    "admission-test": J(ADM, TESTS, TS),
}
NEW = {
    "recheck.test.ts": J(ADM, TESTS, RECHECK, TS),
    "recheck.json": J(ADM, TESTS, RECHECK, JSON_),
    "pressure.test.ts": J(CAP, TESTS, PRESSURE, TS),
    "pressure.json": J(CAP, TESTS, PRESSURE, JSON_),
}
RECOVERY_FILE = J(CAP, RECOVERY, TS)


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf-8") as handle:
        return handle.read()


PATCHES = {
    "admission": [
        (staged("interface.old.txt"), staged("interface.new.txt")),
        (staged("port.old.txt"), staged("port.new.txt")),
        ('await publish("cfg");const limits=', 'await publish("cfg");await port.recheck(control,"unit");const limits='),
        ('await publish("complete");\n }\n}\n', 'await publish("complete");\n }\n await port.recheck(control,"complete");\n}\n'),
    ],
    "admission-schema": [
        ('    "readCheckpoint": {\n      "const": 8\n    }\n', '    "readCheckpoint": {\n      "const": 8\n    },\n    "storageRecheck": {\n      "const": {\n        "intervalMilliseconds": 15000\n      }\n    }\n'),
    ],
    "feature": [
        ("      | original cancellation |\n", "      | original cancellation |\n" + staged("feature.add.txt")),
    ],
    "admission-test": [
        ("{async cfg(unit){cfgCalls++;return[unit.compilerFlags[1]!];}", "{async recheck(){},async cfg(unit){cfgCalls++;return[unit.compilerFlags[1]!];}"),
        ("{async cfg(_unit,received){expect(received).toBe(control);return[];}", "{async recheck(){},async cfg(_unit,received){expect(received).toBe(control);return[];}"),
        ("{async cfg(_unit,received){expect(received).toBe(control);cfgCalls++;return[];}", "{async recheck(){},async cfg(_unit,received){expect(received).toBe(control);cfgCalls++;return[];}"),
        ("{async cfg(){return[];},async read(){reads++;", "{async recheck(){},async cfg(){return[];},async read(){reads++;"),
        ("{async cfg(){calls.cfg++;", "{async recheck(){},async cfg(){calls.cfg++;"),
    ],
    "read": [
        ("const bytes=Buffer.alloc(Number(facts.size));let offset=0;while(offset<bytes.byteLength){await recheck();", "const bytes=Buffer.alloc(Number(facts.size));let offset=0;while(offset<bytes.byteLength){if(offset>0)await recheck();"),
        ("for(let position=0;position<bytes.byteLength;){await recheck();", "for(let position=0;position<bytes.byteLength;){if(position>0)await recheck();"),
    ],
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


def apply_ops(text, ops, name):
    for index, (anchor, replacement) in enumerate(ops):
        found, done = text.count(anchor), text.count(replacement)
        if done == 1 and (found == 0 or anchor in replacement):
            continue
        if found == 1 and done == 0:
            text = text.replace(anchor, replacement)
            continue
        raise SystemExit("anchor %s[%d] occurs %d times and its replacement %d times (expected one of them exactly once): %s" % (name, index, found, done, anchor[:90]))
    return text


def patched_texts(strict=True):
    bases = json.load(open(bases_path()))
    out = {}
    for name, ops in PATCHES.items():
        raw = read(absolute(TARGET[name]))
        if strict and sha(raw) != bases[name]:
            raise SystemExit("target changed since staging: %s (rerun with --rebase to re-anchor the edits onto the current file)" % TARGET[name])
        out[name] = apply_ops(raw.decode("utf-8"), ops, name)
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
        if executable == "bun" and ("dependencies sync" in command or "dependencies lock" in command):
            found.append(line)
        elif executable in ("sh", "bash", "zsh") and any(name in command for name in ("deps4-run.sh", "deps4-loop.sh", "deps4-loop2.sh")) and "-c" not in words[1:2]:
            found.append(line)
    return found


def cmd_bases():
    data = {name: sha(read(absolute(path))) for name, path in TARGET.items()}
    json.dump(data, open(bases_path(), "w"), indent=1)
    print(json.dumps(data, indent=1))


def cmd_check(args):
    for name, text in patched_texts("--rebase" not in args).items():
        print("ok", name, len(text))
    for key, path in NEW.items():
        print("new", key, "EXISTS" if os.path.exists(absolute(path)) else "absent")


def cmd_apply(args):
    running = sync_running()
    if running and "--ignore-sync" not in args:
        print("refusing: a dependency sync is running (the freeze is not lifted):")
        for line in running:
            print("  " + line[:160])
        raise SystemExit(2)
    for key, path in NEW.items():
        if os.path.exists(absolute(path)) and read(absolute(path)) != read(os.path.join(STAGED, key)) and "--overwrite-new" not in args:
            raise SystemExit("refusing: new target already exists with different bytes: " + path)
    texts = patched_texts("--rebase" not in args)
    if "--dry-run" in args:
        for name, text in texts.items():
            print("would patch" if read(absolute(TARGET[name])).decode("utf-8") != text else "unchanged", name)
        for key, path in NEW.items():
            print("would write", key)
        return
    for name, text in texts.items():
        path = absolute(TARGET[name])
        if read(path).decode("utf-8") == text:
            print("unchanged", name)
            continue
        tmp = path + ".cargo-pressure.tmp"
        with open(tmp, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.replace(tmp, path)
        print("patched", name)
    for key, path in NEW.items():
        target = absolute(path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        data = read(os.path.join(STAGED, key))
        if not os.path.exists(target) or read(target) != data:
            with open(target, "wb") as handle:
                handle.write(data)
            print("wrote", key)


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
    entries = [(TARGET[name].split(os.sep), text.encode("utf-8")) for name, text in patched_texts("--rebase" not in args).items()]
    entries += [(path.split(os.sep), read(os.path.join(STAGED, key))) for key, path in NEW.items()]
    for relative in [args[i + 1] for i, value in enumerate(args) if value == "--copy" and i + 1 < len(args)]:
        for directory, _names, filenames in os.walk(absolute(relative)):
            for filename in filenames:
                full = os.path.join(directory, filename)
                if not os.path.islink(full):
                    entries.append((os.path.relpath(full, ROOT).split(os.sep), read(full)))
    if "--recovery-head" in args:
        head = subprocess.run(["git", "show", "HEAD:" + RECOVERY_FILE], cwd=ROOT, capture_output=True, check=True).stdout
        entries.append((RECOVERY_FILE.split(os.sep), head))
    if os.path.lexists(mirror):
        subprocess.run(["rm", "-rf", mirror], check=True)
    overlay(ROOT, mirror, entries)
    print("mirror", mirror)


FLAGS = {"--dry-run", "--ignore-sync", "--rebase", "--overwrite-new", "--recovery-head"}


def refuse_unknown(args):
    unknown = [word for index, word in enumerate(args) if word.startswith("-") and word not in FLAGS and not (index and args[index - 1] == "--copy")]
    if unknown:
        raise SystemExit("refusing unknown flag(s) " + " ".join(unknown))


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    refuse_unknown(sys.argv[2:])
    if command == "bases":
        cmd_bases()
    elif command == "check":
        cmd_check(sys.argv[2:])
    elif command == "apply":
        cmd_apply(sys.argv[2:])
    elif command == "mirror":
        cmd_mirror(sys.argv[2:])
    else:
        raise SystemExit("usage: apply.py bases|check [--rebase]|apply [--dry-run] [--rebase] [--ignore-sync] [--overwrite-new]|mirror DIR [--rebase] [--recovery-head] [--copy RELATIVE_DIR]...")
