/** 🪜 Install the independently tested ray-containment change on the live source. */
import {readFileSync,writeFileSync} from 'node:fs'
import {accelerate} from './digital-containment-study.ts'
const path=process.argv[2],source=readFileSync(path,'utf8'),result=accelerate(source)
if(source!==readFileSync(path,'utf8'))throw Error('Concurrent source update')
writeFileSync(path,result)
