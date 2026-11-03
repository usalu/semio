/** 🔗️ `change-conn-welds-beta-w` wire twin: the leaf payload `ChangeConnWeldsBetaW`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnWeldsBetaW {
  mutation: "changeConnWeldsBetaW";
  connectionId: string;
  newWeldsBetaW: number;
}

export const parseChangeConnWeldsBetaW: NormWireReader<ChangeConnWeldsBetaW> = normWireObject<ChangeConnWeldsBetaW>({ mutation: normWireRequired(normWireLiteral("changeConnWeldsBetaW")), connectionId: normWireRequired(normWireString), newWeldsBetaW: normWireRequired(normWireNumber) });
