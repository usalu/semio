import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts",[
('''  await expect(lease(directory,"producer:graph","shared")).resolves.toBeDefined();''','''  (await lease(directory,"producer:graph","shared")).release();'''),
])
