[package]
workspace = "../../../../../../../.."
name = "semio-s-artifact-draw-drawing-diff"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Drawing diff"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-dsl-record = { workspace = true }
semio-framework-dsl-record-derive = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-pack-json = { workspace = true }
semio-framework-schema = { workspace = true }
semio-framework-value = { workspace = true }
semio-framework-value-derive = { workspace = true }
semio-s-artifact-draw-drawing-geometry = { workspace = true }
semio-s-artifact-draw-drawing-model = { workspace = true }

[dev-dependencies]
semio-framework-async-macros = { workspace = true }
semio-framework-os-kernel = { workspace = true, features = ["mutation-testing", "protocol-laws"] }
serde = { workspace = true }
