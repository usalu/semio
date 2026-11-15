import fixture from "/mir/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import {readTopologyV1,workerPartsV1,serverPartsV1} from "/mir/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/🟦️.ts";
import {writeFileSync} from "node:fs";
const topology=readTopologyV1(fixture.topology),id=fixture.topology.groups[0]!.id;
writeFileSync("/out/replica.service",workerPartsV1(topology,id,"linux")["definition"]!);
const server=serverPartsV1(topology,id);
writeFileSync("/out/cache.service",server["cache-unit"]!);writeFileSync("/out/relay.service",server["relay-unit"]!);
writeFileSync("/out/replica.plist",workerPartsV1(topology,id,"macos")["definition"]!);
writeFileSync("/out/replica.xml",workerPartsV1(topology,id,"windows")["definition"]!);
