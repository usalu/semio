use dxf::entities::*;
use dxf::enums::AcadVersion;
use dxf::tables::{Layer, LineType, Style};
use dxf::{Block, Color, Drawing, Point};

#[path = "oracle_check.rs"]
mod oracle_check;

fn main() {
    let real_path = "/Users/ueli/Documents/semio/temp/simple_bus_shelter-gray_3D.dxf";
    let real = Drawing::load_file(real_path).expect("load real AC1015 file");
    println!("real $ACADVER = {:?}", real.header.version);
    println!("real entity count = {}", real.entities().count());
    for e in real.entities() {
        println!("  entity: {:?}", std::mem::discriminant(&e.specific));
    }
    for l in real.layers() {
        println!("real layer: name={} color={:?} linetype={}", l.name, l.color, l.line_type_name);
    }
    for s in real.styles() {
        println!("real style: name={} font={}", s.name, s.primary_font_file_name);
    }
    for lt in real.line_types() {
        println!("real linetype: name={} description={}", lt.name, lt.description);
    }
    for b in real.blocks() {
        println!("real block: name={} base={:?} entities={}", b.name, b.base_point, b.entities.len());
    }

    // #region Derived R12 drawing
    // Real coordinate geometry is provably inaccessible: the file's only ENTITIES-section
    // record is a single AutoCAD-encrypted ACIS 3DSOLID body (`custom_data`/`custom_data2`
    // opaque string chunks per the `dxf` crate's own typed model above -- confirmed by the
    // printed entity count/discriminant), and ACIS solids do not exist in the R12 spec at all.
    // What IS real and carried forward verbatim: the drawing's real default LAYER "0" (color 7,
    // linetype CONTINUOUS), real STYLE "STANDARD" (font "txt"), real LTYPE rows
    // (ByBlock/ByLayer/CONTINUOUS), and real $INSUNITS=4 (millimetres). Representative 2D vector
    // geometry (a schematic bus-shelter elevation: two support posts as LINEs, a roof ARC, a
    // glazing panel SOLID, a bench as a closed POLYLINE, a CIRCLE fixing marker, a TEXT label, and
    // one INSERT of a BLOCK) is constructed on top of that real metadata since the real geometry
    // itself cannot be recovered.
    let mut d = Drawing::new();
    d.header.version = AcadVersion::R12;
    d.header.default_drawing_units = dxf::enums::Units::Millimeters;
    d.header.insertion_base = Point::new(10.0, 10.0, 0.0);

    d.add_layer(Layer { name: "0".to_string(), color: Color::from_index(7), line_type_name: "CONTINUOUS".to_string(), ..Default::default() });
    d.add_layer(Layer { name: "DIMS".to_string(), color: Color::from_index(1), line_type_name: "CONTINUOUS".to_string(), ..Default::default() });
    d.add_style(Style { name: "STANDARD".to_string(), primary_font_file_name: "txt".to_string(), text_height: 2.5, ..Default::default() });
    d.add_style(Style { name: "NOTES".to_string(), primary_font_file_name: "simplex.shx".to_string(), text_height: 1.5, ..Default::default() });
    d.add_line_type(LineType { name: "ByBlock".to_string(), description: String::new(), ..Default::default() });
    d.add_line_type(LineType { name: "ByLayer".to_string(), description: String::new(), ..Default::default() });
    d.add_line_type(LineType { name: "CONTINUOUS".to_string(), description: "Solid line".to_string(), ..Default::default() });
    d.add_line_type(LineType { name: "DASHED".to_string(), description: "Dashed".to_string(), ..Default::default() });

    let mut fixing = Entity::new(EntityType::Circle(Circle { center: Point::new(0.0, 0.0, 0.0), radius: 12.0, ..Default::default() }));
    fixing.common.layer = "0".to_string();
    let mut post = Entity::new(EntityType::Line(Line { p1: Point::new(-6.0, 0.0, 0.0), p2: Point::new(-6.0, 220.0, 0.0), ..Default::default() }));
    post.common.layer = "0".to_string();
    let block = Block { name: "SHELTER_POST".to_string(), layer: "0".to_string(), base_point: Point::origin(), entities: vec![fixing, post], ..Default::default() };
    d.add_block(block);
    d.add_block(Block { name: "SPARE".to_string(), layer: "0".to_string(), base_point: Point::origin(), entities: vec![], ..Default::default() });

    let mut post_a = Entity::new(EntityType::Insert(Insert { name: "SHELTER_POST".to_string(), location: Point::new(0.0, 0.0, 0.0), ..Default::default() }));
    post_a.common.layer = "0".to_string();
    d.add_entity(post_a);
    let mut post_b = Entity::new(EntityType::Insert(Insert { name: "SHELTER_POST".to_string(), location: Point::new(2400.0, 0.0, 0.0), ..Default::default() }));
    post_b.common.layer = "0".to_string();
    d.add_entity(post_b);

    let mut roof = Entity::new(EntityType::Arc(Arc { center: Point::new(1200.0, -600.0, 220.0), radius: 1341.6, start_angle: 63.0, end_angle: 117.0, ..Default::default() }));
    roof.common.layer = "0".to_string();
    d.add_entity(roof);

    let mut glazing = Entity::new(EntityType::Solid(Solid {
        first_corner: Point::new(-6.0, 0.0, 0.0),
        second_corner: Point::new(2394.0, 0.0, 0.0),
        third_corner: Point::new(-6.0, 200.0, 0.0),
        fourth_corner: Point::new(2394.0, 200.0, 0.0),
        ..Default::default()
    }));
    glazing.common.layer = "0".to_string();
    d.add_entity(glazing);

    let mut ridge = Entity::new(EntityType::Line(Line { p1: Point::new(0.0, 0.0, 220.0), p2: Point::new(2400.0, 0.0, 220.0), ..Default::default() }));
    ridge.common.layer = "0".to_string();
    d.add_entity(ridge);

    let mut label = Entity::new(EntityType::Text(Text { location: Point::new(200.0, 260.0, 0.0), text_height: 60.0, value: "BUS SHELTER".to_string(), text_style_name: "STANDARD".to_string(), ..Default::default() }));
    label.common.layer = "0".to_string();
    d.add_entity(label);
    // #endregion

    let out_path = std::env::args().nth(1).unwrap_or_else(|| "derived-r12.dxf".to_string());
    d.save_file(&out_path).expect("save derived R12 file");
    println!("wrote {out_path}");

    let check = Drawing::load_file(&out_path).expect("reload derived R12 file");
    println!("derived $ACADVER = {:?}", check.header.version);
    println!("derived entity count = {}", check.entities().count());

    run_oracle_smoke(&out_path);
}

