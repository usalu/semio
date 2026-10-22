import sys
sys.path.insert(0,".")
from anchored import patch
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
patch(L+"🏗️build/🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts",[
('''expect(host).toContain("producer.classifyImplementationRowsV1(rows)");expect(host).toContain("graph-generations-v3");''','''expect(host).toContain("producer.classifyImplementationRowsV1(rows)");expect(host).toContain("generationSchemaV1.definitions.Identity.const");expect(readFileSync(join(library,"🏗️build","🕸️graph","🕰️generations","🧬️schema","🔣️.json"),"utf8")).toContain("graph-generations-v3");'''),
])
