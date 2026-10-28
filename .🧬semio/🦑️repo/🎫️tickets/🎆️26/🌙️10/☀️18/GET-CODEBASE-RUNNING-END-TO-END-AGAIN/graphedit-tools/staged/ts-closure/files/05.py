"""🐍 Independent model of the declared program closure facts: the program is the config plus every file reachable from its `files` by relative imports, a program file's fact is its physical identity (a regular file reached through real directories), never its bytes."""
import json, posixpath, re, sys

IMPORT = re.compile(r'(?:import|export)\s+(?:type\s+)?(?:[^"\';]*?\s+from\s+)?["\'](\.[^"\']+)["\']')
REMOVING = {"delete", "symlink-file", "symlink-directory", "directory"}


def resolve(files, importer, spec):
    base = posixpath.normpath(posixpath.join(posixpath.dirname(importer), spec))
    for candidate in (base, base + ".ts", base + ".d.ts", base + "/index.ts"):
        if candidate in files and (candidate.endswith(".ts") or candidate.endswith(".json")):
            return candidate
    return None


def program(files, config):
    roots = json.loads(files[config]).get("files", [])
    seen, stack = set(), [root for root in roots if root in files]
    while stack:
        path = stack.pop()
        if path in seen:
            continue
        seen.add(path)
        if path.endswith(".ts"):
            for spec in IMPORT.findall(files[path]):
                target = resolve(files, path, spec)
                if target:
                    stack.append(target)
    return sorted(seen | {config})


def mutated(files, mutation):
    after = dict(files)
    op, path = mutation["op"], mutation["path"]
    if op in ("write", "rewrite-keep-times", "replace"):
        after[path] = mutation["text"]
    elif op == "append":
        after[path] = after[path] + mutation["text"]
    elif op == "delete":
        after.pop(path, None)
    return after


def verdict(corpus, mutation):
    files, config = corpus["files"], corpus["config"]
    before = program(files, config)
    path, op = mutation["path"], mutation["op"]
    touched = any(item == path or item.startswith(path + "/") for item in before)
    holds = not (op in REMOVING and touched)
    after = program(mutated(files, mutation), config) if op not in ("symlink-file", "symlink-directory", "directory") else before
    return {"id": mutation["id"], "program": before, "holds": holds, "programChanges": after != before}


corpus = json.load(sys.stdin)
json.dump([verdict(corpus, mutation) for mutation in corpus["mutations"]], sys.stdout)
