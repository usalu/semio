export function reconcileCargoBodies(text) {
 const owners=[];let current=null,pending=null;
 const settle=status=>{if(pending){pending.status=status;pending=null;}};
 for(const raw of text.split(/\r?\n/)){
  const line=raw.replace(/\u001b\[[0-9;]*m/g,"");
  const owner=line.match(/Running unittests .*\/build\/([^/]+)\/[^/]+\/out\/([^/)]+)\)/);
  if(owner){current={package:owner[1],library:owner[2].replace(/-[a-f0-9]+$/,""),bodies:[],summary:null};owners.push(current);pending=null;continue;}
  if(!current)continue;
  const body=line.match(/^test (\S+) \.\.\.(.*)$/);
  if(body){pending={law:body[1],status:"started"};current.bodies.push(pending);const status=body[2].trim();if(/^(ok|FAILED|ignored)$/.test(status))settle(status);continue;}
  if(/^(ok|FAILED|ignored)$/.test(line.trim())){settle(line.trim());continue;}
  const summary=line.match(/^test result: (\S+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in (\S+)/);
  if(summary)current.summary={status:summary[1],passed:+summary[2],failed:+summary[3],ignored:+summary[4],measured:+summary[5],filtered:+summary[6],elapsed:summary[7]};
 }
 return owners;
}
export function matchAdvertised(owners,triples){return triples.map(triple=>{const owner=owners.find(x=>x.package===triple.package&&x.library===triple.library);const matches=owner?.bodies.filter(x=>x.law===triple.law)??[];return {...triple,status:matches.length===1?matches[0].status:matches.length===0?"not-reached":"duplicate"};});}
