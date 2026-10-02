/** 🔥️ `change-weld-throat` wire twin: the leaf payload `ChangeWeldThroat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWeldThroat {
  mutation: "changeWeldThroat";
  connectionId: string;
  newThroat: number;
}

export const parseChangeWeldThroat: NormWireReader<ChangeWeldThroat> = normWireObject<ChangeWeldThroat>({ mutation: normWireRequired(normWireLiteral("changeWeldThroat")), connectionId: normWireRequired(normWireString), newThroat: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
