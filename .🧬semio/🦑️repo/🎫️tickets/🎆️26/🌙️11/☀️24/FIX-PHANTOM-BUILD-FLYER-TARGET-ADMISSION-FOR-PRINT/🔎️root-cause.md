# 🔎️ Root Cause: Phantom `build-flyer` Admission for `@semio-tech/print`

## Symptom

- `repo run @semio-tech/print:test -- quick` → `BUILD_OWNER_FAILED … Unknown admitted build target: @semio-tech/print:build-flyer`.
- `repo run @semio-tech/print:test-assignment` stalled in the dependency stage.

## Why `build-flyer` exists at all

`build-flyer` is not authored; it is **synthesized** from the Print catalogue
(`📋️project.json` → `metadata.printCatalog` → `🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json`) by
`printDocumentTargets` in `🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs`. It produces one
`build-<id>`/`watch-<id>` per document (`report`, `paper`, `phd`, `thesis`, `flyer`, `forschungsbericht`,
`zwischenbericht`, `kompaktbericht`, `viz-…`) and rewrites `build`/`build-viz` to depend on them.
`build-thesis` (README, `🧪️tests/🎓️thesis`) is one of these synthesized targets, so it is a real target.

## The defect

Two code paths assemble an owner's target table and disagreed:

| Path | Targets |
| --- | --- |
| `projectWithDefaults` (`🧱️project-assembly/🟨️.mjs`) | cargo ∪ authored ∪ component ∪ **print catalogue** |
| `discoverBuildOwnerAdmissionFactsV1` (`🔎️owner-discovery/🟨️.mjs`) | cargo ∪ authored ∪ component — **no print catalogue** |

Admission (`🚪️admission/🟦️.ts`) selects `test` → `dependsOn: build`. Owner discovery normalizes the selected
`build` through `projectWithDefaults`, which returns `build` with `dependsOn: [build-report, build-flyer, …]`.
Admission then asks owner discovery for those dependency tasks, and owner discovery looks them up in its own
table, which lacks every `build-<id>` → `Unknown admitted build target`. `build-flyer` is simply the first
of them alphabetically. The failure is deterministic on unchanged inputs, which is why `repo` reported
“cannot be inferred: reproduces on unchanged inputs”.

## Fix

`🔎️owner-discovery/🟨️.mjs` now merges `printDocumentTargets(json, owner.root, workspaceRoot, scripts)` last,
with the same precedence as project assembly (print catalogue > component > authored > native).

## Second defect exposed: `Generated consumer task absent`

With admission fixed, the read failed later in `📦️project-nodes/🟨️.mjs`: admission now reports generated
task prerequisites for `build-<id>` (its command closure imports generated sources), and project-nodes
applied them with `applyBuildGeneratedTaskDependenciesV1` to the **raw authored** `📋️project.json`, where no
`build-<id>` exists yet (the catalogue targets appear only inside `projectWithDefaults`).

The admission map rides inside the job JSON through the farm/pool/record-key paths, so it stays there.
`withOwnerGeneratedDependenciesV1` (project-nodes, used by the wave and by `ownerJobV1`) applies it to
authored consumers and leaves consumers the Print catalogue synthesizes (`printDocumentTargetNamesV1` in
component-targets: `build`, `build-viz`, `build-<id>`, `watch-<id>`) to project assembly, which replaces those
targets anyway and derives the same producers from their command closure
(`closures.generatedDependencies` in `🧱️project-assembly/🟨️.mjs`). Any other absent consumer is still refused.
Test: `📦️project-nodes/🧪️tests/🖨️print-generated` (fixture + schema + SQLite oracle).

## Tests

- `🔎️owner-discovery/🧪️tests/🔁️observation` gained the `print-catalog-precedence` case (fixture + schema +
  SQLite oracle): the print catalogue `build` wins and `build-flyer` is present only when a catalogue exists.
- Extracted-function tests that inject owner-discovery bindings now inject `printDocumentTargets`
  (`🗃️owner-names`, `🔁️observation`, `👁️observation/🧪️tests/👥️analysis-pair`, `🧪️tests/🕸️graph/🌱️native-scope`).
- `👥️analysis-pair` › “uses one physical Source image …” fails **with and without** this change
  (verified by temporarily removing the fix): pre-existing, unrelated.

## Docs

README German document-id list was missing `thesis`; added. All `repo run @semio-tech/print:*` targets
advertised in the README exist (authored or catalogue-synthesized).
