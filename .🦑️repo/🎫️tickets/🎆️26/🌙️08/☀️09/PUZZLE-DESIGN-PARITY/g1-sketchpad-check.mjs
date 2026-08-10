
import { readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
const ticket = "/Users/ueli/Documents/semio/.\ud83e\udd91\ufe0frepo/\ud83c\udfab\ufe0ftickets/\ud83c\udf86\ufe0f26/\ud83c\udf19\ufe0f08/\u2600\ufe0f09/PUZZLE-DESIGN-PARITY";
const src = readFileSync("/Users/ueli/Documents/semio/compose/client/lib/sketchpad/js/index.ts", "utf8");
const { runChecks } = await import(pathToFileURL("/Users/ueli/Documents/semio/.\ud83e\udd91\ufe0frepo/\ud83c\udfab\ufe0ftickets/\ud83c\udf86\ufe0f26/\ud83c\udf19\ufe0f08/\u2600\ufe0f09/PUZZLE-DESIGN-PARITY/g1-sketchpad-check.generated.mjs").href + "?t=" + Date.now());
const failures = runChecks(src);
const report = { ok: failures.length === 0, failures, checked: [
  "sketchpadConnectionTransformParamsFromDto u/v->x/y",
  "sketchpadPiecePuzzleAnchor Fixed/Connected",
  "sketchpadPieceAuthoredPose ignores flatPosition",
  "fixture builders Fixed-seed + anchor source contracts",
] };
writeFileSync(ticket + "/🧪g1-sketchpad-check-result.json", JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
if (!report.ok) process.exit(1);
