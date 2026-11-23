import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {randomUUID} from "node:crypto";
import {chmod,mkdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {hostname,networkInterfaces} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fleetCacheFactsPathV1,requireFleetCacheShapeV1,type FleetCacheFactsV1} from "../🟦️.ts";

/** 📣️ What a publisher knows about its group. */
export type FleetCachePublicationV1=Readonly<{admin:Pick<import("../🪪️identity/🟦️.ts").FleetSignerV1,"role"|"originId">;storeRoot:string;directory:string;group:string;instance:string;endpoints:readonly string[];port:number;host?:string;transport?:"sealed"|"trusted-lan";now?:number}>;


const write=async(path:string,text:string,mode:number):Promise<void>=>{
 const temporary=`${path}.${process.pid}-${randomUUID()}.tmp`;
 await mkdir(dirname(path),{recursive:true,mode:0o755});
 try{await writeFile(temporary,text,{mode});await chmod(temporary,mode).catch(()=>{});await rename(temporary,path);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
};

/** 🌐️ The origins by which workers can reach this server, most preferred first: the given ones (environment), every private IPv4 address of this machine on the serving port, then the fallbacks (the host name of the topology, which may not resolve everywhere). */
export function fleetCacheEndpointsV1(port:number,given:readonly string[]=[],fallback:readonly string[]=[],interfaces:ReturnType<typeof networkInterfaces>=networkInterfaces()):readonly string[]{
 const found:string[]=[];
 for(const rows of Object.values(interfaces))for(const row of rows??[])if(row.family==="IPv4"&&!row.internal&&!row.address.startsWith("169.254.")&&/^(10\.|192\.168\.|172\.(1[6-9]|2[0-9]|3[01])\.)/u.test(row.address))found.push(`http://${row.address}:${port}`);
 const all=[...new Set([...given,...found.sort(),...fallback])].slice(0,8);
 return all.length>0?all:[`http://127.0.0.1:${port}`];
}

/** 🛰️ The identity a cache server publishes under; two servers are the same when directory, port and host agree. */
export type FleetPublisherV1=Readonly<{directory:string;port:number;host:string}>;

/** 🚫 Raised when a publisher must not touch the facts: another identity, a lower generation, or a caller without the admin signing key of the group server. */
export class FleetPublicationRefusedV1 extends Error{readonly reason:string; constructor(reason:string){super(`Fleet cache facts publication refused: ${reason}`);this.reason=reason;}}

const TOPOLOGY="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🔣️.json";

/** 🗺️ What this machine's group says about itself and its peers in the committed topology: its worker network (`SEMIO_FLEET_NETWORK` wins), whether the network is an additional access condition, the ports of its services, the hosts it advertises to other networks, the link to each peer group and the folder where the store of each peer is mounted for this platform. The addresses and ports of the fleet are read here and from the attested facts, never written in code. */
export type FleetGroupViewV1=Readonly<{id:string;network:string|undefined;enforceNetwork:boolean;ports:Readonly<{cache:number;scheduler:number;worker:number}>|undefined;advertise:readonly string[];inGroup:"sealed"|"trusted-lan";links:Readonly<Record<string,Readonly<{transport:"direct"|"relayed";endpoints?:readonly string[];serverKey?:string}>>>;peerStores:Readonly<Record<string,string>>}>;

/** 🔎️ Reads the topology row of the group named by `.cache/f/machine.json`; `undefined` when the machine or the topology is unreadable. */
export async function fleetGroupViewV1(root:string,environment:Readonly<Record<string,string|undefined>>):Promise<FleetGroupViewV1|undefined>{
 try{
  const id=(JSON.parse(await readFile(machinePathV1(root,".cache","f","machine.json"),"utf8")) as {groupId?:string}).groupId,{readTopologyV1,peerStorePathV1}=await import("../../../../../🤝️shared-worktree/🗺️topology/🟦️.ts"),topology=readTopologyV1(JSON.parse(await readFile(join(root,TOPOLOGY),"utf8"))),group=topology.groups.find(row=>row.id===id);
  if(group===undefined)return undefined;
  const named=environment.SEMIO_FLEET_NETWORK;
  return {id:group.id,network:named!==undefined&&named!==""?named:group.server.network,enforceNetwork:group.access?.enforceNetwork===true,ports:group.ports,advertise:group.server.advertise??[],inGroup:group.access?.transport?.inGroup??"sealed",links:group.links??{},peerStores:Object.fromEntries(Object.keys(group.peers).map(peer=>[peer,peerStorePathV1(topology,group,peer)]))};
 }catch{return undefined;}
}

/** 🌐️ The CIDR network of the fleet this machine belongs to: `SEMIO_FLEET_NETWORK`, else the worker network of the topology row of its group. It chooses the interface a server binds to; it is an access condition only when the topology row says `access.enforceNetwork`. */
export async function fleetNetworkV1(root:string,environment:Readonly<Record<string,string|undefined>>):Promise<string|undefined>{
 const named=environment.SEMIO_FLEET_NETWORK;
 if(named!==undefined&&named!=="")return named;
 return (await fleetGroupViewV1(root,environment))?.network;
}

const comparable=(facts:Pick<FleetCacheFactsV1,"group"|"cache"|"auth"|"publisher"|"execution">|undefined):string=>facts===undefined?"":JSON.stringify({group:facts.group,cache:facts.cache,auth:facts.auth,publisher:facts.publisher,execution:facts.execution});
const AUTH_V1={scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]} as const;

