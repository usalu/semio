# Terra glTF Artifact-Definition Acceptance Correction

## Baseline And Ownership

- Owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json` plus the direct root validator needed to accept exact live semantic component IDs. Repository, `✏️s`, and `🗄️stdio` instructions were reread; no deeper instruction file exists.
- Baseline definition SHA-256: `93ca97f2f5b45564a2d0e099d1caa4f7860f1643193d5dc9a24dd6ad4b519104`.
- The definition is a protected concurrent added/staged source file, but this explicit acceptance lease owns its source-of-truth correction. Existing standard/subset/inference/mutation manifests are dirty concurrent work and remain read-only.
- Stale `s.stdio.gltf.standard.2-0` occurs only in the artifact definition: its standard, profile, source dialect, representation, and support-ledger profile. The active standards manifest owns the canonical `s.stdio.gltf.standard.2.0` ID.
- The registry and artifact TypeScript facade consume the definition by path, not by stale standard/component literal; no registry, facade, generated glue, or central registrar update is required.

## Live Component Source Roster

- Standard: `s.stdio.gltf.standard.2.0` from `🏅️standards/🔣️component.json`.
- Concrete I/O route components (six `io` members): `s.stdio.gltf.inference.io.binary`, `s.stdio.gltf.inference.io.text`, `s.stdio.gltf.io.export.artifacts`, `s.stdio.gltf.io.import.artifacts`, `s.stdio.gltf.io.mutation.binary`, and `s.stdio.gltf.io.mutation.text`.
- Mutation components (28): `s.stdio.gltf.mutation.no-mutation`, `s.stdio.gltf.mutation.set-snapshot`, `s.stdio.gltf.mutation.set-asset`, `s.stdio.gltf.mutation.insert-scene`, `s.stdio.gltf.mutation.remove-scene`, `s.stdio.gltf.mutation.set-scene`, `s.stdio.gltf.mutation.insert-node`, `s.stdio.gltf.mutation.remove-node`, `s.stdio.gltf.mutation.set-node`, `s.stdio.gltf.mutation.transform-node`, `s.stdio.gltf.mutation.reparent-node`, `s.stdio.gltf.mutation.bind-node-mesh`, `s.stdio.gltf.mutation.insert-mesh`, `s.stdio.gltf.mutation.remove-mesh`, `s.stdio.gltf.mutation.set-mesh`, `s.stdio.gltf.mutation.insert-accessor`, `s.stdio.gltf.mutation.remove-accessor`, `s.stdio.gltf.mutation.set-accessor`, `s.stdio.gltf.mutation.insert-material`, `s.stdio.gltf.mutation.remove-material`, `s.stdio.gltf.mutation.set-material`, `s.stdio.gltf.mutation.bind-primitive-material`, `s.stdio.gltf.mutation.insert-buffer`, `s.stdio.gltf.mutation.remove-buffer`, `s.stdio.gltf.mutation.set-buffer`, `s.stdio.gltf.mutation.insert-animation`, `s.stdio.gltf.mutation.remove-animation`, and `s.stdio.gltf.mutation.set-animation`.
- Inference components (15, deduplicated from the inference root and geometric-analysis facet): `s.stdio.gltf.inference.size`, `s.stdio.gltf.inference.area-volume`, `s.stdio.gltf.inference.compactness`, `s.stdio.gltf.inference.proportion`, `s.stdio.gltf.inference.mass-distribution`, `s.stdio.gltf.inference.curvature`, `s.stdio.gltf.inference.thickness`, `s.stdio.gltf.inference.concavity`, `s.stdio.gltf.inference.clearance`, `s.stdio.gltf.inference.adjacency`, `s.stdio.gltf.inference.orientation`, `s.stdio.gltf.inference.symmetry`, `s.stdio.gltf.inference.roughness`, `s.stdio.gltf.inference.topology`, and `s.stdio.gltf.inference.geometric-analysis`.

## Required Validator Alignment

The direct root definition validator previously permitted only synthetic `*.codec.<slug>.vN`, `*.mutation.<slug>.vN`, and `*.inference.<slug>.vN` identities, and rejected the canonical dotted `2.0` standard revision. The correction accepts the existing canonical identity grammar for a revision and removes the obsolete synthetic-shape gates while retaining canonical `s.stdio.*` identity validation and mandatory executable-registration metadata. It adds no alias or parallel legacy identity.

## Applied Source-Of-Truth Correction

- Replaced every glTF definition reference to `s.stdio.gltf.standard.2-0` with the active `s.stdio.gltf.standard.2.0`, including the profile, source dialect, document representation, and support-ledger profile. The exact revision is now `2.0`.
- Declared the six manifested I/O codec boundaries with canonical IDs, source dialect endpoints, `unimplemented` support state, and `executable_registration: true`.
- Declared all 28 direct mutation commands and 15 direct inference components, with the support ledger exactly mirroring those two source-of-truth arrays.
- Updated the direct `stdio` definition validator in `📜️script.ts` to accept canonical dotted revision fragments and to require executable registration without imposing a parallel, synthetic versioned identifier grammar.
- Direct consumers require no edit: `📇️registry/📇️catalog.json` and `📇️registry/🦀️component.rs` consume the definition by path, and `🧊️gltf/🟦️component.ts` imports its local definition by path. No generated glue, registrar, or runtime facade changed.

## Validation And Release Evidence

- Post-change definition SHA-256: `0f59b17ec70542aca6a7ca7d6b2bd44ab7371b7bf4360a5a3c081391cb48cc7f`.
- JSON parsing passed.
- Manifest-to-definition roster check passed: exactly 6 codecs, 28 mutations, and 15 inferences; all codec endpoints are `s.stdio.gltf.standard.2.0.dialect.source`; every declared component has executable registration; both support-ledger arrays exactly equal their definition arrays.
- `bun ./📜️script.ts stdio quick` passed: 36 artifacts, 40 dialects, 6 codecs.
- Referrer sweep found no remaining `s.stdio.gltf.standard.2-0` literal. The three direct definition consumers are path-only and remain valid.
- `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` passed: 59 components, 0 errors, 0 warnings, `No findings`.
- `git diff --check -- 📜️script.ts ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json` passed.

## Changed Paths And Scope

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json`
- `📜️script.ts` (only the direct definition-identity/registration validation required for the canonical schema source)
- This report

No generated artifacts, taxonomy contracts, registrars, glue, or manifests were edited under this lease. The prior Luna source-of-truth blocker is resolved by the canonical definition and its validated live component roster.
