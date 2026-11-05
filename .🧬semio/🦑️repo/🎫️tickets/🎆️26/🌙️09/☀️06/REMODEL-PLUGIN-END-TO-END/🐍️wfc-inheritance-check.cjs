const fs=require("node:fs"),path=require("node:path"),assert=require("node:assert/strict"),toml=require("smol-toml"),reference=require("@iarna/toml");
const root="✏️s/🔌️plugins/🀄️wfc",text=fs.readFileSync(path.join(root,"Cargo.toml"),"utf8"),workspace=toml.parse(text);
assert.deepEqual(workspace,reference.parse(text));
const missing=[];
function check(dependencies,file,section){for(const[name,value]of Object.entries(dependencies??{}))if(value?.workspace===true&&!workspace.workspace.dependencies?.[name])missing.push({file,section,name});}
for(const member of workspace.workspace.members){const file=path.join(root,member,"Cargo.toml"),text=fs.readFileSync(file,"utf8"),value=toml.parse(text);assert.deepEqual(value,reference.parse(text));for(const section of ["dependencies","dev-dependencies","build-dependencies"])check(value[section],file,section);for(const[target,groups]of Object.entries(value.target??{}))for(const section of ["dependencies","dev-dependencies","build-dependencies"])check(groups[section],file,target+"/"+section);}
assert.deepEqual(missing,[],"all actual workspace-inherited dependencies must have an owner");
for(const[name,value]of Object.entries(workspace.workspace.dependencies??{}))if(value.path)assert(fs.existsSync(path.resolve(root,value.path,"Cargo.toml")),name);
console.log("All eight WFC workspace manifests inherit declared existing owners; both independent TOML parsers agree");
