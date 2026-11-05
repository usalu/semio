# 💾 Awaited Picked File Commit

Production PickedFileFlowPortV1.commit now accepts owner, instanceId and task and returns boolean or Promise<boolean>. Existing synchronous commit callbacks remain assignable. Task carries cancellation/progress authority for persistence integration. The opened notice is published only after commit resolves true and cancellation is checked again.

While commit waits, the flow retains created-instance ownership. False, rejection or cancellation releases that instance and reports failed/cancelled. Cancellation races a still-pending commit so the user is not held by storage; downstream persistence must observe task.signal to prevent its own late side effects. The abort listener is removed on completion. No reload success was invented; root owns actual actor/catalog persistence.

Schema-first neutral cases cover deferred accepted persistence, failure and cancellation, independently validated by Ajv. Tests also cover commit rejection and import returning after cancellation. Initial test-first run:5fail11pass plus one unhandled async rejection, demonstrating old synchronous commit defect. Final command: SEMIO_LANGUAGE=en repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITOR-COMPLETION/vitest.picked-commit.config.mjs. Result:exit0,1file16tests passed,duration825ms,tests223ms.

Owned product files: ShellHost/📂️picked-file-opening/🟦️.ts; engine/🧪️tests/📂️picked-file-opening/🌊️flow.ts; new engine/🧪️tests/📂️picked-file-opening/🧫️fixtures/🔣️.json. All under 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine (ShellHost under 🧱️elements/🏛️ShellHost). No document-transfer scope changes made.
