# Wave 3 — Plugin Root Builder Contract

## Outcome
All 32 plugins under `✏️s/🔌️plugins/` now expose:

```
🔌️plugin/
  🦀️component.rs          # pub fn plugin() -> Plugin via Plugin::builder
  🛂️manifest/🦀️component.rs
  🎟️capabilities/🦀️component.rs
  🔧️setup/🦀️component.rs
  🎛️apps/🦀️component.rs
```

Glue crates wire `#[path = "../../🔌️plugin/🦀️component.rs"] mod plugin;` and
`plugin_exports!(plugin::plugin)` (energy is library-only: `pub mod plugin` without exports).

## Outliers
- **space**: multi-app + `local_backbone_storage` + `register_s_exports`
- **demonstrator**: delegates to `crate::panes::bundle()`
- **energy**: `.library()` headless plugin; SDK dep added

## Macro
Active `semio_plugin!` usages in plugin glues: **0** (retired in Wave 4 SDK cleanup).
