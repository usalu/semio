#!/usr/bin/env python3
import os
F="/Users/ueli/Documents/semio/🧰️framework/"
O=F+"🔨️modules/🏃️process/🔒️leases/🧭️order/"
R=F+"🛍️products/🦑️repo/🔨️modules/📚️library/"
RES=R+"🏗️build/🏡️resident/"
LIVE={
 "order":O+"🟦️.ts","contract":O+"📋️contract/🟦️.ts","oschema":O+"🧬️schema/🔣️.json","policy":O+"🔣️policy.json","registry":O+"🗂️registry/🟦️.ts",
 "failure":RES+"🏛️service/🧯️failure/🟨️.mjs","rschema":RES+"🧬️schema/🔣️.json","speed":R+"🏗️build/🐢️speed/🟨️.mjs",
}
def once(text,old,new,label):
    n=text.count(old)
    assert n==1,(label,n)
    return text.replace(old,new)
GROUND_REASON={"en":"The mapping of a native image is the ground every image operation stands on: its shared readers wait only for a maintenance or retirement writer, and those writers take every other lease without waiting (try-locks), so a task that holds anything may wait for it without closing a cycle; the cycle detector still enforces that, and the wait is bounded by the speed-scaled ground wait bound.","de":"Die Abbildung eines nativen Images ist der Grund, auf dem jede Image-Operation steht: ihre geteilten Leser warten nur auf einen Wartungs- oder Stilllegungsschreiber, und diese Schreiber nehmen jede andere Lease ohne zu warten (Try-Locks); eine Task, die etwas hält, darf deshalb auf sie warten, ohne einen Zyklus zu schließen; der Zyklusdetektor erzwingt das weiterhin, und das Warten ist durch die geschwindigkeitsskalierte Grundwartegrenze begrenzt."}
import json
def edit(key,text):
    if key=="policy":
        text=once(text,'''  "cycle": {
    "intervalMilliseconds": 500
  },''','''  "groundWait": {
    "boundMilliseconds": 1800000
  },
  "cycle": {
    "intervalMilliseconds": 500
  },''',"policy-ground-wait")
        cls='''    {
      "id": "image-mapping",
      "rank": 7,
      "scope": "generation",
      "owner": "native-storage",
      "ground": {
        "reason": %s
      },
      "title": {
        "en": "Native image parent mapping transactions",
        "de": "Abbildungs-Transaktionen der Eltern nativer Images"
      },
      "match": [
        {
          "kind": "prefix",
          "value": "native-storage-mapping:"
        }
      ]
    },
    {
      "id": "bazel-picker",''' % json.dumps(GROUND_REASON,ensure_ascii=False,indent=8).replace("\n","\n      ")
        text=once(text,'''    {
      "id": "bazel-picker",''',cls,"policy-class")
    elif key=="oschema":
        text=once(text,'''        "inert": {
          "type": "boolean"
        },''','''        "inert": {
          "type": "boolean"
        },
        "ground": {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "reason"
          ],
          "properties": {
            "reason": {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "en",
                "de"
              ],
              "properties": {
                "en": {
                  "type": "string",
                  "minLength": 1
                },
                "de": {
                  "type": "string",
                  "minLength": 1
                }
              }
            }
          }
        },''',"schema-ground")
        text=once(text,'''        "cycle": {
          "type": "object",
          "additionalProperties": false,''','''        "groundWait": {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "boundMilliseconds"
          ],
          "properties": {
            "boundMilliseconds": {
              "type": "integer",
              "minimum": 1000,
              "maximum": 86400000
            }
          }
        },
        "cycle": {
          "type": "object",
          "additionalProperties": false,''',"schema-groundwait")
        text=once(text,'''        "blockerHeldSeconds": {
          "type": "integer",
          "minimum": 0
        },''','''        "blockerHeldSeconds": {
          "type": "integer",
          "minimum": 0
        },
        "blockerCommand": {
          "type": "string",
          "maxLength": 240
        },
        "blockerSelf": {
          "type": "boolean"
        },
        "boundMilliseconds": {
          "type": "integer",
          "minimum": 1000
        },
        "speedFactor": {
          "type": "number",
          "minimum": 1
        },''',"schema-violation")
        text=once(text,'''        "selfDeadlock": {
          "en": "Self-deadlock:''','''        "waitBound": {
          "en": "Refusing to wait any longer: {requested} ({requestedMode}) has been unavailable for {seconds} s{blocker}, longer than the {bound} s this machine allows for {requestedClass} (declared {base} s, scaled by the machine speed factor {factor}). This process holds {holds}. The holder has to finish first: run the command again afterwards.",
          "de": "Warten nicht länger zugelassen: {requested} ({requestedMode}) ist seit {seconds} s nicht verfügbar{blocker}, länger als die {bound} s, die dieser Rechner für {requestedClass} zulässt (festgelegt {base} s, mit dem Geschwindigkeitsfaktor {factor} des Rechners skaliert). Dieser Prozess hält {holds}. Zuerst muss der Halter fertig werden: den Befehl danach erneut ausführen."
        },
        "selfDeadlock": {
          "en": "Self-deadlock:''',"schema-waitbound")
        text=once(text,'''        "observed": {''','''        "blockerCommand": {
          "en": ", held by process {pid} ({command}) for {heldSeconds} s",
          "de": ", gehalten von Prozess {pid} ({command}) seit {heldSeconds} s"
        },
        "blockerSelf": {
          "en": ", held by another task of this very process for {heldSeconds} s",
          "de": ", gehalten von einer anderen Task desselben Prozesses seit {heldSeconds} s"
        },
        "blockerUnknown": {
          "en": ", held by no process of the lease registry (a process of older code, or a claim queue ahead)",
          "de": ", gehalten von keinem Prozess der Lease-Registrierung (ein Prozess mit älterem Code oder eine Warteschlange davor)"
        },
        "holdsNothing": {
          "en": "nothing",
          "de": "nichts"
        },
        "observed": {''',"schema-blockers")
    elif key=="contract":
        text=once(text,'inert?:boolean;title:','inert?:boolean;ground?:Readonly<{reason:Readonly<{en:string;de:string}>}>;title:',"contract-class")
        text=once(text,'holdAndWait:Readonly<{graceMilliseconds:number;pollMilliseconds:number}>;cycle:','holdAndWait:Readonly<{graceMilliseconds:number;pollMilliseconds:number}>;groundWait?:Readonly<{boundMilliseconds:number}>;cycle:',"contract-policy")
        text=once(text,'blockerHeldSeconds?:number;cycle?:','blockerHeldSeconds?:number;blockerCommand?:string;blockerSelf?:boolean;boundMilliseconds?:number;speedFactor?:number;cycle?:',"contract-violation")
    elif key=="registry":
        text=once(text,'''holderOf(directory:string,resource:string,mode:LeaseModeV1):Readonly<{pid:number;heldSeconds:number}>|undefined{
  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false)){
   if(entry.pid===process.pid)continue;
   const hold=entry.holds.find(row=>row.resource===resource&&(row.mode==="exclusive"||mode==="exclusive"));
   if(hold)return Object.freeze({pid:entry.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-hold.sinceMs)/1000))});
  }
  return undefined;
 },''','''holderOf(directory:string,resource:string,mode:LeaseModeV1,ownTask?:number):Readonly<{pid:number;heldSeconds:number;command?:string;self?:true}>|undefined{
  const own=source?.snapshot().holds.find(row=>row.directory===directory&&row.resource===resource&&row.task!==ownTask&&(row.mode==="exclusive"||mode==="exclusive"));
  if(own)return Object.freeze({pid:process.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-own.atMs)/1000)),command:command(),self:true as const});
  for(const entry of readLeaseRegistryEntriesV1(rootNow(),directory,Date.now(),false)){
   if(entry.pid===process.pid)continue;
   const hold=entry.holds.find(row=>row.resource===resource&&(row.mode==="exclusive"||mode==="exclusive"));
   if(hold)return Object.freeze({pid:entry.pid,heldSeconds:Math.max(0,Math.floor((Date.now()-hold.sinceMs)/1000)),command:entry.command.slice(0,240)});
  }
  return undefined;
 },''',"registry-holder")
    elif key=="order":
        text=once(text,'''type Request=Readonly<{directory:string;resource:string;mode:LeaseModeV1;nonblocking:boolean;bounded?:boolean}>;''','''type Request=Readonly<{directory:string;resource:string;mode:LeaseModeV1;nonblocking:boolean;bounded?:boolean}>;
type Blocker=Readonly<{pid:number;heldSeconds:number;command?:string;self?:true}>;
let waitScale:()=>number=()=>1;
/** 🐢️ Registers the machine speed factor the ground wait bound is scaled by; the layer that knows the machine (the build library) supplies it, so this module stays free of it. A factor below 1 or not finite counts as 1. */
export function registerLeaseWaitScaleV1(scale:()=>number):void{waitScale=scale;}
/** 🧮️ The wait bound of a ground class in milliseconds: the declared base times the machine speed factor (1 where none is registered), at most 16. */
export function groundWaitBoundV1(policy:LeaseOrderPolicyV1):Readonly<{boundMilliseconds:number;factor:number}>|undefined{
 if(!policy.groundWait)return undefined;
 let factor=1;try{const value=Number(waitScale());if(Number.isFinite(value)&&value>1)factor=Math.min(16,value);}catch{}
 return Object.freeze({boundMilliseconds:Math.ceil(policy.groundWait.boundMilliseconds*factor),factor});
}''',"order-types")
        text=once(text,'''export function checkHoldAndWaitV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number,blocker?:Readonly<{pid:number;heldSeconds:number}>):LeaseViolationV1|undefined{
 if(request.nonblocking||policy.rules.holdAndWait==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 const wanted=classifyLeaseResourceV1(policy,request.resource);
 if(wanted.inert===true)return undefined;''','''export function checkHoldAndWaitV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number,blocker?:Blocker):LeaseViolationV1|undefined{
 if(request.nonblocking||policy.rules.holdAndWait==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 const wanted=classifyLeaseResourceV1(policy,request.resource);
 if(wanted.inert===true||wanted.ground!==undefined)return undefined;''',"order-holdwait-head")
        text=once(text,'''return hold?violation("hold-and-wait",policy,hold,request,{waitedMilliseconds,...blocker?{blockerPid:blocker.pid,blockerHeldSeconds:blocker.heldSeconds}:{}}):undefined;
}''','''return hold?violation("hold-and-wait",policy,hold,request,{waitedMilliseconds,...blockerFields(blocker)}):undefined;
}

const blockerFields=(blocker?:Blocker):Partial<LeaseViolationV1>=>blocker?{blockerPid:blocker.pid,blockerHeldSeconds:blocker.heldSeconds,...blocker.command?{blockerCommand:blocker.command}:{},...blocker.self?{blockerSelf:true}:{}}:{};

/** 🕰️ Decides whether a request for a ground class (the mapping of a native image) that has already waited `waitedMilliseconds` has outlived the declared bound scaled by the machine speed factor: the wait is legal by order and holds nothing against anybody, so it is never a hold-and-wait, but it ends in a typed refusal that names the holder (process, command, age) instead of waiting for ever. Requests that cannot wait are exempt; the oldest hold of this task is named in `held`, or the request itself when it holds nothing. */
export function checkGroundWaitV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number,blocker?:Blocker):LeaseViolationV1|undefined{
 if(request.nonblocking||policy.rules.holdAndWait==="off")return undefined;
 const wanted=classifyLeaseResourceV1(policy,request.resource),bound=wanted.ground===undefined?undefined:groundWaitBoundV1(policy);
 if(!bound||waitedMilliseconds<bound.boundMilliseconds)return undefined;
 const hold=held.filter(row=>!(row.directory===request.directory&&row.resource===request.resource)).sort((left,right)=>left.atMs-right.atMs)[0]??{directory:request.directory,resource:request.resource,mode:request.mode,atMs:Date.now(),task:0};
 return violation("hold-and-wait",policy,hold,request,{waitedMilliseconds,boundMilliseconds:bound.boundMilliseconds,speedFactor:bound.factor,...blockerFields(blocker)});
}''',"order-holdwait-tail")
        text=once(text,'''const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\\{([A-Za-z]+)\\}/gu,(match,key:string)=>key in values?String(values[key]):match);''','''const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\\{([A-Za-z]+)\\}/gu,(match,key:string)=>key in values?String(values[key]):match);
const blockerText=(language:"en"|"de",row:LeaseViolationV1):string=>{
 const catalogue=schema.definitions.Messages.const;
 if(row.blockerPid===undefined)return row.boundMilliseconds===undefined?"":catalogue.blockerUnknown[language];
 const age=row.blockerHeldSeconds??0;
 return row.blockerSelf===true?fill(catalogue.blockerSelf[language],{heldSeconds:age}):row.blockerCommand?fill(catalogue.blockerCommand[language],{pid:row.blockerPid,command:row.blockerCommand,heldSeconds:age}):fill(catalogue.blocker[language],{pid:row.blockerPid,heldSeconds:age});
};''',"order-fill")
        text=once(text,'''const catalogue=schema.definitions.Messages.const,messages={outOfOrder:catalogue.outOfOrder[language],holdAndWait:catalogue.holdAndWait[language],selfDeadlock:catalogue.selfDeadlock[language],cycle:catalogue.cycle[language],blocker:catalogue.blocker[language]},''','''const catalogue=schema.definitions.Messages.const,messages={outOfOrder:catalogue.outOfOrder[language],holdAndWait:row.boundMilliseconds===undefined?catalogue.holdAndWait[language]:catalogue.waitBound[language],selfDeadlock:catalogue.selfDeadlock[language],cycle:catalogue.cycle[language]},''',"order-format-messages")
        text=once(text,'''blocker:row.blockerPid===undefined?"":fill(messages.blocker,{pid:row.blockerPid,heldSeconds:row.blockerHeldSeconds??0})};''','''blocker:blockerText(language,row),bound:Math.floor((row.boundMilliseconds??0)/1000),base:Math.round((row.boundMilliseconds??0)/(row.speedFactor??1)/1000),factor:row.speedFactor??1,holds:row.held===row.requested?catalogue.holdsNothing[language]:shortLeaseResourceV1(row.held)};''',"order-format-values")
        text=once(text,'''  const blocker=waitedMilliseconds>=policy.holdAndWait.graceMilliseconds?leaseRegistryV1.holderOf(request.directory,request.resource,request.mode):undefined;
  const refusal=checkHoldAndWaitV1(policy,current,request,waitedMilliseconds,blocker);
  if(refusal)react(policy,policy.rules.holdAndWait,refusal,request.directory);''','''  const blocker=waitedMilliseconds>=Math.min(policy.holdAndWait.graceMilliseconds,groundWaitBoundV1(policy)?.boundMilliseconds??Infinity)?leaseRegistryV1.holderOf(request.directory,request.resource,request.mode,request.task):undefined;
  const refusal=checkHoldAndWaitV1(policy,current,request,waitedMilliseconds,blocker)??checkGroundWaitV1(policy,current,request,waitedMilliseconds,blocker);
  if(refusal)react(policy,policy.rules.holdAndWait,refusal,request.directory);''',"order-contention")
    elif key=="speed":
        text=once(text,'''import{hostProfileV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";''','''import{hostProfileV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";
import{registerLeaseWaitScaleV1}from"../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";''',"speed-import")
        text=text.rstrip("\n")+'''

registerLeaseWaitScaleV1(()=>localMachineSpeedFactorV1());
'''
    elif key=="failure":
        text=once(text,'''/** 🏷️ The text a failure is logged by:''','''/** 🔒️ The words of a read refused by the lease order (`SEMIO_LEASE_ORDER`): the violation the error carries (what was wanted, what this process held, how long it waited, who held it) as language-neutral fields, or `undefined` for every other failure. @see ../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts */
export function leaseRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 const row=error.code==="SEMIO_LEASE_ORDER"?error.violation:undefined;
 if(row&&typeof row==="object"&&typeof row.requested==="string"&&typeof row.held==="string"){
  const clip=text=>text.length>90?"…"+text.slice(-89):text;
  return{requested:clip(row.requested),mode:row.requestedMode==="exclusive"?"exclusive":"shared",held:row.held===row.requested?"-":clip(row.held),seconds:String(Math.floor((row.waitedMilliseconds??0)/1000)),kind:String(row.kind),holderPid:row.blockerPid===undefined?"?":String(row.blockerPid),holderCommand:String(row.blockerCommand??"?").slice(0,120),holderSeconds:row.blockerHeldSeconds===undefined?"?":String(row.blockerHeldSeconds),bound:row.boundMilliseconds===undefined?"-":String(Math.floor(row.boundMilliseconds/1000))};
 }
 return leaseRefusalOfV1(error.cause,depth-1);
}

/** 🏷️ The text a failure is logged by:''',"failure-func")
        text=once(text,''' const refused=graphMemoryRefusalOfV1(error);''',''' const lease=leaseRefusalOfV1(error);
 if(lease!==undefined){log("lease-refused",JSON.stringify(lease));return{code:"unserved",key:lease.holderPid==="?"?"host-lease-refused-unregistered":"host-lease-refused",fields:lease,retry:"later"};}
 const refused=graphMemoryRefusalOfV1(error);''',"failure-hook")
    elif key=="rschema":
        en='''      "host-lease-refused": "The project graph could not be read on this machine yet: it waited {seconds} s for {requested} ({mode}) while holding {held}, and the lease is held by process {holderPid} ({holderCommand}) for {holderSeconds} s. That process is a cache image operation of this machine (a maintenance, a retirement or a publication); nothing is wrong with the graph. Wait until it has ended and run the command again; the lease status names the holder.",
      "host-lease-refused-unregistered": "The project graph could not be read on this machine yet: it waited {seconds} s for {requested} ({mode}) while holding {held}, and the lease is held by no process of the lease registry (a process of older code on this machine, or a claim queue ahead of this read). Nothing is wrong with the graph. Wait until the holder has ended and run the command again; the lease status names every holder it knows.",
'''
        de='''      "host-lease-refused": "Der Projektgraph konnte auf diesem Rechner noch nicht gelesen werden: er wartete {seconds} s auf {requested} ({mode}), während er {held} hielt, und die Lease wird von Prozess {holderPid} ({holderCommand}) seit {holderSeconds} s gehalten. Dieser Prozess ist eine Cache-Image-Operation dieses Rechners (eine Wartung, eine Stilllegung oder eine Veröffentlichung); am Graphen ist nichts falsch. Warten, bis sie beendet ist, und den Befehl erneut ausführen; der Lease-Status nennt den Halter.",
      "host-lease-refused-unregistered": "Der Projektgraph konnte auf diesem Rechner noch nicht gelesen werden: er wartete {seconds} s auf {requested} ({mode}), während er {held} hielt, und die Lease wird von keinem Prozess der Lease-Registrierung gehalten (ein Prozess mit älterem Code auf diesem Rechner oder eine Warteschlange vor dieser Leseanfrage). Am Graphen ist nichts falsch. Warten, bis der Halter beendet ist, und den Befehl erneut ausführen; der Lease-Status nennt jeden Halter, den er kennt.",
'''
        a='      "host-graph-owner-too-heavy": "The project graph'
        b='      "host-graph-owner-too-heavy": "Der Projektgraph'
        text=once(text,a,en+a,"rschema-en")
        text=once(text,b,de+b,"rschema-de")
    return text
