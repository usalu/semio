/** 🪵️ `change-pile-design-shear` wire twin: the leaf payload `ChangePileDesignShear`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDesignShear {
  id: string;
  newDesignShear: number;
}

export const parseChangePileDesignShear: NormWireReader<ChangePileDesignShear> = normWireObject<ChangePileDesignShear>({ id: normWireRequired(normWireString), newDesignShear: normWireRequired(normWireNumber) });
