import {runDependencyVerification} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts";
import {dependencyJsLockParitySelfTests} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts";
runDependencyVerification(process.cwd(),process.argv.slice(2),{jsLockParity:dependencyJsLockParitySelfTests,truth:()=>0});
