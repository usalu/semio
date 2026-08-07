# Wave 3 plugin root template

Target layout for every plugin under `✏️s/🔌️plugins/<p>/`:

```
🔌️plugin/
  🦀️component.rs
  🛂️manifest/🦀️component.rs
  🎟️capabilities/🦀️component.rs
  🔧️setup/🦀️component.rs
  🎛️apps/🦀️component.rs
```

## `🔌️plugin/🦀️component.rs` pattern

```rust
//! 🔌️ Plugin entry — typestate builder.

#[path = "🛂️manifest/🦀️component.rs"]
mod manifest;
#[path = "🎟️capabilities/🦀️component.rs"]
mod capabilities;
#[path = "🔧️setup/🦀️component.rs"]
mod setup;
#[path = "🎛️apps/🦀️component.rs"]
mod apps;

pub fn plugin() -> semio_framework_plugin::Plugin {
    semio_framework_plugin::Plugin::builder(manifest::ID)
        .label(manifest::LABEL)
        .version(manifest::VERSION)
        .setup(setup::register)
        // .capability(capabilities::…) as needed
        // .document_app::<…>(apps::…) for each app
        .build()
}
```

## glue.rs tail

Replace `semio_plugin! { … }` with:

```rust
#[path = "../../🔌️plugin/🦀️component.rs"]
pub mod plugin;

semio_framework_plugin::plugin_exports!(plugin::plugin);
```

## Outliers
- space: multi-app + local_backbone_storage
- demonstrator: panes aggregation
- energy: `.library()` terminal, no apps
