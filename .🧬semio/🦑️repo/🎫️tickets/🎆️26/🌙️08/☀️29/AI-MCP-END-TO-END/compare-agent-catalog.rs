use semio_os_mcp_catalog::catalog::{compile_with_audiences, CatalogSource, AGENT_AUDIENCES, ALL_AUDIENCES};
use semio_ui_locale::{Locale, Terminology};
use std::{collections::BTreeMap, env, fs};

fn main() {
    let sources: Vec<CatalogSource> = env::args().skip(1).map(|file| CatalogSource {
        descriptors: vec![serde_json::from_slice(&fs::read(file).unwrap()).unwrap()],
        ..Default::default()
    }).collect();
    assert_eq!(sources.len(), 2);
    for locale in [Locale::En, Locale::De] {
        for (label, audience) in [("agent", AGENT_AUDIENCES), ("all", ALL_AUDIENCES)] {
            let projections: Vec<_> = sources.iter().map(|source| compile_with_audiences(source, locale, Terminology::Native, audience).unwrap()).collect();
            let [old, new] = &projections[..] else { unreachable!() };
            if label == "agent" { assert_eq!(old, new); }
            let entries = |catalog: &semio_os_mcp_catalog::catalog::Catalog| catalog.entries.iter().map(|entry| (entry.id.0.clone(), serde_json::to_value(entry).unwrap())).collect::<BTreeMap<_,_>>();
            let a = entries(old);
            let b = entries(new);
            let differing: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).filter(|key| a.get(*key) != b.get(*key)).collect();
            println!("{locale:?} {label}: {} -> {} entries, {} -> {}, changed {:?}", a.len(), b.len(), old.hash, new.hash, differing);
        }
    }
}
