import os,sys,tempfile,re
P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts"
s=open(P,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:60],s.count(old))
    s=s.replace(old,new)
def cut(start,end,new):
    global s
    a=s.index(start);b=s.index(end,a)
    s=s[:a]+new+s[b:]
rep('import {createServer} from "node:net";\nimport type {networkInterfaces} from "node:os";','import {spawn as spawnProcess} from "node:child_process";\nimport {createConnection,createServer} from "node:net";\nimport {networkInterfaces} from "node:os";')
cut('/** 📇️ The addresses a machine publishes','/** ⏱️ How long a published set','''/** 📇️ The addresses a machine publishes in its origin record: identified by the origin id, never by an address and never by the host name of the operating system; `name` is the fleet name `semio-<origin>` derived from the origin. */
export type FleetAddressSetV1=Readonly<{version:1;origin:string;name:string;addresses:readonly FleetAddressV1[];observedAtMs:number;ttlMs:number;generation:number}>;

''')
rep('const NAME=/^[a-z0-9][a-z0-9-]{0,62}$/u;','''const ORIGIN=/^[a-z][a-z0-9-]{0,63}$/u;

/** 🏷️ The DNS-SD service type under which every machine advertises itself (Bonjour on macOS and Windows, avahi on Linux): one instance per origin, named by the origin id. */
export const FLEET_SERVICE_TYPE_V1="_semio-fleet._tcp";

/** 🪪️ The fleet name of an origin: `semio-<origin>`, semantic and unique, derived from the origin id and never from the host name of the operating system. */
export function fleetNameV1(origin:string):string{
 if(!ORIGIN.test(origin))throw Error(`Declared fleet origin required: ${origin}`);
 return `semio-${origin}`;
}''')
cut('/** 🪪️ The multicast DNS name of a host','/** 🔎️ Every address a machine','')
rep("return JSON.stringify([set.mdns,set.addresses.map(row=>[row.interface,row.address,row.prefix])]);","return JSON.stringify(set.addresses.map(row=>[row.interface,row.address,row.prefix]));")
rep('export function fleetAddressSignatureV1(set:Pick<FleetAddressSetV1,"mdns"|"addresses">)','export function fleetAddressSignatureV1(set:Pick<FleetAddressSetV1,"addresses">)')
rep("/** 🧾️ The identity of what a set says (addresses and name, never the time)","/** 🧾️ The identity of what a set says (the addresses, never the time)")
rep("input:Readonly<{origin:string;hostname:string;interfaces:ReturnType<typeof networkInterfaces>;now:number;generation?:number;ttlMs?:number}>):FleetAddressSetV1{\n return {version:1,origin:input.origin,hostname:input.hostname,mdns:fleetMdnsNameV1(input.hostname),addresses:","input:Readonly<{origin:string;interfaces:ReturnType<typeof networkInterfaces>;now:number;generation?:number;ttlMs?:number}>):FleetAddressSetV1{\n return {version:1,origin:input.origin,name:fleetNameV1(input.origin),addresses:")
rep('["addresses","generation","hostname","mdns","observedAtMs","origin","ttlMs","version"]','["addresses","generation","name","observedAtMs","origin","ttlMs","version"]')
rep(''' if(typeof set.hostname!=="string"||set.hostname==="")return fail("hostname");
 if(set.mdns!==null&&(typeof set.mdns!=="string"||!/^[a-z0-9][a-z0-9-]{0,62}\\.local$/u.test(set.mdns)))return fail("mdns");
''',''' if(set.name!==`semio-${set.origin}`)return fail("name semio-<origin>");
''')
rep("origin:string;hostname:string;interfaces:ReturnType<typeof networkInterfaces>;now?:number;ttlMs?:number}>):Promise<Readonly<{changed","origin:string;interfaces:ReturnType<typeof networkInterfaces>;now?:number;ttlMs?:number}>):Promise<Readonly<{changed")
rep("origin:input.origin,hostname:input.hostname,interfaces:input.interfaces,now,","origin:input.origin,interfaces:input.interfaces,now,")
rep("&&previous.set.hostname===draft.hostname&&previous.set.ttlMs","&&previous.set.ttlMs")
rep("It writes only when the addresses or the name changed","It writes only when the addresses changed")
rep("origin:string;hostname:string;interfaces:()=>ReturnType","origin:string;interfaces:()=>ReturnType")
rep("origin:input.origin,hostname:input.hostname,interfaces:input.interfaces(),","origin:input.origin,interfaces:input.interfaces(),")
rep('if(result.set.addresses.length===0&&result.set.mdns!==null)input.report(fleetAddressTextV1(input.language,"addresses.none",{origin:input.origin,name:result.set.mdns}));','if(result.set.addresses.length===0)input.report(fleetAddressTextV1(input.language,"addresses.none",{origin:input.origin,service:FLEET_SERVICE_TYPE_V1}));')
# candidates + connection + resolve
cut('/** 🧭️ The endpoints to try for one service of a peer','/** 📌️ What `connectFleetEndpointV1` found','''/** 🧭️ The endpoints to try for one service of a peer from what is known without asking the network, most promising first: the last good endpoint, then the published addresses ordered by `orderFleetAddressesV1`. */
export function fleetEndpointCandidatesV1(input:Readonly<{set:FleetAddressSetV1|undefined;local:readonly FleetAddressV1[];port:number;scheme?:"http"|"https";lastGood?:string}>):readonly string[]{
 const scheme=input.scheme??"http",url=(host:string):string=>`${scheme}://${host}:${input.port}`;
 const all=[...new Set(input.set===undefined?[]:orderFleetAddressesV1(input.set.addresses,input.local).map(row=>url(fleetUrlHostV1(row.address))))];
 return input.lastGood!==undefined&&all.includes(input.lastGood)?[input.lastGood,...all.filter(endpoint=>endpoint!==input.lastGood)]:all;
}

''')
rep("/** 📌️ What `connectFleetEndpointV1` found: the first endpoint that answered and every attempt made on the way. */\nexport type FleetConnectionV1=Readonly<{endpoint:string;attempts:","/** 🛤️ By which path an endpoint was found: `last-good` (remembered from the previous run), `published` (addresses in the shared store), `mirror` (the local copy of those addresses while the store is unreadable) or `dns-sd` (service discovery on the local network). */\nexport type FleetResolutionPathV1=\"last-good\"|\"published\"|\"mirror\"|\"dns-sd\";\n\n/** 📌️ What `connectFleetEndpointV1` found: the first endpoint that answered and every attempt made on the way. */\nexport type FleetConnectionV1=Readonly<{endpoint:string;attempts:")
a=s.index('/** 🧭️ Resolves and connects one service')
s=s[:a]+open(os.path.join(os.path.dirname(__file__),"addresses-tail.ts"),encoding="utf8").read()
t=tempfile.NamedTemporaryFile("w",dir=os.path.dirname(P),delete=False,encoding="utf8",suffix=".tmp")
t.write(s);t.close();os.rename(t.name,P);print("ok")
