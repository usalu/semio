import {Vfs} from "../tax-stage-tools/vfs.ts";
import {planLifts} from "../tax-stage-tools/plans/lifts.ts";
const rows: Record<string, unknown>[] = [];
planLifts(new Vfs(), (row) => rows.push(row));
for (const row of rows) console.log(row.result === "lifted" ? "lifted" : `FAILED ${row.result}`, String(row.script).slice(-90));
