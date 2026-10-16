"""🔁 Duplicate-mount scan: every `#[path = "..."]` of every tracked or untracked .rs file (git grep, repo-wide) resolved to an absolute file; reports the mount count of each target below the given prefixes and of every file the given roots mount.
usage: dupmounts.py <repo-rel-prefix>... """
import collections, os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
prefixes = sys.argv[1:]
rx = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
cnt = collections.defaultdict(list)
for flag in (["--untracked"],):
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-I", "-n", "-F", *flag, "path", "--", "*.rs"], cwd=ROOT, capture_output=True, text=True).stdout
    for line in out.split("\n"):
        if not line or "#[" not in line:
            continue
        f, _, rest = line.partition(":")
        _, _, text = rest.partition(":")
        for m in rx.finditer(text):
            tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            cnt[tgt].append(f)
seen = set()
bad = 0
for tgt, srcs in sorted(cnt.items()):
    if not any(tgt.startswith(p) for p in prefixes):
        continue
    srcs = sorted(set(srcs)) if False else srcs
    uniq = {}
    for s in srcs:
        uniq[s] = uniq.get(s, 0) + 1
    n = sum(uniq.values())
    if n > 1:
        bad += 1
        print("DUPLICATE", n, tgt[-90:], [s[-70:] for s in uniq])
    if not os.path.exists(os.path.join(ROOT, tgt)):
        print("MISSING TARGET", tgt[-100:], "from", [s[-60:] for s in uniq])
print(f"targets below {prefixes}: {sum(1 for t in cnt if any(t.startswith(p) for p in prefixes))}; with more than one mount: {bad}")
