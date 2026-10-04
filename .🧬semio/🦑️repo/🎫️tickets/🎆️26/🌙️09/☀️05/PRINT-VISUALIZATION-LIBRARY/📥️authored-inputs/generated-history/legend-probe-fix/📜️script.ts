import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const file='C:/git/semio/🧰️framework/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/🟦️.ts';
let source=readFileSync(file,'utf8');source=source.replace('body:bodies},{workDir,keepWorkDir:true});\n  let count=0;','body:bodies},{workDir,scenario:undefined,keepWorkDir:true});\n  let count=0;').replace('items.filter(item=>item.kind==="text"&&item.baseline==="middle")','items.filter(item=>item.kind==="text").filter(item=>item.baseline==="middle")');
const temporary=import.meta.dir+'/source.tmp';writeFileSync(temporary,source);renameSync(temporary,file);
