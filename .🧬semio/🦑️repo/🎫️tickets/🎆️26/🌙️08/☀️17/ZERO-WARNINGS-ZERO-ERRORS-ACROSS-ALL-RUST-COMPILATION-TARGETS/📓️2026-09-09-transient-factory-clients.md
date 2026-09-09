# Transient Factory Clients

Current removed-factory call sites: 22 in 7 Rust files. WindowTransientOwner now requires build_owners returning WindowTransientOwnerBundle with preparation, owned state retirement, and owned mutation retirement. The shared transient store disposer accepts an explicit owned state retirement factory. No removed generic factories are restored. Consumer completion and runtime proofs remain pending.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:553 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:557 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:572 — Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:576 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:580 — Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:794 — Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:798 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:827 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:831 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:839 — Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:192 — pub fn bounded_transient_preparation_factory<P, M>() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P, M>>
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:200 — pub fn bounded_transient_root_retirement_factory<P>() -> Arc<dyn store::SnapshotRetirementFactory<P>>
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs:207 — pub fn bounded_transient_store_disposer<P, M>() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<P, M>>>
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:492 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:496 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:738 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:742 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:93 — semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>()
- ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:97 — semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>()
- ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs:101 — semio_framework_plugin::bounded_window_transient_store_disposer::<Self>()
- ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:760 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
- ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:764 — Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
