# Current Native Runtime Verification

The correct current dashboard build entrypoint is `bun repo run @semio-tech/repo-dashboard-rs:build --exclude-dependencies --skip-cache --output=stream`. Current mode-specific version-3 installation records are absent; the older version-2 executable is not accepted as proof of current runtime.

The first attempt found duplicated declarations in the current build entry. The obsolete duplicated invocation body was removed while retaining the newer preflight and dependency-store path. The actual next invocation reached native storage, created `.cache/b/l/fixed.vhd` (66,060,800 bytes, creation 2026-10-19 03:05:36 local), and failed `Bounded ordinary exact creator handle required`. A second invocation refused its incomplete state because `.cache/b/l/image.json` is absent.

Read-only Windows observations confirm the VHD is not attached, has Archive attributes, one link, exact footer size and bounded allocation; its `m` directory is empty. Re-executing the same sequential allocation action in a ticket-scoped bounded fixture succeeded and returned coherent before/after/named descriptor facts. The production failure therefore remains a real unproven runtime issue, not permission to weaken descriptor custody.

The attempted recovery verified the absolute literal cache paths, current creation timestamp, detached VHD status, exact size, absence of receipt and empty mount directory; it would remove only the failed image and its two empty directories without recursion. Automatic approval review rejected the command before execution with `blocked by policy`. No alternative tool or route was used to bypass that rejection. This describes the earlier failed allocation; the subsequent shared-checkout change is recorded below.

## Subsequent Normal Build Retry

A later read-only check found that `.cache/b/l` and `.cache/b/m`, including the failed `fixed.vhd`, no longer existed. This task did not remove them through another tool. The responsible concurrent change is not attributed. The mode-specific prototype installation and image receipt were still absent.

The same normal admitted dashboard build was therefore retried, with its output retained at `🗑️generated/native-build-retry.log`. It exited 1 before cache creation because the current Windows process token lacks the required `SeManageVolumePrivilege`. The current implementation reports that requirement explicitly and does not request elevation. No token/account configuration, elevation, alternate cache path, alternate publication or storage bypass was attempted.

The current runtime blocker is this host privilege prerequisite. The earlier automatic cleanup rejection remains part of the execution record, but the missing failed image is no longer the immediate cause. Whole-dashboard compilation, current installation, native dry-run resolution, terminal interaction, readiness, stop/descendant behavior and native retained-log verification remain outstanding.

A frozen Bun dependency synchronization with scripts disabled completed successfully (1,395 packages, 54.44 seconds). The repository intentionally uses the isolated linker; test/source imports requiring ambient root workspace aliases must name their public package entry surfaces explicitly, not add hoisted undeclared dependencies. The lock file was not intentionally changed.

Runtime completion is unproven. The build/install, live service/readiness/cancellation/log and current PTY journeys must still execute successfully after legitimate cache recovery and any native failure repair. Focused static and independent schema tests establish only their specific contracts. Logs and diagnostic generated data stay under this ticket's generated directory.
