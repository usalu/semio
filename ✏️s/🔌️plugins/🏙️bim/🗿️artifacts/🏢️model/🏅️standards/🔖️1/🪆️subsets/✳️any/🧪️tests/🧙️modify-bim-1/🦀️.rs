//! 🧙️ BIM modify case `modify-bim-1`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py` beside this file (`shapely` 2 / GEOS and `numpy`); this adapter
//! answers the same scenarios from the modify kernel (`🧬️schema/🧬️mutations/🧙️modify`) and the wall layout, and the platform compares both projections under `floating-point-v1`.
//! It registers no oracle handler: a subject that re-read the committed expectation would be a self-comparison reporting a pass. Quantities that meet shapely only through
//! sampled arcs are compared on a rounded grid of both sides.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crates only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_geometry::{loops, Point};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone::compute_wall_layout;
    use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::cut::{cut_loop, offset_axis, trim_extend};
    use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::{map, ArrayPattern, Map, WallEnd};
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Axis, EndJoin, Layer, LayerFunction, LocationLine, Point2, Vertex};
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Component, ModelSnapshot, Wall, WallType};

    const NOTHING: Json = Json::Null;

    fn key<'a>(value: &'a Json, name: &str) -> &'a Json {
        value.get(name).unwrap_or(&NOTHING)
    }

    fn items(value: &Json) -> &[Json] {
        match value {
            Json::Array(members) => members,
            _ => &[],
        }
    }

    fn number(value: &Json) -> f64 {
        match value {
            Json::Number(found) => *found,
            _ => f64::NAN,
        }
    }

    fn point(value: &Json) -> Point2 {
        Point2 { x: number(&items(value)[0]), y: number(&items(value)[1]) }
    }

    fn pair(at: Point2) -> Json {
        Json::Array(vec![Json::Number(at.x), Json::Number(at.y)])
    }

    fn unit(angle: f64) -> Json {
        Json::Array(vec![Json::Number(angle.cos()), Json::Number(angle.sin())])
    }

    fn grid(value: f64, places: i32) -> f64 {
        let scale = 10f64.powi(places);
        (value * scale).round() / scale
    }

    fn gridded(at: Point2, places: i32) -> Json {
        pair(Point2 { x: grid(at.x, places), y: grid(at.y, places) })
    }

    fn object(members: Vec<(&str, Json)>) -> Json {
        Json::Object(members.into_iter().map(|(name, member)| (name.to_string(), member)).collect())
    }

    fn cases(ctx: &Context) -> Result<Json, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.ends_with("🧙️modify/🔣️.json")).ok_or_else(|| "the scenario names no modify cases".to_string())?;
        parse_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed cases are not UTF-8: {error}"))?)
    }

    fn row(ctx: &Context) -> Result<(String, usize), String> {
        let row = ctx.row()?;
        let (family, index) = row.rsplit_once('-').unwrap_or(("", row));
        Ok((family.to_string(), index.parse().map_err(|error| format!("row {row} names no case index: {error}"))?))
    }

    fn case(ctx: &Context, family_of: impl Fn(&Json, &str) -> Json) -> Result<(String, Json), String> {
        let (family, index) = row(ctx)?;
        items(&family_of(&cases(ctx)?, &family)).get(index).cloned().map(|found| (family, found)).ok_or_else(|| format!("the committed cases hold no case for row {}", ctx.row().unwrap_or_default()))
    }

    fn section(name: &'static str) -> impl Fn(&Json, &str) -> Json {
        move |all, _| key(all, name).clone()
    }

    fn map_of(family: &str, case: &Json) -> Map {
        match family {
            "translate" => Map::Translate(point(key(case, "vector"))),
            "rotate" => Map::Rotate { pivot: point(key(case, "pivot")), angle: number(key(case, "angle")) },
            _ => Map::Mirror { start: point(&items(key(case, "line"))[0]), end: point(&items(key(case, "line"))[1]) },
        }
    }

    fn loop_of(value: &Json) -> Vec<Vertex> {
        items(value).iter().map(|vertex| Vertex { point: point(vertex), bulge: number(&items(vertex)[2]) }).collect()
    }

    fn inner(vertices: &[Vertex]) -> Vec<loops::Vertex> {
        vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
    }

    fn axis_of(value: &Json) -> Axis {
        let (start, end, bulge) = (point(key(value, "start")), point(key(value, "end")), number(key(value, "bulge")));
        if bulge == 0.0 { Axis::Line { start, end } } else { Axis::Arc { start, end, bulge } }
    }

    fn answer(projection: Json) -> Result<Outcome, String> {
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🗺️ The images of the committed points and directions under the translation, rotation or reflection of the row.
    pub fn maps(ctx: &Context) -> Result<Outcome, String> {
        let (family, case) = case(ctx, |all, family| key(key(all, "maps"), family).clone())?;
        let map = map_of(&family, &case);
        answer(object(vec![
            ("images", Json::Array(items(key(&case, "points")).iter().map(|at| pair(map.point(point(at)))).collect())),
            ("turned", Json::Array(items(key(&case, "directions")).iter().map(|theta| unit(map.turn(number(theta)))).collect())),
        ]))
    }

    /// 🪑️ A free-standing component goes where its origin and its family frame go under the map of the case.
    pub fn frames(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("frames"))?;
        let map = map_of(key(&case, "kind").str_of(), &case);
        let images = items(key(&case, "instances")).iter().map(|instance| {
            let row = Component { storey: "st".into(), family: "fam".into(), position: point(key(instance, "position")), elevation: 0.0, rotation: number(key(instance, "rotation")), mirrored: matches!(key(instance, "mirrored"), Json::Bool(true)), host: None, system: None, name: "C".into() };
            let image = map::component(&row, &map);
            object(vec![("position", pair(image.position)), ("rotation", unit(image.rotation)), ("mirrored", Json::Bool(image.mirrored))])
        });
        answer(object(vec![("images", Json::Array(images.collect()))]))
    }

    /// ➰️ A mirrored bulged loop keeps its area, runs counter-clockwise and has its centroid mirrored.
    pub fn loops(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("loops"))?;
        let (loop_, map) = (loop_of(key(&case, "loop")), map_of("mirror", &case));
        let image = inner(&map::loop_image(&loop_, &map));
        let (original, centroid, image_centroid) = (inner(&loop_), loops::centroid(&inner(&loop_)), loops::centroid(&image));
        answer(object(vec![
            ("area", Json::Number(grid(loops::signed_area(&original), 4))),
            ("image_area", Json::Number(grid(loops::signed_area(&image), 4))),
            ("centroid", gridded(Point2 { x: centroid.x, y: centroid.y }, 4)),
            ("image_centroid", gridded(Point2 { x: image_centroid.x, y: image_centroid.y }, 4)),
            ("image_counter_clockwise", Json::Bool(loops::is_ccw(&image))),
        ]))
    }

    /// 〰️ The wall axis moves to the parallel curve.
    pub fn offsets(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("offsets"))?;
        let found = offset_axis(&axis_of(key(&case, "axis")), number(key(&case, "distance"))).ok_or_else(|| "the offset does not exist".to_string())?;
        let (start, end, bulge) = match found {
            Axis::Line { start, end } => (start, end, 0.0),
            Axis::Arc { start, end, bulge } => (start, end, bulge),
        };
        answer(object(vec![("start", gridded(start, 6)), ("end", gridded(end, 6)), ("bulge", Json::Number(grid(bulge, 6)))]))
    }

    /// 🎯️ The named end lands on the intersection with the carrier of the target.
    pub fn trims(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("trims"))?;
        let end = if key(&case, "end").str_of() == "Start" { WallEnd::Start } else { WallEnd::End };
        let trimmed = trim_extend(&axis_of(key(&case, "axis")), end, &axis_of(key(&case, "target"))).map_err(|error| format!("the end does not move: {}", error.message()))?;
        let (Axis::Line { start, end: finish } | Axis::Arc { start, end: finish, .. }) = trimmed.axis;
        answer(object(vec![("point", pair(if end == WallEnd::Start { start } else { finish }))]))
    }

    /// ✂️ A cut along a line leaves two halves.
    pub fn splits(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("splits"))?;
        let line = items(key(&case, "line"));
        let cut = cut_loop(&loop_of(key(&case, "loop")), point(&line[0]), point(&line[1])).map_err(|error| format!("the loop is not cut: {}", error.message()))?;
        let (left, right) = (inner(&cut.left), inner(&cut.right));
        let centre = |half: &[loops::Vertex]| {
            let found = loops::centroid(half);
            gridded(Point2 { x: found.x, y: found.y }, 6)
        };
        answer(object(vec![
            ("left_area", Json::Number(grid(loops::signed_area(&left), 6))),
            ("right_area", Json::Number(grid(loops::signed_area(&right), 6))),
            ("left_centroid", centre(&left)),
            ("right_centroid", centre(&right)),
        ]))
    }

    /// 🌀️ The copies of a radial array sit at the k-th turn of the source.
    pub fn arrays(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("arrays"))?;
        let count = number(key(&case, "count")) as u32;
        let pattern = ArrayPattern::Radial { count, center: point(key(&case, "center")), step: number(key(&case, "step")) };
        answer(object(vec![("images", Json::Array((1..=count).map(|copy| pair(pattern.map_of(copy).point(point(key(&case, "point"))))).collect()))]))
    }

    fn joined_model(ctx: &Context, case: &Json) -> Result<ModelSnapshot, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("🔗️wall-joins")).ok_or_else(|| "the scenario names no joins model".to_string())?;
        let mut snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed model is not UTF-8: {error}"))?)?;
        let template = snapshot.walls.values().next().ok_or_else(|| "the joins model has no wall".to_string())?.clone();
        let material = snapshot.materials.keys().next().ok_or_else(|| "the joins model has no material".to_string())?.clone();
        snapshot.walls.cold_clear();
        snapshot.openings.cold_clear();
        for wall in items(key(case, "walls")) {
            let thickness = number(key(wall, "thickness"));
            let kind = format!("wt-{thickness}");
            snapshot.wall_types.cold_insert(kind.clone(), WallType { name: kind.clone(), layers: vec![Layer { material: material.clone(), thickness, function: LayerFunction::Structure }] });
            let join = |name: &str| match key(wall, name).str_of() {
                "Miter" => Some(EndJoin::Miter),
                "Butt" => Some(EndJoin::Butt),
                "None" => Some(EndJoin::None),
                _ => None,
            };
            let axis = Axis::Line { start: point(key(wall, "start")), end: point(key(wall, "end")) };
            snapshot.walls.cold_insert(key(wall, "id").str_of().to_string(), Wall { wall_type: kind, axis, location: LocationLine::Center, start_join: join("start_join"), end_join: join("end_join"), ..template.clone() });
        }
        Ok(snapshot)
    }

    /// 🧱️ The walls of the case resolve to the footprints of their authored end joins.
    pub fn joins(ctx: &Context) -> Result<Outcome, String> {
        let (_, case) = case(ctx, section("joins"))?;
        let layouts = compute_wall_layout(&joined_model(ctx, &case)?);
        let footprints = items(key(&case, "walls")).iter().map(|wall| {
            let id = key(wall, "id").str_of();
            let corners = layouts.get(id).map(|layout| layout.footprint.iter().map(|corner| pair(corner.point)).collect()).unwrap_or_default();
            (id.to_string(), Json::Array(corners))
        });
        answer(object(vec![("footprints", Json::Object(footprints.collect()))]))
    }

    trait Text {
        fn str_of(&self) -> &str;
    }

    impl Text for Json {
        fn str_of(&self) -> &str {
            match self {
                Json::String(found) => found,
                _ => "",
            }
        }
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; an outline id serves every row its Examples table expands.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("modify-maps", subject::maps)
            .subject("modify-frames", subject::frames)
            .subject("modify-loops", subject::loops)
            .subject("modify-offsets", subject::offsets)
            .subject("modify-trims", subject::trims)
            .subject("modify-splits", subject::splits)
            .subject("modify-arrays", subject::arrays)
            .subject("modify-joins", subject::joins);
    }
    built
}
//#endregion 🔖️Registration
