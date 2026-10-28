/** 🪡️ Exchanges supplied field owners while preserving unrelated object and child identities. */
export function exchangeSupplied<R extends object>(row:R,patch:Partial<R>,keys:readonly(keyof R)[]):void{for(const key of keys){if(Object.prototype.hasOwnProperty.call(patch,key)){const next=patch[key];patch[key]=row[key];row[key]=next as R[typeof key]}}}

/** 🎨️ Presence distinguishes untouched, cleared and every concrete profile while preserving displaced ownership. */
export function exchangeImageProfile(row:{colorProfile:string|null},patch:{colorProfile?:string|null}){exchangeSupplied(row,patch,['colorProfile'])}
