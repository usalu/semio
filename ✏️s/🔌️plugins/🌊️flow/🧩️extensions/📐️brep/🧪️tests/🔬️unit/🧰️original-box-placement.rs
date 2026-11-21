//! 📐️ Authored placement fixture for retained Box operations.

fn placed_finish_box(
    registry:&Registry,shape:&Value,origin:[f64;3],angles:[f64;2],mirrored:bool,
    caller:&mut OriginalJobFixtureCaller,born:&mut usize,free:&mut usize,
)->neural_engine::ColdOwner<Dictionary>{
    use semio_s_spatial_kernel_semio_session::{point_dictionary,vector_dictionary};
    let mut geometry=shape.clone();
    for (axis,angle) in [[0.,1.,0.],[1.,0.,0.]].into_iter().zip(angles){
        if angle==0.{continue;}
        let input=Dictionary::new().insert("geometry",geometry)
            .insert("axis",Value::Dictionary(vector_dictionary(axis))).insert("angle",number(angle));
        let result=retained_boolean_result(registry,"brep.xform.rotate",input,caller,born,free);
        geometry=result.get("geometryOut").unwrap().clone();
    }
    if mirrored{
        let input=Dictionary::new().insert("geometry",geometry)
            .insert("origin",Value::Dictionary(point_dictionary([0.;3])))
            .insert("normal",Value::Dictionary(point_dictionary([1.,0.,0.])));
        let result=retained_boolean_result(registry,"brep.xform.mirror",input,caller,born,free);
        geometry=result.get("geometryOut").unwrap().clone();
    }
    let input=Dictionary::new().insert("geometry",geometry).insert("offset",Value::Dictionary(vector_dictionary(origin)));
    retained_boolean_result(registry,"brep.xform.translate",input,caller,born,free)
}
