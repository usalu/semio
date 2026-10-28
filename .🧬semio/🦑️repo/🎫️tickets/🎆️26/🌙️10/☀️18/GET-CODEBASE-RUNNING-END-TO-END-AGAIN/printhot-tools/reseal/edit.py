"""🔂 Anchored edits of batch `reseal` (lane printhot): `python3 -I edit.py <root> check|apply`.

The child seals the slice of a light record when the generation it serves now holds the SAME slice (the carry rule), not only when it is the same generation: in G36 print1 kept a light record of generation 633, a course build published 634 (outside print's slice), print2 was refused `other-generation` and re-lit, so print could hit at the earliest on its 4th run. New law `🧪️tests/🔂️reseal/🟦️.ts` (red on live)."""
import os, sys

M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
HERE = os.path.dirname(os.path.abspath(__file__))

EDITS = {
    M + "/📤️relevance/🟦️.ts": [
        ('import {readGenerationPinV1} from "../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";\n',
         'import {readGenerationPinV1} from "../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";\nimport {graphMemoSliceSameV1,type MemoCarryManifestV1} from "../🧳️carry/🟦️.ts";\n'),
        ('import {GENERATION_POLICY_V1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,readGenerationPointerV1,selectFactClassesV1,type FactIndexV1} from',
         'import {GENERATION_POLICY_V1,compareCodePointsV1,createGenerationStoreV1,createNodeFilePortV1,readGenerationPointerV1,selectFactClassesV1,type FactIndexV1,type GenerationManifestV1} from'),
        ('  if(input.expected!==undefined&&input.expected!==pointer.manifest)return unsealable(header,"other-generation");\n', ''),
        ('  const engine=createGenerationStoreV1({root:generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(signal),manifest=sealed?.manifest;\n',
         '  const engine=createGenerationStoreV1({root:generations,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}}),sealed=await engine.read(signal),manifest=sealed?.manifest;\n  if(input.expected!==undefined&&input.expected!==pointer.manifest&&!(manifest&&await sameSliceV1(engine,input.expected,manifest,input.pin,signal)))return unsealable(header,"other-generation");\n'),
        ("/** 🧪️ Classifies the facts of the slice the planner just served (pointer, manifest, fact index, frontier of its generation) against the journal's own sight and writes the sidecar the parent seals.",
         "/** 🔂️ Whether the slice the pin covers in `manifest` is the slice the light record's generation `expected` held: that generation is still retained and the carry rule finds every shared digest and every covered owner row identical. */\nasync function sameSliceV1(engine:ReturnType<typeof createGenerationStoreV1>,expected:string,manifest:GenerationManifestV1,pin:string|undefined,signal?:AbortSignal):Promise<boolean>{\n try{const before=await engine.readManifestAt(expected,signal);if(!before)return false;const covered=decodeVerifiedScopeV1(Object.keys(manifest.owners).sort(compareCodePointsV1),readGenerationPinV1(pin as string).covered);return graphMemoSliceSameV1(before as unknown as MemoCarryManifestV1,manifest as unknown as MemoCarryManifestV1,covered)===\"same\";}catch{return false;}\n}\n\n/** 🧪️ Classifies the facts of the slice the planner just served (pointer, manifest, fact index, frontier of its generation) against the journal's own sight and writes the sidecar the parent seals. A light record's `expected` generation that differs from the served one still seals when the served slice is the same there ({@link sameSliceV1}), so a peer's refresh between two identical requests does not defer sealing."),
    ],
    M + "/🟦️.ts": [
        (" /** 🧲️ The generation a light record kept for this request: the child of the next run computes its relevance only when it serves this one. */",
         " /** 🧲️ The generation a light record kept for this request: the child of the next run computes its relevance only when it serves this generation or a newer one holding the same slice. */"),
    ],
    M + "/🧬️schema/🔣️.json": [
        ("sealed: with the relevance of the generation, written by the run that found the same generation again (so the sealing cost is paid only by a repeated request, never by an edit-and-run loop).",
         "sealed: with the relevance of the generation, written by the run that found the same slice again, in the same generation or in a newer one whose rows of the slice are identical (Policy.carry), so the sealing cost is paid only by a repeated request, never by an edit-and-run loop."),
    ],
}
NEW = {M + "/🧪️tests/🔂️reseal/🟦️.ts": "new/law.ts"}


def run(root, verb):
    bad = 0
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for old, new in edits:
            if new and text.count(new) == 1 and (new not in old):
                continue
            if not new and old not in text:
                continue
            if text.count(old) != 1:
                print("ANCHOR", text.count(old), rel[-60:], old[:80].replace("\n", "\\n")); bad += 1; continue
            text = text.replace(old, new)
        if verb == "apply" and not bad:
            open(path, "w", encoding="utf8").write(text)
    for rel, source in NEW.items():
        if verb == "apply" and not bad:
            os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
            open(os.path.join(root, rel), "wb").write(open(os.path.join(HERE, source), "rb").read())
    print(("applied" if verb == "apply" else "applicable") if not bad else "DRIFT", bad)
    sys.exit(2 if bad else 0)


run(os.path.abspath(sys.argv[1]), sys.argv[2])
