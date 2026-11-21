//! 🧪️ Shared component witness for the actual Mesh widget and Flow continuation.

use serde_json::{Value,json};
use std::collections::{BTreeMap,BTreeSet};

pub fn fixture()->Value {
    fixture_named("🧩️disconnected-difference.json")
}

pub fn fixture_named(name:&str)->Value{
    let path=std::path::PathBuf::from(std::env::var("SEMIO_MESH_COMPOUND_FIXTURE").unwrap()).with_file_name(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

pub fn operands(faces:&Value,row:&Value,positions:&str)->[Value;2] {
    ["left","right"].map(|side|{
        let mut rings:Vec<Vec<u32>>=serde_json::from_value(faces.clone()).unwrap();
        if side=="left"&&row["leftReflected"]==true{for ring in &mut rings{ring.reverse();}}
        let mut value=json!({"faces":rings});
        value[positions]=row[side].clone();
        value
    })
}

pub fn prism_operands(law:&Value,positions:&str)->[Value;2]{
    let footprint:Vec<[f32;2]>=serde_json::from_value(law["footprint"].clone()).unwrap();
    let height:[f32;2]=serde_json::from_value(law["height"].clone()).unwrap();
    let points:Vec<_>=height.into_iter().flat_map(|z|footprint.iter().map(move |&[x,y]|[x,y,z])).collect();
    let mut stock=json!({"faces":law["stock"]["faces"]});
    stock[positions]=law["stock"]["positions"].clone();
    let mut tool=json!({"faces":law["faces"]});
    tool[positions]=json!(points);
    [stock,tool]
}

pub fn prism_enclosure(law:&Value,case:&str,positions:&str)->Value{
    let [[a,b,c],[d,e,f]]:[[f32;3];2]=serde_json::from_value(law[case]["bounds"].clone()).unwrap();
    let mut enclosure=json!({"faces":law["stock"]["faces"]});
    enclosure[positions]=json!([[a,b,c],[d,b,c],[d,e,c],[a,e,c],[a,b,f],[d,b,f],[d,e,f],[a,e,f]]);
    enclosure
}

pub fn verify(points:&[[f32;3]],faces:&[Vec<u32>],row:&Value) {
    let mut uses=BTreeMap::<_, (usize,bool)>::new();
    let mut paired=BTreeSet::new();
    let mut neighbors=vec![Vec::new();faces.len()];
    for (face,ring) in faces.iter().enumerate(){
        assert_eq!(ring.iter().copied().collect::<BTreeSet<_>>().len(),ring.len());
        for (a,b) in ring.iter().copied().zip(ring.iter().copied().cycle().skip(1)){
            let key=(a.min(b),a.max(b));
            assert!(!paired.contains(&key));
            if let Some((other,forward))=uses.remove(&key) {
                assert_ne!(a<b, forward);
                paired.insert(key);
                neighbors[other].push(face);
                neighbors[face].push(other);
            } else {
                uses.insert(key,(face,a<b));
            }
        }
    }
    assert!(uses.is_empty());
    let mut remaining=(0..faces.len()).collect::<BTreeSet<_>>();
    let diff=|a:u32,b:u32|std::array::from_fn::<_,3,_>(|i|f64::from(points[a as usize][i])-f64::from(points[b as usize][i]));
    let cross=|[a,b,c]:[f64;3],[d,e,f]:[f64;3]|[b*f-c*e,c*d-a*f,a*e-b*d];
    let dot=|a:[f64;3],b:[f64;3]|(0..3).map(|i|a[i]*b[i]).sum::<f64>();
    let mut measured=Vec::new();
    while let Some(seed)=remaining.pop_first(){
        let mut vertices=BTreeSet::new();
        let (mut edge_uses,mut count)=(0,0);
        let (mut area,mut volume)=(0.,0.);
        let mut visit=vec![seed];
        let anchor=faces[seed][0];
        while let Some(face)=visit.pop(){
            let ring=&faces[face];
            let first=ring[0];
            let mut normal=[0.;3];
            for pair in ring[1..].windows(2){
                for (total,part) in normal.iter_mut().zip(cross(diff(pair[0],first),diff(pair[1],first))){*total+=part;}
                volume+=dot(diff(first,anchor),cross(diff(pair[0],anchor),diff(pair[1],anchor)))/6.;
            }
            area+=dot(normal,normal).sqrt()/2.;
            vertices.extend(ring.iter().copied());
            edge_uses+=ring.len();
            count+=1;
            visit.extend(neighbors[face].iter().copied().filter(|near|remaining.remove(near)));
        }
        measured.push((volume,area,vertices.len() as i64-(edge_uses/2) as i64+count));
    }
    measured.sort_unstable_by(|a,b|a.0.total_cmp(&b.0));
    let single=[json!({"volume":row["volume"],"area":row["area"],"euler":2})];
    let expected=row["components"].as_array().map_or(single.as_slice(),Vec::as_slice);
    assert_eq!(measured.len(),expected.len());
    for ((volume,area,euler),want) in measured.into_iter().zip(expected){
        assert!((volume-want["volume"].as_f64().unwrap()).abs()<1e-4);
        assert!((area-want["area"].as_f64().unwrap()).abs()<1e-4);
        assert_eq!(euler,want["euler"].as_i64().unwrap());
    }
}
