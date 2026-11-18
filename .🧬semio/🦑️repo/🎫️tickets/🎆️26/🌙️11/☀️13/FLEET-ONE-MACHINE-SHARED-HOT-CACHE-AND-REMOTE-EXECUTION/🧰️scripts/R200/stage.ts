import {plugin} from "bun";
const S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R200/s/si.new.mjs";
plugin({name:"r200-stage",setup(b){b.onLoad({filter:/\.mjs$/},async({path})=>{
 if(/script-inputs\/[^/]*\.mjs$/.test(path)&&!process.env.NOSTAGE)return {contents:await Bun.file(S).text(),loader:"js"};
 return {contents:await Bun.file(path).text(),loader:"js"};});}});
