# E2E Harness Repair

Scope: ticket-folder scripts only. No PDF engine or editor source was touched. Plan: `e2e-run-plan.md`.

## Changes

| # | Item | Change |
|---|---|---|
| 1 | Import-time `sys.argv[2]` | `📜️large-vector-pdf-oracle.py` takes `generate <file> <spec>` (exclusive create, then verify) or `verify <file> <spec>`. `📜️large-mounted-pdf-oracle.py` and `📜️pdf-invalid-open-oracle.py` use the same `generate`/`verify` contract. Nothing overwrites an existing target. |
| 2 | Oracle steps | The suite verifies `large-vector`, `large-text`, `cancel` and `reject-invalid`. Their smoke modes save no download, so the oracles check the independent input and, for `cancel` and `reject-invalid`, the shell notices in the console report (`📜️mounted-notice-oracle.py`). |
| 3 | Missing cases | New suite cases `image-cancel` (`image-authoring-cancel-browser.ts` and `📜️image-cancel-oracle.py`) and `hub-check-in`. The cancel script derives `image-source.pdf` itself from the PNG history oracle self-test. |
| 4 | G17 driver | `📜️pdf-first-archive-browser.ts` opens a PDF in the mounted shell and lists new `os/local-documents/<sha256>/.semio/events.semio` files. `📜️first-archive-oracle.py` decodes them (FNV64, kind 4, framing, folder hash, SQLite). Suite case `first-archive`. |
| 5 | Port | One port, 6260, read from `reactPort` of the `stdio-pdf-1-7` variant fixture by `📜️e2e-harness.ts`. All 6070 references are gone from the plan, and the natural-file acceptance row uses 6260. |
| 6 | PyMuPDF pin | `pymupdf==1.26.5` and `pillow==13.0.0`, set once in `📜️e2e-harness.ts` (was 1.26.7 and 12.0.0). Source: the staffage oracles `pyproject.toml` and `uv.lock`. The root `🔒️dependencies.json`, `pyproject.toml` and `uv.lock` carry no PyMuPDF. The authoring package pins 2.1 (not used: the oracles import `fitz`). |
| 7 | Hub start | `repo run os-hub:dev` (port 8787) is the plain Hub. `os-hub:dev-collaboration` is the authenticated lane, and `📜️pdf-hub-runtime.ts` runs it in process with two shell dev servers, then `📜️pdf-hub-image-check-in.ts`. The runtime now takes the output directory and PNG as arguments, and the suite passes them. |

Shared module: `📜️e2e-harness.ts` (port, pins, `python()` launcher). The suite and the cancel script import it.

## Verification

- Parse: `bun build --no-bundle` accepted the suite, harness, cancel, first-archive, hub runtime and hub check-in scripts.
- Type-check: `tsc --noEmit --strict` is clean for the suite, harness, cancel and first-archive scripts (the only errors come from an unrelated framework file they import).
- Python: `python3 -I -m py_compile` passed for every new or changed oracle.
- Oracle self-run: generate, verify, refuse-overwrite, notice (pass and fail), image-cancel, PNG self-test and Hub export oracles all passed (`ALL-ORACLES-OK`). It ran on PyMuPDF 1.24.14 and Pillow 12.0.0, the only versions in the local uv cache; the pinned 1.26.5 and 13.0.0 are not cached, so they were not run. The launcher no longer passes `--offline`, so a fresh machine resolves the pin on first use.
- Not run: the browser suite, Hub and dev host (builds blocked by G9a, G9d).

## Known limits

- `large-vector`, `large-text`, `cancel` and `reject-invalid` have no saved-output oracle, because the browser saves nothing for them.
- `first-archive` and `hub-check-in` are expected to fail until G17 and G9d are resolved.
- The suite runs the oracle steps after the browser step, so a failed browser case still skips its oracle.
