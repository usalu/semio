import { libraryBootstrap, prepareSourceAnalysesV1 } from "../../../🟨️.mjs";
import { analyzeSourceRecordV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";

/** 🔌️ Opens one worker session: the same parsers as the parent. */
export async function open() { await libraryBootstrap; return prepareSourceAnalysesV1().module.revision; }

/** ▶️ Analyses a batch of source files and answers the parser record of each; a file that cannot be read has no record and is left to the parent. */
export async function run(job) {
  const provider = prepareSourceAnalysesV1().module, records = [];
  for (const path of job.files) { let record; try { record = analyzeSourceRecordV1(provider, path); } catch { continue; } records.push({ key: record.key, bytes: record.bytes }); }
  return { records };
}
