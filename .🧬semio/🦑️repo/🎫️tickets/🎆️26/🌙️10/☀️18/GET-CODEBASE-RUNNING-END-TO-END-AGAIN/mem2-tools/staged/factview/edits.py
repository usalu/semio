DOC = """🍂 Staged batch `factview` (lane mem2): a generation build releases the verification view of the fact index (every row its plan verified stays materialized in it: 548,588 rows of generation 669 = 292 MB live on top of the 241 MB the open view costs) before it computes, so a graph child infers without carrying it."""
G = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/"
EDITS = [
    (G + "🟨️.mjs", [
        (" /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. */",
         " /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. The verification view of the fact index (the rows the plan read stay materialized in it) is released before `compute` runs, so the inference never carries it; a later plan in this process opens it again. */"),
        ("  const result=await compute(spec,signal);", "  if(usable)indexCache.delete(\"view:\"+usable.factIndex);\n  const result=await compute(spec,signal);"),
    ]),
]
NEW = {G + "🧪️tests/🍂️view-release/🟦️.ts": "view-release.ts"}
