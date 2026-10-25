import {plugin} from "bun";
import {readFileSync} from "node:fs";
plugin({name:"unfix",setup(build){build.onLoad({filter:/component-pass\/.*\.ts$/u},args=>{if(args.path.includes("tests"))return undefined;const text=readFileSync(args.path,"utf8");return{contents:text.replace("withNestedInputOwnersRetiredV1(inputContinuations,()=>executePasses(actual.control))","executePasses(actual.control)"),loader:"ts"};});}});
