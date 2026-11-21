//! 📐️ Typed elevation sample, brush, and grid actions.

use crate::mutations::{GisTerrainMutation, ReplaceElevationSource};
use crate::schema::snapshot::ElevationSource;
use crate::GisTerrainSnapshot;
use semio_s_artifact_gis_gisterrain_inferences::elevation_edit::{ElevationEdit, ElevationEditMode as Operation, LAST_SAMPLE, MAX_ELEVATION, MAX_STROKE_SAMPLES, MIN_ELEVATION};
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_plugin::{ActionArgDef, ActionArgOption, ArgSchema, ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_ui_locale::LocalizedLabel;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "edit-elevation")]
pub struct EditElevation {
    pub operation: String,
    pub column: u32,
    pub row: u32,
    pub radius: u32,
    pub value: i32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Vec<[u32; 2]>>,
}

const OPERATIONS: [(Operation, &str, &str, &str); 5] = [
    (Operation::SetSample, "set-sample", "Set Sample Height", "Punkthöhe festlegen"),
    (Operation::RaiseLower, "raise-lower", "Raise or Lower Area", "Bereich anheben oder absenken"),
    (Operation::Flatten, "flatten", "Grade to Elevation", "Auf Zielhöhe nivellieren"),
    (Operation::Smooth, "smooth", "Smooth Area", "Bereich glätten"),
    (Operation::Offset, "offset", "Offset Entire Grid", "Gesamtes Raster verschieben"),
];

impl EditElevation {
    fn edit(&self) -> Result<ElevationEdit, Fault> {
        let mode = OPERATIONS.iter().find(|(_, key, _, _)| *key == self.operation)
            .map(|(operation, _, _, _)| *operation).ok_or_else(|| Fault::from("gis-terrain-elevation-operation-unknown"))?;
        Ok(ElevationEdit { mode, column: self.column, row: self.row, radius: self.radius, value: self.value })
    }

    /// 🎛️ Admits a staged command without truncating fractional values.
    pub fn from_action(args: &DslValue) -> Result<Self, Fault> {
        let integer = |key: &str| args.get(key).and_then(|value| {
            let number = value.as_f64()?;
            (number.fract() == 0.0 && (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&number)).then_some(number as i32)
        }).ok_or_else(|| Fault::from(format!("gis-terrain-elevation-edit-invalid-{key}")));
        let index = |key: &str| u32::try_from(integer(key)?).map_err(|_| Fault::from(format!("gis-terrain-elevation-edit-invalid-{key}")));
        let invalid_stroke = || Fault::from("gis-terrain-elevation-stroke-invalid");
        let stroke = args.get("stroke").map(|value| {
            let samples = value.as_array().filter(|samples| (1..=MAX_STROKE_SAMPLES).contains(&samples.len())).ok_or_else(invalid_stroke)?;
            samples.iter().map(|sample| {
                let Some([column, row]) = sample.as_array() else { return Err(invalid_stroke()) };
                let coordinate = |value: &DslValue| {
                    value.as_f64().filter(|value| value.fract() == 0.0 && (0.0..=f64::from(LAST_SAMPLE)).contains(value))
                        .map(|value| value as u32).ok_or_else(invalid_stroke)
                };
                Ok([coordinate(column)?, coordinate(row)?])
            }).collect::<Result<Vec<_>, Fault>>()
        }).transpose()?;
        let command = Self {
            operation: args.get("operation").and_then(DslValue::as_str).ok_or_else(|| Fault::from("gis-terrain-elevation-operation-required"))?.into(),
            column: index("column")?,
            row: index("row")?,
            radius: index("radius")?,
            value: integer("value")?,
            stroke,
        };
        command.validate()?;
        Ok(command)
    }

    /// 🎟️ Validates bounded direct command admission.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    fn validate(&self) -> Result<(), Fault> {
        let edit = self.edit()?;
        let result = match self.stroke.as_deref() {
            Some(samples) => edit.validate_stroke(samples).map(|_| ()),
            None => edit.validate().map(|_| ()),
        };
        result.map_err(|error| Fault::from(error.message.into_owned()))
    }
}

