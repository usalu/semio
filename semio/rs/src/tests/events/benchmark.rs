use crate::benchmark::BenchmarkFullDto;
use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::port::PortFullDto;
use crate::quality::QualityFullDto;
use crate::typ::TypeFullDto;

#[test]
fn benchmark_set_min_emits() {
    let tg = Guid::new_v7();
    let pg = Guid::new_v7();
    let qg = Guid::new_v7();
    let bg = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        types: vec![TypeFullDto {
            guid: tg.clone(),
            name: "t".into(),
            ports: vec![PortFullDto {
                guid: pg.clone(),
                qualities: vec![QualityFullDto {
                    guid: qg.clone(),
                    key: "qk".into(),
                    benchmarks: vec![BenchmarkFullDto {
                        guid: bg.clone(),
                        name: "bn".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let b = {
        let kr = kit.read().unwrap();
        let t = kr.types[0].clone();
        let tr = t.read().unwrap();
        let p = tr.ports[0].clone();
        let pr = p.read().unwrap();
        let q = pr.qualities[0].clone();
        let qr = q.read().unwrap();
        qr.benchmarks[0].clone()
    };
    b.write().unwrap().set_min(Some(1.0));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "min", .. })));
}
