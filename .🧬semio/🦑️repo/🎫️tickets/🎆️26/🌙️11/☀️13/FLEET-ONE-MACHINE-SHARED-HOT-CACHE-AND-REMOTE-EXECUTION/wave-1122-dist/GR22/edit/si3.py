import sys,re
p=sys.argv[1];s=open(p,encoding='utf-8').read()
def rep(old,new,count=1):
    global s
    n=s.count(old);assert n==count,(n,old[:160]);s=s.replace(old,new)
def cut(a_marker,b_marker,new=""):
    global s
    a=s.index(a_marker);b=s.index(b_marker,a);s=s[:a]+new+s[b:]
# walkFacts docstring: only repository walks and programs now
a=s.index("/** 🪢️ The derived fact of every command walk whose stored observations a project can reuse")
b=s.index("const walkFactsV1 = new WeakMap();",a)
s=s[:a]+"/** 🪢️ The derived fact of every repository walk and TypeScript program whose stored observations a project can reuse, by those observations: the question it answered and the digest of its answer, so a project names it by one row instead of by the rows it read (a command walk is named by its `closure-graph` row instead, see {@link boundedScriptInputsV1}). */\n"+s[b:]
# remove walkMembersV1, walkPartsV1, walkQuestionV1, walkAnswerV1
cut("/** ✍️ The stored observations of every file a command walk visited","/** 🔁️ Asks a derived fact of the command closure again")
# deriveClosureFact: docstring + remove walk branches
rep("/** 🔁️ Asks a derived fact of the command closure again against the live files (a command walk or a TypeScript program):","/** 🔁️ Asks a derived fact of the command closure again against the live files (a repository walk or a TypeScript program):")
a=s.index("  if (question?.walk !== 1 || !Array.isArray(question.roots)")
b=s.index("\n}\n",a)
s=s[:a]+"  throw refuse();"+s[b:]
# sidecar: facts only
rep("const closureSidecarV1 = layers => ({ facts: layers.map(layer => walkFactsV1.get(layer) ?? null), members: layers.map(layer => { const walks = walkMembersV1.get(layer); return walks ? [...walks] : null; }) });","const closureSidecarV1 = layers => ({ facts: layers.map(layer => walkFactsV1.get(layer) ?? null) });")
rep('const walkFactsByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-facts", maxEntries: 65536, maxBytes: 65536 }), walkMembersByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-members", maxEntries: 65536, maxBytes: 65536 }), layerListsV1 = new WeakMap();','const walkFactsByDigestV1 = createBoundedCacheV1({ name: "inference:closure-walk-facts", maxEntries: 65536, maxBytes: 65536 }), layerListsV1 = new WeakMap();')
cut("const seededMembersV1 = new WeakSet(), someMemberV1","/** 🌱️ Hands the walk facts and members of a derived entry's layers")
rep('''/** 🌱️ Hands the walk facts and members of a derived entry's layers to the digest tables before the entry drops its parsed arrays, so the stored shape answers the same questions. */
const seedLayerDigestsV1 = entry => entry.layers.forEach((layer, at) => {
  const digest = entry.digests[at], fact = walkFactsV1.get(layer), members = walkMembersV1.get(layer);
  if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
  if (members) mergeWalkMembersV1(walkMembersByDigestV1, digest, [...members]);
});''','''/** 🌱️ Hands the derived facts of a derived entry's layers to the digest table before the entry drops its parsed arrays, so the stored shape answers the same questions. */
const seedLayerDigestsV1 = entry => entry.layers.forEach((layer, at) => {
  const digest = entry.digests[at], fact = walkFactsV1.get(layer);
  if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
});''')
a=s.index("/** 🧱️ The shared layers of an entry as views:")
b=s.index("const closureLayerViewsV1 = entry => {",a)
s=s[:a]+"/** 🧱️ The shared layers of an entry as views: a derived entry names its parsed arrays, a stored entry only the digests of its layers (read from the store when a view is consumed, never cached parsed) with the derived facts its sidecar and every other stored entry of the process recorded for them. */\n"+s[b:]
rep('''  if (entry.layers) return entry.layers.map(layer => ({ key: layer, rows: layer, fact: walkFactsV1.get(layer), members: walkMembersV1.get(layer) }));
  if (entry.digests === undefined) throw layerLostRefusalV1(undefined);
  return entry.digests.map((digest, at) => {
    const fact = entry.sidecar?.facts[at], members = entry.sidecar?.members[at];
    if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
    if (members && (!seededMembersV1.has(members) || !walkMembersByDigestV1.has(digest))) { seededMembersV1.add(members); mergeWalkMembersV1(walkMembersByDigestV1, digest, members); }
    return { key: digest, digest, store: entry.store, fact: walkFactsByDigestV1.get(digest), members: walkMembersByDigestV1.get(digest) };
  });''','''  if (entry.layers) return entry.layers.map(layer => ({ key: layer, rows: layer, fact: walkFactsV1.get(layer) }));
  if (entry.digests === undefined) throw layerLostRefusalV1(undefined);
  return entry.digests.map((digest, at) => {
    const fact = entry.sidecar?.facts[at];
    if (fact && !walkFactsByDigestV1.has(digest)) walkFactsByDigestV1.set(digest, fact);
    return { key: digest, digest, store: entry.store, fact: walkFactsByDigestV1.get(digest) };
  });''')
