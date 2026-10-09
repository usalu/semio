# GIS Diff Physical Ownership

The previous five-project Rust check rejected GIS native diff macros because the owned child model has no physical DSL field binder. Native codecs now consume the logical ToValue model explicitly. Semantic diff types no longer derive physical record binding. Three inserted imports were moved below their module docstrings. The native codec test uses an authored neutral diff and independent serde_json oracle. Verification pending.

- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️diff-native-ownership/🔣️.json
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs
