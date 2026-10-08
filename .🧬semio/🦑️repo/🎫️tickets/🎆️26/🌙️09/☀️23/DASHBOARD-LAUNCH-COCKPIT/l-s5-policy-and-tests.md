# L-S5: Policy Re-Homing And Remaining Launch Tests

Slice L-S5 of ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`. Working log; the final sections are written last.

## Working log

- A1 worker-cell: target env added, verb resolves against the workspace root, test re-expressed; `bun test` 5 pass / 0 fail; verb run from the package directory with the declared relative value wrote into the repo cache.
- A2 hub: credential-source-order (launch -> project), foundation-source test+fixture+inputs, socket-grant test+fixture+inputs, hub script (isolation proof on manifest, command roots resolved against workspace, admin dev target), hub manifest env + `stdio-only` configuration. foundation-source 12 pass, socket-grant 7 pass, provider `--oracle-only` reaches `proveVcsNativeProviderSelectionFixture` then pre-existing red `vcs-native-provider-selection/unlinked package: contract stage rejected unlinked-package`; probe `l-s5-headless-isolation-probe.ts` real manifest isolated, hostile 9/9 denied; three negative root cases throw. NOTE: foundation-source test was truncated by a non-unique anchor and restored from `git show HEAD:` (identical to index), then re-edited.
- A3 wfc: fixture `owningLaunchCommands`/`owningSourceLaunchCommands` -> `owningSqliteTargets`/`owningSourceTargets`; test 8 pass; tsc strict exit 0; owner verb `test-snapshot-sqlite source` 8 pass.
- A3 energy: capability test re-expressed; 2 pass; tsc strict exit 0; owner verb source mode: 11 pass 1 fail without `SEMIO_TEST_ARTIFACT_DIR` (pre-existing: sibling suite `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:38` throws `ticket output directory missing`, no launch row ever supplied it), 12 pass with a caller directory.
