/** ✅️ Chart validity is inferred with the framework's owned schema validator. */
import resultDocument from "../🔣️.json";
import document from "../../🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
export type VizChartDiagnostic = { readonly code: string; readonly path: string; readonly message: string };
export function validateVizChartSpecification(chart: unknown): readonly VizChartDiagnostic[] {
  try { return validateJsonSchemaSubset(document.$defs.ChartSpecification, chart, document).map((message) => ({ code: "print.chart.schema", path: "chart", message })); }
  catch (error) { return [{ code: "print.chart.schema", path: "chart", message: error instanceof Error ? error.message : String(error) }]; }
}

/** 🪪️ Admits canonical wire output against every owned inference result variant. */
export function validateVizChartInference(value: unknown): readonly VizChartDiagnostic[] {
  try { const wire = JSON.parse(JSON.stringify(value)); return validateJsonSchemaSubset(resultDocument, wire).map(message => ({ code: "print.chart.inference-schema", path: "inference", message })); }
  catch (error) { return [{ code: "print.chart.inference-schema", path: "inference", message: error instanceof Error ? error.message : String(error) }]; }
}
