/** 🔘 `change-column-rebar-cover-m` wire twin: the leaf payload `ChangeColumnRebarCoverM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnRebarCoverM {
  id: string;
  newRebarCoverM: number;
}

export const parseChangeColumnRebarCoverM: NormWireReader<ChangeColumnRebarCoverM> = normWireObject<ChangeColumnRebarCoverM>({ id: normWireRequired(normWireString), newRebarCoverM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
