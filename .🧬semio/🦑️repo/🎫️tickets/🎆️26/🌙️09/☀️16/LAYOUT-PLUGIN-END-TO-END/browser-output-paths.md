# Browser Output Path Validation

Added ticket-only schema, neutral corpus, pure path planner and test. Journey validates resolved output root, every planned report/media/screenshot filename, downloads directory and36-character browser-generated download child before mkdirSync or browser launch. Invalid empty/dot/parent names and paths over256JavaScript UTF-16 units fail before those effects. Sanitization remains ASCII, path joining uses platform tools, and emoji checkout roots count surrogate pairs correctly.

TDD: normal managed Bun RED66677 eventually admitted and exited1 because helper was absent. After minimal implementation, normal managed Bun GREEN exited0 with all8neutral cases across POSIX and Windows path ports. Independent Ajv validates the schema against independently node:path-computed path lengths; no --light semantic test or browser launch. Command: repo --tool=bazel tools exec bun -- <ABS_TICKET>/browser-output-paths.test.mjs.

Files: browser-output-paths.schema.json, browser-output-paths-cases.json, browser-output-paths.mjs, browser-output-paths.test.mjs, browser-journey.mjs and this report. No product sources changed. Existing launcher ownership remains root lane. Browser runtime acceptance remains pending.
