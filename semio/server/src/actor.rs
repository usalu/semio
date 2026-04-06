// #region 🔖Header
// [👤semio📚server💻semio-session🔖actor](repo://p/u/semio/b/l/server/f/actor.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Session actor: single-writer task processing commands sequentially.
// #endregion 🔖Header

use sqlx_postgres::PgPool;
use tokio::sync::{broadcast, mpsc, oneshot};
use uuid::Uuid;

use crate::command::*;
use crate::domain::*;
use crate::error::SessionError;
use crate::event::*;
use crate::persistence;
use crate::state::*;

// #region 🔖ActorMessage
// ActorMessage MUST be the inbox message kind for the session actor.

pub enum ActorMessage {
    DomainCommand {
        envelope: CommandEnvelope,
        command: DomainCommand,
        reply: oneshot::Sender<Result<CommandResult, SessionError>>,
    },
    SemioCommand {
        envelope: SemioEnvelope,
        command: SemioCommand,
        reply: oneshot::Sender<Result<(), SessionError>>,
    },
    GetSnapshot {
        reply: oneshot::Sender<SessionSnapshot>,
    },
}

// #endregion 🔖ActorMessage

// #region 🔖SessionSnapshot
// SessionSnapshot MUST hold the full state for HTTP snapshot responses.

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionSnapshot {
    pub session_id: Uuid,
    pub domain_version: DomainVersion,
    pub semio_version: SemioVersion,
    pub kit: serde_json::Value,
}

// #endregion 🔖SessionSnapshot

// #region 🔖SessionActor
// SessionActor MUST process commands one at a time in arrival order.

pub struct SessionActor {
    state: SessionState,
    pool: PgPool,
    event_tx: broadcast::Sender<SessionEvent>,
}

impl SessionActor {
    pub fn new(
        state: SessionState,
        pool: PgPool,
        event_tx: broadcast::Sender<SessionEvent>,
    ) -> Self {
        Self { state, pool, event_tx }
    }

