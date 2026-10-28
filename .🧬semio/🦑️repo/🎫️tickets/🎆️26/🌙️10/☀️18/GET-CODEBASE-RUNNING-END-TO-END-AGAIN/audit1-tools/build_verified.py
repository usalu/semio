#!/usr/bin/env python3
"""🕰️ Builds (or rebuilds onto the current live files) the staged batch `audit1-tools/staged/verified`: the graph memo seals only a slice the planner verified after the memo cursor.
usage: python3 -I build_verified.py [--root DIR]   (default: walk up to the directory holding 🧰️framework)
Every edit is anchored (exactly one occurrence); exit 1 names a lost anchor. Writes files/, base/ and manifest.json beside stage.py."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
STAGE = os.path.join(HERE, "staged", "verified")
MEMO = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/": here = os.path.dirname(here)
    return here
EDITS = {
 MEMO + "/🧬️schema/🔣️.json": [
  ('"expect": "SEMIO_BUILD_GRAPH_MEMO_EXPECT"', '"expect": "SEMIO_BUILD_GRAPH_MEMO_EXPECT",\n        "requestedAt": "SEMIO_BUILD_GRAPH_REQUESTED_AT"'),
  ("A missing or foreign pin makes the sidecar unsealable (`pin`, `other-generation`).", "A missing or foreign pin makes the sidecar unsealable (`pin`, `other-generation`). A slice the planner did not verify after the memo's journal cursor (served from a peer's verified stamp or from the pin of an outer invocation) is unsealable too (`unverified`): the sealing child is started with `requestedAt` set to the clock time right after the cursor, so a stamp serves it only when its observation began after the cursor, and the pin it records must not be older than that time."),
 ],
 MEMO + "/🟦️.ts": [
  ("version:1;directory:string;switch:string;sidecar:string;expect:string;", "version:1;directory:string;switch:string;sidecar:string;expect:string;requestedAt:string;"),
  (" /** 📍️ The journal position taken before the planner child started (`undefined` when the journal cannot vouch). */\n readonly cursor:string|undefined;\n", " /** 📍️ The journal position taken before the planner child started (`undefined` when the journal cannot vouch). */\n readonly cursor:string|undefined;\n /** ⏲️ The clock time right after the cursor was taken (`undefined` without a cursor): a planner child whose slice is sealed must have verified it at or after this time, never earlier (a peer's stamp or an outer pin may predate an edit that lands before the cursor). */\n readonly cursorAtMs:number|undefined;\n"),
  (' let cursorText:string|undefined;\n try{const taken=await client.cursor(signal);if(!("exact" in taken))cursorText=formatChangeCursorV1(taken);}catch(error){signal?.throwIfAborted();}', ' let cursorText:string|undefined,cursorAtMs:number|undefined;\n try{const taken=await client.cursor(signal);if(!("exact" in taken)){cursorText=formatChangeCursorV1(taken);cursorAtMs=Date.now();}}catch(error){signal?.throwIfAborted();}'),
  ("  cursor:cursorText,\n", "  cursor:cursorText,\n  cursorAtMs,\n"),
 ],
 MEMO + "/🚪️command/🟦️.ts": [
  ("/** 🗺️ Serves a request from its memo when the journal proves that nothing the generation read has changed; otherwise runs the planner child (asking it for its relevance when a light record names the generation it will serve) and keeps what it produced. A request without a memo is planned as before.", "/** 🗺️ Serves a request from its memo when the journal proves that nothing the generation read has changed; otherwise runs the planner child (asking it for its relevance when a light record names the generation it will serve, with the request time moved to the moment after the memo cursor so that only a verification that began after the cursor serves the slice it seals) and keeps what it produced. A request without a memo is planned as before."),
  (' const expect=memo?.cursor===undefined?undefined:memo.expected(),produced=await input.plan(expect===undefined?undefined:{[GRAPH_MEMO_POLICY_V1.sidecar]:stem+".json",[GRAPH_MEMO_POLICY_V1.expect]:expect}),graph=input.parse(produced.plan);', ' const expect=memo?.cursor===undefined?undefined:memo.expected(),after=memo?.cursorAtMs,produced=await input.plan(expect===undefined||after===undefined?undefined:{[GRAPH_MEMO_POLICY_V1.sidecar]:stem+".json",[GRAPH_MEMO_POLICY_V1.expect]:expect,[GRAPH_MEMO_POLICY_V1.requestedAt]:String(after)}),graph=input.parse(produced.plan);'),
 ],
 MEMO + "/📤️relevance/🟦️.ts": [
  ("/** 🧾️ What the sidecar of one served slice is computed from: the workspace root, the generation store, the journal's exclusion rule, the pin the child served and the sidecar stem. */\nexport type MemoSidecarInputV1=Readonly<{root:string;generations:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>|undefined;pin:string|undefined;stem:string;expected?:string|undefined;client:ChangeJournalClientV1;insensitive:boolean;signal?:AbortSignal}>;", "/** 🧾️ What the sidecar of one served slice is computed from: the workspace root, the generation store, the journal's exclusion rule, the pin the child served, the sidecar stem and the earliest coverage time the pin may carry (`verifiedAfter`, the clock time right after the memo cursor). */\nexport type MemoSidecarInputV1=Readonly<{root:string;generations:string;exclude:Readonly<{rooted:readonly string[];names:readonly string[]}>|undefined;pin:string|undefined;stem:string;expected?:string|undefined;verifiedAfter?:number|undefined;client:ChangeJournalClientV1;insensitive:boolean;signal?:AbortSignal}>;"),
  ("Best effort: any failure leaves no header and the parent seals nothing.", "A pin whose coverage time is older than `verifiedAfter` (the slice came from a peer's stamp or an outer pin, not from a verification after the memo cursor) is unsealable (`unverified`). Best effort: any failure leaves no header and the parent seals nothing."),
  ('  if(input.expected!==undefined&&input.expected!==pointer.manifest)return unsealable(header,"other-generation");\n', '  if(input.expected!==undefined&&input.expected!==pointer.manifest)return unsealable(header,"other-generation");\n  if(input.verifiedAfter!==undefined){let at:number;try{at=readGenerationPinV1(input.pin as string).covered.atMs;}catch{return unsealable(header,"pin");}if(!(at>=input.verifiedAfter))return unsealable(header,"unverified");}\n'),
  ("/** 🏗️ The child's last step: the sidecar of the slice it served (`pin`), when its parent asked for one, from the generation store of the repository and the state of its change journal. */", "/** 🏗️ The child's last step: the sidecar of the slice it served (`pin`), when its parent asked for one, from the generation store of the repository and the state of its change journal; the request time the parent set (the moment after its memo cursor) is the earliest coverage time the pin may carry, and a missing one seals nothing. */"),
  (" const stem=target.replace(/\\.json$/u,\"\");\n", " const stem=target.replace(/\\.json$/u,\"\"),after=Number(environment[GRAPH_MEMO_POLICY_V1.requestedAt]),verifiedAfter=Number.isSafeInteger(after)&&after>0?after:Number.POSITIVE_INFINITY;\n"),
  ("expected:environment[GRAPH_MEMO_POLICY_V1.expect],client,", "expected:environment[GRAPH_MEMO_POLICY_V1.expect],verifiedAfter,client,"),
 ],
}
NEW = {MEMO + "/🧪️tests/🕰️verified/🟦️.ts": os.path.join(STAGE, "files", "law.ts")}
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    law = open(NEW[MEMO + "/🧪️tests/🕰️verified/🟦️.ts"], "rb").read()
    for sub in ("files", "base"):
        path = os.path.join(STAGE, sub)
        for name in os.listdir(path) if os.path.isdir(path) else []:
            if name != "law.ts": os.remove(os.path.join(path, name))
        os.makedirs(path, exist_ok=True)
    manifest = {}
    for index, (rel, edits) in enumerate(EDITS.items()):
        base = open(os.path.join(root, rel), "rb").read(); text = base.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1: sys.exit(f"anchor lost ({text.count(old)}x) in {rel}: {old[:80]}")
            text = text.replace(old, new)
        name = f"{index:02d}" + os.path.splitext(rel)[1]
        open(os.path.join(STAGE, "files", name), "wb").write(text.encode("utf8")); open(os.path.join(STAGE, "base", name), "wb").write(base)
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(text.encode("utf8"))}
    rel = MEMO + "/🧪️tests/🕰️verified/🟦️.ts"
    manifest[rel] = {"file": "law.ts", "base": None, "new": sha(law)}
    json.dump(manifest, open(os.path.join(STAGE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("staged", len(manifest), "files from", root)
main()
