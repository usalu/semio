//! Assign [`crate::events::EventBus`] weak handles to every node after kit construction.

use std::sync::{Arc, Weak};

use crate::benchmark::BenchmarkStoreRef;
use crate::connection::ConnectionStoreRef;
use crate::connector::ConnectorStoreRef;
use crate::design::DesignStoreRef;
use crate::events::EventBus;
use crate::file::FileStoreRef;
use crate::folder::FolderStoreRef;
use crate::group::GroupStoreRef;
use crate::kit::KitStoreRef;
use crate::layer::LayerStoreRef;
use crate::piece::PieceStoreRef;
use crate::port::PortStoreRef;
use crate::quality::QualityStoreRef;
use crate::representation::RepresentationStoreRef;
use crate::stat::StatStoreRef;
use crate::typ::TypeStoreRef;

pub(crate) fn wire_graph_bus(kit: &KitStoreRef) {
    let w = {
        let kr = kit.read().expect("kit read");
        Arc::downgrade(&kr.event_bus)
    };
    let kg = kit.read().expect("kit read");
    for t in &kg.types {
        wire_type(t, &w);
    }
    for d in &kg.designs {
        wire_design(d, &w);
    }
    for f in &kg.files {
        wire_file(f, &w);
    }
    for f in &kg.folders {
        wire_folder(f, &w);
    }
    for a in &kg.authors {
        if let Ok(mut g) = a.write() {
            g.event_bus = w.clone();
        }
    }
    for c in &kg.concepts {
        if let Ok(mut g) = c.write() {
            g.event_bus = w.clone();
        }
    }
    for t in &kg.tags {
        if let Ok(mut g) = t.write() {
            g.event_bus = w.clone();
        }
    }
    for q in &kg.qualities {
        wire_quality(q, &w);
    }
    for p in &kg.props {
        if let Ok(mut g) = p.write() {
            g.event_bus = w.clone();
        }
    }
    for a in &kg.attributes {
        if let Ok(mut g) = a.write() {
            g.event_bus = w.clone();
        }
    }
}

fn wire_type(t: &TypeStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = t.write() {
        g.event_bus = w.clone();
        for p in &g.ports {
            wire_port(p, w);
        }
        for c in &g.connectors {
            wire_connector(c, w);
        }
        for r in &g.representations {
            wire_representation(r, w);
        }
        for a in &g.authors {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
        for c in &g.concepts {
            if let Ok(mut cw) = c.write() {
                cw.event_bus = w.clone();
            }
        }
        for tg in &g.tags {
            if let Ok(mut tw) = tg.write() {
                tw.event_bus = w.clone();
            }
        }
        for q in &g.qualities {
            wire_quality(q, w);
        }
        for p in &g.props {
            if let Ok(mut pw) = p.write() {
                pw.event_bus = w.clone();
            }
        }
        for a in &g.attributes {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
    }
}

fn wire_port(p: &PortStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = p.write() {
        g.event_bus = w.clone();
        for q in &g.qualities {
            wire_quality(q, w);
        }
        for a in g.attributes.iter_mut() {
            a.event_bus = w.clone();
        }
    }
}

fn wire_connector(c: &ConnectorStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = c.write() {
        g.event_bus = w.clone();
        for q in &g.qualities {
            wire_quality(q, w);
        }
        for a in g.attributes.iter_mut() {
            a.event_bus = w.clone();
        }
    }
}

fn wire_representation(r: &RepresentationStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = r.write() {
        g.event_bus = w.clone();
        for tg in g.tags.iter_mut() {
            tg.event_bus = w.clone();
        }
        for q in &g.qualities {
            wire_quality(q, w);
        }
        for a in g.attributes.iter_mut() {
            a.event_bus = w.clone();
        }
    }
}

fn wire_design(d: &DesignStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = d.write() {
        g.event_bus = w.clone();
        for p in &g.pieces {
            wire_piece(p, w);
        }
        for c in &g.connections {
            wire_connection(c, w);
        }
        for l in &g.layers {
            wire_layer(l, w);
        }
        for gr in &g.groups {
            wire_group(gr, w);
        }
        for a in &g.authors {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
        for c in &g.concepts {
            if let Ok(mut cw) = c.write() {
                cw.event_bus = w.clone();
            }
        }
        for t in &g.tags {
            if let Ok(mut tw) = t.write() {
                tw.event_bus = w.clone();
            }
        }
        for q in &g.qualities {
            wire_quality(q, w);
        }
        for p in &g.props {
            if let Ok(mut pw) = p.write() {
                pw.event_bus = w.clone();
            }
        }
        for a in &g.attributes {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
        for s in &g.stats {
            wire_stat(s, w);
        }
    }
}

fn wire_piece(p: &PieceStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = p.write() {
        g.event_bus = w.clone();
        for pr in &g.props {
            if let Ok(mut pw) = pr.write() {
                pw.event_bus = w.clone();
            }
        }
        for a in &g.attributes {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
    }
}

fn wire_connection(c: &ConnectionStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = c.write() {
        g.event_bus = w.clone();
        if let Ok(mut s) = g.connected.write() {
            s.event_bus = w.clone();
        }
        if let Ok(mut s) = g.connecting.write() {
            s.event_bus = w.clone();
        }
        for a in &g.attributes {
            if let Ok(mut aw) = a.write() {
                aw.event_bus = w.clone();
            }
        }
    }
}

fn wire_layer(l: &LayerStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = l.write() {
        g.event_bus = w.clone();
    }
}

fn wire_group(gr: &GroupStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = gr.write() {
        g.event_bus = w.clone();
    }
}

fn wire_file(f: &FileStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = f.write() {
        g.event_bus = w.clone();
    }
}

fn wire_folder(f: &FolderStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = f.write() {
        g.event_bus = w.clone();
    }
}

fn wire_quality(q: &QualityStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = q.write() {
        g.event_bus = w.clone();
        for b in &g.benchmarks {
            wire_benchmark(b, w);
        }
    }
}

fn wire_stat(s: &StatStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = s.write() {
        g.event_bus = w.clone();
    }
}

fn wire_benchmark(b: &BenchmarkStoreRef, w: &Weak<EventBus>) {
    if let Ok(mut g) = b.write() {
        g.event_bus = w.clone();
    }
}
