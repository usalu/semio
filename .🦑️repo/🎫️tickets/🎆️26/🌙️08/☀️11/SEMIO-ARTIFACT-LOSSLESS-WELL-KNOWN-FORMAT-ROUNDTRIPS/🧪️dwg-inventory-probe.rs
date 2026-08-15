extern crate semio_s_plugin_stdio;

use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::decode_dwg;
use std::collections::BTreeMap;

fn main() {
    let path = std::env::args().nth(1).expect("fixture path");
    let bytes = std::fs::read(path).expect("fixture read");
    let snapshot = decode_dwg(&bytes).expect("fixture decode");
    let mut counts = BTreeMap::<(u16, String), usize>::new();
    let mut handles = BTreeMap::<(u16, String), Vec<u64>>::new();
    for object in &snapshot.drawing.objects {
        *counts.entry((object.type_code, object.class_name.clone())).or_default() += 1;
        handles.entry((object.type_code, object.class_name.clone())).or_default().push(object.handle);
    }
    println!("objects={}", snapshot.drawing.objects.len());
    for ((type_code, class_name), count) in counts {
        let object_handles = &handles[&(type_code, class_name.clone())];
        println!("type={type_code} class={class_name} count={count} handles={object_handles:?}");
    }
    println!("classes={}", snapshot.classes.len());
    for class in &snapshot.classes {
        println!("class-number={} dxf={} cpp={} app={} proxy-flags={} item-class-id={}", class.number, class.dxf_name, class.cpp_class_name, class.application_name, class.proxy_flags, class.item_class_id);
    }
}
