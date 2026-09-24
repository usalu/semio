#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for the complete `{graph, geometry, equation}` equation JSON
// carrier. Its standalone Rust workspace links only `json` (json-rust), never production mutation
// code and never `serde_json`, which the production plugin itself links.
//
//   bun 📜️script.ts generate [--out <dir>]   # writes the reviewed fixture pairs
//   bun 📜️script.ts manifests                 # refreshes digests and provenance of this oracle's fixture manifests
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️remaining-sixty-anatomy.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { cargoTargetDirectory } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { currentPlatform } from "../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE = join(HERE, "🧩️json", "📦️packages", "🦀️rust");
const TARGET = cargoTargetDirectory(getWorkspaceRoot());
const OWNER = join(HERE, "..", "..", "➗️equation");
const FIXTURES_DIR = join(OWNER, "🧫️fixtures");
const CATALOG = join(OWNER, "🔮️oracles", "🔣️.json");
const ORACLE_ID = "json-rust-equation-carrier-reader";
const GENERATOR = { oracle: ORACLE_ID, packageVersion: "0.12", engineFamily: "json-rust", engineVersion: "0.12", command: "bun ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate" } as const;
//#endregion 🧬️Contract

//#region 🔨️Build
function build(): void {
  const result = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(ENGINE, "Cargo.toml")], { stdio: "inherit" });
  if (result.status !== 0) throw new Error(`cargo build failed with status ${result.status}`);
}

async function sha256(path: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", readFileSync(path));
  return `sha256:${[...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}
//#endregion 🔨️Build

//#region 🚪️Commands
function generate(outRoot: string): number {
  build();
  const result = spawnSync(join(TARGET, "release", "generate"), [outRoot], { stdio: "inherit" });
  return result.status ?? 1;
}

async function manifests(): Promise<void> {
  const catalog = JSON.parse(readFileSync(CATALOG, "utf8")) as { fixtureManifests?: { generator?: { oracle?: string }; files: { path: string; sha256: string; bytes: number }[] }[] };
  const owned = (catalog.fixtureManifests ?? []).filter((entry) => entry.generator?.oracle === ORACLE_ID);
  if (owned.length === 0) throw new Error(`no fixture manifest names ${ORACLE_ID}`);
  for (const entry of owned) {
    for (const file of entry.files) {
      const path = join(dirname(CATALOG), file.path);
      file.sha256 = await sha256(path);
      file.bytes = readFileSync(path).length;
    }
    entry.generator = { ...GENERATOR, platform: currentPlatform() };
  }
  writeFileSync(CATALOG, `${JSON.stringify(catalog, null, 2)}\n`);
  console.log(`${owned.length} fixture manifest(s) refreshed in ${CATALOG}`);
}
//#endregion 🚪️Commands

//#region 🚀️Entry
const [command, ...rest] = process.argv.slice(2);
const outFlagIndex = rest.indexOf("--out");
const outRoot = outFlagIndex >= 0 ? rest[outFlagIndex + 1]! : (process.env.SEMIO_FIXTURE_OUT ?? FIXTURES_DIR);
if (command === "generate") process.exit(generate(outRoot));
else if (command === "manifests") await manifests();
else {
  console.error("usage: bun 📜️script.ts <generate [--out <dir>]|manifests>");
  process.exit(2);
}
//#endregion 🚀️Entry
