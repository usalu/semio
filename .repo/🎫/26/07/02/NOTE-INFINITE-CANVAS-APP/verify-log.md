# Verify Log — Note Infinite Canvas App

## Tests

- `bun run test:note` — PASSED (note-core 4 tests, note-react 1 test)

## Dev host

- `NOTE_PLAY_PORT=6080 bun run dev:note` — Vite ready at http://127.0.0.1:6080/ (HTTP 200)
- Pre-existing dependency scan warnings in monolithic playground renderer (trinity/procedural exports) — unrelated to note

## Runtime (controller)

```
[DEBUG] note block added text text-1
[DEBUG] note block added table table-2
[DEBUG] note block added math math-3
[DEBUG] note block added image image-4
[DEBUG] note block added ink ink-5
[DEBUG] note selection [ "text-1" ]
[DEBUG] verify: blocks 5
[DEBUG] verify: selected 1
```

## Circular dependency fix

- Extracted play IDs to `note/core/play-ids.ts`
- Local import + re-export in `note/core/index.ts` for layout bindings
