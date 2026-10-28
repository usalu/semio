#!/usr/bin/env python3
"""[gram-g] writes a variant of 🕰️generations/🟨️.mjs (absolute imports, outside the source tree) with the staged patch rule: refine only when the partition outgrew its bound AND grew by a quarter since the base. usage: make-variant.py <out.mjs>"""
import re, sys, os
ROOT = "/Users/ueli/Documents/semio"
G = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
text = open(G + "/🟨️.mjs", encoding="utf8").read()
text = re.sub(r'(from\s*|import\()\s*(["\'])(\.[^"\']*)\2', lambda m: m.group(1) + '"' + os.path.normpath(os.path.join(G, m.group(3))) + '"', text)
old = "return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor?refineFactIndexV1(result):result;"
new = "return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length?refineFactIndexV1(result):result;"
assert text.count(old) == 1
text = text.replace(old, new)
a = " compactionFactor:2,compactionFloor:1024,"
assert text.count(a) == 1
text = text.replace(a, " compactionFactor:2,compactionFloor:1024,compactionGrowth:1.25,")
open(sys.argv[1], "w", encoding="utf8").write(text)
