#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for every subset under `✏️s/🔌️plugins/🎥️shooting`.
//
// The test platform asks an owner what production dispatch offers without knowing its language: this
// process answers `list-mutations <artifact> <standard> <subset> [<surface>]` with a RuntimeMutationInventory on
// stdout, by running the sibling Rust binary that reads the production aggregates' DESCRIPTORS; a surface names a
// state lane (config, presence, transient) of the subset's editor or viewer.
//
//   bun 📜️script.ts list-mutations <artifact> <standard> <subset> [<surface>]
//
// @see 🦀️.rs — the binary that reads the dispatch aggregates
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
//#endregion 🔌️Adapters

//#region 🚪️Entry
const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {
  const [command = "", artifact = "", standard = "", subset = "", ...surface] = argv;
  if (command !== "list-mutations") {
    console.error(`[bridge] unknown command ${JSON.stringify(command)} — expected list-mutations <artifact> <standard> <subset> [<surface>]`);
    return 2;
  }
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-shooting-mutation-bridge", "--", command, artifact, standard, subset, ...surface], {
    cwd: import.meta.dir,
    encoding: "utf8",
  });
  if (built.status !== 0) {
    console.error(`[bridge] cargo exited ${built.status}: ${(built.stderr ?? "").trim().split("\n").slice(-6).join("\n")}`);
    return 1;
  }
  process.stdout.write(built.stdout);
  return 0;
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export { BRIDGE_VERSION };
//#endregion 🚪️Entry
