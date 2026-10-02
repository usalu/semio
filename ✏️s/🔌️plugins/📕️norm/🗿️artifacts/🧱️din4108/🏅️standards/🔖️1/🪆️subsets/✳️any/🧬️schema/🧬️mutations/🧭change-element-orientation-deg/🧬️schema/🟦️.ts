/** 🧭 `change-element-orientation-deg` wire twin: the leaf payload `ChangeElementOrientationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementOrientationDeg {
  elementId: string;
  newOrientationDeg: number;
}

export const parseChangeElementOrientationDeg: NormWireReader<ChangeElementOrientationDeg> = normWireObject<ChangeElementOrientationDeg>({ elementId: normWireRequired(normWireString), newOrientationDeg: normWireRequired(normWireNumber) });