/// 🎚️ Stages bounded coordinate, radius and strength fields in the standard Actions UI.
pub fn action_arguments() -> Vec<ActionArgDef> {
    let numbers = |id, en, de, minimum, maximum, default| {
        ActionArgDef::with_schema(id, LocalizedLabel::native(en, de), ArgSchema::number(Some(minimum), Some(maximum), Some(1.0), true)).default_value(&default)
    };
    let options = OPERATIONS.iter().map(|(_, id, en, de)| ActionArgOption::new(*id, LocalizedLabel::native(en, de))).collect();
    let mut stroke = ActionArgDef::with_schema(
        "stroke", LocalizedLabel::native("Terrain Brush Stroke", "Geländepinselstrich"),
        ArgSchema::Array {
            items: Box::new(ArgSchema::Array {
                items: Box::new(ArgSchema::number(Some(0.0), Some(f64::from(LAST_SAMPLE)), None, true)),
                min_items: Some(2),
                max_items: Some(2),
            }),
            min_items: Some(1),
            max_items: Some(MAX_STROKE_SAMPLES as u32),
        },
    );
    stroke.presentation = Some(semio_plugin::ArgPresentation::Hidden);
    vec![
        ActionArgDef::select("operation", LocalizedLabel::native("Operation", "Vorgang"), options).default_value(&"raise-lower"),
        numbers("column", "Column from West", "Spalte ab Westen", 0.0, f64::from(LAST_SAMPLE), 16_i32),
        numbers("row", "Row from South", "Zeile ab Süden", 0.0, f64::from(LAST_SAMPLE), 16),
        numbers("radius", "Radius in Samples", "Radius in Rasterpunkten", 0.0, f64::from(LAST_SAMPLE), 4),
        numbers("value", "Meters or Smoothing Percent", "Meter oder Glättungsprozent", f64::from(MIN_ELEVATION), f64::from(MAX_ELEVATION), 25),
        stroke,
    ]
}

fn apply(source: &ElevationSource, command: &EditElevation) -> Result<ElevationSource, Fault> {
    let edit = command.edit()?;
    let edited = match command.stroke.as_deref() {
        Some(samples) => edit.apply_stroke(source, samples),
        None => edit.apply(source),
    };
    edited.map_err(|error| Fault::from(error.message.into_owned()))
}

/// 🧾️ Publishes the edited grid as one reversible artifact mutation.
pub fn handle(payload: &EditElevation, doc: &ArtifactView<'_, GisTerrainSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, Fault> {
    let source = doc.snapshot.elevation_source.as_ref().ok_or_else(|| Fault::from("gis-terrain-elevation-source-required"))?;
    let next = apply(source, payload)?;
    Ok(elevation_emit(source, next))
}

fn elevation_emit(source: &ElevationSource, next: ElevationSource) -> Emit<GisTerrainMutation, NoConfigMutation> {
    if next == *source {
        return Emit::default();
    }
    Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: Some(next) })])
}

type Editor = semio_plugin::EditorApp<crate::editor::gis3d::Gis3dPlayApp>;

/// 🖌️ Meters the bounded stroke as one cancellable artifact command.
#[derive(Default)]
pub struct StrokeWork {
    consumed: bool,
}

impl semio_plugin::retained_command::ArtifactCommandWork<Editor> for StrokeWork {
    fn tool_id(&self) -> &'static str { "editElevation" }

    fn extent(&self, command: &crate::editor::gis3d::Gis3dCommand, snapshot: &GisTerrainSnapshot, interaction: &semio_os_kernel::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<Editor>>) -> Option<usize> {
        crate::editor::gis3d::gis3d_retained_extent(command, snapshot, interaction)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }

