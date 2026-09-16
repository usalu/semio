# 📓️ Fix — Förderhinweis funding logos (2026-09-16)

## Symptom

On the demonstrator landing introduction step **Förderhinweis**, some or all funding logos (BMWSB, BBSR, Zukunft Bau) did not appear.

## Root cause

1. **Asset layout vs URLs** — `🪧️brand.ts` points at nested files under `🪧️logos/<org>/☀️logo.png`, while an older production `dist/` tree still had flat `🖼️bmwsb.png` names. A production/preview bundle that was not rebuilt after the reorganisation had **no files at the new paths**.

2. **Static-dir middleware** — Missing files under a `static-dir` route called `next()`, so Vite answered with **SPA `index.html` (200, `text/html`)**. `<img>` tags then failed decode; `IntroductionLogoRow` keeps the row `visibility: hidden` until every logo reports an aspect ratio from `onLoad`.

3. **Build copy** — `staticDirVitePlugin` `closeBundle` used `cpSync` without removing the previous destination, so stale PNGs could remain beside new paths and mask the problem until URLs changed.

## Fix

- `staticDirVitePlugin`: 404 for missing files under the mounted route; wipe destination before `cpSync`.
- `IntroductionLogoRow`: run `onLoad` on dark-mode `<img>` as well (dark-only decode path).
- Tests: on-disk logo existence (engine contract), static-dir 404 (vite unit), landing Förderhinweis visible logo sizes (Playwright acceptance).

## Verification

- Dev server (`:6029`): all six funding logo URLs return `Content-Type: image/png`.
- After the next demonstrator **production build**, `dist/♻️mit-bestand/🧺️demonstrator/🖼️asset/🪧️logos/` must mirror source (nested org folders only).
