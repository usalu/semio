# Current Lower Drawing Native Execution

The original full 48-provider/family/grammar/assembly route remains unchanged and pending. A separate additive full lower Drawing family route will retain the original oracles feature and900,000ms native budget. This family depends only on the dependency-free repository test host and private optional dxf; no Kernel/UI/Renderer/DSL consumer is selected.

## Full Before 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📜️script.ts

SHA256 `fcd39877a26b059e6879c80d18b5156f9ce8126995afe28ce0b0fa0c023d3882`

```typescript

#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import contract from "./🧫️fixtures/🧩️composition/🔣️.json";
import { resolve } from "node:path";

/** 🧩️ Runs portable laws against the canonical oracle's actual source and contribution graph. */
class CompositionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🧩️composition/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🖊️ Proves the actual shared DXF reader's ownership and preserved definition bodies. */
class DrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🖊️drawing-reader/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🧫️ Checks exact private reader ownership and unchanged frozen caller content. */
class PrivateReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🧫️private-reader/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🦀️ Executes every original provider, family, neutral-law and full assembly unit cohort. */
class NativeOracleTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw new Error("The complete native oracle gate accepts no cohort or scenario filter.");
    const root = resolve(this.root, "../../../..");
    const packages = [contract.neutralLaw.source.replace(/\/🦀️\.rs$/u, "/📦️packages/🦀️rust"), ...contract.providers.map(({ package: pkg }) => pkg.path), ...contract.families.map(({ package: pkg }) => pkg.path), contract.grammar.package.path, contract.package.path];
    for (const path of packages) {
      console.info(`🦀️ Oracle unit cohort: ${path}`);
      const features = path === packages[0] ? [] : ["--features", "oracles"];
      await runRepositoryTestCommand("cargo", ["test", "--manifest-path", resolve(root, path, "Cargo.toml"), ...features], { cwd: root, budgetMs: 900_000 });
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-private-reader", PrivateReaderTestScript).register("test-composition", CompositionTestScript).register("test-drawing-reader", DrawingReaderTestScript).register("test-native-oracles", NativeOracleTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test-composition" });

```

## Full Before 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/📋️project.json

SHA256 `43121e26f3bbc30e00477861c08ba9168e8e4ab9251e99f90aa7bdb896efe942`

```json

{
  "name": "@semio-tech/hub-stdio-test-oracle",
  "root": "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles",
  "projectType": "library",
  "tags": [
    "lang:typescript",
    "role:hub",
    "test-only"
  ],
  "targets": {
    "test-drawing-reader": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun 📜️script.ts test-drawing-reader",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test-composition": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun 📜️script.ts test-composition",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test-native-oracles": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun 📜️script.ts test-native-oracles",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    },
    "test-private-reader": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "command": "bun 📜️script.ts test-private-reader",
        "cwd": "{projectRoot}",
        "forwardAllArgs": true
      }
    }
  }
}

```

## Full Before 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/package.json

SHA256 `97b8aaccb0fbef44cc5af5cdca50f91a8cb5a5f7ac5e2d081ff57bdada4a4a17`

```json

{
  "name": "@semio-tech/hub-stdio-test-oracle",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "test-drawing-reader": "bun nx run @semio-tech/hub-stdio-test-oracle:test-drawing-reader",
    "test-composition": "bun nx run @semio-tech/hub-stdio-test-oracle:test-composition",
    "test-native-oracles": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-oracles",
    "test-private-reader": "bun nx run @semio-tech/hub-stdio-test-oracle:test-private-reader"
  },
  "nx": {
    "includedScripts": []
  }
}

```

## Full Before 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts

SHA256 `29122cb8688f8ca62c3c216d59617ee96b089d0c1a24f800ab71bc6387bf01aa`

```typescript

import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { inspectRustCompileReferences, rustTokens, rustTokenPairs } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import contract from "../../🧫️fixtures/🖊️drawing-reader/🔣️.json";
import { extractedDxfMutationSource } from "./🧩️preservation/🟦️.ts";
import schema from "../../🧬️schema/🖊️drawing-reader/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");
const familyManifest = `${contract.owner}/📦️packages/🦀️rust/Cargo.toml`;

test("owned schema and independent Ajv close exact reader authorities and hostile cases", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const [value, expected] of [[contract, true], [{ ...contract, unknown: true }, false], [{ ...contract, owner: "🌎️hub" }, false], [{ ...contract, externalDependency: { version: "0.6", optional: false } }, false], [{ ...contract, cases: [{ id: "silent", input: "x" }] }, false]] as const) {
    expect(validateJsonSchemaSubset(schema, value).length === 0).toBe(expected);
    expect(validate(value)).toBe(expected);
  }
  for (const original of contract.originals) {
    expect(digest(original.source), original.path).toBe(original.sha256);
    expect(Buffer.byteLength(original.source)).toBe(original.bytes);
  }
});

test("the actual artifact-free family owns every original semantic reader body exactly", () => {
  expect(existsSync(resolve(root, familyManifest)), familyManifest).toBe(true);
  for (const row of contract.functions) {
    const source = read(row.destination), tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
    const start = tokens.findIndex(({ text }, index) => text === "fn" && tokens[index + 1]?.text === row.name);
    expect(start, row.name).toBeGreaterThanOrEqual(0);
    let body = start;
    while (tokens[body]?.text !== "{") body++;
    const actual = source.slice(tokens[body]!.start, tokens[pairs.get(body)!]!.end);
    expect(digest(actual), row.name).toBe(row.sha256);
    expect(inspectRustCompileReferences(source).some(({ path }) => path.includes("🗿️artifacts")), row.destination).toBe(false);
  }
  const raw = read(familyManifest), cargo = toml.parse(raw) as any;
  expect(Bun.TOML.parse(raw)).toEqual(cargo);
  expect(cargo.dependencies.dxf).toEqual(contract.externalDependency);
  expect(Object.keys(cargo.dependencies).sort()).toEqual(["dxf", "semio-repo-test-host"]);
  expect(cargo.features.oracles).toEqual(["dep:dxf"]);
});

test("both actual providers select the lower reader without sibling or higher assembly edges", () => {
  for (const original of contract.originals.filter(({ path }) => path.endsWith("Cargo.toml"))) {
    const raw = read(original.path), cargo = toml.parse(raw) as any;
    expect(Bun.TOML.parse(raw)).toEqual(cargo);
    const dependency = cargo.dependencies[contract.package];
    expect(dependency, original.path).toBeDefined();
    expect(resolve(root, original.path, "..", dependency.path)).toBe(resolve(root, familyManifest, ".."));
    expect(cargo.features.oracles).toContain(`${contract.package}/oracles`);
    expect(Object.keys(cargo.dependencies).some((name) => name.startsWith("semio-s-artifact-") || name.startsWith("semio-hub-"))).toBe(false);
  }
  const note = contract.originals.find(({ path }) => path.includes("/🗒️note/") && path.endsWith("/🦀️.rs"))!;
  const source = read(note.path);
  expect(source).toContain(`${contract.library}::project_dxf_r12(bytes)`);
  expect(source).not.toContain("crate::artifacts::dxf");
  expect(relative(root, resolve(root, contract.owner))).not.toContain("🗿️artifacts");
});

test("the entire retained mutation source and original reader callers preserve exact fresh bytes", () => {
  const dxf = contract.originals.find(({ path }) => path === contract.functions[0]!.source)!;
  const support = relative(resolve(root, dxf.path, ".."), resolve(root, contract.owner, "🧰️support/🦀️.rs")).replaceAll("\\", "/");
  const mount = `\n\n#[cfg(feature = "oracles")]\n#[path = ${JSON.stringify(support)}]\nmod reference_support;`;
  const current = read(dxf.path).replace(mount, "").replace("\n    use super::reference_support::{load, point_json, obj};", "");
  expect(current).toBe(extractedDxfMutationSource(dxf.source));
  expect(read(dxf.path)).not.toContain("pub fn project_dxf_r12");
  for (const row of contract.originals.filter(({ path }) => path.endsWith(".rs") && path !== dxf.path)) {
    let expected = row.source;
    if (row.path.includes("/🗒️note/")) expected = expected.replaceAll("crate::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12", `${contract.library}::project_dxf_r12`);
    else if (row.path.includes("/🧪️tests/")) {
      expected = expected.replaceAll(", project_dxf_r12}", "}").replaceAll("use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;", `use ${contract.library}::project_dxf_r12;`);
      if (!expected.includes(`use ${contract.library}::project_dxf_r12;`)) expected = `use ${contract.library}::project_dxf_r12;\n${expected}`;
    }
    expect(read(row.path), row.path).toBe(expected);
  }
  expect(JSON.parse(read(`${contract.owner}/🧫️fixtures/🖊️semantic/🔣️.json`))).toEqual(contract.cases);
});

test("the actual private shared helpers and public byte boundary carry no foreign types", () => {
  const support = read(`${contract.owner}/🧰️support/🦀️.rs`);
  for (const name of ["load", "point_json", "obj"]) expect(support).toContain(`pub(super) fn ${name}`);
  expect(support).not.toMatch(/pub (?:fn|struct|enum|type|use)/u);
  const source = read(`${contract.owner}/🦀️.rs`);
  expect(source).toContain("pub use reader::project_dxf_r12;");
  for (const parameter of ["bytes: &[u8]", "_bytes: &[u8]"]) expect(source).toContain(`pub fn project_dxf_r12(${parameter}) -> Result<Json, String>`);
  const original = contract.originals.find(({ path }) => path === contract.functions[0]!.source)!.source;
  const refusal = /pub fn project_dxf_r12\(_bytes: &\[u8\]\) -> Result<Json, String> (\{[^}]*\})/u.exec(original)![1]!;
  expect(source).toContain(refusal);
  const mounts = contract.originals.filter(({ path }) => path === contract.functions[0]!.source).flatMap(({ path }) => inspectRustCompileReferences(read(path)).filter((row) => row.path.includes("🧰️support")).map((row) => resolve(root, path, "..", row.path)));
  expect(mounts).toEqual([resolve(root, contract.owner, "🧰️support/🦀️.rs")]);
  expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🧰️support")).map(({ path }) => resolve(root, contract.owner, path))).toEqual(mounts);
});

```

## Full Before .vscode/🧩️launch.seed.jsonc

SHA256 `6de7b2c47d26dc9f7c23c2181ff16288810e36236ed4027b8e823d1b2b7b2778`

```json

