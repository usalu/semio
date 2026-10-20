import { registerBoundaryFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🛡️boundary/🟦️.ts";
import { registerLoadFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🏋️load/🟦️.ts";
import { registerMaterialFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🟦️.ts";
import { registerMeshFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🕸️mesh/🟦️.ts";
import { registerGeometryFieldIntent } from "../../../../🧪️tests/🧱️field-intent/🧩️geometry/🟦️.ts";
import { parseFem3dCombinationPatch, parseFem3dLoadPatch, parseFem3dMaterialPatch, parseFem3dMaterialsDelta, parseFem3dNodePatch, parseFem3dSolidPatch, parseFem3dSectionPatch, parseFem3dSupportPatch } from "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";
import { parseFemSupport } from "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";

registerMaterialFieldIntent("3d", { parsePatch: parseFem3dMaterialPatch, parseDelta: parseFem3dMaterialsDelta });
registerLoadFieldIntent("3d", { parseLoad: parseFem3dLoadPatch, parseCombination: parseFem3dCombinationPatch });
registerMeshFieldIntent("3d", { parseNodePatch: parseFem3dNodePatch, parseSectionPatch: parseFem3dSectionPatch });
registerBoundaryFieldIntent("3d", { parseSupportPatch: parseFem3dSupportPatch, parseSupportRow: parseFemSupport });
registerGeometryFieldIntent("3d", { parseEntityPatch: parseFem3dSolidPatch });
