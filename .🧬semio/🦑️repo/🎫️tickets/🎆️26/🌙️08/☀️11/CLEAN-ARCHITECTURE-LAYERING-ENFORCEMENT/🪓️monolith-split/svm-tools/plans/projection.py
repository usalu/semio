PLAN = dict(
    name="projection",
    orig="svm-tools/orig/projection.rs",
    real="🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs",
    real_lib="svm-root/🦀️.rs",
    children=[dict(key="a", folder="🧾️rows-a-h"), dict(key="b", folder="🎟️rows-h-t"), dict(key="c", folder="📖️rows-t-z")],
    synthetic_lib=dict(path="svm-root/🦀️.rs", text='#[path = "../🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"]\npub mod schema_metadata;\n#[cfg(test)]\n#[path = "../🧰️framework/🔨️modules/🧬️schema/📽️projection/🧪️tests/🦀️.rs"]\nmod typegen_tests;\n'),
    oracle=dict(trees=["🧰️framework/🔨️modules/🧬️schema/📽️projection", "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest"], crate="semio_framework", cfgs=["lib", "test", "wasm", "p2"]),
)
