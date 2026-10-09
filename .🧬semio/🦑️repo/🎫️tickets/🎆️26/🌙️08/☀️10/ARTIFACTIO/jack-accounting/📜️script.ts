import { readFileSync, writeFileSync } from "node:fs";
const base = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor";
const file = `${base}/🪜️execution/🦀️.rs`;
let source = readFileSync(file, "utf8");
const helpers = `fn reserve_result<T>(values: &mut Vec<T>, owned: &mut usize, maximum: usize) -> Result<(), ValueError> {
    if values.len() < values.capacity() { return Ok(()); }
    let inline = size_of::<T>();
    add_result_owned(owned, inline, maximum)?;
    let previous = values.capacity();
    values.try_reserve_exact(1).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "query result allocation refused"))?;
    let extra = values.capacity().checked_sub(previous).and_then(|count| count.checked_sub(1)).and_then(|count| count.checked_mul(inline)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result capacity overflow"))?;
    add_result_owned(owned, extra, maximum)
}

fn add_result_owned(owned: &mut usize, additional: usize, maximum: usize) -> Result<(), ValueError> {
    *owned = owned.checked_add(additional).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result byte count overflow"))?;
    if *owned > maximum { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "query result exceeds its retained ownership grant")); }
    Ok(())
}

fn graph_result_metadata_owned_bytes(graph: &Graph, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<JackSnapshot>() + size_of::<SemioGraphSnapshot>();
    for text in [JackSnapshot::SCHEMA, graph.name.as_str(), " subgraph", graph.manifest_id.as_deref().unwrap_or(""), graph.root_node_id.as_deref().unwrap_or("")] { add_result_owned(&mut bytes, text.len(), maximum)?; }
    let mut properties = |values: &[crate::PropertyDef]| -> Result<(), ValueError> {
        for value in values {
            add_result_owned(&mut bytes, size_of::<crate::PropertyDef>() + value.name.len() + value.expr.as_ref().map_or(0, String::len), maximum)?;
            let mut value_type = &value.value_type;
            let mut depth = 0;
            loop {
                match value_type {
                    semio_framework_value::ValueType::List(inner) => {
                        depth += 1;
                        if depth > QUERY_ENTITY_NESTING_MAXIMUM { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "query result metadata exceeds its nesting admission")); }
                        add_result_owned(&mut bytes, size_of::<semio_framework_value::ValueType>(), maximum)?;
                        value_type = inner;
                    }
                    semio_framework_value::ValueType::Schema(text) => { add_result_owned(&mut bytes, text.len(), maximum)?; break; }
                    _ => break,
                }
            }
        }
        Ok(())
    };
    for kind in &graph.manifest.node_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.edge_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.port_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.node_kinds {
        add_result_owned(&mut bytes, size_of::<crate::NodeKindDef>() + kind.name.len(), maximum)?;
        for port in &kind.port_kinds { add_result_owned(&mut bytes, size_of::<String>() + port.len(), maximum)?; }
    }
    for kind in &graph.manifest.edge_kinds { add_result_owned(&mut bytes, size_of::<crate::EdgeKindDef>() + kind.name.len(), maximum)?; }
    for kind in &graph.manifest.port_kinds { add_result_owned(&mut bytes, size_of::<crate::PortKindDef>() + kind.name.len(), maximum)?; }
    Ok(bytes)
}

`;
source = source.slice(0, source.indexOf("fn json_string_bytes")) + helpers + source.slice(source.indexOf("/// 🧱 One preparation turn"));
source = source.replaceAll("metadata_output_upper_bound", "maximum_result_owned_bytes").replaceAll("output_bytes", "result_owned_bytes").replaceAll("add_output", "add_owned");
source = source.replace("maximum_result_owned_bytes: 512", "maximum_result_owned_bytes: QUERY_RESULT_MAXIMUM_OWNED_BYTES");
source = source.replace("    pub fn new(query: Query) -> Self {\n        Self", "    pub fn new(query: Query) -> Self { Self::with_result_grant(query, QUERY_RESULT_MAXIMUM_OWNED_BYTES) }\n\n    pub fn with_result_grant(query: Query, maximum_result_owned_bytes: usize) -> Self {\n        Self");
source = source.replace("maximum_result_owned_bytes: QUERY_RESULT_MAXIMUM_OWNED_BYTES, closing", "maximum_result_owned_bytes, closing");
source = source.replace(/JackSnapshotCloneStep::Pending \{ copied_bytes \} => \{\s*self.maximum_result_owned_bytes = [^;]+;/u, "JackSnapshotCloneStep::Pending { .. } => {");
source = source.replaceAll("with_maximum_result_owned_bytes", "with_result_grant");
source = source.replace("result_owned_bytes: 256", "result_owned_bytes: size_of::<QueryResult>()");
const start = source.indexOf("    fn add_owned(&mut self, bytes: usize)");
const end = source.indexOf("    fn advance_pair", start);
source = source.slice(0, start) + `    fn add_owned(&mut self, bytes: usize) -> Result<(), ValueError> {
        add_result_owned(&mut self.result_owned_bytes, bytes, self.maximum_result_owned_bytes)
    }
` + source.slice(end);
source = source.replace("self.add_owned(json_string_bytes(&column)?.saturating_add(1))?;", "self.add_owned(column.capacity())?;\n                    reserve_result(&mut self.columns, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;");
source = source.replace("self.add_owned(self.maximum_result_owned_bytes)?;", "self.add_owned(graph_result_metadata_owned_bytes(graph, self.maximum_result_owned_bytes)?)?;");
source = source.replace("                        self.add_owned(2)?;\n", "");
source = source.replace("self.add_owned(bytes.saturating_mul(6).saturating_add(512))?;", "self.add_owned(bytes - size_of::<Node>())?;\n                        reserve_result(&mut self.nodes, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;");
source = source.replace("self.add_owned(bytes.saturating_mul(6).saturating_add(384))?;", "self.add_owned(bytes - size_of::<Edge>())?;\n                        reserve_result(&mut self.edges, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;");
source = source.replace("                        self.add_owned(3)?;", "                        reserve_result(&mut self.rows, &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;");
source = source.replace("let mut output_items = 0;\n                        self.add_owned(property_json_upper_bound(&value, 0, &mut output_items)?.saturating_add(1))?;", "let mut retained_items = 0;\n                        let mut retained_bytes = 0;\n                        property_owned_bytes(&value, 0, &mut retained_items, &mut retained_bytes, self.maximum_result_owned_bytes)?;\n                        self.add_owned(retained_bytes - size_of::<PropertyValue>())?;\n                        reserve_result(&mut self.rows[self.binding], &mut self.result_owned_bytes, self.maximum_result_owned_bytes)?;");
source = source.replace("Self::with_result_grant(graph, query, 512)", "Self::with_result_grant(graph, query, QUERY_RESULT_MAXIMUM_OWNED_BYTES)");
source = source.replace("    fn with_result_grant(graph:", "    pub fn with_result_grant(graph:");
writeFileSync(file, source);
const tests = `${base}/🧪️tests/🔬️unit/🦀️.rs`;
writeFileSync(tests, readFileSync(tests,"utf8").replace("use crate::language_service::{complete,", "use crate::language_service::{parse, complete,"));
const root = `${base}/🦀️.rs`;
writeFileSync(root, readFileSync(root,"utf8").replace("QueryPreparationStep};", "QueryPreparationStep, QUERY_RESULT_MAXIMUM_OWNED_BYTES};"));
console.log("[DEBUG] Jack retained result ownership admission replaced physical JSON estimates");
