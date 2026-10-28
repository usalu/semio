"""Anchored edit of the memo edit law: harvest/sealed take the deriver, planner takes the oracle, the two-step law is appended. Usage: edit_law.py <root> [check|apply]."""
import pathlib,sys
root=pathlib.Path(sys.argv[1]);mode=sys.argv[2] if len(sys.argv)>2 else "apply"
law=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/✏️edit/🟦️.ts"
text=law.read_text(encoding="utf8")
pairs=[
 ("async function harvest(root:string):Promise<","async function harvest(root:string,derive:MemoDeriverV1=deriveModel):Promise<"),
 ("fact:null as unknown},answer=await deriveModel(question)","fact:null as unknown},answer=await derive(question)"),
 ("async function sealed(root:string){\n const {rows,raw,index}=await harvest(root),","async function sealed(root:string,derive:MemoDeriverV1=deriveModel){\n const {rows,raw,index}=await harvest(root,derive),"),
 ("ask:suspects=>deriveGraphMemoSuspectsV1({index,suspects,derive:deriveModel})})};\n const pin=","ask:suspects=>deriveGraphMemoSuspectsV1({index,suspects,derive})})};\n const pin="),
 ("async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:number|undefined):Promise<boolean>{\n const same=d!==undefined&&modelAnswer(root)===d;","async function planner(root:string,raw:readonly MemoRowV1[],frontier:Awaited<ReturnType<typeof walk>>,d:number|undefined,answer:(root:string)=>number|undefined=modelAnswer):Promise<boolean>{\n const same=d!==undefined&&answer(root)===d;"),
]
appendix=(pathlib.Path(__file__).parent/"law.ts.txt").read_text(encoding="utf8")
done=appendix.strip() in text
if mode=="check":print("applied" if done else "base");sys.exit(0)
if done:print("already applied");sys.exit(0)
for old,new in pairs:
  if text.count(old)!=1:print("anchor lost:",old[:80]);sys.exit(1)
  text=text.replace(old,new)
law.write_text(text.rstrip("\n")+"\n"+appendix,encoding="utf8");print("applied")
