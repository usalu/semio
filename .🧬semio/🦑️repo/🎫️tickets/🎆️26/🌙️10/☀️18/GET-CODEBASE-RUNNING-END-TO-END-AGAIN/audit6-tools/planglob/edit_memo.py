"""Anchored edit of the Cargo planning memo: a workspace whose members Cargo expands from a glob is never remembered. Usage: edit_memo.py <root> [check|apply]."""
import pathlib,sys
root=pathlib.Path(sys.argv[1]);mode=sys.argv[2] if len(sys.argv)>2 else "apply"
d=root/"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🧠️memo"
code=d/"🟦️.ts";schema=d/"🧬️schema/🔣️.json"
C=[
 ('import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";\n',
  'import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";\nimport {runtimeParseTomlV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";\n'),
 ('/** 🗺️ Names every physical question Cargo answered the metadata from: the workspace manifest and lock, each local package\'s manifest, its target auto-discovery paths (files, directories and their nested `main.rs`), and the manifest of every ancestor directory Cargo\'s workspace search reads (up to the workspace root for its members, up to the filesystem root for a path package outside it). */\nexport async function cargoPlanQuestionsV1(metadata:unknown):Promise<readonly Omit<CargoPlanInputV1,"answer">[]>{',
  '/** 🗺️ Names every physical question Cargo answered the metadata from: the workspace manifest and lock, each local package\'s manifest, its target auto-discovery paths (files, directories and their nested `main.rs`), and the manifest of every ancestor directory Cargo\'s workspace search reads (up to the workspace root for its members, up to the filesystem root for a path package outside it). `undefined` when the workspace manifest cannot be read or Cargo expands its members from a glob: a new member then moves no recorded text, and the lock receipt only re-reads the manifests it recorded.\n * @see https://doc.rust-lang.org/cargo/reference/workspaces.html#the-members-and-exclude-fields */\nexport async function cargoPlanQuestionsV1(metadata:unknown):Promise<readonly Omit<CargoPlanInputV1,"answer">[]|undefined>{'),
 (' const workspace=facts.workspace_root;\n',
  ' const workspace=facts.workspace_root;\n let declared:{members?:unknown;"default-members"?:unknown}|undefined;try{declared=(runtimeParseTomlV1(await readFile(join(workspace,protocol.manifest),"utf8")) as {workspace?:typeof declared}).workspace;}catch{return undefined;}\n if([declared?.members,declared?.["default-members"]].some(list=>Array.isArray(list)&&list.some(member=>typeof member!=="string"||protocol.memberGlob.some(character=>member.includes(character)))))return undefined;\n'),
 ('   const inputs=await cargoPlanAnswersV1(await cargoPlanQuestionsV1(answers.metadata),startedMs,control);if(!inputs)return;',
  '   const questions=await cargoPlanQuestionsV1(answers.metadata),inputs=questions&&await cargoPlanAnswersV1(questions,startedMs,control);if(!inputs)return;'),
]
S=[('        "lock": "Cargo.lock",\n','        "lock": "Cargo.lock",\n        "memberGlob": ["*", "?", "["],\n')]
def run(path,pairs):
  text=path.read_text(encoding="utf8")
  if all(new in text for _,new in pairs):return "applied",None
  for old,new in pairs:
    if text.count(old)!=1:return "anchor lost: "+old[:80],None
    text=text.replace(old,new)
  return "base",text
res=[(p,)+run(p,pairs) for p,pairs in ((code,C),(schema,S))]
for p,s,_ in res:print(s,p.name)
if mode=="check":sys.exit(0)
if any(s not in("base","applied") for _,s,_ in res):sys.exit(1)
for p,s,t in res:
  if t is not None:p.write_text(t,encoding="utf8")
print("done")
