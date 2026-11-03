/** 📍 `change-bar-point-layer-id` wire twin: the leaf payload `ChangeBarPointLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarPointLayerId {
  memberId: string;
  barId: string;
  newLayerId: string;
}

export const parseChangeBarPointLayerId: NormWireReader<ChangeBarPointLayerId> = normWireObject<ChangeBarPointLayerId>({ memberId: normWireRequired(normWireString), barId: normWireRequired(normWireString), newLayerId: normWireRequired(normWireString) });
