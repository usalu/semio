use semio_s_plugin_stdio_test_oracle::markup::live::{parse_markup, write_markup, MarkupNode};

fn walk(node: &MarkupNode, path: Vec<usize>, depth: usize) {
    if depth > 4 { return }
    if let MarkupNode::Element { children, .. } = node {
        for (i, c) in children.iter().enumerate() {
            let mut p = path.clone(); p.push(i);
            let label = match c {
                MarkupNode::Element { name, attrs, .. } => format!("<{name}> {attrs:?}"),
                MarkupNode::Text(t) => format!("TEXT {t:?}"),
                MarkupNode::CData(t) => format!("CDATA {t:?}"),
                MarkupNode::Comment(t) => format!("COMMENT {t:?}"),
                MarkupNode::Pi { target, data } => format!("PI {target} {data}"),
            };
            println!("{:?} {}", p, label);
            walk(c, p, depth + 1);
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let input = std::fs::read(&path).unwrap();
    let doc = parse_markup(&input).unwrap();
    println!("decl={:?}", doc.declaration);
    println!("doctype={:?}", doc.doctype);
    println!("prolog={:?}", doc.prolog.len());
    if let Some(root) = &doc.root {
        if let MarkupNode::Element { name, attrs, .. } = root { println!("root <{name}> {attrs:?}"); }
        walk(root, vec![], 0);
    }
    let out = write_markup(&doc).unwrap();
    println!("in={} out={} identical={}", input.len(), out.len(), out == input);
}