{
  "version": "0.2.0",
  "configurations": [
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.693}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.692}},

    {"name":"⚖️test-snapshot-sqlite📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.691}},
    {"name":"⚖️test-snapshot-sqlite-source📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.69109999999995}},
    {"name":"⚖️test-snapshot-sqlite-native📏️layout🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/layout-layout-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6912}},
    {"name":"⚖️test-quick📖️pdf🌊️structural🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pdf-rs:test --skip-nx-cache -- quick --lib lossless_structural_flow_law_bachelor_thesis_snapshot_mutation_diff_io_and_inverse --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6904}},
    {"name":"⚖️sqlite-source🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.688}},
    {"name":"⚖️sqlite-public🧱️🖐️5d","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/block-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.689}},
    {"name":"⚖️test-quick📕️xlsx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-xlsx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6901}},
    {"name":"⚖️test-quick📜️docx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-docx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6902}},
    {"name":"⚖️test-quick📽️pptx🦀️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-pptx-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.6903}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.685}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.686}},
    {"name":"⚖️gate🏗️fem🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.687}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.684}},
    {"name":"⚖️gate🧩️puzzle🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.683}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.682}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.681}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.679}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.678}},
    {"name":"⚖️gate🏗️fem🧊️3d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.677}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.674}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.675}},
    {"name":"⚖️gate🏗️fem◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"NX_DAEMON":"false"},"presentation":{"group":"4_gate","order":408.676}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.673}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.672}},
    {"name":"⚖️gate🏗️fem◻️2d🪶️sqlite","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/fem-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.671}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.670}},
    {"name":"⚖️gate🧩️puzzle🖐️5d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-5d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.669}},
    {"name":"⚖️gate📸️remodel📸️remodeling🪶️sqlite🔍️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/remodel-remodeling-rs:verify-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.668}},
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.65099999999995
      }
    },
    {
      "name": "⚖️gate📸️remodel📸️remodeling🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:test-snapshot-sqlite-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.652
      }
    },
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.623}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.624}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.625}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🏗️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.626}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🔍️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.627}},
    {"name":"⚖️gate🔋️energy🔋️model🪶️sqlite🧪️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model-rs:test --skip-nx-cache -- quick --no-fail-fast","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.628}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:build --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.629}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:check --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.63}},
    {"name":"⚖️gate🔋️energy🔋️model🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/energy-model:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.631}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.632}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.633}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.634}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.635}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.636}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.637}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.638}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.639}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.64}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.641}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.642}},
    {"name":"⚖️gate🀄️wfc◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.643}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.644}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.645}},
    {"name":"⚖️gate🀄️wfc🧊️3d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.646}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.653}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.654}},
    {"name":"⚖️gate🀄️wfc🖼️bitmap🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-bitmap:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.655}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.656}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.657}},
    {"name":"⚖️gate🀄️wfc🔲️grid2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.658}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.659}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.66}},
    {"name":"⚖️gate🀄️wfc🧱️grid3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-grid3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.661}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.662}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.663}},
    {"name":"⚖️gate🀄️wfc◻️2d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-2d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.664}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️build","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:build --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.665}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️check","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:check --skip-nx-cache","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.666}},
    {"name":"⚖️gate🀄️wfc🧊️3d🟦️test","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/wfc-3d:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.667}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.619}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.62}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-forms-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.621}},
    {"name":"⚖️gate📋️forms📋️forms🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/forms-js:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick","NX_DAEMON":"false","NX_ISOLATE_PLUGINS":"false"},"presentation":{"group":"4_gate","order":408.622}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.596}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.597}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.598}},
    {"name":"⚖️gate🗄️stdio🧿️semio🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-semio:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.599}},

    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.592}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59299999999996}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.594}},
    {"name":"⚖️gate📐️cad📐️cad🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/cad-cad-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59499999999997}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.588}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-native --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.589}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg-rs:test-snapshot-sqlite-source --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.59000000000003}},
    {"name":"⚖️gate🗄️stdio🖊️dwg🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-dwg:test --skip-nx-cache","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.591}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.584}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.585}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.586}},
    {"name":"⚖️gate🔁️workflow🔁️workflow🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-workflow-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.587}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58099999999996}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.582}},
    {"name":"⚖️gate🔁️workflow🏃️run🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/framework-workflow-run-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.58299999999997}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.576}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.577}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.578}},
    {"name":"⚖️gate💠️lowpoly🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/lowpoly-lowpoly:test","cwd":"${workspaceFolder}","env":{"SEMIO_TEST_LEVEL":"quick"},"presentation":{"group":"4_gate","order":408.579}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.572}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.573}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.574}},
    {"name":"⚖️gate🧩️puzzle◻️2d🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/puzzle-2d-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.575}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.568}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.569}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.57}},
    {"name":"⚖️gate💡️reasoning🔌️wires🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/reasoning-wires:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.571}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.564}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.565}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.566}},
    {"name":"⚖️gate🪐️space🪐️space🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-space:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.567}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.56}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.561}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.562}},
    {"name":"⚖️gate🪵️sourcing🗂️curation🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sourcing-curation-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.563}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.55}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.551}},
    {"name":"⚖️gate🪐️space🏠️home🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/space-home-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.552}},
    {
      "name": "⚖️gate🪐️space🏠️home🪶️sqlite🟦️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.553
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.6
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.601
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60200000000003
      }
    },
    {
      "name": "⚖️gate🏛️architect🏛️program🪶️sqlite🏭️public",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.603
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.604
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60499999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio📼️avi🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-avi-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.606
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60699999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.608
      }
    },
    {
      "name": "⚖️gate🗄️stdio💬️bcf🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-bcf-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.609
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.60999999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.611
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎵️mp3🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp3-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61199999999997
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.613
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.614
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎥️mp4🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-mp4-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.615
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.616
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.61699999999996
      }
    },
    {
      "name": "⚖️gate🗄️stdio🌦️epw🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-epw-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "quick",
        "NX_DAEMON": "false",
        "NX_ISOLATE_PLUGINS": "false"
      },
      "presentation": {
        "group": "4_gate",
        "order": 408.618
      }
    },
    {"name": "⚖️test-controlled-encoding🌱️value🦀️", "type": "node-terminal", "request": "launch", "command": "bun nx run @semio-tech/value-rs:test-controlled-encoding", "cwd": "${workspaceFolder}", "presentation": {"group": "4_gate", "order": 900.033208}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:verify-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.543}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.540}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.541}},
    {"name":"⚖️gate🎥️shooting🎥️shooting🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/shooting-shooting-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.542}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.530}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.531}},
    {"name":"⚖️gate🎞️animate🎬️presentation🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/animate-presentation-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.532}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.523}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.52}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.521}},
    {"name":"⚖️gate🎬️sequence🎬️sequence🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/sequence-sequence-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.522}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.5}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.501}},
    {"name":"⚖️gate🗄️stdio🔺️stl🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-stl-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.502}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.503}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.504}},
    {"name":"⚖️gate🗄️stdio🗽️obj🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-obj-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.505}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.506}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.507}},
    {"name":"⚖️gate🗄️stdio🧱️ply🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/stdio-ply-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.508}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️public","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-js:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.494}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.491}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.492}},
    {"name":"⚖️gate🌿️vcs🌿️vcs🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/vcs-vcs-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.493}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.465}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.466}},
    {"name":"⚖️gate📕️norm🏭️vdi3805🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.467}},
    {"name":"⚖️build📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.468}},
    {"name":"⚖️check📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.469}},
    {"name":"⚖️test📕️norm🏭️vdi3805🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-vdi3805:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.47}},
    {"name":"⚖️gate📕️norm🧬️contract🌱️bytes🖥️bounded","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-artifact-contract-rs:test-byte-property","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.471}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.453}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.454}},
    {"name":"⚖️gate📕️norm🫨️en1998🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.455}},
    {"name":"⚖️build📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.456}},
    {"name":"⚖️check📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.457}},
    {"name":"⚖️test📕️norm🫨️en1998🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1998:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.458}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.447}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.448}},
    {"name":"⚖️gate📕️norm🧩️en1994🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.449}},
    {"name":"⚖️build📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.45}},
    {"name":"⚖️check📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.451}},
    {"name":"⚖️test📕️norm🧩️en1994🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-en1994:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.452}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.417}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.418}},
    {"name":"⚖️gate📕️norm🧱️din4108🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.419}},
    {"name":"⚖️build📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.420}},
    {"name":"⚖️check📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.421}},
    {"name":"⚖️test📕️norm🧱️din4108🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din4108:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.422}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🧪️all","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.435}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🦀️native","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-native","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.436}},
    {"name":"⚖️gate📕️norm⚡️din18599🪶️sqlite🟦️source","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599-rs:test-snapshot-sqlite-source","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.437}},
    {"name":"⚖️build📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:build","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.438}},
    {"name":"⚖️check📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:check","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.439}},
    {"name":"⚖️test📕️norm⚡️din18599🟦️","type":"node-terminal","request":"launch","command":"bun nx run @semio-tech/norm-din18599:test","cwd":"${workspaceFolder}","presentation":{"group":"4_gate","order":408.440}},

    {
      "name": "⚖️build📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.414
      }
    },
    {
      "name": "⚖️check📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.415
      }
    },
    {
      "name": "⚖️test📕️norm🌬️din16798🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.416
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.411
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.412
      }
    },
    {
      "name": "⚖️gate📕️norm🌬️din16798🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-din16798-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.413
      }
    },
    {
      "name": "⚖️build📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.402
      }
    },
    {
      "name": "⚖️check📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.403
      }
    },
    {
      "name": "⚖️test📕️norm🪵️en1995🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.404
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.399
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.4
      }
    },
    {
      "name": "⚖️gate📕️norm🪵️en1995🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1995-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.401
      }
    },
    {
      "name": "⚖️build📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.396
      }
    },
    {
      "name": "⚖️check📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.397
      }
    },
    {
      "name": "⚖️test📕️norm🪨️en1996🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.398
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.393
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.394
      }
    },
    {
      "name": "⚖️gate📕️norm🪨️en1996🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.395
      }
    },
    {
      "name": "⚖️build📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "⚖️check📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.391
      }
    },
    {
      "name": "⚖️test📕️norm🏋️en1991🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.392
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.387
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.388
      }
    },
    {
      "name": "⚖️gate📕️norm🏋️en1991🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1991-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.389
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.381
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.382
      }
    },
    {
      "name": "⚖️gate📕️norm⚖️en1990🪶️sqlite🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990-rs:test-snapshot-sqlite-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.383
      }
    },
    {
      "name": "⚖️build📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.384
      }
    },
    {
      "name": "⚖️check📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.385
      }
    },
    {
      "name": "⚖️test📕️norm⚖️en1990🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1990:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.386
      }
    },
    {
      "name": "🦑️Repo 📋️native owner command 🧪️policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-owner-command-policy --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05751
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command dispatch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-command-dispatch --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05752
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🦀️native 🧭️command types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:check-command-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05753
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧪️portable schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-schema --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05754
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "◻️2D 🧮️compute 🧭️contract types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05755
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/2d-compute:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05787
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057871
      }
    },
    {
      "name": "🧪️test◻️2d🧮️compute🔏️keys",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-2d:test-compute-keys --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057872
      }
    },
    {
      "name": "🌐️UI locale 🦀️native owner laws",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05808
      }
    },
    {
      "name": "🧪️test🖱️ui🧊️feature-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-feature-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05812
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1814
      }
    },
    {
      "name": "⚖️check↔️paged-history-stack🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:paged-history-stack-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1815
      }
    },
    {
      "name": "⚖️check🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1811
      }
    },
    {
      "name": "⚖️test🔐️hub-auth-client🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1812
      }
    },
    {
      "name": "⚖️test-source🔐️hub-auth-client🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-auth-client-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1813
      }
    },
    {
      "name": "⚖️check👁️source-watch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:source-watch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.181
      }
    },
    {
      "name": "⚖️verify🔗️puzzle-browser-contribution🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-browser-contribution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1809
      }
    },
    {
      "name": "⚖️check📍️distribution-output🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:distribution-output-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1808
      }
    },
    {
      "name": "⚖️test-component-owners📇️registry🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-component-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1807
      }
    },
    {
      "name": "⚖️test-artifact-kind🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05665
      }
    },
    {
      "name": "⚖️test-artifact-kind-source🧰️framework🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-rs:test-artifact-kind-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056651
      }
    },
    {
      "name": "⚖️check🖍️draw-guest-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-plugin:guest-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2647
      }
    },
    {
      "name": "⚖️check🧩️hub-component-codecs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2648
      }
    },
    {
      "name": "⚖️check⏱️hub-gis-codec-budget🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:component-codec-budget-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2649
      }
    },
    {
      "name": "⚖️check🧫️host-fixture🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2638
      }
    },
    {
      "name": "⚖️build🧫️host-fixture-component🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host-fixture:component-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2639
      }
    },
    {
      "name": "⚖️check🧫️host-owned-instance🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin-host:owned-instance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️check🗒️note-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️check🗒️note-sqlite-snapshot-guest🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-sqlite-snapshot-guest",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "⚖️check🗺️gis-mcp-inference-process🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-bridge-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2644
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.8
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-stdio-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-stdio-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.81
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.82
      }
    },
    {
      "name": "🧑‍💻mcp🌍️gis2d-http-release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:mcp-gis2d-http-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 332.83
      }
    },
    {
      "name": "⚖️browser-test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:browser-test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️contract-check🎮️native-host🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playground-native-host:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️test📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️wasm📐️cad-composition🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-cad-composition-rs:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌍️gis🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-codec-oracle🌿️vcs🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/vcs-plugin:native-codec-oracle",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️source-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️native-build-release💡️services🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:native-build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-presentation🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-presentation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2634
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-mcp🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2635
      }
    },
    {
      "name": "⚖️check🗺️gis-inference-native-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-native-service",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2636
      }
    },
    {
      "name": "⚖️check🧩️mcp-installed-service🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2637
      }
    },
    {
      "name": "⚖️publish🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️test🗄️stdio-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-publication:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️build🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️check🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️test🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️canonical-architecture🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️deletion-proof🗄️stdio-assembly🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-assembly-rs:deletion-proof",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️flow-add-widget-retained-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:add-widget-retained-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-edit-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-edit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-child-identity-check🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:child-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️flow-test-source🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:test-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-architect-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-architect-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-cad-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cad-cad-rs:verify-cad-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-curation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sourcing-curation-rs:verify-curation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-dag-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dag-dag-rs:verify-dag-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-drawing-canvas-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:verify-drawing-canvas-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-assembly-physical-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-assembly-physical-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-live-visual-publication🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-live-visual-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-mesh-preparation-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-mesh-preparation-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-microcursor🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-microcursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-numerical-page-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-numerical-page-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-pcg-publication-owners🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-pcg-publication-owners",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem-scalar-owners-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem-scalar-owners-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem2d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem2d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-numerical-child-native🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/fem-3d-rs:verify-fem3d-numerical-child-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-fem3d-window-config-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fem-composition-tests:verify-fem3d-window-config-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-flow-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/flow-flow-rs:verify-flow-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/forms-forms-rs:verify-forms-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-forms-try-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-forms-composition-tests:verify-forms-try-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-flow-physical-retirement🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:verify-framework-flow-physical-retirement",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-framework-ui-protocol-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-space-composition-tests:verify-framework-ui-protocol-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation2d-window-camera-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation2d-rs:verify-generation2d-window-camera-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-document-io🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-document-io",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-generation3d-preview-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:verify-generation3d-preview-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-map-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-gis-map-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-gis-terrain-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-gis-terrain-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-home-host-panel-owner🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/space-home-rs:verify-home-host-panel-owner",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-jack-query-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-rs:verify-jack-query-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-layout-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/layout-layout-rs:verify-layout-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-map-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-map-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-norm-results-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-norm-composition-tests:verify-norm-results-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-note-composition-tests:verify-note-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-note-empty-config-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-rs:verify-note-empty-config-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-playbook-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/playbook-playbook-rs:verify-playbook-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-presentation-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/animate-presentation-rs:verify-presentation-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-procedure-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/imperative-procedure-rs:verify-procedure-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-program-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/architect-program-rs:verify-program-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-puzzle-fill-policy-self-tests🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-remodel-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-remodel-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-map-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-map-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-rewriting-window-config🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-rewriting-rs:verify-rewriting-window-config",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-sequence-window-ownership🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/sequence-sequence-rs:verify-sequence-window-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-stdio-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:verify-stdio-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-terrain-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gisterrain-rs:verify-terrain-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-document-contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-document-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-wires-window-transient🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/reasoning-wires-rs:verify-wires-window-transient",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️verify-writer-window-state🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/writer-writer-rs:verify-writer-window-state",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️owned-script-routes🦑️repo🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-owned-script-routes",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️installed-service🧰️os🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:installed-service-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2631
      }
    },
    {
      "name": "⚖️inference🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:inference-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2632
      }
    },
    {
      "name": "⚖️inference-browser🗺️gismap🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-gismap-js:cold-document-pair-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2633
      }
    },
    {
      "name": "⚖️browser-dock-widgets⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2641
      }
    },
    {
      "name": "⚖️browser-dock-widgets🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --suite widgets --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2642
      }
    },
    {
      "name": "⚖️browser-embedded-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-embedded-acceptance -- --configuration \"${workspaceFolder}/${input:wgpuEmbeddedConfiguration}\" --output \"${workspaceFolder}/${input:wgpuEmbeddedArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2643
      }
    },
    {
      "name": "🧪️test🖍️draw🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.4
      }
    },
    {
      "name": "🧪️test🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🔍️check🖍️draw🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/draw-drawing-rs:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "📚️deps print tex",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tex",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -36
      }
    },
    {
      "name": "📥️deps print compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:deps-tectonic",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -37
      }
    },
    {
      "name": "🔄️watch logo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 19
      }
    },
    {
      "name": "▶️repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20
      }
    },
    {
      "name": "🚀️start repo coordinator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 21
      }
    },
    {
      "name": "📥️styling Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️styling .NET dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-dotnet:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️energy oracle Python dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/energy-oracle-py:deps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "⚙️setup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -50
      }
    },
    {
      "name": "▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.9
      }
    },
    {
      "name": "🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.8
      }
    },
    {
      "name": "🧹lint",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:lint",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.7
      }
    },
    {
      "name": "🎨format",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:format",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.6
      }
    },
    {
      "name": "🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.5
      }
    },
    {
      "name": "📦️build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.4
      }
    },
    {
      "name": "🚢️publish",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49.3
      }
    },
    {
      "name": "🧰️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -49
      }
    },
    {
      "name": "⚙️setup🪟️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -48
      }
    },
    {
      "name": "⚙️setup🐙️git",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:setup-git",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -47
      }
    },
    {
      "name": "📥️deps javascript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-javascript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -46
      }
    },
    {
      "name": "📥️deps python",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-python",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -45
      }
    },
    {
      "name": "📥️deps cargo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -44
      }
    },
    {
      "name": "📥️deps go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-go",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43
      }
    },
    {
      "name": "📥️deps dotnet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-dotnet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -42
      }
    },
    {
      "name": "📥️deps cpp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cpp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -41
      }
    },
    {
      "name": "📥️deps browsers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-browsers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -40
      }
    },
    {
      "name": "📥️deps wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "📥️deps Trunk",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-trunk",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.75
      }
    },
    {
      "name": "📥️deps wasm optimizer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-wasm-opt",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38.5
      }
    },
    {
      "name": "📥️deps tools",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -38
      }
    },
    {
      "name": "🧪️test⚡️cache-command-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-command-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.5
      }
    },
    {
      "name": "⚖️gate🦀️cargo🧾️build-dir-provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": -11.4
      }
    },
    {
      "name": "🚦️ci baseline",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:ci-baseline",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -39
      }
    },
    {
      "name": "🦑️mcp dev",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./📜️script.ts dev mcp stdio client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -4
      }
    },
    {
      "name": "⌨️gemini",
      "type": "node-terminal",
      "request": "launch",
      "command": "gemini --yolo",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 10
      }
    },
    {
      "name": "⌨️kiro",
      "type": "node-terminal",
      "request": "launch",
      "command": "kiro-cli chat --trust-all-tools",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "1_keyboard",
        "order": 20
      }
    },
    {
      "name": "🖱️f3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "f3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 10
      }
    },
    {
      "name": "🖱️gitkraken",
      "type": "node-terminal",
      "request": "launch",
      "command": "gitkraken --path \"${workspaceFolder}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 20
      }
    },
    {
      "name": "🖱️mcpinspector",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp",
      "env": {
        "CLIENT_PORT": "6274",
        "SERVER_PORT": "6277"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 30
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎛️dashboard",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon▶️start",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- start",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.1
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌀daemon📎attach",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:daemon -- attach",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.2
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌊️workflow",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:workflow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.3
      }
    },
    {
      "name": "🛠️dev🎛️dashboard🌳️command-tree",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts run command-tree --dump-tree",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 1.4
      }
    },
    "@generated:cad:react",
    "@generated:cad:wgpu",
    {
      "name": "🛠️dev📐️cad🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 10.2
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6020",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6020)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- cad fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "CAD_JS_RENDERER_PLAY_PORT": "6120",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 20.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6120)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📐️cad🧩️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- cad",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 20.2
      }
    },
    "@generated:dag:react",
    "@generated:dag:wgpu",
    {
      "name": "🛠️dev🌳️dag🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- dag",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 140.2
      }
    },
    "@generated:mathematical:react",
    "@generated:mathematical:wgpu",
    {
      "name": "🛠️dev🧮️mathematical🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- mathematical",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 141.2
      }
    },
    "@generated:architect:react",
    "@generated:architect:wgpu",
    {
      "name": "🛠️dev🏛️architect🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- architect",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 142.2
      }
    },
    "@generated:flow:react",
    "@generated:flow:wgpu",
    {
      "name": "🛠️dev🌊️flow🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- flow",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 150.2
      }
    },
    "@generated:imperative:react",
    "@generated:imperative:wgpu",
    {
      "name": "🛠️dev⚙️imperative🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- imperative",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 155.2
      }
    },
    "@generated:sequence:react",
    "@generated:sequence:wgpu",
    {
      "name": "🛠️dev📜️sequence🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sequence",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 156.2
      }
    },
    "@generated:lowpoly:react",
    "@generated:lowpoly:wgpu",
    {
      "name": "🛠️dev🔷️lowpoly🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- lowpoly",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 157.2
      }
    },
    "@generated:layout:react",
    "@generated:layout:wgpu",
    {
      "name": "🛠️dev📄️layout🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- layout",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 158.2
      }
    },
    "@generated:gis2d:react",
    "@generated:gis2d:wgpu",
    {
      "name": "🛠️dev🌐️gis📍️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.2
      }
    },
    "@generated:gis3d:react",
    "@generated:gis3d:wgpu",
    {
      "name": "🛠️dev🌐️gis⛰️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- gis3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 160.5
      }
    },
    "@generated:animate:react",
    "@generated:animate:wgpu",
    {
      "name": "🛠️dev🎬️animateplay🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- animate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 170.2
      }
    },
    {
      "name": "🛠️dev🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch-viz",
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "🛠️dev🖨️print",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 175
      }
    },
    "@generated:generation2d:react",
    "@generated:generation2d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 180.2
      }
    },
    "@generated:generation3d:react",
    "@generated:generation3d:wgpu",
    {
      "name": "🛠️dev🔧️procedural🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- generation3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 190.2
      }
    },
    "@generated:process3d:react",
    "@generated:process3d:wgpu",
    {
      "name": "🛠️dev🪚️process🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- process3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 195.2
      }
    },
    "@generated:sourcing:react",
    "@generated:sourcing:wgpu",
    {
      "name": "🛠️dev🛒️sourcing🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- sourcing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 196.2
      }
    },
    "@generated:bitmap:react",
    "@generated:bitmap:wgpu",
    {
      "name": "🛠️dev🀄️wfc🖼️bitmap🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- bitmap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 201.2
      }
    },
    "@generated:grid2d:react",
    "@generated:grid2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🔲️grid2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 202.2
      }
    },
    "@generated:wfc2d:react",
    "@generated:wfc2d:wgpu",
    {
      "name": "🛠️dev🀄️wfc◻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 203.2
      }
    },
    "@generated:grid3d:react",
    "@generated:grid3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧱️grid3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- grid3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 204.2
      }
    },
    "@generated:wfc3d:react",
    "@generated:wfc3d:wgpu",
    {
      "name": "🛠️dev🀄️wfc🧊️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- wfc3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 205.2
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d🎛️hexagonal🍄️mushroom🧱️column🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_DEFAULT_EXAMPLE": "hexagonal-mushroom-column"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6018",
        "SEMIO_RENDERER": "react",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.2
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6018)",
        "uriFormat": "%s/?plugin=generation3d"
      }
    },
    {
      "name": "🛠️dev🔧️procedural🏙️3d👁️viewer🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- procedural 3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6118",
        "SEMIO_RENDERER": "wgpu",
        "SEMIO_APP_ROLE": "viewer"
      },
      "presentation": {
        "group": "3_dev",
        "order": 200.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6118)",
        "uriFormat": "%s/?plugin=generation3d&role=viewer"
      }
    },
    {
      "name": "🛠️dev📽️projektetage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PRAESENTATION_PROJEKTETAGE_PORT": "6050"
      },
      "presentation": {
        "group": "3_dev",
        "order": 210
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6050)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📚️bericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 211
      }
    },
    "@generated:aggregator:react",
    {
      "name": "🛠️dev♻️mit-bestand🧺️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "MIT_BESTAND_DEMONSTRATOR_PORT": "6029",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6029)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.1
      }
    },
    {
      "name": "🖥️serve🏚️mitbestand🎪️demonstrator",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-demonstrator:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.2
      }
    },
    {
      "name": "🛠️dev🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TECH_PLAY_PORT": "6033",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.3
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6033)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "♻️activate🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:activate-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.4
      }
    },
    {
      "name": "🖥️serve🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:serve",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.5
      }
    },
    {
      "name": "🛠️dev🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "TEACHING_ARCHITECTURE_QUIZ_PORT": "6061",
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.6
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6061)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "PROCTOR_PORT": "8791"
      },
      "presentation": {
        "group": "3_dev",
        "order": 213.61
      }
    },
    {
      "name": "🔁️rebuild🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:rebuild",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.611
      }
    },
    {
      "name": "🧪️test❓️quiz🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.63
      }
    },
    {
      "name": "🧪️test❓️quiz⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.64
      }
    },
    {
      "name": "🧪️test❓️quiz🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.65
      }
    },
    {
      "name": "🧪️test🎓️teaching🛂️proctor🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.66
      }
    },
    {
      "name": "🧪️test🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.67
      }
    },
    {
      "name": "🛠️dev❓️quiz⚛️react🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/quiz-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 213.68
      }
    },
    "@generated:generator:react",
    "@generated:koordinator:react",
    "@generated:aussuchen:react",
    "@generated:bearbeiten:react",
    "@generated:verfolgen:react",
    "@generated:puzzle2d:react",
    "@generated:puzzle2d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 220.2
      }
    },
    "@generated:puzzle3d:react",
    "@generated:puzzle3d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 230.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6013",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6013)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 3d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_3D_PLAY_PORT": "6113",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 240.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6113)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle🏙️3d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 240.2
      }
    },
    "@generated:puzzle5d:react",
    "@generated:puzzle5d:wgpu",
    {
      "name": "🛠️dev🧩️puzzle👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 250.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6014",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6014)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d fixture concrete",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6114",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 260.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6114)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️concrete🌲️forest🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 260.2
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6015",
        "SEMIO_RENDERER": "react",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6015)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🧩️puzzle👯️5d🎛️capsule🌙️dream🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- 5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "PUZZLE_5D_PLAY_PORT": "6115",
        "SEMIO_RENDERER": "wgpu",
        "PLAYGROUND_LOCKED_EXAMPLE_ID": "capsule-dream",
        "SEMIO_DEFAULT_EXAMPLE": "capsule-dream"
      },
      "presentation": {
        "group": "3_dev",
        "order": 261.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6115)",
        "uriFormat": "%s"
      }
    },
    "@generated:block2d:react",
    "@generated:block2d:wgpu",
    {
      "name": "🛠️dev🧱️block🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 261.2
      }
    },
    "@generated:block3d:react",
    "@generated:block3d:wgpu",
    {
      "name": "🛠️dev🧱️block🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 262.2
      }
    },
    "@generated:block5d:react",
    "@generated:block5d:wgpu",
    {
      "name": "🛠️dev🧱️block👯️5d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- block5d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 263.2
      }
    },
    "@generated:reasoning-wires:react",
    "@generated:reasoning-wires:wgpu",
    {
      "name": "🛠️dev🧠️reasoning🔗️wires🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- reasoning-wires",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 270.2
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp repo",
      "env": {
        "MCP_AUTO_OPEN_ENABLED": "false",
        "MCP_PROXY_AUTH_TOKEN": "repo-mcp-token"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://[^\\s]+:6274/\\?MCP_PROXY_AUTH_TOKEN=[^\\s]+)",
        "uriFormat": "%s&transport=stdio&serverCommand=cargo&serverArgs=run&serverArgs=--release&serverArgs=-p&serverArgs=semio-framework-repo-cli&serverArgs=--&serverArgs=mcp&MCP_PROXY_FULL_ADDRESS=http://127.0.0.1:6277"
      }
    },
    {
      "name": "🛠️dev🧰️repo🤖️mcp⌨️cursor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio cursor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.1
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.15
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️cli🦀️semio-repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:repo --args=\"--help\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.16
      }
    },
    {
      "name": "🛠️dev🧰️repo🔌️mcp🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:mcp",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.17
      }
    },
    {
      "name": "🛠️dev🧰️repo⌨️client",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-cli-rs:run",
      "env": {
        "SEMIO_REPO_IMPLEMENTATION": "go",
        "GOWORK": "${workspaceFolder}/go.work"
      },
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.25
      }
    },
    {
      "name": "🛠️build🧰️repo🔌️mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo-mcp:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.4
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🐹️go",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-go:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.5
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🦀️rust",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator-rs:run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.6
      }
    },
    {
      "name": "🛠️dev🧰️repo🖥️coordinator🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-coordinator:dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 280.7
      }
    },
    "@generated:shooting:react",
    "@generated:shooting:wgpu",
    {
      "name": "🛠️dev📸️shooting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 290.2
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6019",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6019)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- shooting fixture base-icon",
      "cwd": "${workspaceFolder}",
      "env": {
        "SHOOTING_PLAY_PORT": "6119",
        "SEMIO_RENDERER": "wgpu"
      },
      "presentation": {
        "group": "3_dev",
        "order": 300.1
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6119)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📸️shooting🎛️base🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- shooting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 300.2
      }
    },
    {
      "name": "🛠️dev📖️storybook",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 310
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle◻️2d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-2d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 340
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-ui",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 350
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎨️styling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-styling",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 460
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🧊️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-3d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 470
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🧩️puzzle🖐️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-puzzle-5d",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 480
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 490
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🔌️hosts",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-hosts",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 400
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🛠️framework🖥️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-framework-os",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 410
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-infinite",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 420
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-cad",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 430
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev📖️storybook🎬️animate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev-storybook-animate",
      "cwd": "${workspaceFolder}",
      "env": {
        "STORYBOOK_PORT": "6010"
      },
      "presentation": {
        "group": "3_dev",
        "order": 450
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6010)",
        "uriFormat": "%s"
      }
    },
    "@generated:trinity-jack:react",
    "@generated:trinity-jack:wgpu",
    {
      "name": "🛠️dev🔺️trinity🃏️jack🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 360.2
      }
    },
    {
      "name": "🛠️dev🔺️trinity🃏️jack🦀️shell",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/trinity-jack-shell:run -- trinity/fixture/nakagin-capsule-tower.trinity.json \"MATCH (a:Piece) RETURN a.name\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 370
      }
    },
    "@generated:trinity-rewriting:react",
    "@generated:trinity-rewriting:wgpu",
    {
      "name": "🛠️dev🔺️trinity♻️rewriting🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- trinity-rewriting",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 380.2
      }
    },
    "@generated:forms:react",
    "@generated:forms:wgpu",
    {
      "name": "🛠️dev📋️forms🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- forms",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    {
      "name": "🧪️test🖨️raster🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🧪️test🖨️raster🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/raster-raster-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.8
      }
    },
    {
      "name": "🧪️test🔲️pixels🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-typescript",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.9
      }
    },
    {
      "name": "🧪️test🔲️pixels🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/pixels:test-rust",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387
      }
    },
    "@generated:raster:react",
    "@generated:raster:wgpu",
    {
      "name": "🛠️dev🖼️raster🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- raster",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.2
      }
    },
    "@generated:vcs:react",
    "@generated:vcs:wgpu",
    {
      "name": "🛠️dev🗄️vcs🎛️play🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- vcs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 385.2
      }
    },
    "@generated:s:react",
    "@generated:s:wgpu",
    "@generated:s:users",
    {
      "name": "🛠️dev🖥️s⚛️react📦️served",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s served",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🖥️s🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.1
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.5
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins📏️size",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:plugin -- size",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.6
      }
    },
    {
      "name": "🛠️dev🦀️os-plugins🧫️scale-fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:generate-scale-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 386.7
      }
    },
    {
      "name": "🛠️dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-postgres",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-postgres/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.002
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-neo4j",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev-neo4j/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.003
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
        "uriFormat": "%s/admin"
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬆️up",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-up -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.004
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends🩺️status",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-status -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.005
      }
    },
    {
      "name": "🛠️dev🐳️hub-backends⬇️down",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-down -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.006
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🎫️local",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-hub-owner",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.01
      }
    },
    {
      "name": "🛠️dev🗄️os-hub🛡️admin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-admin:dev",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_URL": "http://127.0.0.1:8787"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8790)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🪐️space⚛️react🔒local-only",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- s",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_PORT": "6070",
        "SEMIO_PLUGIN": "s",
        "SEMIO_RENDERER": "react",
        "S_LOCAL_ONLY": "1"
      },
      "presentation": {
        "group": "3_dev",
        "order": 386.25
      },
      "serverReadyAction": {
        "action": "openExternally",
        "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):6070)",
        "uriFormat": "%s"
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-suite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-suite",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "S_OS_PORT": "6066",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-native🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-native",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/",
        "SEMIO_PLUGIN": "s"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.061
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-mcp🌉️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-mcp",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.062
      }
    },
    {
      "name": "🛠️dev🔐️os-secure-admin🛡️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-secure-admin",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_HUB_PORT": "8787",
        "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/"
      },
      "presentation": {
        "group": "3_dev",
        "order": 387.063
      }
    },
    {
      "name": "🛠️dev🤝️os-collab-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:dev-collaboration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.064
      }
    },
    {
      "name": "🛠️dev🪐️os-s🩺️cold-boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:cold-boot-check-s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.065
      }
    },
    {
      "name": "🛠️dev🪐️os-s🔭️foreign-kind",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-foreign-kind-s -- http://127.0.0.1:6070/ --tag launch raster dag block=s.block.block2d@1/*#editor block=s.block.block3d@1/*#editor block=s.block.block5d@1/*#editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.066
      }
    },
    {
      "name": "🛠️dev🪐️os-s🤏️pinch-a11y",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:s-host-pinch-diagram-contrast-s -- http://127.0.0.1:6070/ --tag launch",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.067
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🧵️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp stdio os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.05
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🌐️http",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:dev -- mcp http os",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.06
      }
    },
    {
      "name": "🛠️dev🌉️os-mcp🤝️client-e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:client-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.07
      }
    },
    {
      "name": "🛠️dev⏳️async🛌️worker-parking-check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-async-rs:worker-parking-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.071
      }
    },
    {
      "name": "🖱️mcpinspector🌉️os",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x @modelcontextprotocol/inspector --config .mcp.json --server semio",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "2_mouse",
        "order": 31
      }
    },
    {
      "name": "🛠️dev🕸️os-run",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:os -- run",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.1
      }
    },
    "@generated:draw:react",
    "@generated:draw:wgpu",
    {
      "name": "🛠️dev✏️draw🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- draw",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:note:react",
    "@generated:note:wgpu",
    {
      "name": "🛠️dev📝️note🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- note",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 388.2
      }
    },
    "@generated:writer:react",
    "@generated:writer:wgpu",
    {
      "name": "🛠️dev✍️writer🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- writer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 387.2
      }
    },
    "@generated:remodel:react",
    "@generated:remodel:wgpu",
    {
      "name": "🛠️dev🏺️remodel🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- remodel",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 389.2
      }
    },
    {
      "name": "🛠️dev🖱️ui🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-react:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390
      }
    },
    {
      "name": "🛠️dev🧰️framework🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.1
      }
    },
    {
      "name": "🛠️dev💻️os🪁️typecheck",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 390.2
      }
    },
    "@generated:fem2d:react",
    "@generated:fem2d:wgpu",
    {
      "name": "🛠️dev🏗️fem🩻️2d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem2d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.2
      }
    },
    "@generated:fem3d:react",
    "@generated:fem3d:wgpu",
    {
      "name": "🛠️dev🏗️fem🏙️3d🧊️wgpu🖥️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native -- fem3d",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": 392.5
      }
    },
    {
      "name": "🛠️dev🧊️wgpu🖥️native🚢️release",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-release -- s",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 121.4
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📋️zwischenbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-zwischenbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand📑️forschungsbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-forschungsbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.001
      }
    },
    {
      "name": "🛠️dev♻️mit-bestand kompaktbericht",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/mit-bestand-bericht:watch-kompaktbericht",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "0_dev",
        "order": 392.002
      }
    },
    {
      "name": "📦️build✏️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 9
      }
    },
    {
      "name": "📦️build🖥️s",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-s-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build📐️cad",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-cad-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 10
      }
    },
    {
      "name": "📦️build🏢️semio-tech🎡️play",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11
      }
    },
    {
      "name": "⚖️gate🏢️semio-tech🎡️play🎭️e2e",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/semio-tech-play:test-e2e",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.1
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.15
      }
    },
    {
      "name": "📦️build🎓️teaching🛂️proctor",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.2
      }
    },
    {
      "name": "📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.3
      }
    },
    {
      "name": "🚚️publish🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 11.35
      }
    },
    {
      "name": "✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.1
      }
    },
    {
      "name": "✅️check🎓️teaching🛂️proctor📚️catalog",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/proctor:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.15
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-image",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-image-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.2
      }
    },
    {
      "name": "⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-stack",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @teaching/architecture-quiz:docker-stack-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 11.25
      }
    },
    {
      "name": "📦️build🧩️puzzle🏙️3d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle3d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 140
      }
    },
    {
      "name": "📦️build🧩️puzzle👯️5d",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-puzzle5d-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 150
      }
    },
    {
      "name": "📦️build📸️shooting",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:build-shooting-react-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 180
      }
    },
    {
      "name": "📦️check🧊️wgpu🔒️lockfile",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:lockfile-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.145
      }
    },
    {
      "name": "📦️generate🧊️wgpu🚀️boot",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.146
      }
    },
    {
      "name": "📦️check🧊️wgpu🌐️browser",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.147
      }
    },
    {
      "name": "📦️check⚛️react🔐️hub-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-react:hub-sign-in-spaces-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1475
      }
    },
    {
      "name": "📦️test🧊️wgpu🖱️ui",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-wgpu-engine",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.148
      }
    },
    {
      "name": "📦️test🧊️wgpu📺️renderer",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.149
      }
    },
    {
      "name": "📦️test🧊️wgpu♾️infinite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-infinite:test-wgpu-world-terrain",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.15
      }
    },
    {
      "name": "📦️build🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.159
      }
    },
    {
      "name": "📦️build-dev🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:build-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16
      }
    },
    {
      "name": "🚚️publish🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1601
      }
    },
    {
      "name": "📦️generate🌐️gis🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/gis-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16013
      }
    },
    {
      "name": "📦️generate🗄️stdio🧬️native-codec-projection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:native-codec-projection",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16014
      }
    },
    {
      "name": "🛫️preflight-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-preflight --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160145
      }
    },
    {
      "name": "🚚️publish-catalog🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:trusted-catalog-bootstrap --packages all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16015
      }
    },
    {
      "name": "🧊️check-guest-framework🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:guest-framework-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.160155
      }
    },
    {
      "name": "🔁️rebuild-all🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:rebuild-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16016
      }
    },
    {
      "name": "📦️build-release🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:build-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1602
      }
    },
    {
      "name": "🚚️publish🌉️os-mcp",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:publish",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1603
      }
    },
    {
      "name": "📦️test🗄️os-hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.161
      }
    },
    {
      "name": "📦️test🗄️os-hub♾️all-features",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:test-all-features",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.162
      }
    },
    {
      "name": "📦️test🖥️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server-rs:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1625
      }
    },
    {
      "name": "📦️test🖥️server🟦️typescript",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-server:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1626
      }
    },
    {
      "name": "📦️check🗄️os-hub🚀️launch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-bootstrap-launch-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.163
      }
    },
    {
      "name": "📦️check🤖️generated🔬️corruption",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-generated-corruption",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.164
      }
    },
    {
      "name": "📦️check🗄️os-hub🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🦑️repo🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:typecheck",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-clean-scaffold-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-clean-scaffold-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🧬schema🗺️surface🏛️abstraction🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-surface-abstraction-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "🧬schema🗿artifact🧪root-artifact-schema-law-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-artifact-schema-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "🧬schema💡️inference🧪source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-inference-law-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-schema-field-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-schema-field-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🧹clean🦑️repo🧪️source-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-repo-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️vitest-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-vitest-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.173
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎚️tool-configuration-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-tool-configuration-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.174
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📦️package-body-policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-package-body-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.175
      }
    },
    {
      "name": "📦️test🥾️bootstrap🪟️cross-platform",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cross-platform-bootstrap",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.176
      }
    },
    {
      "name": "📦️test🏃️process🪓️tree-termination",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.177
      }
    },
    {
      "name": "📦️test🪟️windows🧭️command-paths",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-windows-command-paths",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1775
      }
    },
    {
      "name": "📦️test🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cargo-provenance",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.178
      }
    },
    {
      "name": "🧹clean🦀️cargo🧾️provenance",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:cargo-provenance-repair",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.179
      }
    },
    {
      "name": "📦️check🗿️taxonomy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.165
      }
    },
    {
      "name": "📦️check🗿️taxonomy🧩️implementation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-taxonomy-implementation-report",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.166
      }
    },
    {
      "name": "📦️check🔒️dependencies📃️literal-external",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-dependencies-literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.167
      }
    },
    {
      "name": "🏛️check🧩️canonical-architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 899.99
      }
    },
    {
      "name": "📦️check🧅️layering",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify-layering",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.168
      }
    },
    {
      "name": "📦️check🔌️plugin-registry",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.169
      }
    },
    {
      "name": "📦️generate🖨️print📊️viz",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/print:generate-viz",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1689
      }
    },
    {
      "name": "📦️generate📕️norm🪨️en1996🖼️example-assets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-en1996-rs:regenerate-example-assets",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.16915
      }
    },
    {
      "name": "📦️generate🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1691
      }
    },
    {
      "name": "📦️check🧬️surface-schema",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:surface-schema-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1692
      }
    },
    {
      "name": "📦️generate✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1693
      }
    },
    {
      "name": "📦️check✨️dsl-derive🧬️mutation-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/dsl-derive-rs:check-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1694
      }
    },
    {
      "name": "📦️check🧹️fixture-sweep",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-fixture-sweep-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.17
      }
    },
    {
      "name": "📦️check🌊️flow-composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-flow-composition-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1701
      }
    },
    {
      "name": "📦️check🌐️semio-session",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-spatial-kernel-semio-session-rs:source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.1702
      }
    },
    {
      "name": "📦️check📡️channel-version",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.171
      }
    },
    {
      "name": "🛠️dev📡️channel-version🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-generate",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 206.172
      }
    },
    {
      "name": "📦️wasm🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.81
      }
    },
    {
      "name": "📦️wasm-release🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:wasm-release",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 500.811
      }
    },
    {
      "name": "⚖️gate🧱️hub-foundations📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:foundation-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10755
      }
    },
    {
      "name": "⚖️gate🧭️local-relay📛️admission",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:local-relay-routing-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10756
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🤝️live-sign-in",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:live-sign-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10757
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-creation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-creation-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🔐️hub-auth🧊️wgpu-live-collaboration",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:hub-live-collaboration-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🧊️wgpu⏯️native-guest-journey",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:native-guest-journey-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚨️capability-audit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107575
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10758
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤖️live-agent-loop🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:live-agent-loop-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.107582
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-edit-durability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-edit-durability-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107585
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🤝️hub-agent-participant",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:hub-agent-participant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.10759
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💬️agent-reply",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:agent-reply-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.107595
      }
    },
    {
      "name": "🧹clean🧩️taxonomy❄️frozen-markdown-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-frozen-markdown-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.18
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🕰️historical-json-source-encoding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-json-source-encoding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.19
      }
    },
    {
      "name": "⚖️gate🌊️flow🌐️startup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.8
      }
    },
    {
      "name": "⚖️gate🌊️flow⏱️consumed-clock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-clock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.81
      }
    },
    {
      "name": "⚖️gate🌊️flow🏷️browser-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:test-browser-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.82
      }
    },
    {
      "name": "📦️preview🤖️flow-browser-package",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run semio-framework-os-flow-core:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.83
      }
    },
    {
      "name": "📦️build🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.84
      }
    },
    {
      "name": "📦️check🗄️stdio",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_build",
        "order": 407.85
      }
    },
    {
      "name": "⚖️gate🗄️stdio🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.86
      }
    },
    {
      "name": "⚖️gate🗄️stdio🛂️package-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.87
      }
    },
    {
      "name": "⚖️gate🗄️stdio🕸️package-graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-js:package-graph",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.88
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-artifact-contract-rs:test -- --lib",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.89
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editors🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --target=test --projects=\"@semio-tech/stdio-*-rs\" --exclude=@semio-tech/stdio-artifact-contract-rs --parallel=2 -- --features component-app-assembly --lib editor",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.9
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️catalog🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test editor_catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.91
      }
    },
    {
      "name": "⚖️gate🗄️stdio🚢️shipped-fleet🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test shipped_fleet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.915
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️editing🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-snapshot-editing-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.92
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-rs:test -- --lib lanes_preserve_complete_unicode_documents",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.93
      }
    },
    {
      "name": "⚖️gate🗄️stdio🎬️lanes🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-scene-js:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.94
      }
    },
    {
      "name": "⚖️gate🗄️stdio🪟️kits🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test -- window_kits_tests",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.95
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️component🌐️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:editor-component-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.96
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️fixture🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- editor-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.97
      }
    },
    {
      "name": "⚖️gate🗄️stdio✏️launch🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test -- 🧪️tests/🚀️launch/🟦️.ts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.98
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.32
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.33
      }
    },
    {
      "name": "⚖️gate🌱️value💾️resident🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.34
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🔣️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test-source --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.35
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.36
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🦀️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.37
      }
    },
    {
      "name": "⚖️gate🖱️ui🖥️host🌐️wasm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/ui-host-rs:check-wasm --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.38
      }
    },
    {
      "name": "⚖️gate🔌️plugin🦀️lib",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 408.39
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-support",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-support --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.06
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🏺️historical-package-owner-identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-historical-package-owner-identity --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.07
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧲️rust-physical-reference-context",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-physical-reference-context --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.08
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️cli-cancellation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-cli-cancellation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.09
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💠️inventory-artifact-shards",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-inventory-artifact-shards --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.1
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-script-compiler",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-script-compiler",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.11
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔎️json-reference-owner-lookup",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-json-reference-owner-lookup --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.12
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚧️cargo-discovery-exclusions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-discovery-exclusions --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.13
      }
    },
    {
      "name": "🧹clean🧩️taxonomy💥️nested-cargo-collision-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nested-cargo-collision-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.14
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🌐️registry-import-language",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-import-language --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.15
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛫️preflight-reference-basis",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-preflight-reference-basis --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.16
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🛤️typescript-path-collection",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-typescript-path-collection --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.17
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🗺️testing-readme-coordinates",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-testing-readme-coordinates --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.192
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-residue",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-residue --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.194
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔖️readme-current-source-revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.195
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🚚️readme-move-source-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-move-source-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.197
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️artifact-source-commit",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-source-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.198
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🫙️artifact-empty-facet-authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authority --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.199
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🟢️readme-current-source-activation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-current-source-activation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.204
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🪶️artifact-empty-facet-authoring",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-artifact-empty-facet-authoring --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.205
      }
    },
    {
      "name": "🧹clean🧩️taxonomy👀️readme-reviewed-fixture-inputs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-readme-reviewed-fixture-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.206
      }
    },
    {
      "name": "🧹clean🧩️taxonomy♻️taxonomy-pattern-compiler-reuse",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-pattern-compiler-reuse --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.207
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🔤️taxonomy-leading-grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-taxonomy-leading-grapheme --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.209
      }
    },
    {
      "name": "🧹clean🧩️taxonomy📈️reference-coordinate-progress",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-reference-coordinate-progress --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.21
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️draw-destination-observation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-draw-destination-observation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.211
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧾️registry-catalog-gitlink-boundary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-registry-catalog-gitlink-boundary --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2152
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🎯️cargo-target-discovery-skip",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-target-discovery-skip --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 410.2154
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant📐️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-command-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.09999
      }
    },
    {
      "name": "⚖️gate🔌️socket-grant🛡️server",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:socket-grant-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.1
      }
    },
    {
      "name": "⚖️gate📌️check-in🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.11
      }
    },
    {
      "name": "⚖️gate📌️check-in🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-native-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.12
      }
    },
    {
      "name": "⚖️gate📌️check-in🌉️process",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub:check-in-process-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.13
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.15
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.151
      }
    },
    {
      "name": "⚖️gate🔐️wal-writer-fence🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:wal-writer-fence-live -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.152
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run os-hub:directory-live-lanes -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.153
      }
    },
    {
      "name": "⚖️gate🗂️directory-live-lanes🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run os-hub:directory-live-lanes -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.154
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.16
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.17
      }
    },
    {
      "name": "⚖️gate🤝️two-client-document🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:two-client-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.18
      }
    },
    {
      "name": "⚖️gate📈️document-growth🪶️sqlite",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- sqlite",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.19
      }
    },
    {
      "name": "⚖️gate📈️document-growth🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2
      }
    },
    {
      "name": "⚖️gate📈️document-growth🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:document-growth-e2e -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.21
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.22
      }
    },
    {
      "name": "⚖️browser-dock-contract🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️browser-dock-acceptance⚛️react🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run react \"${input:wgpuDockReactServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.261
      }
    },
    {
      "name": "⚖️browser-dock-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-dock-acceptance -- run wgpu \"${input:wgpuDockServe}\" --output \"${workspaceFolder}/${input:wgpuDockArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.262
      }
    },
    {
      "name": "⚖️browser-media-acceptance🧊️wgpu🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:browser-media-acceptance -- --serve \"${input:wgpuMediaServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:wgpuMediaArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.263
      }
    },
    {
      "name": "⚖️parity-journey🧑‍💻dev🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:parity-journey -- --renderer ${input:rendererParityRenderer} --react-serve \"${input:wgpuDockReactServe}\" --wgpu-serve \"${input:wgpuDockServe}\" --locale ${input:wgpuMediaLocale} --output \"${workspaceFolder}/${input:rendererParityJourneyArtifacts}\"",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.264
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal🔗️hub",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-goal -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --local-serve ${input:acceptanceLocalServeUrl} --users ${input:acceptanceUsers}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.221
      }
    },
    {
      "name": "⚖️gate🎯️repo-goal📋️plan",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:acceptance-plan",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.23
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.24
      }
    },
    {
      "name": "⚖️gate🧮️program-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:program-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de --roles editor,viewer",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.25
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.251
      }
    },
    {
      "name": "⚖️gate⏯️tool-run⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:tool-run-matrix -- --serve http://127.0.0.1:6070/ --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.252
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-en --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.253
      }
    },
    {
      "name": "⚖️gate🗂️hub-document-sweep⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:hub-document-sweep -- --serve http://127.0.0.1:6071/ --hub ${input:acceptanceHubUrl} --tag launch-de --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.254
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.255
      }
    },
    {
      "name": "⚖️gate🚪️io-matrix⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:io-matrix -- --serve http://127.0.0.1:6070/ --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.256
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.26
      }
    },
    {
      "name": "⚖️gate👥️two-human⚛️react🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:two-human -- --hub ${input:acceptanceHubUrl} --serve ${input:acceptanceServeUrl} --users ${input:acceptanceUsers} --locale de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.27
      }
    },
    {
      "name": "⚖️gate🔀️connection-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:connection-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.275
      }
    },
    {
      "name": "⚖️gate💤️idle-budget⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:idle-budget -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.276
      }
    },
    {
      "name": "⚖️gate🫧️memory-soak⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:memory-soak -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.277
      }
    },
    {
      "name": "⚖️gate⏱️interaction-latency⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:interaction-latency -- ${input:acceptanceServeUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.278
      }
    },
    {
      "name": "⚖️gate⏪️time-travel⚛️react",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2781
      }
    },
    {
      "name": "⚖️gate⏪️time-travel🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6112/ --renderer wgpu --locales en,de --chords en,de",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.2782
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🧩️plugin-coverage",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:plugin-coverage-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.28
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "en"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.29
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🚶️user-path🌐️de",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:user-path-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "S_OS_MCP_LIVE_SHELL_URL": "${input:acceptanceServeUrl}",
        "S_OS_MCP_LIVE_LOCALE": "de"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.3
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp🛡️security",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:security-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}",
        "OS_HUB_ADMIN_CAPABILITY_FILE": "${input:acceptanceHubAdminCapability}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.301
      }
    },
    {
      "name": "⚖️gate🌉️os-mcp💼️inference-quartet",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/s-services-native:inference-quartet-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "OS_MCP_HUB_ORIGIN": "${input:acceptanceHubUrl}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 411.302
      }
    },
    {
      "name": "⚖️gate💾️hub-backup-restore",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backup-restore-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.31
      }
    },
    {
      "name": "⚖️gate🛑️hub-graceful-shutdown",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:shutdown-drill",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.315
      }
    },
    {
      "name": "⚖️gate🧠️hub-residency",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:residency-watch -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.32
      }
    },
    {
      "name": "⚖️gate🌅️hub-boot-watch",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:boot-watch -- --restarts 1",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.325
      }
    },
    {
      "name": "⚖️gate🏷️hub-freshness",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:hub-freshness -- --hub ${input:acceptanceHubUrl}",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.33
      }
    },
    {
      "name": "⚖️gate🤖️hub-agent-ceiling",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:agent-ceiling-check -- --hub ${input:acceptanceHubUrl} --locale en",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.335
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.331
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🐘️postgres",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- postgres -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- postgres",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.332
      }
    },
    {
      "name": "⚖️gate🌪️reopen-storm🕸️neo4j",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run os-hub-ts:backend-run -- neo4j -- bun nx run @semio-tech/framework-os-kernel:reopen-storm-check -- neo4j",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.333
      }
    },
    {
      "name": "⚖️gate🚧️production-placeholders",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- production-placeholders",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.34
      }
    },
    {
      "name": "⚖️gate🎛️command-reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity commands",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.35
      }
    },
    {
      "name": "⚖️gate🪆️composed-child-refs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- composed-child-refs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.42
      }
    },
    {
      "name": "⚖️gate🚫️history-closure",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- history-closure",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.425
      }
    },
    {
      "name": "⚖️gate⚡️interactivity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.43
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🎯️tool-jobs",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity tool-jobs",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.44
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.45
      }
    },
    {
      "name": "⚖️gate⚡️interactivity🧭️apps🎛️actions",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- interactivity apps --actions",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.46
      }
    },
    {
      "name": "⚖️gate📦️dependencies",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.47
      }
    },
    {
      "name": "⚖️gate📦️dependencies0️⃣",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:verify -- dependencies literal-external",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.48
      }
    },
    {
      "name": "⚖️gate🧿️semio✉️base🔬️carrier-reproduce",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/probe-stdio-semio-v1-base:carrier-reproduce",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.49
      }
    },
    {
      "name": "⚖️gate🧪️test🏭️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test-domain:test-inventory",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 411.5
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-plugin-publication-source-ownership📚️library🟦️",
      "command": "bun nx run @semio-tech/repo-lib:test-plugin-publication-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.04
      }
    },
    {
      "name": "⚖️gate🔌️plugin📇️catalog🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-catalog-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05667
      }
    },
    {
      "name": "⚖️gate🔌️plugin🧾️schema-owner🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:schema-document-authority-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05668
      }
    },
    {
      "name": "⚖️gate🔁️graph revision📚️repo",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-graph-revision",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1806
      }
    },
    {
      "name": "⚖️gate🪪️installation identity🧰️framework",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework:installation-identity-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1805
      }
    },
    {
      "name": "📥️deps javascript lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.8
      }
    },
    {
      "name": "⚖️gate📦️javascript dependency commands",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1804
      }
    },
    {
      "name": "📥️deps cargo lock",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-cargo-lock",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -43.9
      }
    },
    {
      "name": "⚖️gate🔌️plugin📦️deployment🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-deployment-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05669
      }
    },
    {
      "name": "⚖️gate🔌️plugin🚀️launch🏷️name🧬️contract🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-launch-name-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0567
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧭️routing 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-routing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05671
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel-version 🏭️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05672
      }
    },
    {
      "name": "🧰️framework 💻️os 🎮️playground ⭐️default 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/plugin-registry:test-playground-default-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05673
      }
    },
    {
      "name": "🧰️framework 💻️os 🔖️channel version 📣️contributions 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-dev:channel-version-contributions-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05674
      }
    },
    {
      "name": "🛡️styling verification contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-verification-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05675
      }
    },
    {
      "name": "📏️styling relative sizing",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-relative-sizing",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05676
      }
    },
    {
      "name": "🎥️video container providers",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/remodel-remodeling-rs:verify-video-container-providers",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05677
      }
    },
    {
      "name": "🗒️note artifact document contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/note-note-js:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05678
      }
    },
    {
      "name": "🧰️framework modules 🛍️product dependency direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-framework-module-product-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05679
      }
    },
    {
      "name": "🧰️framework 🦑️repo ⚡️cache owner policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-cache-policy",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.056795
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:bun-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568099999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🔍️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568199999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📣️members",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:members-write",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05683
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05684
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🟦️workspace 📦️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run workspace:deps-js-all",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568499999999
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0568599999999
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 📥️runtime 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:runtime-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05687
      }
    },
    {
      "name": "🧰️framework 🦑️repo 🦀️workspace 🧩️capability 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/cargo-workspaces:capability-contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05688
      }
    },
    {
      "name": "✏️s 🧿️semio 🧩️composition 🛠️prepare",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:composition-prepare",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05689
      }
    },
    {
      "name": "✏️s 🔊️wav 🧩️architecture 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-wav-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.0569
      }
    },
    {
      "name": "🎭️styling color primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-styling-tokens:test-color-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.05696
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05691
      }
    },
    {
      "name": "🌎️hub 🗄️stdio 📼️artifact 🧪️tests-removal",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:parent-removal-test-check -- \"${workspaceFolder}/${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}"
      },
      "presentation": {
        "group": "4_gate",
        "order": 900.05692
      }
    },
    {
      "name": "🧹clean🧬️schema🧩️subset-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-subset-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05693
      }
    },
    {
      "name": "🧹clean🥽️mesh🛡️transport-contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-mesh-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05694
      }
    },
    {
      "name": "🧹clean🧩️taxonomy🧪️root-taxonomy-workflow-source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-root-taxonomy-workflow-source",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05695
      }
    },
    {
      "name": "🧰️framework 🏃️process 📦️artifact files 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-files",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05697
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️ Framework Process Test Budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-process:test-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056975
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️ Framework Test Adapter Ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-test:test-adapter-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056976
      }
    },
    {
      "name": "🪪️ Framework Identity Grapheme",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun x nx run @semio-tech/framework-identity:test-grapheme",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "repo-gate",
        "order": 900.056977
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:identityContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎛️owned execution 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owned-execution",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05698
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🪓️tree termination 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-process-tree-termination",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05699
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 📏️pixel policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-pixels",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05701
      }
    },
    {
      "name": "🦑️repo 🎨️workspace styling 🎭️color policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-styling-colors",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05702
      }
    },
    {
      "name": "🗄️stdio 🦛️Semio 🧬️conversion definition contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-semio-rs:conversion-definition-check",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/conversion-definition"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05703
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️canonical architecture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:canonical-architecture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05704
      }
    },
    {
      "name": "🧰️framework 🏃️process ⏱️execution budgets",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-execution-budget",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05705
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🧪️bounded test command",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-test-command",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05706
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️repo 🧬️native input vocabulary",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run repo:test-native-input-vocabulary",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05707
      }
    },
    {
      "name": "🗄️stdio 🧊️GLTF 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-gltf-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/gltf-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05708
      }
    },
    {
      "name": "🧰️framework 🖼️assets 🗺️tile proxy 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-tile-proxy-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05709
      }
    },
    {
      "name": "🧪️framework🏃️process🦀️cargo🧭️driver",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-cargo-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05711
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧪️framework🖼️assets🧭️dispatch🛂️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-dispatch-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05712
      }
    },
    {
      "name": "🖥️OS 🎭️actor transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-actor-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0571
      }
    },
    {
      "name": "🖥️OS 🎬️media transport 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os:test-media-transport",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057101
      }
    },
    {
      "name": "🦑️Repo ⚙️native source ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-native-source-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.057102
      }
    },
    {
      "name": "🕸️Graph 🛂️manifest 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-graph:test-manifest-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05713
      },
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:graphContractArtifacts}"
      }
    },
    {
      "name": "🖋️SVG 🎥️video 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:test-svg-video-contract",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05719
      }
    },
    {
      "name": "🏃️Process 🔒️resource leases 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-resource-leases",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0572
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 🧪️Vitest 🧪️driver contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-vitest-driver",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05721
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧬️Schema 🏷️entity ownership 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-entity-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05714
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🏭️generate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:generate-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05715
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 👁️preview",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:preview-generated",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05716
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds ✅️check",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:check-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05717
      }
    },
    {
      "name": "🦑️Repo 🏷️entity kinds 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-client:test-entity-kinds",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05718
      }
    },
    {
      "name": "🪧️Logo 🎬️video export",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/assets:logo",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACTS_DIR": "${workspaceFolder}/${input:svgVideoArtifacts}"
      },
      "presentation": {
        "group": "2_build",
        "order": 206.061
      }
    },
    {
      "name": "🏃️Process 📦️artifact publication 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-artifact-publication",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05722
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🏃️Process 📋️owner context 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-owner-context",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05723
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🗄️stdio 🧾️JSON 🧬️native schema identity",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-json-rs:native-schema-check -- \"${input:stdioRemovalSnapshot}\"",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:stdioRemovalArtifacts}/json-native-schema"
      },
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05724
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧭️router ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-ui-router-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05725
      }
    },
    {
      "name": "🧰️framework 🏃️process 🎯️exact Cargo 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-exact-cargo-laws",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05731
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🦀️native artifacts 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-native-artifacts",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05732
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🧰️framework 🏃️process 🌐️Wasm build 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-process:test-wasm-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05733
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧱️gate",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05739
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🦀️dependency direction 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-cargo-dependency-direction",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05738
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/cargo-direction"
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🧱️primitives",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-ui-primitives",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05726
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 🌐️chrome i18n",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:check-chrome-i18n",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05727
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook development",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-dev",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05728
      }
    },
    {
      "name": "🦑️Repo 🖱️UI 📖️Storybook build",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:ui-storybook-build",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05729
      }
    },
  
    {
      "name": "🦑️Repo 🧱️fixture law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-fixture-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05734
      }
    },
  
    {
      "name": "📇️Directory 🔏️lease fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:test-directory-lease-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05735
      }
    },
  
    {
      "name": "🌉️MCP ✅️approval fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-mcp:test-approval-request-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05736
      }
    },
  
    {
      "name": "🌊️Flow 🏷️slider labels fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-flow-flow-rs:test-slider-labels-fixture",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05737
      }
    },
    {
      "name": "🌐️Locale 🏷️localized label fixture",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-kernel:test-localized-label",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.0574
      }
    },
    {
      "name": "🦑️Repo 🌐️locale law ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-locale-law-ownership",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05741
      }
    },
    {
      "name": "🧬️Schema 🧺️mutation leaf registration 🧪️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-mutation-leaf-registration",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "🧹clean🛡️gates",
        "order": 900.05744
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🧩️neutral owner 🧪️products absent",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-value:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05745
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🌱️Value 🦀️native 🧪️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05746
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🧪️portable policy",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05747
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🦑️Repo 🦀️physical source 🛡️live direction",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:lint-rust-source-direction --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05748
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️test🧬️validator🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-rs:test-neutral-owner --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.05749
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "⚖️document-http-check💻️os🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-os-kernel:document-http-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "9_gates",
        "order": 900.0575
      },
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}"
      }
    },
    {
      "name": "🖱️UI 🌐️locale 🧪️strict portable contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:test-contract --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05756
      }
    },
    {
      "name": "⚖️check🖱️ui🌐️locale🟦️types",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-locale:check-types --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/${input:processContractArtifacts}/ui-locale"
      },
      "presentation": {
        "group": "9_clean_architecture",
        "order": 901.05756
      }
    },
    {
      "name": "🦑️Repo 🧱️policy 🚀️fresh authority",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-dependency-policy-bootstrap --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05758
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05759
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🔗️rust-binding🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-binding-native --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0576
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🧾️attributes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-attributes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05761
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🏘️scopes",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-scopes --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05762
      }
    },
    {
      "name": "🧪️test🦑️repo🧪️test🚷️discovery-boundaries",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-test:test-discovery-boundaries --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05763
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🦀️source🕸️graph",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-graph --excludeTaskDependencies --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05764
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️scaffolding",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-scaffolding --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05765
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️inventory",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-inventory --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05766
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️reachability",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-reachability --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05767
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧬️mutation🛂️type-origin",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-mutation-authority-type-origin --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05768
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05769
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔁️revision",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-revision --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0577
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library📥️inference🔍️imports",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-nx-project-inference-imports --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05771
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🔗️participation",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-participation --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05772
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧾️serialization🔣️json",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-canonical-json --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05773
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧹️normalization🏗️source-services",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-source-services --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05774
      }
    },
    {
      "name": "🧪️test🦑️repo📚️library🧱️rust-source-direction🪵️root",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-source-roots --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05775
      }
    },
    {
      "name": "🧪️test🧰️framework🎠️kernel🫧️transient🧬️contract",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-contract --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05776
      }
    },
    {
      "name": "🧪️test🧰️framework🫧️transient📦️retained",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-transient:test-retained --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05777
      }
    },
    {
      "name": "🧪️test🧰️framework🧬️schema✅️validator🧩️shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-validator-ts:test-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05778
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-source --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05779
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite-native --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057791
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🏭️scalar-schema🧪️all",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run-many --targets=test-snapshot-sqlite --projects=@semio-tech/stdio-pdf-rs,@semio-tech/stdio-dxf-rs,@semio-tech/stdio-jpg-rs --parallel=1 --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057792
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio🖊️dwg🎛️controlled-metadata",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-controlled-metadata --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0578
      }
    },
    {
      "name": "🧪️test✏️s🗄️stdio📖️pdf♻️recursive-retirement",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pdf-rs:test-recursive-retirement --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05781
      }
    },
    {
      "name": "🧪️test🦑️repo🧱️rust🧫️fixture-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-fixture-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05782
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05783
      }
    },
    {
      "name": "🧪️test🧰️framework💻️os🔌️plugin🤝️cooperative-host🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:cooperative-host-native-check --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/cooperative-host"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057831
      }
    },
    {
      "name": "🧪️test🌱️value🛬️portable",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-portable --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05784
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🟦️source",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- source",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05785
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache -- native",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057851
      }
    },
    {
      "name": "🧪️test✏️s🖊️dwg🏗️grammar-shape",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-dwg-rs:test-grammar-shape --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_LEVEL": "long",
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/test-artifacts"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057852
      }
    },
    {
      "name": "🧪️test🦑️repo🔐️pool-use-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-pool-use-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/pool-use-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05786
      }
    },
    {
      "name": "🧪️test🦑️repo🔔️deferred-wake-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-deferred-wake-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/deferred-wake-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05788
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05789
      }
    },
    {
      "name": "🧪️test🦑️repo📍️rust-family-ownership🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-lib:test-rust-family-ownership-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework/captured-family"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.057891
      }
    },
    {
      "name": "🧪️test🧬️schema🧱️neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-schema:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.0579
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🧩️composition",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-composition --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05791
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-native-oracles --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-oracle-native"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05792
      }
    },
    {
      "name": "🧪️test🧬️schema📇️registry🦀️test-neutrality",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-registry-rs:test-neutrality --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05793
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05794
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05795
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05796
      }
    },
    {
      "name": "🧪️test🧬️schema📶️state🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-state-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05797
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05798
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-quick",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-quick --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05799
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-long",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-long --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.058
      }
    },
    {
      "name": "🧪️test🧬️schema🧩️composition🦀️test-exhaustive",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/schema-composition-rs:test-exhaustive --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-schema-neutrality"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05801
      }
    },
    {
      "name": "🧪️test📕️norm🧾️definition🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/norm-artifact-contract-rs:test-definition-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/norm-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05802
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05803
      }
    },
    {
      "name": "🧪️test🧩️puzzle🧵️retained🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-puzzle-retained-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-puzzle-retained"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05804
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture📍️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05805
      }
    },
    {
      "name": "🧪️test🪐️space🧫️fixture🦀️native",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-space-fixture-sources-test:test-native --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-space-fixture-sources"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05806
      }
    },
    {
      "name": "🧪️test🔌️plugin🏗️fixture🧬️interfaces",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-plugin:test-fixture-channel-interfaces --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/fixture-channel-interfaces"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05807
      }
    },
    {
      "name": "🧪️test📽️pptx📐️transform🔣️wire",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-pptx-rs:test-transform-wire --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/pptx-transform-wire"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05809
      }
    },
    {
      "name": "🧪️test🛠️tool🕸️rows🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-tool-machine-rs:test-node-graph-row-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/node-graph-row-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05810
      }
    },
    {
      "name": "🧪️test🗄️stdio🔮️oracle🖊️drawing-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-drawing-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio-drawing"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05811
      }
    },
    {
      "name": "🧪️test🌱️value🏷️type🧬️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/value-rs:test-type-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/value-type-ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05813
      }
    },
    {
      "name": "🧪️test🖱️ui🎚️ring📬️press",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/ui-rs:test-control-commit --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ui-neutrality/ring-press"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05815
      }
    },
    {
      "name": "🧪️test🗣️dsl🧱️ownership",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-dsl-rs:test-ownership --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-lexical/ownership"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05814
      }
    },
    {
      "name": "🧪️test🌎️hub🧫️private-reader",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/hub-stdio-test-oracle:test-private-reader --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "env": {
        "SEMIO_TEST_ARTIFACT_DIR": "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/private-reader-preservation"
      },
      "presentation": {
        "group": "9_gates",
        "order": 900.05817
      }
    }
  ],
  "compounds": [
    {
      "name": "🧭️compound🖥️s⚛️react🌉️os-mcp",
      "configurations": [
        "🛠️dev🌉️os-mcp🌐️http",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🖥️s⚛️react🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.15
      }
    },
    {
      "name": "🧭️compound🖥️s👥️users🗄️os-hub",
      "configurations": [
        "🛠️dev🗄️os-hub",
        "🛠️dev🪐️space👤️1⚛️react",
        "🛠️dev🪐️space👤️2⚛️react"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 386.16
      }
    },
    {
      "name": "🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor",
      "configurations": [
        "🛠️dev🎓️teaching🛂️proctor",
        "🛠️dev🎓️teaching🏛️architecture❓️quiz"
      ],
      "stopAll": true,
      "presentation": {
        "group": "3_dev",
        "order": 213.62
      }
    }
  ],
  "inputs": [
    {
      "id": "wgpuEmbeddedConfiguration",
      "type": "promptString",
      "description": "Ticket input configuration for actual embedded renderer acceptance",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️embedded-browser/🧊️wgpu.json"
    },
    {
      "id": "wgpuEmbeddedArtifacts",
      "type": "promptString",
      "description": "Ticket directory for embedded browser receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/embedded-browser"
    },
    {
      "id": "wgpuDockReactServe",
      "type": "promptString",
      "description": "React renderer host URL",
      "default": "http://127.0.0.1:7300/"
    },
    {
      "id": "wgpuDockServe",
      "type": "promptString",
      "description": "WGPU renderer host URL",
      "default": "http://127.0.0.1:7301/?plugin=puzzle3d"
    },
    {
      "id": "wgpuDockArtifacts",
      "type": "promptString",
      "description": "Ticket directory for Dock acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/dock-browser"
    },
    {
      "id": "wgpuMediaServe",
      "type": "promptString",
      "description": "WGPU stdio media viewer host URL",
      "default": "http://127.0.0.1:7303/?plugin=stdio-wav"
    },
    {
      "id": "wgpuMediaLocale",
      "type": "pickString",
      "description": "Explicit acceptance language",
      "options": [
        "en",
        "de"
      ],
      "default": "en"
    },
    {
      "id": "wgpuMediaArtifacts",
      "type": "promptString",
      "description": "Ticket directory for media acceptance receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/media-browser"
    },
    {
      "id": "rendererParityRenderer",
      "type": "pickString",
      "description": "Renderer comparison",
      "options": [
        "paired",
        "react",
        "wgpu"
      ],
      "default": "paired"
    },
    {
      "id": "rendererParityJourneyArtifacts",
      "type": "promptString",
      "description": "Ticket directory for shell interaction receipts",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/shell-interaction"
    },
    {
      "id": "nativeScaleRegistry",
      "type": "promptString",
      "description": "Native scale registry JSON path",
      "default": "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/🤖️generated/📇️registry/🔣️.json"
    },
    {
      "id": "gisVerification",
      "type": "pickString",
      "description": "GIS verification",
      "options": [
        "imports",
        "watcher",
        "map",
        "graph"
      ],
      "default": "map"
    },
    {
      "id": "nxCacheTicket",
      "type": "promptString",
      "description": "Active ticket directory for Nx diagnostics"
    },
    {
      "id": "artifactPackageProject",
      "type": "promptString",
      "description": "Artifact package Nx project, for example @semio-tech/stdio-pdf-rs"
    },
    {
      "id": "artifactPackageTarget",
      "type": "pickString",
      "description": "Artifact package target",
      "options": [
        "build",
        "check",
        "test"
      ]
    },
    {
      "id": "catalogFreshBuildRoot",
      "type": "promptString",
      "description": "Absolute fresh plugin catalog build root"
    },
    {
      "id": "acceptanceHubUrl",
      "type": "promptString",
      "description": "Hub under acceptance (origin)",
      "default": "http://127.0.0.1:8787"
    },
    {
      "id": "acceptanceServeUrl",
      "type": "promptString",
      "description": "s React serve joined to that hub",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceLocalServeUrl",
      "type": "promptString",
      "description": "Local-only s React serve (every plugin loaded; launch row 🛠️dev🪐️space⚛️react🔒local-only)",
      "default": "http://127.0.0.1:6070/"
    },
    {
      "id": "acceptanceHubAdminCapability",
      "type": "promptString",
      "description": "The hub launcher's admin-capability.json (0600) for gates that read the hub's connection census",
      "default": ""
    },
    {
      "id": "acceptanceUsers",
      "type": "promptString",
      "description": "Optional JSON file {\"users\":[{\"email\",\"password\"}…]} with the hub's test users (empty: each gate's development users)",
      "default": ""
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️ownership🌎️hub🧩️compositions🟦️",
      "command": "bun nx run @semio-tech/hub-compositions:ownership",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.06 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "🧪️native🌎️hub🧩️compositions🦀️",
      "command": "bun nx run @semio-tech/hub-compositions:native",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "4_gate", "order": 900.07 }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️check🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.08
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.09
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-quick🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-quick",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-long🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-long",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.11
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️test-exhaustive🧪️repo-test-host🦀️",
      "command": "bun nx run semio-repo-test-host:test-exhaustive",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.12
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️mutation-verb-vocabulary-check🧬️mutation-verbs🟦️",
      "command": "bun nx run @semio-tech/framework-os:mutation-verb-vocabulary-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.13
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.14
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️verify-inference-client-native🧬️gis-gismap🦀️",
      "command": "bun nx run @semio-tech/gis-gismap-rs:verify-inference-client-native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.15
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️contract-check📦️component-deployment🟦️",
      "command": "bun nx run @semio-tech/component-deployment-contract:contract-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.16
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-source🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.17
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️generation3d-semantic-wire-native🧊️procedural🦀️",
      "command": "bun nx run @semio-tech/procedural-generation3d-rs:semantic-wire-check -- native",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1701
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-composition-source🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:mcp-composition-source-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.18
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-mcp-untrusted-content🧩️s🟦️",
      "command": "bun nx run @semio-tech/s-services-native:untrusted-content-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1801
      }
    },
    {
      "type": "node-terminal",
      "request": "launch",
      "name": "⚖️services-fixture-ownership🧩️s🦀️",
      "command": "bun nx run @semio-tech/s-services-native:fixture-ownership-check",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1802
      }
    }
