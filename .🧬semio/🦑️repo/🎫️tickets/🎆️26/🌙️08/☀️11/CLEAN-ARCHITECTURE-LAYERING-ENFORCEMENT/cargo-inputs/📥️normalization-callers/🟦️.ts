import policy from "./🧫️fixtures/🔣️.json";
import {CargoCliOperationOwner} from "../../🗂️workspaces/🦀️cargo/🟦️.ts";

/** 🎛️ Authors the finite operation owned by one normalization law and its physical fixture. */
export function normalizationProbeOwner():CargoCliOperationOwner{return new CargoCliOperationOwner({...policy,schemaVersion:1},{publish:event=>console.log("[DEBUG] "+JSON.stringify({normalizationProbe:event})),yieldContinuation:()=>new Promise(resolve=>setImmediate(resolve))});}
