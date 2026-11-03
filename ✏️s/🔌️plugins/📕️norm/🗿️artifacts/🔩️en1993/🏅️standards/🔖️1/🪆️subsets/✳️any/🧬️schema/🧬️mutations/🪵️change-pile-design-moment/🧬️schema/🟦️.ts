/** 🪵️ `change-pile-design-moment` wire twin: the leaf payload `ChangePileDesignMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDesignMoment {
  id: string;
  newDesignMoment: number;
}

export const parseChangePileDesignMoment: NormWireReader<ChangePileDesignMoment> = normWireObject<ChangePileDesignMoment>({ id: normWireRequired(normWireString), newDesignMoment: normWireRequired(normWireNumber) });
