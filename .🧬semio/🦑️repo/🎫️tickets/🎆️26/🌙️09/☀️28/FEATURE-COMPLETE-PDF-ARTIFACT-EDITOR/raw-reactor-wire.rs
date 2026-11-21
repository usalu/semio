//! 🎟️ Closed wire vocabulary for the original-media command prologue.

/// 📥️ Out-of-band fixed-page family reserved before generic command assembly.
pub const ORIGINAL_MEDIA_COMMAND_KIND: u8 = 240;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct OriginalMediaWireLoan {
    pub operation_id: String,
    pub generation: String,
    pub instance_id: u32,
    pub document_id: String,
    pub base_revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum OriginalMediaWireControl {
    Begin { name: String, media_type: String },
    Complete,
    Cancel,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct OriginalMediaControlEvent {
    pub loan: OriginalMediaWireLoan,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
    pub control: OriginalMediaWireControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum OriginalMediaWirePhase { Begin, Page, Complete, Cancel }

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum OriginalMediaWireResult { Retry, Pending, Accepted, Stale, Refused }

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::semio_framework_value", rename_all = "camelCase", deny_unknown_fields)]
pub struct OriginalMediaWireAck {
    pub loan: OriginalMediaWireLoan,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub extent: u32,
    pub phase: OriginalMediaWirePhase,
    pub page_index: Option<u32>,
    pub result: OriginalMediaWireResult,
}
