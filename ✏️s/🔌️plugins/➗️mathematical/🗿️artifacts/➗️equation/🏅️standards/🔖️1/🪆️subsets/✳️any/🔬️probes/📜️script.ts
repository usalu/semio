#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External carrier probe for `s.mathematical.equation@1/✳️any`.
//
// Everything here MARSHALS and INVOKES; nothing here reads a carrier or computes a number. Every
// measurement comes out of the `csv` crate in the sibling `🦀️oracle-probe` crate. This script only
// stamps the ProbeReport envelope around what it reported.
//
// WHY A RUST CHILD PROCESS RATHER THAN AN INLINE JS PARSER. The approved `test-oracle` reader for
// this carrier is a Rust crate (`🔒️dependencies.json`: `csv 1`, already registered for the `sequence`
// subset's own `csv-rfc4180-reader`). No vendored JS CSV library (`papaparse`, `csv-parse`, `d3-dsv`)
// is an approved oracle package here, so reading this carrier with one would require a new,
// unapproved dependency. The child crate declares its own `[workspace]` and links no repository
// crate at all, so it builds and runs independently of the plugin (which does not currently compile).
//
// THE CARRIER DECIDES WHAT IS CHECKABLE. This subset's own `EquationIntoCsv` serializer writes
// one row per node — `id,label,x,y` — and nothing else: no `edges`, no `directed`/`algorithm`, no
// point-cloud geometry, no `equation` AST. A probe handed anything but a `.csv` returns
// `unsupported`, never an empty `ok`.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts csv-rows    --input <a.csv>
//   bun 📜️script.ts csv-compare --input <expected.csv> --input <actual.csv>
//   bun 📜️script.ts gate-inputs --out <dir>
//   bun 📜️script.ts fixtures    --out <dir>
//
// @see 📖️reader/🦀️.rs — the reader itself
// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts
//      — the spawn/offline/agent-scoped-target pattern this file mirrors

//#endregion 🧲️Header

//#region 🔌️Adapters
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `run` — runs the program of this script. */
class RunScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("run", RunScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["run"] }));
