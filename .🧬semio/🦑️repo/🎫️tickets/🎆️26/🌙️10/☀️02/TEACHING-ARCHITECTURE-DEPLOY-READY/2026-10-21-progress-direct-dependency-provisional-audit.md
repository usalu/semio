# Provisional Progress Direct Dependency Audit

Static read-only snapshot while owner finishes runtime. No imported function/helper execution, tests, commands or Source edits. Final acceptance requires rereading final candidate bytes.

Observed five-file snapshot:

```json
[
  {
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json",
    "bytes": 26463,
    "sha256": "65b7e9bfe5a4e46156a052dac9f5732c52af455acd33d322601d992dec7f2db6"
  },
  {
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json",
    "bytes": 138684,
    "sha256": "a6219f4d9f3b33f550892e1369f338dc4d63d2ee3eae1f37fd318cca5f58d24e"
  },
  {
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🥒️.feature",
    "bytes": 4260,
    "sha256": "f88e4d486984c0e943398f667c8e1fb62128e59dd09c950bf6c6828a4ffecd21"
  },
  {
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts",
    "bytes": 53498,
    "sha256": "d31bf36711252f64b9159f0439acfa03fbf5a2738bf6207c9b46f019645c6065"
  },
  {
    "path": "🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts",
    "bytes": 51992,
    "sha256": "681451969f4307f42ec8ee7183671753fe7810e84a4af3853dbd37fb8a369af6"
  }
]
```

No newly introduced direct Source read omission found. Progress test imports the progress schema and local JSON corpus, and every actual-writer AST test reads only the same process runtime via `readFileSync(new URL("../../🟦️.ts", import.meta.url))`. All five extracted declarations (sourceProviderCounterV1, cacheScanActivityV1, toolingActivityV1, dependencyWorkActivityV1, createScriptProcessProgressWriterV1) come from that one runtime. The newly added classifier is included in every observed extraction. The feature is a captured behavioral artifact, not a test-time dynamic read. No producer/facade/CJS research source is dynamically opened.

Existing test imports: Bun test/SQLite, Ajv, TypeScript, node:fs; local schema validator; type-only ScriptProgress; actual envelope/formatter/Original-control functions from the process runtime. Runtime imports existing issuance/cleanup/process/budget/progress/source-admission/policy JSON and validation/routing/JSON helpers; the only feature-owned schema import is the existing progress schema. These normal module dependencies remain managed by ordinary Source/compiler capture; this audit does not claim five direct pins recursively cover their entire import graph. Neither test nor runtime currently introduces fileURLToPath-based additional reads.

The exact five proposed pins therefore cover new direct feature-owned bytes and actual AST read targets in this provisional snapshot. No broad transitive roster expansion is justified by these observed accesses. Final runtime review must recheck imports/dynamic paths because authoring remains active.
