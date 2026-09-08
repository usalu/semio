mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn derive_from_geometry_classifies_box() {
        let mut kernel = Brep::new();
        let solid = kernel.box_prim(2.0, 2.0, 3.0).expect("box");
        let source = vec![CadObject {
            id: "object-box".into(),
            label: "Box".into(),
            typology: "spatial.shape.primitive.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([2.0, 2.0, 3.0]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        }];
        let derived = run_derive_from_geometry(&mut kernel, &source, "energy");
        let typos: Vec<_> = derived.iter().map(|o| o.typology.as_str()).collect();
        assert!(derived.iter().any(|object| object.typology == "energy.energy.hull"), "missing hull in {typos:?}");
        assert!(derived.iter().any(|object| object.typology == "energy.energy.roof" || object.typology == "energy.energy.baseplate"), "missing roof/baseplate in {typos:?}");
        assert!(derived.iter().any(|object| object.typology == "energy.energy.externalwall"), "missing wall in {typos:?}");
        assert!(derived.iter().any(|object| object.typology == "energy.energy.windows"), "missing windows in {typos:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn face_plane_group_key_is_stable() {
        let key = face_plane_group_key([0.0, 0.0, 1.0], [1.0, 2.0, 3.0]);
        assert!(key.starts_with("z:1:"));
    }
}
