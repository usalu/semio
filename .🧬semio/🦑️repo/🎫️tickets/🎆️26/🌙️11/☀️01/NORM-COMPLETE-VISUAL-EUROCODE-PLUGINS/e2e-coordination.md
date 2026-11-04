# 🧭 Norm Plugins End to End

Date: 2026-11-04. Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` was already open (`ticket_reopen` returned `ticket is already open`). Goals read from the native `repo goal list`: `🎯norm` and `🎯runningnorm` (Norm loads in s and exposes EN 1990–EN 1999, DIN, ISO 16757, VDI 3805).

## What “working” means

1. Each Eurocode artifact compiles and its check and test targets pass through `repo`.
2. A user can load the norm plugin, edit one EN 1990 example, and see a check result.
3. Clause-complete NDP verification against printed standards stays a later gate. Status matrix 6 (2026-11-03) already records 2802 of 3060 NDP rows unverified.

## Evidence already in tree

- Status matrix 6: no EN family has a green build, check, or test. Newest logs are stale (OOM, cancelled graph-plan, or pre-rustc). Shared `norm-artifact-contract-rs:build` was green at 01:42 on 2026-11-03.
- `En1990Snapshot` and `En1990Snapshot::example` and `road-bridge-compliant` now set `factor_overrides` and `traffic_groups`. The run-14 `E0063` is not still missing on those constructors.
- `semio-s-plugin-norm-ndp-en1990` has one Cargo.toml and one `build-owners.json` row. The duplicate table path from en1999 run 11 is not present as a second package name.

## Unblock applied 2026-11-04

`repo run norm-en1990-rs:check --dry-run` failed before any norm crate compiled.

1. `build-owners.json` had four roots claimed twice. Removed the names that disagree with the manifest: `@semio-tech/replication-mutation-retained-field-rs`, `@semio-tech/replication-mutation-retained-rs`, `semio-s-artifact-stdio-ifc-2x3-base-catalog`, `semio-s-artifact-stdio-ifc-4-catalog`. Kept the names in `project.json` and `Cargo.toml`. Recheck: 4609 owners, 0 duplicate names, 0 duplicate roots.
2. Build source admission then required `@semio-tech/assets` to be a private workspace package because `ui/styling` projection imports `@semio-tech/assets/resolver/delivery`. Set `"private": true` on `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/package.json`.

The follow-up dry-run used the wrong project id `norm-en1990-rs`. The declared project is `@semio-tech/norm-en1990-rs` (`…/⚖️en1990/📦️packages/🦀️rust/📋️project.json`). That run was stopped while it held `dependencies:store-stage`. Host load is `@semio-tech/norm-plugin` and `repo run @semio-tech/framework-os-dev:dev-en1990-react-prototype`, then `http://127.0.0.1:6094/?plugin=en1990`.

EN 1990 still cannot compile until `ReliabilityAnalysis` and `TestSeries` match the scene: `second_action_*`, `member_id`, `pairs`, `theory_variation`, `theory_mean`. JSON already has `memberId`, `pairs`, and `theoryVariation`. Rust, GraphQL, proto, and TypeScript do not.

## Outliner descriptor

`en1990-outliner-manifest.md`: the editor registers `norm-en1990-outliner`. Both hub twins omit it. `describe()` owns the pair, so it is not hand-edited. Command: `repo run workspace:components describe norm`. Schema alignment is in `en1990-schema-align.md`. `:build` is the target that has gone green here; `:check` is held until that ingress patch is validated.

Storybook freeze is cleared. `repo dependencies lock` exited 0 for `✏️s`, `♻️mit-bestand`, `🌎️hub`, `🎓️teaching`, and `🏢️semio-tech`. Every `bun.lock` pins `storybook` `10.6.5`. No package manifest still asks for `^10.4`.

`@semio-tech/norm-en1990-rs:build` is not restarted yet. `activate-s-react-prototype` is the live graph. Its log restarted once because another process changed bun workspace ownership during the read (`activate-s.log`, store dependency sync incomplete, retry 1 of 6). The child is in source admission. Do not edit workspace files while that read is open. Two stopped `framework-os-mcp-rs:build` processes do not hold a lock. Swap is full.

The EN 1990 sqlite admission called `.len()` on `PermanentKind`, `DistributionKind`, and `TestDistribution`. Those calls now use `as_str().len()`, and the same loop admits `member_id`, `second_action_distribution`, and test `pairs`.

## S1 leftovers (2026-11-04)

The cited S1 defects are already in source except these, which are now edited:

- EN 1997 `rw_01` and `rw_02` use `atan(tan φ / γ_φ)`, and Rankine cohesion is divided by `γ_c`. The Python catalogue oracle matches. Default DA2 uses M1 (`γ_φ = 1`), so the pinned EX97-11 numbers stay.
- Footing sliding passes that design angle into `passive_earth_force_n`.
- EN 1992 `flexural_resistance_nm` compares `N_Ed` in newtons with the section force. The `/ 1e3` on `N_Ed` is removed. The moment return stays in N·m.

## Host

2026-11-04 06:15: about 7.9 GB spare above the floor, load 11, swap about 8.6 GB. Four fleet rows are 11-hour leases whose PIDs are gone. Do not start ten parallel Nx graphs. First proof is one serial `norm-en1990-rs:check`.

## Fleet

Main coordination stays in this chat. Execution agents use `grok-4.7-medium`. A Grok 4.7 Light Non-Fast model is not in the session catalog, so read-only exploration also runs on `grok-4.7-medium` with the explore agent type.