# canonicalSidecarV1 removed (members were its only subject)
cut("/** 🔢️ The sidecar of an exported bundle as a function of the entry:","const TRAVEL_DIGEST_V1 =")
rep("bundle: { ...described.bundle, sidecar: canonicalSidecarV1(described.bundle.sidecar) } }));","bundle: described.bundle }));")
# farm
rep('''/** 🏭️ The walk of one recipe in a walk child while one is granted, else on this thread: the owners that wait for a shared entry wait on the child, this thread parses the answer and hands the walk facts and members of the layers back to the process. */''','''/** 🏭️ The walk of one recipe in a walk child while one is granted, else on this thread: the owners that wait for a shared entry wait on the child, this thread parses the answer and hands the derived facts of the layers back to the process. */''')
rep("  farmed.layers.forEach((layer, at) => { const fact = farmed.sidecar?.facts[at], members = farmed.sidecar?.members[at]; if (fact) walkFactsV1.set(layer, fact); if (members) walkMembersV1.set(layer, new Set(members)); });","  farmed.layers.forEach((layer, at) => { const fact = farmed.sidecar?.facts[at]; if (fact) walkFactsV1.set(layer, fact); });")
# resolve / plan without members
rep("const values = new Map(), derived = new Map(), walked = new Set(), layers = new Map(), seen = new Set(), rows = [],","const values = new Map(), derived = new Map(), layers = new Map(), seen = new Set(), rows = [],")
rep("derived.set(row.path, { row, support: supportOf(layer) }); walked.add(layer.fact.row.path); } } }","derived.set(row.path, { row, support: supportOf(layer) }); } } }")
rep("      for (const layer of layers.values()) { if (layer.fact) continue; if (!layer.members || !someMemberV1(layer.members, question => walked.has(question))) addLayer(layer); }","      for (const layer of layers.values()) if (!layer.fact) addLayer(layer);")
rep("      const values = new Map(), derived = new Map(), layers = new Map(), walked = new Set(), entries = [];","      const values = new Map(), derived = new Map(), layers = new Map(), entries = [];")
rep("derived.set(row.path, { row, digest: layer.digest }); walked.add(layer.fact.row.path); } } }","derived.set(row.path, { row, digest: layer.digest }); } } }")
rep("layers: [...layers.values()].filter(layer => !layer.fact && (!layer.members || !someMemberV1(layer.members, question => walked.has(question)))).map(layer => layer.digest)","layers: [...layers.values()].filter(layer => !layer.fact).map(layer => layer.digest)")
# reset and collections
rep("walkFactsByDigestV1.clear(); walkMembersByDigestV1.clear();","walkFactsByDigestV1.clear();")
rep(', once("closure:walkMembers (real)", walkMembersByDigestV1, list => list)','')
open(p,'w',encoding='utf-8').write(s)
print("ok")
