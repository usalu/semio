/** 🦴️ `change-fire-insulation-fire-id` wire twin: the leaf payload `ChangeFireInsulationFireId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireInsulationFireId {
  mutation: "changeFireInsulationFireId";
  fireInsulationId: string;
  newFireId: string;
}

export const parseChangeFireInsulationFireId: NormWireReader<ChangeFireInsulationFireId> = normWireObject<ChangeFireInsulationFireId>({ mutation: normWireRequired(normWireLiteral("changeFireInsulationFireId")), fireInsulationId: normWireRequired(normWireString), newFireId: normWireRequired(normWireString) });