,
    {
      "name": "⚖️check-wgpu-boot-cache-inputs🧊️wgpu",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/framework-renderer-wgpu:check-boot-cache-inputs --skip-nx-cache",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 900.1803
      }
    },
    {
      "id": "stdioRemovalArtifacts",
      "type": "promptString",
      "description": "Ticket output directory containing the retained copied workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio"
    },
    {
      "id": "stdioRemovalSnapshot",
      "type": "promptString",
      "description": "Retained workspace copy with only AVI absent, relative to the workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-stdio/parent-avi-removal"
    },
    {
      "id": "graphContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned graph contract artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/graph-contract"
    },
    {
      "id": "svgVideoArtifacts",
      "type": "promptString",
      "description": "Ticket-owned SVG video artifacts directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/svg-video"
    },
    {
      "id": "processContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned framework process contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/process-contract"
    },
    {
      "id": "identityContractArtifacts",
      "type": "promptString",
      "description": "Ticket-owned identity contract output directory relative to workspace",
      "default": ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-root/grapheme-removal"
    }
  ],

  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated
  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.
  "devLaunchers": {
    "aggregator": {
      "namePrefix": "♻️mit-bestand🧩️puzzle🧊️3d",
      "order": 212
    },
    "animate": {
      "namePrefix": "🎞️animate🎬️presentation",
      "order": 170,
      "wgpuOrder": 170.1
    },
    "architect": {
      "namePrefix": "🏛️architect🏛️program",
      "order": 142,
      "wgpuOrder": 142.1
    },
    "aussuchen": {
      "namePrefix": "♻️mit-bestand🪵️sourcing🗂️curation",
      "order": 216
    },
    "bearbeiten": {
      "namePrefix": "♻️mit-bestand🏭️process🧊️process3d",
      "order": 217
    },
    "block2d": {
      "namePrefix": "🧱️block◻️2d",
      "order": 261,
      "wgpuOrder": 261.1
    },
    "block3d": {
      "namePrefix": "🧱️block🧊️3d",
      "order": 262,
      "wgpuOrder": 262.1
    },
    "block5d": {
      "namePrefix": "🧱️block🖐️5d",
      "order": 263,
      "wgpuOrder": 263.1
    },
    "cad": {
      "namePrefix": "📐️cad",
      "order": 10,
      "wgpuOrder": 10.1
    },
    "dag": {
      "namePrefix": "🕸️dag",
      "order": 140,
      "wgpuOrder": 140.1
    },
    "draw": {
      "namePrefix": "🖍️draw🖍️drawing",
      "order": 387,
      "wgpuOrder": 387.1
    },
    "fem2d": {
      "namePrefix": "🏗️fem◻️2d",
      "order": 392,
      "wgpuOrder": 392.1
    },
    "fem3d": {
      "namePrefix": "🏗️fem🧊️3d",
      "order": 392.3,
      "wgpuOrder": 392.4
    },
    "flow": {
      "namePrefix": "🌊️flow",
      "order": 150,
      "wgpuOrder": 150.1
    },
    "forms": {
      "namePrefix": "📋️forms",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "generator": {
      "namePrefix": "♻️mit-bestand🌀️procedural🧊️generation3d",
      "order": 214
    },
    "gis2d": {
      "namePrefix": "🌍️gis🗺️gismap",
      "order": 160,
      "wgpuOrder": 160.1
    },
    "gis3d": {
      "namePrefix": "🌍️gis🏔️gisterrain",
      "order": 160.3,
      "wgpuOrder": 160.4
    },
    "imperative": {
      "namePrefix": "📜️imperative📜️procedure",
      "order": 155,
      "wgpuOrder": 155.1
    },
    "koordinator": {
      "namePrefix": "♻️mit-bestand📐️cad",
      "order": 215
    },
    "layout": {
      "namePrefix": "📏️layout",
      "order": 158,
      "wgpuOrder": 158.1
    },
    "lowpoly": {
      "namePrefix": "💠️lowpoly",
      "order": 157,
      "wgpuOrder": 157.1
    },
    "mathematical": {
      "namePrefix": "➗️mathematical➗️equation",
      "order": 141,
      "wgpuOrder": 141.1
    },
    "note": {
      "namePrefix": "🗒️note",
      "order": 388,
      "wgpuOrder": 388.1
    },
    "bitmap": {
      "namePrefix": "🀄️wfc🖼️bitmap",
      "order": 201,
      "wgpuOrder": 201.1
    },
    "grid2d": {
      "namePrefix": "🀄️wfc🔲️grid2d",
      "order": 202,
      "wgpuOrder": 202.1
    },
    "wfc2d": {
      "namePrefix": "🀄️wfc◻️2d",
      "order": 203,
      "wgpuOrder": 203.1
    },
    "grid3d": {
      "namePrefix": "🀄️wfc🧱️grid3d",
      "order": 204,
      "wgpuOrder": 204.1
    },
    "wfc3d": {
      "namePrefix": "🀄️wfc🧊️3d",
      "order": 205,
      "wgpuOrder": 205.1
    },
    "generation2d": {
      "namePrefix": "🌀️procedural🌀️generation2d",
      "order": 180,
      "wgpuOrder": 180.1
    },
    "generation3d": {
      "namePrefix": "🌀️procedural🧊️generation3d",
      "order": 190,
      "wgpuOrder": 190.1
    },
    "process3d": {
      "namePrefix": "🏭️process🧊️process3d",
      "order": 195,
      "wgpuOrder": 195.1
    },
    "puzzle2d": {
      "namePrefix": "🧩️puzzle◻️2d",
      "order": 220,
      "wgpuOrder": 220.1
    },
    "puzzle3d": {
      "namePrefix": "🧩️puzzle🧊️3d",
      "order": 230,
      "wgpuOrder": 230.1
    },
    "puzzle5d": {
      "namePrefix": "🧩️puzzle🖐️5d",
      "order": 250,
      "wgpuOrder": 250.1
    },
    "raster": {
      "namePrefix": "🖨️raster",
      "order": 386,
      "wgpuOrder": 386.1
    },
    "reasoning-wires": {
      "namePrefix": "💡️reasoning🔌️wires",
      "order": 270,
      "wgpuOrder": 270.1
    },
    "remodel": {
      "namePrefix": "📸️remodel📸️remodeling",
      "order": 389,
      "wgpuOrder": 389.1
    },
    "s": {
      "namePrefix": "🪐️space",
      "order": 386.2,
      "wgpuOrder": 386,
      "env": {
        "S_HUB_URL": "http://127.0.0.1:8787",
        "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-dev"
      },
      "users": {
        "namePrefixPattern": "🖥️s👤️{N}",
        "emailPattern": "user{N}@semio.dev",
        "env": {
          "S_HUB_URL": "http://127.0.0.1:8787",
          "S_DATA_DIR": "${workspaceFolder}/.🧬semio/🔗space/s-user{N}"
        }
      }
    },
    "sequence": {
      "namePrefix": "🎬️sequence",
      "order": 156,
      "wgpuOrder": 156.1
    },
    "shooting": {
      "namePrefix": "🎥️shooting",
      "order": 290,
      "wgpuOrder": 290.1
    },
    "sourcing": {
      "namePrefix": "🪵️sourcing🗂️curation",
      "order": 196,
      "wgpuOrder": 196.1
    },
    "trinity-jack": {
      "namePrefix": "🔱️trinity🔌️jack",
      "order": 360,
      "wgpuOrder": 360.1
    },
    "trinity-rewriting": {
      "namePrefix": "🔱️trinity♻️rewriting",
      "order": 380,
      "wgpuOrder": 380.1
    },
    "vcs": {
      "namePrefix": "🌿️vcs",
      "order": 385,
      "wgpuOrder": 385.1
    },
    "verfolgen": {
      "namePrefix": "♻️mit-bestand🌍️gis🗺️gismap",
      "order": 218
    },
    "writer": {
      "namePrefix": "✒️writer",
      "order": 387,
      "wgpuOrder": 387.1
    }
  },
  // 📋️projectLaunchers — how every declared `📋️project.json` target becomes a launch row (not part of the generated
  // output); see 🚀️launch/🟦️.ts `renderProjectTargetLaunchers` for the contract. A curated row above that runs a target wins.
  "projectLaunchers": {
    "familyMinimumProjects": 3,
    "familyEmoji": "📋️",
    "fallbackClass": "run",
    "classes": [
      {
        "id": "dev",
        "emoji": "🛠️",
        "group": "3_dev",
        "orderBase": 900,
        "tokens": [
          "dev",
          "serve",
          "start",
          "watch",
          "activate",
          "open",
          "launch",
          "inspect",
          "demo",
          "playground",
          "attach"
        ]
      },
      {
        "id": "build",
        "emoji": "📦️",
        "group": "4_build",
        "orderBase": 900,
        "tokens": [
          "build",
          "package",
          "wasm",
          "publish",
          "release",
          "bundle",
          "generate",
          "generator",
          "typegen",
          "codegen",
          "preview",
          "deps",
          "fonts",
          "prepare",
          "materialize",
          "install",
          "compile",
          "sign",
          "deploy",
          "bootstrap",
          "clean",
          "prune",
          "restage",
          "rebuild",
          "setup",
          "format",
          "fix",
          "regenerate"
        ]
      },
      {
        "id": "gate",
        "emoji": "⚖️",
        "group": "4_gate",
        "orderBase": 900,
        "tokens": [
          "test",
          "check",
          "verify",
          "lint",
          "typecheck",
          "oracle",
          "native",
          "e2e",
          "probe",
          "audit",
          "census",
          "bench",
          "parity",
          "law",
          "laws",
          "drill",
          "smoke",
          "conformance",
          "contract",
          "validate",
          "scan",
          "report",
          "doctor",
          "discover",
          "inventory",
          "metrics"
        ]
      },
      {
        "id": "run",
        "emoji": "▶️",
        "group": "3_dev",
        "orderBase": 950,
        "tokens": [
          "run"
        ]
      }
    ],
    "languageSegments": {
      "🦀️rust": "🦀️",
      "🟦️typescript": "🟦️",
      "🐍️python": "🐍️"
    },
    "transparentSegments": [
      "📦️packages"
    ],
    "skipDirectories": [
      "node_modules",
      "dist",
      "target",
      "temp",
      "pkg",
      "storybook-static",
      "🤖️generated",
      "🗑️generated",
      "🎫️tickets"
    ]
  }
}

