# Reference Oracle Classification — 2026-09-12

The syn Rust executable and its Cargo manifest are independent comparator code and belong together in the library's canonical oracles. Flow VCS's serialized expected operation history is static test data and receives a descriptive fixture case name.

## Exact Paths and Pre-Move Hashes

```json
{
  "moves": [
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/⚙️.toml",
      "sha256": "028b6d8e393f26bef506d164406d9f0dc294c28726bd4597b52b47d1819f74cf"
    },
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🦀️.rs",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/🦀️.rs",
      "sha256": "279838effa048bb124223db4f980ebd415e6d344a26c59d4643d366325bdeaed"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧫️fixtures/🔮️oracle/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧫️fixtures/🧾️semantic-history/🔣️.json",
      "sha256": "b2d080b2b9889457daf9c9130ea8559ecf6d06669af47f22a6714f4ece777249"
    }
  ],
  "consumers": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/↪️rust-divergence-callback/🔣️.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs"
  ]
}
```

## baseline Runtime

```json
{
  "args": [
    "bun",
    "test",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts",
    "--test-name-pattern",
    "independent syn (parsing|callback)",
    "--timeout",
    "180000"
  ],
  "status": 1,
  "tail": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:\n(pass) independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts [9168.19ms]\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:\n183 | \n184 | test(\"independent syn callback AST, spans, free variables and target tuples match the neutral contract\", async () => {\n185 |   const started = performance.now(), owner = runOwner(\"syn-120s-cold\"), manifest = join(owner, \"Cargo.toml\"), source = join(owner, \"../↪️rust-divergence-callback/🦀️.rs\"), target = join(owner, \"🏗️target\");\n186 |   expect(existsSync(target)).toBe(false);\n187 |   writeFileSync(manifest, readFileSync(join(root, vector.oracle.manifestInput)), { flag: \"wx\" });\n188 |   writeFileSync(source, readFileSync(join(root, vector.oracle.sourceInput)), { flag: \"wx\" });\n        ^\nENOENT: no such file or directory, open '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/↪️rust-divergence-callback/🦀️.rs'\n    path: \"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/↪️rust-divergence-callback/🦀️.rs\",\n syscall: \"open\",\n   errno: -2,\n    code: \"ENOENT\"\n\n      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:188:3)\n(fail) independent syn callback AST, spans, free variables and target tuples match the neutral contract [1.74ms]\n\n 1 pass\n 78 filtered out\n 1 fail\n 3 expect() calls\nRan 2 tests across 2 files. [9.51s]\n"
}
```

## Baseline Consumer Repair

The physical reference syn comparison passed. The callback comparison failed before relocation because its compiler source path incorrectly escaped the owned run and its executable path referenced JSON fixture coordinates. Corrected generated compiler source/binary names and moved runtime allocation from a hardcoded historical ticket to the platform temporary directory (the task runner sets this inside current ticket generated output). Its test implementation remains 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts.

# Reference Oracle Classification — 2026-09-12

The syn Rust executable and its Cargo manifest are independent comparator code and belong together in the library's canonical oracles. Flow VCS's serialized expected operation history is static test data and receives a descriptive fixture case name.

## Exact Paths and Pre-Move Hashes

```json
{
  "moves": [
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🦀️.rs",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/↪️rust-divergence-callback/🦀️.rs",
      "sha256": "abf99f985ec6327ec44483c60765997c37122f6c2cf4545daabd429674301e1b"
    },
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/⚙️.toml",
      "sha256": "028b6d8e393f26bef506d164406d9f0dc294c28726bd4597b52b47d1819f74cf"
    },
    {
      "oldPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🦀️.rs",
      "newPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/🦀️.rs",
      "sha256": "279838effa048bb124223db4f980ebd415e6d344a26c59d4643d366325bdeaed"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧫️fixtures/🔮️oracle/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧫️fixtures/🧾️semantic-history/🔣️.json",
      "sha256": "b2d080b2b9889457daf9c9130ea8559ecf6d06669af47f22a6714f4ece777249"
    }
  ],
  "consumers": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/↪️rust-divergence-callback/🔣️.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs"
  ]
}
```

