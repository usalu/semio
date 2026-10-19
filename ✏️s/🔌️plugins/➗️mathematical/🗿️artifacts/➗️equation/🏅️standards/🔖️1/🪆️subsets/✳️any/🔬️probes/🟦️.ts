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
const CRATE_BIN = "semio-equation-oracle-probe";
const KNOWN_PROBES = new Set(["csv-rows", "csv-compare", "gate-inputs", "fixtures"]);
//#endregion 🧬️Contract

//#region 🚀️Entry
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

  const args = [...rest];
  if ((probe === "fixtures" || probe === "gate-inputs") && !args.includes("--out")) {
    const fallback = probe === "fixtures" ? join(import.meta.dir, "..", "🧫️fixtures") : join(import.meta.dir, "📖️reader", "📦️packages", "🦀️rust", "target", "gate-inputs");
    args.push("--out", process.env.SEMIO_FIXTURE_OUT ?? fallback);
  }

  const run = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", CRATE_BIN, "--", probe, ...args], {
    cwd: join(import.meta.dir, "📖️reader", "📦️packages", "🦀️rust"),
    encoding: "utf8",
  });
  if (run.status !== 0) {
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
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if (true) process.exit(main(process.argv.slice(2)));
}
