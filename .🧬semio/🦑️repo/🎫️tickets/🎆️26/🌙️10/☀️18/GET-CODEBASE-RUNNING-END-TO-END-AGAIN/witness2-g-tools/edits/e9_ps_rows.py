#!/usr/bin/env python3
"""🔭️ e9: the host probe table names `ps` only by the exact argument vectors the planner child runs (live children, tree CPU time, lease age); a pid placeholder is a closed token, a row without an argument list no longer exists."""
import sys, json
root = sys.argv[1]
W = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/"
def edit(name, pairs):
    text = open(W + name, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (name, text.count(old), old[:70])
        text = text.replace(old, new)
    open(W + name, "w", encoding="utf8").write(text)
edit("🟨️.mjs", [
 ('''anyArguments = new Set(config.hostProbes.filter(row => row.arguments === null).map(row => row.program)), probes = new Set(config.hostProbes.filter(row => row.arguments !== null).map(row => [row.program, ...row.arguments].join("\\0")));''',
  '''probes = config.hostProbes.map(row => [row.program, ...row.arguments]), placeholders = {"{pid}": /^[0-9]{1,10}$/u, "{pids}": /^[0-9]{1,10}(,[0-9]{1,10})*$/u};'''),
 ('''  return anyArguments.has(program) || probes.has([program, ...command.slice(1)].join("\\0"));''',
  '''  const given = [program, ...command.slice(1)];
  return probes.some(row => row.length === given.length && row.every((token, at) => token === given[at] || (placeholders[token] !== undefined && placeholders[token].test(given[at]))));'''),
])
edit("🟨️.d.mts", [('arguments: readonly string[] | null }', 'arguments: readonly string[] }')])
path = W + "🧬️schema/🔣️.json"
text = open(path, encoding="utf8").read()
old = '''{"program": "ps", "arguments": null}'''
assert text.count(old) == 1
new = '''{"program": "ps", "arguments": ["-axo", "pid=,ppid="]},
   {"program": "ps", "arguments": ["-o", "cputime=", "-p", "{pids}"]},
   {"program": "ps", "arguments": ["-o", "etime=", "-p", "{pid}"]}'''
text = text.replace(old, new)
old = '"arguments": { "type": ["array", "null"], "items": { "type": "string" } }'
assert text.count(old) == 1
open(path, "w", encoding="utf8").write(text.replace(old, '"arguments": { "type": "array", "items": { "type": "string" } }'))
