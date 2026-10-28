"""Idempotently adds `tools: ["bazel"]` to the build suites that hold Bazel-gated laws (lands AFTER route2-g's staged table, gate batch 6). usage: python3 apply-build-tools.py [--check]"""
import json, sys
R = '/Users/ueli/Documents/semio/'
P = R + '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json'
BAZEL = ['build-remote-cache', 'build-remote-execution', 'build-incremental-concurrent', 'build-incremental', 'build-doctor-preflight', 'build-bazel-execution', 'build-bazel-evidence', 'build-bazel-host', 'build-bazel-snapshots', 'build-bazel-contracts', 'build-bazel-sessions']
raw = open(P, encoding='utf-8').read()
doc = json.loads(raw)
assert json.dumps(doc, ensure_ascii=False, indent=2) + '\n' == raw, 'table is not in canonical form'
ids = {s['id']: s for s in doc['suites']}
missing = [i for i in BAZEL if i not in ids]
assert not missing, missing
todo = [i for i in BAZEL if ids[i].get('tools') != ['bazel']]
print('to change:', todo)
if '--check' in sys.argv: sys.exit(1 if todo else 0)
for i in todo: ids[i]['tools'] = ['bazel']
open(P, 'w', encoding='utf-8').write(json.dumps(doc, ensure_ascii=False, indent=2) + '\n')
