#!/usr/bin/env python3
"""🔎 [optcache] lists the test files that reach the checkout's own repository store, caches or leases: a store API (`withRepository*`, `openRepositoryCachePartitionV1`, `repositoryLeaseDirectoryV1`, `repositoryCacheBudgetV1`, `withRepositoryCacheStoreV1`, `acquireQueuedResourceLease`) used in a file that names the checkout as the workspace (`process.cwd()`, `SEMIO_WORKSPACE_ROOT`, a literal `.cache/b`). Streams `git grep` output, prints `count<TAB>file` sorted, a per-module summary on stderr. usage: audit.py [ROOT]  (default: the checkout holding this ticket)"""
import os, re, subprocess, sys, collections
root = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
api = re.compile(r"withRepositoryBuildGraph|withRepositoryCacheStoreV1|openRepositoryCachePartitionV1|repositoryLeaseDirectoryV1|repositoryCacheBudgetV1|acquireQueuedResourceLease|withRepositoryBuildInferenceDatasetV1|openNativePlatformStorageV1")
live = re.compile(r"process\.cwd\(\)|SEMIO_WORKSPACE_ROOT|\.cache/b(?:/|\b)|⚡️cache")
files = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", "🧰️framework"], capture_output=True, check=True).stdout.decode().split("\0")
rows = []
for rel in files:
    if not rel.endswith((".ts", ".mjs", ".js")) or not re.search(r"/🧪️tests?/|/🧪️testing/", rel): continue
    try: text = open(os.path.join(root, rel), encoding="utf-8").read()
    except (OSError, UnicodeDecodeError): continue
    if not api.search(text): continue
    n = len(live.findall(text))
    if n: rows.append((n, rel))
for n, rel in sorted(rows, reverse=True): print("%d\t%s" % (n, rel))
by = collections.Counter(rel.split("/🧪️test")[0].split("🦑️repo/🔨️modules/")[-1] for _, rel in rows)
sys.stderr.write("%d files, %d modules\n" % (len(rows), len(by)))
for k, v in by.most_common(): sys.stderr.write("%4d %s\n" % (v, k))
