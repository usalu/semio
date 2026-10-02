/** ✂️ `change-beam-stud-spacing-m` wire twin: the leaf payload `ChangeBeamStudSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamStudSpacingM {
  index: number;
  newSpacingM: number;
}

export const parseChangeBeamStudSpacingM: NormWireReader<ChangeBeamStudSpacingM> = normWireObject<ChangeBeamStudSpacingM>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newSpacingM: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
