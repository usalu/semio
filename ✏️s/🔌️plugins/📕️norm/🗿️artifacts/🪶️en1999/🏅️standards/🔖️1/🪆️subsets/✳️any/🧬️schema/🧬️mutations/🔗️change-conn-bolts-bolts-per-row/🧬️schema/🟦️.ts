/** 🔗️ `change-conn-bolts-bolts-per-row` wire twin: the leaf payload `ChangeConnBoltsBoltsPerRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsBoltsPerRow {
  mutation: "changeConnBoltsBoltsPerRow";
  connectionId: string;
  newBoltsBoltsPerRow: number;
}

export const parseChangeConnBoltsBoltsPerRow: NormWireReader<ChangeConnBoltsBoltsPerRow> = normWireObject<ChangeConnBoltsBoltsPerRow>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsBoltsPerRow")), connectionId: normWireRequired(normWireString), newBoltsBoltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
