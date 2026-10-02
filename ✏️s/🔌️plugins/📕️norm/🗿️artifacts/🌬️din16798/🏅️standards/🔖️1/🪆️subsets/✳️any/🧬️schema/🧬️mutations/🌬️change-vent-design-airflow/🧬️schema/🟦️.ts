/** 🌬️ `change-vent-design-airflow` wire twin: the leaf payload `ChangeVentDesignAirflow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeVentDesignAirflow {
  ventId: string;
  newDesignAirflowM3H: number;
}

export const parseChangeVentDesignAirflow: NormWireReader<ChangeVentDesignAirflow> = normWireObject<ChangeVentDesignAirflow>({ ventId: normWireRequired(normWireString), newDesignAirflowM3H: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
