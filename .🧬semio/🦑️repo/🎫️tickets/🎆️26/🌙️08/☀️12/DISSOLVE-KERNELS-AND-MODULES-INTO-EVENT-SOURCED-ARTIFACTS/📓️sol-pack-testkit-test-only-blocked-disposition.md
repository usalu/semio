# Pack Testkit Test-Only Blocked Disposition

## Read-Only Audit

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`
- Pack testkit SHA-256: `a5396de70904912a74f503386a5a3dc3e755237d1dafe25b5db6860f89060136`
- OS Rust glue SHA-256: `d2e846bf87210e7edc433ce33a6b5a973776c8051dc711815ab961d00a2a9504`
- Spr testkit SHA-256: `6fe4e25499d6852094f399a85d739c05277d9aa100ea416b9fb0e1cbe0308c5a`

`RecordValueGen`, the five law helpers, and local fixtures have zero external consumers. `CorruptionLevel`, `CorruptionReport`, `fuzz_truncation`, `fuzz_bit_flips`, and `golden_hash_hex` are consumed only by Spr and DB tests. Spr reexports these test helpers; DB glue reexports the testkit module and DB tests invoke the fuzzers. None is a production consumer under the binding consumer rule.

## Disposition

The production-module disposition is deletion, with useful fixtures inlined into their owning Spr and DB test components. The change is not currently conflict-free: preserving those tests requires the central OS glue registrar, which is externally staged from P-01. The alternative Pack `http` and `async` candidates also overlap that glue/facade and the Cargo feature surface.

P-02 remains blocked until the OS glue registrar lease is released. No source was edited and no test was run. The eventual scoped check is:

```text
bun nx run @semio-tech/framework-os-kernel:check
```
