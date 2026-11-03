/** 🏗️ `change-member-nominal-dimension` wire twin: the leaf payload `ChangeMemberNominalDimension`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberNominalDimension {
  mutation: "changeMemberNominalDimension";
  id: string;
  newNominalDimension: number;
}

export const parseChangeMemberNominalDimension: NormWireReader<ChangeMemberNominalDimension> = normWireObject<ChangeMemberNominalDimension>({ mutation: normWireRequired(normWireLiteral("changeMemberNominalDimension")), id: normWireRequired(normWireString), newNominalDimension: normWireRequired(normWireNumber) });
