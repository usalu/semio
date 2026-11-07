import os
V="🧰️framework/🔨️modules/🖱️ui/🪟️viewport"
os.makedirs(V+"/📥️receiving",exist_ok=True)
open(V+"/📥️receiving/🦀️.rs","w",encoding="utf-8").write('''//! 📥️ Viewport poses receive their record encoding through the bounded scalar receiver and agree with the codec.
use crate::{Viewport2d,Viewport3dOrbit};
use semio_framework_dsl_record::{receiving::owned::invalid,DslField,FieldValue};
use semio_framework_value::ValueError;

fn bounded<T:DslField>(value:&FieldValue,fields:usize)->Result<T,ValueError>{match value{FieldValue::Record(record) if record.fields.len()<=fields=>T::from_value(value).map_err(|_|invalid()),_=>Err(invalid())}}
fn planar(value:&FieldValue)->Result<Viewport2d,ValueError>{bounded(value,3)}
fn orbit(value:&FieldValue)->Result<Viewport3dOrbit,ValueError>{bounded(value,4)}
semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy Viewport2d,planar);
semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy Viewport3dOrbit,orbit);
''')
p=V+"/🦀️.rs";s=open(p,encoding="utf-8").read()
if "📥️receiving" not in s:
    s=s.rstrip("\n")+'\n#[path = "📥️receiving/🦀️.rs"]\nmod receiving;\n'
    open(p,"w",encoding="utf-8").write(s)
T=V+"/🧪️tests/📥️receiving"
os.makedirs(T,exist_ok=True)
open(T+"/🦀️.rs","w",encoding="utf-8").write('''//! 📥️ Viewport poses round-trip through their DslField encoding via original receiving and refuse foreign records.
use semio_framework_dsl_record::receiving::owned::OwnedScalar;
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue};
use semio_ui_viewport::{Viewport2d,Viewport3dOrbit};

#[test]
fn planar_pose_round_trips_through_receiving(){
    for pose in [Viewport2d::default(),Viewport2d{x:-3.5,y:12.0,zoom:0.25}]{
        let mut source=Some(pose.to_value());
        assert_eq!(<Viewport2d as OwnedScalar>::take_source(&mut source).unwrap(),pose);
        assert_eq!(<Viewport2d as OwnedScalar>::take_source(&mut source).unwrap(),Viewport2d::from_value(&pose.to_value()).unwrap());
    }
}
#[test]
fn orbit_pose_round_trips_with_and_without_up(){
    for up in [None,Some([0.0,1.0,0.0])]{
        let pose=Viewport3dOrbit{position:[1.0,2.0,3.0],target:[0.0,0.0,0.0],zoom:2.0,up};
        let mut source=Some(pose.to_value());
        assert_eq!(<Viewport3dOrbit as OwnedScalar>::take_source(&mut source).unwrap(),pose);
    }
}
#[test]
fn invalid_poses_are_refused_and_keep_their_source(){
    let zoom_zero=FieldValue::Record(RecordValue{fields:[(1,FieldValue::Float(0.0)),(2,FieldValue::Float(0.0)),(3,FieldValue::Float(0.0))].into_iter().collect()});
    let oversized=FieldValue::Record(RecordValue{fields:(0..64u16).map(|id|(id,FieldValue::Float(1.0))).collect()});
    for foreign in [FieldValue::Bool(true),zoom_zero,oversized]{
        let mut source=Some(foreign.clone());
        assert!(<Viewport2d as OwnedScalar>::take_source(&mut source).is_err(),"refused: {foreign:?}");
        assert!(source.is_some());
    }
}
''')
cp=V+"/📦️packages/🦀️rust/Cargo.toml";c=open(cp,encoding="utf-8").read()
if "viewport_receiving" not in c:
    c=c.rstrip("\n")+'\n\n[[test]]\nname = "viewport_receiving"\npath = "../../🧪️tests/📥️receiving/🦀️.rs"\n'
    open(cp,"w",encoding="utf-8").write(c)
# puzzle scales
def scale(p,ty,anchor):
    s=open(p,encoding="utf-8").read()
    if "dsl_owned_scalar_receiving" in s: return
    a=anchor
    assert s.count(a)==1,(p,s.count(a))
    code=f'''
fn {ty.lower()}_receive(value:&semio_framework_dsl_record::FieldValue)->Result<{ty},semio_framework_value::ValueError>{{
    use semio_framework_dsl_record::{{receiving::owned::invalid,FieldValue}};
    let FieldValue::List(items)=value else{{return Err(invalid())}};
    let axis=|value:&FieldValue|match value{{FieldValue::Float(axis)=>Ok(*axis),_=>Err(invalid())}};
    match items.as_slice(){{[x]=>Ok({ty}::Uniform(axis(x)?)),[x,y,z]=>Ok({ty}::Vec3([axis(x)?,axis(y)?,axis(z)?])),_=>Err(invalid())}}
}}
semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy {ty},{ty.lower()}_receive);

#[cfg(test)]
mod {ty.lower()}_receiving_tests{{
    use super::*;
    use semio_framework_dsl_record::{{receiving::owned::OwnedScalar,DslField,FieldValue}};
    #[test]
    fn {ty.lower()}_round_trips_through_original_receiving_and_refuses_foreign_shapes(){{
        for scale in [{ty}::Uniform(2.0),{ty}::Vec3([1.0,2.0,3.0])]{{
            let mut source=Some(scale.to_value());
            assert_eq!(<{ty} as OwnedScalar>::take_source(&mut source).unwrap(),scale);
            assert_eq!(scale,{ty}::from_value(&scale.to_value()).unwrap());
        }}
        for foreign in [FieldValue::List(vec![FieldValue::Float(1.0),FieldValue::Float(2.0)]),FieldValue::Bool(true)]{{
            let mut source=Some(foreign);
            assert!(<{ty} as OwnedScalar>::take_source(&mut source).is_err());
            assert!(source.is_some());
        }}
    }}
}}
'''
    s=s.replace(a,a+code,1)
    open(p,"w",encoding="utf-8").write(s)
scale("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📚️document-model/🦀️.rs","Puzzle3dScale","semio_framework_value::artifact_retire_leaf!(Puzzle3dScale);")
scale("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️document-records/🦀️.rs","Puzzle5dScale","semio_framework_value::artifact_retire_leaf!(Puzzle5dScale);")
