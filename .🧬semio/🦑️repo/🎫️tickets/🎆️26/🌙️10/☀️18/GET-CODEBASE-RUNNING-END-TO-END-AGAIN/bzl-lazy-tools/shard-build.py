#!/usr/bin/env python3
# Experiment: moves the source filegroups of a generated BUILD into N hash-sharded packages //g00..//gNN so Bazel loads them in parallel and reloads only the shard that changed.
import hashlib, os, re, shutil, sys
src, dst, shards = sys.argv[1], sys.argv[2], int(sys.argv[3])
if os.path.exists(dst): shutil.rmtree(dst)
shutil.copytree(src, dst, symlinks=True)
text = open(dst + "/BUILD.bazel", encoding="utf-8").read()
blocks = text.split("\n\n")
shard = lambda name: int(hashlib.sha256(name.encode()).hexdigest()[:8], 16) % shards
ref = re.compile(r'"(:)((?:d|k|i)_[0-9a-f]+)"')
groups = {}; root = []
for b in blocks:
    m = re.match(r'filegroup\(\s*name = "([^"]+)"', b)
    if m: groups.setdefault(shard(m.group(1)), []).append(b)
    else: root.append(b)
fix_root = lambda s: ref.sub(lambda m: '"//g%02d:%s"' % (shard(m.group(2)), m.group(2)), s)
def fix_shard(s, k):
    s = re.sub(r'"s/', '"//:s/', s)
    return ref.sub(lambda m: '"%s%s"' % (":" if shard(m.group(2)) == k else "//g%02d:" % shard(m.group(2)), m.group(2)), s)
exported = set()
for k, bl in groups.items():
    for s in fix_shard("\n\n".join(bl), k).split('"'):
        if s.startswith("//:s/"): exported.add(s[3:])
body = fix_root("\n\n".join(root))
exports = "exports_files([\n" + "".join('    "%s",\n' % e for e in sorted(exported)) + "], visibility = [\"//visibility:public\"])\n\n"
open(dst + "/BUILD.bazel", "w", encoding="utf-8").write(body.replace("\n\n", "\n\n" + exports, 1))
for k, bl in groups.items():
    os.makedirs(dst + "/g%02d" % k, exist_ok=True)
    open(dst + "/g%02d/BUILD.bazel" % k, "w", encoding="utf-8").write('package(default_visibility = ["//visibility:public"])\n\n' + fix_shard("\n\n".join(bl), k) + "\n")
print(len(blocks), {k: len(v) for k, v in sorted(groups.items())})
