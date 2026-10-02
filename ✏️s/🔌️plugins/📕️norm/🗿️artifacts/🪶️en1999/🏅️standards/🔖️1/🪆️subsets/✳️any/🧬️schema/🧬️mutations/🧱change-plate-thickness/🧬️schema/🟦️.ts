/** 🧱 `change-plate-thickness` wire twin: the leaf payload `ChangePlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangePlateThickness {
  mutation: "changePlateThickness";
  sectionId: string;
  elementId: string;
  newThickness: number;
}

export const parseChangePlateThickness: NormWireReader<ChangePlateThickness> = normWireObject<ChangePlateThickness>({ mutation: normWireRequired(normWireLiteral("changePlateThickness")), sectionId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newThickness: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