```

## Additive Route Source Receipts

First source-law attempt refused before execution due an authored Unicode-regex brace syntax error; it is retained as prelaw failure, not TDD evidence. Corrected exact source gate genuinely passed5original laws and failed the absent native-target law:103assertions,907msBun,1.8suncachedNx.

After registration, the full source gate genuinely passed6laws/107assertions/zero failures/247msBun/777msuncachedNx. Original full 48-class SHA256 remains `b1c48a55654bc46efdb49cce14da41fb2048868bfb9eb573d44a1f5c0fb9f226`; original complete native features/roster/budget remain byte-identical. Root private-reader command remains registered once. Seed900.05818 is authored once with caller-owned generated target/artifact directories; ordinary launch generation remains Root-owned, no generated file was handpatched.

The separate native command is launched through the new registered uncached Nx route with long active policy, original `oracles` feature and900,000ms per-cohort budget. Full native unit/docs terminal and actual selected compiler input proof are pending. This additive run cannot stand for the original full 48-host route.

## Actual Native Compiler RED and Full Before Law

Original complete selected lower family command reached compiler RED in7.8sNx: fixture `include_str!` had one excessive parent segment. Actual lower family, dependency-free host and external dxf were compiled; no runtime law dispatched. The complete original body below is retained; only the literal `../../../🧫️fixtures/🖊️semantic/🔣️.json` will become `../../🧫️fixtures/🖊️semantic/🔣️.json`, and reversing that literal must reconstruct these bytes. SHA256 `8a97391b77b28d7799f28ec72e5b6c0568550a8e30d92231bc8d2cefb436931e`.

```rust
use super::project_dxf_r12;
use dxf::entities::EntityType;
use semio_repo_test_host::Json;

#[test]
fn semantic_golden_entities_match_the_independent_dxf_reader() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🖊️semantic/🔣️.json")).expect("declared semantic cases");
    for case in cases.as_array().expect("closed case array") {
        let bytes = case["input"].as_str().expect("declared DXF input").as_bytes();
        if let Some(prefix) = case["errorPrefix"].as_str() {
            let error = project_dxf_r12(bytes).expect_err("malformed input refuses");
            assert!(error.starts_with(prefix), "{}: {error}", case["id"]);
            assert!(dxf::Drawing::load(&mut &bytes[..]).is_err());
            continue;
        }
        let drawing = dxf::Drawing::load(&mut &bytes[..]).expect("independent library parses fixture");
        let independent: Vec<serde_json::Value> = drawing.entities().map(|entity| match &entity.specific {
            EntityType::Line(line) => serde_json::json!({ "start": [line.p1.x, line.p1.y, line.p1.z], "end": [line.p2.x, line.p2.y, line.p2.z], "entityKind": "line", "layer": entity.common.layer }),
            _ => panic!("golden cases declare only line entities"),
        }).collect();
        assert_eq!(serde_json::Value::Array(independent), case["entities"], "{}", case["id"]);
        let projected = project_dxf_r12(bytes).expect("owned semantic reader parses fixture");
        let entities = projected.array("entities");
        let expected = case["entities"].as_array().expect("declared entity projection");
        assert_eq!(entities.len(), expected.len(), "{}", case["id"]);
        for (entity, expected) in entities.iter().zip(expected) {
            assert_eq!(entity.str("entityKind"), expected["entityKind"].as_str().unwrap());
            assert_eq!(entity.str("layer"), expected["layer"].as_str().unwrap());
            for key in ["start", "end"] {
                let actual: Vec<f64> = entity.array(key).iter().map(|value| match value { Json::Number(number) => *number, _ => panic!("coordinate must be numeric") }).collect();
                let expected: Vec<f64> = expected[key].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
                assert_eq!(actual, expected, "{key}");
            }
        }
    }
}

```

## Actual Native Numeric Representation RED

Replay2 compiled12.98s and ran the complete one-law cohort:0passed/1 failed/zero ignored/zero filtered,13.6sNx. The actual third-party dxf coordinates are typed f64 and serialize as1.0…6.0; the expected fixture was authored as integer JSON1…6. This is an expected representation mismatch before the owned projection assertion, not permission to loosen it. Original native comparisons remain intact. Coordinate literals below are handcrafted as f64 in all three declared copies; case IDs, DXF inputs and every parsed scalar/object value must remain exact. Full before inputs follow.

### ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧫️fixtures/🖊️semantic/🔣️.json

SHA256 `384efb0428b90056ef78ce2f4aa41f0a1482505e3c50a7bfe41dd4a130d6037d`

```json
[
  {
    "id": "line",
    "input": "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n1.0\n20\n2.0\n30\n3.0\n11\n4.0\n21\n5.0\n31\n6.0\n0\nENDSEC\n0\nEOF\n",
    "entities": [
      {
        "start": [
          1,
          2,
          3
        ],
        "end": [
          4,
          5,
          6
        ],
        "entityKind": "line",
        "layer": "0"
      }
    ]
  },
  {
    "id": "empty",
    "input": "0\nSECTION\n2\nENTITIES\n0\nENDSEC\n0\nEOF\n",
    "entities": []
  },
  {
    "id": "malformed",
    "input": "not a group code\n",
    "errorPrefix": "dxf oracle: load failed:"
  }
]

