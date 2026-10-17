[package]
workspace = "../../../../../../../../.."
name = "semio-s-artifact-layout-layout-mutations-frames"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Layout frame mutations: create, delete, move, resize, rotate, scale, drag, reorder and restyle frames and layers"

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
semio-framework-ui-locale = { workspace = true }
semio-framework-value = { workspace = true }
semio-framework-value-derive = { workspace = true }
semio-s-artifact-layout-layout-diff = { workspace = true }
semio-s-artifact-layout-layout-model = { workspace = true }
serde = { workspace = true }

[dev-dependencies]
semio-framework-async-macros = { workspace = true }
semio-framework-diagnostic = { workspace = true }
semio-framework-os-kernel = { workspace = true, features = ["mutation-testing", "protocol-laws"] }
semio-s-artifact-layout-layout-engine = { workspace = true }
serde_json = { workspace = true }
semio-framework-io-ports = { workspace = true, features = ["testkit"] }
