# the existing test file of the remote cache: anchored substring edits, because other sets edit the same file (loaded by edits.py)
import os as _os

TEST = R + "/🧪️tests/🟦️.ts"
_block = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "staged", "test-command-block.ts")
text(TEST, 'import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";', 'import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";')
text(TEST, 'import {parseRemoteCacheFlagsV1,remoteCacheStatusV1,runRemoteCacheCommandV1,serveRemoteCacheV1,type RemoteCacheCommandContextV1} from "../⌨️command/🟦️.ts";', 'import {collectRemoteCacheV1,parseRemoteCacheArgumentsV1,remoteCacheLanguagesV1,remoteCacheStatusV1,remoteCacheTextV1,runRemoteCacheCommandV1,serveRemoteCacheV1,type RemoteCacheCommandContextV1} from "../⌨️command/🟦️.ts";\nimport texts from "../📖️texts/🔣️.json" with {type:"json"};')
text(TEST, 'maximumAgeDays:30,defaultInstance:"default",tokens:{},...options});', 'maximumAgeDays:30,defaultInstance:"default",tokens:{},validation:"off",...options});')
text(TEST, 'if(op.read!==undefined){store.read("default","ac",keys[op.read]!);continue;}', 'if(op.read!==undefined){await store.read("default","ac",keys[op.read]!);continue;}')
text(TEST, 'type Scenario={name:string;server:{tokens:{read?:string;write?:string};maximumBytes?:number;maximumEntryBytes?:number};steps:Step[]};', 'type Scenario={name:string;server:{tokens:{read?:string;write?:string};maximumBytes?:number;maximumEntryBytes?:number;readOnly?:boolean;validation?:"off"|"structure"|"complete"};steps:Step[]};')
text(TEST, '...(scenario.server.maximumEntryBytes?{maximumEntryBytes:scenario.server.maximumEntryBytes}:{})});', '...(scenario.server.maximumEntryBytes?{maximumEntryBytes:scenario.server.maximumEntryBytes}:{}),...(scenario.server.readOnly?{readOnly:true}:{}),...(scenario.server.validation?{validation:scenario.server.validation}:{})});')
text(TEST, ' const run=async(environment:Record<string,string>,command:string[],extra:string[]=[])=>{', ' const run=async(environment:Record<string,string>,command:string[],extra:string[]=[],place:{cwd:string;output:string}={cwd:workspace,output})=>{')
text(TEST, '`--output_base=${output}`', '`--output_base=${place.output}`')
text(TEST, '{cwd:workspace,timeout:300000,env:{PATH', '{cwd:place.cwd,timeout:300000,env:{PATH')
text(TEST, 'output=join(base,"ob"),events:', 'output=join(base,"ob"),second=join(base,"ws2"),outputTwo=join(base,"ob2"),events:')
text(TEST, """  expect((await run(rw,["clean"])).status).toBe(0);
  const anonymous""", r"""  expect((await run(rw,["shutdown"])).status).toBe(0);
  cpSync(workspace,second,{recursive:true});
  const fresh=await run(rw,["build","//:b"],[],{cwd:second,output:outputTwo});expect(fresh.status,fresh.stderr).toBe(0);
  const summary=/INFO: [0-9]+ processes?: ([^\n]*?)\./u.exec(fresh.stderr);expect(summary,fresh.stderr).not.toBeNull();
  const kinds=summary![1]!.split(", ").map(part=>/^([0-9]+) (.+)$/u.exec(part.trim())!).filter(part=>part[2]!=="internal");
  expect(kinds.map(part=>part[2]),"a second output base in another directory runs nothing: every process is a remote cache hit").toEqual(["remote cache hit"]);expect(Number(kinds[0]![1])).toBe(2);
  expect(readFileSync(join(second,/Target \/\/:b up-to-date:\s+(\S+)/u.exec(fresh.stderr)![1]!),"utf8")).toBe("hello\na\nb\n");
  expect((await run(rw,["shutdown"],[],{cwd:second,output:outputTwo})).status).toBe(0);
  expect((await run(rw,["clean"])).status).toBe(0);
  const anonymous""")
region(TEST, 'test("the command words parse like the catalogue says and refuse everything else",()=>{', 'const bazelBinary=process.env.SEMIO_TEST_BAZEL_BINARY;', open(_block, encoding="utf8").read())
