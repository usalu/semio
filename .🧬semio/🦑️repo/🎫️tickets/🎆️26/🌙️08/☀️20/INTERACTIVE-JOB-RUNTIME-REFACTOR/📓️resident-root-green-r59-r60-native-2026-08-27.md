# Canonical Resident Root Binding — Native R59–R60

The document arena now uses the shared resident ledger's 64 slot identities. A document is installed only at its affine permit's exact slot and epoch; the old independent eight-slot allocation choice is gone. `open_with_permit` moves root owner 1 into the canonical slot before payload allocation. Typed retirement closes that permit only after aliases, node descendants, surface bytes, and scalar fields have retired. Ledger contention leaves the root nonterminal. All cold document constructors also reserve from the same aggregate rather than bypassing it; their convenience reservation is the existing 8 MiB surface ceiling.

Both commands used `bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='--lib retained_document_root_permit_ -- --nocapture'` and exited 0.

R59 actual output:

```text
[DEBUG] document-root-reader grants=1,64,4096 credit-until-final=true typed-descendants-before-credit=true
[DEBUG] document-root-permit actual-surfaces=9 exact-bytes=589824 final-credit=0
Summary [0.095s] 3 tests run: 3 passed, 150 skipped
```

R60 added aggregate pressure and exact slot/epoch retry:

```text
[DEBUG] document-root-pressure aggregate=33554432 captured-reader-keeps-credit=true exact-slot-epoch-retry=true
Summary [0.047s] 4 tests run: 4 passed, 150 skipped
NX Successfully ran target test for project @semio-tech/ui-contract-rs
```

Nine actual canonical roots contain readable typed payloads and use 589,824 bytes of reservation credit, not a claim that their physical payloads occupy that many bytes. The pressure case holds four full-ceiling reservations, preserves one through a captured reader, refuses a fifth reservation, then verifies that a reused reservation epoch cannot install over the previous document's still-pending final scalar retirement. The same exact refused permit and surface succeed after that terminal step. Cancellation under a held resident ledger retains the candidate until it can return credit.

Raw logs: `🧪️member-ui-resident-root-green-r59-native-2026-08-27.txt`, `🧪️member-ui-resident-root-green-r60-native-2026-08-27.txt`. R60's source oracle ran before the subsequent pressure-field fixture assertions were added; the next full UI gate validates the expanded 72-check schema/oracle.

These are canonical document/permit laws, not nine live app surfaces through the reconciler. Runtime replacement of the old retained tree, full simultaneous-owner census, and original R30/R31 remain open. No larger grant, Process-fit, timing, or fresh consumed-Wasm claim follows.
