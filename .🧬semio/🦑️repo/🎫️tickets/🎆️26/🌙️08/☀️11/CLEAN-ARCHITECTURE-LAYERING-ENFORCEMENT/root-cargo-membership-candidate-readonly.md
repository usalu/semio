# Root Cargo Membership Candidate Read-Only Audit

Current root manifest changed by other authors during inspection (four new leading members); use manifest paths as stable evidence, not the earlier line offsets. No Cargo/native/test execution or edits. Candidate membership is not deletion-qualified.

## Exact Artifact Owners and Incoming Dependencies

All four manifests reside under `🧰️framework/🛍️products/💻️os/🔨️modules/`:

| Module-relative manifest | Package |
| --- | --- |
| `♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml` | semio-framework-artifact-infinite-dag |
| `🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml` | semio-framework-artifact-flow-flow |
| `🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust/Cargo.toml` | semio-framework-artifact-workflow-run |
| `🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust/Cargo.toml` | semio-framework-artifact-workflow-workflow |

These leaf manifests do not declare their own workspace. Current root workspace dependencies at 354/355/357/360 supply the four paths. **Membership globbing alone is insufficient:** OS flow package `🌊️flow/📦️packages/🦀️rust/Cargo.toml:28–29` requires infinite DAG and flow-flow through workspace dependencies, and flow-flow artifact manifest:36 requires infinite DAG. Thus deleting an artifact leaves a required incoming dependency from a surviving parent/module. Handcraft child-owned mounted contribution selection/producer boundaries before claiming deletion independence.

## Candidate Cohort Roster

For just the four named artifacts, these existing taxonomy cohorts cover exactly their artifact-family level and avoid S/WFC trees:

```toml
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/*/📦️packages/🦀️rust"
"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/*/📦️packages/🦀️rust"
"🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/*/📦️packages/🦀️rust"
```

A complete root taxonomy **candidate** representing all current explicit root member locations uses the following containing cohorts, with the exclusions below. Every inspected explicit member is beneath one of these prefixes and ends in the exact Rust package leaf. They also admit presently unlisted packages; this expansion is intentional only after physical eligibility qualification.

```toml
"🧰️framework/🔨️modules/**/📦️packages/🦀️rust"
"🧰️framework/🛍️products/*/**/📦️packages/🦀️rust"
```

Do not use repository-wide `**/Cargo.toml` membership: member patterns name package directories, and that scope crosses S, WFC, standards and standalone owners. Do not use packages/*: TypeScript and other language package directories are not Cargo crates.

### Required Nested-Owner Exclusion Roster

The current Framework `[workspace]` source scan found these nested workspace manifests. Broad cohorts must not swallow their packages:

- `🧰️framework/Cargo.toml` (root owner already excluded; the root itself does not match the package cohorts).
- `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.toml`.
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/📦️packages/🦀️rust/Cargo.toml`.
- `🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml`.
- `🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/📦️packages/🦀️rust/Cargo.toml`.
- `🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.toml` (not matched itself, but its owned descendants must stay out).
- `🛍️products/💻️os/🧪️testing/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.toml`.
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testing/🌊️actor-import/👽️guest/📦️packages/🦀️rust/Cargo.toml`.

Paths above omit the common `🧰️framework/` prefix after the first item. Preserve explicit standalone-owner exclusions and standards/oracles/guest boundaries. Add physical nested-owner eligibility as a defining discovery condition; source inspection does not prove Cargo glob exclusion precedence or an exact present cohort expansion.

There is an existing scope conflict to resolve explicitly: root currently names `💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust` as a member. A blanket fixture exclusion cannot both retain that existing test component member and reject every fixture. Classify this actual test producer deliberately rather than claiming both requirements hold. Production member cohorts should reject incidental fixtures; required test producers need explicit own admission.

## Semantic Impacts and Risks

- No current root default-members or patch section found. With virtual root defaults, changing membership changes default workspace selection; it can admit more crates than the original list even when a targeted General invocation is used.
- Root workspace dependency paths remain required only when inherited by actual selected/admitted members, but missing required path dependencies can defeat workspace resolution. Remove incoming artifact requirements through child-owned contribution boundaries, not by merely deleting unused dependency table rows.
- Root profiles are shared compiler policy. Package-specific profile names are tuning references, not source membership; preserve broad profile settings and qualify missing-package diagnostics independently. Do not treat these as a proven hard deletion dependency.
- Glob discovery of physically present package directories addresses absent named member admission; it cannot remove another surviving crate's compile-time dependency or module import.
- Repo member-manifest metadata already uses Framework Rust package discovery. Keep Cargo cohort semantics and that authored admission aligned, and test nested-owner rejection using the existing controlled discovery implementation.
- S is root-excluded and outside proposed cohorts. Its plugin workspaces remain independent. The procedural bridge's required generation2d/3d path dependencies remain a separate surviving-tooling failure; root membership refactoring does not repair them.

Before implementation acceptance, compare expanded physical candidates with current explicit/inherited package closure, classify newly admitted crates and fixtures, exercise actual Cargo admission with isolated deletion vectors and original resource authority, and verify no nested workspace conflicts. None of these runtime qualifications were performed here.
