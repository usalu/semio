# Native Admission Neutral Packet R1

Command: `bun x nx run @semio-tech/value-resident:test`.

Actual exit0. Strict Ajv accepted the updated resident/admission schemas and fixtures; the permanent Immer model reproduced all seven native-ownership trace rows. The existing full shared TypeScript resident gate also passed, including strictTS=0. This is schema/model/TypeScript evidence only; Rust admission implementation is absent, and its five new tests await missing-API RED through the sole compiler. The model does not measure allocation, destructor timing, or unknown-fault final disposal.

The following is the complete tool output, with terminal color sequences removed for readability; no omitted success/failure text.

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

[DEBUG] Native ownership neutralTrace=7 oracle=Ajv+Immer actualNativeExecution=false unknownFaultFinalDisposal=false
[DEBUG] Resident capacity=6 actualOverflow=2 ownerReader=1 partialExtent=4 simultaneousRawUiScratch=1 postedCancel=1 unsubmittedCancel=1 transferredViewFault=1 controlAxes=3 childClose=5 childFault=2 privateDispatch=5 quarantine=11 domainRecord=1 recordOverflow=3 finalizerFrontiers=8 admissionFailures=5 admissionBootstrap=7 firstFault=4 resourceWrapper=5 terminalAliasDetach=1 strictTS=0 oracle=Ajv+Immer+Buffer+BigInt



 NX   Successfully ran target test for project @semio-tech/value-resident



```

