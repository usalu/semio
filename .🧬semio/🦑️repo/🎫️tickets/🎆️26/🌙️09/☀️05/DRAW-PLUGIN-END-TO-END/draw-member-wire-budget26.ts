import { Database } from "bun:sqlite";
import { ADMISSION_POLICY_V1, unitCeilingAlarmV1, type UnitObservationV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import law from "./draw-member-wire-budget26.json" with { type: "json" };

const [history, mode, earliest] = process.argv.slice(2);
if (!history || (mode !== "unsplit" && mode !== "split" && mode !== "hub")) throw Error("A history path and unsplit, split or hub mode are required");
const since = Number(earliest ?? 0);
if (!Number.isSafeInteger(since) || since < 0 || (mode === "hub" && since === 0)) throw Error("A positive run start in epoch milliseconds is required for hub measurements");
const policy = ADMISSION_POLICY_V1.unitCeiling;
if (policy.classes[policy.workerClass]?.libraryMegabytes !== law.maximumMegabytes) throw Error("Policy differs from the authored 16g worker library limit");
const rows = (await Bun.file(history).text()).split("\n").filter(Boolean).map(line=>JSON.parse(line) as UnitObservationV1);
const sqlite = new Database(":memory:");
try {
 const maximum = sqlite.query("select ((?1 + 1048575) / 1048576) <= ?2 as fits");
 let accepted = true;
 for (const name of law[mode]) {
  const key = `${name}@${name.replaceAll("-", "_")}|${law.target}|${law.profile}|${law.role}`;
  const measured = rows.filter(row=>row.key === key && row.atMs >= since && Number.isSafeInteger(row.peakRssBytes) && row.peakRssBytes > 0).sort((a,b)=>b.atMs-a.atMs)[0];
  if (!measured) throw Error(`No measured compiler row for ${key}`);
  const fits = unitCeilingAlarmV1(policy, measured) === null;
  const oracle = maximum.get(measured.peakRssBytes, law.maximumMegabytes) as { fits: number };
  if (fits !== !!oracle.fits) throw Error("Production ceiling and SQLite disagree");
  accepted &&= fits;
  console.log(JSON.stringify({ key, fits, peakRssBytes: measured.peakRssBytes, maximumMegabytes: law.maximumMegabytes }));
 }
 if (!accepted) process.exitCode = 1;
} finally {
 sqlite.close();
}
