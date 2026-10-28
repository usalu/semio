"""keys-g read oracle (macOS Seatbelt): runs a command so that it can read ONLY the declared files of the workspace.
usage: python3 readoracle.py --root R --allow-list FILE [--allow-tree REL]... [--cwd REL] [--env K=V]... [--rounds N] [--timeout S] [--out JSON] -- cmd args...
A read of a workspace file outside the allow list fails with EPERM (kernel enforced, every child process, native code included).
With --rounds > 1 the denied paths of a failing round are added to a LEARNED list and the command runs again, so the report names every
undeclared read the command needs (the missing inputs), not only the first one. Directories stay listable (names are not contents).
Streams the allow list; aborts above 500 MB RSS."""
import argparse, json, os, resource, subprocess, sys, time

p = argparse.ArgumentParser()
p.add_argument("--root", required=True)
p.add_argument("--allow-list", required=True)
p.add_argument("--allow-tree", action="append", default=[])
p.add_argument("--budget", type=int, default=0, help="complement mode: deny the undeclared siblings of declared files down to this many rules (the compiled profile has a size limit); directories still mixed at the budget stay readable")
p.add_argument("--allow-name", action="append", default=[], help="a basename readable anywhere in the workspace (package.json)")
p.add_argument("--cwd", default=".")
p.add_argument("--env", action="append", default=[])
p.add_argument("--rounds", type=int, default=1)
p.add_argument("--timeout", type=int, default=600)
p.add_argument("--out")
p.add_argument("cmd", nargs=argparse.REMAINDER)
a = p.parse_args()
cmd = a.cmd[1:] if a.cmd and a.cmd[0] == "--" else a.cmd
if sys.platform != "darwin":
    sys.exit("readoracle: Seatbelt backend needs macOS / Seatbelt-Backend braucht macOS")
root = os.path.realpath(a.root)
AMBIENT = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"] + a.allow_tree


def q(text):
    return '"' + text.replace("\\", "\\\\").replace('"', '\\"') + '"'


def cover(allowed):
    """Smallest rule set: a directory whose every real file is allowed becomes one subpath, the rest stay literals (the compiled profile is limited to 64 KB)."""
    wanted = set(allowed)
    tree = {}
    for rel in wanted:
        node = tree
        for part in rel.split("/"):
            node = node.setdefault(part, {})
    rules = []

    def full(path, node):
        if not node:
            return True
        try:
            names = os.listdir(os.path.join(root, path))
        except OSError:
            return False
        return set(names) == set(node) and all(full(path + "/" + name, child) for name, child in node.items())

    def walk(path, node):
        if path and node and full(path, node):
            rules.append(("subpath", path))
            return
        for name, child in sorted(node.items()):
            walk(path + "/" + name if path else name, child) if child else rules.append(("literal", path + "/" + name if path else name))
    walk("", tree)
    return rules


def complement(allowed):
    """Deny rules for everything that is not declared, as fine-grained as the rule budget allows (breadth first over the declared trie)."""
    tree = {}
    for rel in allowed:
        node = tree
        for part in rel.split("/"):
            node = node.setdefault(part, {})
    denies, queue, expanded = [], [("", tree)], 0
    while queue:
        path, node = queue.pop(0)
        try:
            names = sorted(os.listdir(os.path.join(root, path))) if path else sorted(os.listdir(root))
        except OSError:
            continue
        fresh = [n for n in names if n not in node and not (path == "" and any(n == a.split("/")[0] for a in AMBIENT))]
        if len(denies) + len(fresh) > a.budget and expanded > 0:
            break
        expanded += 1
        for n in fresh:
            rel = path + "/" + n if path else n
            if rel in AMBIENT or any(rel == tree_ or rel.startswith(tree_ + "/") for tree_ in AMBIENT):
                continue
            kind = "subpath" if os.path.isdir(os.path.join(root, rel)) and not os.path.islink(os.path.join(root, rel)) else "literal"
            denies.append((kind, rel))
        for n, child in sorted(node.items()):
            if child:
                queue.append((path + "/" + n if path else n, child))
    return denies


