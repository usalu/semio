import{isBuiltin}from"node:module";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import { isRuntimeBuiltinSpecifierV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { TranspilerV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🔬️source/🟦️.ts";
const policy=schema.$defs.Policy.const,messages=schema.$defs.Messages.const;
/** 🧭️ One literal module specifier a source file asks for; a lenient site is derived from text rather than the scanner, so a miss is not fatal. */
export interface ClosureLiteralSiteV1{readonly kind:"static"|"dynamic"|"require"|"resolve"|"create-require";readonly specifier:string;readonly lenient:boolean;readonly scanned:boolean}
/** 🧩️ The text codes a refusal can carry. */
export type ClosureRefusalCodeV1=keyof typeof messages;
const quoted=String.raw`(?<q>["'\x60])(?<lit>[^"'\x60\\\r\n$]+)\k<q>`,creatorNames=(source:string)=>["createRequire",...[...source.matchAll(/createRequire\s+as\s+([A-Za-z_$][\w$]*)/gu)].map(match=>match[1]!.replaceAll("$","\\$"))].join("|"),created=(names:string)=>new RegExp(String.raw`(?<![\w$.])(?:${names})\s*\((?:[^()]|\((?:[^()]|\([^()]*\))*\))*\)\s*(?<resolve>\.resolve)?\s*\(\s*${quoted}\s*\)`,"gu"),bound=(names:string)=>new RegExp(String.raw`(?<![\w$.])([A-Za-z_$][\w$]*)\s*=\s*(?:[\w$]+\.)?(?:${names})\s*\(`,"gu"),meta=new RegExp(String.raw`import\.meta\.resolve\s*\(\s*${quoted}\s*\)`,"gu");
const namedCall=(name:string)=>new RegExp(String.raw`(?<![\w$.])${name.replaceAll("$","\\$")}\s*(?<resolve>\.resolve)?\s*\(\s*${quoted}\s*\)`,"gu");
/** 🔎️ Lists every literal import, dynamic import, require, createRequire(...)("x"), require.resolve and import.meta.resolve specifier of one source file. */
export function closureLiteralSitesV1(path:string,source:string):readonly ClosureLiteralSiteV1[]{
 const rows=new Map<string,ClosureLiteralSiteV1>(),add=(kind:ClosureLiteralSiteV1["kind"],specifier:string,lenient:boolean,scanned=false)=>{if(!specifier||rows.size>=policy.maximumSitesPerFile&&!rows.has(specifier))return;const earlier=rows.get(specifier);if(!earlier||earlier.lenient&&!lenient||!earlier.scanned&&scanned)rows.set(specifier,Object.freeze({kind,specifier,lenient:lenient&&(earlier?.lenient??true),scanned:scanned||earlier?.scanned===true}));};
 const kinds={"import-statement":"static","dynamic-import":"dynamic","require-call":"require","require-resolve":"resolve","import-rule":"static"} as const,scanner=new TranspilerV1({loader:/\.[cm]?tsx?$/u.test(path)?"ts":"js",target:"node",define:{"import.meta.vitest":"undefined"}});
 for(const row of /\.c(?:js|ts)$/u.test(path)?scanner.scanImports(source):scanner.scan(source).imports)add(kinds[row.kind as keyof typeof kinds]??"static",row.path,false,true);
 if(source.includes("createRequire")){const creators=creatorNames(source);for(const match of source.matchAll(created(creators)))add(match.groups!.resolve?"resolve":"create-require",match.groups!.lit!,!/^[./]/u.test(match.groups!.lit!));const names=new Set(["require"]);for(const match of source.matchAll(bound(creators)))names.add(match[1]!);for(const name of names)for(const match of source.matchAll(namedCall(name)))add(match.groups!.resolve?"resolve":"require",match.groups!.lit!,true);}
 else if(source.includes("require"))for(const match of source.matchAll(namedCall("require")))add(match.groups!.resolve?"resolve":"require",match.groups!.lit!,true);
 if(source.includes("import.meta.resolve"))for(const match of source.matchAll(meta))add("resolve",match.groups!.lit!,true);
 return Object.freeze([...rows.values()]);
}
/** 🏷️ Names how a specifier is addressed: a runtime builtin, a relative or absolute path, another URL scheme, or a package. */
export function closureSpecifierKindV1(specifier:string):"builtin"|"relative"|"absolute"|"scheme"|"package"{return specifier.startsWith("node:")||isRuntimeBuiltinSpecifierV1(specifier)||isBuiltin(specifier)?"builtin":specifier.startsWith(".")?"relative":specifier.startsWith("/")||/^[A-Za-z]:[\\/]/u.test(specifier)?"absolute":/^[a-z][a-z0-9+.-]*:/iu.test(specifier)?"scheme":"package";}
/** 📦️ Reads the package name of a bare specifier, including its scope. */
export function closurePackageNameV1(specifier:string):string{const parts=specifier.split("/");return specifier.startsWith("@")?parts.slice(0,2).join("/"):parts[0]!;}
/** 🗣️ Renders one refusal in the requested language, or in every language when none is resolved; self-contained so loaders can embed its source. */
export function closureRefusalTextV1(table:Readonly<Record<string,Readonly<Record<string,Readonly<{refusal:string;fix:string}>>>>>,code:string,fields:Readonly<Record<string,string|null|undefined>>,environment:Readonly<Record<string,string|undefined>>):string{
 const row=table[code];if(!row)return code+": "+JSON.stringify(fields);const tag=[environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG].map(value=>value?.trim().toLowerCase().split(/[._@-]/u)[0]).find(value=>value&&row[value]),languages=tag?[tag]:Object.keys(row),fill=(text:string)=>text.replace(/\{(\w+)\}/gu,(_all,key:string)=>String(fields[key]??"?"));
 return languages.map(language=>fill(row[language]!.refusal)+". "+fill(row[language]!.fix)).join(" | ");
}
/** 🧾️ Binds the refusal texts of this repository to the process environment. */
export function closureRefusalV1(code:ClosureRefusalCodeV1,fields:Readonly<Record<string,string|null|undefined>>,environment:Readonly<Record<string,string|undefined>>=process.env):Error{return Error(closureRefusalTextV1(messages,code,fields,environment));}
/** 🧬️ The refusal text table the loaders embed. */
export const closureRefusalMessagesV1=messages;
/** 📍️ Where the refusal catalog sits, relative to the repository root: a snapshot that carries this file renders every refusal after its authentication from it, so a loader never inlines the catalog and its command line stays bounded however the catalog grows; only the refusals raised while the snapshot itself is read, before anything is authenticated, are machine tokens. */
export const closureRefusalCatalogPathV1=policy.catalogPath;
/** 🗃️ Reads the refusal table out of the base64 bytes of the catalog file as a snapshot row carries them, once their declared hash holds; undefined for anything else, so the refusal then names its code and fields; self-contained so loaders can embed its source. */
export function closureRefusalCatalogTableV1(encoded:string|undefined,sha256:string|undefined,hash:(bytes:Uint8Array)=>string):Readonly<Record<string,Readonly<Record<string,Readonly<{refusal:string;fix:string}>>>>>|undefined{
 try{const bytes=Buffer.from(encoded??"","base64");if(!encoded||bytes.toString("base64")!==encoded||hash(bytes)!==sha256)return undefined;const table=JSON.parse(bytes.toString("utf8"))?.$defs?.Messages?.const;return table&&typeof table==="object"?table:undefined;}catch{return undefined;}
}
/** 🔖️ The target schemes of derived import-map entries. */
export const closureTargetSchemesV1=Object.freeze({installed:policy.installedScheme,root:policy.rootScheme});
