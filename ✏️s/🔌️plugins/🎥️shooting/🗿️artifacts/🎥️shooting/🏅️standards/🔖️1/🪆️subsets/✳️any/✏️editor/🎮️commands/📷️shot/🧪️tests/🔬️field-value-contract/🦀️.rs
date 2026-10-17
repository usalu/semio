use semio_framework_pack_json::json;
use semio_framework_pack_json::Value;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::{editor::shooting::config::ShootingConfig, editor::shooting::config::ShootingConfigMutation};
#[allow(unused_imports)]
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
#[allow(unused_imports)]
use crate::mutations::change_shot_format::ChangeShotFormat;
#[allow(unused_imports)]
use crate::mutations::change_shot_height::ChangeShotHeight;
#[allow(unused_imports)]
use crate::mutations::change_shot_shape::ChangeShotShape;
#[allow(unused_imports)]
use crate::mutations::change_shot_width::ChangeShotWidth;
#[allow(unused_imports)]
use crate::mutations::create_shot::CreateShot;
#[allow(unused_imports)]
use crate::mutations::rename_shot::RenameShot;
#[allow(unused_imports)]
use crate::mutations::set_active_shot::SetActiveShot as SetActiveShotMutation;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
#[allow(unused_imports)]
use crate::ShootingShot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::shooting::commands::shot::{shot_mutation_for_field};
use crate::editor::shooting::commands::shot::*;

#[test]
fn shooting_shot_field_values_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../🎮️commands/📷️shot/🧪️tests/🧫️fixtures/🔢️field-values.json")).expect("neutral input vectors");
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let id = base.shots[0].id.clone();
    for vector in vectors["cases"].as_array().expect("cases") {
        let field = vector["field"].as_str().expect("field");
        let value = Value::String(vector["value"].as_str().expect("input string").into());
        let mutation = shot_mutation_for_field(id.clone(), field, &value);
        if vector["expected"].is_null() {
            assert!(mutation.is_none(), "invalid dimension must reject without truncation");
        } else {
            let (next, _) = store::apply_mutation(&base, &mutation.expect("valid input")).expect("apply shot field");
            let json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&next)).expect("independent snapshot oracle");
            assert_eq!(json["shots"][0][field], vector["expected"]);
        }
    }
}
