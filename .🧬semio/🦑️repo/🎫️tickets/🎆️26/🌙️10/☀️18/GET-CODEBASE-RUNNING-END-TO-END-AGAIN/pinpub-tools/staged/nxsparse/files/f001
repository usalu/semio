/** 🪶️ Law: the Nx graph pool and its graph child images admit a sparse backing like a preallocated one (native images are created sparse since store-g, 10-27), and still refuse links, hard-linked and non-regular backings. Corpus `./🔣️.json`; independent oracle: the Python lstat model `./🐍️.py`; negative control: the former rule (`blocks*512 < size` refuses) refuses the sparse backing, which is how every freshly created Nx partition was refused (`Fully allocated Nx graph pool backing required`). */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {closeSync,ftruncateSync,linkSync,lstatSync,mkdirSync,mkdtempSync,openSync,rmSync,symlinkSync,writeFileSync,writeSync,type Stats} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxGraphBackingAdmittedV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";

test("the Nx graph backing rule admits sparse and preallocated images and refuses every other shape",()=>{
 const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"nxs-")),cases=join(root,"cases"),twins=join(root,"twins"),former=(row:Stats)=>row.isFile()&&!row.isSymbolicLink()&&row.nlink===1&&!(row.blocks*512<row.size);
 try{
  mkdirSync(cases);mkdirSync(twins);
  for(const[name,row]of Object.entries(corpus.cases)){const path=join(cases,name);
   if(row.kind==="sparse"){const fd=openSync(path,"wx");try{writeSync(fd,Buffer.alloc(corpus.headBytes,1));ftruncateSync(fd,corpus.sparseBytes);}finally{closeSync(fd);}}
   else if(row.kind==="preallocated"||row.kind==="hardlink"){writeFileSync(path,Buffer.alloc(corpus.preallocatedBytes,1));if(row.kind==="hardlink")linkSync(path,join(twins,name));}
   else if(row.kind==="symlink")symlinkSync(join(twins,"target"),path);
   else mkdirSync(path);
  }
  writeFileSync(join(twins,"target"),Buffer.alloc(corpus.preallocatedBytes,1));
  const run=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py"),cases],{encoding:"utf8"});expect(run.status).toBe(0);const oracle=JSON.parse(run.stdout);
  for(const[name,row]of Object.entries(corpus.cases)){expect({name,admitted:nxGraphBackingAdmittedV1(lstatSync(join(cases,name)))}).toEqual({name,admitted:row.admitted});expect({name,admitted:oracle[name].admitted}).toEqual({name,admitted:row.admitted});}
  if(process.platform!=="win32"){expect(oracle.sparse.sparse).toBe(true);expect(oracle.preallocated.sparse).toBe(false);expect(former(lstatSync(join(cases,"sparse")))).toBe(false);}
  expect(former(lstatSync(join(cases,"preallocated")))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
