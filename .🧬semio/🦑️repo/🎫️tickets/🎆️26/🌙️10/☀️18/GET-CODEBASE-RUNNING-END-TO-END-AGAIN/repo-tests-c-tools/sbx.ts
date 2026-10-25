import {testSandboxParentV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox/📐️layout/🟦️.ts";
import {currentAgentSessionV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🪪️agent-sessions/🟦️.ts";
const s=currentAgentSessionV1(),h=testSandboxParentV1()+"/"+s.resources.tests.root;
console.log("[DEBUG]",h,h.length,JSON.stringify(s.resources.tests));
