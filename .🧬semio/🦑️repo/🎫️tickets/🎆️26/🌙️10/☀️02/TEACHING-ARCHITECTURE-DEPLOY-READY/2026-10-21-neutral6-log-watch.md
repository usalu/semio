# Neutral 6 Read-Only Log Watch

Expected manifest SHA-256 `262a2e8969066adb32158208d2746d70cebc1e0e67dd8251a1a2e3b1b2181019`; seventeen phases. Sole executor owns actual terminal polling. This auditor reads bounded log extents only; no Source edits, function/test execution or terminal control. Initial zero/missing log would establish no job-status claim.

Root relayed executor confirmation: actual sole-owned TTY `62305` launched after immediate full precheck at 23:58:19.472 UTC. Executor alone polls that handle.

## First Bounded Startup Observation

At 2026-10-21T00:08:22.262Z, observed log device 16777229/inode 454629794, 16325 bytes; cumulative examined sequential extent [0,16325). Each read was bounded to 32768 bytes and spaced at least thirty seconds. All examined content so far is capacity admission waiting: need 3584 MB, one running/two ahead, chiefly load above ceiling. No combined phase start/completion, pass/fail collection or receipt marker occurred in the examined extent (counts zero). This establishes startup-stage log evidence only, not terminal status. Latest exact row:

```text
[capacity] waiting 597s for exec: not enough free memory (need 3584 MB; 1910 MB spare above the 6553 MB floor, load 26.01; 1 running, 2 ahead)
```

## Capacity Admission Transition

At 00:25:03 UTC, sequential examined extent reached [0,43649). New exact startup records:

```text
[capacity] admitted exec after 1597s although the load is above its ceiling: nothing of ours is running and memory holds
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
```

This establishes native startup progressed past capacity waiting, not combined phase collection or terminal success.

At 00:30:22 UTC, examined sequential extent reached [0,1683077) using bounded 32 KiB chunks (at most 1 MiB new extent per check). Startup traversed graph-cache inventory/retirement and reached graph inference 3/3736 plus build:command-source-imports. No phase, receipt, concrete error or graph-settle marker encountered in examined content. No actual terminal inference.

## Intentional-Owned Interruption Evidence

Root reported assembly pin revocation and instructed only the sole executor to settle its own live invocation. At 00:36:19 UTC, log size and fully examined sequential extent reached 2279902 bytes. Last graph inference was 405/3736, then build:project-assembly. No combined phase start/completion, pass/fail collection, receipt or graph-settle marker encountered in examined [0,2279902). Concrete log:

```text
error: Original Script process interrupted
      at interrupt (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:188:191)

Bun v1.3.14 (macOS arm64)
```

Actual exit code is not inferred from this log; executor terminal confirmation remains required.

## Owner-Confirmed Terminal

Root relayed sole-executor confirmation: actual TTY `62305` exited 1 at 00:35:33 UTC after intentional cancellation of that exact owned invocation. Final log 2279902 bytes/SHA-256 `e61944acac977407df5337069b5e53bf21da404f043fa59b04d20aeb03fc47af`, device 16777229/inode 454629794, modified 00:35:31.524 UTC. The fully examined final extent contains zero combined phase/receipt/pass-fail/graph-settle markers. No live native job remains. Watch complete; no terminal access or Source modification performed by auditor.
