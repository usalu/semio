import json,re,sys
base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/"
p=base+"🟦️.ts";s=open(p,encoding="utf8").read()
def rep(a,b):
    global s
    assert s.count(a)==1,a[:70]
    s=s.replace(a,b)
rep("import {fetchGraphEntryV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,type","import {fetchGraphEntryV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,tryGraphEntryV1,type")
old=''' const stored=await publishGraphEntryOutcomeV1({client,state:ownerPoolStateV1("manifest",head.id),owner,documents:[{name:P.headName,bytes:encode(head)},{name:P.sessionName,bytes:manifest.session},{name:P.unitsName,bytes:encode(manifest.units)}]});
 if(!stored.stored){
  if(stored.refusal?.status!==409||ownerPoolIdV1(head,manifest.units)!==head.id)return refused("manifest",stored.refusal,stored.timeout);
  const present=await readOwnerPoolV1(client,head.id);
  if(!present||ownerPoolIdV1(present.head,present.units)!==head.id)return refused("manifest-verification",stored.refusal);
 }
'''
new=''' const state=ownerPoolStateV1("manifest",head.id),site=client.settings.endpoint+"\\u0000"+client.settings.instance+"\\u0000"+head.id,intact=async():Promise<boolean>=>{const present=await readOwnerPoolV1(client,head.id);return present!==undefined&&ownerPoolIdV1(present.head,present.units)===head.id;};
 let hold:Awaited<ReturnType<typeof tryGraphEntryV1>>|undefined,found=false;
 if(!WRITTEN.has(site)){
  hold=await tryGraphEntryV1({client,state,owner,expectedMilliseconds:P.manifestExpectedMilliseconds});
  for(let wait=0;hold.outcome==="held"&&!found&&wait<P.manifestWaitPolls;wait++){await new Promise<void>(done=>setTimeout(done,P.manifestPollMilliseconds));found=await intact();}
  if(hold.outcome==="stored")found=await intact();
 }
 if(!found){
  const stored=await publishGraphEntryOutcomeV1({client,state,owner,...hold?.outcome==="granted"?{hold:hold.hold}:{},documents:[{name:P.headName,bytes:encode(head)},{name:P.sessionName,bytes:manifest.session},{name:P.unitsName,bytes:encode(manifest.units)}]});
  if(!stored.stored){
   if(stored.refusal?.status!==409||ownerPoolIdV1(head,manifest.units)!==head.id)return refused("manifest",stored.refusal,stored.timeout);
   if(!await intact())return refused("manifest-verification",stored.refusal);
  }
 }
 WRITTEN.add(site);if(WRITTEN.size>P.wavesKept*8)WRITTEN.delete(WRITTEN.values().next().value as string);
'''
rep(old,new)
rep("/** ⬆️ Publishes the pool of a wave to the fleet cache:","const WRITTEN=new Set<string>();\n\n/** ⬆️ Publishes the pool of a wave to the fleet cache:")
rep("a swap against the pointer read just before, so a wave of another machine is kept and nobody replaces anybody)","a swap against the pointer read just before, so a wave of another machine is kept and nobody replaces anybody; the id of a wave is the digest of its owners, so a manifest that is stored already is verified and never written a second time by another machine: its head, session and cost sheet are the first writer's, a second write would only be refused as a divergence after its upload, and machines that publish the same owners at once take turns by the claim of the manifest state; a machine that wrote the manifest itself renews it by writing it again)")
open(p,"w",encoding="utf8").write(s)
q=base+"🧬️schema/🔣️.json";t=open(q,encoding="utf8").read()
a='        "heldRetryMilliseconds": 15000,\n'
assert t.count(a)==1
t=t.replace(a,a+'        "manifestExpectedMilliseconds": 20000,\n        "manifestPollMilliseconds": 1000,\n        "manifestWaitPolls": 8,\n')
open(q,"w",encoding="utf8").write(t)
json.loads(t)
print("patched")
