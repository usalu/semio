//! 🐚 Jack query shell for trinity graphs.

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use trinity_jack::{run, QueryResult};
use trinity_ram::{Graph, PropertyValue};

fn main() {
    if let Err(err) = run_main() {
        eprintln!("trinity_jack_shell: {err}");
        std::process::exit(1);
    }
}

fn run_main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let fixture_path = args.get(1).map(String::as_str).unwrap_or("trinity/fixture/nakagin-capsule-tower.trinity.json");
    let json = fs::read_to_string(fixture_path).map_err(|e| format!("read {fixture_path}: {e}"))?;
    let mut graph = Graph::load_json(&json)?;
    graph.recompute_derived();
    println!("[DEBUG] trinity jack shell loaded {} nodes, {} edges from {fixture_path}", graph.nodes.len(), graph.edges.len());
    if args.len() > 2 {
        let query = args[2..].join(" ");
        print_result(&run(&mut graph, &query)?);
        return Ok(());
    }
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    writeln!(stdout, "jack> trinity graph loaded ({})", graph.name).map_err(|e| e.to_string())?;
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        match run(&mut graph, trimmed) {
            Ok(result) => print_result(&result),
            Err(err) => writeln!(stdout, "error: {err}").map_err(|e| e.to_string())?,
        }
    }
    Ok(())
}

fn print_result(result: &QueryResult) {
    if result.columns.is_empty() {
        println!("ok");
        return;
    }
    println!("{}", result.columns.join("\t"));
    for row in &result.rows {
        let cells: Vec<String> = row.iter().map(format_cell).collect();
        println!("{}", cells.join("\t"));
    }
}

fn format_cell(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => s.clone(),
        PropertyValue::Array(a) => format!("[{}]", a.len()),
        PropertyValue::Object(o) => {
            let pairs: Vec<String> = o.iter().map(|(k, v)| format!("{k}={}", format_cell(v))).collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trinity_ram::{Camera, Edge, GraphFixture, Manifest, Node, Port, PortDirection, PropertyBag};

    fn mini_json() -> String {
        let fixture = GraphFixture {
            schema: GraphFixture::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![Node {
                id: "root".into(),
                kind: "Piece".into(),
                name: "core".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 40.0,
                properties: PropertyBag::new(),
                ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
            }],
            edges: vec![],
        };
        fixture.to_json().unwrap()
    }

    #[test]
    fn shell_loads_fixture() {
        let mut graph = Graph::load_json(&mini_json()).unwrap();
        graph.recompute_derived();
        let result = run(&mut graph, "MATCH (a:Piece) RETURN a.name").unwrap();
        assert_eq!(result.rows.len(), 1);
    }
}
