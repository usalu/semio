import {applyPatch} from 'fast-json-patch';
/** 🔮️ The test-only third-party patcher owns its complete input and exports no library types. */
export function ownedPatchedDocument(base:unknown,patch:unknown):unknown{return applyPatch(structuredClone(base),structuredClone(patch) as Parameters<typeof applyPatch>[1],true,true).newDocument}
