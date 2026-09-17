# Verification — Deployed Websites Out Of Box

## Shared Vite deploy surface

- `semioEmojiIndexHtmlVitePlugin` emits `index.html` + `404.html` on build.
- Favicon plugins emit `favicon.ico` + `favicon.svg` aliases.
- Tests: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` (favicon delivery, declared HTML entry, build-write authority).

## `runViteBuild`

Invokes `vite.js` with `--configLoader bundle` (same as `buildViteArtifact`), fixing `bun run vite` native TypeScript strip failures.

## Sites built in this ticket

| Site | Command | `index.html` | `404.html` | `favicon.ico` | `.nojekyll` | `CNAME` |
|------|---------|--------------|------------|---------------|-------------|---------|
| Demonstrator | `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/📜️script.ts build` | yes | yes | yes | yes | yes |
| Hub admin | `🌎️hub/…/📜️script.ts build` | yes | yes | yes | yes | — |
| Projektetage | `♻️mit-bestand/…/projektetage/📜️script.ts build` | yes | yes | yes | yes | yes |

Dist roots:

- `♻️mit-bestand/🧺️demonstrator/dist/site`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist`
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/dist`
