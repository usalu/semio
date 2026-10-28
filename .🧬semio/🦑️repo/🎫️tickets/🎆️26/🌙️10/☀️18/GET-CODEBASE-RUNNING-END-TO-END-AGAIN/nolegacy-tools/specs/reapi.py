"""🛰️ Batch `nl-reapi`: the in-house remote executor speaks REAPI 2.1 to 2.3 only. The 2.0 output declaration (`output_files` + `output_directories`, answered with `output_file_symlinks` + `output_directory_symlinks`) was a compatibility branch for clients older than the pinned Bazel 8.4.2, which declares `output_paths`; a command that still declares the 2.0 fields is refused with INVALID_ARGUMENT."""

TITLE = "🛰️ Staged batch `nl-reapi` (lane nolegacy): the remote executor advertises REAPI 2.1-2.3, reads only `output_paths` and refuses a 2.0 command (`output_files`/`output_directories`) instead of answering it with the deprecated symlink lists."
X = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/"

WORKER_DECLARE_OLD = '''  if(command.arguments.length===0)return fail(3,"Command without arguments");
  const rootDigest=action.input_root_digest?unwire(action.input_root_digest):{hash:EMPTY_HASH,size:0},fetchStart=Date.now();
  await materialize(rootDigest,root,0);
  const cwdRelative=command.working_directory===""?"":safeRelative(command.working_directory),cwd=cwdRelative===""?root:join(root,cwdRelative),legacy=command.output_paths.length===0,declared=legacy?[...command.output_files,...command.output_directories]:command.output_paths;
  await mkdir(cwd,{recursive:true});
  for(const path of declared){const clean=safeRelative(path),directory=dirname(join(cwd,clean));await mkdir(directory,{recursive:true});}
  for(const path of legacy?command.output_directories:[])await mkdir(join(cwd,safeRelative(path)),{recursive:true});
'''
WORKER_DECLARE_NEW = '''  if(command.arguments.length===0)return fail(3,"Command without arguments");
  if(command.output_files.length>0||command.output_directories.length>0)return fail(3,"Command declares REAPI 2.0 output_files or output_directories; declare output_paths (REAPI 2.1)");
  const rootDigest=action.input_root_digest?unwire(action.input_root_digest):{hash:EMPTY_HASH,size:0},fetchStart=Date.now();
  await materialize(rootDigest,root,0);
  const cwdRelative=command.working_directory===""?"":safeRelative(command.working_directory),cwd=cwdRelative===""?root:join(root,cwdRelative),declared=command.output_paths;
  await mkdir(cwd,{recursive:true});
  for(const path of declared){const clean=safeRelative(path),directory=dirname(join(cwd,clean));await mkdir(directory,{recursive:true});}
'''

TEST_TREE_OLD = '''test("directory outputs come back as a Tree with children, executable bits and symbolic links; symlink outputs are reported",async()=>{
 const cluster=await start({workers:1}),script="mkdir -p out/sub; echo A > out/sub/a.txt; echo B > out/b.sh; chmod +x out/b.sh; ln -s sub/a.txt out/link; ln -s out/b.sh top-link; mkdir -p out/empty";
 const action=await cluster.action({arguments:["/bin/sh","-c",script],outputDirectories:["out"],outputFiles:["top-link"]}),run=await cluster.execute(action),result=run.response.result;'''
TEST_TREE_NEW = '''test("directory outputs come back as a Tree with children, executable bits and symbolic links; symlink outputs are reported; a REAPI 2.0 command is refused",async()=>{
 const cluster=await start({workers:1}),script="mkdir -p out/sub; echo A > out/sub/a.txt; echo B > out/b.sh; chmod +x out/b.sh; ln -s sub/a.txt out/link; ln -s out/b.sh top-link; mkdir -p out/empty";
 for(const declared of [{outputFiles:["top-link"]},{outputDirectories:["out"]},{outputFiles:["top-link"],outputPaths:["top-link"]}]){
  const refused=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c",script],...declared}));
  expect(refused.response.status.code,JSON.stringify(declared)).toBe(GRPC_CODES_V1.INVALID_ARGUMENT);
 }
 const action=await cluster.action({arguments:["/bin/sh","-c",script],outputPaths:["out","top-link"]}),run=await cluster.execute(action),result=run.response.result;'''

EDITS = {
    X + "👷️worker/🟦️.ts": [
        (WORKER_DECLARE_OLD, WORKER_DECLARE_NEW, 1),
        ("  const outputFiles:ProtobufMessageV1[]=[],outputDirectories:ProtobufMessageV1[]=[],outputSymlinks:ProtobufMessageV1[]=[],fileSymlinks:ProtobufMessageV1[]=[],directorySymlinks:ProtobufMessageV1[]=[];", "  const outputFiles:ProtobufMessageV1[]=[],outputDirectories:ProtobufMessageV1[]=[],outputSymlinks:ProtobufMessageV1[]=[];", 1),
        ("   if(info.isSymbolicLink()){\n    const row={path:clean,target:await readlink(full)};outputSymlinks.push(row);\n    if(legacy)(command.output_directories.includes(path)?directorySymlinks:fileSymlinks).push(row);\n   }else if", "   if(info.isSymbolicLink())outputSymlinks.push({path:clean,target:await readlink(full)});\n   else if", 1),
        ("output_symlinks:outputSymlinks,...(legacy?{output_file_symlinks:fileSymlinks,output_directory_symlinks:directorySymlinks}:{}),", "output_symlinks:outputSymlinks,", 1),
    ],
    X + "🖥️server/🟦️.ts": [("low_api_version:{major:2,minor:0},high_api_version:{major:2,minor:3},", "low_api_version:{major:2,minor:1},high_api_version:{major:2,minor:3},", 1)],
    X + "🧪️tests/🟦️.ts": [
        ("caps.high_api_version.minor]).toEqual([2,0,2,3]);", "caps.high_api_version.minor]).toEqual([2,1,2,3]);", 1),
        (TEST_TREE_OLD, TEST_TREE_NEW, 1),
        (' expect(result.output_file_symlinks.map((row:any)=>[row.path,row.target])).toEqual([["top-link","out/b.sh"]]);expect(result.output_symlinks.map((row:any)=>row.path)).toEqual(["top-link"]);', ' expect(result.output_symlinks.map((row:any)=>[row.path,row.target])).toEqual([["top-link","out/b.sh"]]);expect(result.output_file_symlinks).toEqual([]);expect(result.output_directory_symlinks).toEqual([]);', 1),
    ],
    X + "🧪️tests/📜️scenarios/🧫️fixtures/🔣️.json": [('    "low_api_version": {\n     "major": 2,\n     "minor": 0\n    },', '    "low_api_version": {\n     "major": 2,\n     "minor": 1\n    },', 1)],
}
TESTS = ["./" + X + "🧪️tests/🟦️.ts", "./" + X + "🧪️tests/📜️scenarios/🟦️.ts"]
