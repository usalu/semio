import { test } from "vitest";
import { verifyChipFrameModel, verifyPageChromeBorders, verifyPageChromeLogo } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🟦️.ts";
import { verifyFrameJunctions } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/🟦️.ts";
import { verifyPrintMacroStaging } from "../../../../../../../🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts";
test("page chrome borders and chip glass tokens", () => { verifyPageChromeBorders(); verifyChipFrameModel(); });
test("page chrome logo", async () => { await verifyPageChromeLogo(); });
test("frame junctions", () => { verifyFrameJunctions(); });
test("macro staging", () => { verifyPrintMacroStaging(); });
