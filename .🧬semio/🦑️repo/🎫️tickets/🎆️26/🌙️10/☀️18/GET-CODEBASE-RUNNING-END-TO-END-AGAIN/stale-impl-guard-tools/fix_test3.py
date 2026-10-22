import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts",[
('''expect(lines[0],row.name).toBe(expected[index]!.en);expect(lines[1],row.name).toBe(expected[index]!.de);''','''expect(lines[0],row.name).toBe(expected[index]!.en!);expect(lines[1],row.name).toBe(expected[index]!.de!);'''),
])
