"""🐍️ Independent oracle of the repository walks (Python standard library only): replays edit sequences on its own copy of a tree and answers, after every step, the crate index of the Cargo manifests and the TypeScript files that entry globs and tsconfig includes select, by walking the whole tree in place. usage: python3 -I oracle.py <input.json> -> JSON on stdout."""
import json
import os
import re
import shutil
import sys
import tempfile

SOURCE = re.compile(r"\.(?:ts|tsx|mts|cts)$")
SKIPPED = {"node_modules", ".git"}
WILDCARD = re.compile(r"[*?{\[]")
WORKSPACE = "{workspaceRoot}/"
_compiled = {}


def glob(pattern):
    if pattern in _compiled:
        return _compiled[pattern]
    out, depth, i = "", 0, 0
    while i < len(pattern):
        c = pattern[i]
        if c == "*":
            if pattern[i + 1:i + 2] != "*":
                out += "[^/]*"
            elif pattern[i + 2:i + 3] == "/":
                i += 2
                out += "(?:.*/)?"
            else:
                i += 1
                out += ".*"
        elif c == "?":
            out += "[^/]"
        elif c == "{":
            depth += 1
            out += "(?:"
        elif c == "}" and depth:
            depth -= 1
            out += ")"
        elif c == "," and depth:
            out += "|"
        else:
            out += re.escape(c)
        i += 1
    _compiled[pattern] = re.compile("^" + out + "$", re.S)
    return _compiled[pattern]


def normalize(path):
    parts = []
    for part in path.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            if parts:
                parts.pop()
        else:
            parts.append(part)
    return "/".join(parts)


def excluded(patterns):
    return lambda path: any(glob(p).match(path) or glob(p + "/**").match(path) for p in patterns)


class Tree:
    def __init__(self, root):
        self.root = root

    def local(self, path):
        return os.path.join(self.root, path) if path else self.root

    def entries(self, directory):
        try:
            with os.scandir(self.local(directory)) as scan:
                return [(e.name, e.is_dir(follow_symlinks=False), e.is_file(follow_symlinks=False), e.is_symlink()) for e in scan]
        except OSError:
            return []

    def exists(self, path):
        return os.path.exists(self.local(path))

    def text(self, path):
        try:
            with open(self.local(path), encoding="utf8") as handle:
                return handle.read()
        except OSError:
            return None


def package_name(text):
    inside = False
    for line in re.split(r"\r?\n", text):
        header = re.match(r"^\s*\[([^\]]+)\]\s*$", line)
        if header:
            inside = header.group(1).strip() == "package"
            continue
        found = re.match(r'^\s*name\s*=\s*"([^"]+)"', line) if inside else None
        if found:
            return found.group(1)
    return None


def crate_index(tree, skip):
    found = []
    paths, names = set(skip["paths"]), set(skip["names"])

    def walk(directory):
        for name, is_dir, is_file, is_link in tree.entries(directory):
            path = directory + "/" + name if directory else name
            if is_link or path in paths or name in names:
                continue
            if is_dir:
                walk(path)
            elif is_file and name == "Cargo.toml":
                found.append(directory)

    walk("")
    index = {}
    for directory in sorted(found):
        if any(part.startswith(".") for part in directory.split("/")):
            continue
        name = package_name(tree.text(directory + "/Cargo.toml" if directory else "Cargo.toml") or "")
        if name and name not in index:
            index[name] = directory
    return [[name, directory] for name, directory in index.items()]


def source_roots(tree, question):
    found = set()
    if question["mode"] == "entries":
        globs = question["globs"]
        strip = lambda g: normalize(g.replace(WORKSPACE, "", 1))
        is_excluded = excluded([strip(g[1:]) for g in globs if g.startswith("!")])
        for pattern in [strip(g) for g in globs if not g.startswith("!")]:
            stop = WILDCARD.search(pattern)
            if stop is None:
                if SOURCE.search(pattern) and tree.exists(pattern) and not is_excluded(pattern):
                    found.add(pattern)
                continue
            cut = pattern.rfind("/", 0, stop.start() + 1)
            pending = [normalize(pattern[:max(0, cut)])]
            while pending:
                directory = pending.pop()
                for name, is_dir, _, _ in tree.entries(directory):
                    path = directory + "/" + name if directory else name
                    if name in SKIPPED or name.startswith(".") or is_excluded(path):
                        continue
                    if is_dir:
                        pending.append(path)
                    elif SOURCE.search(path) and glob(pattern).match(path):
                        found.add(path)
    else:
        is_excluded = excluded(question["exclude"])
        for pattern in question["include"]:
            stop = WILDCARD.search(pattern)
            if stop is None:
                if SOURCE.search(pattern) and tree.exists(pattern):
                    found.add(pattern)
                continue
            cut = pattern.rfind("/", 0, stop.start() + 1)
            assert cut >= 0, "fixture include patterns name a directory before the first wildcard"
            pending = [normalize(pattern[:cut])]
            while pending:
                directory = pending.pop()
                for name, is_dir, _, _ in tree.entries(directory):
                    path = directory + "/" + name if directory else name
                    if name in SKIPPED or is_excluded(path):
                        continue
                    if is_dir:
                        pending.append(path)
                    elif SOURCE.search(path) and glob(pattern).match(path):
                        found.add(path)
    return sorted(found)


def write(root, files):
    for path, text in files.items():
        os.makedirs(os.path.dirname(os.path.join(root, path)) or root, exist_ok=True)
        with open(os.path.join(root, path), "w", encoding="utf8") as handle:
            handle.write(text)


def apply(root, op):
    kind = op["op"]
    if kind == "write":
        write(root, {op["path"]: op["text"]})
    elif kind == "mkdir":
        os.makedirs(os.path.join(root, op["path"]), exist_ok=True)
    elif kind == "remove":
        target = os.path.join(root, op["path"])
        if os.path.isdir(target) and not os.path.islink(target):
            shutil.rmtree(target)
        elif os.path.lexists(target):
            os.remove(target)
    elif kind == "rename":
        os.makedirs(os.path.dirname(os.path.join(root, op["to"])), exist_ok=True)
        os.rename(os.path.join(root, op["from"]), os.path.join(root, op["to"]))
    elif kind == "symlink":
        os.makedirs(os.path.dirname(os.path.join(root, op["path"])), exist_ok=True)
        os.symlink(op["target"], os.path.join(root, op["path"]))
    else:
        raise SystemExit("unknown op " + kind)


def answers(tree, spec):
    return {"crates": crate_index(tree, spec["skip"]), "sources": {name: source_roots(tree, question) for name, question in spec["questions"].items()}}


def main():
    spec = json.load(open(sys.argv[1], encoding="utf8"))
    report = []
    for sequence in spec["sequences"]:
        root = tempfile.mkdtemp(prefix="walk-oracle-")
        try:
            write(root, sequence["files"])
            tree, rows = Tree(root), []
            rows.append(answers(tree, spec))
            for step in sequence["steps"]:
                for op in step["ops"]:
                    apply(root, op)
                rows.append(answers(tree, spec))
            report.append(rows)
        finally:
            shutil.rmtree(root, ignore_errors=True)
    json.dump(report, sys.stdout, ensure_ascii=False)


main()
