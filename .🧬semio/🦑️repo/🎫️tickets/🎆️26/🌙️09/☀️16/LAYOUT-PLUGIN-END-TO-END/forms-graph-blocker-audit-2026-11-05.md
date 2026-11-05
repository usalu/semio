# Forms Graph Blocker Audit — 2026-11-05

Read-only inspection, no builds/Forms edits/contact with other owners.

NativeDeps retained log generated/fleet-native-dependencies-code-root-current-2026-11-04.log explicitly refuses missing old manifest:

✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📦️packages/🦀️rust/Cargo.toml

It currently does not exist. Git status marks this tracked manifest and its old config subtree deleted. The relocated manifest at any/🎚️config/📦️packages/🦀️rust/Cargo.toml exists and declares semio-s-artifact-forms-forms-config. Forms artifact root Cargo.toml:8 already points to that new location; aggregate 📦️packages/🦀️rust/Cargo.toml:26 and editor consumer Cargo.toml:47 likewise use relative paths resolving to the new location. Thus this is an active Forms Source relocation, not a missing Layout package.

Tracked 🔒️dependencies.json:6962,:8444,:10940 still declare the old full path. Exact JSON ownership locations:

- $.entries.281.users.9
- $.entries.283.users.31
- $.entries.341.users.428
- $.entries.342.users.558
- $.entries.378.users.27

These stale dependency inventory entries directly match the refused path. No clean inference-scope fix is demonstrated: silently skipping missing declared Cargo manifests would weaken ownership. The clean correction belongs to Forms relocation/dependency inventory convergence through its existing owner, preserving canonical lock/sync semantics; this lane did not edit inventory or launch regeneration. Current graph readiness is not proven until that declaration settles and actual owner admission reruns.

Related local ownership evidence: sibling ticket ../FORMS-PLUGIN-END-TO-END exists; prior authoritative doctor identified PID92700 as its Vitest config invocation. That is historical activity, not a current owner identity or completion claim. No other agents contacted. Fresh private witness73695 is parent-reported and may postdate the previous fleet snapshot; no assertion about its liveness from that old snapshot.

## Baseline Structure and Reconciliation Owner

Correction: FIVE exact stale users coordinates, not three (earlier line list was partial): /entries/281/users/9 calamine; /entries/283/users/31 csv; /entries/341/users/428 serde; /entries/342/users/558 serde_json; /entries/378/users/27 zip. Entries are direct ecosystem/name/version/kinds/users/productionReachable plus optional oracleIds. There are no per-entry/user digests. Top-level schemaVersion2/generatedAt/commit are baseline provenance, not a digest of these coordinates.

Canonical whole-baseline writer: library/🕸️dependencies/📇️inventory/🟦️.ts:975–980, dependencyFreezeWriteBaseline, computes current third-party entries then writes complete baseline and fresh timestamp/HEAD. Registered target 📋️project.json:4203 is repo:verify-dependencies-freeze-write-baseline, command repo ./📜️script.ts verify dependencies write-baseline. No per-package/coordinate option is visible at this writer. This is distinct from dependencies lock/sync, which reconciles Bun installation locks/generations.

For a narrow authorized declarative relocation, recommend exactly five scalar users-coordinate replacements, preserving all dependency identity/version/kinds/oracleIds/reachability and unrelated bytes/provenance. Avoid broad baseline rewrite because it can enroll unrelated inventory changes. Existing taxonomy sourceDisposition authored-policy-state and preserveNonCoordinateBytes true at library/🔍️discovery/🚏️audit-projection-and-package-contracts/🟦️.ts:126 demonstrates this distinction for its separately closed jcoprobe-guest projection; that specific projector does NOT authorize Forms or provide a Forms command. No scoped owned Forms reconciliation command was found, so do not claim one. No edits performed.
