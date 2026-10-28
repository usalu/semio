# Print Exploration

The reusable document class is semio.cls in 🧰️framework/🛍️products/📓️print/🖋️latex/. It selects geometry and KOMA base class by type (report, paper, phd, flyer). Assignment should be a first-class type=assignment using article sectioning, with a companion semio-assignment.sty for reusable assignment metadata/layout primitives. semio-core.sty stores the type and configures appendix sectioning; unknown types already fall back to section appendix level, but document kind registration remains in semio.cls. Existing zukunftbau.cls demonstrates a domain class wrapping semio. No nested AGENTS.md was found in the framework/products/print ancestry beyond framework descriptive instructions.

## Exact Integration Files

- 🧰️framework/🛍️products/📓️print/🖋️latex/semio.cls: register assignment geometry and base class and load assignment package.
- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty: proposed reusable Assignment implementation.
- 🧰️framework/🛍️products/📓️print/🧾️template/: add the requested Parametrisierung worked example in a semantic assignment directory.
- 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json: add document id, texPath, collection templates and every authored source. All latex files are already staged through librarySources, so a new .sty needs no separate staging registration.
- 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🧫️fixtures/🧫️merge-contract.json: template ids are asserted exactly; update when registering assignment.
- 🧰️framework/🛍️products/📓️print/README.md: document Assignment API and build id.

## Tests and Build

Cases use 🥒️.feature and TypeScript adapters, with fixture contracts in JSON and third-party oracle registrations under 🔮️oracles/🔣️.json. Existing pure document/class checks are embedded in 🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts. The quick suite validates catalog, template list and semio.cls/phd macros. Add an assignment verification called from this suite or the native macro suite; compiled PDF inspection uses the existing owned withCompiledPrintProbeV1 interface and its inspect() PDF oracle, avoiding direct Tectonic calls.

Owner: @semio-tech/print; build, fonts, deps-tectonic, deps-tex, generate, test, test-macro-staging are authored repo targets in 📦️packages/🟦️typescript/📋️project.json. build is aggregate completion and only checks published PDFs; it does not accept an individual document id. Individual compile is the authored script 🔨️modules/🖨️tectonic-template-compilation/📜️script.ts with exactly build <id> via repo script infrastructure. Each registered document gets light and dark PDFs automatically. Canonical source must have documentclass options and explicit language=de or en; dark derivation appends/replaces theme=dark and rewrites logo assets.

The compiler publishes under the repository artifact store through 📦️artifacts/🎛️operation, so do not invent an output path. printTemplatePdfNames(texPath) returns basename.pdf and basename-dark.pdf. Compiled probes expose retained PDF path only during the owned callback. Temporary extracted artifacts for ticket review belong under ticket 🗑️generated and must be removed at completion.

## Existing Primitives

semio-window provides titled boxes; semio-table provides header rows and long/short tables, so use these instead of raw ad hoc box styling. semio-viz is already loaded by semio.cls for a K8 graph and box geometry if appropriate. All theme colors come from semio design tokens. semio.cls currently defaults language to de; Assignment should require/validate explicit document language to meet the task instructions without broad unrelated changes.
