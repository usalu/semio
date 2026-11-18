const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
export const SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
export const staleTake={
 file:SI,
 edits:[[`async function takenClosureEntryV1(workspaceRoot, id) {
  const scripts = closureScriptsV1.get(workspaceRoot), store = closureStoreOfV1(workspaceRoot);
  if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
  const stored = store ? await store.load(id) : undefined, entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; }
  if (entry) closureCountsV1.stale++;
  throw Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS" });
}`,`async function takenClosureEntryV1(workspaceRoot, id) {
  const scripts = closureScriptsV1.get(workspaceRoot), store = closureStoreOfV1(workspaceRoot);
  if (!scripts) throw refuseBuildV1("missing", "closure.scripts-unbound", { workspace: String(workspaceRoot) });
  const miss = () => Object.assign(new Error("A closure entry is not stored here or no longer holds: the owner is inferred by the owner loop, not taken"), { code: "BUILD_TAKE_MISS" });
  const round = closurePassV1 ?? observationRoundV1(), stamp = round !== null && typeof round === "object" && store ? await store.stamp(id) : undefined, judged = stamp === undefined ? undefined : staleTakesV1.get(round) ?? staleTakesV1.set(round, new Map()).get(round);
  if (judged?.get(id) === stamp) throw miss();
  const stored = store ? await store.load(id) : undefined, entry = stored && { ...stored, layers: undefined, store, answers: new Map(), checked: null };
  if (entry && await closureEntryHoldsAsyncV1(entry, scripts)) { closureCountsV1.loaded++; closureEntriesV1.set(id, entry); return entry; }
  if (entry) { closureCountsV1.stale++; judged?.set(id, stamp); }
  throw miss();
}

/** 🪦️ The entries a take found stale, per round, with the stamp of the stored entry that was judged: the next look of the same round that meets the same stored entry is a miss at once, without loading and verifying it again (a take pass met the same stale shared entries in every owner: 13 084 stale verdicts over 582 looks, each a load of the entry file and a check of its layers). A round is one snapshot of the tree, so the verdict cannot change within it; a stored entry of another stamp is judged anew.
 * @see ./🧪️tests/🪦️stale-take */
const staleTakesV1 = new WeakMap();`]]
};
export const NODES=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs";
export const knownUnit={
 file:NODES,
 edits:[["knownMegabytes:Math.round((enforcement.known?.(name)??0)/1048576)","knownMegabytes:Math.round(enforcement.known?.(name)??0)"]]
};
export const ENTRIES=B+"🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";
export const membersSet={
 file:ENTRIES,
 edits:[[`const unionMembers=(digest,mapped)=>{const held=sharedMembers.get(digest);if(held===undefined){sharedMembers.set(digest,mapped);return mapped;}if(held===mapped||sameList(held,mapped))return held;const have=new Set(held),extra=mapped.filter(walk=>!have.has(walk));if(extra.length===0)return held;held.push(...extra);return held;};`,
 `const memberSets=new WeakMap(),unionMembers=(digest,mapped)=>{const held=sharedMembers.get(digest);if(held===undefined){sharedMembers.set(digest,mapped);return mapped;}if(held===mapped||sameList(held,mapped))return held;let have=memberSets.get(held);if(have===undefined||have.size!==held.length){have=new Set(held);memberSets.set(held,have);}const extra=mapped.filter(walk=>!have.has(walk));if(extra.length===0)return held;held.push(...extra);for(const walk of extra)have.add(walk);return held;};`]]
};
export const all=[staleTake,knownUnit,membersSet];
export const apply=(file,text,list=all)=>{for(const p of list)if(file.endsWith(p.file)){for(const[a,b]of p.edits){if(!text.includes(a))throw new Error("anchor missing in "+p.file+": "+a.slice(0,60));if(text.split(a).length!==2)throw new Error("anchor not unique");text=text.replace(a,()=>b);}}return text;};