    pub async fn run(&mut self, mut rx: mpsc::Receiver<ActorMessage>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                ActorMessage::DomainCommand { envelope, command, reply } => {
                    let result = self.handle_domain_command(envelope, command).await;
                    let _ = reply.send(result);
                }
                ActorMessage::SemioCommand { envelope, command, reply } => {
                    let result = self.handle_semio_command(envelope, command).await;
                    let _ = reply.send(result);
                }
                ActorMessage::GetSnapshot { reply } => {
                    let snapshot = self.build_snapshot();
                    let _ = reply.send(snapshot);
                }
            }
        }
    }

    async fn handle_domain_command(
        &mut self,
        envelope: CommandEnvelope,
        command: DomainCommand,
    ) -> Result<CommandResult, SessionError> {
        let session_id = self.state.session_id.0;
        let cmd_id = envelope.command_id.0;

        let is_new = persistence::record_command(
            &self.pool, session_id, cmd_id,
            envelope.client_id.0, envelope.request_id.0,
            envelope.base_domain_version,
            &format!("{:?}", command),
            envelope.actor_person_id.0,
        ).await?;

        if !is_new {
            return Ok(CommandResult::IdempotentDuplicate);
        }

        let new_version = self.state.domain_version + 1;
        let changes = self.apply_domain_command(&command, new_version, cmd_id).await?;

        persistence::bump_domain_version(&self.pool, session_id, new_version).await?;
        persistence::mark_command_accepted(&self.pool, cmd_id, new_version).await?;

        self.state.domain_version = new_version;

        let event = SessionEvent::DomainCommandAccepted {
            command_id: envelope.command_id,
            domain_version: new_version,
            changes,
        };
        let _ = self.event_tx.send(event);

        Ok(CommandResult::Accepted { domain_version: new_version })
    }

    async fn apply_domain_command(
        &mut self,
        command: &DomainCommand,
        version: DomainVersion,
        cmd_id: Uuid,
    ) -> Result<Vec<EntityChange>, SessionError> {
        let sid = self.state.session_id.0;
        let mut changes = Vec::new();

        match command {
            DomainCommand::PatchKit(patch) => {
                if let Some(name) = patch.fields.get("name").and_then(|v| v.as_str()) {
                    self.state.kit.name = name.to_string();
                    sqlx_core::query::query("UPDATE core.kit SET name = $3 WHERE session_id = $1 AND kit_id = $2")
                        .bind(sid).bind(self.state.kit.kit_id).bind(name)
                        .execute(&self.pool).await?;
                    persistence::upsert_property_clock(
                        &self.pool, sid, "kit", self.state.kit.kit_id,
                        "kit_name", version, cmd_id,
                    ).await?;
                }
                changes.push(EntityChange::Updated {
                    entity_kind: EntityKind::Kit,
                    entity_id: self.state.kit.kit_id,
                    changed_fields: patch.fields.clone(),
                });
            }
            DomainCommand::CreateType(create) => {
                let name = create.fields.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled");
                sqlx_core::query::query(
                    "INSERT INTO core.type_entity (session_id, type_id, name)
                     VALUES ($1, $2, $3)"
                ).bind(sid).bind(create.entity_id).bind(name)
                .execute(&self.pool).await?;

                self.state.types.insert(create.entity_id, TypeState {
                    type_id: create.entity_id,
                    name: name.to_string(),
                    parent_type_id: None, description: None,
                    icon: None, image: None, folder: None, unit: None,
                    stock: None, is_abstract: None, virtual_type: None,
                    location_id: None,
                    connectors: std::collections::BTreeMap::new(),
                    models: std::collections::BTreeMap::new(),
                    props: std::collections::BTreeMap::new(),
                    lifecycle: Lifecycle::Active,
                });
                changes.push(EntityChange::Created {
                    entity_kind: EntityKind::Type,
                    entity_id: create.entity_id,
                    snapshot: create.fields.clone(),
                });
            }
            DomainCommand::DeleteType(del) => {
                sqlx_core::query::query(
                    "UPDATE core.type_entity SET lifecycle = 'tombstoned'
                     WHERE session_id = $1 AND type_id = $2"
                ).bind(sid).bind(del.entity_id)
                .execute(&self.pool).await?;

                if let Some(t) = self.state.types.get_mut(&del.entity_id) {
                    t.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted {
                    entity_kind: EntityKind::Type,
                    entity_id: del.entity_id,
                });
            }
            DomainCommand::CreateDesign(create) => {
                let name = create.fields.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled");
                sqlx_core::query::query(
                    "INSERT INTO core.design (session_id, design_id, name)
                     VALUES ($1, $2, $3)"
                ).bind(sid).bind(create.entity_id).bind(name)
                .execute(&self.pool).await?;

                self.state.designs.insert(create.entity_id, DesignState {
                    design_id: create.entity_id,
                    name: name.to_string(),
                    parent_design_id: None, description: None,
                    icon: None, image: None, folder: None, unit: None,
                    is_abstract: None, can_scale: None, can_mirror: None,
                    active_layer_id: None, location_id: None,
                    pieces: std::collections::BTreeMap::new(),
                    connections: std::collections::BTreeMap::new(),
                    layers: std::collections::BTreeMap::new(),
                    groups: std::collections::BTreeMap::new(),
                    stats: std::collections::BTreeMap::new(),
                    props: std::collections::BTreeMap::new(),
                    lifecycle: Lifecycle::Active,
                });
                changes.push(EntityChange::Created {
                    entity_kind: EntityKind::Design,
                    entity_id: create.entity_id,
                    snapshot: create.fields.clone(),
                });
            }
            DomainCommand::DeleteDesign(del) => {
                sqlx_core::query::query(
                    "UPDATE core.design SET lifecycle = 'tombstoned'
                     WHERE session_id = $1 AND design_id = $2"
                ).bind(sid).bind(del.entity_id)
                .execute(&self.pool).await?;

                if let Some(d) = self.state.designs.get_mut(&del.entity_id) {
                    d.lifecycle = Lifecycle::Tombstoned { at: version, by: CommandId(cmd_id) };
                }
                changes.push(EntityChange::Deleted {
                    entity_kind: EntityKind::Design,
                    entity_id: del.entity_id,
                });
            }
            DomainCommand::CreatePiece(create) => {
                let name = create.fields.get("name").and_then(|v| v.as_str());
                sqlx_core::query::query(
                    "INSERT INTO core.piece (session_id, piece_id, design_id, name)
                     VALUES ($1, $2, $3, $4)"
                ).bind(sid).bind(create.piece_id).bind(create.design_id).bind(name)
                .execute(&self.pool).await?;

                if let Some(design) = self.state.designs.get_mut(&create.design_id) {
                    design.pieces.insert(create.piece_id, PieceState {
                        piece_id: create.piece_id,
                        name: name.map(|s| s.to_string()),
                        type_id: None, design_ref_id: None, plane: None,
                        center: None, scale: None, mirror_plane: None,
                        is_hidden: None, is_locked: None,
                        color: None, description: None,
                        lifecycle: Lifecycle::Active,
                    });
                }
                changes.push(EntityChange::Created {
                    entity_kind: EntityKind::Piece,
                    entity_id: create.piece_id,
                    snapshot: create.fields.clone(),
                });
            }
            DomainCommand::CreateConnection(create) => {
                let connected_piece = create.fields.get("connected_piece_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or(Uuid::nil());
                let connecting_piece = create.fields.get("connecting_piece_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or(Uuid::nil());

                sqlx_core::query::query(
                    "INSERT INTO core.connection
                        (session_id, connection_id, design_id,
                         connected_piece_id, connecting_piece_id)
                     VALUES ($1, $2, $3, $4, $5)"
                ).bind(sid).bind(create.connection_id).bind(create.design_id)
                .bind(connected_piece).bind(connecting_piece)
                .execute(&self.pool).await?;

                if let Some(design) = self.state.designs.get_mut(&create.design_id) {
                    design.connections.insert(create.connection_id, ConnectionState {
                        connection_id: create.connection_id,
                        connected_piece_id: connected_piece,
                        connected_design_piece_id: None,
                        connected_connector_id: None,
                        connecting_piece_id: connecting_piece,
                        connecting_design_piece_id: None,
                        connecting_connector_id: None,
                        gap: 0.0, shift: 0.0, rise: 0.0,
                        rotation: 0.0, turn: 0.0, tilt: 0.0,
                        u: None, v: None, description: None,
                        lifecycle: Lifecycle::Active,
                    });
                }
                changes.push(EntityChange::Created {
                    entity_kind: EntityKind::Connection,
                    entity_id: create.connection_id,
                    snapshot: create.fields.clone(),
                });
            }
            DomainCommand::Batch(batch) => {
                for sub in &batch.commands {
                    let sub_changes = Box::pin(
                        self.apply_domain_command(sub, version, cmd_id)
                    ).await?;
                    changes.extend(sub_changes);
                }
            }
            _ => {
                tracing::warn!("unhandled command variant: {:?}", std::mem::discriminant(command));
            }
        }

        Ok(changes)
    }

    async fn handle_semio_command(
        &mut self,
        envelope: SemioEnvelope,
        command: SemioCommand,
    ) -> Result<(), SessionError> {
        let sid = self.state.session_id.0;
        let pid = envelope.person_id.0;
        let fid = &envelope.frontend_id;
        let new_version = self.state.semio_version + 1;

        let update = match &command {
            SemioCommand::UpsertCursor(c) => {
                sqlx_core::query::query(
                    "INSERT INTO semio.cursor (session_id, person_id, frontend_id, u, v)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (session_id, person_id, frontend_id)
                     DO UPDATE SET u = $4, v = $5, updated_at = now()"
                ).bind(sid).bind(pid).bind(fid).bind(c.u).bind(c.v)
                .execute(&self.pool).await?;
                SemioUpdate::CursorMoved { u: c.u, v: c.v }
            }
            SemioCommand::UpsertLook(l) => {
                sqlx_core::query::query(
                    "INSERT INTO semio.look
                        (session_id, person_id, frontend_id,
                         position_x, position_y, position_z,
                         forward_x, forward_y, forward_z,
                         up_x, up_y, up_z)
                     VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
                     ON CONFLICT (session_id, person_id, frontend_id)
                     DO UPDATE SET
                         position_x=$4, position_y=$5, position_z=$6,
                         forward_x=$7, forward_y=$8, forward_z=$9,
                         up_x=$10, up_y=$11, up_z=$12, updated_at=now()"
                ).bind(sid).bind(pid).bind(fid)
                .bind(l.position[0]).bind(l.position[1]).bind(l.position[2])
                .bind(l.forward[0]).bind(l.forward[1]).bind(l.forward[2])
                .bind(l.up[0]).bind(l.up[1]).bind(l.up[2])
                .execute(&self.pool).await?;
                SemioUpdate::LookChanged {
                    position: l.position, forward: l.forward, up: l.up,
                }
            }
            SemioCommand::SetSelection(s) => {
                sqlx_core::query::query("DELETE FROM semio.selection_piece WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                for piece_id in &s.piece_ids {
                    sqlx_core::query::query("INSERT INTO semio.selection_piece (session_id,person_id,frontend_id,piece_id) VALUES ($1,$2,$3,$4)")
                        .bind(sid).bind(pid).bind(fid).bind(piece_id)
                        .execute(&self.pool).await?;
                }
                SemioUpdate::SelectionChanged {
                    piece_ids: s.piece_ids.clone(),
                    design_ids: s.design_ids.clone(),
                }
            }
            SemioCommand::ClearPresence(_) => {
                sqlx_core::query::query("DELETE FROM semio.cursor WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                sqlx_core::query::query("DELETE FROM semio.look WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                sqlx_core::query::query("DELETE FROM semio.selection_piece WHERE session_id=$1 AND person_id=$2 AND frontend_id=$3")
                    .bind(sid).bind(pid).bind(fid).execute(&self.pool).await?;
                SemioUpdate::PresenceCleared
            }
        };

        persistence::bump_semio_version(&self.pool, sid, new_version).await?;
        self.state.semio_version = new_version;

        let _ = self.event_tx.send(SessionEvent::SemioUpdated {
            semio_version: new_version,
            person_id: envelope.person_id,
            frontend_id: envelope.frontend_id.clone(),
            update,
        });

        Ok(())
    }

    fn build_snapshot(&self) -> SessionSnapshot {
        let kit_json = serde_json::json!({
            "kit_id": self.state.kit.kit_id,
            "name": self.state.kit.name,
            "version": self.state.kit.version,
            "description": self.state.kit.description,
            "types": self.state.types.keys().collect::<Vec<_>>(),
            "designs": self.state.designs.keys().collect::<Vec<_>>(),
        });
        SessionSnapshot {
            session_id: self.state.session_id.0,
            domain_version: self.state.domain_version,
            semio_version: self.state.semio_version,
            kit: kit_json,
        }
    }
}

// #endregion 🔖SessionActor
