# Drawing Text Owned Result

The normal owning run 68944 exited 0: 18 pass, 0 fail, 926,942 assertions (1.127s). The sequential strict7 run 64297 exited 2; its genuine diagnostics are retained in the strict log. All 116 captured inputs matched after both terminals and current verification. No input was edited or silently resealed.

The owning runtime result covers the original Scene12 and new DrawingText6 at the captured bytes. It does not establish strict acceptance, the original fourteen glyph-scene rows, all sixteen production callers, or full Layout/browser acceptance.

## Retained evidence

- Command: drawing-text-owned-command.json
- Receipt: 🗑️generated/drawing-text-owned/launch-receipt.json
- Owning log: 🗑️generated/drawing-text-owned/test/test.log
- Strict log: 🗑️generated/drawing-text-owned/strict/strict.log

## Exact terminal receipts

{
  "test": {
    "session": 68944,
    "exitCode": 0,
    "terminalChunk": "b6c82f",
    "log": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/🗑️generated/drawing-text-owned/test/test.log",
    "bytes": 2425,
    "sha256": "34d88f47e04385db0c27a2080c62dd71be4fa861cee7da1e8704ea887c46cdfe"
  },
  "strict": {
    "session": 64297,
    "exitCode": 2,
    "terminalChunk": "84a2ab",
    "log": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/🗑️generated/drawing-text-owned/strict/strict.log",
    "bytes": 723,
    "sha256": "c004d4b7dc6aa74d5b360a9b6d6e550656da98b404a1425955e95186aaf04951"
  }
}

## Strict diagnostics

[tools] bun waits for machine capacity.
[capacity] admitted command: 1 job(s), 2048 MB reserved (20495 MB available, 4096 MB floor)
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🟦️.ts(7,32): error TS2724: '"@semio-tech/framework/2d/text/font/run"' has no exported member named 'FontOutlineRun'. Did you mean 'FontOutlineRunJob'?
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🟦️.ts(7,52): error TS2305: Module '"@semio-tech/framework/2d/text/font/run"' has no exported member 'FontRunInput'.

