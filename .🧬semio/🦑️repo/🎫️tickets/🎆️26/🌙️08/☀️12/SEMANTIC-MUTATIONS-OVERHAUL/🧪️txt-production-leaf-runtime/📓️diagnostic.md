# TXT Production Leaf Runtime Diagnostic

This retained ticket-only harness mounts the actual production TXT snapshot, sparse diff, direct mutations, leaf codecs and root codec registries. It does not duplicate their algorithms. The module wiring mirrors the inspected STDIO glue with unrelated artifact families omitted.

The diagnostic links the already-built native checkpoint dependencies named in the current STDIO compiler invocation. Those exact library names intentionally identify this local checkpoint, not a permanent cross-platform development command. The registered Bun/Nx STDIO target remains the required full integration gate. A before/after source fingerprint rejects edits during the diagnostic compilation; all generated fixture/compiler/test output is retained inside this ticket directory.

Execution command once the correction sources are stable:

```text
bun <ticket>/🧪️txt-production-leaf-runtime/📜️script.ts
```

Compilation has a bounded180-second timeout; test execution has a60-second timeout. The harness prints the exact compiler command, retained paths, statuses and source fingerprints.

## Executed Result

The first invocation failed with five metadata-stub errors because this checkpoint builds `.rlib` files without embedded metadata. The harness was corrected to pass each matching `.rmeta` alongside its `.rlib`, as the inspected Cargo invocation already does. No production source was changed for that correction.

The second invocation compiled successfully and executed28 tests:28 passed, zero failures/ignored/filtered tests, runtime0.06seconds. This includes the432-case native-carrier predicate matrix, direct inverse restoration, one-line removal to an empty document, strict payload rejection, malformed codec frames, sparse-diff algebra and grammar laws. Log: `🧪️metadata-retry.log`; retained source/compiler/test artifacts: `🧫️run-4DJs55`.

Production source fingerprint before and after compilation was identical:

```text
a34d91f9c19f9cd57b33b82ce7e6f81b4444b82c4ed6f0101637182d06683dc4
```

This proves the scoped mounted production TXT runtime against the checkpoint dependencies. It does not prove the complete STDIO package, other owners, the current taxonomy filename cutover, or the still-open mandatory descriptor/derive contract.
