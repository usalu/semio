# Baseline Verification

## 2026-08-15 — stdio quick suite before production edits

Command:

```text
bun nx run @semio-tech/stdio-plugin:test-quick
```

Result: failed before reaching GLTF tests. The fail-fast run executed 43 of 3,417 tests: 42 passed, 1 failed, 10 skipped, and 3,374 were not run. The failing test was the unrelated BCF 2.1 fixture honesty law:

```text
artifacts::bcf::standards::v2_1::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
```

The failure is a byte comparison between two differently encoded BCF zip payloads. No GLTF production path had been changed when this baseline was captured.

## 2026-08-15 — focused GLTF baseline before production edits

Command:

```text
bun nx run @semio-tech/stdio-plugin:test-quick -- gltf
```

Result: passed. All 60 GLTF-filtered tests passed; 3,367 unrelated tests were skipped. This is the comparison baseline for subsequent focused gates.
