/** 🔥️ `change-weld-throat` wire twin: the leaf payload `ChangeWeldThroat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldThroat {
  mutation: "changeWeldThroat";
  connectionId: string;
  newThroat: number;
}

export const parseChangeWeldThroat: NormWireReader<ChangeWeldThroat> = normWireObject<ChangeWeldThroat>({ mutation: normWireRequired(normWireLiteral("changeWeldThroat")), connectionId: normWireRequired(normWireString), newThroat: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
