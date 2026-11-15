import sys,json
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:80])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🩺️attest/🟦️.ts",[
 ('import {nextDelayV1,silentLatencyV1,','import {platformV1} from "../🗺️topology/🟦️.ts";\nimport {nextDelayV1,silentLatencyV1,'),
 ('fallback?:FleetFallbackV1}>;','fallback?:FleetFallbackV1;legacy?:true}>;'),
 ('kind:"git-version"|"volume"|"over-budget"|"symlinks"|"code-fallback";detail:string}>;','kind:"git-version"|"volume"|"over-budget"|"symlinks"|"code-fallback"|"guards-unknown";detail:string}>;'),
 ('''  const value=JSON.parse(text) as unknown;
  requireAttestShapeV1("Attestation",value,"Closed fleet attestation required");
  return (value as FleetAttestationV1).originId===originId?value as FleetAttestationV1:undefined;''','''  const value=olderV1(JSON.parse(text) as unknown);
  const legacy=value.legacy;
  requireAttestShapeV1("Attestation",legacy===undefined?value.attestation:value.attestation,"Closed fleet attestation required");
  const found=value.attestation as FleetAttestationV1;
  return found.originId===originId?legacy===true?{...found,legacy:true}:found:undefined;'''),
 ('/** 👀️ Reads the attestation of one origin;','''const olderV1=(value:unknown):{attestation:unknown;legacy:boolean}=>{
 if(typeof value!=="object"||value===null||Array.isArray(value)||(value as {version?:unknown}).version!==1)return {attestation:value,legacy:false};
 const record=value as Record<string,unknown>;
 if("guards" in record)return {attestation:value,legacy:false};
 return {attestation:{...record,guards:null,..."latency" in record?{}:{latency:silentLatencyV1()}},legacy:true};
};

/** 👀️ Reads the attestation of one origin;'''),
 ('''for(const origin of origins){const fallback=attestations[origin]!.fallback;''','''for(const origin of origins){const item=attestations[origin]!;if(item.legacy===true)warnings.push({originId:origin,kind:"guards-unknown",detail:item.code.slice(0,12)});}
 for(const origin of origins){const fallback=attestations[origin]!.fallback;'''),
 ('platform=options.platform??process.platform','platform=options.platform??platformV1()'),
])
