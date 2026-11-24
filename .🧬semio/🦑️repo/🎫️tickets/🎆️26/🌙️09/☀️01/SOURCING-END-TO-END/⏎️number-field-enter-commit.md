# Continuous number field: Enter releases, Escape cancels (2026-11-24)

Host: `✍️input-views/🟦️.tsx` (continuous number field only, `pressNumber`).

- Enter: releases the open press (`lane.commit()`, commit:true, value = last typed value the lane holds), keeps focus; the next input opens a new press (new gesture id). No open press: no-op.
- Escape: aborts the open press with `lane.abort("blur")` (existing wire reason; no new reason, so the native side is untouched). Escape = abort/revert, blur = commit (unchanged). The later blur releases nothing (lane `cancelled`).
- Bug fix: the field ticks on `onInput` (React `onChange` is a no-op for it). A controlled field that reverted to its shown value (`0`) saw no `onChange` for `fill("0")`; every input event now ticks, so the lane's last value, and hence every release, is the current typed value.
- Slider semantics unchanged (blur = abort).

Tests: `🧪️tests/🎚️continuous-gesture-lane/🟦️.ts` (describe "continuous number field's press"), cases in `🧪️tests/🧫️fixtures/🎚️number-field-press/🔣️.json` (language-agnostic: steps input/enter/escape/blur, expected ticks/releases/aborts with press names A/B). Run with `node_modules/.bin/vitest run --config $T/🗑️generated/vitest.number-field.config.mts`: 21 passed (log `🗑️generated/number-field-vitest.txt`).

Note: the third-party cross-check is the browser (Playwright) journey in `record-min-availability-wire.ts`; rerun it against a fresh build.
