#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External carrier probe for `s.mathematical.mathematical@1/✳️any`.
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
// THE CARRIER DECIDES WHAT IS CHECKABLE. This subset's own `MathematicalIntoCsv` serializer writes
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
// @see 🦀️oracle-probe/🦀️.rs — the reader itself
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🔬️probes/📜️script.ts
//      — the spawn/offline/agent-scoped-target pattern this file mirrors

//#endregion 🧲️Header

//#region 🔌️Adapters
import { spawnSync } from "node:child_process";
import { join } from "node:path";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

const ENGINE = { family: "burntsushi-csv", implementation: "rust-csv reader", version: "1.4.0" } as const;
const PROBE_VERSION = "csv@1.4.0";
const CRATE_BIN = "semio-mathematical-oracle-probe";
const KNOWN_PROBES = new Set(["csv-rows", "csv-compare", "gate-inputs", "fixtures"]);
//#endregion 🧬️Contract

//#region 🚀️Entry
/** 🏭️ `--offline` and an agent-scoped target directory: probes run inside a test sweep alongside peer
 *  sessions, and a shared cargo target directory is the single biggest source of lock contention. */
function cargoTargetDir(): string {
  return process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "🦀️oracle-probe", "target"), "oracle-probe");
}

function emit(report: ProbeReport): number {
  process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  return report.status === "failed" ? 1 : 0;
}

function main(argv: readonly string[]): number {
  const [probe = "", ...rest] = argv;
  const started = Date.now();
  const envelope = (status: ProbeReport["status"], measurements: Record<string, unknown>, diagnostics?: ProbeReport["diagnostics"]): ProbeReport => ({
    schema: "semio.repository-test.probe-report/v2",
    probe,
    probeVersion: PROBE_VERSION,
    engine: ENGINE,
    status,
    durationMs: Date.now() - started,
    measurements,
    ...(diagnostics ? { diagnostics } : {}),
  });

  if (!KNOWN_PROBES.has(probe)) {
    return emit(envelope("failed", {}, [{ severity: "error", message: `unknown probe ${probe || "(none)"}`, detail: `known: ${[...KNOWN_PROBES].join(", ")}` }]));
  }

  // 📌️ `--out` resolves `SEMIO_FIXTURE_OUT` first (the harness's own `fixture reproduce` sets this to
  // a scratch root so reproduction never overwrites the committed fixture — see
  // `🧰️framework/…/🧪️tests/📜️cript.ts`'s `reproduce` case), else an explicit `--out`, else a default
  // relative to THIS SCRIPT'S OWN directory, never to the process's cwd — a recorded
  // `generator.command` is replayed by the harness from an unspecified cwd, and a relative
  // `--out ../🧫️fixtures` resolved against that cwd silently lands nowhere (mirrors
  // `…✳️document/🏭️generator/📜️script.ts`'s own `SEMIO_FIXTURE_OUT ?? --out ?? join(import.meta.dir, "..", "🧫️fixtures")` order).
  const args = [...rest];
  if ((probe === "fixtures" || probe === "gate-inputs") && !args.includes("--out")) {
    const fallback = probe === "fixtures" ? join(import.meta.dir, "..", "🧫️fixtures") : join(import.meta.dir, "🦀️oracle-probe", "target", "gate-inputs");
    args.push("--out", process.env.SEMIO_FIXTURE_OUT ?? fallback);
  }

  const run = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", CRATE_BIN, "--", probe, ...args], {
    cwd: join(import.meta.dir, "🦀️oracle-probe"),
    encoding: "utf8",
    env: { ...process.env, CARGO_TARGET_DIR: cargoTargetDir() },
  });
  if (run.status !== 0) {
    // 🚫️A reader that cannot run must SAY SO and exit non-zero. Emitting a plausible-looking empty
    // measurement would let the pipeline compare nothing and call it agreement.
    return emit(envelope("failed", {}, [{ severity: "error", message: `${CRATE_BIN} exited ${run.status}`, detail: (run.stderr ?? "").trim().split("\n").slice(-6).join("\n") }]));
  }
  let parsed: { status: ProbeReport["status"]; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };
  try {
    parsed = JSON.parse((run.stdout ?? "").trim().split("\n").at(-1) ?? "") as typeof parsed;
  } catch (error) {
    return emit(envelope("failed", {}, [{ severity: "error", message: "probe output was not JSON", detail: String((error as Error).message ?? error) }]));
  }
  return emit(envelope(parsed.status, parsed.measurements, parsed.diagnostics));
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
//#endregion 🚀️Entry
