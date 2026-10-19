import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
import { cargoTargetDirectory } from "@semio-tech/framework/process/execution";
import { findWorkspaceRoot } from "@semio-tech/framework/process/routing";
import { currentPlatform } from "@semio-tech/framework/test/adapter";

//#endregion 🔌️Adapters

//#region 🧬️Contract
const HERE = import.meta.dir;
const ENGINE = join(HERE, "🧩️json", "📦️packages", "🦀️rust");
const TARGET = cargoTargetDirectory(findWorkspaceRoot());
const OWNER = join(HERE, "..", "..", "➗️equation");
const FIXTURES_DIR = join(OWNER, "🧪️tests/🧫️fixtures");
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
  const catalog = JSON.parse(readFileSync(CATALOG, "utf8")) as { testEvidence?: { generator?: { oracle?: string; platform?: string }; files: { path: string; sha256: string; bytes: number }[] }[] };
  const owned = (catalog.testEvidence ?? []).filter((entry) => entry.generator?.oracle === ORACLE_ID);
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
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
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
}
