#!/usr/bin/env python3
"""F123 (land after 19:00): the transaction-v2 launcher stops snapshotting the live taxonomy (the fixtures pin their own), its identity record names the pinned snapshot, and its static title scan follows the suite's local case modules. Atomic writes; take landing.lock.transactions around the run."""
import os
LIB = "/Users/ueli/Documents/semio/" + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(rel, pairs):
    p = LIB + rel; s = open(p, encoding="utf8").read()
    for a, b in pairs:
        assert s.count(a) == 1, (rel, a[:60]); s = s.replace(a, b)
    open(p + ".tmp123", "w", encoding="utf8").write(s); os.replace(p + ".tmp123", p)
V = "🔄️transactions/🧪️verification/"
edit(V + "🧾️provenance/🟦️.ts", [('taxonomy: join(repoRoot, library, "🔣️taxonomy.json"),', 'taxonomy: join(repoRoot, library, "🧪️tests/🧫️fixtures/🌳️transaction-taxonomy-snapshot/🔣️.json"),')])
edit(V + "🏃️shard-execution/🟦️.ts", [("  schemaSnapshot: string;\n", ""), ("        SEMIO_TRANSACTION_V2_SCHEMA: options.schemaSnapshot,\n", "")])
edit(V + "📋️orchestration/🟦️.ts", [
 ('  const schemaSnapshot = join(bundleRoot, "🔣️.json");\n  copyFileSync(identityPaths.taxonomy!, schemaSnapshot);\n', ''),
 ('    schemaSnapshot,\n', ''),
 ('  const staticTitles = [...readFileSync(identityPaths.suite!, "utf8").matchAll(/\\btest(?:\\.concurrent)?\\("([^"]+)"/gu)].map((match) => match[1]!);\n',
  '  const suiteSources = [identityPaths.suite!, ...[...readFileSync(identityPaths.suite!, "utf8").matchAll(/^import\\b[^"\\n]*"(\\.\\/[^"]+)"/gmu)].map((match) => join(dirname(identityPaths.suite!), match[1]!))];\n  const staticTitles = suiteSources.flatMap((source) => [...readFileSync(source, "utf8").matchAll(/\\btest(?:\\.concurrent)?\\("([^"]+)"/gu)].map((match) => match[1]!));\n'),
 ('import { copyFileSync, mkdirSync, readFileSync } from "node:fs";', 'import { mkdirSync, readFileSync } from "node:fs";'),
])
