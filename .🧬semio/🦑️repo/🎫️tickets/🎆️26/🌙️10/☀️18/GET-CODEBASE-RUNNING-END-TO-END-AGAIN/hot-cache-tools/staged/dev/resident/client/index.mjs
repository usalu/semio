/** 🚪 Thin client of the resident service: connects, forwards one command, streams its output, and tells the caller to fall back in-process whenever the service cannot answer. */
import {connect} from "node:net";
import {lstatSync,readFileSync} from "node:fs";
import {endpointsV1} from "../protocol/endpoint.mjs";
import {createFrameDecoderV1,encodeFrameV1} from "../protocol/frames.mjs";
import {mintCapabilityV1} from "../protocol/capability.mjs";

/** 🔐 True when the runtime directory is private and ours; a hijacked predictable path must never receive a capability. */
function trustedDirectory(path){
 if(process.platform==="win32")return true;
 try{const status=lstatSync(path);return status.isDirectory()&&status.uid===process.getuid()&&(status.mode&0o077)===0;}catch{return false;}
}

/** 🎫 The default token of a local launcher: everything but administration, for one minute. */
export function launcherCapabilityV1(key,now=Date.now()){
 return mintCapabilityV1(key,[{k:"scope",v:["status","query","run","watch"]},{k:"notAfter",v:now+60000}]);
}

/** 🚪 Forwards one command. Resolves `{handled:false,reason}` when the caller must run the command itself, otherwise `{handled:true,exit}`. */
export function forwardCommandV1(options){
 const{root,command,args,cwd,languages,lane="default",client="repo",env={},cap,stdout=process.stdout,stderr=process.stderr,environment=process.env,platform=process.platform,deadlineMs,signal,progress=false}=options;
 const runtime=endpointsV1(root,environment,platform);
 return new Promise(resolve=>{
  if(!trustedDirectory(runtime.directory)){resolve({handled:false,reason:"down"});return;}
  let token=cap;
  if(token===undefined){try{token=launcherCapabilityV1(readFileSync(runtime.key));}catch{resolve({handled:false,reason:"key"});return;}}
  const socket=connect(runtime.endpoint);
  let accepted=false,wroteOutput=false,finished=false,welcome;
  const timing={startedAt:performance.now()};
  const finish=value=>{if(!finished){finished=true;socket.destroy();resolve({...value,timing});}};
  const decoder=createFrameDecoderV1({maximumFrameBytes:16<<20,
   onFrame:frame=>{
    switch(frame.t){
     case"welcome":welcome=frame;timing.welcomeAt=performance.now();break;
     case"accepted":accepted=true;timing.acceptedAt=performance.now();break;
     case"progress":if(progress&&frame.message){const text=frame.message[languages[0]]??Object.values(frame.message)[0];stderr.write(text+"\n");}break;
     case"output":wroteOutput=true;(frame.stream==="stderr"?stderr:stdout).write(frame.text??Buffer.from(frame.b64,"base64"));break;
     case"result":timing.resultAt=performance.now();finish({handled:true,exit:frame.exit,served:frame.served,serviceTiming:frame.timing,welcome});break;
     case"refused":{
      const text=frame.message[languages[0]]??frame.message.en;
      if(frame.retry==="fallback"){finish({handled:false,reason:frame.code});break;}
      stderr.write(text+"\n");finish({handled:true,exit:frame.retry==="later"?75:1,refused:frame.code});break;
     }
     default:break;
    }
   },
   onError:()=>finish({handled:false,reason:"protocol"}),
  });
  socket.on("connect",()=>{
   timing.connectedAt=performance.now();
   const lines=[encodeFrameV1({t:"hello",v:1,client:{name:client,pid:process.pid,lane},languages,cap:token}),encodeFrameV1({t:"request",id:1,command,args,cwd,env,...(deadlineMs?{deadlineMs}:{})})];
   socket.write(lines.join(""));
  });
  socket.on("data",chunk=>decoder.push(chunk));
  socket.on("end",()=>socket.destroy());
  socket.on("error",error=>finish(accepted||wroteOutput?{handled:true,exit:70,lost:true}:{handled:false,reason:error?.code==="ENOENT"||error?.code==="ECONNREFUSED"?"down":"error"}));
  socket.on("close",()=>finish(accepted||wroteOutput?{handled:true,exit:70,lost:true}:{handled:false,reason:"closed"}));
  signal?.addEventListener("abort",()=>{try{socket.write(encodeFrameV1({t:"cancel",id:1}));}catch{}},{once:true});
 });
}
