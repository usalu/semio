/** 🔗️ `change-conn-bolts-edge-distance` wire twin: the leaf payload `ChangeConnBoltsEdgeDistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnBoltsEdgeDistance {
  mutation: "changeConnBoltsEdgeDistance";
  connectionId: string;
  newBoltsEdgeDistance: number;
}

export const parseChangeConnBoltsEdgeDistance: NormWireReader<ChangeConnBoltsEdgeDistance> = normWireObject<ChangeConnBoltsEdgeDistance>({ mutation: normWireRequired(normWireLiteral("changeConnBoltsEdgeDistance")), connectionId: normWireRequired(normWireString), newBoltsEdgeDistance: normWireRequired(normWireNumber) });
