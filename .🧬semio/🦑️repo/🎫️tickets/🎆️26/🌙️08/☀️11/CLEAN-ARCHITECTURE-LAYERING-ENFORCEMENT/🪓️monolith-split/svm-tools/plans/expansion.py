PLAN = dict(
    name="expansion",
    orig="svm-tools/orig/expansion.rs",
    real="🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs",
    real_lib="🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs",
    mount_after=132,
    children=[
        dict(key="typed_path", folder="🧭️typed-path", mod="typed_path", doc="🧭️ Typed path, key and edit bodies for structs and enums.", ranges=[(717, 1568)]),
        dict(key="entries", folder="🎯️expand-entries", mod="expand_entries", doc="🎯️ The `ToValue` and `FromValue` derive entry points.", ranges=[(1570, 2137)]),
        dict(key="controlled", folder="🎚️controlled-output", mod="controlled_output", doc="🎚️ Controlled (cumulative-budget) encode, decode and retirement bodies.", ranges=[(2139, 2349)]),
    ],
    oracle=dict(trees=["🧰️framework/🔨️modules/🌱️value/✨️derive"], crate="semio_framework_value_derive", cfgs=["lib", "test"], crate_type="proc-macro"),
)