```

Exact ordered coordinate-array literal inverses:

```json
[
  [
    "\n          1,\n          2,\n          3\n        ",
    "\n          1.0,\n          2.0,\n          3.0\n        "
  ],
  [
    "\n          4,\n          5,\n          6\n        ",
    "\n          4.0,\n          5.0,\n          6.0\n        "
  ]
]
```

### 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🖊️drawing-reader/🔣️.json

SHA256 `3a3603407c5a86d00ac019d144b60f8340de4803a9d6e487e495cb7c1b21f757`

```json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing",
  "package": "semio-s-plugin-stdio-drawing-test-oracle",
  "library": "semio_s_plugin_stdio_drawing_test_oracle",
  "externalDependency": {
    "version": "0.6",
    "optional": true
  },
  "functions": [
    {
      "name": "load",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 4880,
      "end": 5038,
      "declaration": "fn load(bytes: &[u8]) -> Result<Drawing, String> {\n        Drawing::load(&mut &bytes[..]).map_err(|error| format!(\"dxf oracle: load failed: {error:?}\"))\n    }",
      "body": "{\n        Drawing::load(&mut &bytes[..]).map_err(|error| format!(\"dxf oracle: load failed: {error:?}\"))\n    }",
      "sha256": "e57dbfbf4802509d8a7256b8f76fb16c5c19f54e5166b5970cc216536dacb2c2",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧰️support/🦀️.rs"
    },
    {
      "name": "point_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 6091,
      "end": 6214,
      "declaration": "fn point_json(p: &Point) -> Json {\n        Json::Array(vec![Json::Number(p.x), Json::Number(p.y), Json::Number(p.z)])\n    }",
      "body": "{\n        Json::Array(vec![Json::Number(p.x), Json::Number(p.y), Json::Number(p.z)])\n    }",
      "sha256": "0be95492ce005e0ae8138c58e99b2a3acd0f74d1ef321cc4e25d659503e63953",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧰️support/🦀️.rs"
    },
    {
      "name": "obj",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 6219,
      "end": 6354,
      "declaration": "fn obj(entries: Vec<(&str, Json)>) -> Json {\n        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())\n    }",
      "body": "{\n        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())\n    }",
      "sha256": "a59c66f6be5efc35524445ff9b27e9fcbf50ca081056dcc6dc6ff7399a424841",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧰️support/🦀️.rs"
    },
    {
      "name": "entity_to_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 10808,
      "end": 12451,
      "declaration": "fn entity_to_json(entity: &Entity) -> Result<Json, String> {\n        let (kind, mut fields): (&str, Vec<(String, Json)>) = match &entity.specific {\n            EntityType::Line(l) => (\"line\", vec![(\"start\".to_string(), point_json(&l.p1)), (\"end\".to_string(), point_json(&l.p2))]),\n            EntityType::Circle(c) => (\"circle\", vec![(\"center\".to_string(), point_json(&c.center)), (\"radius\".to_string(), Json::Number(c.radius))]),\n            EntityType::Arc(a) => {\n                (\"arc\", vec![(\"center\".to_string(), point_json(&a.center)), (\"radius\".to_string(), Json::Number(a.radius)), (\"startAngle\".to_string(), Json::Number(a.start_angle)), (\"endAngle\".to_string(), Json::Number(a.end_angle))])\n            }\n            EntityType::Text(t) => (\"text\", vec![(\"position\".to_string(), point_json(&t.location)), (\"height\".to_string(), Json::Number(t.text_height)), (\"value\".to_string(), Json::String(t.value.clone()))]),\n            EntityType::Solid(s) => (\"solid\", vec![(\"points\".to_string(), Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),\n            EntityType::Insert(i) => (\"insert\", vec![(\"blockName\".to_string(), Json::String(i.name.clone())), (\"position\".to_string(), point_json(&i.location))]),\n            other => return Err(format!(\"dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}\")),\n        };\n        fields.push((\"entityKind\".to_string(), Json::String(kind.to_string())));\n        fields.push((\"layer\".to_string(), Json::String(entity.common.layer.clone())));\n        Ok(Json::Object(fields))\n    }",
      "body": "{\n        let (kind, mut fields): (&str, Vec<(String, Json)>) = match &entity.specific {\n            EntityType::Line(l) => (\"line\", vec![(\"start\".to_string(), point_json(&l.p1)), (\"end\".to_string(), point_json(&l.p2))]),\n            EntityType::Circle(c) => (\"circle\", vec![(\"center\".to_string(), point_json(&c.center)), (\"radius\".to_string(), Json::Number(c.radius))]),\n            EntityType::Arc(a) => {\n                (\"arc\", vec![(\"center\".to_string(), point_json(&a.center)), (\"radius\".to_string(), Json::Number(a.radius)), (\"startAngle\".to_string(), Json::Number(a.start_angle)), (\"endAngle\".to_string(), Json::Number(a.end_angle))])\n            }\n            EntityType::Text(t) => (\"text\", vec![(\"position\".to_string(), point_json(&t.location)), (\"height\".to_string(), Json::Number(t.text_height)), (\"value\".to_string(), Json::String(t.value.clone()))]),\n            EntityType::Solid(s) => (\"solid\", vec![(\"points\".to_string(), Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),\n            EntityType::Insert(i) => (\"insert\", vec![(\"blockName\".to_string(), Json::String(i.name.clone())), (\"position\".to_string(), point_json(&i.location))]),\n            other => return Err(format!(\"dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}\")),\n        };\n        fields.push((\"entityKind\".to_string(), Json::String(kind.to_string())));\n        fields.push((\"layer\".to_string(), Json::String(entity.common.layer.clone())));\n        Ok(Json::Object(fields))\n    }",
      "sha256": "fe1adfd7b8206da8d072cdf331657855109ae50b357afbf1901eaee1f481e10d",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "entity_projection",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 12527,
      "end": 12746,
      "declaration": "fn entity_projection(entity: &Entity) -> Json {\n        entity_to_json(entity).unwrap_or_else(|_| obj(vec![(\"entityKind\", Json::String(\"other\".to_string())), (\"layer\", Json::String(entity.common.layer.clone()))]))\n    }",
      "body": "{\n        entity_to_json(entity).unwrap_or_else(|_| obj(vec![(\"entityKind\", Json::String(\"other\".to_string())), (\"layer\", Json::String(entity.common.layer.clone()))]))\n    }",
      "sha256": "8b27e27506a98579f30d4652e226e39fa1c2841f0cac9425e52fc93a2ca20afb",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "layer_to_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 13292,
      "end": 13525,
      "declaration": "fn layer_to_json(layer: &Layer) -> Json {\n        obj(vec![(\"name\", Json::String(layer.name.clone())), (\"color\", Json::Number(layer.color.index().unwrap_or(7) as f64)), (\"linetype\", Json::String(layer.line_type_name.clone()))])\n    }",
      "body": "{\n        obj(vec![(\"name\", Json::String(layer.name.clone())), (\"color\", Json::Number(layer.color.index().unwrap_or(7) as f64)), (\"linetype\", Json::String(layer.line_type_name.clone()))])\n    }",
      "sha256": "9a11c194b50ff367982866fbb43eed4543950169e4b6579a7ae753f0b8a5c315",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "style_to_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 13914,
      "end": 14085,
      "declaration": "fn style_to_json(style: &Style) -> Json {\n        obj(vec![(\"name\", Json::String(style.name.clone())), (\"font\", Json::String(style.primary_font_file_name.clone()))])\n    }",
      "body": "{\n        obj(vec![(\"name\", Json::String(style.name.clone())), (\"font\", Json::String(style.primary_font_file_name.clone()))])\n    }",
      "sha256": "474fff9ae082e24390746dfe497fecb3d52e9544075b3fc48a8d6c6cd40131f9",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "linetype_to_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 14464,
      "end": 14646,
      "declaration": "fn linetype_to_json(linetype: &LineType) -> Json {\n        obj(vec![(\"name\", Json::String(linetype.name.clone())), (\"description\", Json::String(linetype.description.clone()))])\n    }",
      "body": "{\n        obj(vec![(\"name\", Json::String(linetype.name.clone())), (\"description\", Json::String(linetype.description.clone()))])\n    }",
      "sha256": "d2f19fe554df028fc01d1fdbbece7fce57f2f3e679cc8850ff4f44d4dcfc1213",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "block_to_json",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 14970,
      "end": 15285,
      "declaration": "fn block_to_json(block: &Block) -> Result<Json, String> {\n        let entities = block.entities.iter().map(entity_to_json).collect::<Result<Vec<_>, String>>()?;\n        Ok(obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(entities))]))\n    }",
      "body": "{\n        let entities = block.entities.iter().map(entity_to_json).collect::<Result<Vec<_>, String>>()?;\n        Ok(obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(entities))]))\n    }",
      "sha256": "3f724bfba406e929f3362334d94e7f667acbf84bc8ef9fab7a8ed68fe3147734",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "block_projection",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 15604,
      "end": 15838,
      "declaration": "fn block_projection(block: &Block) -> Json {\n        block_to_json(block).unwrap_or_else(|_| obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(vec![]))]))\n    }",
      "body": "{\n        block_to_json(block).unwrap_or_else(|_| obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(vec![]))]))\n    }",
      "sha256": "bc2269bd016f252f1115c2f26a35ec93e94eefda027e2617785054ef7a12b9e0",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    },
    {
      "name": "project_dxf_r12",
      "source": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "start": 33468,
      "end": 34411,
      "declaration": "fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String> {\n        let drawing = load(bytes)?;\n        let layers: Vec<Json> = drawing.layers().map(layer_to_json).collect();\n        let styles: Vec<Json> = drawing.styles().map(style_to_json).collect();\n        let linetypes: Vec<Json> = drawing.line_types().map(linetype_to_json).collect();\n        let blocks: Vec<Json> = drawing.blocks().map(block_projection).collect();\n        let entities: Vec<Json> = drawing.entities().map(entity_projection).collect();\n        Ok(obj(vec![\n            (\"acadVersion\", Json::String(format!(\"{:?}\", drawing.header.version))),\n            (\"insertionBase\", point_json(&drawing.header.insertion_base)),\n            (\"layers\", Json::Array(layers)),\n            (\"styles\", Json::Array(styles)),\n            (\"linetypes\", Json::Array(linetypes)),\n            (\"blocks\", Json::Array(blocks)),\n            (\"entities\", Json::Array(entities)),\n        ]))\n    }",
      "body": "{\n        let drawing = load(bytes)?;\n        let layers: Vec<Json> = drawing.layers().map(layer_to_json).collect();\n        let styles: Vec<Json> = drawing.styles().map(style_to_json).collect();\n        let linetypes: Vec<Json> = drawing.line_types().map(linetype_to_json).collect();\n        let blocks: Vec<Json> = drawing.blocks().map(block_projection).collect();\n        let entities: Vec<Json> = drawing.entities().map(entity_projection).collect();\n        Ok(obj(vec![\n            (\"acadVersion\", Json::String(format!(\"{:?}\", drawing.header.version))),\n            (\"insertionBase\", point_json(&drawing.header.insertion_base)),\n            (\"layers\", Json::Array(layers)),\n            (\"styles\", Json::Array(styles)),\n            (\"linetypes\", Json::Array(linetypes)),\n            (\"blocks\", Json::Array(blocks)),\n            (\"entities\", Json::Array(entities)),\n        ]))\n    }",
      "sha256": "51ac03b5de56f616124c5923f4db3adadd2987bed1a87d907a02798866db716d",
      "destination": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs"
    }
  ],
  "originals": [
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs",
      "source": "//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the\n//! registered `dxf` 0.6 reference implementation so the subject's own mutation has an independent\n//! result to be compared against instead of being checked against its own reading. `dxf` reads AND\n//! writes DXF (unlike a reader-only reference), so it is a genuine differential second producer, not\n//! merely an independent projector.\n//!\n//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare\n//! different mutations, and a subset that shares an implementation with another reaches it through\n//! the shared family modules rather than by copying it.\n//!\n//! JSON mutation-spec shape: `{\"kind\": \"<kebab-case-kind>\", \"params\": {...}}`, where `params` IS the leaf wire payload\n//! (design §11) — `{index, layer: DxfLayer}`, `{name, style: DxfStyle}`, `{index, entity: DxfEntity}` (externally tagged,\n//! `{\"circle\": {center, radius, layer}}`), `{index, block: DxfBlock}`, `{name, headerVar: DxfHeaderVar}` and\n//! `{snapshot: DxfSnapshot}` — read here into `dxf`'s own typed model, independently of this subset's codec.\n//!\n//! @see ./🔣️.json — the mutation catalog this module is measured against.\n//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`KINDS`).\n\nuse semio_repo_test_host::Json;\n\n//#region 🔖️Dispatch\n/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.\n/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped\n/// reports as a passing test.\n#[cfg(feature = \"oracles\")]\npub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {\n    imp::oracle_apply_mutation(input, spec)\n}\n\n/// 🔁️ The identity round trip's own producer: `dxf` parses the document and re-serializes it from its own typed\n/// `Drawing` alone.\n#[cfg(feature = \"oracles\")]\npub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {\n    imp::oracle_round_trip(input)\n}\n\n/// 🔁️ Applies the mutation, then applies its own computed inverse (built from the PRE-mutation\n/// state, name/index-aware — mirroring `DxfMutation::inverse`'s own contract) and re-serializes.\n/// `apply(inverse(m), apply(m, base)) == base` by the law, so this is the oracle's own independent\n/// exercise of that law, not a comparison of the implementation with itself: the forward mutation,\n/// the inverse computation and the two applications are all performed by `dxf`, never by this\n/// subset's own codec.\n#[cfg(feature = \"oracles\")]\npub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {\n    imp::oracle_apply_mutation_inverse(input, spec)\n}\n\n/// 📄️ Independent semantic projection of a DXF R12 document, read back by `dxf` itself (never by\n/// this subset's own codec) — used to compare the oracle's and the subject's results under the\n/// `semantic-dxf-r12-v1` comparison profile declared in `./🔣️.json`.\n#[cfg(feature = \"oracles\")]\npub fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String> {\n    imp::project_dxf_r12(bytes)\n}\n\n/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.\n#[cfg(not(feature = \"oracles\"))]\npub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n#[cfg(not(feature = \"oracles\"))]\npub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n#[cfg(not(feature = \"oracles\"))]\npub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n#[cfg(not(feature = \"oracles\"))]\npub fn project_dxf_r12(_bytes: &[u8]) -> Result<Json, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n//#endregion 🔖️Dispatch\n\n/// 🔒️ Every `dxf`-linked helper lives inside this ONE `cfg`-gated module, so the non-`oracles` build\n/// never even parses a `dxf::` path — a single `cfg` at the module boundary instead of one on every\n/// function.\n#[cfg(feature = \"oracles\")]\nmod imp {\n    use super::Json;\n    use dxf::entities::{Arc as DxfArc, Circle, Entity, EntityType, Insert, Line, Solid, Text};\n    use dxf::tables::{Layer, LineType, Style};\n    use dxf::enums::AcadVersion;\n    use dxf::{Block, Color, Drawing, Point};\n\n    //#region 🔖️LoadSave\n    /// 📥️ `dxf` fully parses the ASCII group-code stream into its own typed `Drawing` — never a\n    /// byte-level read of this subset's own model.\n    fn load(bytes: &[u8]) -> Result<Drawing, String> {\n        Drawing::load(&mut &bytes[..]).map_err(|error| format!(\"dxf oracle: load failed: {error:?}\"))\n    }\n\n    /// 📤️ Re-serializes from `dxf`'s own typed model alone.\n    fn save(drawing: &Drawing) -> Result<Vec<u8>, String> {\n        let mut out: Vec<u8> = Vec::new();\n        drawing.save(&mut out).map_err(|error| format!(\"dxf oracle: save failed: {error:?}\"))?;\n        Ok(out)\n    }\n    //#endregion 🔖️LoadSave\n\n    //#region 🔖️JsonHelpers\n    fn number(v: &Json, key: &str) -> f64 {\n        match v.get(key) {\n            Some(Json::Number(n)) => *n,\n            _ => 0.0,\n        }\n    }\n    fn index_of(v: &Json, key: &str) -> usize {\n        number(v, key).max(0.0) as usize\n    }\n    fn coord(arr: &Json, i: usize) -> f64 {\n        match arr {\n            Json::Array(items) => match items.get(i) {\n                Some(Json::Number(n)) => *n,\n                _ => 0.0,\n            },\n            _ => 0.0,\n        }\n    }\n    fn point_of(arr: &Json) -> Point {\n        Point::new(coord(arr, 0), coord(arr, 1), coord(arr, 2))\n    }\n    fn point_from(v: &Json, key: &str) -> Point {\n        point_of(v.get(key).unwrap_or(&Json::Null))\n    }\n    fn point_json(p: &Point) -> Json {\n        Json::Array(vec![Json::Number(p.x), Json::Number(p.y), Json::Number(p.z)])\n    }\n    fn obj(entries: Vec<(&str, Json)>) -> Json {\n        Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())\n    }\n    fn member(v: &Json, key: &str) -> Result<Json, String> {\n        v.get(key).cloned().ok_or_else(|| format!(\"dxf oracle: params carry no `{key}`\"))\n    }\n    //#endregion 🔖️JsonHelpers\n\n    //#region 🔖️EntityCodec\n    /// 📥️ The externally tagged `DxfEntity` wire (`{\"circle\": {center, radius, layer}}`) → `dxf::entities::Entity`, over\n    /// the six typed kinds this subset itself models (line/circle/arc/text/solid/insert) — `Other` excepted (raw\n    /// retention has no meaningful independent-library construction).\n    fn build_entity(wire: &Json) -> Result<Entity, String> {\n        let (tag, spec) = match wire {\n            Json::Object(fields) if fields.len() == 1 => (fields[0].0.as_str(), &fields[0].1),\n            other => return Err(format!(\"dxf oracle: a DxfEntity is a one-member tagged object, found {other:?}\")),\n        };\n        let specific = match tag {\n            \"line\" => EntityType::Line(Line { p1: point_from(spec, \"start\"), p2: point_from(spec, \"end\"), ..Default::default() }),\n            \"circle\" => EntityType::Circle(Circle { center: point_from(spec, \"center\"), radius: number(spec, \"radius\"), ..Default::default() }),\n            \"arc\" => EntityType::Arc(DxfArc { center: point_from(spec, \"center\"), radius: number(spec, \"radius\"), start_angle: number(spec, \"startAngle\"), end_angle: number(spec, \"endAngle\"), ..Default::default() }),\n            \"text\" => EntityType::Text(Text { location: point_from(spec, \"position\"), text_height: number(spec, \"height\"), value: spec.str(\"value\"), text_style_name: \"STANDARD\".to_string(), ..Default::default() }),\n            \"solid\" => {\n                let points = spec.array(\"points\");\n                let corner = |i: usize| point_of(points.get(i).unwrap_or(&Json::Null));\n                EntityType::Solid(Solid { first_corner: corner(0), second_corner: corner(1), third_corner: corner(2), fourth_corner: corner(3), ..Default::default() })\n            }\n            \"insert\" => {\n                let scale = spec.get(\"scale\").cloned().unwrap_or(Json::Null);\n                EntityType::Insert(Insert { name: spec.str(\"blockName\"), location: point_from(spec, \"position\"), x_scale_factor: coord(&scale, 0), y_scale_factor: coord(&scale, 1), z_scale_factor: coord(&scale, 2), rotation: number(spec, \"rotation\"), ..Default::default() })\n            }\n            other => return Err(format!(\"dxf oracle: unsupported DxfEntity tag {other:?}\")),\n        };\n        let mut entity = Entity::new(specific);\n        entity.common.layer = spec.str(\"layer\");\n        Ok(entity)\n    }\n\n    /// 📤️ `dxf::entities::Entity` → the externally tagged `DxfEntity` wire, the exact inverse of [`build_entity`] — how an\n    /// entity's pre-mutation value travels in an inverse spec.\n    fn entity_wire(entity: &Entity) -> Result<Json, String> {\n        let (tag, mut fields): (&str, Vec<(&str, Json)>) = match &entity.specific {\n            EntityType::Line(l) => (\"line\", vec![(\"start\", point_json(&l.p1)), (\"end\", point_json(&l.p2))]),\n            EntityType::Circle(c) => (\"circle\", vec![(\"center\", point_json(&c.center)), (\"radius\", Json::Number(c.radius))]),\n            EntityType::Arc(a) => (\"arc\", vec![(\"center\", point_json(&a.center)), (\"radius\", Json::Number(a.radius)), (\"startAngle\", Json::Number(a.start_angle)), (\"endAngle\", Json::Number(a.end_angle))]),\n            EntityType::Text(t) => (\"text\", vec![(\"position\", point_json(&t.location)), (\"height\", Json::Number(t.text_height)), (\"value\", Json::String(t.value.clone()))]),\n            EntityType::Solid(s) => (\"solid\", vec![(\"points\", Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),\n            EntityType::Insert(i) => (\"insert\", vec![(\"blockName\", Json::String(i.name.clone())), (\"position\", point_json(&i.location)), (\"scale\", Json::Array(vec![Json::Number(i.x_scale_factor), Json::Number(i.y_scale_factor), Json::Number(i.z_scale_factor)])), (\"rotation\", Json::Number(i.rotation))]),\n            other => return Err(format!(\"dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}\")),\n        };\n        fields.push((\"layer\", Json::String(entity.common.layer.clone())));\n        Ok(obj(vec![(tag, obj(fields))]))\n    }\n\n    /// 📄️ `dxf::entities::Entity` → the flat `semantic-dxf-r12-v1` projection shape (`entityKind` plus its fields), and\n    /// recursively for a block's nested entity list.\n    fn entity_to_json(entity: &Entity) -> Result<Json, String> {\n        let (kind, mut fields): (&str, Vec<(String, Json)>) = match &entity.specific {\n            EntityType::Line(l) => (\"line\", vec![(\"start\".to_string(), point_json(&l.p1)), (\"end\".to_string(), point_json(&l.p2))]),\n            EntityType::Circle(c) => (\"circle\", vec![(\"center\".to_string(), point_json(&c.center)), (\"radius\".to_string(), Json::Number(c.radius))]),\n            EntityType::Arc(a) => {\n                (\"arc\", vec![(\"center\".to_string(), point_json(&a.center)), (\"radius\".to_string(), Json::Number(a.radius)), (\"startAngle\".to_string(), Json::Number(a.start_angle)), (\"endAngle\".to_string(), Json::Number(a.end_angle))])\n            }\n            EntityType::Text(t) => (\"text\", vec![(\"position\".to_string(), point_json(&t.location)), (\"height\".to_string(), Json::Number(t.text_height)), (\"value\".to_string(), Json::String(t.value.clone()))]),\n            EntityType::Solid(s) => (\"solid\", vec![(\"points\".to_string(), Json::Array(vec![point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)]))]),\n            EntityType::Insert(i) => (\"insert\", vec![(\"blockName\".to_string(), Json::String(i.name.clone())), (\"position\".to_string(), point_json(&i.location))]),\n            other => return Err(format!(\"dxf oracle: cannot capture inverse value for unsupported entity kind {other:?}\")),\n        };\n        fields.push((\"entityKind\".to_string(), Json::String(kind.to_string())));\n        fields.push((\"layer\".to_string(), Json::String(entity.common.layer.clone())));\n        Ok(Json::Object(fields))\n    }\n\n    /// 📄️ Semantic projection of one entity, for `project_dxf_r12`.\n    fn entity_projection(entity: &Entity) -> Json {\n        entity_to_json(entity).unwrap_or_else(|_| obj(vec![(\"entityKind\", Json::String(\"other\".to_string())), (\"layer\", Json::String(entity.common.layer.clone()))]))\n    }\n    //#endregion 🔖️EntityCodec\n\n    //#region 🔖️TableCodecs\n    fn build_layer(spec: &Json) -> Layer {\n        Layer { name: spec.str(\"name\"), color: Color::from_index(number(spec, \"color\").max(0.0) as u8), line_type_name: spec.str(\"linetype\"), ..Default::default() }\n    }\n    fn layer_wire(layer: &Layer) -> Json {\n        obj(vec![(\"name\", Json::String(layer.name.clone())), (\"color\", Json::Number(layer.color.index().unwrap_or(7) as f64)), (\"linetype\", Json::String(layer.line_type_name.clone())), (\"flags\", Json::Number(0.0))])\n    }\n    fn layer_to_json(layer: &Layer) -> Json {\n        obj(vec![(\"name\", Json::String(layer.name.clone())), (\"color\", Json::Number(layer.color.index().unwrap_or(7) as f64)), (\"linetype\", Json::String(layer.line_type_name.clone()))])\n    }\n\n    fn build_style(spec: &Json) -> Style {\n        Style { name: spec.str(\"name\"), primary_font_file_name: spec.str(\"fontName\"), text_height: 2.5, ..Default::default() }\n    }\n    fn style_wire(style: &Style) -> Json {\n        obj(vec![(\"name\", Json::String(style.name.clone())), (\"flags\", Json::Number(0.0)), (\"fontName\", Json::String(style.primary_font_file_name.clone()))])\n    }\n    fn style_to_json(style: &Style) -> Json {\n        obj(vec![(\"name\", Json::String(style.name.clone())), (\"font\", Json::String(style.primary_font_file_name.clone()))])\n    }\n\n    fn build_linetype(spec: &Json) -> LineType {\n        LineType { name: spec.str(\"name\"), description: spec.str(\"description\"), ..Default::default() }\n    }\n    fn linetype_wire(linetype: &LineType) -> Json {\n        obj(vec![(\"name\", Json::String(linetype.name.clone())), (\"flags\", Json::Number(0.0)), (\"description\", Json::String(linetype.description.clone()))])\n    }\n    fn linetype_to_json(linetype: &LineType) -> Json {\n        obj(vec![(\"name\", Json::String(linetype.name.clone())), (\"description\", Json::String(linetype.description.clone()))])\n    }\n\n    fn build_block(spec: &Json) -> Result<Block, String> {\n        let entities = spec.array(\"entities\").iter().map(build_entity).collect::<Result<Vec<_>, String>>()?;\n        Ok(Block { name: spec.str(\"name\"), layer: \"0\".to_string(), base_point: point_from(spec, \"basePoint\"), entities, ..Default::default() })\n    }\n    fn block_to_json(block: &Block) -> Result<Json, String> {\n        let entities = block.entities.iter().map(entity_to_json).collect::<Result<Vec<_>, String>>()?;\n        Ok(obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(entities))]))\n    }\n    fn block_wire(block: &Block) -> Result<Json, String> {\n        let entities = block.entities.iter().map(entity_wire).collect::<Result<Vec<_>, String>>()?;\n        Ok(obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(entities))]))\n    }\n    fn block_projection(block: &Block) -> Json {\n        block_to_json(block).unwrap_or_else(|_| obj(vec![(\"name\", Json::String(block.name.clone())), (\"basePoint\", point_json(&block.base_point)), (\"entities\", Json::Array(vec![]))]))\n    }\n    //#endregion 🔖️TableCodecs\n\n    //#region 🔖️HeaderVar\n    /// 🏷️ `$INSBASE` is the one generic `$VAR` this oracle mutates directly — `dxf`'s `Header` is a\n    /// fixed typed struct (no arbitrary `$VAR` insertion), so `set-header-var`/`remove-header-var`\n    /// are exercised against a header point every DXF R12 file actually persists. `$INSUNITS` was\n    /// tried first and rejected: `dxf`'s own generated `Header::add_code_pairs` only emits it for\n    /// `version >= AcadVersion::R2000` (confirmed against `target/.../out/generated/header.rs`), so\n    /// it never survives a save/reload of an R12 document at all — not a representable R12 mutation\n    /// target through this reference library, regardless of what this module set in memory.\n    /// `$INSBASE` (`header.insertion_base`) has no such gate — written unconditionally every save.\n    //#endregion 🔖️HeaderVar\n\n    //#region 🔖️OrderedRebuild\n    /// 🧱️ `dxf::Drawing::add_*` only appends; a true insert-at-`index` needs the whole ordered\n    /// collection rebuilt. Repeated per collection kind since `Drawing` exposes no shared trait over\n    /// its five ordered tables.\n    fn insert_layer_at(drawing: &mut Drawing, index: usize, layer: Layer) {\n        let mut items: Vec<Layer> = drawing.layers().cloned().collect();\n        items.insert(index.min(items.len()), layer);\n        while drawing.remove_layer(0).is_some() {}\n        for item in items {\n            drawing.add_layer(item);\n        }\n    }\n    fn insert_style_at(drawing: &mut Drawing, index: usize, style: Style) {\n        let mut items: Vec<Style> = drawing.styles().cloned().collect();\n        items.insert(index.min(items.len()), style);\n        while drawing.remove_style(0).is_some() {}\n        for item in items {\n            drawing.add_style(item);\n        }\n    }\n    fn insert_linetype_at(drawing: &mut Drawing, index: usize, linetype: LineType) {\n        let mut items: Vec<LineType> = drawing.line_types().cloned().collect();\n        items.insert(index.min(items.len()), linetype);\n        while drawing.remove_line_type(0).is_some() {}\n        for item in items {\n            drawing.add_line_type(item);\n        }\n    }\n    fn insert_block_at(drawing: &mut Drawing, index: usize, block: Block) {\n        let mut items: Vec<Block> = drawing.blocks().cloned().collect();\n        items.insert(index.min(items.len()), block);\n        while drawing.remove_block(0).is_some() {}\n        for item in items {\n            drawing.add_block(item);\n        }\n    }\n    fn insert_entity_at(drawing: &mut Drawing, index: usize, entity: Entity) {\n        let mut items: Vec<Entity> = drawing.entities().cloned().collect();\n        items.insert(index.min(items.len()), entity);\n        while drawing.remove_entity(0).is_some() {}\n        for item in items {\n            drawing.add_entity(item);\n        }\n    }\n    //#endregion 🔖️OrderedRebuild\n\n    //#region 🔖️Apply\n    /// ▶️ Performs one mutation kind against `drawing` in place — the forward half both\n    /// `oracle_apply_mutation` and `oracle_apply_mutation_inverse` share (the latter calls it twice:\n    /// the mutation, then its own computed inverse).\n    fn apply_kind(drawing: &mut Drawing, kind: &str, params: &Json) -> Result<(), String> {\n        match kind {\n            \"set-snapshot\" => {\n                *drawing = snapshot_drawing(&member(params, \"snapshot\")?)?;\n                Ok(())\n            }\n\n            \"set-header-var\" => match params.str(\"name\").as_str() {\n                \"$INSBASE\" => {\n                    drawing.header.insertion_base = header_point(&member(params, \"headerVar\")?)?;\n                    Ok(())\n                }\n                other => Err(format!(\"dxf oracle: unsupported header var {other:?}\")),\n            },\n            \"remove-header-var\" => match params.str(\"name\").as_str() {\n                \"$INSBASE\" => {\n                    drawing.header.insertion_base = Point::origin();\n                    Ok(())\n                }\n                other => Err(format!(\"dxf oracle: unsupported header var {other:?}\")),\n            },\n\n            \"insert-layer\" => {\n                insert_layer_at(drawing, index_of(params, \"index\"), build_layer(&member(params, \"layer\")?));\n                Ok(())\n            }\n            \"remove-layer\" => {\n                let name = params.str(\"name\");\n                let at: Option<usize> = drawing.layers().position(|l| l.name == name);\n                if let Some(at) = at {\n                    drawing.remove_layer(at);\n                }\n                Ok(())\n            }\n            \"set-layer\" => {\n                let name = params.str(\"name\");\n                let replacement = build_layer(&member(params, \"layer\")?);\n                match drawing.layers_mut().find(|l| l.name == name) {\n                    Some(slot) => {\n                        slot.color = replacement.color;\n                        slot.line_type_name = replacement.line_type_name;\n                        Ok(())\n                    }\n                    None => Err(format!(\"dxf oracle: set-layer target {name:?} not found\")),\n                }\n            }\n\n            \"insert-style\" => {\n                insert_style_at(drawing, index_of(params, \"index\"), build_style(&member(params, \"style\")?));\n                Ok(())\n            }\n            \"remove-style\" => {\n                let name = params.str(\"name\");\n                let at: Option<usize> = drawing.styles().position(|s| s.name == name);\n                if let Some(at) = at {\n                    drawing.remove_style(at);\n                }\n                Ok(())\n            }\n            \"set-style\" => {\n                let name = params.str(\"name\");\n                let font = member(params, \"style\")?.str(\"fontName\");\n                match drawing.styles_mut().find(|s| s.name == name) {\n                    Some(slot) => {\n                        slot.primary_font_file_name = font;\n                        Ok(())\n                    }\n                    None => Err(format!(\"dxf oracle: set-style target {name:?} not found\")),\n                }\n            }\n\n            \"insert-linetype\" => {\n                insert_linetype_at(drawing, index_of(params, \"index\"), build_linetype(&member(params, \"linetype\")?));\n                Ok(())\n            }\n            \"remove-linetype\" => {\n                let name = params.str(\"name\");\n                let at: Option<usize> = drawing.line_types().position(|l| l.name == name);\n                if let Some(at) = at {\n                    drawing.remove_line_type(at);\n                }\n                Ok(())\n            }\n            \"set-linetype\" => {\n                let name = params.str(\"name\");\n                let description = member(params, \"linetype\")?.str(\"description\");\n                match drawing.line_types_mut().find(|l| l.name == name) {\n                    Some(slot) => {\n                        slot.description = description;\n                        Ok(())\n                    }\n                    None => Err(format!(\"dxf oracle: set-linetype target {name:?} not found\")),\n                }\n            }\n\n            \"insert-entity\" => {\n                let entity = build_entity(&member(params, \"entity\")?)?;\n                insert_entity_at(drawing, index_of(params, \"index\"), entity);\n                Ok(())\n            }\n            \"remove-entity\" => {\n                drawing.remove_entity(index_of(params, \"index\"));\n                Ok(())\n            }\n            \"set-entity\" => {\n                let index = index_of(params, \"index\");\n                let replacement = build_entity(&member(params, \"entity\")?)?;\n                match drawing.entities_mut().nth(index) {\n                    Some(slot) => {\n                        slot.specific = replacement.specific;\n                        slot.common.layer = replacement.common.layer;\n                        Ok(())\n                    }\n                    None => Err(format!(\"dxf oracle: set-entity target index {index} not found\")),\n                }\n            }\n\n            \"insert-block\" => {\n                let block = build_block(&member(params, \"block\")?)?;\n                insert_block_at(drawing, index_of(params, \"index\"), block);\n                Ok(())\n            }\n            \"remove-block\" => {\n                drawing.remove_block(index_of(params, \"index\"));\n                Ok(())\n            }\n            \"set-block\" => {\n                let index = index_of(params, \"index\");\n                let replacement = build_block(&member(params, \"block\")?)?;\n                match drawing.blocks_mut().nth(index) {\n                    Some(slot) => {\n                        slot.base_point = replacement.base_point;\n                        slot.entities = replacement.entities;\n                        Ok(())\n                    }\n                    None => Err(format!(\"dxf oracle: set-block target index {index} not found\")),\n                }\n            }\n\n            other => Err(format!(\"mutation kind {other:?} has no oracle implementation\")),\n        }\n    }\n    //#endregion 🔖️Apply\n\n    //#region 🔖️SnapshotWire\n    /// 🏷️ The point a `$INSBASE` `DxfHeaderVar` wire carries (`value` is the `{\"kind\": \"point\", \"value\": [x, y, z]}` DxfValue).\n    fn header_point(header_var: &Json) -> Result<Point, String> {\n        let value = member(header_var, \"value\")?;\n        match value.str(\"kind\").as_str() {\n            \"point\" => Ok(point_from(&value, \"value\")),\n            other => Err(format!(\"dxf oracle: `{}` must carry a point, found a {other:?} value\", header_var.str(\"name\"))),\n        }\n    }\n\n    /// 📸️ A whole new drawing built by `dxf` from the `DxfSnapshot` wire alone — `set-snapshot` replaces the document, so\n    /// nothing of the input survives. `Drawing::new()`'s ensured default table entries are dropped so the tables hold exactly\n    /// what the snapshot declares; the header honours `$ACADVER` and `$INSBASE`, the two variables this reference models.\n    fn snapshot_drawing(snapshot: &Json) -> Result<Drawing, String> {\n        let mut drawing = Drawing::new();\n        for header_var in snapshot.array(\"headerVars\") {\n            match header_var.str(\"name\").as_str() {\n                \"$ACADVER\" => drawing.header.version = AcadVersion::from(member(&header_var, \"value\")?.str(\"value\")).map_err(|error| format!(\"dxf oracle: $ACADVER: {error:?}\"))?,\n                \"$INSBASE\" => drawing.header.insertion_base = header_point(&header_var)?,\n                other => return Err(format!(\"dxf oracle: unsupported header var {other:?}\")),\n            }\n        }\n        while drawing.remove_layer(0).is_some() {}\n        while drawing.remove_style(0).is_some() {}\n        while drawing.remove_line_type(0).is_some() {}\n        let tables = snapshot.get(\"tables\").cloned().unwrap_or(Json::Null);\n        for layer in tables.array(\"layers\") {\n            drawing.add_layer(build_layer(&layer));\n        }\n        for style in tables.array(\"styles\") {\n            drawing.add_style(build_style(&style));\n        }\n        for linetype in tables.array(\"linetypes\") {\n            drawing.add_line_type(build_linetype(&linetype));\n        }\n        for block in snapshot.array(\"blocks\") {\n            drawing.add_block(build_block(&block)?);\n        }\n        for entity in snapshot.array(\"entities\") {\n            drawing.add_entity(build_entity(&entity)?);\n        }\n        Ok(drawing)\n    }\n    //#endregion 🔖️SnapshotWire\n\n    //#region 🔖️Inverse\n    /// ↩️ What undoes a forward `(kind, params)` applied to `base`.\n    enum Undo {\n        /// 🚫️ Nothing to undo: the forward step had nothing to act on.\n        Nothing,\n        /// 🔁️ Apply this `(kind, wire params)` on top of the forward result.\n        Apply(String, Json),\n        /// 📦️ Put the original drawing back — a whole-document replacement is undone by the whole original document, which\n        /// `dxf`'s typed `Header` cannot restate as a `DxfSnapshot` wire.\n        Original,\n    }\n\n    /// ↩️ `DxfMutation::inverse`'s own per-variant contract, transplanted onto `dxf::Drawing`: reads whatever pre-state it\n    /// needs from `base` (name/index-aware) and answers the undo as leaf wire params.\n    fn inverse_of(base: &Drawing, kind: &str, params: &Json) -> Result<Undo, String> {\n        let apply = |kind: &str, params: Json| Ok(Undo::Apply(kind.to_string(), params));\n        let name = params.str(\"name\");\n        let index = index_of(params, \"index\");\n        let insbase = |name: String| obj(vec![(\"name\", Json::String(name.clone())), (\"headerVar\", obj(vec![(\"name\", Json::String(name)), (\"groupCode\", Json::Number(10.0)), (\"value\", obj(vec![(\"kind\", Json::String(\"point\".to_string())), (\"value\", point_json(&base.header.insertion_base))]))]))]);\n        match kind {\n            \"set-snapshot\" => Ok(Undo::Original),\n\n            \"set-header-var\" | \"remove-header-var\" => match name.as_str() {\n                \"$INSBASE\" => apply(\"set-header-var\", insbase(name)),\n                other => Err(format!(\"dxf oracle: unsupported header var {other:?}\")),\n            },\n\n            \"insert-layer\" => apply(\"remove-layer\", obj(vec![(\"name\", Json::String(member(params, \"layer\")?.str(\"name\")))])),\n            \"remove-layer\" => match base.layers().position(|l| l.name == name) {\n                Some(at) => apply(\"insert-layer\", obj(vec![(\"index\", Json::Number(at as f64)), (\"layer\", layer_wire(base.layers().nth(at).expect(\"position valid\")))])),\n                None => Ok(Undo::Nothing),\n            },\n            \"set-layer\" => match base.layers().find(|l| l.name == name) {\n                Some(layer) => apply(\"set-layer\", obj(vec![(\"name\", Json::String(name.clone())), (\"layer\", layer_wire(layer))])),\n                None => apply(\"remove-layer\", obj(vec![(\"name\", Json::String(name))])),\n            },\n\n            \"insert-style\" => apply(\"remove-style\", obj(vec![(\"name\", Json::String(member(params, \"style\")?.str(\"name\")))])),\n            \"remove-style\" => match base.styles().position(|s| s.name == name) {\n                Some(at) => apply(\"insert-style\", obj(vec![(\"index\", Json::Number(at as f64)), (\"style\", style_wire(base.styles().nth(at).expect(\"position valid\")))])),\n                None => Ok(Undo::Nothing),\n            },\n            \"set-style\" => match base.styles().find(|s| s.name == name) {\n                Some(style) => apply(\"set-style\", obj(vec![(\"name\", Json::String(name.clone())), (\"style\", style_wire(style))])),\n                None => apply(\"remove-style\", obj(vec![(\"name\", Json::String(name))])),\n            },\n\n            \"insert-linetype\" => apply(\"remove-linetype\", obj(vec![(\"name\", Json::String(member(params, \"linetype\")?.str(\"name\")))])),\n            \"remove-linetype\" => match base.line_types().position(|l| l.name == name) {\n                Some(at) => apply(\"insert-linetype\", obj(vec![(\"index\", Json::Number(at as f64)), (\"linetype\", linetype_wire(base.line_types().nth(at).expect(\"position valid\")))])),\n                None => Ok(Undo::Nothing),\n            },\n            \"set-linetype\" => match base.line_types().find(|l| l.name == name) {\n                Some(linetype) => apply(\"set-linetype\", obj(vec![(\"name\", Json::String(name.clone())), (\"linetype\", linetype_wire(linetype))])),\n                None => apply(\"remove-linetype\", obj(vec![(\"name\", Json::String(name))])),\n            },\n\n            \"insert-entity\" => apply(\"remove-entity\", obj(vec![(\"index\", Json::Number(index as f64))])),\n            \"remove-entity\" | \"set-entity\" => match base.entities().nth(index) {\n                Some(entity) => apply(if kind == \"remove-entity\" { \"insert-entity\" } else { \"set-entity\" }, obj(vec![(\"index\", Json::Number(index as f64)), (\"entity\", entity_wire(entity)?)])),\n                None => Ok(Undo::Nothing),\n            },\n\n            \"insert-block\" => apply(\"remove-block\", obj(vec![(\"index\", Json::Number(index as f64))])),\n            \"remove-block\" | \"set-block\" => match base.blocks().nth(index) {\n                Some(block) => apply(if kind == \"remove-block\" { \"insert-block\" } else { \"set-block\" }, obj(vec![(\"index\", Json::Number(index as f64)), (\"block\", block_wire(block)?)])),\n                None => Ok(Undo::Nothing),\n            },\n\n            other => Err(format!(\"mutation kind {other:?} has no oracle inverse implementation\")),\n        }\n    }\n    //#endregion 🔖️Inverse\n\n    //#region 🔖️Entry\n    pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {\n        let mut drawing = load(input)?;\n        let kind = spec.str(\"kind\");\n        let params = spec.get(\"params\").cloned().unwrap_or(Json::Null);\n        apply_kind(&mut drawing, &kind, &params)?;\n        save(&drawing)\n    }\n\n    pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {\n        save(&load(input)?)\n    }\n\n    pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {\n        let base = load(input)?;\n        let kind = spec.str(\"kind\");\n        let params = spec.get(\"params\").cloned().unwrap_or(Json::Null);\n        let undo = inverse_of(&base, &kind, &params)?;\n        let mut drawing = load(input)?;\n        apply_kind(&mut drawing, &kind, &params)?;\n        match undo {\n            Undo::Nothing => {}\n            Undo::Apply(inverse_kind, inverse_params) => apply_kind(&mut drawing, &inverse_kind, &inverse_params)?,\n            Undo::Original => drawing = base,\n        }\n        save(&drawing)\n    }\n\n    /// 📄️ Semantic projection of a DXF R12 document. Handles, owner pointers and any R13+ subclass\n    /// marker a writer still emits are excluded: not normative.\n    pub fn project_dxf_r12(bytes: &[u8]) -> Result<Json, String> {\n        let drawing = load(bytes)?;\n        let layers: Vec<Json> = drawing.layers().map(layer_to_json).collect();\n        let styles: Vec<Json> = drawing.styles().map(style_to_json).collect();\n        let linetypes: Vec<Json> = drawing.line_types().map(linetype_to_json).collect();\n        let blocks: Vec<Json> = drawing.blocks().map(block_projection).collect();\n        let entities: Vec<Json> = drawing.entities().map(entity_projection).collect();\n        Ok(obj(vec![\n            (\"acadVersion\", Json::String(format!(\"{:?}\", drawing.header.version))),\n            (\"insertionBase\", point_json(&drawing.header.insertion_base)),\n            (\"layers\", Json::Array(layers)),\n            (\"styles\", Json::Array(styles)),\n            (\"linetypes\", Json::Array(linetypes)),\n            (\"blocks\", Json::Array(blocks)),\n            (\"entities\", Json::Array(entities)),\n        ]))\n    }\n    //#endregion 🔖️Entry\n}\n\n//#region 🔖️SmokeTests\n/// 🧪️ Scratch smoke coverage exercising every planned `mutate-<kind>`/`inverse-<kind>` JSON row\n/// against the real committed fixture before the feature file locks them in — ticket\n/// 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.\n#[cfg(all(test, feature = \"oracles\"))]\n#[path = \"🧪️tests/🔬️smoke/🦀️.rs\"]\nmod smoke_tests;\n//#endregion 🔖️SmokeTests\n",
      "sha256": "6fbb892a45cc8dbab6c98b189184a4cd88ba1d99a4765bb7f1652662fd5d1b25",
      "bytes": 35097
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🦀️.rs",
      "source": "//! 🔮️ Mutation oracle for `s.note.note@1/✳️any` — the reader half of the THREE registered\n//! third-party carriers (`dxf-crate-note-ink-reader`, `quick-xml-note-drawing-reader`,\n//! `lopdf-note-text-reader`, all declared in `./🔣️.json`). This subset's own domain\n//! vocabulary (33 kinds) is note-native, not DXF/SVG/PDF-native — unlike `s.stdio.dxf@r12/✳️any`,\n//! whose vocabulary IS the format — so this module's job is to READ what `NoteIntoDxf`/`NoteIntoSvg`/\n//! `NoteIntoPdf` (`../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/🦀️.rs`) actually wrote,\n//! independently of this subset's own codec, exactly the role `three-carrier-reader` plays for\n//! `s.stdio.semio@v1/✳️mesh`.\n//!\n//! Every function below DELEGATES to an already-registered, already-oracle-qualified projector this\n//! crate carries for OTHER subsets, rather than re-implementing a DXF/XML/PDF reader a third time:\n//! `crate::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12` is the same qualifying\n//! `dxf` 0.6 reader `s.stdio.dxf@r12/✳️any` registers under `dxf-crate-r12-mutate`; the DXF bytes\n//! `NoteIntoDxf` writes are ordinary DXF R12 (only ever containing `LINE` entities), so the reader\n//! that already qualifies against the full grammar reads this narrower subset of it for free.\n//! `semio_s_plugin_stdio_markup_test_oracle::live::{parse_markup, project_markup}` is the `quick-xml` 0.42 tree reader/semantic\n//! projector this crate's `📰markup` family module already carries for the `🎨️svg` subsets — SVG is\n//! XML, so nothing note-specific is needed to read it. `semio_s_plugin_stdio_document_test_oracle::project_pdf` is the `lopdf`\n//! 0.44 reader already registered under `pdf-edit`/`pdf-parse`.\n//!\n//! WHAT THE PROJECTIONS DO AND DO NOT WITNESS, per `./🔣️.json`'s `mutationManifests`\n//! `carriers`/`oracleRequirements`: `project_note_dxf` sees only `LINE` entities built from an Ink\n//! block's raw `points` (no block `x`/`y`/`rotation`, no visibility filter, no width — read straight\n//! from `NoteIntoDxf::serialize`'s body). `project_note_svg` sees every visible block's `<g\n//! transform=\"matrix(…)\">` (position+rotation) and, per kind, a `<path>` (Ink: geometry+stroke-width;\n//! Table/Math/Group/image-fallback: an outline rectangle keyed to width/height only), a `<text>`\n//! (Text: joined paragraph content; `font_size` is wired to `y`, never to a size attribute), or an\n//! `<image>` (real bytes when the referenced asset exists). `project_note_pdf` sees the title and\n//! every Text block's content flattened onto one page's text stream — no position, no other kind.\n//!\n//! @see `./🔣️.json` — the three oracle registrations and the per-mutation carrier list.\n//! @see ../🚪️io/📤️export/🧵️serializers/🗿️artifacts/**/✳️any/🦀️.rs — what is projected.\n\nuse semio_repo_test_host::Json;\n\n//#region 🔖️Dispatch\n/// 🖊️ Independent semantic projection of the `LINE` entities `NoteIntoDxf` wrote, via the same\n/// qualifying `dxf` reader `s.stdio.dxf@r12/✳️any` registers.\n#[cfg(feature = \"oracles\")]\npub fn project_note_dxf(bytes: &[u8]) -> Result<Json, String> {\n    crate::artifacts::dxf::standards::v_r12::subsets::header::project_dxf_r12(bytes)\n}\n\n/// 🎨️ Independent semantic projection of the SVG XML `NoteIntoSvg` wrote (via the real semio/drawing\n/// bridge), using this crate's shared `quick-xml` tree reader/projector — SVG is XML, so nothing\n/// note-specific is needed to read it.\n#[cfg(feature = \"oracles\")]\npub fn project_note_svg(bytes: &[u8]) -> Result<Json, String> {\n    let doc = semio_s_plugin_stdio_markup_test_oracle::live::parse_markup(bytes)?;\n    Ok(semio_s_plugin_stdio_markup_test_oracle::live::project_markup(&doc))\n}\n\n/// 📄️ Independent semantic projection (media box + `Tj` text operands per page) of the PDF\n/// `NoteIntoPdf` wrote, via this crate's shared `lopdf` reader.\n#[cfg(feature = \"oracles\")]\npub fn project_note_pdf(bytes: &[u8]) -> Result<Json, String> {\n    semio_s_plugin_stdio_document_test_oracle::project_pdf(bytes)\n}\n\n/// 🚫️ Without the `oracles` feature no reference implementation is linked at all.\n#[cfg(not(feature = \"oracles\"))]\npub fn project_note_dxf(_bytes: &[u8]) -> Result<Json, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n#[cfg(not(feature = \"oracles\"))]\npub fn project_note_svg(_bytes: &[u8]) -> Result<Json, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n#[cfg(not(feature = \"oracles\"))]\npub fn project_note_pdf(_bytes: &[u8]) -> Result<Json, String> {\n    Err(\"the `oracles` feature is disabled — this host was not built with the registered reference implementations\".to_string())\n}\n//#endregion 🔖️Dispatch\n\n//#region 🧪️SmokeTests\n/// 🧪️ Runtime proof (not merely compilation) that each delegated projector actually reads bytes\n/// shaped the way `NoteIntoDxf`/`NoteIntoSvg`/`NoteIntoPdf` produce them. This crate carries no\n/// dependency on `semio_s_plugin_stdio` (the production codec that plugin belongs to — an oracle\n/// crate must never link the subject it is evidence for), so each fixture below is built from the\n/// FORMAT'S OWN minimal grammar rather than by calling note's real serializer.\n#[cfg(all(test, feature = \"oracles\"))]\n#[path = \"🧪️tests/🔬️smoke/🦀️.rs\"]\nmod smoke_tests;\n//#endregion 🧪️SmokeTests\n",
      "sha256": "ed8cb8c6ff62a64fdb5cce0be37ccf2adc42fbb6028652f0064f9ca1d4b9c13d",
      "bytes": 5573
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust/Cargo.toml",
      "source": "[workspace]\n\n[package]\nname = \"semio-s-artifact-stdio-dxf-test-oracle\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\ndescription = \"🔮️ Owned independent test oracle provider\"\n\n[package.metadata.semio]\nrole = \"test\"\n\n[lib]\nname = \"semio_s_artifact_stdio_dxf_test_oracle\"\npath = \"../../🦀️.rs\"\n\n[features]\noracles = [\"dep:dxf\"]\n\n[dependencies]\n\"dxf\" = { version = \"0.6\", optional = true }\n\"semio-repo-test-host\" = { path = \"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust\", default-features = false }\n",
      "sha256": "35521e83239b86e43ea44fb51f9f23b3c36e59d6423294079c36ab6db4d31fbb",
      "bytes": 589
    },
    {
      "path": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.toml",
      "source": "[workspace]\n\n[package]\nname = \"semio-s-artifact-note-note-test-oracle\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\ndescription = \"🔮️ Owned independent test oracle provider\"\n\n[package.metadata.semio]\nrole = \"test\"\n\n[lib]\nname = \"semio_s_artifact_note_note_test_oracle\"\npath = \"../../🦀️.rs\"\n\n[features]\noracles = [\"dep:dxf\", \"semio-s-plugin-stdio-document-test-oracle/oracles\", \"semio-s-plugin-stdio-markup-test-oracle/oracles\"]\n\n[dependencies]\n\"dxf\" = { version = \"0.6\", optional = true }\n\"semio-repo-test-host\" = { path = \"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust\", default-features = false }\n\"semio-s-plugin-stdio-document-test-oracle\" = { path = \"../../../../../../🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust\", default-features = false }\n\"semio-s-plugin-stdio-markup-test-oracle\" = { path = \"../../../../../../🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust\", default-features = false }\n\n[dev-dependencies]\n\"lopdf\" = { version = \"0.44\" }\n",
      "sha256": "5cc17a2813ab22f57185ed446a0312ec8e987f67ae68e4b3830452e1b36fc9d8",
      "bytes": 1087
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/🧱️blocks/🧪️tests/🧱️mutate-dxf-r12-blocks/🦀️.rs",
      "source": "//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR\n//! wave 7.\n//!\n//! Every scenario copies the derived, committed R12 `🚏️bus-shelter` drawing into the case work\n//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`\n//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs`'s\n//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's\n//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind\n//! `DxfMutation` vocabulary. Each side hands the drawing it produced to the `semantic-dxf-r12-v1`\n//! profile's `dxf-r12-reader-compare-v1` pipeline — the oracle's as `expected-dxf`, the subject's as\n//! `actual-dxf` — whose `dxf` 0.6 probes read both files independently. The subject half is gated\n//! behind the generated host's `sut` feature so the oracle-only run never compiles the local\n//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject\n//! phase runs, and wave 14 ran the full differential comparison against the oracle.\n\nuse semio_repo_test_host::{Adapter, Context, Json, Outcome};\nuse semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};\n\n\n//#region 🔖️Input\nconst INPUT: &str = \"asset://🚏️bus-shelter/🖊️.dxf\";\n\n/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.\nfn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {\n    let copy = ctx.copy_fixture(INPUT, Some(\"bus-shelter-r12.dxf\"))?;\n    std::fs::read(&copy).map_err(|error| error.to_string())\n}\n\n/// 📦️ The drawing one side produced, written as the `role` artifact (`expected-dxf` for the oracle, `actual-dxf` for the\n/// subject) the `dxf-r12-reader-compare-v1` pipeline hands to its probes.\nfn produced(ctx: &Context, role: &str, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {\n    let path = ctx.artifact(role, &format!(\"{role}.dxf\"))?;\n    std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;\n    Ok(Outcome::with_raw(bytes, projection).artifact(role, &path, \"image/vnd.dxf\"))\n}\n//#endregion 🔖️Input\n\n//#region 🔖️Laws\n/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law\n/// violation must name the field that broke it rather than dump two whole documents at the reader.\nfn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {\n    match (expected, actual) {\n        (Json::Object(left), Json::Object(right)) => {\n            for (key, value) in left {\n                match right.iter().find(|(name, _)| name == key) {\n                    Some((_, other)) => {\n                        if let Some(found) = first_divergence(&format!(\"{path}.{key}\"), value, other) {\n                            return Some(found);\n                        }\n                    }\n                    None => return Some(format!(\"{path}.{key} is absent from the result\")),\n                }\n            }\n            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!(\"{path}.{key} appeared in the result out of nowhere\"))\n        }\n        (Json::Array(left), Json::Array(right)) => {\n            if left.len() != right.len() {\n                return Some(format!(\"{path} holds {} member(s), expected {}\", right.len(), left.len()));\n            }\n            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!(\"{path}[{index}]\"), value, other))\n        }\n        _ if expected == actual => None,\n        _ => Some(format!(\"{path}: expected {} but read {}\", expected.to_string(), actual.to_string())),\n    }\n}\n\n/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise\n/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and\n/// its scenario passes whenever the reference library merely declined to error.\nfn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {\n    match first_divergence(\"projection\", expected, actual) {\n        Some(divergence) => Err(format!(\"{law}: {divergence}\")),\n        None => Ok(()),\n    }\n}\n//#endregion 🔖️Laws\n\n//#region 🔖️Oracle\n/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`\n/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in\n/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row\n/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference\n/// library declined to error, which is exactly the failure this platform exists to prevent. The\n/// baseline runs one `dxf` round trip so the comparison isolates the mutation rather than the\n/// writer's own normal form.\nfn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let kind = spec.str(\"kind\");\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    if projection == baseline {\n        return Err(format!(\"{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name\"));\n    }\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse\n/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and\n/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the\n/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken\n/// through one `dxf` round trip so the two sides carry the same serializer normalisation and the\n/// comparison isolates the mutation pair itself.\nfn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(&format!(\"inverse law violated for {:?} -- undoing it did not restore the drawing\", spec.str(\"kind\")), &baseline, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real\n/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST\n/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is\n/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --\n/// so the byte tripwire is real evidence that the document was parsed rather than copied.\nfn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let before = project_dxf_r12(&input)?;\n    let bytes = oracle_round_trip(&input)?;\n    if bytes == input {\n        return Err(\"byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed\".to_string());\n    }\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(\"identity round trip is not semantics-preserving\", &before, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n//#endregion 🔖️Oracle\n\n//#region 🔖️Subject\n#[cfg(feature = \"sut\")]\nmod subject {\n    use super::{mutable_input, produced};\n    use semio_repo_test_host::{Context, Json, Outcome};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, DxfMutation};\n    use semio_repo_test_host::law::wire_operation;\n    use semio_s_artifact_stdio_dxf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document};\n    use semio_s_artifact_stdio_dxf::DxfSnapshot;\n    use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;\n\n    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.\n    fn mutation_of(spec: &Json) -> Result<DxfMutation, String> {\n        wire_operation(&spec.str(\"kind\"), &spec.get(\"params\").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)\n    }\n\n    fn decode(ctx: &Context) -> Result<DxfSnapshot, String> {\n        let input = mutable_input(ctx)?;\n        parse_dxf_document(std::str::from_utf8(&input).map_err(|error| error.to_string())?)\n    }\n\n    fn outcome(ctx: &Context, snapshot: &DxfSnapshot) -> Result<Outcome, String> {\n        let output = print_dxf_document(snapshot).into_bytes();\n        let projection = project_dxf_r12(&output)?;\n        produced(ctx, \"actual-dxf\", output, projection)\n    }\n\n    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` leaves the snapshot untouched and\n    /// reports the refusal as the outcome's only messages — every `DxfMutation::diff` arm builds its outcome without one —\n    /// so a non-empty message list IS a refusal.\n    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {\n        match apply_dxf_mutation(snapshot, mutation).messages().first() {\n            Some(refusal) => Err(format!(\"{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}\")),\n            None => Ok(()),\n        }\n    }\n\n    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        applied(&mut snapshot, &mutation_of(&spec)?, &spec.str(\"kind\"))?;\n        outcome(ctx, &snapshot)\n    }\n\n    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.\n    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        let kind = spec.str(\"kind\");\n        let forward = mutation_of(&spec)?;\n        let backward = mutation_inverse(&forward, &snapshot);\n        applied(&mut snapshot, &forward, &kind)?;\n        for mutation in &backward {\n            applied(&mut snapshot, mutation, &format!(\"the inverse of {kind}\"))?;\n        }\n        outcome(ctx, &snapshot)\n    }\n\n    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its\n    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/\n    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`\n    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module\n    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.\n    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {\n        let snapshot = decode(ctx)?;\n        if print_dxf_document(&snapshot).into_bytes() == mutable_input(ctx)? {\n            return Err(\"byte pass-through: output is bit-identical to the input\".to_string());\n        }\n        outcome(ctx, &snapshot)\n    }\n}\n//#endregion 🔖️Subject\n\n//#region 🔖️Registration\n/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline\n/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.\npub fn adapter() -> Adapter {\n    let mut built = Adapter::new(\"rust\");\n    built = built.oracle(\"mutate\", mutate_oracle).oracle(\"inverse\", inverse_oracle);\n    built = built.oracle(\"identity-round-trip\", identity_round_trip_oracle);\n    #[cfg(feature = \"sut\")]\n    {\n        built = built.subject(\"mutate\", subject::mutate).subject(\"inverse\", subject::inverse);\n        built = built.subject(\"identity-round-trip\", subject::identity_round_trip);\n    }\n    built\n}\n//#endregion 🔖️Registration\n",
      "sha256": "1587d658472db1e0599e666ea6ef22a4522fcdc84e9fbc2cbb607454e7d4992f",
      "bytes": 12603
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧪️tests/📰️mutate-dxf-r12/🦀️.rs",
      "source": "//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR\n//! wave 7.\n//!\n//! Every scenario copies the derived, committed R12 `🚏️bus-shelter` drawing into the case work\n//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`\n//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs`'s\n//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's\n//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind\n//! `DxfMutation` vocabulary. Each side hands the drawing it produced to the `semantic-dxf-r12-v1`\n//! profile's `dxf-r12-reader-compare-v1` pipeline — the oracle's as `expected-dxf`, the subject's as\n//! `actual-dxf` — whose `dxf` 0.6 probes read both files independently. The subject half is gated\n//! behind the generated host's `sut` feature so the oracle-only run never compiles the local\n//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject\n//! phase runs, and wave 14 ran the full differential comparison against the oracle.\n\nuse semio_repo_test_host::{Adapter, Context, Json, Outcome};\nuse semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};\n\n\n//#region 🔖️Input\nconst INPUT: &str = \"asset://🚏️bus-shelter/🖊️.dxf\";\n\n/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.\nfn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {\n    let copy = ctx.copy_fixture(INPUT, Some(\"bus-shelter-r12.dxf\"))?;\n    std::fs::read(&copy).map_err(|error| error.to_string())\n}\n\n/// 📦️ The drawing one side produced, written as the `role` artifact (`expected-dxf` for the oracle, `actual-dxf` for the\n/// subject) the `dxf-r12-reader-compare-v1` pipeline hands to its probes.\nfn produced(ctx: &Context, role: &str, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {\n    let path = ctx.artifact(role, &format!(\"{role}.dxf\"))?;\n    std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;\n    Ok(Outcome::with_raw(bytes, projection).artifact(role, &path, \"image/vnd.dxf\"))\n}\n//#endregion 🔖️Input\n\n//#region 🔖️Laws\n/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law\n/// violation must name the field that broke it rather than dump two whole documents at the reader.\nfn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {\n    match (expected, actual) {\n        (Json::Object(left), Json::Object(right)) => {\n            for (key, value) in left {\n                match right.iter().find(|(name, _)| name == key) {\n                    Some((_, other)) => {\n                        if let Some(found) = first_divergence(&format!(\"{path}.{key}\"), value, other) {\n                            return Some(found);\n                        }\n                    }\n                    None => return Some(format!(\"{path}.{key} is absent from the result\")),\n                }\n            }\n            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!(\"{path}.{key} appeared in the result out of nowhere\"))\n        }\n        (Json::Array(left), Json::Array(right)) => {\n            if left.len() != right.len() {\n                return Some(format!(\"{path} holds {} member(s), expected {}\", right.len(), left.len()));\n            }\n            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!(\"{path}[{index}]\"), value, other))\n        }\n        _ if expected == actual => None,\n        _ => Some(format!(\"{path}: expected {} but read {}\", expected.to_string(), actual.to_string())),\n    }\n}\n\n/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise\n/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and\n/// its scenario passes whenever the reference library merely declined to error.\nfn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {\n    match first_divergence(\"projection\", expected, actual) {\n        Some(divergence) => Err(format!(\"{law}: {divergence}\")),\n        None => Ok(()),\n    }\n}\n//#endregion 🔖️Laws\n\n//#region 🔖️Oracle\n/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`\n/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in\n/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row\n/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference\n/// library declined to error, which is exactly the failure this platform exists to prevent. The\n/// baseline runs one `dxf` round trip so the comparison isolates the mutation rather than the\n/// writer's own normal form.\nfn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let kind = spec.str(\"kind\");\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    if projection == baseline {\n        return Err(format!(\"{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name\"));\n    }\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse\n/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and\n/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the\n/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken\n/// through one `dxf` round trip so the two sides carry the same serializer normalisation and the\n/// comparison isolates the mutation pair itself.\nfn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(&format!(\"inverse law violated for {:?} -- undoing it did not restore the drawing\", spec.str(\"kind\")), &baseline, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real\n/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST\n/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is\n/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --\n/// so the byte tripwire is real evidence that the document was parsed rather than copied.\nfn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let before = project_dxf_r12(&input)?;\n    let bytes = oracle_round_trip(&input)?;\n    if bytes == input {\n        return Err(\"byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed\".to_string());\n    }\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(\"identity round trip is not semantics-preserving\", &before, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n//#endregion 🔖️Oracle\n\n//#region 🔖️Subject\n#[cfg(feature = \"sut\")]\nmod subject {\n    use super::{mutable_input, produced};\n    use semio_repo_test_host::{Context, Json, Outcome};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, DxfMutation};\n    use semio_repo_test_host::law::wire_operation;\n    use semio_s_artifact_stdio_dxf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document};\n    use semio_s_artifact_stdio_dxf::DxfSnapshot;\n    use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;\n\n    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.\n    fn mutation_of(spec: &Json) -> Result<DxfMutation, String> {\n        wire_operation(&spec.str(\"kind\"), &spec.get(\"params\").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)\n    }\n\n    fn decode(ctx: &Context) -> Result<DxfSnapshot, String> {\n        let input = mutable_input(ctx)?;\n        parse_dxf_document(std::str::from_utf8(&input).map_err(|error| error.to_string())?)\n    }\n\n    fn outcome(ctx: &Context, snapshot: &DxfSnapshot) -> Result<Outcome, String> {\n        let output = print_dxf_document(snapshot).into_bytes();\n        let projection = project_dxf_r12(&output)?;\n        produced(ctx, \"actual-dxf\", output, projection)\n    }\n\n    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` leaves the snapshot untouched and\n    /// reports the refusal as the outcome's only messages — every `DxfMutation::diff` arm builds its outcome without one —\n    /// so a non-empty message list IS a refusal.\n    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {\n        match apply_dxf_mutation(snapshot, mutation).messages().first() {\n            Some(refusal) => Err(format!(\"{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}\")),\n            None => Ok(()),\n        }\n    }\n\n    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        applied(&mut snapshot, &mutation_of(&spec)?, &spec.str(\"kind\"))?;\n        outcome(ctx, &snapshot)\n    }\n\n    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.\n    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        let kind = spec.str(\"kind\");\n        let forward = mutation_of(&spec)?;\n        let backward = mutation_inverse(&forward, &snapshot);\n        applied(&mut snapshot, &forward, &kind)?;\n        for mutation in &backward {\n            applied(&mut snapshot, mutation, &format!(\"the inverse of {kind}\"))?;\n        }\n        outcome(ctx, &snapshot)\n    }\n\n    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its\n    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/\n    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`\n    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module\n    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.\n    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {\n        let snapshot = decode(ctx)?;\n        if print_dxf_document(&snapshot).into_bytes() == mutable_input(ctx)? {\n            return Err(\"byte pass-through: output is bit-identical to the input\".to_string());\n        }\n        outcome(ctx, &snapshot)\n    }\n}\n//#endregion 🔖️Subject\n\n//#region 🔖️Registration\n/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline\n/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.\npub fn adapter() -> Adapter {\n    let mut built = Adapter::new(\"rust\");\n    built = built.oracle(\"mutate\", mutate_oracle).oracle(\"inverse\", inverse_oracle);\n    built = built.oracle(\"identity-round-trip\", identity_round_trip_oracle);\n    #[cfg(feature = \"sut\")]\n    {\n        built = built.subject(\"mutate\", subject::mutate).subject(\"inverse\", subject::inverse);\n        built = built.subject(\"identity-round-trip\", subject::identity_round_trip);\n    }\n    built\n}\n//#endregion 🔖️Registration\n",
      "sha256": "1587d658472db1e0599e666ea6ef22a4522fcdc84e9fbc2cbb607454e7d4992f",
      "bytes": 12603
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧫️fixtures/🦀️.rs",
      "source": "//! 🏭️ Third-party fixture codec for `s.stdio.dxf@r12/📰️header` — built entirely through the real `dxf`\n//! 0.6 crate's own typed `Drawing`/`entities`/`tables` model, never through this repository's own\n//! DXF codec (the same discipline `../../🔬️probes/📜️script.ts` documents in its own header). No\n//! wall-clock and no randomness: byte-for-byte reproducible on every run, which is what\n//! `test fixture reproduce` checks (see `FIXED_STAMP_HEADER` below).\n//!\n//! One shared base document (`base_doc`, IDENTICAL in content and construction order to what this\n//! file produced before this retrofit — the committed `drafting-plate` fixture's bytes are\n//! unaffected) backs TWO families of recipe:\n//!\n//! * `drafting-plate` — the pre-existing single-document fixture for the `cross-semio-implementation`\n//!   oracle (`dxf-crate-r12-mutate`). Untouched in content; still written as `<out>/drafting-plate/\n//!   drafting-plate.dxf`.\n//! * one dedicated `<kind>-applied` / `<kind>-no-op` / `<kind>-rejected-<reason>` recipe per\n//!   WITNESSABLE mutation kind (`../../../🔮️oracles/🔣️.json`'s `mutationCatalogs[].kinds`, 19 total) —\n//!   the new corpus this retrofit adds, each written as `<out>/<recipe-id>/⬅️before.dxf[\n//!   +after.dxf]`. A `-rejected-*` recipe writes ONLY `before.dxf`: the mutation described in its own\n//!   comment is refused by the real production dispatch (`../../🧬️schema/🔺️diff/🦀️.rs`'s\n//!   `validate_indexed_targets`/`validate_named_targets`, read directly, never assumed) before any\n//!   DXF encoding would even happen, so there is no legal `after` state to write.\n//!\n//! `set-header-var` carries `-applied` only — see `SET_HEADER_VAR_REJECTED_NOTE` below for why no\n//! `-rejected` recipe exists for it, verified from the same validation code, not asserted.\n//!\n//! Three subcommands:\n//!   build <recipe-id> <out-dir>   — writes `<out-dir>/<recipe-id>/…`\n//!   project <path-to-dxf>         — decodes a real DXF file and prints a typed JSON projection on\n//!                                   stdout, the exact shape `semantic-dxf-r12-v1` compares (mirrors\n//!                                   `../../../🔮️oracles/🦀️.rs`'s own `project_dxf_r12` shape,\n//!                                   independently re-derived here since that module is gated behind\n//!                                   the `oracles` feature of a host crate this standalone binary\n//!                                   never links)\n//!   list-recipes                  — prints every known recipe id, one per line\n//!\n//! @see ../📜️script.ts — the only caller; drives both `drafting-plate` and the new corpus\n//! @see ../../🔬️probes/📜️script.ts — the only caller of `project`\n//! @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md\n\nuse dxf::entities::{Arc as DxfArc, Circle, Entity, EntityType, Insert, Line, Solid, Text};\nuse dxf::enums::AcadVersion;\nuse dxf::tables::{Layer, LineType, Style};\nuse dxf::{Block, Color, Drawing, Point};\nuse std::env;\nuse std::fs;\nuse std::path::Path;\n\n/// 🕰️ Identical to this file's pre-retrofit content: `dxf::Header::default` stamps\n/// `$TDCREATE`/`$TDUPDATE` with `chrono::Local::now()`, and `Drawing::save` writes those two fields\n/// verbatim — the ONLY non-content-derived values in the whole output. A fixed stamp is required for\n/// byte reproducibility; obtained by having `dxf`'s own reader parse it out of this literal R12\n/// header and `dxf`'s own writer write it back (see the pre-existing doc comment this is copied\n/// from for the full measurement).\nconst FIXED_STAMP_HEADER: &str = \"  0\\nSECTION\\n  2\\nHEADER\\n  9\\n$ACADVER\\n  1\\nAC1009\\n  9\\n$TDCREATE\\n 40\\n2461281.0\\n  9\\n$TDUPDATE\\n 40\\n2461281.0\\n  0\\nENDSEC\\n  0\\nEOF\\n\";\n\n// 📖 `set-header-var`'s real dispatch (`🧬️schema/🧬️mutations/🦀️.rs` `DxfMutation::diff` ->\n// `diff_set_header_var`, `🧬️schema/🔺️diff/🦀️.rs:1774`) branches only on whether the\n// target name already exists in `header_vars`: if it does, it emits a single `modified` entry\n// against that unique name (always valid — `validate_named_targets` only rejects a modify when the\n// name is ABSENT or duplicated, and a well-formed document never has either for its own $INSBASE);\n// if it does not, it emits a single `added` entry at `index = header_vars.len()` (`index > length`\n// is never true there, so `validate_named_targets`'s add-path never rejects it either). Every\n// reachable branch therefore succeeds — reaching `invalid-modify-target` needs a BASE document\n// whose `header_vars` already contains the target name MORE THAN ONCE, which no real DXF writer\n// (this crate included — `dxf::Header` is a fixed struct with one `insertion_base` field, not a\n// repeatable list) can produce. `remove-header-var` has no such problem: its target can simply be a\n// name that is genuinely absent, which any real document can exhibit trivially (see\n// `remove-header-var-rejected-missing` below) — the asymmetry is real, not an oversight. So there is\n// NO `set-header-var-rejected-*` recipe in `RECIPE_IDS` below, deliberately.\n\n//#region 🔖️BaseDocument\n/// 🏷️ `LTYPE` rows, declaration order — unchanged from the pre-retrofit content.\nfn line_types() -> Vec<LineType> {\n    vec![(\"BYLAYER\", \"\"), (\"BYBLOCK\", \"\"), (\"CONTINUOUS\", \"Solid line\"), (\"DASHED\", \"Dashed __ __ __ __\"), (\"HIDDEN\", \"Hidden - - - - - -\")]\n        .into_iter()\n        .map(|(name, description)| LineType { name: name.to_string(), description: description.to_string(), ..Default::default() })\n        .collect()\n}\n\nfn layers() -> Vec<Layer> {\n    vec![(\"0\", 7u8, \"CONTINUOUS\"), (\"DIMS\", 3, \"DASHED\"), (\"TEXT\", 5, \"CONTINUOUS\")]\n        .into_iter()\n        .map(|(name, color, line_type_name)| Layer { name: name.to_string(), color: Color::from_index(color), line_type_name: line_type_name.to_string(), ..Default::default() })\n        .collect()\n}\n\nfn styles() -> Vec<Style> {\n    vec![(\"STANDARD\", \"txt\"), (\"NOTES\", \"romans.shx\"), (\"TITLES\", \"italicc.shx\")]\n        .into_iter()\n        .map(|(name, font)| Style { name: name.to_string(), primary_font_file_name: font.to_string(), text_height: 2.5, ..Default::default() })\n        .collect()\n}\n\nfn on_layer(layer: &str, specific: EntityType) -> Entity {\n    let mut entity = Entity::new(specific);\n    entity.common.layer = layer.to_string();\n    entity\n}\n\nfn blocks() -> Vec<Block> {\n    vec![\n        Block {\n            name: \"SHELTER_POST\".to_string(),\n            layer: \"0\".to_string(),\n            base_point: Point::new(0.0, 0.0, 0.0),\n            entities: vec![\n                on_layer(\"0\", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(0.0, 240.0, 0.0), ..Default::default() })),\n                on_layer(\"0\", EntityType::Circle(Circle { center: Point::new(0.0, 240.0, 0.0), radius: 12.0, ..Default::default() })),\n            ],\n            ..Default::default()\n        },\n        Block {\n            name: \"BENCH\".to_string(),\n            layer: \"0\".to_string(),\n            base_point: Point::new(15.0, -5.0, 0.0),\n            entities: vec![on_layer(\"0\", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(180.0, 0.0, 0.0), ..Default::default() }))],\n            ..Default::default()\n        },\n    ]\n}\n\n/// 🧱️ Top-level `ENTITIES`, order-significant: index 0..=6, spanning all six typed kinds the subset\n/// models — unchanged from the pre-retrofit content.\nfn entities() -> Vec<Entity> {\n    vec![\n        on_layer(\"0\", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(1200.0, 0.0, 0.0), ..Default::default() })),\n        on_layer(\"0\", EntityType::Line(Line { p1: Point::new(1200.0, 0.0, 0.0), p2: Point::new(1200.0, 800.0, 0.0), ..Default::default() })),\n        on_layer(\"0\", EntityType::Circle(Circle { center: Point::new(600.0, 400.0, 0.0), radius: 150.0, ..Default::default() })),\n        on_layer(\"DIMS\", EntityType::Arc(DxfArc { center: Point::new(600.0, 400.0, 0.0), radius: 220.0, start_angle: 30.0, end_angle: 150.0, ..Default::default() })),\n        on_layer(\"DIMS\", EntityType::Solid(Solid { first_corner: Point::new(0.0, 0.0, 0.0), second_corner: Point::new(60.0, 0.0, 0.0), third_corner: Point::new(60.0, 40.0, 0.0), fourth_corner: Point::new(0.0, 40.0, 0.0), ..Default::default() })),\n        on_layer(\"TEXT\", EntityType::Text(Text { location: Point::new(80.0, 720.0, 0.0), text_height: 35.0, value: \"DRAFTING PLATE\".to_string(), text_style_name: \"STANDARD\".to_string(), ..Default::default() })),\n        on_layer(\"0\", EntityType::Insert(Insert { name: \"SHELTER_POST\".to_string(), location: Point::new(300.0, 120.0, 0.0), ..Default::default() })),\n    ]\n}\n\n/// 🧬 Builds a fresh document — same content, same construction order as this file's pre-retrofit\n/// `build_dxf`, just returning the typed `Drawing` instead of already-encoded bytes so BOTH\n/// `drafting-plate` and every new recipe share exactly this construction.\nfn base_doc() -> Drawing {\n    let stamp = Drawing::load(&mut FIXED_STAMP_HEADER.as_bytes()).expect(\"dxf parses the fixed-stamp header\");\n    let mut drawing = Drawing::new();\n    drawing.header.version = AcadVersion::R12;\n    drawing.header.creation_date = stamp.header.creation_date;\n    drawing.header.update_date = stamp.header.update_date;\n    drawing.header.insertion_base = Point::new(12.5, -7.25, 0.0);\n\n    while drawing.remove_line_type(0).is_some() {}\n    for line_type in line_types() {\n        drawing.add_line_type(line_type);\n    }\n    while drawing.remove_layer(0).is_some() {}\n    for layer in layers() {\n        drawing.add_layer(layer);\n    }\n    while drawing.remove_style(0).is_some() {}\n    for style in styles() {\n        drawing.add_style(style);\n    }\n    while drawing.remove_block(0).is_some() {}\n    for block in blocks() {\n        drawing.add_block(block);\n    }\n    while drawing.remove_entity(0).is_some() {}\n    for entity in entities() {\n        drawing.add_entity(entity);\n    }\n    drawing\n}\n\nfn encode(drawing: &Drawing) -> Vec<u8> {\n    let mut out: Vec<u8> = Vec::new();\n    drawing.save(&mut out).expect(\"dxf save\");\n    out\n}\n//#endregion 🔖️BaseDocument\n\n//#region 🔖️OrderedRebuild\n/// 🧱️ `dxf::Drawing::add_*` only appends; a true insert-at-`index` needs the whole ordered\n/// collection rebuilt — same small pattern `../../../🔮️oracles/🦀️.rs`'s own `imp` module uses\n/// (independently re-derived here, never imported: that module is gated behind a host crate this\n/// binary never links).\nfn insert_layer_at(drawing: &mut Drawing, index: usize, layer: Layer) {\n    let mut items: Vec<Layer> = drawing.layers().cloned().collect();\n    items.insert(index.min(items.len()), layer);\n    while drawing.remove_layer(0).is_some() {}\n    for item in items {\n        drawing.add_layer(item);\n    }\n}\nfn insert_style_at(drawing: &mut Drawing, index: usize, style: Style) {\n    let mut items: Vec<Style> = drawing.styles().cloned().collect();\n    items.insert(index.min(items.len()), style);\n    while drawing.remove_style(0).is_some() {}\n    for item in items {\n        drawing.add_style(item);\n    }\n}\nfn insert_linetype_at(drawing: &mut Drawing, index: usize, linetype: LineType) {\n    let mut items: Vec<LineType> = drawing.line_types().cloned().collect();\n    items.insert(index.min(items.len()), linetype);\n    while drawing.remove_line_type(0).is_some() {}\n    for item in items {\n        drawing.add_line_type(item);\n    }\n}\nfn insert_block_at(drawing: &mut Drawing, index: usize, block: Block) {\n    let mut items: Vec<Block> = drawing.blocks().cloned().collect();\n    items.insert(index.min(items.len()), block);\n    while drawing.remove_block(0).is_some() {}\n    for item in items {\n        drawing.add_block(item);\n    }\n}\nfn insert_entity_at(drawing: &mut Drawing, index: usize, entity: Entity) {\n    let mut items: Vec<Entity> = drawing.entities().cloned().collect();\n    items.insert(index.min(items.len()), entity);\n    while drawing.remove_entity(0).is_some() {}\n    for item in items {\n        drawing.add_entity(item);\n    }\n}\n//#endregion 🔖️OrderedRebuild\n\n//#region 🔖️Recipes\n/// 🧪 Either the pre-existing single-document fixture, or a (before, optional-after) pair — `None`\n/// after means the recipe is `-rejected-*`: only `before.dxf` is ever written for it.\nenum RecipeOutput {\n    Single(Drawing),\n    Pair(Drawing, Option<Drawing>),\n}\n\n/// 🎯 One recipe per witnessable `(mutation, outcome)` coordinate declared in\n/// `../../../🔮️oracles/🔣️.json`'s `mutationManifests`, PLUS the pre-existing `drafting-plate`. Every\n/// `-applied`/`-no-op` `after` touches EXACTLY the field(s) the real dispatch\n/// (`../../🧬️schema/🧬️mutations/🦀️.rs` + `../../🧬️schema/🔺️diff/🦀️.rs`, both read directly,\n/// never assumed) would touch for that kind against `base_doc()`. Every `-rejected-*` recipe names,\n/// in its own match arm comment, the exact validation function and branch that refuses it — `before`\n/// only, no `after`.\nfn recipe(id: &str) -> Option<RecipeOutput> {\n    if id == \"drafting-plate\" {\n        return Some(RecipeOutput::Single(base_doc()));\n    }\n\n    let before = base_doc();\n    match id {\n        // 🧬 the `no-mutation` scenario id (no DxfMutation variant of its own) — before==after content.\n        \"no-mutation-no-op\" => Some(RecipeOutput::Pair(before, Some(base_doc()))),\n\n        // 🧬 SetSnapshot — diff_set_snapshot = DxfDiff::between(base, next) across every field;\n        // widens the circle entity's radius (matching this subset's own declared\n        // mutationCatalogs scenario id \"widens-the-circle-entity-radius\"), moves $INSBASE, and adds\n        // a fourth layer, so header_vars/tables/entities all move together, exactly like this\n        // subset's own `set-snapshot` test leaf.\n        \"set-snapshot-applied\" => {\n            let mut after = base_doc();\n            after.header.insertion_base = Point::new(100.0, 50.0, 0.0);\n            insert_layer_at(&mut after, 1, Layer { name: \"MARKERS\".to_string(), color: Color::from_index(6), line_type_name: \"CONTINUOUS\".to_string(), ..Default::default() });\n            for entity in after.entities_mut() {\n                if let EntityType::Circle(circle) = &mut entity.specific {\n                    circle.radius = 300.0;\n                }\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetSnapshot with next == base — diff is DxfDiff::default(), is_empty() true.\n        \"set-snapshot-no-op\" => Some(RecipeOutput::Pair(before, Some(base_doc()))),\n        // 🧬 SetSnapshot whose PAYLOAD snapshot declares two layers both named \"DIMS\" — one\n        // collides with the base's own existing \"DIMS\": DxfDiff::between's named_between computes\n        // a `modified` entry for the first \"DIMS\" match AND an `added` entry for the second, and\n        // `validate_named_targets`'s add-path rejects it (`present(key)` is true for a name that\n        // already exists in `base` — 🔺️diff/🦀️.rs:1571) — `invalid-add-target`. No `after`\n        // state is producible through the real dispatch, so only `before.dxf` is written; the\n        // payload that would be rejected is never itself encoded (rejected recipes never are).\n        \"set-snapshot-rejected-duplicate-layer\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 SetHeaderVar{name:\"$INSBASE\"} — the one generic $VAR `dxf`'s fixed Header struct\n        // persists unconditionally on an R12 save (../../../🔮️oracles/🦀️.rs's own\n        // 🔖️HeaderVar note, independently reconfirmed against the generated writer).\n        \"set-header-var-applied\" => {\n            let mut after = base_doc();\n            after.header.insertion_base = Point::new(50.0, 30.0, 0.0);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // (no set-header-var-rejected-* recipe — see SET_HEADER_VAR_REJECTED_NOTE)\n\n        // 🧬 RemoveHeaderVar{name:\"$INSBASE\"} — dispatch resets to Point::origin().\n        \"remove-header-var-applied\" => {\n            let mut after = base_doc();\n            after.header.insertion_base = Point::origin();\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveHeaderVar{name:\"$SEMIO_TEST_MISSING_VAR\"} — a name no real DXF writer emits, so\n        // it is genuinely absent from `base.header_vars`: validate_named_targets's removal path\n        // requires `unique(key)` (occurrences == Some(1)); an absent key is `None`, not `Some(1)`,\n        // so this is `invalid-remove-target` (🔺️diff/🦀️.rs:1551) regardless of which\n        // reference library reads the bytes — a carrier-independent rejection.\n        \"remove-header-var-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 InsertLayer{index:1, layer:\"MARKERS\"} — a name that does not yet exist, at a valid index.\n        \"insert-layer-applied\" => {\n            let mut after = base_doc();\n            insert_layer_at(&mut after, 1, Layer { name: \"MARKERS\".to_string(), color: Color::from_index(6), line_type_name: \"CONTINUOUS\".to_string(), ..Default::default() });\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 InsertLayer{index:1, layer:\"0\"} — \"0\" already exists in the base layer table:\n        // validate_named_targets's add-path rejects any `present(key)` name — `invalid-add-target`.\n        \"insert-layer-rejected-duplicate\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 RemoveLayer{name:\"DIMS\"} — the unique, present target.\n        \"remove-layer-applied\" => {\n            let mut after = base_doc();\n            let at = after.layers().position(|l| l.name == \"DIMS\").expect(\"DIMS present\");\n            after.remove_layer(at);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveLayer{name:\"GHOST_LAYER\"} — absent: `invalid-remove-target`.\n        \"remove-layer-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 SetLayer{name:\"DIMS\", layer:{color:4, linetype:\"DASHED\"}} — whole-value replace of the\n        // named row (colour changes; linetype restated).\n        \"set-layer-applied\" => {\n            let mut after = base_doc();\n            if let Some(slot) = after.layers_mut().find(|l| l.name == \"DIMS\") {\n                slot.color = Color::from_index(4);\n                slot.line_type_name = \"DASHED\".to_string();\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetLayer{name:\"GHOST_LAYER\", ..} — `diff_set_layer` ALWAYS emits a `modified` entry\n        // (unlike SetHeaderVar, `layer_diff_between(&old.unwrap_or_default(), layer)` runs\n        // regardless of presence — 🧬️mutations/🦀️.rs:247-250); absent name fails the modify path's\n        // `unique(key)` check — `invalid-modify-target`.\n        \"set-layer-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 InsertStyle{index:1, style:\"LABELS\"} — new name, valid index.\n        \"insert-style-applied\" => {\n            let mut after = base_doc();\n            insert_style_at(&mut after, 1, Style { name: \"LABELS\".to_string(), primary_font_file_name: \"arial.ttf\".to_string(), text_height: 2.5, ..Default::default() });\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 InsertStyle{index:1, style:\"STANDARD\"} — duplicate of the base's own row.\n        \"insert-style-rejected-duplicate\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 RemoveStyle{name:\"NOTES\"}.\n        \"remove-style-applied\" => {\n            let mut after = base_doc();\n            let at = after.styles().position(|s| s.name == \"NOTES\").expect(\"NOTES present\");\n            after.remove_style(at);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveStyle{name:\"GHOST_STYLE\"} — absent.\n        \"remove-style-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 SetStyle{name:\"NOTES\", style:{font:\"arial.ttf\"}} — font changes from \"romans.shx\".\n        \"set-style-applied\" => {\n            let mut after = base_doc();\n            if let Some(slot) = after.styles_mut().find(|s| s.name == \"NOTES\") {\n                slot.primary_font_file_name = \"arial.ttf\".to_string();\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetStyle{name:\"GHOST_STYLE\", ..} — same always-modifies shape as SetLayer; absent name\n        // fails `unique(key)` — `invalid-modify-target`.\n        \"set-style-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 InsertLinetype{index:1, linetype:\"CENTER\"} — new name, valid index.\n        \"insert-linetype-applied\" => {\n            let mut after = base_doc();\n            insert_linetype_at(&mut after, 1, LineType { name: \"CENTER\".to_string(), description: \"Center line\".to_string(), ..Default::default() });\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 InsertLinetype{index:1, linetype:\"CONTINUOUS\"} — duplicate of the base's own row.\n        \"insert-linetype-rejected-duplicate\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 RemoveLinetype{name:\"DASHED\"}.\n        \"remove-linetype-applied\" => {\n            let mut after = base_doc();\n            let at = after.line_types().position(|l| l.name == \"DASHED\").expect(\"DASHED present\");\n            after.remove_line_type(at);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveLinetype{name:\"GHOST_LTYPE\"} — absent.\n        \"remove-linetype-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 SetLinetype{name:\"DASHED\", linetype:{description:\"Dash pattern\"}}.\n        \"set-linetype-applied\" => {\n            let mut after = base_doc();\n            if let Some(slot) = after.line_types_mut().find(|l| l.name == \"DASHED\") {\n                slot.description = \"Dash pattern\".to_string();\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetLinetype{name:\"GHOST_LTYPE\", ..} — same always-modifies shape; absent name fails\n        // `unique(key)` — `invalid-modify-target`.\n        \"set-linetype-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 InsertEntity{index:2, entity:circle} — valid index (<= current length 7).\n        \"insert-entity-applied\" => {\n            let mut after = base_doc();\n            insert_entity_at(&mut after, 2, on_layer(\"0\", EntityType::Circle(Circle { center: Point::new(1200.0, 100.0, 0.0), radius: 30.0, ..Default::default() })));\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 InsertEntity{index:99, ..} — 99 > the evolving length (7): `validate_indexed_targets`'s\n        // add-path rejects `index > length` — `invalid-add-index`.\n        \"insert-entity-rejected-out-of-bounds\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 RemoveEntity{index:3} — the ARC, a valid middle target.\n        \"remove-entity-applied\" => {\n            let mut after = base_doc();\n            after.remove_entity(3);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveEntity{index:99} — 99 >= base length (7): `invalid-remove-index`.\n        \"remove-entity-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 SetEntity{index:5, entity:text} — index 5 (the TEXT) exists, so `diff()` takes the\n        // `Some(old) => diff_set_entity(...)` branch (🧬️mutations/🦀️.rs:268-271): a genuine\n        // whole-value replace, not the insert-fallback.\n        \"set-entity-applied\" => {\n            let mut after = base_doc();\n            if let Some(slot) = after.entities_mut().nth(5) {\n                slot.specific = EntityType::Text(Text { location: Point::new(200.0, 260.0, 0.0), text_height: 80.0, value: \"PLATE REVISION B\".to_string(), text_style_name: \"STANDARD\".to_string(), ..Default::default() });\n                slot.common.layer = \"DIMS\".to_string();\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetEntity{index:99, ..} — 99 is absent, so `diff()` takes the `None =>\n        // diff_insert_entity(*index, ..)` FALLBACK (🧬️mutations/🦀️.rs:268-271) — an out-of-bounds\n        // insert, not a \"missing target\": 99 > the evolving length (7), so\n        // `validate_indexed_targets`'s add-path rejects it the same way `insert-entity` does.\n        \"set-entity-rejected-out-of-bounds\" => Some(RecipeOutput::Pair(before, None)),\n\n        // 🧬 InsertBlock{index:1, block:\"BENCH_MARK\"} — valid index (<= current length 2).\n        \"insert-block-applied\" => {\n            let mut after = base_doc();\n            insert_block_at(&mut after, 1, Block { name: \"BENCH_MARK\".to_string(), layer: \"0\".to_string(), base_point: Point::new(0.0, 0.0, 0.0), entities: vec![on_layer(\"0\", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(100.0, 0.0, 0.0), ..Default::default() }))], ..Default::default() });\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 InsertBlock{index:99, ..} — 99 > the evolving length (2): `invalid-add-index`.\n        \"insert-block-rejected-out-of-bounds\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 RemoveBlock{index:1} — \"BENCH\", a valid target.\n        \"remove-block-applied\" => {\n            let mut after = base_doc();\n            after.remove_block(1);\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 RemoveBlock{index:99} — 99 >= base length (2): `invalid-remove-index`.\n        \"remove-block-rejected-missing\" => Some(RecipeOutput::Pair(before, None)),\n        // 🧬 SetBlock{index:0, ..} — index 0 (\"SHELTER_POST\") exists, so `diff()` takes the\n        // `Some(old) => diff_set_block(...)` branch: a genuine whole-value replace of base\n        // point + nested entities.\n        \"set-block-applied\" => {\n            let mut after = base_doc();\n            if let Some(slot) = after.blocks_mut().nth(0) {\n                slot.base_point = Point::new(5.0, 5.0, 0.0);\n                slot.entities = vec![on_layer(\"0\", EntityType::Circle(Circle { center: Point::new(0.0, 0.0, 0.0), radius: 20.0, ..Default::default() }))];\n            }\n            Some(RecipeOutput::Pair(before, Some(after)))\n        }\n        // 🧬 SetBlock{index:99, ..} — 99 is absent, falls back to `diff_insert_block(99, ..)`; 99 >\n        // the evolving length (2), so the fallback's own add-path rejects it — `invalid-add-index`.\n        \"set-block-rejected-out-of-bounds\" => Some(RecipeOutput::Pair(before, None)),\n\n        _ => None,\n    }\n}\n\nconst RECIPE_IDS: &[&str] = &[\n    \"drafting-plate\",\n    \"no-mutation-no-op\",\n    \"set-snapshot-applied\",\n    \"set-snapshot-no-op\",\n    \"set-snapshot-rejected-duplicate-layer\",\n    \"set-header-var-applied\",\n    \"remove-header-var-applied\",\n    \"remove-header-var-rejected-missing\",\n    \"insert-layer-applied\",\n    \"insert-layer-rejected-duplicate\",\n    \"remove-layer-applied\",\n    \"remove-layer-rejected-missing\",\n    \"set-layer-applied\",\n    \"set-layer-rejected-missing\",\n    \"insert-style-applied\",\n    \"insert-style-rejected-duplicate\",\n    \"remove-style-applied\",\n    \"remove-style-rejected-missing\",\n    \"set-style-applied\",\n    \"set-style-rejected-missing\",\n    \"insert-linetype-applied\",\n    \"insert-linetype-rejected-duplicate\",\n    \"remove-linetype-applied\",\n    \"remove-linetype-rejected-missing\",\n    \"set-linetype-applied\",\n    \"set-linetype-rejected-missing\",\n    \"insert-entity-applied\",\n    \"insert-entity-rejected-out-of-bounds\",\n    \"remove-entity-applied\",\n    \"remove-entity-rejected-missing\",\n    \"set-entity-applied\",\n    \"set-entity-rejected-out-of-bounds\",\n    \"insert-block-applied\",\n    \"insert-block-rejected-out-of-bounds\",\n    \"remove-block-applied\",\n    \"remove-block-rejected-missing\",\n    \"set-block-applied\",\n    \"set-block-rejected-out-of-bounds\",\n];\n//#endregion 🔖️Recipes\n\n//#region 🔖️Json\nfn json_str(s: &str) -> String {\n    let mut out = String::with_capacity(s.len() + 2);\n    out.push('\"');\n    for c in s.chars() {\n        match c {\n            '\"' => out.push_str(\"\\\\\\\"\"),\n            '\\\\' => out.push_str(\"\\\\\\\\\"),\n            '\\n' => out.push_str(\"\\\\n\"),\n            '\\r' => out.push_str(\"\\\\r\"),\n            '\\t' => out.push_str(\"\\\\t\"),\n            c if (c as u32) < 0x20 => out.push_str(&format!(\"\\\\u{:04x}\", c as u32)),\n            c => out.push(c),\n        }\n    }\n    out.push('\"');\n    out\n}\n\nfn json_num(n: f64) -> String {\n    if n.is_finite() { format!(\"{n}\") } else { \"0\".to_string() }\n}\n\nfn point_json(p: &Point) -> String {\n    format!(\"[{},{},{}]\", json_num(p.x), json_num(p.y), json_num(p.z))\n}\n\nfn layer_json(l: &Layer) -> String {\n    format!(\"{{\\\"name\\\":{},\\\"color\\\":{},\\\"linetype\\\":{}}}\", json_str(&l.name), l.color.index().unwrap_or(7), json_str(&l.line_type_name))\n}\nfn style_json(s: &Style) -> String {\n    format!(\"{{\\\"name\\\":{},\\\"font\\\":{}}}\", json_str(&s.name), json_str(&s.primary_font_file_name))\n}\nfn linetype_json(l: &LineType) -> String {\n    format!(\"{{\\\"name\\\":{},\\\"description\\\":{}}}\", json_str(&l.name), json_str(&l.description))\n}\n\n/// 📄️ Semantic projection of one entity — the exact field set `../../../🔮️oracles/🦀️.rs`'s\n/// own `entity_to_json` produces for the six typed kinds this subset's mutations construct\n/// (line/circle/arc/text/solid/insert); any other kind projects as `{\"entityKind\":\"other\"}` plus\n/// its layer, mirroring that module's own fallback.\nfn entity_json(e: &Entity) -> String {\n    let layer = json_str(&e.common.layer);\n    match &e.specific {\n        EntityType::Line(l) => format!(\"{{\\\"entityKind\\\":\\\"line\\\",\\\"layer\\\":{layer},\\\"start\\\":{},\\\"end\\\":{}}}\", point_json(&l.p1), point_json(&l.p2)),\n        EntityType::Circle(c) => format!(\"{{\\\"entityKind\\\":\\\"circle\\\",\\\"layer\\\":{layer},\\\"center\\\":{},\\\"radius\\\":{}}}\", point_json(&c.center), json_num(c.radius)),\n        EntityType::Arc(a) => format!(\"{{\\\"entityKind\\\":\\\"arc\\\",\\\"layer\\\":{layer},\\\"center\\\":{},\\\"radius\\\":{},\\\"startAngle\\\":{},\\\"endAngle\\\":{}}}\", point_json(&a.center), json_num(a.radius), json_num(a.start_angle), json_num(a.end_angle)),\n        EntityType::Text(t) => format!(\"{{\\\"entityKind\\\":\\\"text\\\",\\\"layer\\\":{layer},\\\"position\\\":{},\\\"height\\\":{},\\\"value\\\":{}}}\", point_json(&t.location), json_num(t.text_height), json_str(&t.value)),\n        EntityType::Solid(s) => format!(\"{{\\\"entityKind\\\":\\\"solid\\\",\\\"layer\\\":{layer},\\\"points\\\":[{},{},{},{}]}}\", point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)),\n        EntityType::Insert(i) => format!(\"{{\\\"entityKind\\\":\\\"insert\\\",\\\"layer\\\":{layer},\\\"blockName\\\":{},\\\"position\\\":{}}}\", json_str(&i.name), point_json(&i.location)),\n        _ => format!(\"{{\\\"entityKind\\\":\\\"other\\\",\\\"layer\\\":{layer}}}\"),\n    }\n}\n\nfn block_json(b: &Block) -> String {\n    let entities: Vec<String> = b.entities.iter().map(entity_json).collect();\n    format!(\"{{\\\"name\\\":{},\\\"basePoint\\\":{},\\\"entities\\\":[{}]}}\", json_str(&b.name), point_json(&b.base_point), entities.join(\",\"))\n}\n\n/// 📄️ Whole-document semantic projection, the exact shape `semantic-dxf-r12-v1` compares —\n/// independently re-derived from `../../../🔮️oracles/🦀️.rs`'s own `project_dxf_r12` (same\n/// field names, same field set), never imported from it.\nfn project_json(drawing: &Drawing) -> String {\n    let layers: Vec<String> = drawing.layers().map(layer_json).collect();\n    let styles: Vec<String> = drawing.styles().map(style_json).collect();\n    let linetypes: Vec<String> = drawing.line_types().map(linetype_json).collect();\n    let blocks: Vec<String> = drawing.blocks().map(block_json).collect();\n    let entities: Vec<String> = drawing.entities().map(entity_json).collect();\n    format!(\n        \"{{\\\"acadVersion\\\":{},\\\"insertionBase\\\":{},\\\"layers\\\":[{}],\\\"styles\\\":[{}],\\\"linetypes\\\":[{}],\\\"blocks\\\":[{}],\\\"entities\\\":[{}]}}\",\n        json_str(&format!(\"{:?}\", drawing.header.version)),\n        point_json(&drawing.header.insertion_base),\n        layers.join(\",\"),\n        styles.join(\",\"),\n        linetypes.join(\",\"),\n        blocks.join(\",\"),\n        entities.join(\",\"),\n    )\n}\n//#endregion 🔖️Json\n\n//#region 🔖️Entry\nfn cmd_build(id: &str, out_dir: &str) -> i32 {\n    let Some(output) = recipe(id) else {\n        eprintln!(\"[dxf-r12-any-engine] unknown recipe {id:?} — known: {}\", RECIPE_IDS.join(\", \"));\n        return 1;\n    };\n    let dir = Path::new(out_dir);\n    fs::create_dir_all(&dir).expect(\"create fixture recipe directory\");\n    match output {\n        RecipeOutput::Single(doc) => {\n            let path = dir.join(\"📐️drafting-plate.dxf\");\n            fs::write(&path, encode(&doc)).unwrap_or_else(|e| panic!(\"write {}: {e}\", path.display()));\n            eprintln!(\"[dxf-r12-any-engine] {id}: {} -> {}\", path.display(), out_dir);\n        }\n        RecipeOutput::Pair(before, after) => {\n            let before_path = dir.join(\"⬅️before.dxf\");\n            fs::write(&before_path, encode(&before)).unwrap_or_else(|e| panic!(\"write {}: {e}\", before_path.display()));\n            match after {\n                Some(after) => {\n                    let after_path = dir.join(\"➡️after.dxf\");\n                    fs::write(&after_path, encode(&after)).unwrap_or_else(|e| panic!(\"write {}: {e}\", after_path.display()));\n                    eprintln!(\"[dxf-r12-any-engine] {id}: before.dxf + after.dxf -> {}\", dir.display());\n                }\n                None => eprintln!(\"[dxf-r12-any-engine] {id}: before.dxf only (rejected) -> {}\", dir.display()),\n            }\n        }\n    }\n    0\n}\n\nfn cmd_project(path: &str) -> i32 {\n    let bytes = match fs::read(path) {\n        Ok(b) => b,\n        Err(e) => {\n            eprintln!(\"[dxf-r12-any-engine] cannot read {path}: {e}\");\n            return 1;\n        }\n    };\n    let drawing = match Drawing::load(&mut &bytes[..]) {\n        Ok(d) => d,\n        Err(e) => {\n            eprintln!(\"[dxf-r12-any-engine] dxf load failed for {path}: {e:?}\");\n            return 1;\n        }\n    };\n    println!(\"{}\", project_json(&drawing));\n    0\n}\n\n/// 🧪️ SCRATCH-ONLY verification helper, never part of `RECIPE_IDS`/the committed corpus: loads a\n/// real DXF, perturbs the FIRST circle entity's radius by `delta` through `dxf`'s own typed\n/// `Circle.radius` field (never a byte-level text edit), and re-saves through `dxf`'s own writer —\n/// used exactly once, ad hoc, to demonstrate `semantic-dxf-r12-v1`'s 1e-4 tolerance discriminates\n/// for real (ticket-root report). Output goes to `🗑️temp/`, never `../🧫️fixtures/`.\nfn cmd_perturb_radius_debug(in_path: &str, out_path: &str, delta: f64) -> i32 {\n    let bytes = fs::read(in_path).unwrap_or_else(|e| panic!(\"read {in_path}: {e}\"));\n    let mut drawing = Drawing::load(&mut &bytes[..]).unwrap_or_else(|e| panic!(\"dxf load {in_path}: {e:?}\"));\n    let mut touched = false;\n    for entity in drawing.entities_mut() {\n        if let EntityType::Circle(circle) = &mut entity.specific {\n            circle.radius += delta;\n            touched = true;\n            break;\n        }\n    }\n    if !touched {\n        eprintln!(\"[dxf-r12-any-engine] perturb-radius-debug: no circle entity found in {in_path}\");\n        return 1;\n    }\n    fs::write(out_path, encode(&drawing)).unwrap_or_else(|e| panic!(\"write {out_path}: {e}\"));\n    eprintln!(\"[dxf-r12-any-engine] perturb-radius-debug: {in_path} radius+{delta} -> {out_path}\");\n    0\n}\n\nfn main() {\n    let args: Vec<String> = env::args().collect();\n    let code = match args.get(1).map(String::as_str) {\n        Some(\"build\") => {\n            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {\n                eprintln!(\"usage: engine build <recipe-id> <out-dir>\");\n                std::process::exit(2);\n            };\n            cmd_build(id, out_dir)\n        }\n        Some(\"project\") => {\n            let Some(path) = args.get(2) else {\n                eprintln!(\"usage: engine project <path-to-dxf>\");\n                std::process::exit(2);\n            };\n            cmd_project(path)\n        }\n        Some(\"list-recipes\") => {\n            for id in RECIPE_IDS {\n                println!(\"{id}\");\n            }\n            0\n        }\n        Some(\"perturb-radius-debug\") => {\n            let (Some(in_path), Some(out_path), Some(delta)) = (args.get(2), args.get(3), args.get(4).and_then(|s| s.parse::<f64>().ok())) else {\n                eprintln!(\"usage: engine perturb-radius-debug <in.dxf> <out.dxf> <delta>\");\n                std::process::exit(2);\n            };\n            cmd_perturb_radius_debug(in_path, out_path, delta)\n        }\n        _ => {\n            eprintln!(\"usage: engine build <recipe-id> <out-dir> | project <path-to-dxf> | list-recipes | perturb-radius-debug <in> <out> <delta>\");\n            2\n        }\n    };\n    std::process::exit(code);\n}\n//#endregion 🔖️Entry\n\n//#region 🔖️Tests\n#[cfg(test)]\n#[path = \"./🧪️tests/🧪️unit/🦀️.rs\"]\nmod tests;\n//#endregion 🔖️Tests\n",
      "sha256": "fe7a986f96c53778ffaede6ba6c201aec1d0d559abe90297cd8d184ada8820a3",
      "bytes": 37390
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🧪️tests/🔬️smoke/🦀️.rs",
      "source": "\nuse super::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};\nuse semio_repo_test_host::parse_json;\n\nconst FIXTURE: &[u8] = include_bytes!(\"../../../🖼️assets/🚏️bus-shelter/🖊️.dxf\");\n\nconst ROWS: &[(&str, &str)] = &[\n    (\"set-snapshot\", r#\"{\"snapshot\": {\"schema\": \"stdio.dxf\", \"headerVars\": [{\"name\": \"$ACADVER\", \"groupCode\": 1, \"value\": {\"kind\": \"str\", \"value\": \"AC1009\"}}, {\"name\": \"$INSBASE\", \"groupCode\": 10, \"value\": {\"kind\": \"point\", \"value\": [5, 5, 0]}}], \"tables\": {\"layers\": [{\"name\": \"0\", \"color\": 7, \"linetype\": \"CONTINUOUS\", \"flags\": 0}]}, \"otherTables\": [], \"blocks\": [], \"entities\": [{\"circle\": {\"center\": [0, 0, 0], \"radius\": 42, \"layer\": \"0\"}}]}}\"#),\n    (\"set-header-var\", r#\"{\"name\": \"$INSBASE\", \"headerVar\": {\"name\": \"$INSBASE\", \"groupCode\": 10, \"value\": {\"kind\": \"point\", \"value\": [15, 25, 0]}}}\"#),\n    (\"remove-header-var\", r#\"{\"name\": \"$INSBASE\"}\"#),\n    (\"insert-layer\", r#\"{\"index\": 1, \"layer\": {\"name\": \"MARKERS\", \"color\": 6, \"linetype\": \"CONTINUOUS\", \"flags\": 0}}\"#),\n    (\"remove-layer\", r#\"{\"name\": \"DIMS\"}\"#),\n    (\"set-layer\", r#\"{\"name\": \"DIMS\", \"layer\": {\"name\": \"DIMS\", \"color\": 4, \"linetype\": \"DASHED\", \"flags\": 0}}\"#),\n    (\"insert-style\", r#\"{\"index\": 1, \"style\": {\"name\": \"LABELS\", \"flags\": 0, \"fontName\": \"arial.ttf\"}}\"#),\n    (\"remove-style\", r#\"{\"name\": \"NOTES\"}\"#),\n    (\"set-style\", r#\"{\"name\": \"NOTES\", \"style\": {\"name\": \"NOTES\", \"flags\": 0, \"fontName\": \"romans.shx\"}}\"#),\n    (\"insert-linetype\", r#\"{\"index\": 1, \"linetype\": {\"name\": \"CENTER\", \"flags\": 0, \"description\": \"Center line\"}}\"#),\n    (\"remove-linetype\", r#\"{\"name\": \"DASHED\"}\"#),\n    (\"set-linetype\", r#\"{\"name\": \"DASHED\", \"linetype\": {\"name\": \"DASHED\", \"flags\": 0, \"description\": \"Dash pattern\"}}\"#),\n    (\"insert-entity\", r#\"{\"index\": 2, \"entity\": {\"circle\": {\"center\": [1200, 100, 0], \"radius\": 30, \"layer\": \"0\"}}}\"#),\n    (\"remove-entity\", r#\"{\"index\": 3}\"#),\n    (\"set-entity\", r#\"{\"index\": 5, \"entity\": {\"text\": {\"position\": [200, 260, 0], \"height\": 80, \"value\": \"WAVE 7 SHELTER\", \"layer\": \"DIMS\"}}}\"#),\n    (\"insert-block\", r#\"{\"index\": 1, \"block\": {\"name\": \"BENCH_MARK\", \"basePoint\": [0, 0, 0], \"entities\": [{\"line\": {\"start\": [0, 0, 0], \"end\": [100, 0, 0], \"layer\": \"0\"}}]}}\"#),\n    (\"remove-block\", r#\"{\"index\": 1}\"#),\n    (\"set-block\", r#\"{\"index\": 0, \"block\": {\"name\": \"SHELTER_POST\", \"basePoint\": [0, 0, 0], \"entities\": [{\"circle\": {\"center\": [0, 0, 0], \"radius\": 20, \"layer\": \"0\"}}]}}\"#),\n];\n\n#[test]\nfn all_kinds_mutate_and_invert_cleanly() {\n    assert_eq!(ROWS.len(), 18, \"must exercise all 18 declared kinds\");\n    let input = FIXTURE.to_vec();\n    let base_projection = project_dxf_r12(&input).expect(\"project base fixture\");\n\n    for (kind, params) in ROWS {\n        let spec_text = format!(r#\"{{\"kind\": \"{kind}\", \"params\": {params}}}\"#);\n        let spec = parse_json(&spec_text).unwrap_or_else(|e| panic!(\"bad spec JSON for {kind}: {e}\"));\n\n        let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|e| panic!(\"mutate {kind} failed: {e}\"));\n        assert!(!mutated.is_empty(), \"mutate {kind} produced empty bytes\");\n        let mutated_projection = project_dxf_r12(&mutated).unwrap_or_else(|e| panic!(\"project mutate {kind} output failed: {e}\"));\n        assert_ne!(mutated_projection, base_projection, \"mutate {kind} produced no semantic change\");\n\n        let inverted = oracle_apply_mutation_inverse(&input, &spec).unwrap_or_else(|e| panic!(\"inverse {kind} failed: {e}\"));\n        let inverted_projection = project_dxf_r12(&inverted).unwrap_or_else(|e| panic!(\"project inverse {kind} output failed: {e}\"));\n        assert_eq!(inverted_projection, base_projection, \"inverse {kind} did not restore the base projection\");\n    }\n}\n\n#[test]\nfn identity_round_trip_is_not_byte_identical() {\n    let input = FIXTURE.to_vec();\n    let output = oracle_round_trip(&input).expect(\"round-trip re-encode\");\n    assert_ne!(output, input, \"byte pass-through: dxf-crate re-encode is bit-identical to the input\");\n    let base_projection = project_dxf_r12(&input).expect(\"project input\");\n    let output_projection = project_dxf_r12(&output).expect(\"project output\");\n    assert_eq!(base_projection, output_projection, \"the round-trip re-encode changed the semantic projection\");\n}\n",
      "sha256": "9de30ac116ad61b6e7f68743c3b346f545b4030a7d761ead1039508092603383",
      "bytes": 4269
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📊️tables/🧪️tests/📊️mutate-dxf-r12-tables/🦀️.rs",
      "source": "//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR\n//! wave 7.\n//!\n//! Every scenario copies the derived, committed R12 `🚏️bus-shelter` drawing into the case work\n//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`\n//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs`'s\n//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's\n//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind\n//! `DxfMutation` vocabulary. Each side hands the drawing it produced to the `semantic-dxf-r12-v1`\n//! profile's `dxf-r12-reader-compare-v1` pipeline — the oracle's as `expected-dxf`, the subject's as\n//! `actual-dxf` — whose `dxf` 0.6 probes read both files independently. The subject half is gated\n//! behind the generated host's `sut` feature so the oracle-only run never compiles the local\n//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject\n//! phase runs, and wave 14 ran the full differential comparison against the oracle.\n\nuse semio_repo_test_host::{Adapter, Context, Json, Outcome};\nuse semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};\n\n\n//#region 🔖️Input\nconst INPUT: &str = \"asset://🚏️bus-shelter/🖊️.dxf\";\n\n/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.\nfn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {\n    let copy = ctx.copy_fixture(INPUT, Some(\"bus-shelter-r12.dxf\"))?;\n    std::fs::read(&copy).map_err(|error| error.to_string())\n}\n\n/// 📦️ The drawing one side produced, written as the `role` artifact (`expected-dxf` for the oracle, `actual-dxf` for the\n/// subject) the `dxf-r12-reader-compare-v1` pipeline hands to its probes.\nfn produced(ctx: &Context, role: &str, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {\n    let path = ctx.artifact(role, &format!(\"{role}.dxf\"))?;\n    std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;\n    Ok(Outcome::with_raw(bytes, projection).artifact(role, &path, \"image/vnd.dxf\"))\n}\n//#endregion 🔖️Input\n\n//#region 🔖️Laws\n/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law\n/// violation must name the field that broke it rather than dump two whole documents at the reader.\nfn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {\n    match (expected, actual) {\n        (Json::Object(left), Json::Object(right)) => {\n            for (key, value) in left {\n                match right.iter().find(|(name, _)| name == key) {\n                    Some((_, other)) => {\n                        if let Some(found) = first_divergence(&format!(\"{path}.{key}\"), value, other) {\n                            return Some(found);\n                        }\n                    }\n                    None => return Some(format!(\"{path}.{key} is absent from the result\")),\n                }\n            }\n            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!(\"{path}.{key} appeared in the result out of nowhere\"))\n        }\n        (Json::Array(left), Json::Array(right)) => {\n            if left.len() != right.len() {\n                return Some(format!(\"{path} holds {} member(s), expected {}\", right.len(), left.len()));\n            }\n            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!(\"{path}[{index}]\"), value, other))\n        }\n        _ if expected == actual => None,\n        _ => Some(format!(\"{path}: expected {} but read {}\", expected.to_string(), actual.to_string())),\n    }\n}\n\n/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise\n/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and\n/// its scenario passes whenever the reference library merely declined to error.\nfn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {\n    match first_divergence(\"projection\", expected, actual) {\n        Some(divergence) => Err(format!(\"{law}: {divergence}\")),\n        None => Ok(()),\n    }\n}\n//#endregion 🔖️Laws\n\n//#region 🔖️Oracle\n/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`\n/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in\n/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row\n/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference\n/// library declined to error, which is exactly the failure this platform exists to prevent. The\n/// baseline runs one `dxf` round trip so the comparison isolates the mutation rather than the\n/// writer's own normal form.\nfn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let kind = spec.str(\"kind\");\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    if projection == baseline {\n        return Err(format!(\"{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name\"));\n    }\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse\n/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and\n/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the\n/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken\n/// through one `dxf` round trip so the two sides carry the same serializer normalisation and the\n/// comparison isolates the mutation pair itself.\nfn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(&format!(\"inverse law violated for {:?} -- undoing it did not restore the drawing\", spec.str(\"kind\")), &baseline, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real\n/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST\n/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is\n/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --\n/// so the byte tripwire is real evidence that the document was parsed rather than copied.\nfn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let before = project_dxf_r12(&input)?;\n    let bytes = oracle_round_trip(&input)?;\n    if bytes == input {\n        return Err(\"byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed\".to_string());\n    }\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(\"identity round trip is not semantics-preserving\", &before, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n//#endregion 🔖️Oracle\n\n//#region 🔖️Subject\n#[cfg(feature = \"sut\")]\nmod subject {\n    use super::{mutable_input, produced};\n    use semio_repo_test_host::{Context, Json, Outcome};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, DxfMutation};\n    use semio_repo_test_host::law::wire_operation;\n    use semio_s_artifact_stdio_dxf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document};\n    use semio_s_artifact_stdio_dxf::DxfSnapshot;\n    use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;\n\n    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.\n    fn mutation_of(spec: &Json) -> Result<DxfMutation, String> {\n        wire_operation(&spec.str(\"kind\"), &spec.get(\"params\").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)\n    }\n\n    fn decode(ctx: &Context) -> Result<DxfSnapshot, String> {\n        let input = mutable_input(ctx)?;\n        parse_dxf_document(std::str::from_utf8(&input).map_err(|error| error.to_string())?)\n    }\n\n    fn outcome(ctx: &Context, snapshot: &DxfSnapshot) -> Result<Outcome, String> {\n        let output = print_dxf_document(snapshot).into_bytes();\n        let projection = project_dxf_r12(&output)?;\n        produced(ctx, \"actual-dxf\", output, projection)\n    }\n\n    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` leaves the snapshot untouched and\n    /// reports the refusal as the outcome's only messages — every `DxfMutation::diff` arm builds its outcome without one —\n    /// so a non-empty message list IS a refusal.\n    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {\n        match apply_dxf_mutation(snapshot, mutation).messages().first() {\n            Some(refusal) => Err(format!(\"{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}\")),\n            None => Ok(()),\n        }\n    }\n\n    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        applied(&mut snapshot, &mutation_of(&spec)?, &spec.str(\"kind\"))?;\n        outcome(ctx, &snapshot)\n    }\n\n    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.\n    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        let kind = spec.str(\"kind\");\n        let forward = mutation_of(&spec)?;\n        let backward = mutation_inverse(&forward, &snapshot);\n        applied(&mut snapshot, &forward, &kind)?;\n        for mutation in &backward {\n            applied(&mut snapshot, mutation, &format!(\"the inverse of {kind}\"))?;\n        }\n        outcome(ctx, &snapshot)\n    }\n\n    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its\n    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/\n    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`\n    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module\n    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.\n    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {\n        let snapshot = decode(ctx)?;\n        if print_dxf_document(&snapshot).into_bytes() == mutable_input(ctx)? {\n            return Err(\"byte pass-through: output is bit-identical to the input\".to_string());\n        }\n        outcome(ctx, &snapshot)\n    }\n}\n//#endregion 🔖️Subject\n\n//#region 🔖️Registration\n/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline\n/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.\npub fn adapter() -> Adapter {\n    let mut built = Adapter::new(\"rust\");\n    built = built.oracle(\"mutate\", mutate_oracle).oracle(\"inverse\", inverse_oracle);\n    built = built.oracle(\"identity-round-trip\", identity_round_trip_oracle);\n    #[cfg(feature = \"sut\")]\n    {\n        built = built.subject(\"mutate\", subject::mutate).subject(\"inverse\", subject::inverse);\n        built = built.subject(\"identity-round-trip\", subject::identity_round_trip);\n    }\n    built\n}\n//#endregion 🔖️Registration\n",
      "sha256": "1587d658472db1e0599e666ea6ef22a4522fcdc84e9fbc2cbb607454e7d4992f",
      "bytes": 12603
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/🧩️entities/🧪️tests/🧩️mutate-dxf-r12-entities/🦀️.rs",
      "source": "//! 🦀️ DXF R12 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR\n//! wave 7.\n//!\n//! Every scenario copies the derived, committed R12 `🚏️bus-shelter` drawing into the case work\n//! directory first; the committed asset is never written to. `oracle` drives the registered `dxf`\n//! 0.6 reference implementation (`../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs`'s\n//! own `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's\n//! own `parse_dxf_document`/`print_dxf_document`/`apply_dxf_mutation` over the full 19-kind\n//! `DxfMutation` vocabulary. Each side hands the drawing it produced to the `semantic-dxf-r12-v1`\n//! profile's `dxf-r12-reader-compare-v1` pipeline — the oracle's as `expected-dxf`, the subject's as\n//! `actual-dxf` — whose `dxf` 0.6 probes read both files independently. The subject half is gated\n//! behind the generated host's `sut` feature so the oracle-only run never compiles the local\n//! implementation -- §5.3's own role separation, NOT a workaround for anything: the Rust subject\n//! phase runs, and wave 14 ran the full differential comparison against the oracle.\n\nuse semio_repo_test_host::{Adapter, Context, Json, Outcome};\nuse semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_dxf_r12};\n\n\n//#region 🔖️Input\nconst INPUT: &str = \"asset://🚏️bus-shelter/🖊️.dxf\";\n\n/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.\nfn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {\n    let copy = ctx.copy_fixture(INPUT, Some(\"bus-shelter-r12.dxf\"))?;\n    std::fs::read(&copy).map_err(|error| error.to_string())\n}\n\n/// 📦️ The drawing one side produced, written as the `role` artifact (`expected-dxf` for the oracle, `actual-dxf` for the\n/// subject) the `dxf-r12-reader-compare-v1` pipeline hands to its probes.\nfn produced(ctx: &Context, role: &str, bytes: Vec<u8>, projection: Json) -> Result<Outcome, String> {\n    let path = ctx.artifact(role, &format!(\"{role}.dxf\"))?;\n    std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;\n    Ok(Outcome::with_raw(bytes, projection).artifact(role, &path, \"image/vnd.dxf\"))\n}\n//#endregion 🔖️Input\n\n//#region 🔖️Laws\n/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law\n/// violation must name the field that broke it rather than dump two whole documents at the reader.\nfn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {\n    match (expected, actual) {\n        (Json::Object(left), Json::Object(right)) => {\n            for (key, value) in left {\n                match right.iter().find(|(name, _)| name == key) {\n                    Some((_, other)) => {\n                        if let Some(found) = first_divergence(&format!(\"{path}.{key}\"), value, other) {\n                            return Some(found);\n                        }\n                    }\n                    None => return Some(format!(\"{path}.{key} is absent from the result\")),\n                }\n            }\n            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!(\"{path}.{key} appeared in the result out of nowhere\"))\n        }\n        (Json::Array(left), Json::Array(right)) => {\n            if left.len() != right.len() {\n                return Some(format!(\"{path} holds {} member(s), expected {}\", right.len(), left.len()));\n            }\n            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!(\"{path}[{index}]\"), value, other))\n        }\n        _ if expected == actual => None,\n        _ => Some(format!(\"{path}: expected {} but read {}\", expected.to_string(), actual.to_string())),\n    }\n}\n\n/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise\n/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and\n/// its scenario passes whenever the reference library merely declined to error.\nfn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {\n    match first_divergence(\"projection\", expected, actual) {\n        Some(divergence) => Err(format!(\"{law}: {divergence}\")),\n        None => Ok(()),\n    }\n}\n//#endregion 🔖️Laws\n\n//#region 🔖️Oracle\n/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`\n/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in\n/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row\n/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference\n/// library declined to error, which is exactly the failure this platform exists to prevent. The\n/// baseline runs one `dxf` round trip so the comparison isolates the mutation rather than the\n/// writer's own normal form.\nfn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let kind = spec.str(\"kind\");\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    if projection == baseline {\n        return Err(format!(\"{kind:?} left the semantic projection of the R12 drawing unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name\"));\n    }\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse\n/// law -- a law that is checkable in-role, without a subject: `dxf` applies the forward mutation and\n/// then its own base-relative inverse (`oracle_apply_mutation_inverse`, one load/save cycle), and the\n/// restored drawing MUST project exactly as the untouched drawing does. The baseline is taken\n/// through one `dxf` round trip so the two sides carry the same serializer normalisation and the\n/// comparison isolates the mutation pair itself.\nfn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let spec = ctx.doc_json()?;\n    let baseline = project_dxf_r12(&oracle_round_trip(&input)?)?;\n    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(&format!(\"inverse law violated for {:?} -- undoing it did not restore the drawing\", spec.str(\"kind\")), &baseline, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n\n/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: `dxf` fully parses the real\n/// document and re-serializes it from its own typed `Drawing` alone, so the re-encoded bytes MUST\n/// carry the same semantic projection as the input AND MUST NOT be bit-identical to it. DXF R12 is\n/// not a byte-preserving carrier -- `dxf` regenerates the whole group-code stream from its model --\n/// so the byte tripwire is real evidence that the document was parsed rather than copied.\nfn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {\n    let input = mutable_input(ctx)?;\n    let before = project_dxf_r12(&input)?;\n    let bytes = oracle_round_trip(&input)?;\n    if bytes == input {\n        return Err(\"byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed\".to_string());\n    }\n    let projection = project_dxf_r12(&bytes)?;\n    assert_same_projection(\"identity round trip is not semantics-preserving\", &before, &projection)?;\n    produced(ctx, \"expected-dxf\", bytes, projection)\n}\n//#endregion 🔖️Oracle\n\n//#region 🔖️Subject\n#[cfg(feature = \"sut\")]\nmod subject {\n    use super::{mutable_input, produced};\n    use semio_repo_test_host::{Context, Json, Outcome};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::{apply_dxf_mutation, DxfMutation};\n    use semio_repo_test_host::law::wire_operation;\n    use semio_s_artifact_stdio_dxf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};\n    use semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::{parse_dxf_document, print_dxf_document};\n    use semio_s_artifact_stdio_dxf::DxfSnapshot;\n    use semio_s_artifact_stdio_dxf_test_oracle::standards::v_r12::subsets::header::project_dxf_r12;\n\n    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.\n    fn mutation_of(spec: &Json) -> Result<DxfMutation, String> {\n        wire_operation(&spec.str(\"kind\"), &spec.get(\"params\").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)\n    }\n\n    fn decode(ctx: &Context) -> Result<DxfSnapshot, String> {\n        let input = mutable_input(ctx)?;\n        parse_dxf_document(std::str::from_utf8(&input).map_err(|error| error.to_string())?)\n    }\n\n    fn outcome(ctx: &Context, snapshot: &DxfSnapshot) -> Result<Outcome, String> {\n        let output = print_dxf_document(snapshot).into_bytes();\n        let projection = project_dxf_r12(&output)?;\n        produced(ctx, \"actual-dxf\", output, projection)\n    }\n\n    /// 🚫️ A REFUSED mutation is a failure, never a silent no-op. `apply_dxf_mutation` leaves the snapshot untouched and\n    /// reports the refusal as the outcome's only messages — every `DxfMutation::diff` arm builds its outcome without one —\n    /// so a non-empty message list IS a refusal.\n    fn applied(snapshot: &mut DxfSnapshot, mutation: &DxfMutation, kind: &str) -> Result<(), String> {\n        match apply_dxf_mutation(snapshot, mutation).messages().first() {\n            Some(refusal) => Err(format!(\"{kind}: the mutation was REFUSED and the document left untouched — {refusal:?}\")),\n            None => Ok(()),\n        }\n    }\n\n    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        applied(&mut snapshot, &mutation_of(&spec)?, &spec.str(\"kind\"))?;\n        outcome(ctx, &snapshot)\n    }\n\n    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.\n    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {\n        let mut snapshot = decode(ctx)?;\n        let spec = ctx.doc_json()?;\n        let kind = spec.str(\"kind\");\n        let forward = mutation_of(&spec)?;\n        let backward = mutation_inverse(&forward, &snapshot);\n        applied(&mut snapshot, &forward, &kind)?;\n        for mutation in &backward {\n            applied(&mut snapshot, mutation, &format!(\"the inverse of {kind}\"))?;\n        }\n        outcome(ctx, &snapshot)\n    }\n\n    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its\n    /// typed snapshot and re-serialize from the model alone -- `parse_dxf_document`/\n    /// `print_dxf_document` are this subset's ONLY channel from input to output. `print_dxf_document`\n    /// regenerates a canonical NORMAL FORM (documented in `📸️snapshot/🦀️.rs`'s own module\n    /// doc), never raw byte preservation, so the tripwire is real rather than incidental.\n    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {\n        let snapshot = decode(ctx)?;\n        if print_dxf_document(&snapshot).into_bytes() == mutable_input(ctx)? {\n            return Err(\"byte pass-through: output is bit-identical to the input\".to_string());\n        }\n        outcome(ctx, &snapshot)\n    }\n}\n//#endregion 🔖️Subject\n\n//#region 🔖️Registration\n/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline\n/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.\npub fn adapter() -> Adapter {\n    let mut built = Adapter::new(\"rust\");\n    built = built.oracle(\"mutate\", mutate_oracle).oracle(\"inverse\", inverse_oracle);\n    built = built.oracle(\"identity-round-trip\", identity_round_trip_oracle);\n    #[cfg(feature = \"sut\")]\n    {\n        built = built.subject(\"mutate\", subject::mutate).subject(\"inverse\", subject::inverse);\n        built = built.subject(\"identity-round-trip\", subject::identity_round_trip);\n    }\n    built\n}\n//#endregion 🔖️Registration\n",
      "sha256": "1587d658472db1e0599e666ea6ef22a4522fcdc84e9fbc2cbb607454e7d4992f",
      "bytes": 12603
    }
  ],
  "cases": [
    {
      "id": "line",
      "input": "0\nSECTION\n2\nENTITIES\n0\nLINE\n8\n0\n10\n1.0\n20\n2.0\n30\n3.0\n11\n4.0\n21\n5.0\n31\n6.0\n0\nENDSEC\n0\nEOF\n",
      "entities": [
        {
          "start": [
            1,
            2,
            3
          ],
          "end": [
            4,
            5,
            6
          ],
          "entityKind": "line",
          "layer": "0"
        }
      ]
    },
    {
      "id": "empty",
      "input": "0\nSECTION\n2\nENTITIES\n0\nENDSEC\n0\nEOF\n",
      "entities": []
    },
    {
      "id": "malformed",
      "input": "not a group code\n",
      "errorPrefix": "dxf oracle: load failed:"
    }
  ]
}

