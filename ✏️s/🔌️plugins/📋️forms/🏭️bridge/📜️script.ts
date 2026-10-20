#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for every subset under `✏️s/🔌️plugins/📋️forms`.
//
// The test platform asks an owner what production dispatch offers without knowing its language: this
// process answers `list-mutations <artifact> <standard> <subset> [<surface>]` with a RuntimeMutationInventory on
// stdout, by running the sibling Rust binary that reads the production aggregates' DESCRIPTORS; a surface names a
// state lane (config, presence, transient) of the subset's editor or viewer.
//
//   bun 📜️script.ts list-mutations <artifact> <standard> <subset> [<surface>]
//
// @see 🦀️.rs — the binary that reads the dispatch aggregates
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import { runRepositoryMutationBridgeScriptV1 } from "../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🚪️Entry
/** 🚪️ `list-mutations` — delivers the production mutation inventory of this owner through its bridge binary. */
class ListMutationsScript extends BundleScript {
  async run(): Promise<void> {
    await runRepositoryMutationBridgeScriptV1(this.root, "semio-forms-mutation-bridge");
  }
}
const router = new ScriptRouter(import.meta.dir).register("list-mutations", ListMutationsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
//#endregion 🚪️Entry
