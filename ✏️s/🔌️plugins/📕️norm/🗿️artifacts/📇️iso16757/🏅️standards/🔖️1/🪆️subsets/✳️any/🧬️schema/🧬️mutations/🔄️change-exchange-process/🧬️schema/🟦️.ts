/** 🔄️ `change-exchange-process` wire twin: the leaf payload `ChangeExchangeProcess`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Iso16757ExchangeProcess, parseIso16757ExchangeProcess } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeExchangeProcess {
  newExchangeProcess: Iso16757ExchangeProcess;
}

export const parseChangeExchangeProcess: NormWireReader<ChangeExchangeProcess> = normWireObject<ChangeExchangeProcess>({ newExchangeProcess: normWireRequired(parseIso16757ExchangeProcess) });
