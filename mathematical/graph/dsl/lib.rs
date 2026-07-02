//! 🃏 Shared Jack query language for mathematical graph frameworks.

pub mod queryable;
pub mod wire;

pub use queryable::{
    manifest_edge_kinds, manifest_node_kinds, manifest_port_kinds, manifest_property_names, BoardQueryableGraph, QueryableEdge, QueryableGraph,
};
pub use wire::{dag_from_wire_literal, wire_literal_from_dag, WireEdge, WireNode};

use mathematical_graph_manifest::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

include!("jack_impl.rs");
