/** 🌳 Read the existing pollard-head construction into isolated fine foliage control sections. */
import {writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {build} from './r5-london-plane-pollarded-model.ts'
const fine=build()['50'].body.filter(s=>s.material.includes('plane-leaf'))
writeFileSync(join(import.meta.dir,'🗑️generated/living-plane-controls.json'),JSON.stringify(fine))
console.log('[DEBUG]',fine.length)
