import hashlib, sys

path = "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"
text = open(path, encoding="utf8").read()
before = hashlib.sha256(text.encode("utf8")).hexdigest()
if "beginNxFleetV1" in text:
    print("already applied", before[:12])
    sys.exit(0)


def once(source, old, new):
    if source.count(old) != 1:
        raise SystemExit("anchor not unique: " + old[:80] + " x" + str(source.count(old)))
    return source.replace(old, new)


text = once(
    text,
    'let pendingGeneration:import("../🕸️graph/🔏️seal/📌️generation/🟦️.ts").NxGenerationClaimV1|undefined;',
    'let pendingGeneration:import("../🕸️graph/🔏️seal/📌️generation/🟦️.ts").NxGenerationClaimV1|undefined,nxFleet:import("../../🏗️build/🪪️graph-state/🧶️nx/🟦️.ts").NxFleetV1|undefined,receiptWritten=false;',
)
text = once(
    text,
    "pendingGeneration=generationClaim!==undefined&&!generation?generationClaim:undefined;\n",
    "pendingGeneration=generationClaim!==undefined&&!generation?generationClaim:undefined;\n"
    '        const fleetInstalled=pendingGeneration!==undefined?await(async()=>{const owner=await import("../../🏗️build/🪪️graph-state/🧶️nx/🟦️.ts");nxFleet=await owner.beginNxFleetV1({root:this.root,environment:env,claim:pendingGeneration!,directory:workspaceData,manifestFile:env.SEMIO_BUILD_GRAPH_PATH,signal:actualPhase.signal,say:owner.graphSayV1(env),waitLimitMilliseconds:600000});return nxFleet.installed;})().catch(()=>false):false;\n'
    "        if(fleetInstalled)pendingGeneration=undefined;\n",
)
text = once(text, "const sealed=generation||!env.SEMIO_BUILD_GRAPH_DIGEST", "const sealed=generation||fleetInstalled||!env.SEMIO_BUILD_GRAPH_DIGEST")
text = once(
    text,
    '          if(profileBootstrap)sayBootstrapV1("nx.generation-receipt-written",{written:String(written)});\n        }\n',
    '          if(profileBootstrap)sayBootstrapV1("nx.generation-receipt-written",{written:String(written)});\n          receiptWritten=written;\n        }\n'
    "        await nxFleet?.finish(receiptWritten).catch(()=>undefined);\n",
)
text = once(text, "      finishing = true;\n      if (invocation.watch && children.length && !cancelled) stop(\"SIGTERM\");", "      finishing = true;\n      void nxFleet?.finish(false).catch(()=>undefined);\n      if (invocation.watch && children.length && !cancelled) stop(\"SIGTERM\");")
open(path, "w", encoding="utf8").write(text)
print("before", before[:12], "after", hashlib.sha256(text.encode("utf8")).hexdigest()[:12])
