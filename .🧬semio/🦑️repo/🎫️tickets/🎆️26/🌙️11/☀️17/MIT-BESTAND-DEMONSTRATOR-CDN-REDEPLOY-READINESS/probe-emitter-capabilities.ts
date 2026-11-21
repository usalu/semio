import {captureOwnedCommandV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withDescriptorEmitterV1,withDescriptorEmitterTaskV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts";
import {requireDescriptorEmitterCapabilitiesV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter/📋️contract/🟦️.ts";

await receiveScriptProcessInvocation(process.env,async invocation=>{
  const workspace=process.env.SEMIO_WORKSPACE_ROOT!;
  const control=scriptCompilerAdmissionControlV1(invocation,"descriptor:capability-witness",0);
  await withDescriptorEmitterV1(workspace,invocation,control,emitter=>withDescriptorEmitterTaskV1(emitter,invocation,async tool=>{
    const output=await captureOwnedCommandV1(tool.path,["capabilities"],workspace,{control:tool.control,output:{maximumBytes:128,maximumLines:2}});
    requireDescriptorEmitterCapabilitiesV1(output);
    console.log(JSON.stringify({sha256:tool.sha256,capabilities:JSON.parse(output)}));
  }));
});
