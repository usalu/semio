// #region 🔖Header
// [👤semio📚server💻semio-session🔖directory](repo://p/u/semio/b/l/server/f/directory.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// Session directory: process-global registry mapping SessionId to actor handles.
// #endregion 🔖Header

use dashmap::DashMap;
use sqlx_postgres::PgPool;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::actor::{ActorMessage, SessionActor};
use crate::domain::SessionId;
use crate::event::SessionEvent;

// #region 🔖SessionHandle
// SessionHandle MUST hold the sender to an active session actor.

#[derive(Clone)]
pub struct SessionHandle {
    pub command_tx: mpsc::Sender<ActorMessage>,
    pub event_tx: broadcast::Sender<SessionEvent>,
}

// #endregion 🔖SessionHandle

// #region 🔖SessionDirectory
// SessionDirectory MUST provide get-or-create semantics for session actors.

#[derive(Clone)]
pub struct SessionDirectory {
    sessions: Arc<DashMap<Uuid, SessionHandle>>,
    pool: PgPool,
}

impl SessionDirectory {
    pub fn new(pool: PgPool) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            pool,
        }
    }

    pub async fn get_or_activate(
        &self,
        session_id: SessionId,
    ) -> Option<SessionHandle> {
        if let Some(handle) = self.sessions.get(&session_id.0) {
            return Some(handle.clone());
        }

        let state = crate::persistence::load_session_state(
            &self.pool, session_id.0,
        ).await.ok()?;

        let (command_tx, command_rx) = mpsc::channel(256);
        let (event_tx, _) = broadcast::channel(256);

        let handle = SessionHandle {
            command_tx,
            event_tx: event_tx.clone(),
        };
        self.sessions.insert(session_id.0, handle.clone());

        let pool = self.pool.clone();
        let sessions = self.sessions.clone();
        let sid = session_id.0;

        tokio::spawn(async move {
            let mut actor = SessionActor::new(state, pool, event_tx);
            actor.run(command_rx).await;
            sessions.remove(&sid);
            tracing::info!("session actor {} passivated", sid);
        });

        Some(handle)
    }

    pub fn remove(&self, session_id: &Uuid) {
        self.sessions.remove(session_id);
    }
}

// #endregion 🔖SessionDirectory
