/** 🪵️ `change-pile-design-life` wire twin: the leaf payload `ChangePileDesignLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDesignLife {
  id: string;
  newDesignLife: number;
}

export const parseChangePileDesignLife: NormWireReader<ChangePileDesignLife> = normWireObject<ChangePileDesignLife>({ id: normWireRequired(normWireString), newDesignLife: normWireRequired(normWireNumber) });
