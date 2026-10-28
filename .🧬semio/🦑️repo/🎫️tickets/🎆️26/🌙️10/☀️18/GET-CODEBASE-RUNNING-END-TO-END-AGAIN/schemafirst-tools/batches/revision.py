import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

R = BUILD + "📥️inputs/🔏️revision/"
b = Batch("revision", "the source revision capture reads its byte, path, ancestry and settle ceilings from its schema and exports them, so the observation, argument and graph captures read their bounds from the same place")
b.add_defs(R + "🧬️schema/🔣️.json", [Def("Policy", "The ceilings of the source revision capture: the single source of every bounded read of a source or of the schema beside it.", "Die Obergrenzen der Quellrevisions-Erfassung: die einzige Quelle jedes begrenzten Lesens einer Quelle oder des Schemas daneben.", [
    ("maximumSourceBytes", 8388608, "The most bytes of one source file a revision captures (8 MiB).", "Die meisten Bytes einer Quelldatei, die eine Revision erfasst (8 MiB)."),
    ("maximumTextBytes", 65536, "The most bytes of a source or schema whose text is retained (64 KiB): schemas and policy files are read under this ceiling.", "Die meisten Bytes einer Quelle oder eines Schemas, deren Text behalten wird (64 KiB): Schemas und Richtliniendateien werden unter dieser Obergrenze gelesen."),
    ("maximumChunkBytes", 65536, "The size of one read while a file is hashed (64 KiB).", "Die Groesse eines Lesevorgangs beim Hashen einer Datei (64 KiB)."),
    ("maximumPathUnits", 256, "The most UTF-16 units of an absolute source path.", "Die meisten UTF-16-Einheiten eines absoluten Quellpfads."),
    ("maximumAncestors", 64, "The most directory levels between the base and a captured source.", "Die meisten Verzeichnisebenen zwischen Basis und erfasster Quelle."),
    ("settleMilliseconds", 2000, "How long a source must be unchanged before its revision is remembered by file identity.", "Wie lange eine Quelle unveraendert sein muss, bevor ihre Revision unter der Dateiidentitaet gemerkt wird."),
    ("maximumSettledRevisions", 4096, "The most settled revisions remembered at once.", "Die meisten gleichzeitig gemerkten beruhigten Revisionen."),
])])
M = R + "🟨️.mjs"
b.subs(M, [
    ("const maximumSourceBytes=8388608,maximumTextBytes=65536,maximumChunkBytes=65536,maximumPathUnits=256,maximumAncestors=64;\n",
     "/** ‹E› The ceilings of the source revision capture; the schema file is their single source. */\nexport const REVISION_POLICY_V1=Object.freeze({...schemaContract.definitions.Policy.const});\nconst{maximumSourceBytes,maximumTextBytes,maximumChunkBytes,maximumPathUnits,maximumAncestors,settleMilliseconds,maximumSettledRevisions}=REVISION_POLICY_V1;\n"),
    ("const settledRevisions=new Map(),settleMilliseconds=2000,maximumSettledRevisions=4096;", "const settledRevisions=new Map();"),
])
b.sub(R + "🟨️.d.mts", "/** 🧾️ Copies a bounded ordinary Source observation", "/** ‹E› The declared ceilings of the source revision capture: the schema file beside the module is their single source. */\nexport const REVISION_POLICY_V1:Readonly<{maximumSourceBytes:number;maximumTextBytes:number;maximumChunkBytes:number;maximumPathUnits:number;maximumAncestors:number;settleMilliseconds:number;maximumSettledRevisions:number}>;\n/** 🧾️ Copies a bounded ordinary Source observation")
b.row("revision", "library", "🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json", ["Policy"], ["🏗️build/📥️inputs/🔏️revision/🟨️.mjs"], exports=[("🏗️build/📥️inputs/🔏️revision/🟨️.mjs", "REVISION_POLICY_V1", "Policy")])
b.write(tests=["./" + R + "🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
