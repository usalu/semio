//! 🃏 Shared Jack query language for mathematical graph frameworks.

pub mod queryable;

pub use queryable::{
    manifest_edge_kinds, manifest_node_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph,
};

use mathematical_graph_manifest::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

include!("jack_impl.rs");
