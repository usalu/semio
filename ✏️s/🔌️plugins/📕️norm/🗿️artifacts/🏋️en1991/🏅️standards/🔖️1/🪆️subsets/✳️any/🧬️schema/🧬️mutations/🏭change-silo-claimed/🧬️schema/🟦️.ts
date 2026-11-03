/** 🏭 `change-silo-claimed` wire twin: the leaf payload `ChangeSiloClaimed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloClaimed {
  newSiloClaimed: boolean;
}

export const parseChangeSiloClaimed: NormWireReader<ChangeSiloClaimed> = normWireObject<ChangeSiloClaimed>({ newSiloClaimed: normWireRequired(normWireBoolean) });
