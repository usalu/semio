# 1103 Norm Registration Audit

STATUS: READ-ONLY PHYSICAL INVENTORY COMPLETE. Five unresolved EN1993 dependency identities; no matching physical packages.

Audited all 950 physical Cargo.toml files under Norm artifacts using recursive filesystem enumeration and Bun.TOML.parse, including manifests untracked by git. Fifteen workspace owners cover 935 explicit package.workspace members. A second pass checked literal workspace.members against files and actual owner declarations: no missing or unlisted members. Also inspected target-specific dependencies/dev-dependencies/build-dependencies: no additional missing inherited keys. Existing path-based workspace registrations all resolve to Cargo.toml and match package name (or explicit package rename). No source edits or Cargo execution.

## Workspace Inventory

| Owner | Members | Missing Inherited Keys | Existing Path/Identity Mismatches |
|---|---:|---:|---:|
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/Cargo.toml | 68 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml | 77 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.toml | 19 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.toml | 104 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/Cargo.toml | 98 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/Cargo.toml | 44 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/Cargo.toml | 58 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/Cargo.toml | 54 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/Cargo.toml | 69 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.toml | 21 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.toml | 16 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml | 186 | 5 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.toml | 24 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml | 22 | 0 | 0 |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/Cargo.toml | 75 | 0 | 0 |

## Exact Unresolved EN1993 Rows

All five consumers inherit the missing key from EN1993 Cargo.toml. There is no canonical relative registration path for the requested -more package because no Cargo.toml declares that package anywhere in the audited physical tree. Registering the existing parent under that key would produce an identity mismatch/self-dependency and would not supply the requested implementation.

- Missing key: semio-s-artifact-norm-en1993-mut-material-ductility-more
  - Consumer: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪material-ductility/📦️packages/🦀️rust/Cargo.toml
  - Actual consumer package: semio-s-artifact-norm-en1993-mut-material-ductility
  - Canonical existing member path relative to owner: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪material-ductility/📦️packages/🦀️rust
  - Matching requested package: ABSENT across 950 physical manifests.
- Missing key: semio-s-artifact-norm-en1993-mut-buckling-lengths-more
  - Consumer: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜buckling-lengths/📦️packages/🦀️rust/Cargo.toml
  - Actual consumer package: semio-s-artifact-norm-en1993-mut-buckling-lengths
  - Canonical existing member path relative to owner: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜buckling-lengths/📦️packages/🦀️rust
  - Matching requested package: ABSENT across 950 physical manifests.
- Missing key: semio-s-artifact-norm-en1993-mut-forming-gains-more
  - Consumer: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫forming-gains/📦️packages/🦀️rust/Cargo.toml
  - Actual consumer package: semio-s-artifact-norm-en1993-mut-forming-gains
  - Canonical existing member path relative to owner: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥫forming-gains/📦️packages/🦀️rust
  - Matching requested package: ABSENT across 950 physical manifests.
- Missing key: semio-s-artifact-norm-en1993-mut-fatigue-stresses-more
  - Consumer: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳fatigue-stresses/📦️packages/🦀️rust/Cargo.toml
  - Actual consumer package: semio-s-artifact-norm-en1993-mut-fatigue-stresses
  - Canonical existing member path relative to owner: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏳fatigue-stresses/📦️packages/🦀️rust
  - Matching requested package: ABSENT across 950 physical manifests.
- Missing key: semio-s-artifact-norm-en1993-mut-fastened-joints-more
  - Consumer: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛fastened-joints/📦️packages/🦀️rust/Cargo.toml
  - Actual consumer package: semio-s-artifact-norm-en1993-mut-fastened-joints
  - Canonical existing member path relative to owner: 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛fastened-joints/📦️packages/🦀️rust
  - Matching requested package: ABSENT across 950 physical manifests.

## Repair Boundary

EN1991's seven earlier registrations are present and valid now. The EN1993 issue is not another simple omitted registration batch: these five dependency identities have no matching current member. Determine their authored implementation/source intent before editing the dependency contracts. Do not introduce synthetic crates, package aliases, or guessed paths. The independent inventory was sent to execution_baseline, who owns the coherent regression-first repair. Existing path registrations show zero identity mismatches; the mismatch risk is hypothetical only if the absent -more names are pointed at parents.
