import {writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {entryOf,frames} from './volumes-lib.ts'
writeFileSync(join(import.meta.dir,'🗑️generated','living-maple-frames.json'),JSON.stringify(frames(entryOf('vegetation/japanese-maple').asset)))
