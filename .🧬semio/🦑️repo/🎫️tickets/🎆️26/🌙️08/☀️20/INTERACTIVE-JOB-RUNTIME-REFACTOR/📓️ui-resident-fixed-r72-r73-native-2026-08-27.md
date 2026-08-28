# Shared Resident Fixed Backing: Native RED and GREEN

R72 canonical exhaustive exact `retained_resident_fixed_backing_` selector: exit 1, 0 passed, 1 failed, 158 skipped, 0.022s. The existing ledger began with zero bytes despite its retained fixed arenas; the assertion `actual.bytes > 0 == true` failed. Raw: `🧪️member-ui-resident-fixed-red-r72-native-2026-08-27.txt`.

R73 canonical exhaustive `retained_resident_` selector: exit 0, 4 passed, 155 skipped, 0.484s. Raw: `🧪️member-ui-resident-fixed-green-r73-native-2026-08-27.txt`.

```text
[DEBUG] resident-fixed contract=390800 runtime=125088 total=515888 dynamic-slots=64 final-release-excludes-static=true
[DEBUG] resident-permit small=9 slots=64 aggregate=33554432 paired-return=0,65536 explicit-close-drop-does-not-return-again=true
Summary [0.484s] 4 tests run: 4 passed, 155 skipped
```

The contract baseline is derived from the actual resident ledger and return array, 64-slot canonical document arena and its handback storage, and fixed UiValue page/collection/free-index backing and handbacks. Fixed storage is reserved once against the same 32MiB aggregate; no dynamic slot is consumed. A runtime may register its exact fixed domain footprint once. Repeating the same footprint does not charge twice; a different footprint is rejected; zero grant and held-ledger access preserve state. Final paired-root return subtracts only that root's dynamic permit, never static bytes.

The 125,088 runtime bytes in the neutral arithmetic/native primitive are the measured output pool. The live runtime registration also includes its existing handback registry, whose combined amount is a subsequent runtime gate. This is not a claim that the native primitive total equals the full live runtime total.

Full-capacity tests now reserve the actual remaining aggregate bytes: three full 8MiB permits and a fourth reduced by the fixed baseline. They still assert exactly 32MiB resident bytes and refuse the next byte; all 64 dynamic slots remain available with small permits. Root-reader pressure uses the same actual remaining-byte equation. No ceiling was raised.

The new domain fixture/schema is validated by strict Ajv; Node Buffer unsigned arithmetic independently checks fixed + runtime + payload, then final payload release. This synthetic arithmetic vector is not a replacement for the actual native layout calculation. Full UI/runtime regression and complete dynamic ownership census remain separate gates.