    fn step(
        &mut self, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, Editor>,
        cx: &mut semio_plugin::__job::StepContext<'_>,
    ) -> Result<semio_plugin::retained_command::ArtifactCommandWorkStep<Editor>, semio_plugin::__job::OriginalTurnFailure> {
        use semio_plugin::retained_command::ArtifactCommandWorkStep;
        let crate::editor::gis3d::Gis3dCommand::EditElevation(payload) = input.command else {
            return Err(Fault::from("gis-terrain-elevation-stroke-invalid").into());
        };
        let samples = payload.stroke.as_deref().ok_or_else(|| Fault::from("gis-terrain-elevation-stroke-invalid"))?;
        let source = input.snapshot.elevation_source.as_ref().ok_or_else(|| Fault::from("gis-terrain-elevation-source-required"))?;
        let edit = payload.edit()?;
        let fault = |error: semio_framework_value::ValueError| Fault::from(error.message.into_owned());
        let work = edit.stroke_work(samples).map_err(fault)?;
        if work >= cx.original_turn_ceiling().1 || work >= cx.original_total_fuel_remaining() {
            return Err(Fault::from("gis-terrain-elevation-stroke-work-limit").into());
        }
        if work >= cx.fuel_remaining() {
            return Ok(ArtifactCommandWorkStep::Pending {
                stage: "gis-terrain-brush-stroke",
                preview: r#"{"en":"Applying terrain brush","de":"Geländepinsel wird angewendet"}"#.as_bytes(),
            });
        }
        if self.consumed || cx.is_cancelled() || cx.deadline_exceeded() {
            return Err(Fault::from("gis-terrain-elevation-stroke-cancelled").into());
        }
        self.consumed = true;
        cx.consume_fuel(work);
        let next = edit.apply_stroke_with(source, samples, || !cx.is_cancelled() && !cx.deadline_exceeded()).map_err(fault)?;
        if cx.is_cancelled() || cx.deadline_exceeded() {
            return Err(Fault::from("gis-terrain-elevation-stroke-cancelled").into());
        }
        Ok(ArtifactCommandWorkStep::Complete(elevation_emit(source, next)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn raster_edits_match_the_neutral_vectors() {
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/📐️edit-elevation/🧪️tests/🔣️.json")).unwrap();
        let size = fixture["size"].as_u64().unwrap() as usize;
        let height = fixture["height"].as_i64().unwrap() as i16;
        let mut source = ElevationSource {
            bounds: serde_json::from_value(fixture["bounds"].clone()).unwrap(),
            rows: vec![vec![height; size]; size],
        };
        for seed in fixture["seed"].as_array().unwrap() {
            let [column, row, height] = serde_json::from_value::<[i32; 3]>(seed.clone()).unwrap();
            source.rows[row as usize][column as usize] = height as i16;
        }
        let decode = |value: &Value| semio_framework_pack_json::from_json_str::<EditElevation>(
            &value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let edited = apply(&source, &decode(&case["command"])).unwrap();
            let changed = edited.rows.iter().flatten().zip(source.rows.iter().flatten()).filter(|(a, b)| a != b).count();
            assert_eq!(changed, case["changed"].as_u64().unwrap() as usize, "{}", case["command"]);
            assert_eq!(edited.bounds, source.bounds);
            for sample in case["samples"].as_array().unwrap() {
                let [column, row, height] = serde_json::from_value::<[i32; 3]>(sample.clone()).unwrap();
                assert_eq!(i32::from(edited.rows[row as usize][column as usize]), height, "{}", case["command"]);
            }
        }
        for command in fixture["refused"].as_array().unwrap() {
            assert!(apply(&source, &decode(command)).is_err(), "{command}");
        }
        assert_eq!(source.rows[16][16], 190);
        for case in fixture["strokeGestures"].as_array().unwrap() {
            if let Some(anchors) = case["anchors"].as_array() {
                let samples: Vec<[u32; 2]> = serde_json::from_value(anchors.clone().into()).unwrap();
                let [column, row] = samples[0];
                let edit = ElevationEdit { mode: Operation::RaiseLower, column, row, radius: case["radius"].as_u64().unwrap() as u32, value: 1 };
                assert_eq!(serde_json::to_value(edit.stroke_stamps(&samples).unwrap()).unwrap(), case["stamps"]);
            }
        }
        for case in fixture["strokeEdits"].as_array().unwrap() {
            let args = semio_framework_pack_json::parse(&case["command"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let command = EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&args)).unwrap();
            let edited = apply(&source, &command).unwrap();
            let changed = edited.rows.iter().flatten().zip(source.rows.iter().flatten()).filter(|(a, b)| a != b).count();
            assert_eq!(changed, case["changed"].as_u64().unwrap() as usize);
            for sample in case["samples"].as_array().unwrap() {
                let [column, row, height] = serde_json::from_value::<[i32; 3]>(sample.clone()).unwrap();
                assert_eq!(i32::from(edited.rows[row as usize][column as usize]), height);
            }
        }
        for command in fixture["refusedStrokes"].as_array().unwrap() {
            let parsed = semio_framework_pack_json::parse(&command.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert!(EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&parsed)).is_err(), "{command}");
        }
        let float_wire = semio_framework_pack_json::parse(
            r#"{"operation":"smooth","column":8.0,"row":8.0,"radius":0.0,"value":100.0,"stroke":[[8.0,8.0],[10.0,8.0]]}"#,
            semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap();
        let admitted = EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&float_wire)).unwrap();
        let typed = semio_framework_value::ToValue::to_value(&admitted);
        let point = &typed.get("stroke").unwrap().as_array().unwrap()[0].as_array().unwrap()[0];
        assert_eq!(point.as_u64(), Some(8));
        assert_eq!(<EditElevation as semio_framework_value::FromValue>::from_value(typed).unwrap(), admitted);

        let edit = ElevationEdit { mode: Operation::Smooth, column: 0, row: 0, radius: 1, value: 100 };
        let entire_budget = [[0, 0], [32, 0], [1, 0]];
        assert_eq!(edit.stroke_work(&entire_budget).unwrap(), semio_s_artifact_gis_gisterrain_inferences::elevation_edit::MAX_STROKE_WORK - 1);
        let mut polls = 0;
        assert!(edit.apply_stroke_with(&source, &entire_budget, || { polls += 1; polls < 2 }).is_err());
        assert_eq!(polls, 2);
        assert_eq!(source.rows[16][16], 190);
        let overflow = &fixture["overflowStroke"][0];
        let [column, row, height] = serde_json::from_value::<[i32; 3]>(overflow["seed"].clone()).unwrap();
        source.rows[row as usize][column as usize] = height as i16;
        let before = source.clone();
        assert!(apply(&source, &decode(&overflow["command"])).is_err());
        assert_eq!(source, before);
    }

    fn from_fixture_stroke() -> DslValue {
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/📐️edit-elevation/🧪️tests/🔣️.json")).unwrap();
        let command = semio_framework_pack_json::parse(&fixture["strokeEdits"][0]["command"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        semio_framework_pack_json::to_dsl_value(&command)
    }

    #[semio_framework_async_macros::async_test]
    async fn staged_elevation_edit_updates_scene_and_round_trips_history() {
        use crate::editor::gis3d::{create_gis3d_app, Gis3dCommand, Gis3dPlayApp};
        use crate::editor::gis3d::modes::view::windows::terrain::GIS3D_PLAY_BODY_COMPOSITE;
        use crate::editor::gis3d::unit_tests::context;
        use semio_plugin::ArtifactEditor as _;

        let definition = create_gis3d_app();
        let action = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window))
            .find(|action| action.id == "editElevation").expect("elevation action");
        assert!(action.in_palette);
        let mut args = Some(semio_framework::effective_action_args(&action.args, &DslValue::Object(Vec::new()), None));
        let command = Gis3dPlayApp::command_from_action("editElevation", &mut args, &mut None).expect("staged command");
        assert!(matches!(&command, Gis3dCommand::EditElevation(_)));
        let mut app = context::app().await;
        let original = app.snapshot().expect("initial demo document");
        let source = original.elevation_source.as_ref().expect("initial grid");
        let receipt = context::dispatch(&mut app, command).await;
        assert!(receipt.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Artifact));
        let edited = app.snapshot().expect("edited document");
        let updated = edited.elevation_source.as_ref().expect("edited grid");
        assert_eq!(updated.rows[16][16], source.rows[16][16] + 25);
        assert_eq!(updated.rows[0][0], source.rows[0][0]);
        assert_eq!(edited.imported_map, original.imported_map);
        let body = context::render(&mut app, GIS3D_PLAY_BODY_COMPOSITE).await;
        let world = semio_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_plugin::World3dScene>(&body).expect("updated viewport");
        let terrain: Value = serde_json::from_str(world.terrain_json.as_deref().expect("terrain lane")).unwrap();
        assert_eq!(terrain["elevationGrid"]["rows"][16][16], updated.rows[16][16]);
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().expect("undone document"), original);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().expect("redone document"), edited);
        let brush = from_fixture_stroke();
        let stroke = Gis3dPlayApp::command_from_action("editElevation", &mut Some(brush), &mut None).expect("staged stroke");
        let receipt = context::dispatch(&mut app, stroke).await;
        assert_eq!(receipt.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::Artifact).count(), 1);
        let brushed = app.snapshot().expect("brushed terrain");
        for column in 8..=11 {
            assert_eq!(brushed.elevation_source.as_ref().unwrap().rows[8][column], edited.elevation_source.as_ref().unwrap().rows[8][column] + 7);
        }
        context::history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().expect("one undo restores the entire stroke"), edited);
        context::history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().expect("one redo restores the entire stroke"), brushed);
        context::close(&mut app);
    }
}
