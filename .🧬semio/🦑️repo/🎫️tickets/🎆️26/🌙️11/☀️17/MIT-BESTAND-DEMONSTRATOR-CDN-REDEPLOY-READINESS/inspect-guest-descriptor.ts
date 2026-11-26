import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { finalizePluginDescriptor } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { componentStageTargetsV1, settleComponentDescriptorV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts";
import { authoredReleaseDigestV1, authoredReleaseDigestsV1, workspace } from "./authored-release-descriptors.ts";

const [plugin, output, prior, reviewed] = process.argv.slice(2);
if (!plugin || !output) throw Error("Actual diagnostic pack required");
const owner = componentStageTargetsV1(workspace).find(target => target.pluginId === plugin);
if (!owner) throw Error("Authored owner required");
const raw = Buffer.from(readFileSync(join(output,"semantic.b64"),"utf8").trim(),"base64");
const descriptor = finalizePluginDescriptor(raw, plugin);
writeFileSync(join(output,"guest.json"),descriptor.json);
writeFileSync(join(output,"guest.pack"),descriptor.pack);
const expected = authoredReleaseDigestsV1([plugin])[plugin];
if (prior || reviewed) {
  if (!prior || !reviewed || prior !== expected) throw Error("Reviewed source descriptor changed");
  const packBytes = readFileSync(join(output, "native", DESCRIPTOR_PACK_FILENAME));
  const jsonBytes = readFileSync(join(output, "native", DESCRIPTOR_JSON_FILENAME));
  const native = authoredReleaseDigestV1(owner, jsonBytes, packBytes);
  if (native !== reviewed || !packBytes.equals(descriptor.pack)) throw Error("Native and JSPI guest descriptors differ");
  const drift = settleComponentDescriptorV1(workspace, owner, {profile:"release",directory:output}, {packBytes,jsonBytes}, "write");
  if (!drift.length || authoredReleaseDigestsV1([plugin])[plugin] !== reviewed) throw Error("Reviewed pair did not settle");
  console.log(JSON.stringify({plugin, prior, reviewed, differences:drift.length}));
} else console.log(JSON.stringify({ownerRoot:owner.ownerRoot,expected,json:join(owner.ownerRoot,DESCRIPTOR_JSON_FILENAME),pack:join(owner.ownerRoot,DESCRIPTOR_PACK_FILENAME)}));
