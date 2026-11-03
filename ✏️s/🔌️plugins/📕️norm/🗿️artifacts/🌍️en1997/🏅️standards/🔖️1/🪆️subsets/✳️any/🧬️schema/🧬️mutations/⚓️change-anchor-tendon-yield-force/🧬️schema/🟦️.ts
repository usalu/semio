/** ⚓️ `change-anchor-tendon-yield-force` wire twin: the leaf payload `ChangeAnchorTendonYieldForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorTendonYieldForce {
  mutation: "changeAnchorTendonYieldForce";
  id: string;
  newTendonYieldForce: number;
}

export const parseChangeAnchorTendonYieldForce: NormWireReader<ChangeAnchorTendonYieldForce> = normWireObject<ChangeAnchorTendonYieldForce>({ mutation: normWireRequired(normWireLiteral("changeAnchorTendonYieldForce")), id: normWireRequired(normWireString), newTendonYieldForce: normWireRequired(normWireNumber) });
