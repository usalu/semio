import {expect,test} from "bun:test";
import yargsParser from "yargs-parser";
import catalog from "../../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {createBuildBootstrapCapabilitiesV1} from "../../📤️child/🟦️.ts";
import {fileURLToPath} from "node:url";
import fixture from "./🔣️.json" with {type:"json"};

const root=fileURLToPath(import.meta.url).split("/🧰️framework/")[0]!;
const environment=()=>({SEMIO_BUILD_MODE:"prototype"} as NodeJS.ProcessEnv);
const known=catalog.operations.map(row=>row.name);

test("Every accepted row creates its capabilities and every refused row is refused by name in both languages before any resource is taken",async()=>{
 for(const row of fixture.accepted)await expect(createBuildBootstrapCapabilitiesV1(root,["repo",...row.args],environment()),row.name).resolves.toBeObject();
 for(const row of fixture.refused){
  await expect(createBuildBootstrapCapabilitiesV1(root,["repo",...row.args],environment()),row.name).rejects.toThrow(`Unknown repo command "${row.command}"`);
  await expect(createBuildBootstrapCapabilitiesV1(root,["repo",...row.args],environment()),row.name).rejects.toThrow(`Unbekannter repo-Befehl "${row.command}"`);
 }
});

test("The refused commands are exactly the first positional words an independent argument reader finds that the catalogue does not name",()=>{
 const positional=(args:readonly string[])=>yargsParser([...args],{string:["tool"]})._[0] as string|undefined;
 for(const row of fixture.refused){expect(positional(row.args),row.name).toBe(row.command);expect(known.includes(row.command),row.name).toBe(false);}
 for(const row of fixture.accepted){const word=positional(row.args);expect(word===undefined||known.includes(String(word))||row.args.includes("--help"),row.name).toBe(true);}
});

test("Commands outside the repo entry (build, test) are not judged by the repo catalogue",async()=>{
 await expect(createBuildBootstrapCapabilitiesV1(root,["build"],environment())).resolves.toBeObject();
});
