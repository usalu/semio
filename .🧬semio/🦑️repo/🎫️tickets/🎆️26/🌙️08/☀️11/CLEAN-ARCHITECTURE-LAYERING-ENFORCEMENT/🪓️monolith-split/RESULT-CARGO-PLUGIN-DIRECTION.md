# Result: Cargo Plugin Direction (slice CARGO-PLUGIN-DIRECTION, prefix `cpd`)

Status: COMPLETE EXCEPT COMPILE EVIDENCE AND THE PROCEDURAL CORE DECISION (rule: 23 -> 3 strict violations; compiles blocked by the image headroom gate).

## 1. Verdict (current)

- The gate-visible baseline is **23** strict violations, not 121. The 121 of `DEPENDENCY-DIRECTION-AUDIT-2.md` §4 came from a superset scan of every tracked `Cargo.toml`. The real rule inventories only admitted workspace members and their path closure.
- 19 of the 23 (stdio contract crates) are fixed by re-roling.
- 1 of the 4 `cargo-plugins-no-artifacts` edges (`🪐️space/🫀️core`) is fixed by consuming the os host facade.
- Procedural core (3 edges): see §5.

## 2. Authoritative rule and baseline

Rule source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts` (`cargoDirectionInventory`, `cargoDependencyDirectionReport`), policy `cargoDependencyDirections` in `…/📚️library/🔣️taxonomy.json` (line 31003), runner `…/🦀️cargo/🏃️execution/🟦️.ts` (`verifyCargoDependencyDirection`), nx target `@semio-tech/repo-lib:lint-cargo-dependency-direction`.

The nx target needs the `repo` invocation envelope and `cargo metadata` over every workspace; the dda2 run showed it does not finish under load. I therefore ran the **same report function directly** (`🗑️generated/cpd-scan.ts`: `cargoDirectionInventory(root)` as graph and as inventory, taxonomy policy, no cargo). This is the exact logic of the gate minus the metadata-versus-inventory cross-check.

Baseline (before any edit, `cpd-baseline.txt`): `packages=2262 localDeps=31193 violations=23 problems=92`, by rule `cargo-modules-no-plugins` 19, `cargo-plugins-no-artifacts` 4. Problems (not in this slice): `missing-role` 50, `owner-role-mismatch` 41, `unknown-role` 1.

### Why the audit saw 121

- `🏭️bridge/Cargo.toml` (98 of the 102 `cargo-plugins-no-artifacts` rows) is **not** at `🏭️bridge/📦️packages/🦀️rust`. It is a standalone binary crate with its own `[workspace]`, listed in the plugin workspace `exclude` and `exclude-patterns` as `**/🏭️bridge/**`. The inventory walks only `cargoRepositoryPackages` plus their path dependencies, and no member depends on a bridge, so a bridge never enters the gate. The dda2 scan took every tracked manifest.
- The bridge is the test domain's production mutation bridge (`testBridgeDirName` in the taxonomy, `🧪️test/🏭️inventory`, runner `runRepositoryMutationBridgeScriptV1`). It reads `DESCRIPTORS` of the production mutation aggregates of the plugin's own artifacts, so a direct dependency on the artifact crates is its purpose. Splitting it per artifact would change the test-inventory contract (one bridge process per plugin, coordinates `artifact/standard/subset`) and is not a direction fix.
- The remaining 4 `cargo-plugins-no-artifacts` rows and the 19 `cargo-modules-no-plugins` rows are real and inside the gate.

## 3. Designs chosen (written before editing)

### 3.1 Stdio contract crates (19 violations)

Facts: 12 crates under `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/**` carry `role = "s-module"`. The rule `cargo-modules-no-plugins` matches `fromRoles s-module` to any target whose owner path has a `🔌️plugins` segment, so every contract crate that depends on another contract crate trips it. They depend only on `🧰️framework` crates and on each other. Consumers: 122 artifact-role crates, 299 library crates and the stdio plugin crate, all stdio artifacts (`semio-s-artifact-stdio-contract-*`). The taxonomy `ownerRoles` for `^✏️s/🔌️plugins/` admits `s-module, library, tool, test, extension`; `plugin` is admitted only for `^✏️s/🔌️plugins/[^/]+/📦️packages/🦀️rust$`, and 1,004 crates under plugin paths already use `library`.

Decision: they belong to the stdio plugin (not to `✏️s/🔨️modules`, whose crates are plugin-independent kernels). Re-role all 12 to `library`. No move, no dependency change, no lock change.

### 3.2 `🪐️space/🫀️core` (1 violation)

The core imported nine names from `semio-framework-artifact-space-space`. `semio-framework-os` already re-exports them explicitly through `semio-framework-os-host-space` (`pub use … as space; pub use space::*`), and the core already depends on `semio-framework-os`. Decision: import the names from `semio_framework_os`, delete the direct dependency (crate and workspace entry), fix `role = "kernel"` (unknown role) to `library`, and remove the line from the six lock files that embed the crate's dependency list.

## 4. Changes made

| Change | Files |
| --- | --- |
| `role = "s-module"` → `"library"` | 12 `Cargo.toml` under `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/**/📦️packages/🦀️rust/` |
| `🫀️core` imports `semio_framework_os::{empty_space_snapshot, space_backbone_uri, SpaceKind, SpaceMutation, SpaceRole, SpaceSnapshot, SpaceUser, SpaceVisibility, S_SPACE_SCHEMA}` | `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs` (line 21) |
| dependency `semio-framework-artifact-space-space` removed, role `kernel` → `library` | `✏️s/🔌️plugins/🪐️space/🫀️core/📦️packages/🦀️rust/Cargo.toml` |
| workspace dependency entry removed | `✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml` |
| one dependency line removed from the `semio-s-space-core` package block | `Cargo.lock` of `🪐️space/🫀️core`, `🪐️space/🏭️bridge`, `🪐️space/🗿️artifacts/🏠️home`, `🪐️space/🗿️artifacts/🪐️space`, `✏️s/🧑‍💻dev/🧹️fixture-sweep`, `🌎️hub` |

## 5. Procedural core (3 violations left, hand-off)

Edges (all `cargo-plugins-no-artifacts`):

| From | To |
| --- | --- |
| `🌀️procedural/🫀️core/🖼️semantic-ui` | os playbook artifact `semio-framework-artifact-playbook-playbook` |
| `🌀️procedural/🫀️core/🧬️generation/🪶️sqlite` | os flow artifact `semio-framework-artifact-flow-flow` |
| `🌀️procedural/🫀️core/🧬️generation/🪶️sqlite` | os playbook artifact |

Facts: both crates (642 lines in total, role `library`) exist to share code between `generation2d` and `generation3d` (10 library dependents plus the 3d artifact root). They use the flow and playbook snapshot model directly (about 15 flow names including `neural::*` and `retained::*`, seven playbook names). The targets are **framework-layer** artifacts under `🧰️framework/🛍️products/💻️os/🔨️modules/{🌊️flow,📖️playbook}/🗿️artifacts/…`, below the plugin layer. The rule matches them only because `toOwnerSegments` is the bare segment `🗿️artifacts` and `toRoles` contains `artifact`; the audit text itself words the rule as "a plugin depends on its **own** artifacts".

Options (none applied: each is outside this slice's ownership, and none can be proven by a compile while the build image is full, see §8):

1. Framework facades (recommended; matches the intent of the rule and the existing `🪐️space` precedent). `semio-framework-os-host-space` re-exports the space artifact explicitly and `semio-framework-os` re-exports that, which is exactly why `🪐️space/🫀️core` could be fixed without a new crate (§3.2). The same needs one facade for playbook (`📖️playbook/📦️packages/🦀️rust`, about seven names) and one light facade for the flow model (the existing `semio-framework-os-flow` family facade pulls canvas, wasm and host crates into a pure data crate, so it is the wrong host). Each new crate needs an owner row, six launch routes (`PRODUCER-CRATE-RECIPE.md` steps 8 to 10) and framework plus root lock regeneration; the two procedural core crates then change 22 `use` paths and two dependency lines. Framework territory.
2. Scope the rule. Add a closed `toOwnerPaths` selector to `cargoDependencyDirectionReport`, its validation in `🔍️discovery/🟦️.ts` (line 5271 ff.), the schema and fixtures (`🧪️tests/🧱️cargo-dependency-direction`), and give `cargo-plugins-no-artifacts` the target `^✏️s/🔌️plugins/[^/]+/🗿️artifacts/` instead of `toRoles artifact` plus the bare segment. That reads the rule as the audit words it ("its own artifacts") and makes the three edges legal downward edges. It weakens the facade discipline and is a policy decision for the policy owner.
3. Move both crates into the plugin's `🧩️extensions` tier (mirror of `📖️playbook/🧩️extensions/🌀️procedural`). Touches the taxonomy member-name registries (extension members are `📦️packages`, `🧪️tests` and similar today, not `🖼️semantic-ui` or `🧬️generation`), `✏️s/Cargo.toml`, nine `project.json` inputs, `📋️build-owners.json`, the profile constraint and consumption fixtures and the lib-wiring ratchet. Not attempted.

## 6. Bridges (98 rows of the audit)

Not changed. They are outside the gate by construction (see §2). If a later change admits `🏭️bridge` manifests into the Cargo inventory, the same path-based `fromOwnerPaths` pattern would flag them; the fix then is to add `🏭️bridge/` to the negative lookahead of `cargo-plugins-no-artifacts` and `role = "tool"` to the bridge manifests, because a bridge is the test domain's inventory tool that must link the production aggregates of its plugin's artifacts. No such change is needed today.


## 7. Commands run and real results

All logs are `🗑️generated/cpd-*`.

| # | Command | Result |
| --- | --- | --- |
| 1 | `bun cpd-scan.ts` (authoritative `cargoDirectionInventory` + `cargoDependencyDirectionReport`, taxonomy policy) before any edit | exit 0, `packages=2262 localDeps=31193 violations=23 problems=92`; rules `cargo-modules-no-plugins` 19, `cargo-plugins-no-artifacts` 4 (`cpd-baseline.txt`) |
| 2 | `bun cpd-scan.ts --problems` after the stdio re-role and the space core change | exit 0, `packages=2263 localDeps=31296 violations=3 problems=91`; only the three procedural-core edges remain; `unknown-role` gone (`cpd-after-1.txt`) |
| 3 | `bun repo cargo … -- check -p semio-s-space-core --lib … --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml` (first run) | exit 1: `cannot update the lock file … because --locked was passed`. The core lock was stale against the peers' framework changes (the update below adds dozens of new framework crates), not caused by this change |
| 4 | same launcher, `-- update --workspace --offline --manifest-path …/🫀️core/Cargo.toml` | attempts 1 to 3: `SQLITE_CANTOPEN`, then two 190 s alarm expiries behind the lease queue; a later attempt failed with `ENFILE: file table overflow` (machine-wide); the last attempt exit 0, lock refreshed (`cpd-space-update.log`, previous copy `cpd-space-core-lock-before-update.lock`). The lock diff is large only because of the peer drift |
| 5 | same launcher, `check -p semio-s-space-core --lib` after the update | refused: `Complete compiler descriptor inventory required` (transient), then `Insufficient physical compiler initial headroom` on every retry from 06:07 (image `/dev/disk4s2`: 4,287,512,576 B, later 4,065 MiB, required 4,294,967,296 B, retirement plan 0 units). 28 attempts until 06:50; then switched to the single-slot fleet mutex of `📓️WAVE5-BRIEF.md` with a 10 minute retry |
| 6 | stdio check (12 crates, one launcher call, `--manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml`) | queued behind 5 under the same mutex |

## 8. Status of compile evidence

No compile of a touched crate has been admitted yet (updated below if that changes). Both edits are metadata (12 `role` strings) and one import path swap, so I give static evidence and label it as such:

- Names for the space core: all nine imported names are in the explicit `pub use semio_framework_artifact_space_space::{…}` list of `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs` (lines 8 to 14). That crate is re-exported as `space` by `semio-framework-os` (`🖥️host/📦️packages/🦀️rust/🦀️.rs` line 26) and re-globbed by `pub use crate::space::*` in `🖥️host/🦀️.rs` line 19, which the lib root pulls in through `pub use host_core::*`. `semio_framework_os` is already a dependency of the core. No other glob in the host or the framework facade defines any of the nine names (searched), so there is no glob ambiguity.
- The core's lib root is the only user of the artifact crate inside `🫀️core` (searched all `.rs` and manifests), so removing the dependency leaves no dangling reference.
- `cpd-run.status` records the mutex-guarded retries: attempts at 06:52, 07:02 and 07:12 all ended in `Insufficient physical compiler initial headroom` (image still 4,065 MiB free). I stopped retrying at 07:21; the mutex is released. The two compile checks (space core `--lib`, the 12 stdio crates `--lib`) are therefore NOT run. Re-run them first when headroom is admitted: `check -p semio-s-space-core --lib --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml`, then the stdio batch with `--manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml`.

## 9. Foreign errors and hand-offs

- Build image headroom (`⚡️cache/rust/fixed/m`) blocks every compile fleet-wide (also `RESULT-COMPILE-PASS-1.md` section 5, `RESULT-COMPILE-PASS-2.md` section 6); sanctioned retirement needs Root authority.
- Lock drift: I did not refresh the other five workspaces; their locks carry only the one removed dependency line.
- Procedural core (§5): decision needed from the policy or framework owner. Recommended: option 1 (facades).
- Visible in the baseline, outside this slice: 50 `missing-role`, 41 `owner-role-mismatch`. `semio-s-artifact-norm-contract` and `semio-s-plugin-wfc-engine` still carry `s-module` under plugin paths; they violate nothing today and would trip `cargo-modules-no-plugins` as soon as they depend on a plugin-path crate. Re-role them to `library` the same way when convenient.
- No ticket tool called, no modifying git command, no sub-agent.
