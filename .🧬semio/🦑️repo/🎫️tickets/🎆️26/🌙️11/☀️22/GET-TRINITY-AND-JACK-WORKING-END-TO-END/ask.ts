import {normalizeBuildOptionsV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔤️options/🟦️.ts";
import {parseBuildInvocationV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts";
const a=normalizeBuildOptionsV1(["show","project","x","--graph-policy=current"]);
console.log(JSON.stringify(a));
console.log(JSON.stringify(parseBuildInvocationV1(a,{}).args));
