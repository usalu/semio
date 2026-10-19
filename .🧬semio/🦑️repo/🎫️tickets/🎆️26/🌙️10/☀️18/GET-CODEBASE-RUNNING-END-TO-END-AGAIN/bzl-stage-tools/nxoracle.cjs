const input=JSON.parse(require("fs").readFileSync(0,"utf8")),nx=input.nx,native=require(nx+"/dist/src/native"),{transformProjectGraphForRust}=require(nx+"/dist/src/native/transform-objects"),out=[];
for(const row of input.cases){
 const nodes={},dependencies={};
 for(const[name,project]of Object.entries(input.graph.projects)){nodes[name]={name,type:"lib",data:{root:project.root,targets:{t:{executor:"nx:noop",inputs:name===row.project?row.inputs:["default"]}},...(project.namedInputs?{namedInputs:project.namedInputs}:{})}};dependencies[name]=[];}
 for(const edge of input.graph.dependencies)dependencies[edge.source].push({source:edge.source,target:edge.target,type:"static"});
 try{
  const planner=new native.HashPlanner({namedInputs:{}},native.transferProjectGraph(transformProjectGraphForRust({nodes,dependencies,externalNodes:{}}))),id=row.project+":t",graph={roots:[id],tasks:{[id]:{id,target:{project:row.project,target:"t"},outputs:[],overrides:{},parallelism:true,cache:true}},dependencies:{[id]:[]},continuousDependencies:{[id]:[]}};
  const lines=planner.getPlans([id],graph)[id].filter(line=>{const cut=line.indexOf(":");return cut>0&&Object.hasOwn(nodes,line.slice(0,cut))&&!["ProjectConfiguration","TsConfig"].includes(line.slice(cut+1));});
  out.push({id:row.id,plan:lines.sort()});
 }catch(error){out.push({id:row.id,error:String(error.message)});}
}
process.stdout.write(JSON.stringify(out));
