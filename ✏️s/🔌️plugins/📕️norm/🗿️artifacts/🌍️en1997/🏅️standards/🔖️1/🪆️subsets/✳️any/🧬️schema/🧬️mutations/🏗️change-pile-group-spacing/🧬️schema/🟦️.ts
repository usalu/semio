/** 🏗️ `change-pile-group-spacing` wire twin: the leaf payload `ChangePileGroupSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupSpacing {
  mutation: "changePileGroupSpacing";
  id: string;
  newSpacing: number;
}

export const parseChangePileGroupSpacing: NormWireReader<ChangePileGroupSpacing> = normWireObject<ChangePileGroupSpacing>({ mutation: normWireRequired(normWireLiteral("changePileGroupSpacing")), id: normWireRequired(normWireString), newSpacing: normWireRequired(normWireNumber) });
