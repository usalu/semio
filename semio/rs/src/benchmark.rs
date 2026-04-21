use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// Numeric range benchmark used to qualify quality measurements.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Benchmark {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_excluded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_excluded: Option<bool>,
}

impl Benchmark {
    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("benchmark")
            .str(&self.name)
            .opt_f64(self.min)
            .opt_f64(self.max)
            .opt_bool(self.min_excluded)
            .opt_bool(self.max_excluded);
    }
}
