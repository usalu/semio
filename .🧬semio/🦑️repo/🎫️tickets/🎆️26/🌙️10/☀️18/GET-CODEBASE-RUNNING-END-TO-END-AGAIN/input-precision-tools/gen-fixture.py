#!/usr/bin/env python3
# 🧫 Writes the language-agnostic fixture of the input precision engine (staged/fixture.json): the inputs of every case; expectations are frozen by `freeze-fixture.ts` after review.
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
W = "{workspaceRoot}/"
DISPATCH = '''export class Verify {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "alpha") {
      const { runCargo } = await import("../runner/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "crate-a", "--lib", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "beta" && segments[1] === "native") {
      const root = join(this.root, "crates/b");
      await runCargo(["test", "--lib"], root);
      const oracle = join(this.root, "tests", "unit") + "/🟦️.ts";
      return;
    }
    if (segments[0] === "gamma") {
      const fixture = join(this.root, "data/🔣️.json");
      await runCargo(["test"], this.root);
      return;
    }
    if (segments[0] === "delta") return runPure();
  }
}
'''
files = {
    ".ticket/draft/Cargo.toml": '[package]\nname = "crate-a"\nversion = "9.9.9"\n',
    "Cargo.toml": '[workspace]\nmembers = [\n  "crates/a",\n  "crates/b",\n]\n',
    "crates/a/Cargo.toml": '[package]\nname = "crate-a"\nversion = "0.1.0"\n\n[dependencies]\nb = { path = "../b", package = "crate-b" }\n',
    "crates/a/src/lib.rs": "pub fn a() {}\n",
    "crates/b/Cargo.toml": '[package]\nname = "crate-b"\nversion = "0.1.0"\n',
    "crates/b/src/lib.rs": "pub fn b() {}\n",
    "crates/c/Cargo.toml": '[package]\nname = "crate-c"\nversion = "0.1.0"\n',
    "tools/dispatch.ts": DISPATCH,
    "runner/🟦️.ts": "export const runCargo = () => {};\n",
    "tests/unit/🟦️.ts": 'import { helper } from "../helper.ts";\nexport const t = helper;\n',
    "tests/helper.ts": 'export const helper = 1;\n',
    "data/🔣️.json": "{}\n",
    "owner/📋️project.json": "{}\n",
    "owner/Cargo.toml": '[package]\nname = "owner-crate"\nversion = "0.1.0"\n',
    "owner/📜️script.ts": 'import { own } from "./own.ts";\nexport const x = own;\n',
    "owner/own.ts": "export const own = 1;\n",
    "owner/src/lib.rs": "pub fn own() {}\n",
    "app/tsconfig.json": '// program root\n{\n  "extends": "../tsconfig.base.json",\n  "compilerOptions": { "strict": true, },\n  "include": ["src/**/*.ts"],\n  "exclude": ["src/skip/**"]\n}\n',
    "tsconfig.base.json": '{ "compilerOptions": { "module": "ESNext", "moduleResolution": "bundler", "allowImportingTsExtensions": true, "noEmit": true, "target": "ESNext", "skipLibCheck": true, "types": [] } }\n',
    "app/src/main.ts": 'import { util } from "./util.ts";\nimport type { Shape } from "../../shared/types.ts";\nexport const main: Shape = util;\n',
    "app/src/util.ts": 'export const util = { size: 1 };\n',
    "app/src/feature.ts": 'import { lib } from "./lib";\nexport const feature = lib;\n',
    "app/src/lib/🟦️.ts": 'export const lib = 1;\n',
    "app/src/skip/x.ts": 'export const skipped = 1;\n',
    "shared/types.ts": 'export type Shape = { size: number };\n',
    "shared/package.json": '{ "name": "shared" }\n',
    "unrelated/big.ts": 'export const big = 1;\n',
}
kinds = {"rust-source": {"emoji": "🦀️", "extensionChains": [".rs"]}, "typescript-source": {"emoji": "🟦️", "extensionChains": [".ts", ".tsx"]}, "json": {"emoji": "🔣️", "extensionChains": [".json"]}}
exclusions = [W.replace("/", "") and "!" + W + "**/node_modules/**", "!" + W + "**/target/**"]
document = {
    "version": 1,
    "retained": ["sharedGlobals", "{projectRoot}/📋️project.json"],
    "messages": {"languages": ["en", "de"], "texts": {"deriving": {"en": "Deriving {tsconfig}", "de": "Leite {tsconfig} ab"}, "progress": {"en": "{tsconfig}: {files} visited, {derived} analysed, {reused} reused", "de": "{tsconfig}: {files} besucht, {derived} analysiert, {reused} wiederverwendet"}, "done": {"en": "{tsconfig} done in {seconds} s", "de": "{tsconfig} fertig in {seconds} s"}}},
    "exclusions": exclusions,
    "profiles": {
        "discovery": {"description": "Package manifests of the fixture.", "reads": [{"globs": [W + "**/{Cargo.toml,📋️project.json}"]}]},
        "sources": {"description": "Rust and TypeScript sources of the fixture.", "reads": [{"kinds": ["rust-source", "typescript-source"], "under": ["."]}, {"kinds": ["json"], "under": ["data"], "named": True}]},
    },
    "routes": [{"words": ["verify", "owned"], "project": "owner", "target": "verify-owned", "root": "owner"}],
    "rules": [
        {"id": "delegate", "evidence": "static", "description": "Owner delegation.", "select": {"project": "root", "commands": [["./script.ts", "verify", "owned"]]}, "replaces": ["default"], "reads": [{"profile": "discovery"}, {"owner": "route"}]},
        {"id": "laws", "evidence": "static", "description": "Dispatcher laws.", "select": {"project": "root", "commands": [["./script.ts", "verify"]]}, "replaces": ["default", "nativeTestSources"], "reads": [{"profile": "discovery"}, {"dispatch": {"source": "tools/dispatch.ts", "after": ["verify"]}}, {"cargo": {"packages": ["crate-c"]}}]},
        {"id": "scan", "evidence": "bound", "description": "Scanner.", "select": {"project": "root", "targets": ["scan"]}, "replaces": ["default"], "reads": [{"profile": "sources"}, {"input": "schemaSources"}]},
        {"id": "program", "evidence": "static", "description": "Typecheck program.", "select": {"project": "app", "targets": ["typecheck"]}, "replaces": ["default", "^default"], "reads": [{"program": {"tsconfig": "app/tsconfig.json"}}]},
        {"id": "script", "evidence": "static", "description": "Script literals.", "select": {"project": "app", "targets": ["script"]}, "replaces": ["default"], "reads": [{"script": {"source": "tools/dispatch.ts"}}]},
        {"id": "none", "evidence": "static", "description": "Reads nothing.", "select": {"project": "root", "targets": ["quiet"]}, "replaces": ["default"], "reads": []},
    ],
}
fixture = {
    "globs": [
        {"pattern": "**/📦️packages/*/{Cargo.toml,package.json}", "path": "a/b/📦️packages/🦀️rust/Cargo.toml", "match": True},
        {"pattern": "**/📦️packages/*/{Cargo.toml,package.json}", "path": "📦️packages/🦀️rust/package.json", "match": True},
        {"pattern": "**/📦️packages/*/{Cargo.toml,package.json}", "path": "a/📦️packages/🦀️rust/deep/Cargo.toml", "match": False},
        {"pattern": "src/**/*.ts", "path": "src/a/b/c.ts", "match": True},
        {"pattern": "src/**/*.ts", "path": "src/c.tsx", "match": False},
        {"pattern": "**/*.{ts,tsx}", "path": "x.tsx", "match": True},
        {"pattern": "a?c", "path": "abc", "match": True},
        {"pattern": "a?c", "path": "a/c", "match": False},
        {"pattern": "**/🔣️.json", "path": "🔣️.json", "match": True},
        {"pattern": "x/*.rs", "path": "x/y/z.rs", "match": False},
        {"pattern": "a.b+c", "path": "a.b+c", "match": True},
        {"pattern": "a.b+c", "path": "axb+c", "match": False},
    ],
    "commands": [
        {"command": "bun ./📜️script.ts verify a b", "words": {"script": "./📜️script.ts", "words": ["verify", "a", "b"]}},
        {"command": 'bun "./dir with space/📜️script.ts" test', "words": {"script": "./dir with space/📜️script.ts", "words": ["test"]}},
        {"command": "cargo build", "words": None},
        {"command": "bun run build", "words": None},
    ],
    "jsonc": [
        {"text": '{ // c\n "a": [1,2,], /* x */ "b": "//not", "c": "/*no*/" }', "value": {"a": [1, 2], "b": "//not", "c": "/*no*/"}},
        {"text": '{"extends": "./x.json", /* k */ "include": ["a",],}', "value": {"extends": "./x.json", "include": ["a"]}},
    ],
    "program": {"tsconfig": "app/tsconfig.json", "steps": [
        {"name": "cold", "edits": []},
        {"name": "warm", "edits": []},
        {"name": "edit-body", "edits": [{"path": "app/src/util.ts", "text": "export const util = { size: 2 };\n"}]},
        {"name": "new-import", "edits": [{"path": "app/src/util.ts", "text": 'import { extra } from "./extra.ts";\nexport const util = { size: 2, extra };\n'}, {"path": "app/src/extra.ts", "text": "export const extra = 1;\n"}]},
        {"name": "shadow-resolution", "edits": [{"path": "app/src/lib.ts", "text": "export const lib = 2;\n"}]},
        {"name": "remove-file", "edits": [{"path": "app/src/extra.ts", "text": None}, {"path": "app/src/util.ts", "text": "export const util = { size: 3 };\n"}]},
        {"name": "corrupt-receipt", "edits": [], "corruptReceipt": True},
        {"name": "tsconfig-change", "edits": [{"path": "app/tsconfig.json", "text": '{ "extends": "../tsconfig.base.json", "include": ["src/**/*.ts"], "exclude": ["src/skip/**", "src/never/**"] }\n'}]},
        {"name": "cancel", "edits": [{"path": "app/src/util.ts", "text": "export const util = { size: 4 };\n"}], "corruptReceipt": True, "cancelAfter": 3},
        {"name": "resume", "edits": []},
    ]},
    "messages": [
        {"environment": {"SEMIO_LANGUAGE": "de", "LANG": "en_US.UTF-8"}, "language": "de"},
        {"environment": {"LANG": "de_CH.UTF-8"}, "language": "de"},
        {"environment": {"LC_ALL": "fr_FR"}, "language": "en"},
        {"environment": {}, "language": "en"},
    ],
    "dispatch": [{"source": DISPATCH}],
    "packageNames": [
        {"manifest": '[package]\nname = "p"\n[dependencies]\nname = "x"\n', "name": "p"},
        {"manifest": '[dependencies]\nname = "x"\n[package]\nversion = "1"\nname = "q"\n', "name": "q"},
        {"manifest": '[workspace]\nmembers = []\n', "name": None},
    ],
    "workspace": {
        "files": files,
        "kinds": kinds,
        "cargoRoots": {"crates/a": ["crates/a", "crates/b"], "crates/b": ["crates/b"], "crates/c": ["crates/c"], "owner": ["owner"]},
        "cargoSources": {"crates/a": ["{workspaceRoot}/crates/a/src/lib.rs"], "crates/b": ["{workspaceRoot}/crates/b/src/lib.rs"]},
        "cargoToolchain": ["rust-toolchain.toml"],
    },
    "document": document,
    "derive": [
        {"identity": {"project": "root", "target": "verify-owned", "command": "bun ./script.ts verify owned"}},
        {"identity": {"project": "root", "target": "alpha", "command": "bun ./script.ts verify alpha"}},
        {"identity": {"project": "root", "target": "beta-native", "command": "bun ./script.ts verify beta native"}},
        {"identity": {"project": "root", "target": "gamma", "command": "bun ./script.ts verify gamma"}},
        {"identity": {"project": "root", "target": "delta", "command": "bun ./script.ts verify delta"}},
        {"identity": {"project": "root", "target": "scan", "command": "bun ./script.ts scan"}},
        {"identity": {"project": "root", "target": "quiet", "command": "bun ./script.ts quiet"}},
        {"identity": {"project": "app", "target": "typecheck", "command": "bun ./script.ts typecheck"}},
        {"identity": {"project": "app", "target": "script", "command": "bun ./script.ts test"}},
    ],
    "unselected": [
        {"project": "other", "target": "x", "command": "bun ./script.ts verify alpha"},
        {"project": "root", "target": "build", "command": "bun ./script.ts build"},
    ],
    "refusals": [
        {"kind": "document", "mutate": "wholeRepository", "message": "whole repository"},
        {"kind": "document", "mutate": "wholeRepositoryDot", "message": "whole repository"},
        {"kind": "document", "mutate": "unknownProfile", "message": "unknown profile"},
        {"kind": "document", "mutate": "duplicateRule", "message": "declared twice"},
        {"kind": "document", "mutate": "unknownKey", "message": "refused"},
        {"kind": "derive", "identity": {"project": "root", "target": "unknown-crate", "command": "bun ./script.ts verify alpha"}, "rule": "unknown-crate", "reads": [{"cargo": {"packages": ["no-such-crate"]}}], "message": "Unknown Cargo package"},
        {"kind": "derive", "identity": {"project": "root", "target": "root-crate", "command": "bun ./script.ts verify alpha"}, "rule": "root-crate", "reads": [{"cargo": {"directories": ["."]}}], "message": "selects the whole repository"},
        {"kind": "derive", "identity": {"project": "root", "target": "no-branch", "command": "bun ./script.ts verify missing"}, "rule": "no-branch", "reads": [{"dispatch": {"source": "tools/dispatch.ts", "after": ["verify"]}}], "message": "no branch"},
        {"kind": "derive", "identity": {"project": "root", "target": "unbounded-cargo", "command": "bun ./script.ts verify gamma"}, "rule": "unbounded-cargo", "reads": [{"dispatch": {"source": "tools/dispatch.ts", "after": ["verify"]}}], "message": "runs cargo without a literal package"},
    ],
    "expect": {},
}
out = sys.argv[1] if len(sys.argv) > 1 else HERE + "/staged2/fixture.json"
if os.path.exists(out):
    try: fixture["expect"] = json.load(open(out)).get("expect", {})
    except Exception: pass
json.dump(fixture, open(out, "w"), ensure_ascii=False, indent=1)
open(out, "a").write("\n")
print("fixture written", out)
