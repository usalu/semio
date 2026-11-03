//! ⚡️ Incremental norm evaluation: check units with declared read sets, entity graph, planner, retained session and read audit. See the visual editor architecture §5.3.

use crate::{CheckReport, CheckResult, Owners};
use semio_framework_value::{DslValue, FromValue, Number, ToValue};
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

#[path = "🔍️read-tracker/🦀️.rs"]
mod read_tracker;
pub use read_tracker::*;

#[path = "⚙️units/🦀️.rs"]
mod units;
pub use units::*;

#[path = "🕸️entity-graph/🦀️.rs"]
mod entity_graph;
pub use entity_graph::*;

#[path = "📋️planner/🦀️.rs"]
mod planner;
pub use planner::*;

#[path = "🧱️rows/🦀️.rs"]
mod rows;
pub use rows::*;

#[path = "🌳️value-tree/🦀️.rs"]
mod value_tree;
pub use value_tree::*;

#[path = "🧵️session/🦀️.rs"]
mod session;
pub use session::*;

#[path = "🛰️live/🦀️.rs"]
mod live;

#[path = "🧪️laws/🦀️.rs"]
mod laws;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/⏱️latency/🦀️.rs"]
mod latency;