## baseline Runtime

```json
{
  "args": [
    "bun",
    "test",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts",
    "--test-name-pattern",
    "independent syn (parsing|callback)",
    "--timeout",
    "180000"
  ],
  "status": 0,
  "tail": "bun test v1.3.14 (0d9b296a)\n[DEBUG] cold syn compiler PID 55427, explicit remaining compiler budget 117975ms\n[DEBUG] independent syn callback AST/span/free-variable oracle matched 38 closed vectors\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:\n(pass) independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts [1392.49ms]\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:\n(pass) independent syn callback AST, spans, free variables and target tuples match the neutral contract [4172.65ms]\n\n 2 pass\n 78 filtered out\n 0 fail\n 9 expect() calls\nRan 2 tests across 2 files. [6.89s]\nld\"), manifest = join(owner, \"Cargo.toml\"), source = join(owner, \"../↪️rust-divergence-callback/🦀️.rs\"), target = join(owner, \"🏗️target\");\n186 |   expect(existsSync(target)).toBe(false);\n187 |   writeFileSync(manifest, readFileSync(join(root, vector.oracle.manifestInput)), { flag: \"wx\" });\n188 |   writeFileSync(source, readFileSync(join(root, vector.oracle.sourceInput)), { flag: \"wx\" });\n        ^\nENOENT: no such file or directory, open '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/↪️rust-divergence-callback/🦀️.rs'\n    path: \"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-rust-reference-diagnostics/🧭️divergence-callback/🧾️runs/↪️rust-divergence-callback/🦀️.rs\",\n syscall: \"open\",\n   errno: -2,\n    code: \"ENOENT\"\n\n      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:188:3)\n(fail) independent syn callback AST, spans, free variables and target tuples match the neutral contract [1.74ms]\n\n 1 pass\n 78 filtered out\n 1 fail\n 3 expect() calls\nRan 2 tests across 2 files. [9.51s]\n"
}
```

## baseline Runtime

```json
{
  "args": [
    "bun",
    "test",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts",
    "--test-name-pattern",
    "independent syn (parsing|callback)",
    "--timeout",
    "180000"
  ],
  "status": 0,
  "tail": "bun test v1.3.14 (0d9b296a)\n[DEBUG] cold syn compiler PID 55931, explicit remaining compiler budget 117984ms\n[DEBUG] independent syn callback AST/span/free-variable oracle matched 38 closed vectors\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:\n(pass) independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts [9048.15ms]\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:\n(pass) independent syn callback AST, spans, free variables and target tuples match the neutral contract [4198.12ms]\n\n 2 pass\n 78 filtered out\n 0 fail\n 9 expect() calls\nRan 2 tests across 2 files. [14.93s]\n"
}
```

## Move Verification

All four hashes are preserved; old paths and empty alias directories are absent. Four active consumers have exact new references.

## verify Runtime

```json
{
  "args": [
    "bun",
    "test",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts",
    "--test-name-pattern",
    "independent syn (parsing|callback)",
    "--timeout",
    "180000"
  ],
  "status": 0,
  "tail": "bun test v1.3.14 (0d9b296a)\n[DEBUG] cold syn compiler PID 56850, explicit remaining compiler budget 117982ms\n[DEBUG] independent syn callback AST/span/free-variable oracle matched 38 closed vectors\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts:\n(pass) independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts [4901.59ms]\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts:\n(pass) independent syn callback AST, spans, free variables and target tuples match the neutral contract [2221.71ms]\n\n 2 pass\n 78 filtered out\n 0 fail\n 9 expect() calls\nRan 2 tests across 2 files. [7.47s]\n"
}
```

## Additional Authored Consumer Repair

Updated: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts`. The latest baseline and post-move receipts used explicitly truncated process logs; both independent syn comparisons passed (including 38 callback vectors). An earlier receipt contains a stale prior-failure log suffix; it is retained as historical evidence and superseded by the subsequent clean baseline.
