# Verification Routing and Launcher Warning Review

## Confirmed Registered Sequence

After setup prerequisites and the shared graph generation settle:

1. SEMIO_LANGUAGE=en repo preflight
2. SEMIO_LANGUAGE=en repo run workspace:setup
3. SEMIO_LANGUAGE=en repo run workspace:test-workspace-build
4. SEMIO_LANGUAGE=en repo run repo-mcp:test-install
5. SEMIO_LANGUAGE=en repo run workspace:verify-build-owners-sync
6. SEMIO_LANGUAGE=en repo run workspace:verify-zero-touch
7. SEMIO_LANGUAGE=en repo run workspace:build
8. SEMIO_LANGUAGE=en repo run workspace:test -- long
9. SEMIO_LANGUAGE=en repo run workspace:lint

These are verified declarations, not executed successes. Root build aggregates all project build targets except workspace plus Storybook. Root test aggregates all project test targets except workspace/repo-test-domain, separately includes domain test-contract, then RootTestScript; long adds Storybook browser run. Specific named native/e2e/oracle targets outside those aggregate closures still need resolved graph coverage checks.

## Registration Boundary

BuildTestScript in library/🏗️build/📜️script.ts81 uses an explicit owned Bun test file list. It does not include budget/🧪️tests/🟦️.ts or container runtime-bootstrap law. Thus workspace:test-workspace-build passing does not by itself establish those runtime suites. repo-mcp:test-install is correctly registered and runs installation, Go-layout and Go-lease laws through runRepositoryTestCommand.

RootTestScript delegates generic selections to TestScript, which refuses arbitrary extra arguments outside its declared phases; supplying a raw Bun file to workspace:test is unsupported. To execute budget/runtime-container law directly, add/reuse an explicit owned test target or an existing authenticated runRepositoryTestCommand route; do not assume bare repo tools exec bun creates the original storage authority those runtime cases require. The actual full resolved project graph must reveal which owning test target registers these suites before claiming comprehensive execution.

Prior platform audit established both .NET projects portable net8.0, CMake host guards and Cargo Metal/D3D/Vulkan target predicates. No platform exclusion changes are justified by these generic aggregates alone. Confirm resolved target behavior rather than filtering names containing windows (many mean UI/architectural windows).

## Launcher Duplicate Warning Defect

Launcher Unix installer 📥️install/🐚️.sh188–205 physically resolves each executable PATH/repo; every different physical file is called a duplicate and the warning suggests ln -sf installed-launcher over that entry. A valid separately installed native frontend at ~/.local/bin/repo is physically distinct by design and therefore triggers this warning. Repeated PATH entries also repeat warnings. This is misleading/destructive advice, though the installer only prints it and does not execute the symlink command.

Installation schema already owns per-user receipt roots and Receipt shape; native installed.json identifies exact executable, directory, digest and platform. The clean discriminator is an authenticated matching owned frontend receipt/digest or canonical per-user installation identity, not equality with launcher bytes. Do not suppress genuinely foreign executable shadowing solely by filename/home path. Add neutral cases for launcher itself, symlink to launcher, matching native receipt, mismatching digest and foreign repo, plus PATH deduplication. Preserve user-native frontend rather than replacing it with the launcher. No source edits by audit.
