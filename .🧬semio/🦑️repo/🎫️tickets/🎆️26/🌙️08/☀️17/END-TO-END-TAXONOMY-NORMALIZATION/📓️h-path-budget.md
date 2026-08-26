# H-PATH-BUDGET — Cross-platform normalized path budget

## Scope and source

Read-only audit of the canonical full inventory artifact:

`📊️taxonomy-inventory/🔣️.json`

The artifact is taxonomy schema v7, contains 103,892 entries and 41,539 top-level violations, and has inventory digest `68166b9fdcf70c4ad85d3a521803c4f0e460c5a27a28c0c0cf24f73521878934`. No fresh repository inventory or generator was run. No production, schema, engine, test, manifest, Git state, opaque tree, or temporary opaque tree was changed.

The policy under audit is `collisionPolicy.maxPathBytes = 240`. The engine applies it to the NFC-normalized destination with `Buffer.byteLength(normalizedPath, "utf8")`, not necessarily to the source spelling. This explains source paths as short as 231 bytes among the violations: their registered semantic directory prefixes make the normalized destination at least 241 bytes.

## Stable counts

The artifact contains **14,511 distinct offending entries and 14,511 distinct normalized paths**. They comprise 7,849 files and 6,662 directories.

Normalized UTF-8 byte distribution:

| Statistic | Bytes |
|---|---:|
| Minimum | 241 |
| p50 | 258 |
| p90 | 278 |
| p95 | 284 |
| p99 | 294 |
| Maximum | 518 |

Raising the threshold would leave the following residuals:

| Threshold | Paths still over |
|---:|---:|
| 240 | 14,511 |
| 250 | 10,324 |
| 260 | 6,160 |
| 270 | 2,995 |
| 280 | 1,042 |
| 290 | 263 |
| 300 | 35 |
| 320 | 11 |
| 400 | 7 |

The maximum production/plugin mutation path is 311 bytes, CAD examples reach 293, framework paths reach 279, and ticket/governance evidence reaches 518. A threshold high enough to suppress the census would not be a cross-platform policy.

### Minimal normalized frontiers

For each offending normalized path, the audit selected the first segment-boundary prefix over 240 bytes whose parent prefix is at most 240, then deduplicated those prefixes. This produces **5,630 minimal offending frontiers**:

| Pattern | Offending entries | Minimal frontiers |
|---|---:|---:|
| Plugin mutation tests | 14,367 | 5,527 |
| Plugin examples | 102 | 91 |
| Ticket/governance evidence | 31 | 5 |
| Framework | 10 | 6 |
| Other plugin | 1 | 1 |

Thus **99.0% of entries** and **98.2% of frontiers** are one repeated mutation-test hierarchy, not 14,511 independent naming decisions.

Representative frontier:

```text
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/
🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
✂️disconnect-nodes/🧪️tests/
🧪️rejects-severing-an-edge-that-is-not-in-the-graph
```

The scenario directory is 252 bytes. Its 13-entry subtree reaches 294 bytes through registered physical fixture leaves such as:

```text
./🦀️.rs
./🦠️mutation/🔣️.json
./🔺️diff/🚫️.absent
./📸️snapshot/⬅️before/🔣️.json
./📸️snapshot/➡️after/🔣️.json
./🎯️outcome/🔣️.json
```

### Counts by owner

| Owner | Entries | Frontiers |
|---|---:|---:|
| `📕️norm` | 3,925 | 1,411 |
| `🗄️stdio` | 3,021 | 1,015 |
| `🏛️architect` | 2,807 | 932 |
| `🧱️block` | 501 | 331 |
| `🏗️fem` | 435 | 207 |
| `🌀️procedural` | 407 | 152 |
| `📸️remodel` | 372 | 141 |
| `🎥️shooting` | 286 | 133 |
| `🧩️puzzle` | 247 | 201 |
| `📐️cad` | 227 | 156 |

The top three owners account for 9,753 entries. Owner distribution follows mutation/scenario count rather than a distinct owner-specific filesystem defect.

## Mutation scenario budget

The repeated mutation pattern comprises **1,524 scenario groups and 14,367 offending entries**. For these groups:

| Measure | Min | p50 | p90 | p99 | Max |
|---|---:|---:|---:|---:|---:|
| Parent `…/🧪️tests` bytes | 163 | 185 | 199 | 209 | 214 |
| Scenario segment bytes, including `🧪️` | 17 | 41 | 58 | 68 | 76 |
| Deepest suffix after scenario | 42 | 42 | 42 | 42 | 42 |
| Shortening required for whole subtree | 1 | 31 | 48 | 60 | 71 |

The exact safe scenario-segment budget is contextual:

```text
scenarioBytes <= 240 - parentBytes - 1 separator - 42 descendant reserve
```

