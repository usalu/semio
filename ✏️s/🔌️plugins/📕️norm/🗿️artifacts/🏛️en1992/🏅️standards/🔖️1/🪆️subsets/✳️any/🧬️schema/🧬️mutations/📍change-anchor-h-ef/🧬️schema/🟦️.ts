/** 📍 `change-anchor-h-ef` wire twin: the leaf payload `ChangeAnchorHEf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAnchorHEf {
  anchorId: string;
  newValue: number;
}

export const parseChangeAnchorHEf: NormWireReader<ChangeAnchorHEf> = normWireObject<ChangeAnchorHEf>({ anchorId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
