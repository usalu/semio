# Framework Oracle Runtime Results

Public Bun/Nx execution of all twelve canonical exported fixture functions. Assertions are unchanged.

```json
[
  {
    "name": "testWatchdogTailFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at compile (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:159:26)\n    at testWatchdogTailFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/🧪️tests/🔬️watchdog-tail/🟦️.ts:9:67)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testWireRetirementFixture",
    "status": "passed"
  },
  {
    "name": "testInputWriterFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at compile (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:159:26)\n    at testInputWriterFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/🧪️tests/🔬️input-writer/🟦️.ts:9:67)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testSingleEnqueuePublicationFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at compile (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:159:26)\n    at testSingleEnqueuePublicationFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts:9:67)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testInputAdmissionFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at compile (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:159:26)\n    at testInputAdmissionFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts:13:67)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testInputRootFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:235:34)\n    at testInputRootFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🔬️input-root/🟦️.ts:10:67)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testBuiltTreeRetirementFixture",
    "status": "failed",
    "detail": "AssertionError [ERR_ASSERTION]: false == true\n    at testBuiltTreeRetirementFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts:29:3)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "testInputCommitObserverFixture",
    "status": "failed",
    "detail": "Error: no schema with key or ref \"http://json-schema.org/draft-07/schema#\"\n    at validate (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:148:27)\n    at validateSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:261:28)\n    at _addSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:461:18)\n    at compile (/Users/ueli/Documents/semio/node_modules/ajv/dist/core.js:159:26)\n    at testSingleEnqueuePublicationFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts:9:67)\n    at testInputCommitObserverFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/🧪️tests/🔬️input-commit-observer/🟦️.ts:8:3)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
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
    "detail": "AssertionError [ERR_ASSERTION]: false == true\n    at testBuiltTreeRetirementFixture (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧪️tests/🔬️built-tree-retirement/🟦️.ts:29:3)\n    at testRuntimeTreeRetirement (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts:30:3)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻coordination/📜️script.ts:19:39\n    at processTicksAndRejections (native:7:39)"
  },
  {
    "name": "surfaceOwnershipSelfTests",
    "status": "passed",
    "detail": "40"
  },
  {
    "name": "Nx canonical case discovery regression",
    "status": "failed",
    "detail": "bun test v1.3.14 (0d9b296a)\nThe following filters did not match any test files:\n 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts\n255097 files were searched [65.11s]\n\nnote: Tests need \".test\", \"_test_\", \".spec\" or \"_spec_\" in the filename (ex: \"MyApp.test.ts\")\nnote: To treat the \"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts\" filter as a path, run \"bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts\"\n\nLearn more about bun test: https://bun.com/docs/cli/test\n"
  },
  {
    "name": "Python canonical stage imports",
    "status": "passed",
    "detail": "[DEBUG] 3 canonical stage imports and golden roots verified\n"
  }
]
```
