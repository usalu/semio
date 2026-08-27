# Default Nx Project Discovery Check

Command without temporary Nx environment overrides: `bun x nx show project @semio-tech/value-numeric-index --json`

Exit code: 0. This is one current project-discovery invocation, not a cross-platform launch or every-app proof. No cache was deleted and no project metadata was changed by the coordinator.

```text
{"root":"🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric","name":"@semio-tech/value-numeric-index","targets":{"test":{"executor":"nx:run-commands","options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric","command":"bun ./📜️script.ts test","forwardAllArgs":true},"configurations":{},"parallelism":true,"dependsOn":[],"inputs":["default","^default",{"env":"SEMIO_TEST_LEVEL"},{"env":"SEMIO_TEST_BUDGET_MS"},{"env":"SEMIO_BUILD_BUDGET_MS"}],"cache":true},"test-quick":{"executor":"nx:run-commands","options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric","command":"bun ./📜️script.ts test quick","forwardAllArgs":true},"configurations":{},"parallelism":true,"dependsOn":[],"inputs":["default","^default",{"env":"SEMIO_TEST_LEVEL"},{"env":"SEMIO_TEST_BUDGET_MS"},{"env":"SEMIO_BUILD_BUDGET_MS"}],"cache":true},"test-long":{"executor":"nx:run-commands","options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric","command":"bun ./📜️script.ts test long","forwardAllArgs":true},"configurations":{},"parallelism":true,"dependsOn":[],"inputs":["default","^default",{"env":"SEMIO_TEST_LEVEL"},{"env":"SEMIO_TEST_BUDGET_MS"},{"env":"SEMIO_BUILD_BUDGET_MS"}],"cache":true},"test-exhaustive":{"executor":"nx:run-commands","options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric","command":"bun ./📜️script.ts test exhaustive","forwardAllArgs":true},"configurations":{},"parallelism":true,"dependsOn":[],"inputs":["default","^default",{"env":"SEMIO_TEST_LEVEL"},{"env":"SEMIO_TEST_BUDGET_MS"},{"env":"SEMIO_BUILD_BUDGET_MS"}],"cache":false}},"implicitDependencies":[],"tags":[]}

```

