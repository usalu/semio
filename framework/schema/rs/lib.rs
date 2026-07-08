//! 📋 Schema registry: derive JSON Schema from Rust types and validate at kernel boundaries.

use jsonschema::Validator;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

//#region 🔖Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SchemaError {
    #[error("unknown schema id: {0}")]
    UnknownSchema(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("serialize error: {0}")]
    Serialize(String),
}
//#endregion 🔖Errors

//#region 🔖SchemaRegistry
pub struct SchemaRegistry {
    schemas: HashMap<String, Value>,
    validators: HashMap<String, Validator>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            validators: HashMap::new(),
        }
    }

    pub fn register<T: JsonSchema>(&mut self, id: &str) -> Result<(), SchemaError> {
        let schema = schema_for!(T);
        let value = serde_json::to_value(schema).map_err(|error| SchemaError::Serialize(error.to_string()))?;
        let validator = Validator::new(&value).map_err(|error| SchemaError::Validation(error.to_string()))?;
        self.schemas.insert(id.to_string(), value);
        self.validators.insert(id.to_string(), validator);
        Ok(())
    }

    pub fn register_json(&mut self, id: &str, schema: Value) -> Result<(), SchemaError> {
        let validator = Validator::new(&schema).map_err(|error| SchemaError::Validation(error.to_string()))?;
        self.schemas.insert(id.to_string(), schema);
        self.validators.insert(id.to_string(), validator);
        Ok(())
    }

    pub fn schema(&self, id: &str) -> Option<&Value> {
        self.schemas.get(id)
    }

    pub fn validate(&self, id: &str, value: &Value) -> Result<(), SchemaError> {
        let validator = self
            .validators
            .get(id)
            .ok_or_else(|| SchemaError::UnknownSchema(id.to_string()))?;
        validator
            .validate(value)
            .map_err(|error| SchemaError::Validation(error.to_string()))
    }
}
//#endregion 🔖SchemaRegistry
