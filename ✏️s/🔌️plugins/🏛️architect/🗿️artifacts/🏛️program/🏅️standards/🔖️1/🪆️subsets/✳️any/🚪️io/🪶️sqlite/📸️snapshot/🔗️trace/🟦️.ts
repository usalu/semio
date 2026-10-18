/** 🔗️ Trace link rows of the Architect SQLite projection. */
import type { TraceLink } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import { type Cells, Cursor, optional, symbol, text } from "../📏️cells/🟦️.ts";
import { TraceKind } from "../🏷️symbols/🟦️.ts";

export const trace=(value:TraceLink):Cells=>[...text(value.id),...text(value.fromId),...text(value.toId),...symbol(value.kind,TraceKind),...optional(value.label??null,1,text)];
export function readTrace(c:Cursor):TraceLink{const id=c.text(),fromId=c.text(),toId=c.text(),kind=c.symbol(TraceKind),label=c.optional(1,()=>c.text());return{id,fromId,toId,kind,...label===null?{}:{label}}}