A global scenario-slug cap cannot solve the current hierarchy. At the deepest parent, the formula yields `-17` before any scenario text exists. The structural prefix must shrink first. Once the prefix is compact, scenario slugs should be constrained by this formula and remain readable, unique within their mutation, and vocabulary-backed. They must not be truncated blindly or replaced by hashes.

## Physical-leaf savings already realized

There are **7,728** offending `component.*` source leaves that normalize to physical kind-only leaves. Removing the nine ASCII bytes in `component` saves exactly:

- **9 bytes per basename**;
- **69,552 basename bytes** in aggregate.

The same normalization also adds the required seven UTF-8 bytes of the `🧪️` prefix to formerly unprefixed scenario directories. The complete path therefore saves only:

- **2 bytes per path**;
- **15,456 path bytes** in aggregate.

All 7,728 normalized paths remain over 240. The physical-leaf invariant is correct and worth keeping, but it cannot compensate for the structural/test-scenario hierarchy.

## Can the limit soundly be raised?

**No.** The repository does not provide the guarantees needed for a higher zero-touch limit, and 240 itself should remain a conservative repo-relative guard rather than be represented as an absolute Windows guarantee.

Repository/runtime evidence:

- Root metadata declares `packageManager: bun@1.2.5`, permits Bun `>=1.2.0`, and CI/devcontainer provisioning installs a non-fixed latest Bun. The audit implementation uses Node/Bun `Buffer.byteLength(..., "utf8")` consistently, but no runtime setting extends filesystem path limits.
- Nx is invoked through Bun/Node child processes. Node/Bun filesystem calls inherit host path behavior; the repository has no path abstraction that always emits Windows extended-length paths.
- The devcontainer pins Git 2.53 and uses the short Linux workspace `/workspaces/semio`. Its setup configures Git safe directories and signing, but neither devcontainer nor native setup configures `core.longpaths`.
- Native Windows setup contains Windows toolchain branches but does not provision the Windows `LongPathsEnabled` system policy, constrain the checkout root, or verify long-path awareness for Bun, Node, Git, editors, archive tools, Cargo, CMake, and other consumers.
- `.gitattributes` controls line endings and binary treatment only; it does not alter path handling.

Windows constraints are not expressed in UTF-8 repo-relative bytes. Traditional Win32 paths are limited to 260 UTF-16 code units including the absolute checkout prefix and terminator; extended-length paths can be much longer (approximately 32,767 UTF-16 code units) only when the OS policy and each participating application/API path are long-path aware. Git for Windows additionally needs its long-path behavior enabled. The repository cannot assume any of those host/global settings under a zero-touch native requirement.

Emoji make the current UTF-8 budget conservative relative to UTF-16—many emoji use four UTF-8 bytes but two UTF-16 code units, and VS16 uses three bytes but one code unit—but the unchecked absolute checkout prefix consumes the apparent margin. Raising to 300 or 311 would exceed traditional Windows capacity before the checkout prefix is counted. A sound future policy would check both normalized repo-relative UTF-8 bytes and prospective absolute Windows UTF-16 units for an explicitly provisioned checkout root; it would not relax this guard globally.

## Deterministic semantic-preserving remedies

### 1. Preferred: compact mutation-test projection

Move the repeated test projection from the deep schema implementation hierarchy to the artifact while retaining its profile, mutation ID, scenario ID, and physical fixture structure:

```text
old:
<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
<mutation>/🧪️tests/<🧪️scenario>/<physical fixture>

new:
<artifact>/🧪️tests/🪆️1-any/<mutation>/<🧪️scenario>/<physical fixture>
```

This uses existing semantic meanings—tests, profile `1/any`, registered mutation, registered test case—and removes duplicated structural ownership. It saves **69 bytes on every mutation-test path** and resolves **14,362 of 14,367 mutation entries** without changing scenario meaning or physical leaves. The five residual entries are only one or two bytes over; all are `📕️norm/📗️din16798` humidification/infiltration scenarios and can be resolved with readable two-byte-shorter scenario wording.

This is preferable to globally abbreviating every production structural name because it changes the smallest repeated frontier: one projection contract and 1,524 scenario owners rather than unrelated schema/editor/runtime paths.

The schema contract should encode the projection reversibly:

```text
artifactId + standardVersion + subsetId + mutationId + scenarioId
```

No identity is derived from position alone, no opaque hash is introduced, and reference adapters can deterministically map between the schema owner and its test projection.

### 2. Secondary: compact registered structural names/profile

Measured alternatives demonstrate why partial renames are insufficient by themselves:

| Candidate | Paths brought within 240 | Still over |
|---|---:|---:|
| Registered structural abbreviations (`artifacts→arts`, `standards→stds`, `subsets→sets`, `schema→spec`, `mutations→ops`) | 8,759 | 5,752 |
| Collapse `standards/1/subsets/any` to registered `🪆️1-any` | 10,489 | 4,022 |
| Profile collapse plus those structural abbreviations | 12,921 | 1,590 |
| Preferred compact mutation-test projection | 14,362 | 149 total, including all non-mutation cases |