def profile(allowed):
    import re
    if a.budget:
        lines = ["(version 1)", "(allow default)"]
        lines += [f"(deny file-read-data ({kind} {q(os.path.join(root, rel))}))" for kind, rel in complement(allowed)]
        lines += [f"(allow file-read-data (regex #\"^{re.escape(root)}/(.*/)?{re.escape(name)}$\"))" for name in a.allow_name]
        lines += ["(allow file-read-data (vnode-type DIRECTORY))"]
        return "\n".join(lines) + "\n"
    lines = ["(version 1)", "(allow default)", f"(deny file-read-data (subpath {q(root)}))", "(allow file-read-data (vnode-type DIRECTORY))"]
    lines += [f"(allow file-read-data (subpath {q(os.path.join(root, tree))}))" for tree in AMBIENT]
    lines += [f"(allow file-read-data (regex #\"^{re.escape(root)}/(.*/)?{re.escape(name)}$\"))" for name in a.allow_name]
    lines += [f"(allow file-read-data ({kind} {q(os.path.join(root, rel))}))" for kind, rel in cover([x for x in allowed if os.path.basename(x) not in a.allow_name])]
    return "\n".join(lines) + "\n"


def denials(since):
    out = subprocess.run(["/usr/bin/log", "show", "--start", since, "--style", "compact", "--predicate", 'eventMessage CONTAINS "deny(1) file-read-data" AND eventMessage CONTAINS ' + q(root)], capture_output=True, text=True).stdout
    seen = set()
    for line in out.splitlines():
        at = line.find("deny(1) file-read-data ")
        if at < 0:
            continue
        path = line[at + len("deny(1) file-read-data "):].strip()
        if path.startswith(root + "/") and not any(path.startswith(os.path.join(root, tree) + "/") for tree in AMBIENT):
            seen.add(path[len(root) + 1:])
    return sorted(seen)


allowed = []
with open(a.allow_list, encoding="utf-8") as handle:
    for line in handle:
        line = line.rstrip("\n")
        if line:
            allowed.append(line)
learned, report = [], []
env = dict(os.environ)
for pair in a.env:
    key, _, value = pair.partition("=")
    env[key] = value
for round_ in range(max(1, a.rounds)):
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1048576 > 500:
        sys.exit("readoracle: RSS above 500 MB")
    profile_path = os.path.abspath(f"{a.out or '/tmp/readoracle'}.round{round_}.sb")
    with open(profile_path, "w", encoding="utf-8") as handle:
        handle.write(profile(allowed + learned))
    since = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(time.time() - 2))
    started = time.time()
    try:
        done = subprocess.run(["sandbox-exec", "-f", profile_path, *cmd], cwd=os.path.join(root, a.cwd), env=env, capture_output=True, text=True, timeout=a.timeout)
        code, tail = done.returncode, (done.stdout + done.stderr)[-1500:]
    except subprocess.TimeoutExpired:
        code, tail = 124, "timeout"
    time.sleep(2.5)
    missing = [path for path in denials(since) if path not in learned and path not in allowed]
    report.append({"round": round_, "exit": code, "seconds": round(time.time() - started, 1), "denied": missing, "tail": tail})
    if a.out:
        with open(a.out, "w", encoding="utf-8") as handle:
            json.dump({"command": cmd, "allowed": len(allowed), "learned": learned + missing, "rounds": report, "passed": code == 0, "final": code == 0 or not missing}, handle, ensure_ascii=False, indent=1)
    if code == 0 or not missing:
        break
    learned += missing
result = {"command": cmd, "allowed": len(allowed), "learned": learned, "rounds": report, "passed": report[-1]["exit"] == 0}
text = json.dumps(result, ensure_ascii=False, indent=1)
if a.out:
    with open(a.out, "w", encoding="utf-8") as handle:
        handle.write(text)
print(text[:6000])
