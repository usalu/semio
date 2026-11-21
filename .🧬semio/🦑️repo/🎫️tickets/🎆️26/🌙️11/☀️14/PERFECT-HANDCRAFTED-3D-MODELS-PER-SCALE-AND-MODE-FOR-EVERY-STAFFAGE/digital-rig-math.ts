/** 🧮 Independent frame and free-body arithmetic for the staffage fixtures. */
import {Matrix4,Vector3} from 'three'
export const frameOf=(origin:Vector3,u:Vector3,v:Vector3,w:Vector3)=>new Matrix4().makeBasis(u,v,w).setPosition(origin)
export const momentOf=(at:Vector3,force:Vector3,couple:Vector3)=>new Vector3().crossVectors(at,force).add(couple)
