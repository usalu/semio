/** 🧭️ Maps a workspace file to the project whose root is its longest directory prefix in one walk up its own path instead of a scan of every project; equal roots and the root project "." keep the order of the original sorted scan (longest root first, then insertion order). */
export function createProjectFileIndexV1(projects){
 const byRoot=new Map();
 Object.entries(projects).forEach(([name,project],index)=>{const held=byRoot.get(project.root);if(!held||index<held.index)byRoot.set(project.root,{name,root:project.root,index});});
 const better=(row,other)=>!other||row.root.length>other.root.length||row.root.length===other.root.length&&row.index<other.index;
 return file=>{
  let best=byRoot.get(".");
  for(let cut=file.lastIndexOf("/");cut>0;cut=file.lastIndexOf("/",cut-1)){const row=byRoot.get(file.slice(0,cut));if(row){if(better(row,best))best=row;break;}}
  return best?.name;
 };
}
