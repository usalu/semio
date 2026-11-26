//! 🛠️ BIM authoring case `gestures-bim-1`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py` beside this file (`numpy` and `shapely` 2); this adapter
//! answers the same scenarios through the authoring tools (`✏️editor/🧵️gestures`): the typed-line grammar, the keyboard cursor, the fit and the frame of a wall-mounted component and
//! the route tool, and the platform compares both projections under `floating-point-v1`. It registers no oracle handler: a subject that re-read the committed expectation would be a
//! self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crates only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_bim_model_bim::editor::bim::fixture::{furnished, model, Rig};
    use semio_s_artifact_bim_model_coordination::editor::bim::gestures::typed::{parse, resolve};
    use semio_s_artifact_bim_model_dispatch_context::editor::bim::gestures::session::GestureKey;
    use semio_s_artifact_bim_model_gestures::editor::bim::commands::cursor_keys::{CursorDown, CursorDownFar, CursorLeft, CursorLeftFar, CursorRight, CursorRightFar, CursorUp, CursorUpFar};
    use semio_s_artifact_bim_model_gestures::editor::bim::gestures::route;
    use semio_s_artifact_bim_model_move_storey::editor::bim::gestures::place::{self, fit, Frame};
    use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
    use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Axis;
    use semio_s_artifact_bim_model_storey_levels::editor::bim::gestures::plane::point2;
    use semio_s_artifact_bim_model_storey_levels::editor::bim::marks::Shape;

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

    fn text(value: &Json) -> &str {
        match value {
            Json::String(found) => found,
            _ => "",
        }
    }

    fn pair(value: &Json) -> [f64; 2] {
        [number(&items(value)[0]), number(&items(value)[1])]
    }

    fn optional(value: &Json) -> Option<[f64; 2]> {
        (!matches!(value, Json::Null)).then(|| pair(value))
    }

    fn grid(value: f64) -> Json {
        Json::Number((value * 1e9).round() / 1e9 + 0.0)
    }

    fn list(values: &[f64]) -> Json {
        Json::Array(values.iter().map(|value| grid(*value)).collect())
    }

    fn object(members: Vec<(&str, Json)>) -> Json {
        Json::Object(members.into_iter().map(|(name, member)| (name.to_string(), member)).collect())
    }

    fn cases(ctx: &Context, section: &str) -> Result<Vec<Json>, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.ends_with("🛠️gestures/🔣️.json")).ok_or_else(|| "the scenario names no gesture cases".to_string())?;
        let all = parse_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed cases are not UTF-8: {error}"))?)?;
        Ok(items(key(&all, section)).to_vec())
    }

    fn answer(projection: Json) -> Result<Outcome, String> {
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// ⌨️ The point a typed line names, from the anchor and the point the pointer last was at.
    pub fn typed_point(ctx: &Context) -> Result<Outcome, String> {
        let index: usize = ctx.row()?.rsplit('-').next().unwrap_or_default().parse().map_err(|error| format!("the row names no typed case: {error}"))?;
        let case = cases(ctx, "typed")?.get(index).cloned().ok_or_else(|| "the committed cases hold no such typed line".to_string())?;
        let entry = parse(text(key(&case, "entry"))).ok_or_else(|| format!("{} does not parse", text(key(&case, "entry"))))?;
        let found = resolve(&entry, optional(key(&case, "anchor")), optional(key(&case, "toward"))).ok_or_else(|| format!("{} does not resolve", text(key(&case, "entry"))))?;
        answer(object(vec![("point", list(&found))]))
    }

    fn delta(name: &str) -> Result<[f64; 2], String> {
        Ok(match name {
            "left" => CursorLeft::DELTA,
            "right" => CursorRight::DELTA,
            "up" => CursorUp::DELTA,
            "down" => CursorDown::DELTA,
            "left_far" => CursorLeftFar::DELTA,
            "right_far" => CursorRightFar::DELTA,
            "up_far" => CursorUpFar::DELTA,
            "down_far" => CursorDownFar::DELTA,
            other => return Err(format!("no arrow key {other}")),
        })
    }

    /// ⌨️ The keyboard cursor stands where the arrow keys of each case leave it.
    pub fn cursor_point(ctx: &Context) -> Result<Outcome, String> {
        let mut points = Vec::new();
        for case in cases(ctx, "cursor")? {
            let start = pair(key(&case, "start"));
            let mut rig = Rig::plan("select", model());
            for step in items(key(&case, "steps")) {
                rig.nudge(delta(text(step))?, start);
            }
            points.push(list(&rig.session.cursor(start)));
        }
        answer(object(vec![("points", Json::Array(points))]))
    }

    /// 🪑️ A wall-mounted component stands on the face of the wall with its local depth axis pointing away from it.
    pub fn component_fit(ctx: &Context) -> Result<Outcome, String> {
        let fits = cases(ctx, "component_fits")?.iter().map(|case| {
            let axis = Axis::Line { start: point2(pair(key(key(case, "axis"), "start"))), end: point2(pair(key(key(case, "axis"), "end"))) };
            let (origin, yaw) = fit(&axis, number(key(case, "thickness")), pair(key(case, "point")));
            object(vec![("origin", list(&origin)), ("depth", list(&[-yaw.sin(), yaw.cos()]))])
        });
        answer(object(vec![("fits", Json::Array(fits.collect()))]))
    }

    /// 🧭️ The family frame is mirrored, then turned, then moved to its origin.
    pub fn component_frame(ctx: &Context) -> Result<Outcome, String> {
        let frames = cases(ctx, "component_frames")?.iter().map(|case| {
            let frame = Frame { origin: pair(key(case, "origin")), yaw: number(key(case, "yaw")).to_radians(), mirrored: matches!(key(case, "mirrored"), Json::Bool(true)) };
            Json::Array(items(key(case, "local")).iter().map(|local| list(&frame.world(pair(local)))).collect())
        });
        answer(object(vec![("worlds", Json::Array(frames.collect()))]))
    }

    /// 🔄️ The ghost is turned by each run of keys and the plan is clicked; the written component carries the summed rotation.
    pub fn component_turns(ctx: &Context) -> Result<Outcome, String> {
        let mut rotations = Vec::new();
        for case in cases(ctx, "turns")? {
            let mut rig = Rig::plan(place::UTILITY, furnished());
            rig.mv(2.0, 2.0);
            for step in items(key(&case, "steps")) {
                if !rig.key(GestureKey::Turn(number(step).to_radians())).0 {
                    return Err("the tool refused a turn key".into());
                }
            }
            let step = rig.down(2.0, 2.0);
            let [ModelMutation::CreateComponent(create)] = step.mutations.as_slice() else { return Err(format!("one create-component expected, got {:?}", step.mutations)) };
            let degrees = create.component.rotation.to_degrees().rem_euclid(360.0);
            rotations.push(grid(if degrees > 360.0 - 1e-9 { 0.0 } else { degrees }));
        }
        answer(object(vec![("degrees", Json::Array(rotations))]))
    }

    /// 🌀️ A route is written from its clicks; the rubber band measures it and the element holds the clicked vertices.
    pub fn route_length(ctx: &Context) -> Result<Outcome, String> {
        let mut routes = Vec::new();
        for case in cases(ctx, "routes")? {
            let points: Vec<[f64; 3]> = items(key(&case, "points")).iter().map(|point| [number(&items(point)[0]), number(&items(point)[1]), number(&items(point)[2])]).collect();
            let mut rig = Rig::plan(route::UTILITY, furnished());
            for point in &points {
                if rig.typed(&format!("z {}", point[2])).refused.is_some() {
                    return Err("the route tool refused an elevation".into());
                }
                rig.down(point[0], point[1]);
            }
            let last = points.last().ok_or_else(|| "a route without points".to_string())?;
            rig.mv(last[0], last[1]);
            let label = rig.preview.marks.iter().find(|mark| mark.shape == Shape::Label && mark.text.ends_with(" m")).map(|mark| mark.text.clone()).ok_or_else(|| "the rubber band shows no length".to_string())?;
            let step = rig.finish();
            let [ModelMutation::CreateMepElement(create)] = step.mutations.as_slice() else { return Err(format!("one create-mep-element expected, got {:?}", step.mutations)) };
            let path = create.mep.path.iter().map(|vertex| list(&[vertex.x, vertex.y, vertex.z]));
            routes.push(object(vec![("label", Json::String(label)), ("path", Json::Array(path.collect()))]));
        }
        answer(object(vec![("routes", Json::Array(routes))]))
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
            .subject("typed-point", subject::typed_point)
            .subject("cursor-point", subject::cursor_point)
            .subject("component-fit", subject::component_fit)
            .subject("component-frame", subject::component_frame)
            .subject("component-turns", subject::component_turns)
            .subject("route-length", subject::route_length);
    }
    built
}
//#endregion 🔖️Registration
