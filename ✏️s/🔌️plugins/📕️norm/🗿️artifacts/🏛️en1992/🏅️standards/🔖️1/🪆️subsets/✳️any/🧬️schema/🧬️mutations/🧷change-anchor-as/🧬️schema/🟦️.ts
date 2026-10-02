/** 🧷 `change-anchor-as` wire twin: the leaf payload `ChangeAnchorAs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAnchorAs {
  anchorId: string;
  newValue: number;
}

export const parseChangeAnchorAs: NormWireReader<ChangeAnchorAs> = normWireObject<ChangeAnchorAs>({ anchorId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
