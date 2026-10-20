import { registerBoundaryFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🛡️boundary/🟦️.ts";
import { registerLoadFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🏋️load/🟦️.ts";
import { registerMaterialFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🟦️.ts";
import { registerMeshFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🕸️mesh/🟦️.ts";
import { registerGeometryFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🧩️geometry/🟦️.ts";
import { parseFem2dCombinationPatch, parseFem2dLoadPatch, parseFem2dMaterialPatch, parseFem2dMaterialsDelta, parseFem2dNodePatch, parseFem2dRegionPatch, parseFem2dSectionPatch, parseFem2dSupportPatch } from "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";
import { parseFemSupport } from "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";

registerMaterialFieldIntent("2d", { parsePatch: parseFem2dMaterialPatch, parseDelta: parseFem2dMaterialsDelta });
registerLoadFieldIntent("2d", { parseLoad: parseFem2dLoadPatch, parseCombination: parseFem2dCombinationPatch });
registerMeshFieldIntent("2d", { parseNodePatch: parseFem2dNodePatch, parseSectionPatch: parseFem2dSectionPatch });
registerBoundaryFieldIntent("2d", { parseSupportPatch: parseFem2dSupportPatch, parseSupportRow: parseFemSupport });
registerGeometryFieldIntent("2d", { parseEntityPatch: parseFem2dRegionPatch });
