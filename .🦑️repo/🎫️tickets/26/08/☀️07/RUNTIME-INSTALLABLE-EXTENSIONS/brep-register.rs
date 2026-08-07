        with_kernel_read(|kernel| {
            let geometry = read_geometry(input, "geometry")?;
            let deflection = read_channel_number(input, "deflection")?;
            let data = block_on(kernel.export_dwg(&[geometry], deflection)).map_err(map_kernel_error)?;
            Ok(channel_output("dwg", text_dictionary(encode_base64(&data))))
        })
    }
}

struct ImportDwg;
impl Operation for ImportDwg {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let data = decode_base64(&read_text(input, "data")?)?;
            let tolerance = read_channel_number(input, "tolerance")?;
            let handle = block_on(kernel.import_dwg(&data, tolerance)).map_err(map_kernel_error)?;
            Ok(channel_output("geometry", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️IO

/// 📦️ Registers brep geometry schema and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(geometry_schema());
    registry.register_schema(topology_element_schema("vertex", "Vertex", "emoji:📍️"));
    registry.register_schema(topology_element_schema("edge", "Edge", "emoji:〰"));
    registry.register_schema(topology_element_schema("face", "Face", "emoji:⬜️"));
    registry.register_schema(brep_schema());
    registry.register_schema(text_schema());
    registry.register_operator(
        OperatorInfo {
            id: "brep.brep".into(),
            extension: "brep".into(),
            name: "Brep".into(),
            abbreviation: "Brep".into(),
            icon: "emoji:🧊️".into(),
            summary: "Deconstructs B-Rep geometry into vertices, edges, and faces".into(),
            inputs: vec![geometry_channel("brep", "brep.brep")],
            outputs: vec![
                ChannelSpec::named("B", "Brep", "brep", "BrepGeometry").with_operators(vec!["brep.brep".into()]),
                topology_output("V", "Vtx", "vertex", "vertex"),
                topology_output("E", "Edg", "edge", "edge"),
                topology_output("F", "Fce", "face", "face"),
                ChannelSpec::list_output("errors", vec![]),
            ],
            group: vec!["Schemas".into()],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec!["geometry".into()], operation: Box::new(BrepDeconstruct) }],
        &["geometry", "list"],
    );

    reg_geo(
        registry,
        "brep.prim3d.box",
        "Box",
        "Box",
        "emoji:📦️",
        "Axis-aligned box solid",
        vec![number_channel("width", "brep.prim3d.box", 1.0), number_channel("depth", "brep.prim3d.box", 1.0), number_channel("height", "brep.prim3d.box", 1.0)],
        out_solid("BoxSolid"),
        &["Primitives 3D"],
        Box::new(BoxPrim),
    );
    reg_geo(registry, "brep.prim3d.sphere", "Sphere", "Sphere", "emoji:⚪️", "Sphere solid", vec![number_channel("radius", "brep.prim3d.sphere", 1.0)], out_solid("SphereSolid"), &["Primitives 3D"], Box::new(SpherePrim));
    reg_geo(
        registry,
        "brep.prim3d.cylinder",
        "Cylinder",
        "Cylinder",
        "emoji:🛢️",
        "Cylinder solid",
        vec![number_channel("radius", "brep.prim3d.cylinder", 1.0), number_channel("height", "brep.prim3d.cylinder", 1.0)],
        out_solid("CylinderSolid"),
        &["Primitives 3D"],
        Box::new(CylinderPrim),
    );
    reg_geo(
        registry,
        "brep.prim3d.cone",
        "Cone",
        "Cone",
        "emoji:🛢️",
        "Cone solid",
        vec![number_channel("radius", "brep.prim3d.cone", 1.0), number_channel("height", "brep.prim3d.cone", 1.0)],
        out_solid("ConeSolid"),
        &["Primitives 3D"],
        Box::new(ConePrim),
    );
    reg_geo(
        registry,
        "brep.prim3d.torus",
        "Torus",
        "Torus",
        "emoji:🛢️",
        "Torus solid",
        vec![number_channel("major", "brep.prim3d.torus", 2.0), number_channel("minor", "brep.prim3d.torus", 0.5)],
        out_solid("TorusSolid"),
        &["Primitives 3D"],