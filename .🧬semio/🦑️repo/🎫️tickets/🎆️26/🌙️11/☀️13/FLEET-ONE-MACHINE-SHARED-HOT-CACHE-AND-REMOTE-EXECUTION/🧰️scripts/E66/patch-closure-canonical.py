B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    s=open(B+path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:70])
        s=s.replace(a,b)
    open(B+path,"w",encoding="utf8").write(s)
edit("🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",[
("const TRAVEL_DIGEST_V1 =","""/** 🔢️ The sidecar of an exported bundle as a function of the entry: the walk members of each layer are the walks that include it which the process knew (every walk it derived or loaded before, so another machine knows another set); only the walks that are facts of this entry's own layers can matter when the entry is resolved, so the exported members are those, in code unit order, and two machines that derived the entry after different prior walks export the same bytes. */
export function canonicalSidecarV1(sidecar) {
  if (sidecar === null || sidecar === undefined || !Array.isArray(sidecar.members)) return sidecar;
  const own = new Set((sidecar.facts ?? []).flatMap(fact => fact?.row?.path === undefined ? [] : [fact.row.path]));
  return { ...sidecar, members: sidecar.members.map(list => list ? list.filter(walk => own.has(walk)).sort() : list) };
}
const TRAVEL_DIGEST_V1 ="""),
("""bundle: described.bundle }));
  },""","""bundle: { ...described.bundle, sidecar: canonicalSidecarV1(described.bundle.sidecar) } }));
  },"""),
])
edit("🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts",[
("  packer.add(neutralRowsV1(rows,host,asked,policy,memo));\n  for(const layer of layers)packer.add(neutralRowsV1(layer,host,asked,policy,memo));",
 "  packer.add(canonicalRowOrderV1(neutralRowsV1(rows,host,asked,policy,memo)));\n  for(const layer of layers)packer.add(canonicalRowOrderV1(neutralRowsV1(layer,host,asked,policy,memo)));"),
("/** 🌊️ A closure bundle in the neutral stored form, built layer by layer:","/** 🌊️ A closure bundle in the neutral stored form, with its rows and every layer in the canonical order of the rows (a layer is a set that two machines observed in another order), built layer by layer:"),
])
print("ok")
