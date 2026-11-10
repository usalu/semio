# Print Native Route Audit

## Finding

No concrete POSIX shell or unregistered system executable requirement was found in the inspected canonical x64 Windows route. The pinned Tectonic manifest explicitly supports win32/x64 only: 🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json:53-60. A native Windows arm64 host is refused by tectonicDistributionV1 because no matching authored platform/architecture exists (📦️artifacts/📥️material/🟦️.ts:13-14). This is a concrete unsupported host, not evidence that the current x64 host fails.

## Actual routes

- Course 📦️packages/🟦️typescript/📋️project.json:976-1073 routes the four localized targets through repo scripts, each depending on Print fonts, deps-tectonic, deps-tex and generate. Its 📜️script.ts:59-66,148 binds both assignment IDs and locales to buildCourseAssignmentV1. The shared builder 🏗️builder/📖️assignments/🟦️.ts:21-30 calls publishPrintArtifact directly using authored sources and resources, with no shell command.
- Print 📦️packages/🟦️typescript/📋️project.json:36-55,102-133,326-344,369-386 selects repo script entrypoints. Generate's 🎨print-design-token-paints/📜️script.ts:8-12 writes the generated stylesheet using node:path and first-party rendering. Font catalog 📜️script.ts:7-11 and 🟦️.ts:8-11 call retained first-party material preparation.
- Tectonic toolchain and TeX bundle prepare scripts call preparePrintMaterialV1 for tectonic and tex respectively (🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts:6; 📚️bundle/📜️script.ts:5). Material producer 📦️artifacts/📥️material/📤️producer/🟦️.ts:23-32 selects the exact native distribution, fetches and validates pinned content, decodes with first-party bounded archive code, and stages authored fonts or pinned bundle ranges. It refuses missing materials with an offline grant; availability and successful acquisition were not tested.
- Compilation 🔨️modules/🖨️tectonic-template-compilation/🟦️.ts:44-54 takes the retained executable path and calls captureOwnedCommandResultV1 with an argument array; it builds a file URL for the TeX bundle, selects semicolon for Windows TEXINPUTS, normalizes the relative TeX argument, and retains and rechecks tool/bundle/fonts. This is a managed external executable, not a shell lookup. It enforces a 256-character resource destination limit (line 50), so an overly deep workspace or scratch path can still be refused; no actual overflow was established.

## Scope

Read-only inspection of the selected manifests, router, shared assignment builder, four Print prerequisite entries and directly called material/compiler code. No execution, platform spoofing, acquisition, asset audit, or source modification. Actual native PDF compilation and its external service/archive availability remain unverified.
