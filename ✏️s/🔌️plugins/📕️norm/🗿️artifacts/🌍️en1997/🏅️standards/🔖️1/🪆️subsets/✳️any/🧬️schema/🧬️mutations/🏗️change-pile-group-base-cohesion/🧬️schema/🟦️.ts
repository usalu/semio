/** 🏗️ `change-pile-group-base-cohesion` wire twin: the leaf payload `ChangePileGroupBaseCohesion`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupBaseCohesion {
  mutation: "changePileGroupBaseCohesion";
  id: string;
  newBaseCohesion: number;
}

export const parseChangePileGroupBaseCohesion: NormWireReader<ChangePileGroupBaseCohesion> = normWireObject<ChangePileGroupBaseCohesion>({ mutation: normWireRequired(normWireLiteral("changePileGroupBaseCohesion")), id: normWireRequired(normWireString), newBaseCohesion: normWireRequired(normWireNumber) });
