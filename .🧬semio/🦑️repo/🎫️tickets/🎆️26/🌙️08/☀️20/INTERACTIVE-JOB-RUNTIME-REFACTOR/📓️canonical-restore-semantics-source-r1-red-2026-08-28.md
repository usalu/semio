# Canonical Restore Semantic Source R1 RED

The actual existing command `bun x nx run @semio-tech/framework-replication-rs:test-local-interaction-source` exited 1. The desired fixture law rejected the unchanged twelve-case input: required nonbroadcastDomains was absent, both additional semantic cases were absent, and minimum14 was not met. This is a source/schema coverage RED, not a cold-function behavioral defect or native/live test. Strict Ajv compiled successfully. No fixture merge had occurred and no native command ran.

The exact five selected source hashes were identical before and after execution:

```text
9b200e30396f6637f08b6b3a7d5017eac938a8edc88258ade34e907e5a87348e  🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔣️local-interaction.schema.json
2467d16665faefbdf3aa301bb7d38e41dfa92d59c98fd0b66644b21005069497  🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🟦️component.ts
2771c40a72421ae008a91b2001c5e3fe77a73cb0504354ea737e41b706716b29  🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json
e06b8b631a933334758a82debeb52103fd4c5b974a4e20d2dc2d677e893c3733  🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.schema.json
2921f8aefdfb6d31d667a9f0d3d89503da67d1d5bb8ba31f9620537cf538876d  🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/📜️script.ts
```

Original complete fixture SHA was2771c40a72421ae008a91b2001c5e3fe77a73cb0504354ea737e41b706716b29. Original first12 JSON case-array SHA was1743dc0eb718158941225a5260bad4386f068eada883c0e98a2bed0ea0b0ec50. Original case content must remain unchanged by the later merge.

## Actual Tool Output

ANSI presentation escapes omitted, otherwise captured tool text:

```text

> nx run @semio-tech/framework-replication-rs:test-local-interaction-source

> bun ./📜️script.ts test-local-interaction-source

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

11 | const schema = await Bun.file(new URL("../🧬️schema/🔣️local-interaction.schema.json", import.meta.url)).json();
12 | const fixtureSchema = await Bun.file(new URL("./🔣️local-interaction.schema.json", import.meta.url)).json();
13 | const fixture = await Bun.file(new URL("./🔣️local-interaction.json", import.meta.url)).json();
14 | const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(schema);
15 | const validate = ajv.compile(fixtureSchema);
16 | assert(validate(fixture), JSON.stringify(validate.errors));
     ^
AssertionError: [{"instancePath":"","schemaPath":"#/required","keyword":"required","params":{"missingProperty":"nonbroadcastDomains"},"message":"must have required property 'nonbroadcastDomains'"},{"instancePath":"/cases/0/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/1/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/2/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/3/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/4/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/5/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/6/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/7/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/8/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/9/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/10/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases/11/id","schemaPath":"#/properties/cases/allOf/0/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-explicit-private-replacement"},"message":"must be equal to constant"},{"instancePath":"/cases","schemaPath":"#/properties/cases/allOf/0/contains","keyword":"contains","params":{"minContains":1},"message":"must contain at least 1 valid item(s)"},{"instancePath":"/cases/0/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/1/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/2/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/3/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/4/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/5/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/6/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/7/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/8/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/9/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/10/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases/11/id","schemaPath":"#/properties/cases/allOf/1/contains/properties/id/const","keyword":"const","params":{"allowedValue":"sparse-empty-preserves-three-maps"},"message":"must be equal to constant"},{"instancePath":"/cases","schemaPath":"#/properties/cases/allOf/1/contains","keyword":"contains","params":{"minContains":1},"message":"must contain at least 1 valid item(s)"},{"instancePath":"/cases","schemaPath":"#/properties/cases/minItems","keyword":"minItems","params":{"limit":14},"message":"must NOT have fewer than 14 items"}]
 generatedMessage: false,
     actual: false,
   expected: true,
   operator: "==",
       code: "ERR_ASSERTION"

      at /Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/📜️script.ts:16:1

Bun v1.3.14 (macOS arm64)
Warning: command "bun ./📜️script.ts test-local-interaction-source" exited with non-zero status code


 NX   Running target test-local-interaction-source for project @semio-tech/framework-replication-rs failed

Failed tasks:

- @semio-tech/framework-replication-rs:test-local-interaction-source

Hint: run the command with --verbose for more details.


 NX   Nx detected a flaky task

  @semio-tech/framework-replication-rs:test-local-interaction-source

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

