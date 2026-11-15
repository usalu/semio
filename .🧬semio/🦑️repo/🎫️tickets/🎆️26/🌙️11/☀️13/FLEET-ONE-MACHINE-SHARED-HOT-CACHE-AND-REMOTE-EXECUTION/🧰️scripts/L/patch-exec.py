LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
RE=LIB+"🌐️remote-execution/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)

# --- grpc
edit(RE+'📡️wire/🚪️grpc/🟦️.ts',[
('export type GrpcCallV1=Readonly<{method:string;metadata:Readonly<Record<string,string>>;signal:AbortSignal;peer:string}>;','export type GrpcCallV1=Readonly<{method:string;metadata:Readonly<Record<string,string>>;signal:AbortSignal;peer:string;principal?:string}>;'),
('authorize?:(call:Readonly<{method:string;metadata:Readonly<Record<string,string>>}>)=>void;','authorize?:(call:Readonly<{method:string;metadata:Readonly<Record<string,string>>;peer:string}>)=>void|Readonly<{principal:string}>|Promise<void|Readonly<{principal:string}>>;'),
('  options.authorize?.({method:key,metadata});','  const peer=`${stream.session?.socket?.remoteAddress??"unknown"}`,verdict=await options.authorize?.({method:key,metadata,peer});'),
('const call:GrpcCallV1=Object.freeze({method:key,metadata,signal:controller.signal,peer:`${stream.session?.socket?.remoteAddress??"unknown"}`}),','const call:GrpcCallV1=Object.freeze({method:key,metadata,signal:controller.signal,peer,...(verdict?.principal===undefined?{}:{principal:verdict.principal})}),'),
('export function connectGrpcV1(url:string,options:Readonly<{certificateAuthority?:string}>={}):GrpcClientV1{','export function connectGrpcV1(url:string,options:Readonly<{certificateAuthority?:string;sign?:(method:string)=>Record<string,string>}>={}):GrpcClientV1{'),
('"grpc-accept-encoding":"identity",...(call?.timeoutMilliseconds?{"grpc-timeout":`${call.timeoutMilliseconds}m`}:{}),...(call?.metadata??{})},','"grpc-accept-encoding":"identity",...(call?.timeoutMilliseconds?{"grpc-timeout":`${call.timeoutMilliseconds}m`}:{}),...(options.sign?.(method)??{}),...(call?.metadata??{})},'),
])
# fix: the original used 'serve' code with 'options.authorize?.(...)' preceded by a statement; ensure 'const peer' not duplicated
# --- content
edit(RE+'🗄️content/🟦️.ts',[
('export type RemoteExecutionHttpContentOptionsV1=Readonly<{endpoint:string;token?:string|undefined;concurrency?:number;timeoutSeconds?:number;retries?:number}>;','export type RemoteExecutionHttpContentOptionsV1=Readonly<{endpoint:string;token?:string|undefined;fetch?:typeof fetch;concurrency?:number;timeoutSeconds?:number;retries?:number}>;'),
('    const response=await fetch(target,{method,...init,headers:','    const response=await(options.fetch??fetch)(target,{method,...init,headers:'),
])
# --- worker
edit(RE+'👷️worker/🟦️.ts',[
('import {selectRemoteExecutionSandboxV1,type RemoteExecutionSandboxStrategyV1} from "../🔒️sandbox/🟦️.ts";','import {selectRemoteExecutionSandboxV1,type RemoteExecutionSandboxStrategyV1} from "../🔒️sandbox/🟦️.ts";\nimport {validateRemoteExecutionRustcActionV1,type RemoteExecutionActionPolicyV1} from "../🧱️actions/🟦️.ts";'),
(' scheduler:string;token?:string|undefined;cacheToken?:string|undefined;name?:string;',' scheduler:string;token?:string|undefined;cacheToken?:string|undefined;fetch?:typeof fetch|undefined;cacheFetch?:typeof fetch|undefined;actions?:RemoteExecutionActionPolicyV1;name?:string;'),
('network?:boolean;tools?:RemoteExecutionToolsV1;\n}>;','network?:boolean;tools?:RemoteExecutionToolsV1;actions?:RemoteExecutionActionPolicyV1;\n}>;'),
('''  if(command.arguments.length===0)return fail(3,"Command without arguments");''','''  if(command.arguments.length===0)return fail(3,"Command without arguments");
  if(run.actions!=="any"){
   const refusal=run.actions==="none"?"this worker runs no actions":validateRemoteExecutionRustcActionV1(command);
   if(refusal!==undefined)return fail(9,`Action refused by the worker policy: ${refusal}`);
  }'''),
('function schedulerClient(base:string,token:string|undefined){','function schedulerClient(base:string,token:string|undefined,call:typeof fetch=fetch){'),
('  const response=await fetch(root+path,{method,signal:AbortSignal.any([signal,AbortSignal.timeout(timeout)]),headers:','  const response=await call(root+path,{method,signal:AbortSignal.any([signal,AbortSignal.timeout(timeout)]),headers:'),
(' const call=schedulerClient(options.scheduler,options.token),say=',' const call=schedulerClient(options.scheduler,options.token,options.fetch),say='),
(' if(options.confinement==="required"&&confinement==="none")throw Error(`Confinement required but this host has no confining sandbox (only ${sandbox.name})`);',' if(options.confinement==="required"&&confinement==="none")throw Error(`Confinement required but this host has no confining sandbox (only ${sandbox.name})`);\n const actions:RemoteExecutionActionPolicyV1=options.actions??"rustc";\n if(actions==="any"&&confinement==="none")throw Error(`Running arbitrary actions needs a confining sandbox; this host has only ${sandbox.name}. Use actions "rustc" or "none".`);'),
('     content=httpRemoteExecutionContentV1({endpoint:url,token:options.cacheToken});','     content=httpRemoteExecutionContentV1({endpoint:url,token:options.cacheToken,...(options.cacheFetch===undefined?{}:{fetch:options.cacheFetch})});'),
('network:network==="allowed",materialize:options.materialize,','network:network==="allowed",actions,materialize:options.materialize,'),
])
# --- reporter in load
edit(RE+'🌡️load/🟦️.ts',[
('export async function runRemoteExecutionReporterV1(options:Readonly<{scheduler:string;token?:string|undefined;intervalSeconds','export async function runRemoteExecutionReporterV1(options:Readonly<{scheduler:string;token?:string|undefined;fetch?:typeof fetch|undefined;intervalSeconds'),
('   const response=await fetch(url,{method:"POST",signal:AbortSignal.any([options.signal,AbortSignal.timeout(options.timeoutMilliseconds??5000)]),','   const response=await(options.fetch??fetch)(url,{method:"POST",signal:AbortSignal.any([options.signal,AbortSignal.timeout(options.timeoutMilliseconds??5000)]),'),
])
edit(RE+'🛰️agent/🟦️.ts',[('runRemoteExecutionReporterV1({scheduler:options.scheduler,token:options.token,intervalSeconds','runRemoteExecutionReporterV1({scheduler:options.scheduler,token:options.token,fetch:options.fetch,intervalSeconds')])
print("exec part 1 ok")
