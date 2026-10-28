DOC = """📖 Staged batch `texts` (lane td-misc): the process suite runner refuses an unknown suite with its catalogued EN+DE message `script.suite-unknown` instead of an uncatalogued English `Error`, so the process texts law (every catalogued key is used by its module) holds again."""
P = "🧰️framework/🔨️modules/🏃️process/"
EDITS = [
 (P + "🧪️testing/🗂️suites/🏃️run/🟦️.ts", [
  ('if(!suite)throw new Error(`Unknown test suite: ${run.suite}`);', 'if(!suite)throw refuseProcessV1("invalid","script.suite-unknown",{suite:run.suite,known:table.suites.map(row=>row.id).join(", ")});'),
 ]),
]
