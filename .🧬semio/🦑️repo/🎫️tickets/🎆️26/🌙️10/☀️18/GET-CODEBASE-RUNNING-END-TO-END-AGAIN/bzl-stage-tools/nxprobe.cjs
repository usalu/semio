const nxRoot=process.argv[2];
const native=require(nxRoot+"/dist/src/native");
const {transformProjectGraphForRust}=require(nxRoot+"/dist/src/native/transform-objects");
const node=(n,inputs,named)=>({name:n,type:"lib",data:{root:n,targets:{t:{executor:"nx:noop",inputs}},...(named?{namedInputs:named}:{})}});
const graph={nodes:{a:node("a",["default","^default"]),b:node("b",["default","^default"]),c:node("c",["default","^default"]),d:node("d",["default"]),e:node("e",["^production"],{production:["{projectRoot}/src/**/*","^production"]})},
 dependencies:{a:[{source:"a",target:"b",type:"static"}],b:[{source:"b",target:"c",type:"static"}],c:[],d:[],e:[{source:"e",target:"a",type:"static"}]},externalNodes:{}};
const nxJson={namedInputs:{production:["default","!{projectRoot}/**/*.md"]}};
const ref=native.transferProjectGraph(transformProjectGraphForRust(graph));
const planner=new native.HashPlanner(nxJson,ref);
for(const p of ["a","e"]){
const tg={roots:[p+":t"],tasks:{[p+":t"]:{id:p+":t",target:{project:p,target:"t"},outputs:[],overrides:{},parallelism:true,cache:true}},dependencies:{[p+":t"]:[]},continuousDependencies:{[p+":t"]:[]}};
console.log(p,JSON.stringify(planner.getPlans([p+":t"],tg),null,1));}
