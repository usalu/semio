"""🗂 Anchored edits of batch `gitrow` (lane printhot): `python3 -I edit.py <root> check|apply`.

The witness row `g` of a Bazel plan digests only the tracked entries the planner admits as sources (git's listing minus `.🧬semio/` and `.git/`, the rule `sourceFiles` applies), so a ticket file staged by any agent no longer replans every plan with a root-anchored pattern (g33-print2: `planning again: g [] changed`, +25 s). The whole-universe plan-reuse law uses the real digest function and gains a staged-ticket case (needless replan on base) and an independent oracle of the digest."""
import os, sys

B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
HERE = os.path.dirname(os.path.abspath(__file__))

EDITS = {
    B + "/📜️script.ts": [
        ("/** 🧾️ The digest of the tracked listing, or `!failed` where git cannot answer (no repository), which the filesystem fallback of the planner then observes itself. */\nasync function bazelTrackedDigestV1(root:string,pathspecs:readonly string[],signal:AbortSignal):Promise<string>{\n try{return worldWitnessGitDigestV1(await bazelTrackedListingV1(root,pathspecs,signal));}catch(error){signal.throwIfAborted();return \"!failed\";}\n}",
         "const BAZEL_UNADMITTED_PREFIXES_V1=[\".🧬semio/\",\".git/\"] as const,BAZEL_UNADMITTED_BYTES_V1=BAZEL_UNADMITTED_PREFIXES_V1.map(prefix=>Buffer.from(prefix));\n\n/** 🚪️ Whether a path git lists is a source the planner admits: everything except the checkout's own bookkeeping (`.🧬semio/`, `.git/`), which no task can declare. */\nexport function bazelAdmittedSourceV1(path:string):boolean{return !BAZEL_UNADMITTED_PREFIXES_V1.some(prefix=>path.startsWith(prefix));}\n\n/** ✂️ The admitted entries of a NUL-terminated listing, in git's order and bytes: what the planner can see of it ({@link bazelAdmittedSourceV1}). */\nexport function bazelAdmittedListingV1(listing:Buffer):Buffer{\n const kept:Buffer[]=[];\n for(let at=0;at<listing.length;){let end=listing.indexOf(0,at);if(end<0)end=listing.length-1;const entry=listing.subarray(at,end+1);if(!BAZEL_UNADMITTED_BYTES_V1.some(prefix=>entry.length>prefix.length&&entry.subarray(0,prefix.length).equals(prefix)))kept.push(entry);at=end+1;}\n return Buffer.concat(kept);\n}\n\n/** 🧾️ The digest of the admitted part of the tracked listing, or `!failed` where git cannot answer (no repository), which the filesystem fallback of the planner then observes itself; an entry the planner never admits (a ticket file staged below `.🧬semio/`) leaves it unchanged. */\nexport async function bazelTrackedDigestV1(root:string,pathspecs:readonly string[],signal:AbortSignal):Promise<string>{\n try{return worldWitnessGitDigestV1(bazelAdmittedListingV1(await bazelTrackedListingV1(root,pathspecs,signal)));}catch(error){signal.throwIfAborted();return \"!failed\";}\n}"),
        ('.filter(Boolean))].filter(path=>!path.startsWith(".🧬semio/")&&!path.startsWith(".git/")).sort();',
         '.filter(Boolean))].filter(bazelAdmittedSourceV1).sort();'),
    ],
    B + "/♻️plan-reuse/🧪️tests/🌌️whole/🧫️fixtures/🔣️.json": [
        ('    ".hidden/🧬️schema/h.json": "{\\"h\\":1}\\n",\n',
         '    ".hidden/🧬️schema/h.json": "{\\"h\\":1}\\n",\n    ".🧬semio/🦑️repo/🎫️tickets/t/old.md": "o\\n",\n'),
    ],
    B + "/♻️plan-reuse/🧪️tests/🌌️whole/🟦️.ts": [
        ('import {bazelIgnoreSourcesDigestV1}from"../../../📜️script.ts";',
         'import {bazelAdmittedListingV1,bazelIgnoreSourcesDigestV1,bazelTrackedDigestV1}from"../../../📜️script.ts";'),
        ('const ports=(root:string)=>({git:async(subject:string,expected:string)=>{const specs=JSON.parse(subject) as string[];let listing:string;try{listing=git(root,"ls-files","-c","-z",...(specs.length?["--",...specs]:[]));}catch{return expected==="!failed";}return worldWitnessGitDigestV1(listing)===expected;},ignore:',
         'const ports=(root:string)=>({git:async(subject:string,expected:string)=>(await bazelTrackedDigestV1(root,JSON.parse(subject) as string[],signal))===expected,ignore:'),
        (' "document below an excluded dot directory":root=>write(root,".hidden/🧬️schema/w.json","{}\\n"),',
         ' "document below an excluded dot directory":root=>write(root,".hidden/🧬️schema/w.json","{}\\n"),\n "ticket file another agent staged below .🧬semio":root=>{write(root,".🧬semio/🦑️repo/🎫️tickets/t/n.md","n\\n");git(root,"add","--",".🧬semio");},'),
        ('test("a root-anchored pattern without a pruning negation keeps the plan unreusable",()=>{',
         'test("the tracked digest covers exactly the entries the planner admits: oracle = git\'s own listing filtered as text",async()=>{\n const{base,root}=checkout();\n try{\n  write(root,".🧬semio/🦑️repo/🎫️tickets/t/n.md","n\\n");write(root,"a/🧬️schema/staged.json","{}\\n");git(root,"add","-A");\n  const text=git(root,"ls-files","-c","-z"),oracle=text.split("\\0").filter(path=>path&&!path.startsWith(".🧬semio/")&&!path.startsWith(".git/")).map(path=>path+"\\0").join("");\n  expect(text).toContain(".🧬semio/");expect(oracle).toContain("a/🧬️schema/staged.json");\n  expect(bazelAdmittedListingV1(Buffer.from(text)).toString()).toBe(oracle);\n  expect(await bazelTrackedDigestV1(root,[],signal)).toBe(worldWitnessGitDigestV1(oracle));\n  expect(await bazelTrackedDigestV1(join(base,"none"),[],signal)).toBe("!failed");\n  const before=await bazelTrackedDigestV1(root,[],signal),raw=git(root,"ls-files","-c","-z");\n  write(root,".🧬semio/🦑️repo/🎫️tickets/t/later.md","l\\n");git(root,"add","-A");\n  expect(worldWitnessGitDigestV1(git(root,"ls-files","-c","-z")),"negative control: the digest of the whole listing (the row before this batch) moves").not.toBe(worldWitnessGitDigestV1(raw));\n  expect(await bazelTrackedDigestV1(root,[],signal),"the admitted digest does not").toBe(before);\n }finally{rmSync(base,{recursive:true,force:true});}\n},120000);\n\ntest("a root-anchored pattern without a pruning negation keeps the plan unreusable",()=>{'),
    ],
}
NEW = {}


def run(root, verb):
    bad = 0
    for rel, edits in EDITS.items():
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        for old, new in edits:
            if text.count(new) == 1:
                continue
            if text.count(old) != 1:
                print("ANCHOR", text.count(old), rel[-60:], old[:80].replace("\n", "\\n")); bad += 1; continue
            text = text.replace(old, new)
        if verb == "apply" and not bad:
            open(path, "w", encoding="utf8").write(text)
    print(("applied" if verb == "apply" else "applicable") if not bad else "DRIFT", bad)
    sys.exit(2 if bad else 0)


run(os.path.abspath(sys.argv[1]), sys.argv[2])
