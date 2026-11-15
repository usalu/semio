# Chain run log

## Phase A (static)
- s publication (`✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json`) lists only `✏️s/🔌️plugins`, but `specificCatalogPublicationV1` appends the independent hub contribution read from `🌎️hub/📇️catalog/🔣️.json` (`ownerRoot 🌎️hub`, `packageRoots [🌎️hub/🧩️compositions]`). The earlier chain-audit claim that hub components are filtered out is wrong: norm's component Cargo.toml (`🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust`) is admitted exactly like puzzle, remodel and fem.
- Norm component crate has the same keys as remodel/fem: `package.metadata.component.package = semio:norm`, `role = hub`, `deployment-directory = 📕️norm`, `component-kind = plugin`, 15 playground variants (unique variants, unique ports 6091-6105 / 6191-6205, apps match descriptor app ids), sources.artifacts. Only puzzle additionally has `devContribution` (optional; remodel and fem lack it too). No fix needed.
- `activate-s-react-prototype` / `serve-s-react-prototype` are synthesized per playground variant; variant `s` is declared by the hub `space` component (`catalog-default = true`, ports react 6070), which is admitted by the same hub contribution.
- Fixed pin path in e2e-proof-chain.md.

## Phase B
- Not started: `describe12` (log `🗑️generated/describe12.log`) had no `EXIT` line at 05:39. It was stuck in "Reading the project graph slice ... waiting for memory: 2048 MB are asked for, 4751 MB free above the floor", with an empty running list for over 140 s (the graph-memory admission stall documented in norm-loads-in-s.md, not a norm defect). Chain steps 2-9 not run; describe not relaunched per instructions.
