/** 🥶️ `change-cf-purlin-span` wire twin: the leaf payload `ChangeCfPurlinSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfPurlinSpan {
  id: string;
  newPurlinSpan: number;
}

export const parseChangeCfPurlinSpan: NormWireReader<ChangeCfPurlinSpan> = normWireObject<ChangeCfPurlinSpan>({ id: normWireRequired(normWireString), newPurlinSpan: normWireRequired(normWireNumber) });
