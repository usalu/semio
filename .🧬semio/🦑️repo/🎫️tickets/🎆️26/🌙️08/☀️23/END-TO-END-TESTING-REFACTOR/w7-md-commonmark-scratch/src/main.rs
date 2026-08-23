use comrak::nodes::NodeValue;
use comrak::{parse_document, Arena, Options};

fn short(s: &str) -> String {
    let s: String = s.chars().take(60).collect();
    s.replace('\n', "\\n")
}

fn describe<'a>(node: &'a comrak::nodes::AstNode<'a>, depth: usize) -> String {
    let data = node.data.borrow();
    match &data.value {
        NodeValue::Heading(h) => format!("Heading(level={})", h.level),
        NodeValue::Paragraph => "Paragraph".to_string(),
        NodeValue::List(l) => format!("List(ordered={}, tight={}, start={}, items={})", matches!(l.list_type, comrak::nodes::ListType::Ordered), l.tight, l.start, node.children().count()),
        NodeValue::Item(_) => "Item".to_string(),
        NodeValue::CodeBlock(c) => format!("CodeBlock(info={:?}, len={})", c.info, c.literal.len()),
        NodeValue::BlockQuote => "BlockQuote".to_string(),
        NodeValue::ThematicBreak => "ThematicBreak".to_string(),
        NodeValue::HtmlBlock(h) => format!("HtmlBlock(len={})", h.literal.len()),
        NodeValue::Table(_) => "Table".to_string(),
        other => format!("{:?}", other),
    }
    .to_string()
        + &format!(" depth={depth}")
}

fn walk<'a>(node: &'a comrak::nodes::AstNode<'a>, depth: usize, max_depth: usize) {
    let mut index = 0;
    for child in node.children() {
        let desc = describe(child, depth);
        println!("{}[{}] {}", "  ".repeat(depth), index, desc);
        if depth < max_depth {
            walk(child, depth + 1, max_depth);
        }
        index += 1;
    }
}

fn main() {
    let text = std::fs::read_to_string("README.md").expect("read readme");
    let arena = Arena::new();
    let options = Options::default();
    let root = parse_document(&arena, &text, &options);
    walk(root, 0, 3);

    // also print total top-level block count and first text preview per top-level block
    let mut idx = 0;
    for child in root.children() {
        let data = child.data.borrow();
        let preview = match &data.value {
            NodeValue::Heading(_) | NodeValue::Paragraph => {
                let mut buf = String::new();
                for inline in child.descendants() {
                    if let NodeValue::Text(t) = &inline.data.borrow().value {
                        buf.push_str(t);
                    }
                }
                short(&buf)
            }
            _ => String::new(),
        };
        println!("TOP[{}] {:?} :: {}", idx, std::mem::discriminant(&data.value), preview);
        idx += 1;
    }
}
