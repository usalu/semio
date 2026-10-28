import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

b = Batch("storelimits", "the record store reads its orphan age, pending flush and descriptor limits from its schema (no cache key carries this file)")
K = LIB + "⚡️caching/🗃️records/"
b.add_defs(K + "🧬️schema/🔣️.json", [Def("Limits", "The ages, budgets and descriptor limit of the record store.", "Die Alter, Budgets und die Deskriptorgrenze des Record-Stores.", [
    ("orphanMilliseconds", 3600000, "How old an unreferenced pack or temporary file must be before the store may remove it (1 hour).", "Wie alt ein unreferenziertes Pack oder eine temporaere Datei sein muss, bevor der Store sie entfernen darf (1 Stunde)."),
    ("pendingFlushBytes", 67108864, "The pending bytes at which the store writes a pack (64 MiB).", "Die ausstehenden Bytes, ab denen der Store ein Pack schreibt (64 MiB)."),
    ("descriptorLimit", 32, "How many read descriptors packs keep open across every store of the process.", "Wie viele Lesedeskriptoren Packs ueber alle Stores des Prozesses offen halten."),
])])
b.subs(K + "🟦️.ts", [
    ('import { dirname, join, relative, resolve } from "node:path";\n', 'import { dirname, join, relative, resolve } from "node:path";\nimport schema from "./🧬️schema/🔣️.json" with {type: "json"};\n'),
    ("const ORPHAN_MS = 60 * 60 * 1000;", "const ORPHAN_MS = schema.definitions.Limits.const.orphanMilliseconds;"),
    ("const PENDING_FLUSH_BYTES = 64 * 1024 * 1024;", "const PENDING_FLUSH_BYTES = schema.definitions.Limits.const.pendingFlushBytes;"),
    ("export const RECORD_STORE_DESCRIPTOR_LIMIT = 32;", "export const RECORD_STORE_DESCRIPTOR_LIMIT = schema.definitions.Limits.const.descriptorLimit;"),
])
rel = lambda p: "⚡️caching/🗃️records/" + p
b.row("records", "library", rel("🧬️schema/🔣️.json"), ["Limits"], [rel("🟦️.ts")], minimum_number=30, allow=[(rel("🟦️.ts"), 32, "The number of hex digits of the value digest kept in the index (the schema pattern of the digest), not the descriptor limit.")])
b.write(tests=["./" + K + "🧪️tests", "./" + LAW + "🟦️.ts"])
