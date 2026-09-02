#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party CSV and JSON fixture generator for `s.sequence.sequence@1/✳️any`.
//
// This subset's csv export is REAL — one record per step, `[id, kind, JSON-encoded params]`,
// `has_header: false` — which is what makes `csv-rfc4180-reader` a qualifying oracle for exactly the
// four ROW-LEVEL kinds. Every fixture is an authored `(before.csv, after.csv)` PAIR: the pair IS the
// expectation, the `csv` crate writes it and the reader re-derives the row set from the bytes.
//
// The JSON engine covers the four fields the lossy CSV carrier omits. Each engine is a standalone
// workspace with exactly one third-party dependency.
//
// Generation and execution are SEPARATE — a normal test run must never be able to rewrite the
// expectation it is measured against.
//
//   bun 📜️script.ts generate                      # build both engines and write the CSV/JSON fixture pairs
//   bun 📜️script.ts manifests                     # (re)write 🧫️fixtures/🔣️.json
//
// @see ../../../../../🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🏭️generator/📜️script.ts
//      — the carrier generator this file mirrors in CLI shape and manifest fields.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧭️Paths
const here = dirname(Bun.fileURLToPath(import.meta.url));
const subset = join(here, "..");
const fixtures = join(subset, "🧫️fixtures");
const csvEngine = join(here, "🦀️csv-engine");
const jsonEngine = join(here, "🦀️json-engine");
const target = process.env.CARGO_TARGET_DIR;
const CSV_KINDS = ["create-step", "delete-step", "duplicate-step", "edit-step-params"] as const;
const JSON_KINDS = ["change-step-collapsed", "connect-steps", "disconnect-steps", "move-step"] as const;
//#endregion 🧭️Paths

//#region 🏭️Generate
const generate = (): void => {
  for (const engine of [csvEngine, jsonEngine]) {
    const targetDir = target ?? join(engine, "target");
    const built = spawnSync("cargo", ["build", "--release", "--offline", "--target-dir", targetDir], { cwd: engine, stdio: "inherit" });
    if (built.status !== 0) throw new Error(`${engine} build failed`);
    const run = spawnSync(join(targetDir, "release", "generate"), [fixtures], { stdio: "inherit" });
    if (run.status !== 0) throw new Error(`${engine} fixture generation failed`);
  }
};
//#endregion 🏭️Generate

//#region 🧾️Manifests
const digest = (path: string): string => `sha256:${createHash("sha256").update(readFileSync(path)).digest("hex")}`;

const manifests = (): void => {
  const csvEntries = CSV_KINDS.flatMap((kind) => {
    const dir = join(fixtures, kind);
    if (!existsSync(dir)) return [];
    return [{
      schema: "semio.repository-test.fixture/v2",
      id: `sequence-csv-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.sequence.sequence", standard: "1", subset: "any" },
      mutation: kind,
      outcome: "applied",
      // 📐️A CSV grid carries no geometry; the contract still requires the block to be declared.
      units: { length: "none", angle: "none", handedness: "none", up: "none" },
      files: ["before.csv", "after.csv"].map((name) => ({
        role: name === "before.csv" ? "input-csv" : "expected-csv",
        path: `../🧫️fixtures/${kind}/${name}`,
        mediaType: "text/csv",
        sha256: digest(join(dir, name)),
        bytes: statSync(join(dir, name)).size,
      })),
      generator: {
        oracle: "csv-rfc4180-reader",
        packageVersion: "1.4.0",
        engineFamily: "csv",
        engineVersion: "1.4.0",
        command: `bun ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate`,
        platform: `${process.platform}-${process.arch}`,
      },
      provenance: {
        source: "generated",
        license: "Unlicense OR MIT (csv)",
        attribution: "Serialized with the csv crate (Unlicense OR MIT); no semio code participates in the expectation",
        security: "scanned-clean",
        privacy: "no-personal-data",
      },
      comparisonProfile: "unordered-json-v1",
      reproducible: true,
      family: "sequence-csv-carrier",
      notes: `Authored (before, after) CSV pair for \`${kind}\`. The pair is the expectation; the third-party csv reader re-derives the row set.`,
    }];
  });
  const jsonEntries = JSON_KINDS.flatMap((kind) => {
    const dir = join(fixtures, kind);
    if (!existsSync(dir)) return [];
    return [{
      schema: "semio.repository-test.fixture/v2",
      id: `sequence-json-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.sequence.sequence", standard: "1", subset: "any" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "radian", handedness: "none", up: "none" },
      files: ["before.json", "after.json"].map((name) => ({
        role: name === "before.json" ? "expected-before-json" : "expected-after-json",
        path: `../🧫️fixtures/${kind}/${name}`,
        mediaType: "application/json",
        sha256: digest(join(dir, name)),
        bytes: statSync(join(dir, name)).size,
      })),
      generator: {
        oracle: "serde-json-sequence-carrier-reader",
        packageVersion: "1",
        engineFamily: "serde-json",
        engineVersion: "1",
        command: `bun ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate`,
        platform: `${process.platform}-${process.arch}`,
      },
      provenance: {
        source: "generated",
        license: "MIT OR Apache-2.0 (serde_json)",
        attribution: "Serialized with serde_json; no semio mutation code participates in the expectation",
        security: "scanned-clean",
        privacy: "no-personal-data",
      },
      comparisonProfile: "semantic-sequence-carrier-v1",
      reproducible: true,
      family: "sequence-json-carrier",
      notes: `Complete {schema, steps, edges} JSON carrier pair for ${kind}; the standalone serde_json generator asserts the semantic projections differ.`,
    }];
  });
  const entries = [...csvEntries, ...jsonEntries];
  writeFileSync(join(fixtures, "🔣️.json"), `${JSON.stringify(entries, null, 2)}\n`);
  // 🧾️The REGISTRY reads `fixtureManifests` off the contribution file itself (`loadOracleRegistry`
  // parses it there, not from `🧫️fixtures/🔣️.json`), so the generated block is merged into
  // the catalog too — the standalone file stays as the generator's reviewable output.
  const catalogPath = join(subset, "🧪️oracle", "🔣️.json");
  const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));
  const keep = (catalog.fixtureManifests ?? []).filter((entry: { family?: string }) => entry.family !== "sequence-csv-carrier" && entry.family !== "sequence-json-carrier");
  catalog.fixtureManifests = [...keep, ...entries];
  writeFileSync(catalogPath, `${JSON.stringify(catalog, null, 2)}\n`);
  console.log(`${entries.length} fixture manifest(s) written and registered in 🔣️oracle.json`);
};
//#endregion 🧾️Manifests

//#region 🚀️Main
const command = process.argv[2];
if (command === "generate") generate();
else if (command === "manifests") manifests();
else { console.error("usage: 📜️script.ts <generate|manifests>"); process.exit(2); }
//#endregion 🚀️Main
