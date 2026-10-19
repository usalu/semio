const nxRoot=process.argv[2];
const native=require(nxRoot+"/dist/src/native");
const {transformProjectGraphForRust}=require(nxRoot+"/dist/src/native/transform-objects");
const node=(n,inputs,named)=>({name:n,type:"lib",data:{root:n,targets:{t:{executor:"nx:noop",inputs}},...(named?{namedInputs:named}:{})}});
const graph={nodes:{a:node("a",["default","^custom"]),b:node("b",["default"],{custom:["{projectRoot}/b-only/**/*"]}),c:node("c",["default"]),d:node("d",["default"],{custom:["{projectRoot}/d-only/**/*"]}),f:node("f",["^custom"],{custom:["{projectRoot}/f/**"]})},
 dependencies:{a:[{source:"a",target:"b",type:"static"}],b:[{source:"b",target:"c",type:"static"},{source:"b",target:"d",type:"static"}],c:[],d:[],f:[{source:"f",target:"a",type:"static"}]},externalNodes:{}};
const ref=native.transferProjectGraph(transformProjectGraphForRust(graph));
const planner=new native.HashPlanner({namedInputs:{}},ref);
for(const p of ["a","f"]){
const tg={roots:[p+":t"],tasks:{[p+":t"]:{id:p+":t",target:{project:p,target:"t"},outputs:[],overrides:{},parallelism:true,cache:true}},dependencies:{[p+":t"]:[]},continuousDependencies:{[p+":t"]:[]}};
try{console.log(p,JSON.stringify(planner.getPlans([p+":t"],tg),null,1));}catch(e){console.log(p,"ERR",e.message)}}
