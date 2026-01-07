// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

// #region Error Types

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SemioError {
    #[error("Entity not found: {kind} with guid {guid}")]
    NotFound { kind: String, guid: String },
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
}

pub type Result<T> = std::result::Result<T, SemioError>;

// #endregion Error Types

// #region Utility Functions

pub fn guid() -> String {
    Uuid::now_v7().to_string()
}

pub fn normalize(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

pub fn jaccard<T: Eq + std::hash::Hash>(a: &HashSet<T>, b: &HashSet<T>) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

pub fn deep_equal<T: Serialize>(a: &T, b: &T) -> bool {
    match (serde_json::to_value(a), serde_json::to_value(b)) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

pub fn generate_unique_name(base: &str, existing: &[String]) -> String {
    let existing_set: HashSet<_> = existing.iter().collect();
    if !existing_set.contains(&base.to_string()) {
        return base.to_string();
    }
    let mut counter = 1;
    loop {
        let candidate = format!("{} ({})", base, counter);
        if !existing_set.contains(&candidate) {
            return candidate;
        }
        counter += 1;
    }
}

// #endregion Utility Functions

pub type Guid = String;

mod model;
mod diff;
mod validate;
mod io;

pub use model::*;
pub use diff::*;
pub use validate::*;
pub use io::*;

#[cfg(test)]
mod tests;
