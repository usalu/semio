# Runtime Live Validation

2026-11-04. Revalidated live state; prior runtime report is historical. Neither port 6070 nor 8787 answers HTTP. Prior activation and composition-prepare processes are absent. Started one owned expensive command: `SEMIO_LANGUAGE=en repo run @semio-tech/framework-os-dev:dev-s-react-prototype`; output is in `🗑️generated/runtime-dev.log`. This target self-activates if the runtime receipt is absent. Currently queued behind unrelated capacity requests.

Browser harness source transpiles successfully with Bun.Transpiler. Chromium and headless shell are installed in the existing Playwright cache. Browser journey has not run yet. Its present checks do not establish natural-file persistence: it spawns new programs on reopen/reload and checks global pane paint/check-in counters. Full end-to-end success must include artifact-specific imported content and saved identity assertions.

## Browser Ownership Regression

The browser journey now resolves explicit `ownerApp` IDs within the row owning plugin; IDF selects `s.energy.model@1/*#editor` instead of incorrectly requiring an IDF-named program. Program selection moved to `selectEditorProgram` in the matrix loader. The targeted regression test passed (1 pass, 194 filtered, 0 fail, exit 0). The full offline contract was rerun; exit 1. See generated runtime-contract.log for details; this does not establish editor operation.

Full contract result: **193 pass, 2 fail out of 195, exit 1** (453 ms). Both failures remain IDML: missing editor/viewer in family component/catalog and no valid natural-file sample.
