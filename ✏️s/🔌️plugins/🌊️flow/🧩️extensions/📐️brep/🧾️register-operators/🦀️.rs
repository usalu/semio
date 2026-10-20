//! 🧾️ Register operators: the registration of the brep schemas and operators, the extension manifest json and the module registry. See [`super`].

use super::*;

#[path = "🧊️schemas-primitives-and-curves/🦀️.rs"]
mod schemas_primitives_and_curves;
use schemas_primitives_and_curves::{register_schemas_and_deconstruct, register_primitive_solids, register_curves};

#[path = "🪡️surfaces-and-sweeps/🦀️.rs"]
mod surfaces_and_sweeps;
use surfaces_and_sweeps::{register_surfaces, register_sweeps};

#[path = "🧮️booleans-transforms-and-solid-features/🦀️.rs"]
mod booleans_transforms_and_solid_features;
use booleans_transforms_and_solid_features::{register_booleans, register_transforms_and_patterns, register_solid_features};

#[path = "🔍️intersection-and-evaluation-operators/🦀️.rs"]
mod intersection_and_evaluation_operators;
use intersection_and_evaluation_operators::{register_intersections, register_evaluations};

#[path = "📐️measurement-utility-and-topology-operators/🦀️.rs"]
mod measurement_utility_and_topology_operators;
use measurement_utility_and_topology_operators::{register_measurements, register_utilities, register_topology};

#[path = "📂️io-operators/🦀️.rs"]
mod io_operators;
use io_operators::register_io;

/// 📦️ Registers brep geometry schema and operators.
pub fn register(registry: &mut Registry, session: &Session) {
    register_schemas_and_deconstruct(registry, session);
    register_primitive_solids(registry, session);
    register_curves(registry, session);
    register_surfaces(registry, session);
    register_sweeps(registry, session);
    register_booleans(registry, session);
    register_transforms_and_patterns(registry, session);
    register_solid_features(registry, session);
    register_intersections(registry, session);
    register_evaluations(registry, session);
    register_measurements(registry, session);
    register_utilities(registry, session);
    register_topology(registry, session);
    register_io(registry, session);
    mesh::register_mesh(registry, session);
    registry.finalize();
}

/// 🛂️ Manifest JSON for host contribution install (tests + packaging metadata).
pub async fn extension_manifest_json() -> String {
    let session = semio_s_artifact_stdio_step::geometry::session::geometry_session();
    let registry = neural_engine::ColdOwner::new(module_registry(&session));
    let manifest = build_manifest_json("brep", "Brep", env!("CARGO_PKG_VERSION"), &registry, vec!["onStartup".into()], vec![], vec![], vec![]);
    drop(registry);
    session.close();
    manifest
}

pub fn module_registry(session: &Session) -> Registry {
    let mut registry = Registry::new();
    register(&mut registry, session);
    registry
}
