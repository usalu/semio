#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for the one `s.mathematical.mathematical@1/✳️any` kind that is a
// fact about this subset's own JSON carrier: `change-coefficient`.
//
// This subset's state splits three ways, and the split is exactly why its coverage looks uneven:
//
//   * `equation` is INLINE in the snapshot, so it reaches the JSON export intact — `change-coefficient`
//     edits a labelled node of that tree and is witnessable here.
//   * The graph's NODES reach the CSV export (`id,label,x,y`) — which is why the sibling
//     `csv-rfc4180-mathematical-1-mutate` oracle covers exactly the five node kinds.
//   * EDGES, the `directed` flag, the `algorithm` and the geometry's POINTS reach NO carrier at all.
//     `MathematicalIntoCsv` is `IoFidelity::Lossy` and emits only nodes; the JSON export carries the
//     composed children as `ArtifactChild` handles whose `local_owner` is `#[serde(skip)]`.
//
// The nine kinds over that third group are blocked on EXPORT, not on oracles. A reader cannot witness
// what no carrier records.
//
//   bun 📜️script.ts generate [--out <dir>]   # writes the fixture pair
//   bun 📜️script.ts manifests                 # prints the fixtureManifests entry
//
// @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️remaining-sixty-anatomy.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE = join(HERE, "🦀️json-engine");
const FIXTURES_DIR = join(HERE, "..", "🧫️fixtures");
const ORACLE_ID = "serde-json-mathematical-carrier-reader";
const COMPARISON_PROFILE = "semantic-mathematical-carrier-v1";
const KINDS: readonly string[] = ["change-coefficient"];
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
  const result = spawnSync(join(ENGINE, "target", "release", "generate"), [outRoot], { stdio: "inherit" });
  return result.status ?? 1;
}

async function manifests(): Promise<void> {
  const entries = [];
  for (const kind of KINDS) {
    const dir = join(FIXTURES_DIR, kind);
    if (!existsSync(dir)) throw new Error(`missing fixture directory for ${kind} — run generate first`);
    const files = [];
    for (const [role, name] of [["expected-before-json", "before.json"], ["expected-after-json", "after.json"]] as const) {
      const path = join(dir, name);
      files.push({ role, path: `../🧫️fixtures/${kind}/${name}`, mediaType: "application/json", sha256: await sha256(path), bytes: readFileSync(path).length });
    }
    entries.push({
      schema: "semio.repository-test.fixture/v2",
      id: `carrier-${kind}`,
      class: "third-party-generated",
      target: { artifact: "s.mathematical.mathematical", standard: "1", subset: "any" },
      mutation: kind,
      outcome: "applied",
      units: { length: "unitless", angle: "radian" },
      files,
      provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
      generator: { oracle: ORACLE_ID, packageVersion: "1", engineFamily: "serde-json", engineVersion: "1", command: "bun ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts generate", platform: process.platform },
      comparisonProfile: COMPARISON_PROFILE,
      reproducible: true,
      family: "mechanical",
      notes: `A deterministic snapshot whose inline equation is 3/4·x + 2, with ${kind} rewriting the labelled rational coefficient to 7/5 as an edit to the JSON CARRIER, read back through serde_json — never through this repository's own mutation engine. The coefficient is a Rational lexeme pair, never an f64, matching EquationNodeKind::Rational's own round-trip-exact representation. Observability is checked before the pair is written.`,
    });
  }
  console.log(JSON.stringify(entries, null, 2));
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
