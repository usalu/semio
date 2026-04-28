//! semio rust implementation
#![allow(clippy::new_without_default)]

pub mod entities {

    pub mod weak {}

    pub mod strong {

        trait Entity {
            fn id(&self) -> Id;
            fn hash(&self) -> Hash;
        }

        trait Store {}
        pub mod piece {
            #[derive(Debug)]
            pub struct Piece {
                pub id: Id,
                pub name: Option<String>,
                pub description: Option<String>,
                pub pose: Pose,
                pub scale: Option<f64>,
                pub mirror_plane: Option<Plane>,
                pub hidden: Option<bool>,
                pub locked: Option<bool>,
                pub color: Option<String>,
                pub props: Vec<PropRef>,
                pub attributes: Vec<AttributeRef>,
                pub type_ref: Option<TypeWeak>,
                pub parent_piece: Option<PieceWeak>,
                pub parent_connection: Option<ConnectionWeak>,
                pub parent_design: DesignWeak,
                pub(crate) event_bus: Weak<EventBus>,
                hash_cache: Cache<String>,
                flat_plane: Cache<Plane>,
                flat_center: Cache<Coordinate>,
            }

            impl Piece {
                pub fn new() -> Self {
                    Self {
                        id: Id::new_v7(),
                        name: None,
                        description: None,
                        pose: Pose::default(),
                        scale: None,
                        mirror_plane: None,
                        hidden: None,
                        locked: None,
                        color: None,
                        props: Vec::new(),
                        attributes: Vec::new(),
                        type_ref: None,
                        parent_piece: None,
                        parent_connection: None,
                        parent_design: Weak::new(),
                        event_bus: Weak::new(),
                        hash_cache: Cache::default(),
                        flat_pose: Cache::default(),
                    }
                }
            }

            #[Object]
            impl Piece {
                async fn id(&self) -> Id {
                    self.id
                }
                async fn name(&self) -> Option<String> {
                    self.name.clone()
                }
            }
        }
    }
}
