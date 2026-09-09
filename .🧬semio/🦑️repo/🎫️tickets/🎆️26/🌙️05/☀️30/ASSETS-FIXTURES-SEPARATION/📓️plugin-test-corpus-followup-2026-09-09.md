# Plugin Test Corpus Ownership Follow-up

Eleven JSON test corpora now live under their semantic owner's fixtures folder. Seven publication/interactive-job/action-cohort examples use their plugin; four retained-job examples use their artifact subset shared by the editor and retained-command tests. Every move preserves bytes. Raster's retained publication example has no active reader in the bounded audit; its content and source references identify it as test evidence. Note's action-cohort example is consumed by its canonical case and Flow's cross-plugin cohort audit; those readers now use its fixtures path. The three extra Puzzle retained-job examples are included only by canonical tests or cfg(all(test, feature = component-app-assembly)) functions.

## Preserved Moves

```json
[
  {
    "old": "✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json",
    "new": "✏️s/🔌️plugins/💠️lowpoly/🧫️fixtures/🧪️interactive-job/🔣️.json",
    "sha256": "ea5e233e8456710af7fb0c8a4d2a196e705d75c469c6290c6706e6e47354e638"
  },
  {
    "old": "✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json",
    "new": "✏️s/🔌️plugins/🧱️block/🧫️fixtures/🧪️publication-authority/🔣️.json",
    "sha256": "03fc8a107ffc0cda6d0eef08b69e44c1baaf9b59c6e4d7d19e1b3555b3a78733"
  },
  {
    "old": "✏️s/🔌️plugins/🖍️draw/🧪️publication-authority/🔣️.json",
    "new": "✏️s/🔌️plugins/🖍️draw/🧫️fixtures/🧪️publication-authority/🔣️.json",
    "sha256": "d720744f9f7321745c3122b19622a4f67c6041f8735e05175e9b06345667a2a2"
  },
  {
    "old": "✏️s/🔌️plugins/➗️mathematical/📣️publication-authority/🔣️.json",
    "new": "✏️s/🔌️plugins/➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json",
    "sha256": "2e4c8c75b62af3a65fb489be39479146ac95fbc29da07e64bb4ccdd8f494255a"
  },
  {
    "old": "✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json",
    "sha256": "429454033fe4cdcdb7c00ce3f36050409b171611e67d4244ebdde1466c292552"
  },
  {
    "old": "✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/🔣️.json",
    "new": "✏️s/🔌️plugins/🖨️raster/🧫️fixtures/🔏️publication-authority/🔣️.json",
    "sha256": "c1bcb7305a5e1f7dd6062635bd7305621a1adde6139bff6a8ab09aca2177a967"
  },
  {
    "old": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
    "new": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
    "sha256": "a78ea1e7d920b1d216d4c99b74d46c56a75ea82c3c66dcafc13bd56c01f8c208"
  },
  {
    "old": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
    "sha256": "8ed5dc3985777bb35794ea8ffab684f68f77fca1c733ef5fea1f552c2c3bf85c"
  },
  {
    "old": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
    "sha256": "7cba216e3ecd5572ffda9de90f7a84d02e2c3cebfcda8f667838caeca2e08fdc"
  },
  {
    "old": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
    "new": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
    "sha256": "00c59969f35879843e8813ff46aae4ed72054e894dc38b8aba172d4175737417"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️plugin-corpora/📜️script.ts",
  "✏️s/🔌️plugins/➗️mathematical/📣️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🔣️.json",
  "✏️s/🔌️plugins/💠️lowpoly/🧫️fixtures/🧪️interactive-job/🔣️.json",
  "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🖍️draw/🧪️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🖍️draw/🧫️fixtures/🧪️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🖨️raster/🧫️fixtures/🔏️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🗄️retained-jobs/🔣️.json",
  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🦀️.rs",
  "✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🧱️block/🧪️publication-authority/🔣️.json",
  "✏️s/🔌️plugins/🧱️block/🧫️fixtures/🧪️publication-authority/🔣️.json"
]
```

