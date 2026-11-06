import {spawn} from 'node:child_process';
import {createInterface} from 'node:readline';
import {readFileSync,appendFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import {dirname,join,resolve} from 'node:path';

const ticket=dirname(import.meta.path),generated=join(ticket,'🗑️generated');
const workspace=join(generated,'w');
mkdirSync(workspace,{recursive:true});
const configured=JSON.parse(readFileSync(resolve('.mcp.json'),'utf8')).mcpServers.semio;
if(configured.command!=='repo')throw Error('semio must use its repository-owned launcher');
const args:string[]=[...configured.args];
const folder=args.indexOf('--folder');
if(folder<0)throw Error('the configured semio launcher has no explicit local workspace');
args[folder+1]=workspace;
if(!args.includes('--no-bridge'))args.push('--no-bridge');
const child=spawn('repo',args,{cwd:process.cwd(),env:process.env,windowsHide:true,stdio:['pipe','pipe','pipe']});
const prefix=join(generated,`semio-${process.pid}`),pending=new Map<number,{resolve:(v:any)=>void,reject:(e:Error)=>void,timer:ReturnType<typeof setTimeout>}>();
let sequence=0,closed=false,nativeReady=false,probeActive=false,stagingFailed=false;
const protocol='2026-07-28';
writeFileSync(prefix+'.launch.json',JSON.stringify({command:'repo',args,workspace,protocol,pid:child.pid}));
child.stderr.on('data',chunk=>appendFileSync(prefix+'.stderr.txt',chunk));
createInterface({input:child.stdout}).on('line',line=>{
    appendFileSync(prefix+'.jsonl',line+'\n');
    let value:any;
    try{value=JSON.parse(line);}catch{appendFileSync(prefix+'.protocol-errors.txt',line+'\n');return;}
    if(value.id!==undefined){const request=pending.get(value.id);if(request){clearTimeout(request.timer);pending.delete(value.id);request.resolve(value);}}
});
const drained=new Promise<void>(resolve=>child.once('close',(code,signal)=>{
    closed=true;
    for(const value of pending.values()){clearTimeout(value.timer);value.reject(Error(`semio exited ${code}/${signal}`));}
    pending.clear();
    writeFileSync(prefix+'.exit.json',JSON.stringify({code,signal,nativeReady}));
    console.log(JSON.stringify({event:'semio-exited',code,signal,nativeReady,saved:prefix+'.exit.json'}));
    resolve();
}));
function request(method:string,params:any={},timeout=30000):Promise<any>{
    if(closed)return Promise.reject(Error('semio is closed'));
    const id=++sequence;
    return new Promise((resolve,reject)=>{
        const timer=setTimeout(()=>{pending.delete(id);child.stdin.write(JSON.stringify({jsonrpc:'2.0',method:'notifications/cancelled',params:{requestId:id,reason:'bounded ticket probe expired'}})+'\n');reject(Error(`semio request ${method} expired`));},timeout);
        pending.set(id,{resolve,reject,timer});
        child.stdin.write(JSON.stringify({jsonrpc:'2.0',id,method,params:{...params,_meta:{...params._meta,'io.modelcontextprotocol/protocolVersion':protocol}}})+'\n');
    });
}
function save(label:string,response:any){const file=prefix+`.${sequence}.${label}.json`;writeFileSync(file,JSON.stringify(response));return file;}
async function probe(){
    if(closed||nativeReady||probeActive||stagingFailed)return;
    probeActive=true;
    try{
        const response=await request('server/discover',{},14400000);
        const file=save('discovery',response),result=response.result;
        nativeReady=!!result&&result.serverInfo?.name==='semio'&&result.serverInfo?.version!=='warming';
        stagingFailed=!!response.error;
        console.log(JSON.stringify({event:'semio-discovery',nativeReady,serverInfo:result?.serverInfo,error:response.error,saved:file}));
        if(nativeReady){const tools=await request('tools/list');const file=save('tools',tools);console.log(JSON.stringify({event:'semio-tools',count:tools.result?.tools?.length,error:tools.error,saved:file}));}
    }catch(error){console.log(JSON.stringify({event:'semio-probe-pending',message:String(error)}));}
    finally{probeActive=false;}
}
const interval=setInterval(()=>void probe(),30000);
const controls=createInterface({input:process.stdin});
function control(line:string){
    if(!line.trim())return;
    void(async()=>{try{const input=JSON.parse(line);if(input.control==='close'){controls.close();child.stdin.end();return;}const response=await request(input.method,input.params,input.timeout??120000);const file=save('response',response);console.log(JSON.stringify({event:'semio-response',id:response.id,error:response.error,isError:response.result?.isError,saved:file}));}catch(error){console.log(JSON.stringify({event:'semio-control-error',message:String(error)}));}})();
}
controls.on('line',control);
const inputFile=join(ticket,'semio-client-input.jsonl');
if(!existsSync(inputFile))writeFileSync(inputFile,'');
let inputOffset=readFileSync(inputFile,'utf8').length;
const inputInterval=setInterval(()=>{
    const text=readFileSync(inputFile,'utf8');
    const end=text.lastIndexOf('\n')+1;
    if(end<=inputOffset)return;
    const incoming=text.slice(inputOffset,end);
    inputOffset=end;
    for(const line of incoming.split('\n'))if(line.trim())control(line);
},500);
console.log(JSON.stringify({event:'semio-started',pid:child.pid,saved:prefix+'.launch.json'}));
await probe();
await drained;
clearInterval(interval);
clearInterval(inputInterval);
controls.close();
