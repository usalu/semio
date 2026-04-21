//! Helpers for event-sequence tests.

use crate::connection::ConnectionFullDto;
use crate::connector::ConnectorFullDto;
use crate::design::DesignFullDto;
use crate::events::{EntityKind, EntityRef, KitEvent};
use crate::file::FileFullDto;
use crate::group::GroupFullDto;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore, KitStoreRef};
use crate::layer::LayerFullDto;
use crate::piece::{PieceFullDto, PieceIdDto};
use crate::side::SideMetadataDto;
use crate::port::{PortFullDto, PortIdDto};
use crate::typ::{TypeFullDto, TypeIdDto};

/// Drain all currently queued events from a broadcast receiver (non-blocking).
pub fn drain(rx: &mut async_broadcast::Receiver<KitEvent>) -> Vec<KitEvent> {
    let mut out = Vec::new();
    while let Ok(e) = rx.try_recv() {
        out.push(e);
    }
    out
}

pub fn kit_entity_ref(kit: &KitStoreRef) -> EntityRef {
    let g = kit.read().expect("kit read").guid.clone();
    EntityRef::new(EntityKind::Kit, g)
}

/// Minimal kit with one type, one design, one piece (valid type ref).
pub fn kit_with_piece() -> (KitStoreRef, Guid, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let design_guid = Guid::new_v7();
    let piece_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();

    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        designs: vec![DesignFullDto {
            guid: design_guid.clone(),
            name: "des".into(),
            pieces: vec![PieceFullDto {
                guid: piece_guid.clone(),
                r#type: Some(TypeIdDto {
                    guid: type_guid.clone(),
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, type_guid, design_guid, piece_guid)
}

/// One design with a layer and one piece (piece required for valid design content).
pub fn kit_with_layer() -> (KitStoreRef, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let design_guid = Guid::new_v7();
    let piece_guid = Guid::new_v7();
    let layer_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        designs: vec![DesignFullDto {
            guid: design_guid.clone(),
            name: "des".into(),
            pieces: vec![PieceFullDto {
                guid: piece_guid.clone(),
                r#type: Some(TypeIdDto {
                    guid: type_guid.clone(),
                }),
                ..Default::default()
            }],
            layers: vec![LayerFullDto {
                guid: layer_guid.clone(),
                name: "L".into(),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, design_guid, layer_guid)
}

/// Design with one group referencing the single piece.
pub fn kit_with_group() -> (KitStoreRef, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let design_guid = Guid::new_v7();
    let piece_guid = Guid::new_v7();
    let group_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        designs: vec![DesignFullDto {
            guid: design_guid.clone(),
            name: "des".into(),
            pieces: vec![PieceFullDto {
                guid: piece_guid.clone(),
                r#type: Some(TypeIdDto {
                    guid: type_guid.clone(),
                }),
                ..Default::default()
            }],
            groups: vec![GroupFullDto {
                guid: group_guid.clone(),
                name: "G".into(),
                pieces: vec![PieceIdDto {
                    guid: piece_guid.clone(),
                }],
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, design_guid, group_guid)
}

pub fn kit_with_type_connector() -> (KitStoreRef, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let port_guid = Guid::new_v7();
    let conn_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ports: vec![PortFullDto {
                guid: port_guid.clone(),
                ..Default::default()
            }],
            connectors: vec![ConnectorFullDto {
                guid: conn_guid.clone(),
                code: "C".into(),
                port: Some(PortIdDto {
                    guid: port_guid.clone(),
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, type_guid, conn_guid)
}

/// Kit with one type containing one port (for port setter tests).
pub fn kit_with_type_only() -> (KitStoreRef, Guid) {
    let type_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    (KitStore::from_full_dto(dto), type_guid)
}

pub fn kit_with_port() -> (KitStoreRef, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let port_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ports: vec![PortFullDto {
                guid: port_guid.clone(),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, type_guid, port_guid)
}

pub fn kit_with_file() -> (KitStoreRef, Guid) {
    let file_guid = Guid::new_v7();
    let kit_guid = Guid::new_v7();
    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        files: vec![FileFullDto {
            guid: file_guid.clone(),
            url: "https://example.com/f".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (kit, file_guid)
}

/// One design, two pieces, one connection (for connection / side tests).
pub fn kit_with_connection() -> (KitStoreRef, Guid, Guid, Guid, Guid, Guid) {
    let type_guid = Guid::new_v7();
    let design_guid = Guid::new_v7();
    let piece_a = Guid::new_v7();
    let piece_b = Guid::new_v7();
    let conn_guid = Guid::new_v7();
    let side_a = Guid::new_v7();
    let side_b = Guid::new_v7();
    let kit_guid = Guid::new_v7();

    let dto = KitFullDto {
        guid: kit_guid,
        name: "kit".into(),
        types: vec![TypeFullDto {
            guid: type_guid.clone(),
            name: "typ".into(),
            ..Default::default()
        }],
        designs: vec![DesignFullDto {
            guid: design_guid.clone(),
            name: "des".into(),
            pieces: vec![
                PieceFullDto {
                    guid: piece_a.clone(),
                    r#type: Some(TypeIdDto {
                        guid: type_guid.clone(),
                    }),
                    ..Default::default()
                },
                PieceFullDto {
                    guid: piece_b.clone(),
                    r#type: Some(TypeIdDto {
                        guid: type_guid.clone(),
                    }),
                    ..Default::default()
                },
            ],
            connections: vec![ConnectionFullDto {
                guid: conn_guid.clone(),
                connected: SideMetadataDto {
                    guid: side_a,
                    piece: PieceIdDto {
                        guid: piece_a.clone(),
                    },
                    port: None,
                    design_piece: None,
                },
                connecting: SideMetadataDto {
                    guid: side_b,
                    piece: PieceIdDto {
                        guid: piece_b.clone(),
                    },
                    port: None,
                    design_piece: None,
                },
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let kit = KitStore::from_full_dto(dto);
    (
        kit,
        type_guid,
        design_guid,
        piece_a,
        piece_b,
        conn_guid,
    )
}

/// Design metadata change with a single piece: field change, design hash, flatten+derived, kit hash, validation.
pub fn assert_design_scalar_metadata_events(
    evs: &[KitEvent],
    design_er: EntityRef,
    kit_er: EntityRef,
    piece_g: &Guid,
    field: &'static str,
) {
    assert_eq!(evs.len(), 7, "{evs:?}");
    assert!(
        matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == design_er && *f == field),
        "ev0 {:?}",
        evs.get(0)
    );
    assert!(
        matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == design_er),
        "ev1 {:?}",
        evs.get(1)
    );
    assert!(
        matches!(
            &evs[2],
            KitEvent::FlattenInvalidated { design, pieces }
                if *design == design_er.guid && pieces.len() == 1 && pieces[0] == *piece_g
        ),
        "ev2 {:?}",
        evs.get(2)
    );
    assert!(
        matches!(
            &evs[3],
            KitEvent::DerivedChanged { entity, field: "flat_plane" } if entity.guid == *piece_g
        ),
        "ev3 {:?}",
        evs.get(3)
    );
    assert!(
        matches!(
            &evs[4],
            KitEvent::DerivedChanged { entity, field: "flat_center" } if entity.guid == *piece_g
        ),
        "ev4 {:?}",
        evs.get(4)
    );
    assert!(
        matches!(&evs[5], KitEvent::HashInvalidated { entity } if *entity == kit_er),
        "ev5 {:?}",
        evs.get(5)
    );
    assert!(
        matches!(evs[6], KitEvent::ValidationInvalidated),
        "ev6 {:?}",
        evs.get(6)
    );
}

pub fn assert_piece_geometry_change(
    evs: &[KitEvent],
    piece_er: EntityRef,
    design_er: EntityRef,
    kit_er: EntityRef,
    piece_g: &Guid,
    field: &'static str,
) {
    assert_eq!(evs.len(), 7, "{evs:?}");
    assert!(matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == piece_er && *f == field));
    assert!(matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == piece_er));
    assert!(matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == design_er));
    assert!(matches!(&evs[3], KitEvent::HashInvalidated { entity } if *entity == kit_er));
    assert!(
        matches!(&evs[4], KitEvent::FlattenInvalidated { design, pieces } if *design == design_er.guid && pieces.contains(piece_g)),
        "ev4 {:?}",
        evs.get(4)
    );
    assert!(matches!(&evs[5], KitEvent::DerivedChanged { entity, field: "flat_plane" } if entity.guid == *piece_g));
    assert!(matches!(&evs[6], KitEvent::DerivedChanged { entity, field: "flat_center" } if entity.guid == *piece_g));
}

pub fn assert_piece_scalar_hash_only(
    evs: &[KitEvent],
    piece_er: EntityRef,
    design_er: EntityRef,
    kit_er: EntityRef,
    field: &'static str,
) {
    assert_eq!(evs.len(), 4, "{evs:?}");
    assert!(matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == piece_er && *f == field));
    assert!(matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == piece_er));
    assert!(matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == design_er));
    assert!(matches!(&evs[3], KitEvent::HashInvalidated { entity } if *entity == kit_er));
}

pub fn assert_type_metadata_core(evs: &[KitEvent], typ_er: EntityRef, kit_er: EntityRef, field: &'static str) {
    assert_eq!(evs.len(), 4, "{evs:?}");
    assert!(matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == typ_er && *f == field));
    assert!(matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == typ_er));
    assert!(matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == kit_er));
    assert!(matches!(evs[3], KitEvent::ValidationInvalidated));
}

/// Assert the first events match: FieldChanged(field), HashInvalidated(self), ValidationInvalidated.
pub fn assert_kit_metadata_core(evs: &[KitEvent], kit_ref: EntityRef, field: &'static str) {
    assert!(
        evs.len() >= 3,
        "expected at least 3 events, got {:?}",
        evs
    );
    assert!(
        matches!(
            &evs[0],
            KitEvent::FieldChanged { entity, field: f }
                if *entity == kit_ref && *f == field
        ),
        "ev[0] want FieldChanged {{ field: {} }}, got {:?}",
        field,
        evs.get(0)
    );
    assert!(
        matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == kit_ref),
        "ev[1] want HashInvalidated kit, got {:?}",
        evs.get(1)
    );
    assert!(
        matches!(evs[2], KitEvent::ValidationInvalidated),
        "ev[2] want ValidationInvalidated, got {:?}",
        evs.get(2)
    );
}
