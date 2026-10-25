import {readFileSync} from "node:fs";
import {bazelWarmVerdictsV1,parseBazelWarmSpawnsV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/👁️evidence/♨️warm/🟦️.ts";

const [cold,warm]=process.argv.slice(2).map(path=>parseBazelWarmSpawnsV1(readFileSync(path,"utf8")));
for(const row of bazelWarmVerdictsV1(cold!,warm!))console.log(row.label,row.verdict,"changed",row.changedInputs.length,"derived",row.derivedInputsChanged,"source",row.sourceInputsChanged,"args",row.argumentsChanged,"outputsIdentical",row.outputsIdentical);
