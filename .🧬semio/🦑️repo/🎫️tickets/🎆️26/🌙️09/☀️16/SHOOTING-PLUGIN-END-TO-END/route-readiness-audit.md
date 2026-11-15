# 🧭 Route Readiness Audit (2026-11-15)

Read-only audit.

- The shooting devContribution is declared at `🌎️hub/🧩️compositions/🎥️shooting/🧩️component/📦️packages/🦀️rust/Cargo.toml:22` and points to `✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🧬️schema/🔣️.json`. The entry `🚀️entry/🟦️.ts` and the vite builder exist.
- The route test (`✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🟦️.ts:16`) expects `devContribution === fixture.contribution`, and the fixture matches the Cargo line.
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` is absent, so no generated catalog row is published yet.
- React-lane targets `prepare-/activate-/serve-shooting-react-prototype` are generated per lane, not hand-written. They are referenced from `🏢️semio-tech/🎡️play/📋️project.json:39,109`.
- The TS test project is `@semio-tech/shooting-js`, target `test` (nine runtime entries).
- The old ticket scripts `📜️activate-shooting-react.sh` and `📜️serve-shooting-react.sh` call raw `bun nx` / `bun 📜️script.ts`. They are superseded by `repo run`.
- Nine staged changes sit under the shooting plugin, from peers (Nov 12 and Nov 14).