```

Exact ordered coordinate-array literal inverses:

```json
[
  [
    "\n            1,\n            2,\n            3\n          ",
    "\n            1.0,\n            2.0,\n            3.0\n          "
  ],
  [
    "\n            4,\n            5,\n            6\n          ",
    "\n            4.0,\n            5.0,\n            6.0\n          "
  ]
]
```

### 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧬️schema/🖊️drawing-reader/🔣️.json

SHA256 `62dd55bf743b0f4fffa39fc1a774266cb0b78bc27fea2ec3dcdf048cb3de5c19`

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "additionalProperties": false,
  "required": ["schemaVersion", "owner", "package", "library", "externalDependency", "functions", "originals", "cases"],
  "properties": {
    "schemaVersion": { "const": 1 },
    "owner": { "const": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing" },
    "package": { "const": "semio-s-plugin-stdio-drawing-test-oracle" },
    "library": { "const": "semio_s_plugin_stdio_drawing_test_oracle" },
    "externalDependency": { "type": "object", "additionalProperties": false, "required": ["version", "optional"], "properties": { "version": { "const": "0.6" }, "optional": { "const": true } } },
    "functions": {
      "type": "array", "minItems": 11, "maxItems": 11, "uniqueItems": true,
      "items": { "type": "object", "additionalProperties": false, "required": ["name", "source", "start", "end", "declaration", "body", "sha256", "destination"], "properties": {
        "name": { "type": "string", "minLength": 1 }, "source": { "type": "string", "minLength": 1 },
        "start": { "type": "integer", "minimum": 0 }, "end": { "type": "integer", "minimum": 1 },
        "declaration": { "type": "string", "minLength": 1 }, "body": { "type": "string", "minLength": 1 },
        "sha256": { "type": "string", "pattern": "^[a-f0-9]{64}$" }, "destination": { "type": "string", "minLength": 1 }
      } }
    },
    "originals": {
      "type": "array", "minItems": 10, "uniqueItems": true,
      "items": { "type": "object", "additionalProperties": false, "required": ["path", "source", "sha256", "bytes"], "properties": {
        "path": { "type": "string", "minLength": 1 }, "source": { "type": "string", "minLength": 1 }, "sha256": { "type": "string", "pattern": "^[a-f0-9]{64}$" }, "bytes": { "type": "integer", "minimum": 1 }
      } }
    },
    "cases": {
      "type": "array", "minItems": 3, "uniqueItems": true,
      "items": { "type": "object", "additionalProperties": false, "required": ["id", "input"], "properties": {
        "id": { "type": "string", "minLength": 1 }, "input": { "type": "string", "minLength": 1 }, "entities": { "type": "array" }, "errorPrefix": { "const": "dxf oracle: load failed:" }
      }, "oneOf": [{ "properties": { "entities": {} }, "required": ["entities"] }, { "properties": { "errorPrefix": {} }, "required": ["errorPrefix"] }] }
    }
  }
}

```

