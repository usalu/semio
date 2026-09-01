//! Standalone parity oracle for the two real edits made in this session against the live repo:
//! 1. `semio-framework-value-derive`'s new "plain unit enum -> bare DslValue::String" mode
//!    (mirrors `SelectionMode` in `📡️replication/📡️wire/🦀️.rs`).
//! 2. The mechanical shape of the hand-written `ToValue`/`FromValue` impls added for
//!    `NoConfig`/`NoConfigMutation`, `DomainSelection`, `DomainHover`, `InteractionState`, and
//!    `InteractionConfigMutation` (externally-tagged single-variant enum) in the real files —
//!    reproduced here with `serde`-derived twin types so `serde_json` can serve as the oracle.
//!
//! `semio_framework_os_kernel` here is a verbatim standalone copy of the real
//! `DslValue`/`ToValue`/`FromValue`/`ValueError` surface (see `runtime/src/lib.rs`'s header), not
//! a stub — the derive macro is copied byte-for-byte from the real
//! `🌱️value/✨️derive/🦀️component.rs` (see `derive/src/component.rs`).

use semio_framework_os_kernel::{FromValue, ToValue};
use semio_framework_value_derive::{FromValue as DeriveFromValue, ToValue as DeriveToValue};

//#region 🔖️PlainUnitEnum (mirrors SelectionMode)
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, DeriveToValue, DeriveFromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum SelectionModeTwin {
    Single,
    Multiple,
}
//#endregion 🔖️PlainUnitEnum

//#region 🔖️EmptyStruct (mirrors NoConfig)
#[derive(Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize, DeriveToValue, DeriveFromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NoConfigTwin {}
//#endregion 🔖️EmptyStruct

//#region 🔖️EmptyEnum (mirrors NoConfigMutation)
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, DeriveToValue, DeriveFromValue)]
pub enum NoConfigMutationTwin {}
//#endregion 🔖️EmptyEnum

//#region 🔖️OptionDefaultSkip (mirrors DomainSelection)
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainSelectionTwin {
    pub granularity: String,
    pub ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_id: Option<String>,
}

impl ToValue for DomainSelectionTwin {
    fn to_value(&self) -> semio_framework_os_kernel::DslValue {
        let mut entries: Vec<(String, semio_framework_os_kernel::DslValue)> = vec![
            ("granularity".to_string(), ToValue::to_value(&self.granularity)),
            ("ids".to_string(), ToValue::to_value(&self.ids)),
        ];
        if self.anchor_id.is_some() {
            entries.push(("anchorId".to_string(), ToValue::to_value(&self.anchor_id)));
        }
        semio_framework_os_kernel::DslValue::Object(entries)
    }
}
impl FromValue for DomainSelectionTwin {
    fn from_value(value: semio_framework_os_kernel::DslValue) -> Result<Self, semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let granularity = match entries.iter().find(|(k, _)| k == "granularity") {
            Some((_, v)) => FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("granularity"))?,
            None => return Err(semio_framework_os_kernel::ValueError::new("missing field `granularity`")),
        };
        let ids = match entries.iter().find(|(k, _)| k == "ids") {
            Some((_, v)) => FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("ids"))?,
            None => return Err(semio_framework_os_kernel::ValueError::new("missing field `ids`")),
        };
        let anchor_id = match entries.iter().find(|(k, _)| k == "anchorId") {
            Some((_, v)) => FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("anchorId"))?,
            None => ::std::default::Default::default(),
        };
        Ok(Self { granularity, ids, anchor_id })
    }
}
//#endregion 🔖️OptionDefaultSkip

//#region 🔖️Composite (mirrors InteractionState's BTreeMap<String, T> fields)
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateTwin {
    pub selection: std::collections::BTreeMap<String, DomainSelectionTwin>,
    pub active_mode: std::collections::BTreeMap<String, SelectionModeTwin>,
}

impl ToValue for StateTwin {
    fn to_value(&self) -> semio_framework_os_kernel::DslValue {
        semio_framework_os_kernel::DslValue::object([
            ("selection".to_string(), ToValue::to_value(&self.selection)),
            ("activeMode".to_string(), ToValue::to_value(&self.active_mode)),
        ])
    }
}
impl FromValue for StateTwin {
    fn from_value(value: semio_framework_os_kernel::DslValue) -> Result<Self, semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let selection = match entries.iter().find(|(k, _)| k == "selection") {
            Some((_, v)) => FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("selection"))?,
            None => return Err(semio_framework_os_kernel::ValueError::new("missing field `selection`")),
        };
        let active_mode = match entries.iter().find(|(k, _)| k == "activeMode") {
            Some((_, v)) => FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("activeMode"))?,
            None => return Err(semio_framework_os_kernel::ValueError::new("missing field `activeMode`")),
        };
        Ok(Self { selection, active_mode })
    }
}
//#endregion 🔖️Composite

//#region 🔖️ExternallyTaggedSingleVariant (mirrors InteractionConfigMutation)
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConfigMutationTwin {
    SetState(DomainSelectionTwin),
}

impl ToValue for ConfigMutationTwin {
    fn to_value(&self) -> semio_framework_os_kernel::DslValue {
        let ConfigMutationTwin::SetState(state) = self;
        semio_framework_os_kernel::DslValue::object([("setState".to_string(), ToValue::to_value(state))])
    }
}
impl FromValue for ConfigMutationTwin {
    fn from_value(value: semio_framework_os_kernel::DslValue) -> Result<Self, semio_framework_os_kernel::ValueError> {
        let entries = semio_framework_os_kernel::DslValue::into_object(value)?;
        match entries.iter().find(|(k, _)| k == "setState") {
            Some((_, v)) => Ok(ConfigMutationTwin::SetState(FromValue::from_value(v.clone()).map_err(|error: semio_framework_os_kernel::ValueError| error.under("setState"))?)),
            None => Err(semio_framework_os_kernel::ValueError::new("missing `setState` variant payload")),
        }
    }
}
//#endregion 🔖️ExternallyTaggedSingleVariant
