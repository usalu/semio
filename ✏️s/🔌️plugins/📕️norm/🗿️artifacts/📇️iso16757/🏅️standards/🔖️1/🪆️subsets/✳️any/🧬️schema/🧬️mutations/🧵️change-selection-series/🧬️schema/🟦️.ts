/** 🧵️ `change-selection-series` wire twin: the leaf payload `ChangeSelectionSeries`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireNullable, normWireObject, type NormWireReader, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSelectionSeries {
  newSeriesId: string | null;
}

export const parseChangeSelectionSeries: NormWireReader<ChangeSelectionSeries> = normWireObject<ChangeSelectionSeries>({ newSeriesId: normWireDefault(normWireNullable(normWireString), () => null) });
