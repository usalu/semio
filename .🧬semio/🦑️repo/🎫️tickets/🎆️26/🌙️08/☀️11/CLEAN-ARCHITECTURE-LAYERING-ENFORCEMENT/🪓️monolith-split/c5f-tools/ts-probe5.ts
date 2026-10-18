// 🧪️ C5F probe: evaluates the source-text assertions of the three re-pointed package 5 readers over the module trees (read-only).
import {join} from "node:path";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const PL = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const runtime = readModuleTreeSource(join(PL, "🧠️interpreter/🏃️runtime/🦀️.rs"));
let bad = 0;
const check = (name: string, ok: boolean) => { console.log(ok ? "ok  " : "FAIL", name); if (!ok) bad += 1; };
check("admission test: OriginalActorFixedReceipt::read", runtime.includes("OriginalActorFixedReceipt::read"));
check("host script: typed decoder", runtime.includes("result.map_err(|error| decode_guest_fault_bytes(&error))"));
const resume = runtime.slice(runtime.indexOf("pub fn resume_snapshot("), runtime.indexOf("/// 🪶️ The six owned"));
check("microsecond-clock: slice non-empty and ordered", resume.length > 0 && runtime.indexOf("pub fn resume_snapshot(") < runtime.indexOf("/// 🪶️ The six owned"));
check("microsecond-clock: deadline_us: u64", /deadline_us: u64/u.test(resume));
check("microsecond-clock: fuel, deadline_us, OwnedDeadline::Original", /fuel, deadline_us, OwnedDeadline::Original/u.test(resume));
check("microsecond-clock: no NoFuelProgress|deadline_ms|*1000", !/NoFuelProgress|deadline_ms|\*1000/u.test(resume));
console.log("resume slice chars", resume.length, "bad", bad);
