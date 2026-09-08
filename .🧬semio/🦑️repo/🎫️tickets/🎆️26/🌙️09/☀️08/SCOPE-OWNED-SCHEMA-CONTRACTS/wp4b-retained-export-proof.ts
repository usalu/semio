import { readFileSync } from "node:fs";
import Ajv from "ajv";
const R = "/Users/ueli/Documents/semio/";
const ui = JSON.parse(readFileSync(R + "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json", "utf8"));
const cases: [string, string, string][] = [
  ["✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "PresentationRetainedCommandLimits", "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "ShootingRetainedCommandLimits", "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json", "Fem3dRetainedCommandLimits", "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "RemodelingRetainedCommandLimits", "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "HomeRetainedCommandLimits", "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "SpaceIndexRetainedCommandLimits", "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json", "SpacePlayRetainedCommandLimits", "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json"],
  ["✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "VcsRetainedCommandRoutes", "✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json"],
  ["✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "WiresRetainedCommandRoutes", "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json"],
  ["✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json", "ProcedureRetainedCommandRoutes", "✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json"],
];
for (const [mod, ex, fx] of cases) {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  for (const n of ["double", "float", "int32", "int64", "uint32", "uint64", "base64"]) ajv.addFormat(n, true);
  ajv.addSchema(ui);
  const m = JSON.parse(readFileSync(R + mod, "utf8"));
  ajv.addSchema(m);
  let v;
  try { v = ajv.compile({ $ref: `${m.$id}#/$defs/${ex}` }); }
  catch (e) { console.log(`COMPILE-FAIL ${ex}: ${(e as Error).message}`); continue; }
  const data = JSON.parse(readFileSync(R + fx, "utf8"));
  const ok = v(data);
  console.log(`${ok ? "PASS" : "FAIL"} ${ex}${ok ? "" : " :: " + JSON.stringify(v.errors?.slice(0, 3))}`);
}
