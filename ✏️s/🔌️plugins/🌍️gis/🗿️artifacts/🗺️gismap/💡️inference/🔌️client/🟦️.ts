import { DOCUMENT_SERVICE_TOPIC_V1, admitDocumentServiceDeclarationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🟦️.ts";
import submitInput from "./🧬️schema/submit-input.json" with { type: "json" };
import submitOutput from "./🧬️schema/submit-output.json" with { type: "json" };
import eventsInput from "./🧬️schema/events-input.json" with { type: "json" };
import cancelInput from "./🧬️schema/cancel-input.json" with { type: "json" };
import pageOutput from "./🧬️schema/page-output.json" with { type: "json" };
import approveInput from "./🧬️schema/approve-input.json" with { type: "json" };
import approveOutput from "./🧬️schema/approve-output.json" with { type: "json" };
import reconcileInput from "./🧬️schema/reconcile-input.json" with { type: "json" };
import reconcileOutput from "./🧬️schema/reconcile-output.json" with { type: "json" };
import undoInput from "./🧬️schema/undo-input.json" with { type: "json" };
import undoOutput from "./🧬️schema/undo-output.json" with { type: "json" };

/** 📜 GIS owns the complete finite route and schema declaration shared with its native descriptor producer. */
export function gisMapDocumentServiceDeclarationV1() {
  const operation = (action: string, method: "GET" | "POST", route: readonly string[], sendBody: boolean, cursorField: string | null, input: unknown, output: unknown) => ({ action, method, route, sendBody, cursorField, requestMaxBytes: 1024, responseMaxBytes: 16384, inputSchema: input, outputSchema: output });
  return admitDocumentServiceDeclarationV1("gis", { schema: DOCUMENT_SERVICE_TOPIC_V1, owner: "gis", serviceId: "s.gis.gismap.inference", operations: [
    operation("submit", "POST", ["inference", "gis-map", "jobs"], true, null, submitInput, submitOutput),
    operation("events", "GET", ["inference", "gis-map", "jobs", "{jobId}", "events"], false, "after", eventsInput, pageOutput),
    operation("cancel", "POST", ["inference", "gis-map", "jobs", "{jobId}", "cancel"], false, null, cancelInput, pageOutput),
    operation("approve", "POST", ["inference", "gis-map", "jobs", "{jobId}", "approval"], true, null, approveInput, approveOutput),
    operation("reconcile", "POST", ["inference", "gis-map", "jobs", "reconcile"], true, null, reconcileInput, reconcileOutput),
    operation("undo", "POST", ["inference", "gis-map", "approval-undos"], true, null, undoInput, undoOutput),
  ] });
}
