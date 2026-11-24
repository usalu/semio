/** 🧪️ Runs the quick print pipeline source verification directly (what `repo run @semio-tech/print:test -- quick` reaches through the build graph). */
import { verifyPrintPipelineSourcesV1 } from "../../../../../../../🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts";

await verifyPrintPipelineSourcesV1();
console.log("quick pipeline verification: passed");
