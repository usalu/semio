#!/usr/bin/env python3
"""🗑️ P9 prepared patch set (session 14b): removes trinity jack's LSP module `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/`.
Its Rust crate `semio-s-plugin-trinity-jack-lsp` was a "compatibility shim for existing launch targets until callers migrate"
(it only re-exported `dsl_lsp::lsp::{handle_json_rpc, LanguageSession}`); the Jack language lives in the kernel's `dsl_lsp`
+ Jack's `LanguageSpec`, which writer already drives in-process (`dsl::lsp::LanguageSession::open`). Its only caller, the
TS worker `@semio-tech/trinity-jack-lsp-worker`, imported a `JackLspSession` the shim no longer exports (a stale, gitignored
`pkg/` from 2026-08-06) and is imported by nothing. The callers that remain are registrations, migrated here in the same
set: the Cargo workspace member + lock entry, the bun workspaces + lock entries, the nx-generated launch pick lists, the
generated dependency ledger, the vite wasm stub's `JackLspSession`, and the three repo fixtures that enumerate the two
projects (rust-warnings native scope, nx-contract wasm outputs, vitest configuration owners 46 → 45).
Usage: p9-jack-lsp-removal.py --dry-run | --write"""
from p9_patch import ROOT, delete_tree, finish, replace

PART = "p9-jack-lsp-removal"
LSP = "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp"
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

delete_tree(PART, ROOT / LSP, 12)
replace(PART, ROOT / "Cargo.toml", f'    "{LSP}/📦️packages/🦀️rust",\n', "")
replace(PART, ROOT / "Cargo.lock", '[[package]]\nname = "semio-s-plugin-trinity-jack-lsp"\nversion = "0.1.0"\ndependencies = [\n "semio-framework-os-kernel",\n]\n\n', "")
replace(PART, ROOT / "package.json", f'    "{LSP}/📦️packages/🦀️rust",\n    "{LSP}/📦️packages/🟦️typescript",\n', "")
replace(PART, ROOT / "bun.lock", f'    "{LSP}/📦️packages/🟦️typescript": {{\n      "name": "@semio-tech/trinity-jack-lsp-worker",\n      "version": "0.1.0",\n      "devDependencies": {{\n        "typescript": "^5.9.3",\n        "vitest": "^4.0.17",\n      }},\n    }},\n    "{LSP}/📦️packages/🦀️rust": {{\n      "name": "@semio-tech/trinity-jack-lsp",\n    }},\n', "")
replace(PART, ROOT / "bun.lock", f'    "@semio-tech/trinity-jack-lsp": ["@semio-tech/trinity-jack-lsp@workspace:{LSP}/📦️packages/🦀️rust"],\n', "")
replace(PART, ROOT / "bun.lock", f'    "@semio-tech/trinity-jack-lsp-worker": ["@semio-tech/trinity-jack-lsp-worker@workspace:{LSP}/📦️packages/🟦️typescript"],\n', "")
replace(PART, ROOT / ".vscode/launch.json", '        "@semio-tech/trinity-jack-lsp",\n        "@semio-tech/trinity-jack-lsp-worker",\n', "", count=4)
replace(PART, ROOT / ".vscode/launch.json", '        "@semio-tech/puzzle-plugin",\n        "@semio-tech/trinity-jack-lsp",\n        "semio-framework-os-flow-core"\n', '        "@semio-tech/puzzle-plugin",\n        "semio-framework-os-flow-core"\n')
replace(PART, ROOT / "🔒️dependencies.json", f'        "{LSP}/📦️packages/🟦️typescript/package.json",\n', "", count=2)
replace(PART, ROOT / "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts", "export class TrinitySession {}\nexport class JackLspSession {}\n", "export class TrinitySession {}\n")
replace(PART, ROOT / "🧫️fixtures/🦀️rust-warnings/🔣️.json", '"semio-s-plugin-draw-fsm-macros", "semio-s-plugin-trinity-jack-lsp", ', '"semio-s-plugin-draw-fsm-macros", ')
replace(PART, ROOT / LIBRARY / "⚡️caching/🧫️fixtures/nx-contract/🔣️.json", '    {\n      "project": "@semio-tech/trinity-jack-lsp",\n      "output": "{projectRoot}/pkg"\n    },\n', "")
replace(PART, ROOT / LIBRARY / "🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json", f'    {{\n      "configurationRoot": "{LSP}/📦️packages/🟦️typescript",\n      "previousPath": "{LSP}/📦️packages/🟦️typescript/vitest.config.ts",\n      "ownerPath": "{LSP}/🧪️tests/🎚️config/🟦️.ts",\n      "expectedName": "@semio-tech/trinity-jack-lsp-worker",\n      "projectionSha256": "ed69bda3e001723e61f5e711868215f030f1a50e0f30c2ec2fd021201b59b1a0"\n    }},\n', "")
replace(PART, ROOT / LIBRARY / "🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts", "    expect(fixture.owners).toHaveLength(46);\n    expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(46);\n", "    expect(fixture.owners).toHaveLength(45);\n    expect(new Set(fixture.owners.map(({ ownerPath }) => ownerPath)).size).toBe(45);\n")

finish(__doc__)
