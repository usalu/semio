//! 🎨️ Load a declared example into the open compliance document.

use super::set_snapshot;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
fn example_primary_text(id: &str) -> Option<&'static str> {
    if id == crate::examples::demo::ID {
        return Some(crate::examples::demo::PRIMARY_TEXT);
    }
    if id == crate::examples::nonconforming::ID {
        return Some(crate::examples::nonconforming::PRIMARY_TEXT);
    }
    macro_rules! blatt {
        ($($mod:ident),+ $(,)?) => {
            $(
                if id == crate::examples::$mod::ID {
                    return Some(crate::examples::$mod::PRIMARY_TEXT);
                }
            )+
        };
    }
    blatt!(
        blatt_3, blatt_3_fail, blatt_4, blatt_4_fail, blatt_5, blatt_5_fail, blatt_6, blatt_6_fail, blatt_7, blatt_7_fail, blatt_8, blatt_8_fail, blatt_16, blatt_16_fail, blatt_19,
        blatt_19_fail, blatt_53, blatt_53_fail, blatt_60, blatt_60_fail,
    );
    None
}

/// 🎨️ Replaces the live document with the named example's `PRIMARY_TEXT`, or clears it when the id is empty.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Vdi3805Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Vdi3805Mutation, NoConfigMutation>, Fault> {
    let snapshot = match payload.example_id.trim() {
        "" => Vdi3805Snapshot::default(),
        id => {
            let Some(text) = example_primary_text(id) else {
                return Ok(Emit::default());
            };
            <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| Fault::from(format!("set-active-example: invalid example text: {error:?}")))?
        }
    };
    set_snapshot::handle(&set_snapshot::ReplaceSnapshot { snapshot }, doc, cfg)
}
//#endregion 🔖️Handler
