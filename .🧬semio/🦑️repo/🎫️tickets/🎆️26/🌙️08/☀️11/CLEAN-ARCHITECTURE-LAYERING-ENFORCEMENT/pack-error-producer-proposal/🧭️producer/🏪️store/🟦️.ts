/** 🏪️ Higher owners keep the actual source fault and this reservation's separate charge. */
import {type PackError} from "../../🟦️.ts";

export type TypedPackAllocationFault=Readonly<{allocatedBytes:number;cause:PackError}>;

/** 🧾️ Moves the original cause without reclassifying context or replacing its witness. */
export function typedPackAllocationFault(allocatedBytes:number,cause:PackError):TypedPackAllocationFault{return{allocatedBytes,cause};}
