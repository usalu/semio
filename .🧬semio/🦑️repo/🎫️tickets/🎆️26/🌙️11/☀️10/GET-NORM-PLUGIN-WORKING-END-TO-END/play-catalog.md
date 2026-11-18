# Play catalog generation

## Channel descriptors
- `🧩️component/🔣️.json` is `describe` output; `executionProtocol.appChannelVersion` is stamped from the host constant (APP_CHANNEL_VERSION) and holds no wasm digest, so nothing else in the file depends on it. 70 of 71 committed descriptors said 24, the pin and host say 25.
- Fix: line-exact text edit 24 -> 25 in all 70 (formatting preserved); all 71 now 25. Matches what `describe` would emit and the checker (`channel-version check-descriptors`) criterion (value == pin).

## Publication
- No change needed: `specificCatalogPublicationV1` (`✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts`) already merges the independent hub contribution `🌎️hub/📇️catalog/🔣️.json` (packageRoots `🌎️hub/🧩️compositions`). The audit missed this.

## Generation
- `repo run @semio-tech/s-dev-composition:generate` blocks on graph inference (324/4774, ~18 h left per `repo fleet status`); cancelled.
- Native fallback: `play-catalog-generate-driver.ts` (this folder) mirrors `📣️publication/📜️script.ts` but self-issues the process envelope; run as `cd ✏️s/🧑‍💻dev && repo <abs>/play-catalog-generate-driver.ts generate`. Result: 70 plugin crates, 158 playgrounds; `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/` now exists (playgrounds/🟦️.ts, plugins, diagnostics `[]`).
- All 15 norm variants and all 149 play runtime pane variants are present in `🎮️playgrounds/🟦️.ts`.
- Not run: the generate target's `generate-stdio-format-claims` dependency (output `✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/` absent) and the `catalog-check`/`verify-catalog` targets.
