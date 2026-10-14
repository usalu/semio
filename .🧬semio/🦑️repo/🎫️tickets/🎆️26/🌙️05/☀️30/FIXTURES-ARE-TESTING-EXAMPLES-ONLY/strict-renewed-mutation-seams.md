# Renewed Post-Move Topology and Mutation Seams

One fresh rg file-bearing snapshot found 2070 recognized fixture/example collections, 1608 outside true tests ancestry. Empty/ignored roots excluded. Actual modules named collections are excluded. Inputs 🗑️generated/strict-postmoves-inputs.json; hashes 🗑️generated/strict-renewed-seams-hashes.json. No tests or Cargo executed here; Root's executed evidence is separate.

Current TestPlatform repairs use declared module vocabulary for both tests and fixtures ancestry. Raw inFixture and dependency exemptions now use actual classifier, addressing previous source-backed counterexamples. Confirmed by source inspection only.

## Exact Remaining Mutation Patches

- Normalization mutationFixtureMirrorOwnerPath:4408 needs mirror `/${testsDirName}/${testFixturesDirName}/🧬️mutations/`; strip both collection components before schema owner reconstruction. A string replacement adding tests only in the constructed fixture root is insufficient.
- provenMutationLeafOwners:4463 computes root via dirname(dirname(directory)); directory owner/tests/fixtures/<leaf> now derives owner/tests/schema/mutations incorrectly. Determine fixture owner by removing both tests and fixtures, while preserving exact fixture leaf/case identities.
- normalization fixture source/destination:4851/4852 and expected set:4913 must all construct subset/tests/fixtures/mutations. Pairing regex :4918 must match new prefix together. Every recognized projected subtree should retain source identity and leaf bytes; no compatibility branch for old placement.
- taxonomyClosedScope:7246 currently returns segments before fixtures. A moved scope owner/tests/fixtures/mutations/x therefore closes to owner/tests, which excludes owner/schema implementation half and produces false missing pairing. For fixture branch return prefix before tests; schema branch remains prefix before schema. For scope exactly owner/tests/fixtures holding mutations, same correction applies. Named module fixtures must not become fixture branch automatically.
- Existing :5435 guard `basename(dirname(root)) === testFixturesDirName` already excludes fixture mutation roots because tests insertion does not alter immediate parent of mutations. Preserve it; do not over-adjust.
- Discovery mutation projection :2271 and exactFixture grammar:4839 still omit tests. Update grammar declaration and matching validation together with fixture source roots.
- TestPlatform mutationVectorRegistryBreaches:1336 still uses sourceOwner/fixtures/mutations and needs sourceOwner/tests/fixtures/mutations; grouped leaf loops consume this computed root and need no extra ancestry guess.

## Additional Physical/Declarative Coordinates

Root physical `🧫️fixtures` contains rust-warnings, outcome-law-gate, diff-only-law-gate and lanes JSON. It lacks tests ancestry; canonical root destination is 🧪️tests/🧫️fixtures, subject to Root's ownership choice.

Taxonomy frozenCoordinateEvidenceContracts still point library/fixtures at725,740,768; CAD path projection evidence at1526. When Root moves library fixture files, update current-source authority coordinates only. Historical ticket evidence at755 and immutable document lineage must retain authored evidence; do not rewrite historical source snapshots blindly. Cases embedded in taxonomy include statutes specimen pathPattern/expected path :24558/:24564 and STEP domain fixture coordinates around27010; these are data coordinates whose reader role must be assessed before global text replacement.

The remaining broad collection roots are listed below for bounded relocation planning, not a release runtime claim.

- 🧫️fixtures: 1
- 🧰️framework: 741
- 🌎️hub: 33
- 🎓️teaching: 1
- ♻️mit-bestand: 1
- ✏️s: 831
