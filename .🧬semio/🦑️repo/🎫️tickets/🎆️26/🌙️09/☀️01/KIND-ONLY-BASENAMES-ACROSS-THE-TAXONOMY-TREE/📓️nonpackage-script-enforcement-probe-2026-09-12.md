# Nonpackage Script Enforcement Probe

Root ran the current public classifier, filename policy and normalizer against the actual repository root `📜️script.ts` on 2026-09-12. This is a read-only source-ownership probe, not native execution of the root task body.

| Probe | Observed Result |
| --- | --- |
| `classifyPackageSourceDisposition(content, root-script, typescript)` | `unresolved` |
| `implementationLeafBasenameFinding(path, taxonomy)` | `null` |
| Scoped `inventoryTaxonomy`, one actual source entry | `fixedContractId: root-script`, `fileKind: null`, `packageRole: not-package` |
| Inventory source/body ownership violations | None |
| Other inventory warnings | Five `opaque-reference-target` comment references to `compose` |

The observed root source content hash was `8aeaed79e00f2324f5df457002f842caafeff40a8fc4d72aaccaf269519bc0cc`, size 2,189,869 bytes. These identify the inspected snapshot and are not a permanent implementation hash contract. The complete public result was saved under generated/coordinator/root-script-enforcement-probe.json.

The code explains the behavior: normalization's `classifyPackageRole` returns `not-package` immediately when `packageLocation` is absent, before resolving a fixed source disposition; discovery's `collectPackageRoles` runs inside package roots; the filename gate permits a matching fixed contract before source-body classification. The root filename remains user-mandated, so removing its fixed filename contract would be the wrong correction. Shared semantic enforcement needs a separate source-disposition path for mandatory scripts outside package containers, with its own clear diagnostic and positive/hostile portable fixtures.

This is separate from the current manifestless-package fix: a source outside every package boundary remains outside that path even after absent manifests are inspected. The nine router argument counterexamples are also a different defect in the classifier itself. Correct both precise layers without broadening filename exemptions or moving actual domain behavior into another generic helper.

The five comment-reference warnings are retained as observed warnings. This probe does not establish that they are active imports or authorize deleting the referenced source-as-data fixtures.
