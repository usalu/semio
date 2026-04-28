//! semio rust implementation
#![allow(clippy::new_without_default)]

trait Entity {
    fn id(&self) -> Id;
    fn hash(&self) -> Hash;
}

trait Store {}

pub mod kit {
    pub mod type_ {}

    pub mod design {
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
                pub props: Vec<PropReference>,
                pub attributes: Vec<AttributeReference>,
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

                #[inline]
                fn emit_ev(&self, ev: KitEvent) {
                    emit_weak(&self.event_bus, ev);
                }

                #[inline]
                fn emit_piece_ev(&self, piece_id: Id, event: crate::events::PieceEvent) {
                    if let Some(d) = self.parent_design.upgrade() {
                        if let Ok(d) = d.read() {
                            self.emit_ev(KitEvent::Design { design_id: d.id.clone(), event: crate::events::DesignEvent::Piece { piece_id, event } });
                            return;
                        }
                    }
                    self.emit_ev(KitEvent::Piece { piece_id, event });
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

        pub mod connection {
            pub mod side {}
        }
    }
}

pub mod graphql {
    #[Subscription]
    impl SubscriptionRoot {
        async fn on_user_change(&self, ctx: &Context<'_>) -> impl Stream<Item = User> {
            let tx = ctx.data_unchecked::<broadcast::Sender<UserChangedEvent>>();
            let mut rx = tx.subscribe();

            async_stream::stream! {
                while let Ok(event) = rx.recv().await {
                    // Yield the "Shell" object.
                    // No complex computation has happened yet.
                    yield User { id: event.user_id };
                }
            }
        }
    }
}
