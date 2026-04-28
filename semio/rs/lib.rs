//! semio rust implementation
#![allow(clippy::new_without_default)]

use crate::kit::KitEvent;
use std::rc::Weak;

type Reference = Weak<dyn Entity>;
trait Entity {
    fn id(&self) -> Id;
    fn hash(&self) -> Hash;
    fn owner(&self) -> Reference;
}

trait Store {}

pub enum Event {
    Operation,
    VC,
}
pub mod kit {
    use crate::kit::design::DesignEvent;

    pub enum KitEvent {
        Design(DesignEvent),
    }

    pub mod coordinate {
        #[derive(Debug, Default)]
        pub struct Coordinate {
            pub u: f64,
            pub v: f64,
        }
    }

    pub mod offset {
        #[derive(Debug, Default)]
        pub struct Offset {
            pub u: f64,
            pub v: f64,
        }
    }

    pub mod point {
        #[derive(Debug, Default)]
        pub struct Point {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }
    }

    pub mod vector {
        #[derive(Debug, Default)]
        pub struct Vector {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }
    }

    pub mod plane {
        use crate::kit::point::Point;
        use crate::kit::vector::Vector;

        #[derive(Debug, Default)]
        pub struct Plane {
            pub origin: Point,
            pub x_axis: Vector,
            pub y_axis: Vector,
        }
    }

    pub mod position {
        use crate::kit::{coordinate::Coordinate, plane::Plane};

        #[derive(Debug, Default)]
        pub struct PositionStore {
            pub plane: Option<Plane>,
            pub center: Option<Coordinate>,
        }
    }
    pub mod type_ {}

    pub mod design {
        pub enum DesignOperation {
            CreatedFixedPiece { position: Position },
        }
        pub mod piece {
            use crate::kit::design::piece::blueprint::Blueprint;

            pub mod blueprint {
                pub struct Blueprint {
                    pub type_: Option<TypeReference>,
                    pub design: Option<DesignReference>,
                }
            }

            #[derive(Debug)]
            pub struct Piece {
                pub id: Id,
                pub owner: DesignWeak,
                pub name: Option<String>,
                pub description: Option<String>,
                pub pose: Option<Position>,
                pub scale: Option<f64>,
                pub mirror_plane: Option<Plane>,
                pub hidden: Option<bool>,
                pub locked: Option<bool>,
                pub color: Option<String>,
                pub props: Vec<PropReference>,
                pub attributes: Vec<AttributeReference>,
                pub blueprint: Blueprint,
                pub parent_piece: Option<PieceWeak>,
                pub parent_connection: Option<ConnectionWeak>,
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
                        pose: None,
                        scale: None,
                        mirror_plane: None,
                        hidden: None,
                        locked: None,
                        color: None,
                        props: Vec::new(),
                        attributes: Vec::new(),
                        blueprint: None,
                        parent_piece: None,
                        parent_connection: None,
                        owner: Weak::new(),
                        event_bus: Weak::new(),
                        hash_cache: Cache::default(),
                        flat_pose: Cache::default(),
                    }
                }

                #[inline]
                fn emit_piece_event(&self, event: Event::Kit::Design::Piece) {
                    if let Some(d) = self.parent_design.upgrade() {
                        if let Ok(d) = d.read() {
                            // d.emit_event();
                            return;
                        }
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

            pub mod operation {
                pub struct CreatedFixedPiece {
                    pub position: Position,
                }
            }
        }

        pub mod connection {
            pub mod side {}
        }
    }
}

pub mod graphql {
    use async_graphql::Context;
    use async_graphql::{Context, Object, SimpleObject, Subscription};
    use futures_util::{Stream, StreamExt};

    struct Subscription;
    #[Subscription]
    impl Subscription {
        async fn created_fixed_piece(&self, ctx: &Context<'_>) -> impl Stream<Item = CreatedFixedPiece> {
            let receiver = ctx.data_unchecked::<async_channel::Receiver<Event::Kit::Design::CreatedFixedPiece>>();

            while let Ok(event) = receiver.recv().await {
                if let CreatedFixedPiece { piece_id: event_piece_id, design_id } = event {
                    if event_piece_id == piece_id {
                        return Some(CreatedFixedPiece { piece_id, design_id });
                    }
                }
            }
            None
        }
    }
}
