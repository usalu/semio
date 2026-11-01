import {readFileSync} from "node:fs";
import {requireCanonicalTestAggregateV1,canonicalTestAggregatePolicyV1 as policy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
const project=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json","utf8"));
requireCanonicalTestAggregateV1(project,policy.root);
console.log("aggregate ok",project.name,project.targets.test.dependsOn.length);
