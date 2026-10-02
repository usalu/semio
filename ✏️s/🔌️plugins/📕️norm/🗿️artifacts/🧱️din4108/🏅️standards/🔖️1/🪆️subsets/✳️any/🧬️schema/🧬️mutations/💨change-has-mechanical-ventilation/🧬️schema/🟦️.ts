/** 💨 `change-has-mechanical-ventilation` wire twin: the leaf payload `ChangeHasMechanicalVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeHasMechanicalVentilation {
  newHasMechanicalVentilation: boolean;
}

export const parseChangeHasMechanicalVentilation: NormWireReader<ChangeHasMechanicalVentilation> = normWireObject<ChangeHasMechanicalVentilation>({ newHasMechanicalVentilation: normWireRequired(normWireBoolean) });
