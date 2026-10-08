//! 🔮️ Test-only independent Serde projection for typed data-field changes.
use crate::standards::v1::subsets::any::schema::mutations::change_data_fields::ChangeDataFields;

#[cfg(test)]
impl serde::Serialize for ChangeDataFields {fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(semio_framework_value::ToValue::to_value(self)),serializer)}}
#[cfg(test)]
impl<'de>serde::Deserialize<'de>for ChangeDataFields{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;<Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(serde::de::Error::custom)}}
