# React Custom File Picker Cancellation

## Actual Red And Green

The retained public-source Bun/JSDOM regression ran before the correction:0pass4fail, exit1. Browser cancel left the promise pending; empty selection and successful selection retained the onchange closure; click failure rejected instead of empty cancellation. These were observed failures, not source inference.

After correction the same test passed4tests21assertions, exit0. It calls actual exported requestFileOpen with a real independent JSDOM input/FileReader and a strict-Ajv validated neutral fixture. It verifies cancel, empty selection, picker click failure, exact single cleanup despite repeated terminal events, multiple-file order, Unicode filename, exact dataURL bytes and chooser options.

## Source Change

requestFileOpen now delegates selection to existing requestFileSelectionV1. One owner settles cancellation and retires input handlers/input once before reading selected files. Existing text/dataURL payload behavior is retained; FileReader abort or synchronous read refusal also settles its per-file read.

Permanent renderer Vitest coverage was added alongside media-export-and-natural-files, sharing file-open.json and its strict schema. That complete Vitest suite has not executed while general JavaScript dependency publication is pending. The isolated executed JSDOM regression validates the actual public helper; it is not a live app/browser picker journey or bounded large-file-read proof.

## Files

- Renderer engine ShellHelpers/plumbing/downloads-and-file-requests public implementation.
- Renderer engine media-export-and-natural-files tests and file-open neutral JSON/schema.
- Ticket file-open.test.ts retained test input.
