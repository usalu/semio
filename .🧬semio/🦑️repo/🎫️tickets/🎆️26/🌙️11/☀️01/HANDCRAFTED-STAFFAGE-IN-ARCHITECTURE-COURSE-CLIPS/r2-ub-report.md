# R2 ub: real-browser check of the thermal Span control

## Serving
- `repo run @teaching/staffage-site:dev` (port 6291, watch off) never reached the dev server. It waited about 27 minutes on "Another process is building the dependencies" and `dependencies:current-input`. I stopped it.
- Fallback that earlier reports used (raw Vite) fails too: the site config imports the framework and dies on the unlinked `@semio-tech/framework-transient` (same repo-infra gap as in q2).
- Ticket-local harness in `🧰️ub/`, started with `repo tools exec bun -- x vite --config 🧰️ub/vite.config.ts` on 127.0.0.1:6291: `vite.config.ts` (the same aliases as the real site config, plus framework-transient and d3-format aliases, react and tailwind plugins, and a stub for `node:*` imports because the library barrel re-exports exporters that import `node:fs`), `index.html` (adds a `process` shim), `entry.ts` (same two lines as the site entry). The app code under test is the real React target, unmodified.
- Browser: Playwright Chromium headless shell 1234 (the cached one, `UB_CHROME`), `reducedMotion: no-preference`, clip paused before comparing pictures. Script `🧰️ub/verify.mjs` (49 checks), `🧰️ub/probe.mjs` (page probe).

## Results (all 49 checks pass, 0 console errors, 0 page errors)
- Labels: en radios Automatic / Fixed / Span, de Automatisch / Fest / Spanne, on desktop (1440x900) and mobile (390x844), for `components/panel-radiator` and `humans/walking-commuter`. Span help title present in both languages. Field label "Span around the air temperature (± K)" (de: "Spanne um die Lufttemperatur (± K)"), unit K.
- Keyboard: focus on Automatic, ArrowRight twice selects Span (`aria-checked` true, focus follows), Tab lands on the K field, Ctrl+A and typing 10, 25, 80 updates the address each time (`span=10|25|80`). 0 sets `aria-invalid` and keeps the last valid span.
- Picture: stage screenshots differ for 10, 25 and 80 K on both assets (radiator hashes ef2e06, b409e3, bf38d0; commuter e1bc5c, 3180ff, 29b247). Read: the radiator is saturated red at 10 K and orange at 80 K, so the span changes the colour scale as intended.
- Round trip: reload with `span=80` restores the Span radio, the field value 80 and the same hash. `span=-5` falls back to Automatic.
- Console: 0 errors. Warnings only: "Failed to decode downloaded font" (OTS parsing error) for every font file under the dev harness. The font binaries are not served as fonts here, which also explains the wide word spacing in the screenshots. This is the harness or the font assets, not the Span control, and I did not touch it.

## Defects found and fixed
- None in the React target. No source file changed.

## Tests
- React vitest (`q2-vitest-react-alias.config.ts`): 18 files passed, 117 tests passed, 1 skipped.

## Open
- `repo run ...:dev` of the staffage site is blocked by the shared dependency build queue and the unlinked framework packages (repo infra, not fixed).
- The 24 px column of the unit "K" next to the field sits tight against the panel edge at 1440 px; not a defect.
- Fonts fail to decode in the harness; whether the real site build serves them correctly was not checked.

## Cleanup
- Server (port 6291) and its process tree stopped. `🗑️generated/ub` deleted. Scripts kept in `🧰️ub/`.
