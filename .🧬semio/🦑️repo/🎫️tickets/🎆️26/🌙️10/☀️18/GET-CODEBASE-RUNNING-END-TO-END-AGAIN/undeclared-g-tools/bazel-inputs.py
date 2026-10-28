"""undeclared-g bazel-inputs: the declared source files (srcs) of one generated Bazel task, as real workspace paths.
usage: python3 bazel-inputs.py <bazel workspace dir> <task name t_xxx|label substring>  -> prints one path per line (streams, RSS small)
The workspace holds BUILD.bazel (repo_task rows with srcs groups i_*, filegroups a_* of 'a/<dirhash>/<name>') and sources.json (dirhash -> real directory)."""
import json, os, re, sys
ws, wanted = sys.argv[1], sys.argv[2]
txt = open(os.path.join(ws, "BUILD.bazel"), encoding="utf8").read()
dirs = json.load(open(os.path.join(ws, "sources.json")))["directories"]
groups = {}
for m in re.finditer(r'(?:filegroup|repo_inputs_witness)\(\s*name = "([^"]+)",\s*srcs = \[(.*?)\]', txt, re.S):
    groups[m.group(1)] = re.findall(r'"([^"]+)"', m.group(2))
task = None
for m in re.finditer(r'repo_(?:task|test|launch)\(\s*name = "([^"]+)",(.*?)\n\)', txt, re.S):
    if m.group(1) == wanted or wanted in m.group(2).split("identity = ", 1)[-1].split(",", 1)[0]:
        task = m; break
if task is None: sys.exit("task not found: " + wanted)
srcs = re.search(r'\n    srcs = \[(.*?)\]', task.group(2), re.S)
roots = re.findall(r'"([^"]+)"', srcs.group(1)) if srcs else []
seen, out = set(), set()
def walk(items):
    for s in items:
        if s.startswith(":"):
            n = s[1:]
            if n not in seen:
                seen.add(n); walk(groups.get(n, []))
        elif s.startswith("a/"):
            _, h, name = s.split("/", 2)
            out.add(dirs[h] + "/" + name)
        else:
            out.add(s)
walk(roots)
for p in sorted(out): print(p)
