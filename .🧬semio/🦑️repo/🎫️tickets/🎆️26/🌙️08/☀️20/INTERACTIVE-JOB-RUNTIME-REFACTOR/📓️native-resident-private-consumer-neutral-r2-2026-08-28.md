# Native Resident Private Consumer Neutral R2

## Actual Result

The existing command `bun x nx run @semio-tech/value-resident:test` exited 0 on 2026-08-28. It ran the shared TS schema/implementation controller, including the added native-neutral Immer phase3/cancellation4 model and prior trace7, plus the peer's liveRecord7 and the full existing shared ledger tests. This is not native Rust, allocation, thread or destructor evidence. No Cargo/native command ran in this lane.

The test was dispatched after the peer explicitly released its R60 source hold. Native changes were restricted to the existing admission nativeOwnership fields and the native-neutral controller region. Shared runtime prices and capacity axes were not changed.

## Captured Tool Output

ANSI presentation escapes are omitted; message text is preserved.

```text
> nx run @semio-tech/value-resident:test

> bun ./📜️script.ts test

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

[DEBUG] Native ownership neutralTrace=7 phaseAccess=3 cancellationFrontiers=4 sealedReplacementRefused=true oracle=Ajv+Immer actualNativeExecution=false unknownFaultFinalDisposal=false
[DEBUG] Resident liveRecord=7 identityOnlyDistinct=true oracle=Ajv+Immer
[DEBUG] Resident capacity=6 actualOverflow=2 ownerReader=1 partialExtent=4 simultaneousRawUiScratch=1 postedCancel=1 unsubmittedCancel=1 transferredViewFault=1 controlAxes=3 childClose=5 childFault=2 privateDispatch=5 quarantine=11 domainRecord=1 recordOverflow=3 finalizerFrontiers=8 admissionFailures=5 admissionBootstrap=7 firstFault=4 resourceWrapper=5 terminalAliasDetach=1 strictTS=0 oracle=Ajv+Immer+Buffer+BigInt

 NX   Successfully ran target test for project @semio-tech/value-resident
```

The command returned session 88238 initially and terminal exit_code=0 on polling. No passing Rust stdout or native Layout measurement was produced.

## Post-Run Source Census

These hashes were read after the terminal; no pre-run hash-pair stability claim is made.

| Resident-relative path | SHA-256 |
| --- | --- |
| `🦀️.rs` | `508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f` |
| `🧪️tests/🦀️.rs` | `ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e` |
| `📨️admission/🧬️contract.json` | `c4655f43d54524f15015a753e2e9441c04d63b738601f6ebd3a63eec27a74238` |
| `📨️admission/🧬️schema.json` | `42a213e71a8be05b8b9e9784f53525ba319a2256c1d9b21318ec9e300a1dab37` |
| `📨️admission/🧪️fixture.json` | `8df81492f42dfa1232a718e917149b209d7151a72d5bea397f354091290f55ad` |
| `📨️admission/🧪️schema.json` | `3d9b729ec2fef59a179ce4425a7d1c0554c5937d19512065f3bf760568640b6a` |
| `📜️script.ts` | `50793dbcbf2d873e8391faebfe436322470840a2db5d4e584b95032838f89ab3` |

## Remaining Boundary

The 17-test Rust candidate remains uncompiled at this report. Exact private registered-parent funding, original enclosing-root ownership under loss/poison, actual Opening constructor integration and callback-tail completion are open. The shared neutral model does not supply those authorities.
