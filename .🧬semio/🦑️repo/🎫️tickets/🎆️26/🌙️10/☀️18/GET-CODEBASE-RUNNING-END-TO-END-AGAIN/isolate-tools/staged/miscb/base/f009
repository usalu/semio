import{createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {test,expect} from "bun:test";
import {join,relative} from "node:path";
import Ajv from "ajv";
import {withCoordinatorNextInputsV1} from "../🟦️.ts";
import { configuredSourceInputsV1 } from "@semio-tech/repo-lib/inventory";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import {readdirSync,lstatSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};
test("actual coordinator app Source aliases and routes have a finite complete input closure",async()=>withBuildTestInvocationV1(original=>withRepositoryCacheStoreV1(process.cwd(),"dependencies",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async(_store,phase)=>{expect(new Ajv({strict:true}).compile(schema)(policy)).toBe(true);const entries:string[]=[],walk=(directory:string)=>{for(const name of readdirSync(directory).sort()){const path=join(directory,name),status=lstatSync(path);if(status.isSymbolicLink())throw Error("Original route Source required");if(status.isDirectory())walk(path);else if(/\.[cm]?[jt]sx?$/u.test(name))entries.push(relative(process.cwd(),path).replaceAll("\\","/"));}};walk(join(process.cwd(),policy.root,policy.routes));expect(entries.length).toBeGreaterThan(0);const capture=await configuredSourceInputsV1(process.cwd(),{version:1,entries,project:policy.root+"/"+policy.project,packages:{},externalDependencies:policy.externalDependencies,maximumFiles:policy.maximumFiles,maximumBytes:policy.maximumBytes},original,phase);expect(capture.files.length).toBeLessThanOrEqual(policy.maximumFiles);expect(capture.files.some(path=>path.includes("/🖥️server/📚️library/🗄️persistence/"))).toBe(true);expect(capture.files.some(path=>path.includes("/🔨️modules/🫧️transient/"))).toBe(true);expect(capture.files.some(path=>path.endsWith("/🧰️framework/🟦️.ts"))).toBe(false);await capture.recheck();})),60000);

test("actual Next input producer retains complete Source only through its callback",async()=>withBuildTestInvocationV1(async original=>{let escaped!:()=>Promise<void>;await withCoordinatorNextInputsV1(process.cwd(),original,createIssuedResourceControlV1(original,"cache:entry",0),async inputs=>{expect(inputs.files.length).toBeGreaterThan(13);escaped=inputs.recheck;await inputs.recheck();});await expect(escaped()).rejects.toThrow("callback released");}),60000);
