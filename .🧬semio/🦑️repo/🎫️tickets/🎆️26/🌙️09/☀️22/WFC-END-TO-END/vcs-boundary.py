from pathlib import Path
import os

root = Path.cwd()
vcs = root / '🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs'
identity = vcs / '🚪️io/💾️binary/🪪️entity-identity'
package = identity / '📦️packages/🦀️rust'
package.mkdir(parents=True, exist_ok=True)
relative = lambda path: os.path.relpath(path, package).replace(os.sep, '/')
framework = root / '🧰️framework/🔨️modules'
manifest = f'''[package]
workspace = "{relative(root)}"
name = "semio-os-vcs-entity-identity"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Controlled canonical version-graph identity commitments"

[package.metadata.semio]
role = "framework"
id = "os-vcs-entity-identity"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 65536
maxSourceFiles = 8
maxActiveLines = 2000

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-value = {{ path = "{relative(framework / '🌱️value/📦️packages/🦀️rust')}" }}
semio-framework-hash = {{ path = "{relative(framework / '🔏️hash/📦️packages/🦀️rust')}" }}
'''
(package / 'Cargo.toml').write_text(manifest)
workspace = root / 'Cargo.toml'
text = workspace.read_text()
member = str(package.relative_to(root))
anchor = '    "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/📦️packages/🦀️rust",'
assert anchor in text
text = text.replace(anchor, anchor + '\n    "' + member + '",', 1)
workspace.write_text(text)
manifest_path = vcs / '📦️packages/🦀️rust/Cargo.toml'
text = manifest_path.read_text()
anchor = '[dependencies]\n'
assert anchor in text
text = text.replace(anchor, anchor + 'semio-os-vcs-entity-identity = { path = "../../🚪️io/💾️binary/🪪️entity-identity/📦️packages/🦀️rust" }\n', 1)
manifest_path.write_text(text)
source = vcs / '🦀️.rs'
text = source.read_text()
old = '        #[path="🪪️entity-identity/🦀️.rs"]\n        pub mod entity_identity;'
assert old in text
source.write_text(text.replace(old, '        pub use semio_os_vcs_entity_identity as entity_identity;', 1))
