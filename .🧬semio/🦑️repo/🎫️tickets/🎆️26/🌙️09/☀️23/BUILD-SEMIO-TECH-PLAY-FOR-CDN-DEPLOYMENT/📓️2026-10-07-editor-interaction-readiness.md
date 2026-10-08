# Editor Interaction Readiness

The Play catalog already contains every registered editor pane, including the stdio artifact families. Existing unit coverage compares pane variants, plugin directories, editor applications, viewer siblings, runtime components, curated example identifiers, and committed descriptors. Existing browser acceptance checks each initial editor boot and painted content but does not switch editor/viewer roles or interact with the example picker.

The acceptance suite will gain a language-neutral, schema-validated role round-trip fixture. Playwright will execute each fixture step through accessible controls and independently verify the active role, application identifier, painted content, and absence of runtime errors. Each curated example picker will be exercised by clearing and restoring the authored example before switching roles. Production validation can use the existing Playwright configuration with an isolated static server supplied through `PLAYWRIGHT_BASE_URL`.

Baseline validation command: `NX_TUI=false NX_DAEMON=false bun nx run @semio-tech/semio-tech-play:test`. The first invocation failed because Play's task router still imported removed routing exports from the repository library. The router now imports the owning process routing modules.

The subsequent test run proved the new interaction contract red with a missing fixture. The fixture and schema were then added; the final run completed with 47 passing and 15 failing tests. The new independent Ajv interaction gate passes. All pane default, committed descriptor, schema, and bounded stdio fleet tests pass. Remaining failures comprise eight existing runtime closure/activation/asset coverage gates caused by the generated registry containing only draw and puzzle, plus seven CDN routing gates under parallel development. The registry's omission follows stale descriptor app channel versions and needs fresh descriptor production rather than fabricated entries.

Changes completed:

- The acceptance suite clears/restores every curated example, switches every pane to its viewer and back to its editor, requires painted content on both surfaces, and verifies the same application dialect.
- HTTP responses at or above 400 and every console error now fail acceptance. The prior generic resource-404 and trace-error suppression was removed.
- The language-neutral role sequence and matching control identifiers are validated by an independent Ajv schema oracle. Playwright provides the runtime oracle through active role attributes and painted content witnesses.
- The pane catalog explicitly pins 16 newly published examples for flow and norm apps with their exact descriptor labels.
- Descriptor coverage reads canonical hub composition owners, while still requiring every authored plugin directory and editor to be represented.
- Demonstrator's documented absent example picker is consistently honored by its default gate.

No browser result is claimed yet. The production release fixture and isolated static serving are being prepared by the parent agent. The acceptance suite imports that shared fixture and can run against those produced artifacts once available.