Exact ordered coordinate-array literal inverses:

```json
[]
```

The native law gains one additional strict array representation assertion using its actual owned f64 coordinates and the untouched raw expected JSON. It adds no expected-value coercion, loose comparison or filter. Removing that exact assertion line and restoring the include literal reconstructs the original complete native law bytes. The original complete third-party entity equality and all original per-coordinate assertions remain.

## Complete Current Lower Family Native GREEN

Exact same full registered native route replay3 is terminal GREEN:one unit law passed,zero failures/ignored/measured/filtered;zero doctests,zero ignored;1.90s compile,2.6s uncached Nx. All closed cases execute. Both complete independent dxf entities and the actual owned coordinate arrays strictly match the literal expected f64 JSON; original full third-party equality and all original scalar assertions remain. The authored numeric literal inverses recover each original JSON file exactly while parsed IDs/inputs/scalars/structure remain identical.

The complete native law inverse (remove only the new strict representation assertion and restore only its include path literal) independently matches original full SHA256 `8a97391b77b28d7799f28ec72e5b6c0568550a8e30d92231bc8d2cefb436931e`. Actual executed binary `semio_s_plugin_stdio_drawing_test_oracle-4e4ec33a25c69ce3` hasfour rustc-owned checksum-bearing inputs:all current physical lengths andfirst-party TypeScript BLAKE3 digests match,zero mismatches. Relative dep-info paths were resolved against the actual standalone manifest directory,not a guessed root cwd.

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs`: 4945bytes, BLAKE3 `3c20c7b27ca60238427d87cd1dc7ba9b3586870a69b91872ef2dd83698faaaff`.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧰️support/🦀️.rs`: 606bytes, BLAKE3 `3cd2439e58eb5528ae106ff2af6a8376d5891ef8b79791cfc40c5409812c4d36`.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧪️tests/🔬️semantic/🦀️.rs`: 2524bytes, BLAKE3 `2d28a54483a809b1389a0ca1291b6a8db5585df9fa2d82a2d2894ea0b8dafc8a`.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🧫️fixtures/🖊️semantic/🔣️.json`: 618bytes, BLAKE3 `8013cb33e3ca24f28697a3abbcbeb77b80d9cb8b11be5b950acbd91aa6810944`.

This proves only the complete additional lower Drawing family cohort, including its strict independent reference law and exact compiled fixture/source binding. It does not grant the originalfull 48 native route or any physical artifact-deletion witness; those remain queued after source readiness.
