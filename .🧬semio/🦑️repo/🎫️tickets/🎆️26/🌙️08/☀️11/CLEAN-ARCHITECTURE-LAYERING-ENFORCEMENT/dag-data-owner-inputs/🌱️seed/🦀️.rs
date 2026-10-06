use super::*;
use semio_framework_dsl_record::{DslField, FieldSpec, FieldValue, ParseOptions, RecordLayout, RecordSpec, Shape};

/// 🌱️ Reads the original bundled seed through its three actual defining fields.
pub(super) fn bundled_host_snapshot() -> DagHostSnapshot {
    let text = include_str!("../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let body = text.strip_prefix("semio dag.dag.dsl v1\n").expect("bundled DAG seed envelope");
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![
        FieldSpec::new(0, "schema", String::shape()),
        FieldSpec::new(1, "nodes", Shape::List(Box::new(DagNodeSpec::shape()))),
        FieldSpec::new(2, "edges", Shape::Table(DagHostSnapshotEdge::__dsl_spec_producer())),
    ]);
    let record = semio_framework_dsl_record::parse(body, &spec, &ParseOptions::default()).expect("bundled DAG demo DSL is valid DagSnapshot text");
    let schema = String::from_value(record.fields.get(&0).expect("seed schema")).expect("seed schema value");
    let nodes = match record.fields.get(&1).expect("seed nodes") {
        FieldValue::List(values) => values.iter().map(|value| DagNodeSpec::from_value(value).expect("seed node value")).collect(),
        _ => panic!("seed nodes must be a list"),
    };
    let edges = match record.fields.get(&2).expect("seed edges") {
        FieldValue::List(values) => values.iter().map(|value| DagHostSnapshotEdge::from_value(value).expect("seed edge value")).collect(),
        _ => panic!("seed edges must be a list"),
    };
    DagHostSnapshot { schema, camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges }
}
