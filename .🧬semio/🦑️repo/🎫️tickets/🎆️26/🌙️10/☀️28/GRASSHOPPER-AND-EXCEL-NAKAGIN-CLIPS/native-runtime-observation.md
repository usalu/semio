# Native Runtime Observation

Read-only source inspection while the existing native run remains live. Between the plugin step and item step, Anatomy updates tower_state to TowerB with a selected representative capsule and eagerly calls self._tower(). It then prepares the physical capsule-to-item morph before logging item. Thus the tracer can legitimately remain at plugin while constructing a new detailed tower view and morph. This source gap explains why silence alone is insufficient evidence of a hang; it does not locate the live instruction pointer or prove current runtime progress/completion. No debugger, signal, source/harness edit, private-cache clear or duplicate test was introduced.

Each native animation still executes its actual setup/interpolation/finish/cleanup; the scene code has not been simplified to avoid detailed geometry. Current full profile already completed; exact native per-pair source/runtime endpoints and terminal lifecycle outcomes remain required.

```json
{
  "path": "🎓️teaching\\🏛️architecture\\🦗️grasshopper\\🎬️clip\\🦴️anatomy\\🐍️.py",
  "raw_sha256": "6e04856bc84ae3657e54acf5b5972320d3f04e694b16ce3074fb6d5c960e3a54",
  "plugin_step_line": 617,
  "item_step_line": 660
}
```
