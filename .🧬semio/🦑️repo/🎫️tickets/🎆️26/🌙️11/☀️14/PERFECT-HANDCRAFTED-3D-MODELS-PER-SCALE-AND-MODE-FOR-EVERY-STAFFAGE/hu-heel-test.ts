/** 👞 Compares the two declared heel-contact centres with the actual outsole underside in its own bone frame. */
import { Quaternion,Vector3 } from 'three';
import { entryOf } from './volumes-lib.ts';
const {asset}=entryOf('humans/person-with-rollator');
for(const suffix of ['l','r']){
 const sole=asset.volumes[50].body.find(p=>p.id==='shoe-heel-'+suffix);
 const anchor=asset.anchors.find(p=>p.id==='heel-'+suffix);
 const q=new Quaternion(...sole.transform.rotation).normalize();
 const at=new Vector3(anchor.x,anchor.y,anchor.z??0).applyQuaternion(q.invert());
 const xs=sole.profile.map(p=>p[1]);
 if(Math.abs(at.y-sole.from)>.0002||Math.abs(at.x-(Math.min(...xs)+Math.max(...xs))/2)>.001)throw Error('heel centre is off the outsole '+suffix+' '+at.toArray());
}
