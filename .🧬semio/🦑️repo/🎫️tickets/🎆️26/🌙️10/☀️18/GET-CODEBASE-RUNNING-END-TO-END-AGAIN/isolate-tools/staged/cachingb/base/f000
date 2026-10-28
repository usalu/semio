import {cp} from "node:fs/promises";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {withNxGraphFixtureV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️store/🟦️.ts";
/** 🔏️ Graph seal laws over `./🧫️fixtures/🔣️.json`: a seal written for a fixture workspace holds exactly while nothing Nx could read for
 * its graph changed (each scenario mutates one thing), every seal satisfies `../🧬️schema/🔣️.json` (Ajv), and the stamped directories are
 * exactly the directories fast-glob lists under the fixture's ignore rules (third-party oracle for the indexable walk). */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { brotliCompressSync } from "node:zlib";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync, renameSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import {createRequire} from "node:module";
import Ajv from "ajv";
import fg from "fast-glob";
import { nxGraphSealHolds, writeNxGraphSeal, nxGraphSourceMembershipV2 } from "../🟦️.ts";

type Mutation = { kind: "none" } | { kind: "write"; path: string; content: string } | {kind:"directory"|"delete";path:string} | {kind:"rename";path:string;destination:string};
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "./🧫️fixtures/🔣️.json"), "utf8")) as { schemaVersion: number; workspace: { files: Record<string, string>; observedFile: string; tracked:string[] }; scenarios: { id: string; mutation: Mutation; holds: boolean }[] };
const validate = new Ajv({ strict: false, allErrors: true }).compile(JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️schema/🔣️.json"), "utf8")));
const collectionPolicy=JSON.parse(readFileSync(new URL("../../../../🗂️workspaces/🔣️policy.json",import.meta.url),"utf8")).collections;
const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
mkdirSync(artifacts, { recursive: true });

/** 🏗️ A fixture workspace with a Git repository, a cached graph and both plugin receipts naming inference records that observed one file. */
function workspace(): { root: string; data: string; pid: number; startedAt: number } {
  const root = mkdtempSync(join(artifacts, "graph-seal-")), data = join(root, ".nx", "workspace-data"), pid = 424242, startedAt = Date.now() - 1000;
  spawnSync("git", ["init", "-q"], { cwd: root });
  for (const [path, content] of Object.entries(fixture.workspace.files)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
  const indexHeader=Buffer.alloc(12);indexHeader.write("DIRC");indexHeader.writeUInt32BE(2,4);indexHeader.writeUInt32BE(fixture.workspace.tracked.length,8);const indexEntries=fixture.workspace.tracked.map(path=>{const value=statSync(join(root,path)),name=Buffer.from(path),entry=Buffer.alloc(Math.ceil((62+name.length+1)/8)*8);entry.writeUInt32BE(Math.floor(value.ctimeMs/1000),0);entry.writeUInt32BE(Math.floor(value.mtimeMs/1000),8);entry.writeUInt32BE(value.dev>>>0,16);entry.writeUInt32BE(value.ino>>>0,20);entry.writeUInt32BE(0o100644,24);entry.writeUInt32BE(value.size,36);const content=readFileSync(join(root,path));createHash("sha1").update(Buffer.from(`blob ${content.length}\0`)).update(content).digest().copy(entry,40);entry.writeUInt16BE(name.length,60);name.copy(entry,62);return entry;}),indexBody=Buffer.concat([indexHeader,...indexEntries]);writeFileSync(join(root,".git/index"),Buffer.concat([indexBody,createHash("sha1").update(indexBody).digest()]));
  const indexed=spawnSync("git",["-c","core.fsmonitor=false","ls-files","-z","--cached"],{cwd:root});expect(indexed.status).toBe(0);expect(indexed.stdout.toString().split("\0").filter(Boolean)).toEqual(fixture.workspace.tracked);
  mkdirSync(join(data, "inference"), { recursive: true });
  const observed = join(root, fixture.workspace.observedFile), stamp = statSync(observed);
  const fact = { kind: "file", path: observed, fact: { size: stamp.size, mtimeMs: stamp.mtimeMs, ctimeMs: stamp.ctimeMs, ino: stamp.ino, sha256: createHash("sha256").update(readFileSync(observed)).digest("hex") } };
  const inference: [string, string][] = [];
  for (const name of ["emoji-project-json.json", "test-cases.json"]) {
    const header = JSON.stringify({ digest: "0".repeat(64), observations: [fact] }), path = join(data, "inference", name);
    writeFileSync(path, brotliCompressSync(Buffer.from(`${header}\n{}`)));
    inference.push([path, createHash("sha256").update(header).digest("hex")]);
  }
  for (const name of ["emoji-project-json.receipt.json", "test-cases.receipt.json"]) writeFileSync(join(data, "inference", name), JSON.stringify({ format: 1, revision: "test", pid, completedAt: Date.now(), inference, observations: [] }));
  writeFileSync(join(data, "file-map.json"), "{}\n");
  writeFileSync(join(data, "project-graph.json"), "{\"nodes\":{}}\n");
  return { root, data, pid, startedAt };
}

/** 🧭️ Routes graph-cache mutations to their actual issued image while preserving authored source paths. */
function mutationPath(root:string,data:string,path:string):string{const prefix=".nx/workspace-data/";return path.startsWith(prefix)?join(data,path.slice(prefix.length)):join(root,path.replace("{collection}",collectionPolicy.names[0]));}

/** 🧪️ Keeps semantic fixtures inside an actual isolated graph namespace. */
async function ownedWorkspace(body:(value:ReturnType<typeof workspace>&{environment:Record<string,string|undefined>;signal:AbortSignal})=>Promise<void>):Promise<void>{return withBuildTestInvocationV1(async original=>{const value=workspace();try{await withNxGraphFixtureV1(value.root,original,async(data,environment,_store,_track,operation)=>{await cp(value.data,data,{recursive:true});await body({...value,data,environment,signal:operation.effectControl.signal});});}finally{rmSync(value.root,{recursive:true,force:true});}});}

test("each fixture scenario holds exactly when nothing graph-relevant changed",async()=>{
 expect(fixture.schemaVersion).toBe(1);for(const scenario of fixture.scenarios)await ownedWorkspace(async({root,data,pid,startedAt,environment,signal})=>{
      const originalMembership=(await nxGraphSourceMembershipV2(root,signal)).digest;
      expect(await writeNxGraphSeal(root, data, pid, startedAt,originalMembership,environment,signal), scenario.id).toBe(true);
      const seal = JSON.parse(readFileSync(join(data, "semio-graph-seal.json"), "utf8")) as { directories: [string][] };
      expect(validate(seal), JSON.stringify(validate.errors)).toBe(true);
      const sourceFiles=fg.sync("**/*",{cwd:root,onlyFiles:true,dot:true,ignore:["**/.git/**","**/node_modules/**","dist/**",".nx/workspace-data/**","tickets/**"]}),sourceDirectories=new Set([""]);for(const file of sourceFiles.filter(file=>/^(?:package\.json|project\.json|📋️project\.json|Cargo\.toml|nx\.json|\.nxignore|\.gitignore)$/u.test(file.split("/").at(-1)!))){const parts=file.split("/");for(let i=1;i<parts.length;i++)sourceDirectories.add(parts.slice(0,i).join("/"));}const oracle=[...sourceDirectories].sort();
      expect(seal.directories.map(([path]) => path).sort()).toEqual(oracle);
      expect(await nxGraphSealHolds(root, data,signal), `${scenario.id} before mutation`).toBe(true);
      if (scenario.mutation.kind === "write") { mkdirSync(dirname(mutationPath(root,data,scenario.mutation.path)), { recursive: true }); writeFileSync(mutationPath(root,data,scenario.mutation.path), scenario.mutation.content); }
      if(scenario.mutation.kind==="directory")mkdirSync(mutationPath(root,data,scenario.mutation.path),{recursive:true});
      if(scenario.mutation.kind==="delete")rmSync(mutationPath(root,data,scenario.mutation.path));
      if(scenario.mutation.kind==="rename")renameSync(mutationPath(root,data,scenario.mutation.path),mutationPath(root,data,scenario.mutation.destination));
      expect(await nxGraphSealHolds(root, data,signal), scenario.id).toBe(scenario.holds);
      if((await nxGraphSourceMembershipV2(root,signal)).digest!==originalMembership)expect(await writeNxGraphSeal(root,data,pid,startedAt,originalMembership,environment,signal)).toBe(false);
 });
},100000);

test("a seal is refused for another process's receipts or a graph older than the invocation",async()=>ownedWorkspace(async({root,data,pid,startedAt,environment,signal})=>{
    expect(await writeNxGraphSeal(root, data, pid + 1, startedAt,(await nxGraphSourceMembershipV2(root,signal)).digest,environment,signal)).toBe(false);
    expect(await writeNxGraphSeal(root, data, pid, Date.now() + 60_000,(await nxGraphSourceMembershipV2(root,signal)).digest,environment,signal)).toBe(false);
    expect(await nxGraphSealHolds(root, data,signal)).toBe(false);
    expect(await writeNxGraphSeal(root, data, pid, startedAt,(await nxGraphSourceMembershipV2(root,signal)).digest,environment,signal)).toBe(true);
    expect(await nxGraphSealHolds(root, data,signal)).toBe(true);
}),100000);

test("verified immutable whole graph readers preserve the portable custody corpus and refuse a foreign owner",async()=>{
 const identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!identity||!/^[a-f0-9]{64}$/u.test(identity))throw Error("Actual incoming immutable SDK identity required");
 const sdk=createRequire(join(process.cwd(),".cache/b/t/m/nx",identity,"package.json")),frozen=await sdk("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();expect(frozen.toolingIdentity).toBe(identity);
 for(const scenario of fixture.scenarios)await ownedWorkspace(async({root,data,pid,startedAt,environment,signal})=>{
  const membership=await frozen.authorizers.nxGraphSourceMembershipV2(root,signal);expect(membership).toEqual(await nxGraphSourceMembershipV2(root,signal));
  await expect(frozen.authorizers.writeNxGraphSeal(root,data,pid,startedAt,membership.digest,environment,signal)).rejects.toThrow("Original live native graph namespace");expect(await writeNxGraphSeal(root,data,pid,startedAt,membership.digest,environment,signal),scenario.id).toBe(true);expect(validate(JSON.parse(readFileSync(join(data,"semio-graph-seal.json"),"utf8"))),JSON.stringify(validate.errors)).toBe(true);
  const mutation=scenario.mutation;if(mutation.kind==="write"){const path=mutationPath(root,data,mutation.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,mutation.content);}else if(mutation.kind==="directory")mkdirSync(mutationPath(root,data,mutation.path),{recursive:true});else if(mutation.kind==="delete")rmSync(mutationPath(root,data,mutation.path));else if(mutation.kind==="rename")renameSync(mutationPath(root,data,mutation.path),mutationPath(root,data,mutation.destination));
  const actual=await frozen.authorizers.nxGraphSealHolds(root,data,signal);expect(actual,scenario.id).toBe(scenario.holds);expect(actual).toBe(await nxGraphSealHolds(root,data,signal));
 });
},100000);