type Loose=Readonly<{publisher?:{directory?:string;port?:number;host?:string};generation?:number;publishedAtMs?:number;execution?:FleetCacheFactsV1["execution"]}>;

async function current(storeRoot:string):Promise<{facts:FleetCacheFactsV1|undefined;loose:Loose|undefined;mode:number|undefined}>{
 const path=fleetCacheFactsPathV1(storeRoot);
 try{
  const loose=JSON.parse(await readFile(path,"utf8")) as Loose,mode=(await stat(path)).mode&0o777;
  try{requireFleetCacheShapeV1("Facts",loose,"Closed fleet cache facts required");return {facts:loose as unknown as FleetCacheFactsV1,loose,mode};}catch{return {facts:undefined,loose,mode};}
 }catch{return {facts:undefined,loose:undefined,mode:undefined};}
}

/** 📣️ Publishes `<store>/v1/fleet/cache.json` for the group (only with the admin signing key of the group server, which exists only on that machine's disk and is read by the sidecar the replica daemon supervises): endpoints, instance, the publisher identity (data directory, port, host) and the authentication scheme (signed requests of the origins; no secret travels in the file). It writes only when something but the time changed, and keeps a published `execution` untouched. The generation only grows: it rises whenever the content changes. The admin signing key is the proof, so the supervised server takes over from any earlier publisher at once (the provisioned job of the former design included) and the generation rises above the one in place. A refusal throws `FleetPublicationRefusedV1`; the file stays as it was. */
export async function publishFleetCacheFactsV1(publication:FleetCachePublicationV1):Promise<Readonly<{changed:boolean;facts:FleetCacheFactsV1}>>{
 if(publication.admin.role!=="admin"||publication.admin.originId!==`${publication.group}-server`)throw new FleetPublicationRefusedV1("only the holder of the group server's admin signing key publishes the facts");
 const now=publication.now??Date.now(),publisher:FleetPublisherV1={directory:publication.directory,port:publication.port,host:publication.host??hostname()};
 const {facts:existing,loose,mode}=await current(publication.storeRoot),known=loose?.publisher;void known;
 const draft={group:publication.group,cache:{endpoints:publication.endpoints,instance:publication.instance,access:"read-write" as const,protocols:["bazel-http","nx-self-hosted"] as const,protocolVersion:1 as const,...(publication.transport===undefined?{}:{transport:publication.transport})},auth:AUTH_V1,publisher,...(loose?.execution===undefined?{}:{execution:loose.execution})};
 if(existing!==undefined&&comparable(existing)===comparable(draft)&&mode===0o644)return {changed:false,facts:existing};
 const facts:FleetCacheFactsV1={version:1,group:draft.group,generation:Math.max(1,(loose?.generation??0)+1),publishedAtMs:now,publisher,cache:draft.cache,auth:AUTH_V1,...(draft.execution===undefined?{}:{execution:draft.execution})};
 requireFleetCacheShapeV1("Facts",facts,"Closed fleet cache facts required");
 await write(fleetCacheFactsPathV1(publication.storeRoot),JSON.stringify(facts,null,1)+"\n",0o644);return {changed:true,facts};
}

/** 🧭️ Sets or clears (`null`) the reserved `execution` fact of the published file; the scheduler of the group calls it, the cache server preserves it. Refused while no facts are published. */
export async function setFleetExecutionV1(storeRoot:string,execution:Readonly<{endpoint:string;protocolVersion:number}>|null,now:number=Date.now()):Promise<FleetCacheFactsV1> {
 const {facts}=await current(storeRoot);
 if(facts===undefined)throw Error("Published fleet cache facts required before the execution fact");
 const {execution:_,...rest}=facts,next:FleetCacheFactsV1={...rest,publishedAtMs:now,...(execution===null?{}:{execution})};
 requireFleetCacheShapeV1("Facts",next,"Closed fleet cache facts required");
 if(comparable(next)===comparable(facts))return facts;
 await write(fleetCacheFactsPathV1(storeRoot),JSON.stringify(next,null,1)+"\n",0o644);return next;
}
