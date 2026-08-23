use ruststep::ast::{Exchange, Parameter, EntityInstance, Record};
use std::str::FromStr;

const SENTINEL: char = '\u{E000}';

fn escape_doubled_apostrophes(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut in_string = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c != '\'' { out.push(c); i += 1; continue; }
        if !in_string { in_string = true; out.push(c); i += 1; }
        else if chars.get(i+1) == Some(&'\'') { out.push(SENTINEL); i += 2; }
        else { in_string = false; out.push(c); i += 1; }
    }
    out
}

fn unescape_param(p: &mut Parameter) {
    match p {
        Parameter::String(s) => { if s.contains(SENTINEL) { *s = s.replace(SENTINEL, "'"); } }
        Parameter::List(items) => items.iter_mut().for_each(unescape_param),
        Parameter::Typed { parameter, .. } => unescape_param(parameter),
        _ => {}
    }
}
fn unescape_record(r: &mut Record) { unescape_param(&mut r.parameter); }

fn main() {
    let text = std::fs::read_to_string("fixture.ifc").unwrap();
    let pre = escape_doubled_apostrophes(&text);
    let mut ex = Exchange::from_str(&pre).expect("parse ok");
    ex.header.iter_mut().for_each(unescape_record);
    for section in &mut ex.data {
        for entity in &mut section.entities {
            match entity {
                EntityInstance::Simple { record, .. } => unescape_record(record),
                EntityInstance::Complex { subsuper, .. } => subsuper.0.iter_mut().for_each(unescape_record),
            }
        }
    }
    let total: usize = ex.data.iter().map(|s| s.entities.len()).sum();
    eprintln!("entities = {total}");
    // find entity #17012 and print its args
    for section in &ex.data {
        for entity in &section.entities {
            if let EntityInstance::Simple { id, record } = entity {
                if *id == 17012 {
                    eprintln!("entity 17012 = {:?}", record);
                }
            }
        }
    }
    // check FILE_NAME header record
    for r in &ex.header {
        if r.name == "FILE_NAME" { eprintln!("FILE_NAME = {:?}", r.parameter); }
    }
}
