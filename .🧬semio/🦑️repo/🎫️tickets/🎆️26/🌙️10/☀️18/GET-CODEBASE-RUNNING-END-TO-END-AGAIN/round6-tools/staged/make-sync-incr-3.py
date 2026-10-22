#!/usr/bin/env python3
"""Copy of sync-incr staged3 with the `suite target` anchor re-based on the single-line target rows peers wrote into the caching project manifest."""
import os, sys, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/sync-incr-tools/staged3", "sync-incr-3")
P = T + "/apply.py"
text = open(P, encoding="utf8").read()
start = text.index('    target = [Edit("suite target"')
end = text.index("    return [(STORAGE, storage)", start)
new = '''    row = lambda name: '    "test-javascript-' + name + '": {"executor":"repo:script","cache":false,"outputs":[],"dependsOn":[],"options":{"cwd":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching","command":"bun ./📜️script.ts test javascript-' + name + '","forwardAllArgs":false},"metadata":{"semio":{"execution":{"version":1,"kind":"source"},"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}},\\n'
    target = [Edit("suite target", '    "test-javascript-incremental-sync": {"executor"', row("adopt") + '    "test-javascript-incremental-sync": {"executor"', marker='"test-javascript-adopt"')]
'''
open(P, "w", encoding="utf8").write(text[:start] + new + text[end:])
print("sync-incr-3 copy:", T)
