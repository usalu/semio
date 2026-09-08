# Framework Oracle Runtime Results

Public Bun/Nx execution of all twelve canonical exported fixture functions and focused discovery/import checks. Nx workspace: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/🧫️nx-fixture`. A private minimal Nx fixture is used if the full repository graph remains blocked; this proves function execution, not the full graph. Assertions are unchanged.

```json
[
  {
    "name": "testWatchdogTailFixture",
    "status": "passed"
  },
  {
    "name": "testWireRetirementFixture",
    "status": "passed"
  },
  {
    "name": "testInputWriterFixture",
    "status": "passed"
  },
  {
    "name": "testSingleEnqueuePublicationFixture",
    "status": "passed"
  },
  {
    "name": "testInputAdmissionFixture",
    "status": "passed"
  },
  {
    "name": "testInputRootFixture",
    "status": "passed"
  },
  {
    "name": "testBuiltTreeRetirementFixture",
    "status": "failed",
    "detail": "AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\n\n0 !== 9\n\n    at testBuiltTreeRetirementFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts:36:10)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:20:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testInputCommitObserverFixture",
    "status": "passed"
  },
  {
    "name": "fixedListStorageSelfTests",
    "status": "passed",
    "detail": "75"
  },
  {
    "name": "conformanceCorpusSelfTests",
    "status": "passed",
    "detail": "62"
  },
  {
    "name": "testRuntimeTreeRetirement",
    "status": "failed",
    "detail": "AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:\n\n0 !== 9\n\n    at testBuiltTreeRetirementFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts:36:10)\n    at testRuntimeTreeRetirement (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts:30:3)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:20:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "surfaceOwnershipSelfTests",
    "status": "passed",
    "detail": "40"
  },
  {
    "name": "Nx canonical case discovery regression",
    "status": "failed",
    "detail": "bun test v1.3.14 (0d9b296a)\n\n🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts:\n106 |       const accepted = !vector.expected.some(finding => finding.path === source.path && [\"test-case-name\", \"test-owner-delivery-scope\", \"test-layout-depth\"].includes(finding.code));\n107 |       cases.set(source.path.replace(/🟦️\\.ts$/u, \"🥒️.feature\"), accepted);\n108 |     }\n109 |     for (const [path, accepted] of cases) {\n110 |       const discovered = await plugin.createNodesV2[1]([path], {}, { workspaceRoot: repoRootFromHere() });\n111 |       expect(discovered.length > 0, path).toBe(accepted);\n                                                ^\nerror: 🧩️domain/🧪️tests/🧪️case/🥒️.feature\n\nExpected: true\nReceived: false\n\n      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts:111:43)\n(fail) 📐️ canonical test layout > Nx discovers the same canonical names and semantic owners as the layout vectors [48.50ms]\n\n 0 pass\n 14 filtered out\n 1 fail\n 1 expect() calls\nRan 1 test across 1 file. [4.86s]\n"
  },
  {
    "name": "Python canonical stage imports",
    "status": "passed",
    "detail": "[DEBUG] 3 canonical stage imports and golden roots verified\n"
  }
]
```

Subsequent plugin validation: all 15 layout tests pass, 52 assertions, using the same private Nx fixture. The earlier plugin RED result is retained above as historical evidence. Framework function results remain ten pass and two production depth-guard assertion failures.

The earlier full-repository Nx invocation eventually completed graph construction and executed the first framework runtime round (four passes, eight pre-fix fixture failures). Its process returned status 1 from those assertions, rather than a graph failure. A fresh full-graph case-project inventory is now checking the subsequent Nx plugin change. Focused follow-up tests remain separately attributed to the private fixture.

Current discovery evidence supersedes the pending inventory sentence: the full repository Nx inventory completed with exit 0 and returned 250 test-contract projects, including 248 generated case projects. The latest combined focused suite passed 23 tests and 79 assertions, including the actual plugin hook, five Rust compiler path trees, and the source-evidence compiler oracle.

Final focused policy/source-evidence execution supersedes the earlier focused totals: 25 tests passed with 84 assertions. The framework fixture-function result remains ten passes and the two preserved production depth-guard assertion failures.