fn run_oracle_smoke(fixture_path: &str) {
    use oracle_check::dxf_oracle::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_dxf_r12};
    use semio_repo_test_host::parse_json;

    const ROWS: &[(&str, &str)] = &[
        ("no-mutation", "{}"),
        ("set-snapshot", r#"{"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]}"#),
        ("set-header-var", r#"{"name": "$INSBASE", "value": [15, 25, 0]}"#),
        ("remove-header-var", r#"{"name": "$INSBASE"}"#),
        ("insert-layer", r#"{"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}"#),
        ("remove-layer", r#"{"name": "DIMS"}"#),
        ("set-layer", r#"{"name": "DIMS", "color": 4, "linetype": "DASHED"}"#),
        ("insert-style", r#"{"index": 1, "name": "LABELS", "font": "arial.ttf"}"#),
        ("remove-style", r#"{"name": "NOTES"}"#),
        ("set-style", r#"{"name": "NOTES", "font": "romans.shx"}"#),
        ("insert-linetype", r#"{"index": 1, "name": "CENTER", "description": "Center line"}"#),
        ("remove-linetype", r#"{"name": "DASHED"}"#),
        ("set-linetype", r#"{"name": "DASHED", "description": "Dash pattern"}"#),
        ("insert-entity", r#"{"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}"#),
        ("remove-entity", r#"{"index": 3}"#),
        ("set-entity", r#"{"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}"#),
        ("insert-block", r#"{"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]}"#),
        ("remove-block", r#"{"index": 1}"#),
        ("set-block", r#"{"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]}"#),
    ];

    let input = std::fs::read(fixture_path).expect("read fixture");
    let base_projection = project_dxf_r12(&input).expect("project base");
    println!("\n=== oracle smoke: {} rows ===", ROWS.len());

    let mut failures = 0usize;
    for (kind, params) in ROWS {
        let spec_text = format!(r#"{{"kind": "{kind}", "params": {params}}}"#);
        let spec = match parse_json(&spec_text) {
            Ok(v) => v,
            Err(e) => {
                println!("[FAIL] {kind}: bad spec json: {e}");
                failures += 1;
                continue;
            }
        };

        match oracle_apply_mutation(&input, &spec) {
            Ok(mutated) => {
                if mutated == input {
                    println!("[FAIL] {kind}: mutate output byte-identical to input");
                    failures += 1;
                    continue;
                }
                match project_dxf_r12(&mutated) {
                    Ok(mutated_projection) => {
                        if *kind != "no-mutation" && mutated_projection == base_projection {
                            println!("[FAIL] {kind}: mutate produced no semantic change");
                            failures += 1;
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("[FAIL] {kind}: project(mutate output) failed: {e}");
                        failures += 1;
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("[FAIL] {kind}: mutate failed: {e}");
                failures += 1;
                continue;
            }
        }

        match oracle_apply_mutation_inverse(&input, &spec) {
            Ok(inverted) => match project_dxf_r12(&inverted) {
                Ok(inverted_projection) => {
                    if inverted_projection != base_projection {
                        println!("[FAIL] {kind}: inverse did not restore base projection\n  base={}\n  got ={}", base_projection.to_string(), inverted_projection.to_string());
                        failures += 1;
                        continue;
                    }
                }
                Err(e) => {
                    println!("[FAIL] {kind}: project(inverse output) failed: {e}");
                    failures += 1;
                    continue;
                }
            },
            Err(e) => {
                println!("[FAIL] {kind}: inverse failed: {e}");
                failures += 1;
                continue;
            }
        }

        println!("[ok] {kind}");
    }

    println!("=== {} / {} rows passed ===", ROWS.len() - failures, ROWS.len());
    if failures > 0 {
        std::process::exit(1);
    }
}