If shorter structural slugs are adopted, they must be schema-registered canonical names with one meaning, not ad hoc abbreviations introduced per owner. The compact `🪆️1-any` profile is semantic and reversible; arbitrary truncation is not.

### 3. Remaining 144 non-mutation entries

- **102 CAD example entries / 91 frontiers:** move the profile-independent model-definition catalog to an artifact-level example-asset projection. If profile-dependent, retain the compact `🪆️1-any` segment. Register a shorter structural `🏗️models` name in place of mixed-case `🏗️modelDefinitions`, then apply readable byte budgets to AEC model/action IDs. Preserve the full semantic ID in its manifest rather than hashing filenames.
- **10 framework entries / 6 frontiers:** these are misplaced ticket/governance trees nested beneath Rust package roots, such as `.../📦️packages/🦀️rust/.🧬semio/.../PHASE-9...`. Move them back to their ticket owner; shortening package production names would conceal misplaced evidence.
- **31 ticket/governance entries / 5 frontiers:** 16 are the retained synthetic path-budget fixture, 12 are an older window-policy fixture, two belong to one 264-byte ticket slug, and one is a 244-byte report filename. Ticket creation should enforce a slug byte budget derived from the ticket prefix plus a reserved artifact suffix. Fixture/report names should use concise semantic titles. Synthetic over-budget paths should be represented as inventory/path-policy fixtures rather than checkout-hostile physical descendants when cross-platform execution is required.
- **1 Draw entry:** `.../🏅️standards/🔖️1/🪆️subsets/✳️any/.../📋️project.json` is 242 bytes. A compact profile path resolves it without renaming the fixed Nx manifest.

## Windows-reserved and trailing-dot/space audit

The apparent “two” records in each class are the same distinct entry emitted once in `entries[].violations` and once in the top-level `violations` array. There is **one distinct Windows-reserved path and one distinct trailing-space path**.

### Windows-reserved

```text
.🧬semio/.../END-TO-END-TAXONOMY-NORMALIZATION/
🧪️s-test-collisions-0gXMTl/🧪️tests/🧪️fixture/🧪️platform/CON.ts
```

This is the retained disposable collision fixture, not a production semantic source. The engine correctly matches `CON.ts` against `con|prn|aux|nul|com[1-9]|lpt[1-9]`. It cannot be checked out or created reliably on native Windows. Preserve the test intent through a serialized inventory/path-policy fixture or a pure path-policy test, and keep retained execution repositories outside production normalization scope. Do not “fix” the sentinel by inventing a semantic emoji.

### Trailing space

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/
FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/<single-space filename>
```

This is a 38,467-byte UTF-8 compiler-warning log, content hash `d5246ed036182a3505508fe7c264d1da42fac2d46b2ff391ca3c251cbbebf7fc`, stored as a file whose entire name is one space. It is not an empty directory. Assign it an existing registered ticket evidence/log contract with a descriptive semantic filename and preimage guard. Never normalize by trimming: trimming would produce an empty filename and creates platform-dependent collision behavior.

## Acceptance checks

1. Recompute the canonical full inventory: zero normalized paths exceed 240 bytes, excluding no production owner and without increasing `maxPathBytes`.
2. The mutation-test projection round-trips `(artifact, version, subset, mutation, scenario)` and preserves all fixture bytes and reference edges.
3. Scenario creation rejects a name when `parentBytes + 1 + scenarioBytes + 42 > 240`, before writing any path.
4. The five DIN 16798 residual scenarios receive readable, unique, registered slugs and remain byte-identical in content.
5. CAD models/actions keep stable semantic IDs in manifests and have deterministic readable path names.
6. Misplaced framework ticket trees are absent from package roots; ticket artifacts stay with their ticket owner.
7. Ticket opening and report/fixture creation enforce contextual byte budgets.
8. Native Windows validation runs without registry edits or global Git configuration and never materializes a reserved or trailing-space sentinel.
9. The policy continues to reject reserved device names and trailing dot/space segments before filesystem mutation.
10. A checkout-prefix-aware Windows UTF-16 diagnostic is reported separately from the canonical 240-byte repo-relative invariant.

## Evidence commands

All reductions parsed the canonical JSON with Bun and used `Buffer.byteLength(path, "utf8")`. No filesystem census was substituted for the artifact. Representative read-only command forms:

```text
bun -e '<load canonical inventory; count violation codes and entry violations>'
bun - '<deduplicate first over-budget normalized prefixes and aggregate owners/patterns>'
bun - '<measure scenario parent/slug/suffix bytes and candidate hierarchy transforms>'
rg -n '<long path provisioning/configuration>' <explicit non-opaque config and production files>
```

The candidate transforms are measurements, not applied migrations. The preferred remedy is the compact semantic projection plus contextual scenario budgets, not a policy increase.
