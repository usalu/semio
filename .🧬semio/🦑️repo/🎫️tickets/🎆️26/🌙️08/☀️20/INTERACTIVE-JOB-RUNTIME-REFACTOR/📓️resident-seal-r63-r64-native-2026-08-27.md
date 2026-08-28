# Root-Associated Finalization — R63–R64

R63 was the expected compile-first RED: six E0599 diagnostics for missing `shrink_resident`, `split_resident_output`, and read-only `resident_limits`; no native test executed. Raw: `🧪️member-ui-resident-seal-red-r63-native-2026-08-27.txt`.

R64 command: `bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='--lib retained_document_root_permit_ -- --nocapture'`.

```text
[DEBUG] fixed-list-page-oracle checks=75
[DEBUG] document-root-output shrink-before-split=true output-first-and-root-first=true exact-final-return=32768
Summary [0.104s] 5 tests run: 5 passed, 150 skipped
NX Successfully ran target test for project @semio-tech/ui-contract-rs
```

Exit 0; raw `🧪️member-ui-resident-seal-green-r64-native-2026-08-27.txt`. Finalization methods mutate the exact permit while it remains inside its canonical root. A split produces only output owner 2; both return orders preserve the entire reservation until the final root/output obligation retires. Zero grants change neither credit nor output, and shrink after split is refused.

The shrink guard checks the root's actual node-table backing and fixed slot metadata minimum. It does not compute the complete simultaneous runtime owner census: that census is still the caller's required prerequisite and remains open in the runtime cutover. This gate does not prove Process fit or full renderer lifecycle.
