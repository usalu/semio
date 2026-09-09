# Netz Fixture Relocation

Seven frozen PDF/TeX/JSON examples moved byte-for-byte from the regression case to their shared netz owner. Data/model/stage consumers now resolve that owner. The snapshot generator was executable oracle support, so its model-to-JSON function was integrated into the existing canonical Python case rather than stored with examples; no generator wrapper remains in fixtures. The historical snapshot is not rewritten. With the configured repository .venv interpreter, the snapshot existence/count checks, all data parity checks and ten model snapshot comparisons passed. The separate legacy connectivity oracle failed because its existing generator still reads E:/recherche/_neo4j/netz/overlay.json. The moved snapshot was loaded successfully; a complete stage suite is not claimed.

## Preserved Moves

```json
[
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_tables2.tex",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_tables2.tex",
    "sha256": "a7932138ec8dd39ee2bd5d16bcda276d0202e5cfafd3136d217f457c95dfe0ed"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_abb_stage3_reference.tex",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_abb_stage3_reference.tex",
    "sha256": "ec1bceb38892d2c4f411e52712efcf88e7eae75c1d3963eed94b85b954b6452b"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/model_snapshot.json",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/model_snapshot.json",
    "sha256": "38447da2e107e95c0e82a5f4d91e138fa9e00a876c7a5e713a9def773d7975fa"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_abb.pdf.prev_1.75mm",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_abb.pdf.prev_1.75mm",
    "sha256": "2c944103e52511367fc40b3497f06092c60a81a70566d75e2ac53da7343ad397"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_abb.pdf",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_abb.pdf",
    "sha256": "d6edfb12dab81998c3068fdf80de1bc8b735d13b7e9e02d6d6e65be1492ec849"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_tab.pdf",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_tab.pdf",
    "sha256": "6527da4697d042ae8b3fdf2cff274547b516a27b5daa2620060c86702e8699d4"
  },
  {
    "old": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_abb.tex",
    "new": "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_abb.tex",
    "sha256": "ec1bceb38892d2c4f411e52712efcf88e7eae75c1d3963eed94b85b954b6452b"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️netz-fixture-relocation-2026-09-09.md",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/data/🧪️tests/🔬️stage-one-parity/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/model/🧪️tests/🔬️stage-two-parity/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🐍️.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/dump_snapshot.py",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_abb.pdf",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_abb.pdf.prev_1.75mm",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/baseline_tab.pdf",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_abb.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_abb_stage3_reference.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/frag_tables2.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🧫️fixtures/golden/model_snapshot.json",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_abb.pdf",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_abb.pdf.prev_1.75mm",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/baseline_tab.pdf",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_abb.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_abb_stage3_reference.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/frag_tables2.tex",
  "♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧫️fixtures/🪜️stage-regression/golden/model_snapshot.json"
]
```

## Portable Independent Reference Run

The remaining failure came from importing the legacy figure generator, which used a hard-coded Windows drive and writes a LaTeX output during import. The canonical model parity case now constructs the independent reference model directly from the same explicit input files, temporarily binding its input locator with the standard unittest mock context. It does not import the output generator. The rerun passed all 26 snapshot/data/model/connectivity checks, including all 11 countries; the process exited zero. Production generator sources were not changed. Full pixel/PDF generation stages were not run.
