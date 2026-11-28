import {posix} from "node:path";

const ABSOLUTE_V1=/^(?:\/|[A-Za-z]:\/)/u;
const unified=(path:string):string=>path.replaceAll("\\","/");

/** 🧭️ The absolute paths a `resolve` observation row consulted, or undefined when the row must always be probed: `[]` for a `node:` builtin; for a relative or absolute request the directory of the target and the target itself (both lexically, with the resolved answer required to lie there, so a link that leads elsewhere is never vouched); a failure answer (`!CODE`) consults the same two paths. Bare specifiers, `#` imports and every other request are undefined.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🟦️.ts */
export function resolveConsultedPathsV1(key:string,fact:unknown):readonly string[]|undefined{
  let parsed:unknown;
  try{parsed=JSON.parse(key);}catch{return undefined;}
  if(!Array.isArray(parsed)||parsed.length!==2||typeof parsed[0]!=="string"||typeof parsed[1]!=="string"||typeof fact!=="string")return undefined;
  const from=unified(parsed[0]),request=unified(parsed[1]);
  if(request.startsWith("node:"))return fact===request?[]:undefined;
  if(!ABSOLUTE_V1.test(from))return undefined;
  const relative=request==="."||request===".."||request.startsWith("./")||request.startsWith("../");
  if(!relative&&!ABSOLUTE_V1.test(request))return undefined;
  const target=relative?posix.join(posix.dirname(from),request):posix.normalize(request).replace(/(.)\/+$/u,"$1"),folder=posix.dirname(target);
  if(!fact.startsWith("!")){
    const answer=unified(fact);
    if(!ABSOLUTE_V1.test(answer)||!answer.startsWith(target)||(posix.dirname(answer)!==folder&&!answer.startsWith(target+"/")))return undefined;
  }
  return folder===target?[target]:[folder,target];
}