## Additional Preserved Move

```json
[
  {
    "old": "✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️.json",
    "new": "✏️s/🔌️plugins/🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json",
    "sha256": "cccffa81839b45970c12bc60c8488f3cd68feb5d10cc73ec83df51db74977ae0"
  }
]
```

## Additional Authored Paths

```json
[
  "✏️s/🔌️plugins/🗒️note/🧪️action-cohort/🔣️.json",
  "✏️s/🔌️plugins/🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
  "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
]
```

## Consumer Follow-up

The actual Block package command exposed stale template-string reads in its two canonical I/O cases. The fixture URLs now reach the I/O owner's fixtures, while the DSL URLs reach the subset's example assets. Both actual cases passed: eight tests and eight assertions, including fixed points and DSL-to-JSON parity with the retained Rust output bytes.

The taxonomy no longer registers the old Puzzle/plugin publication-authority scopes or the subset retained-job folders as semantic source members. Their example data is opaque under canonical fixtures.

The retained runtime probe ran the six real TypeScript package test/audit commands through Bun/Nx. Puzzle's publication authority audit passed; Raster's retained fixture independently passed its current strict Ajv schema and owner-source check. The other commands read the current fixture locations but did not pass their broader existing contracts:

| Command | Observed limitation after relocation |
| --- | --- |
| Lowpoly test | Source no longer registers exactly the fixture's 47 classified actions. Its two example tests passed first. |
| Block publication-authority-audit | After the path fixes, its example and I/O tests passed; its publication source oracle rejected the current production source. |
| Draw publication-authority-audit | Its example tests and fixture schema passed; its publication source oracle rejected the current production source. |
| Mathematical publication-authority-audit | The preserved fixture has fewer routes than its current schema's minimum of seven. |
| CAD retained-audit | Strict Ajv compilation rejects the existing unregistered x-semio-formats annotation. The fixture was read before this failure. |

These command failures are not suppressed or described as passing. No fixture payload, schema requirement, or production behavior was weakened to hide source-contract drift. The move ledger proves the corpus bytes are unchanged.

## Additional Audited Corpora

Flow action-cohort, CAD presence retirement, Writer migration, and Sourcing expected-stock JSONs are testing-only examples. Their direct readers now use canonical fixtures. Flow's action-cohort schema remains a declaration, and its cross-plugin Note readers use Note's relocated fixture.

### Preserved Moves

```json
[
  {
    "old": "✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🔣️.json",
    "new": "✏️s/🔌️plugins/🌊️flow/🧫️fixtures/🎬️action-cohort/🔣️.json",
    "sha256": "95926b1a528a12dab70085daafa29fce9cefe7c5f71ca7fb6eb8fb4c5e2ac14c"
  },
  {
    "old": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️retirement.json",
    "new": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧫️fixtures/♻️retirement/🔣️.json",
    "sha256": "3866915001931d0741fd6b6eedd5eaa8767d9dac442557243aba3c6dddb2249e"
  },
  {
    "old": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🔣️.json",
    "new": "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🎬️writer-migration/🔣️.json",
    "sha256": "ddc7c024af60a5e88ac4c37f664aeae4eb446c7161f160d83cf1d348d3bc08ae"
  },
  {
    "old": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️expected-stock.json",
    "new": "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/📦️expected-stock.json",
    "sha256": "a4ebae4a1c1829b58ec683d360792a54d70a4b168a244061415290716ad55c9d"
  }
]
```

### Authored Paths

```json
[
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🔣️.json",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🎬️writer-migration/🔣️.json",
  "✏️s/🔌️plugins/🌊️flow/🎬️action-cohort/🔣️.json",
  "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🌊️flow/🧫️fixtures/🎬️action-cohort/🔣️.json",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️retirement.json",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧫️fixtures/♻️retirement/🔣️.json",
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️expected-stock.json",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs",
  "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/📦️expected-stock.json"
]
```
