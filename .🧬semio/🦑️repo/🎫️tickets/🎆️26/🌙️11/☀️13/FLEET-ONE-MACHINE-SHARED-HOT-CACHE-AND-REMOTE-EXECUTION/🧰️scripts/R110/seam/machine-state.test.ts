import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {MACHINE_STATE_ENVIRONMENT_V1,machineStateDirectoryV1,machineStatePathV1} from "../🟦️.ts";

/** 🏠️ The state directory of a machine is injectable: the same checkout can host several machines, each with its own `.cache/f`, and no site on the readiness path may spell the directory out itself. */
const build=resolve(import.meta.dir,"..","..");
const SITES=[
 "🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts","🌐️remote-cache/🛰️fleet/🟦️.ts","🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts","🌐️remote-execution/🌡️load/🟦️.ts",
 "🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts","🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts","🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts",
 "🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🏃️runner/🟨️.mjs","🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🔒️guard/🟦️.ts","🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🔬️inference/🟨️.mjs",
 "🕸️graph/📥️inference/📦️project-nodes/🔱️warm/🟨️.mjs","🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs","🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts","🧠️memory/📋️slice-report/🟨️.mjs",
 "🪪️graph-state/🌳️tree/🟦️.ts","🪪️graph-state/📍️cursor/🟦️.ts","🪪️graph-state/📜️script.ts","🪪️graph-state/📥️replica/🟦️.ts","🪪️graph-state/📦️project-records/🟦️.ts","🪪️graph-state/🧶️nx/🟦️.ts","🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts",
];

describe("machine state directory",()=>{
 test("a machine without a declaration keeps its state in the checkout",()=>{
  expect(machineStateDirectoryV1("/work/tree",{})).toBe(join("/work/tree",".cache","f"));
  expect(machineStatePathV1("/work/tree",".cache/f/ready",{})).toBe(join("/work/tree",".cache","f","ready"));
  expect(machineStatePathV1("/work/tree","📋️project.json",{})).toBe(join("/work/tree","📋️project.json"));
 });
 test("a declared absolute directory replaces the whole `.cache/f` prefix and nothing else",()=>{
  const environment={[MACHINE_STATE_ENVIRONMENT_V1]:"/machines/one/state"};
  expect(machineStateDirectoryV1("/work/tree",environment)).toBe("/machines/one/state");
  expect(machineStatePathV1("/work/tree",".cache/f/graph-costs",environment)).toBe(join("/machines/one/state","graph-costs"));
  expect(machineStatePathV1("/work/tree",".cache/f",environment)).toBe("/machines/one/state");
  expect(machineStatePathV1("/work/tree",".cache/b/t",environment)).toBe(join("/work/tree",".cache","b","t"));
 });
 test("an empty or relative declaration is ignored",()=>{
  for(const value of ["","state","./state"])expect(machineStateDirectoryV1("/work/tree",{[MACHINE_STATE_ENVIRONMENT_V1]:value})).toBe(join("/work/tree",".cache","f"));
 });
 test("no site of the readiness path spells the machine state directory out",()=>{
  const offenders=SITES.filter(site=>/"\.cache",\s*"f"/u.test(readFileSync(join(build,site),"utf8")));
  expect(offenders).toEqual([]);
 });
});
