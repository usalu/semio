/** 📦 Checks the lift worker's four boot supports and carton base in the squat that starts the lift. */
import {entryOf} from './volumes-lib.ts'
import {rigAt} from './r3-lib.ts'
const {asset}=entryOf('humans/worker-carrying-box')
const rig=rigAt(asset,50,{kind:'pose',pose:'squat-grip'})
for(const id of ['heel-l','heel-r','contact-l','contact-r','box-base'])if(Math.abs(rig.state.anchors[id].at.y)>.001)throw Error(id+' lifts away from the floor: '+rig.state.anchors[id].at.y)
const end=rig.bones.find(p=>p.id==='toe-l')!.tip[0]
const rear=rig.state.anchors['box-base'].at.x-.15
if(!(rear-end>=.018&&rear-end<=.022))throw Error('toe-to-box clearance '+(rear-end))
console.log('Four grounded boot contacts, level carton, toe clearance',rear-end)
