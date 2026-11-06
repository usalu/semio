import assert from "node:assert/strict";
function run(n,sources,periodic,mutate){
const counts={full:0,sourceReads:0,endpointChecks:0};let changed=false,detected=false;
const io={readSource(){counts.sourceReads++;return changed},checkEndpoint(){counts.endpointChecks++}};
function fence(){counts.full++;for(let i=0;i<sources;i++)if(io.readSource())detected=true}
for(let i=1;i<=n;i++){if(mutate&&i===64)changed=true;if(periodic&&i%64===0)fence();else io.checkEndpoint();if(mutate&&i===65)changed=false}
fence();fence();return{n,sources,periodic,detected,...counts}}
for(const n of [64,256,1024,4096]){const a=run(n,n,true,false),b=run(n,n,false,false);assert.equal(a.full,Math.floor(n/64)+2);assert.equal(a.sourceReads,n*(Math.floor(n/64)+2));assert.equal(b.sourceReads,n*2);console.log(JSON.stringify({current:a,finalOnly:b}))}
const current=run(128,128,true,true),finalOnly=run(128,128,false,true);assert.equal(current.detected,true);assert.equal(finalOnly.detected,false);console.log(JSON.stringify({mutationCounterexample:{currentDetected:current.detected,finalOnlyDetected:finalOnly.detected}}));
