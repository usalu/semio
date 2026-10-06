import type {NodeGraphScene} from "@semio-tech/framework";
import {decodeScenePackField,encodePackValue,isPackInteger,type PackValue} from "@semio-tech/framework-os";

export type GraphSceneTransportControlV1 = Readonly<{signal:AbortSignal;maximumInputBytes:number;maximumOutputBytes:number;onProgress:(completed:number,total:number)=>void;yieldContinuation:()=>Promise<void>}>;
const fields=["previewOffJson","lodJson","controlsJson","clustersJson","statusJson","computingJson","capabilitiesJson","hostSnapshotJson"] as const;

/** 🚚️ Projects the Product transport into the eleven declared General Graph scene fields. */
export async function nodeGraphScenePackV1(scene:NodeGraphScene,control:GraphSceneTransportControlV1):Promise<Uint8Array> {
  if(!Number.isSafeInteger(control.maximumInputBytes)||control.maximumInputBytes<0||!Number.isSafeInteger(control.maximumOutputBytes)||control.maximumOutputBytes<0)throw new RangeError("Invalid Graph scene transport limits");
  const payload:Record<string,unknown>={nodes:scene.nodes,edges:scene.edges.map(({id,sourceNodeId,sourcePortId,targetNodeId,targetPortId})=>({id,sourceNodeId,sourcePortId,targetNodeId,targetPortId}))};
  if(scene.viewport!==undefined)payload.viewport=scene.viewport;
  let inputBytes=0,outputBytes=0,units=0;
  const encoder=new TextEncoder();
  const checkpoint=async()=>{control.signal.throwIfAborted();if(++units%32===0){await control.yieldContinuation();control.signal.throwIfAborted();}};
  const append=(text:string,repetitions=1)=>{outputBytes+=encoder.encode(text).length*repetitions;if(outputBytes>control.maximumOutputBytes)throw new RangeError("Graph scene transport output limit exceeded");return text;};
  const json=async(value:PackValue,depth:number):Promise<string>=>{
    await checkpoint();
    if(depth>128)throw new RangeError("Graph scene transport nesting limit exceeded");
    if(isPackInteger(value))return append(value.value.toString());
    if(value===null||typeof value==="boolean"||typeof value==="string")return append(JSON.stringify(value));
    if(typeof value==="number"){if(!Number.isFinite(value))throw new TypeError("Graph scene transport requires finite numbers");return append(JSON.stringify(value));}
    if(Array.isArray(value)){const parts:string[]=[];for(const entry of value)parts.push(await json(entry,depth+1));return append("[")+parts.join(append(",",Math.max(parts.length-1,0)))+append("]");}
    const parts:string[]=[];
    for(const[key,entry]of Object.entries(value)){const encodedKey=append(JSON.stringify(key));parts.push(encodedKey+append(":")+await json(entry,depth+1));}
    return append("{")+parts.join(append(",",Math.max(parts.length-1,0)))+append("}");
  };
  control.onProgress(0,fields.length+1);
  for(let index=0;index<fields.length;index++){
    const name=fields[index]!,value=scene[name];
    if(value!==undefined){inputBytes+=encoder.encode(value).length;if(inputBytes>control.maximumInputBytes)throw new RangeError("Graph scene transport input limit exceeded");control.signal.throwIfAborted();payload[name]=value.startsWith("pk:")?await json(decodeScenePackField(value),0):value;}
    control.onProgress(index+1,fields.length+1);
    await control.yieldContinuation();
    control.signal.throwIfAborted();
  }
  const bytes=encodePackValue(payload);
  if(bytes.length>control.maximumOutputBytes)throw new RangeError("Graph scene transport output limit exceeded");
  control.signal.throwIfAborted();
  control.onProgress(fields.length+1,fields.length+1);
  return bytes;
}
