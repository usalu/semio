# Native Layout Follow-up Review

The stage-one and stage-two helpers contain actual parity assertions through an injected `check_fn`, so they are executable test implementations. They now live in the data and model domains at named canonical cases; the stage-regression implementation loads those explicit files. Golden snapshot data remains shared in the integration case fixture directory. All original comparisons and check calls are preserved.

Corrected package/source-root calculations for standalone case imports and for the snapshot producer. The producer was not executed, so input golden data was not overwritten. Public Bun/Nx syntax/import and shared-fixture resolution proof is in progress.

```json
[
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/check_stage1.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/check_stage2.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/data/🧪️tests/🔬️stage-one-parity/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/model/🧪️tests/🔬️stage-two-parity/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/dump_snapshot.py"
]
```
