# 🪝 Anchored edits of the workspace named inputs (nx.json and the build policy that mirrors it): the `cargo` group that named the root Cargo files is replaced by the base of the generated `cargoToolchain` group.
from patchlib import chain
CARGO_OLD = '''    "cargo": [
      "{projectRoot}/**/*.rs",
      "{projectRoot}/Cargo.toml",
      "{workspaceRoot}/Cargo.toml",
      "{workspaceRoot}/.cargo/config.toml",
      "{workspaceRoot}/rust-toolchain.toml"
    ],
'''
CARGO_NEW = '''    "cargoToolchain": [
      "{workspaceRoot}/rustfmt.toml",
      "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️execution/📜️script.ts"
    ],
'''
def named(text): return chain(text, [("named-cargo", CARGO_OLD, CARGO_NEW)])

# 🎓️ The proctor authors its own Cargo sources group; the root Cargo files and the root lock in it are replaced by the generated resolution identity every Rust target carries.
PROCTOR_OLD = '''    "cargoSources": [
      "cargo",
      "^cargo",
      "sharedGlobals",
      "{workspaceRoot}/Cargo.lock",
      "{workspaceRoot}/🎓️teaching/🛂️proctor/**/*.rs",'''
PROCTOR_NEW = '''    "cargoSources": [
      "cargoToolchain",
      "sharedGlobals",
      "{workspaceRoot}/🎓️teaching/🛂️proctor/**/*.rs",'''
def proctor(text): return chain(text, [("proctor", PROCTOR_OLD, PROCTOR_NEW)])
