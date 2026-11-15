"""🛬 Package N2 landing 2: the trees the bootstrap generates join the path law."""
import os, sys
sys.argv = [sys.argv[0]]
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "land.py"), encoding="utf8").read().split("def main():")[0].replace("def release():", "def release():"))
B = PE + "📐️budget/"
def run2():
    edit(PE + "🧬️schema/🔣️.json", [('        "cacheDirectory": ".cache/b/r/m/u",', '        "generatedTrees": [\n          ".cache",\n          "dist"\n        ],\n        "cacheDirectory": ".cache/b/r/m/u",')])
    edit(B + "🟦️.ts", [('/** 🚧️ Every tracked file whose path', '''/** 🌲️ The longest path below the trees the bootstrap generates, relative to the worktree, as measured on this machine; `undefined` when none of the trees exists. */
export function generatedTreeLongestV1(workspace:string):Readonly<{length:number;path:string}>|undefined{
 let longest:{length:number;path:string}|undefined;
 const walk=(directory:string,relative:string)=>{let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}for(const entry of entries){const path=relative+"/"+entry.name;if(!longest||path.length>longest.length)longest={length:path.length,path};if(entry.isDirectory()&&!entry.isSymbolicLink())walk(join(directory,entry.name),path);}};
 for(const tree of budget.generatedTrees)if(existsSync(join(workspace,tree)))walk(join(workspace,tree),tree);
 return longest;
}
/** 🚧️ The canonical roots below which one generated path exceeds the bound. */
export function generatedPathOffendersV1(longest:Readonly<{length:number;path:string}>):readonly BudgetPathV1[]{return canonicalRootsV1().flatMap(root=>root.root.length+1+longest.length>policy.maximumCharacters?[{subject:longest.path,root:root.id,target:"generated",profile:"-",length:root.root.length+1+longest.length,path:root.root+"/"+longest.path}]:[]);}
/** 🚧️ Every tracked file whose path''')])
    edit(B + "🧪️tests/🟦️.ts", [('import {cargoBudgetV1,canonicalRootsV1,laneDirectoryV1,readBudgetPackagesV1,sourcePathOffendersV1,trackedFilesV1} from "../🟦️.ts";', 'import {cargoBudgetV1,canonicalRootsV1,generatedPathOffendersV1,generatedTreeLongestV1,laneDirectoryV1,readBudgetPackagesV1,sourcePathOffendersV1,trackedFilesV1} from "../🟦️.ts";'),
        ('test("the worst path per root', '''test("the trees the bootstrap generates fit the bound below the longest canonical root on this machine",()=>{
 const longest=generatedTreeLongestV1(workspace);
 if(longest)expect(describe(generatedPathOffendersV1(longest))).toEqual([]);
});
test("the worst path per root''')])
    for path, text in pending: write(path, text)
    for row in log: print(*row)
acquire()
try:
    run2()
except BaseException:
    release(); raise
