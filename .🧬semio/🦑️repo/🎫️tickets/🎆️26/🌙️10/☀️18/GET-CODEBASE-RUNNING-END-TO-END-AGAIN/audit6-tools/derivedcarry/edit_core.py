"""Anchored edit of the graph memo core + schema: a derived serve records the rows it left stale (`relevance.carried`), every later settlement re-checks them. Usage: edit_core.py <root> [check|apply]."""
import pathlib,sys
root=pathlib.Path(sys.argv[1]);mode=sys.argv[2] if len(sys.argv)>2 else "apply"
memo=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
core=memo/"🟦️.ts";schema=memo/"🧬️schema/🔣️.json"
C=[
 ("/** 📋️ What an edit settlement answers: held, with the number of changed paths, the rows it re-checked, the derived questions it asked again and the new stamps of the stamped directories it listed again, or the first path that does not hold. */\nexport type MemoEditOutcomeV1=Readonly<{held:true;paths:number;rows:number;questions:number;stamps:ReadonlyMap<string,readonly[number,number]>}>",
  "/** 📋️ What an edit settlement answers: held, with the number of changed paths, the rows it re-checked, the derived questions it asked again, the new stamps of the stamped directories it listed again and the rows that do not hold but whose derived questions answered the same (`stale`: the sealed generation no longer describes them, so the record carries them into every later settlement), or the first path that does not hold. */\nexport type MemoEditOutcomeV1=Readonly<{held:true;paths:number;rows:number;questions:number;stamps:ReadonlyMap<string,readonly[number,number]>;stale:readonly number[]}>"),
 ("When everything holds, a stamped directory listed again gets the stamp it had before the checks, if it still has it and lies outside the racy window. */\nexport async function settleMemoEditsV1(input:Readonly<{root:string;relevance:MemoRelevanceV1;edits:MemoEditIndexV1;changed:readonly ChangeRowV1[];listed:ReadonlyMap<string,string>;index:string;",
  "The rows an earlier settlement left stale (`carried`) are re-checked with the rows the journal names, because their sealed facts no longer describe the tree the derived questions were last asked on. When everything holds, a stamped directory listed again gets the stamp it had before the checks, if it still has it and lies outside the racy window. */\nexport async function settleMemoEditsV1(input:Readonly<{root:string;relevance:MemoRelevanceV1;edits:MemoEditIndexV1;changed:readonly ChangeRowV1[];listed:ReadonlyMap<string,string>;carried?:readonly number[];index:string;"),
 (" for(const [directory,path] of listed){group(path,edits.inside(directory));touched.set(directory,path);if(edits.frontier(directory))listing.set(directory,path);}\n",
  " for(const [directory,path] of listed){group(path,edits.inside(directory));touched.set(directory,path);if(edits.frontier(directory))listing.set(directory,path);}\n if(input.carried?.length)group(CARRIED_V1,input.carried);\n"),
 (" let questions=0;\n"," let questions=0,left:readonly number[]=[];\n"),
 ("   questions=settled.questions;\n","   questions=settled.questions;left=stale;\n"),
 ("for(const id of plain)if(!stale.includes(id)&&!await input.holds([byId.get(id)!],signal)){stale.push(id);first??=path;}",
  "for(const id of plain)if(!stale.includes(id)&&!await input.holds([byId.get(id)!],signal)){stale.push(id);first??=path===CARRIED_V1?shownPathV1(root,byId.get(id)!.path):path;}"),
 (" return{held:true,paths:new Set([...input.changed.map(row=>row[2]),...input.listed.values()]).size,rows:all.size,questions,stamps};\n}\n",
  " return{held:true,paths:new Set([...input.changed.map(row=>row[2]),...input.listed.values()]).size,rows:all.size,questions,stamps,stale:[...left].sort((a,b)=>a-b)};\n}\nconst CARRIED_V1=\"\\u0000carried\",shownPathV1=(root:string,path:string):string=>path.startsWith(root+\"/\")?path.slice(root.length+1):path;\n"),
 ("(value.relevance?.edit===undefined||isSha(value.relevance.edit?.id)&&isSha(value.relevance.edit?.index)&&(value.relevance.edit?.frontier===null||isSha(value.relevance.edit?.frontier)))",
  "(value.relevance?.edit===undefined||isSha(value.relevance.edit?.id)&&isSha(value.relevance.edit?.index)&&(value.relevance.edit?.frontier===null||isSha(value.relevance.edit?.frontier)))&&(value.relevance?.carried===undefined||value.relevance?.edit!==undefined&&Array.isArray(value.relevance.carried)&&value.relevance.carried.length>0&&value.relevance.carried.length<=policy.maximumEditRows&&value.relevance.carried.every((id:unknown)=>Number.isSafeInteger(id)&&(id as number)>=0))"),
 ("edited=await settleMemoEditsV1({root,relevance,edits,changed:classified.changed,listed,index:edit.index,",
  "edited=await settleMemoEditsV1({root,relevance,edits,changed:classified.changed,listed,carried:record.relevance.carried??[],index:edit.index,"),
 ("     atomic(recordFile,JSON.stringify({...served,cursor:formatChangeCursorV1(answer.cursor),relevance:{...served.relevance,stamps}}));",
  "     const {carried:_carried,...relevance}=served.relevance as Record<string,unknown>,carried:readonly number[]=edited?.held?edited.stale:served.relevance.carried??[];\n     atomic(recordFile,JSON.stringify({...served,cursor:formatChangeCursorV1(answer.cursor),relevance:{...relevance,stamps,...(carried.length?{carried}:{})}}));"),
]
S=[
 ('''            "rechecked": {
              "type": "integer",
              "minimum": 0
            },
            "edit": {''','''            "rechecked": {
              "type": "integer",
              "minimum": 0
            },
            "carried": {
              "$comment": "The rows (positions in the fact index of `edit`) that an edit settlement found stale while every derived question they support answered the same: the memo served without a planner, so the sealed generation no longer describes these rows, and every later settlement re-checks them with the rows the journal names (a later change that restores one of their paths alone is then settled against the tree the questions were last asked on). Absent when there is nothing to carry; requires `edit`.",
              "type": "array",
              "minItems": 1,
              "maxItems": 4096,
              "items": {
                "type": "integer",
                "minimum": 0
              }
            },
            "edit": {'''),
]
def run(path,pairs):
  text=path.read_text(encoding="utf8")
  if all(new in text for _,new in pairs):return "applied",None
  if any(new in text for _,new in pairs):return "partial",None
  for old,new in pairs:
    if text.count(old)!=1:return "anchor lost: "+old[:90],None
    text=text.replace(old,new)
  return "base",text
results=[(p,)+run(p,pairs) for p,pairs in ((core,C),(schema,S))]
for p,state,_ in results:print(state,p.name)
if mode=="check":sys.exit(0)
if any(state not in("base","applied") for _,state,_ in results):sys.exit(1)
for p,state,text in results:
  if text is not None:p.write_text(text,encoding="utf8")
print("done")
