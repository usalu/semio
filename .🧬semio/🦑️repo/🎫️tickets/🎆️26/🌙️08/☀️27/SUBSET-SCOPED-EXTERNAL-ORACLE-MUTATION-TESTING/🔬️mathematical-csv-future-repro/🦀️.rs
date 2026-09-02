struct Graph {
    nodes: Vec<String>,
}

async fn mathematical_graph() -> Graph {
    Graph { nodes: Vec::new() }
}

fn serialize() {
    let graph = mathematical_graph();
    let _ = graph.nodes;
}

fn main() {}
