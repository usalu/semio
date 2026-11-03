/** 🚆 `change-derailment-y` wire twin: the leaf payload `ChangeDerailmentY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDerailmentY {
  id: string;
  newY: number;
}

export const parseChangeDerailmentY: NormWireReader<ChangeDerailmentY> = normWireObject<ChangeDerailmentY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
