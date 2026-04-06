// #region 🔖Header
// [👤semio📚server💻semio-session🔖persistence](repo://p/u/semio/b/l/server/f/persistence.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// PostgreSQL persistence: pool creation, session CRUD, snapshot loading.
// #endregion 🔖Header

use sqlx_postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

use crate::domain::*;
use crate::error::SessionError;
use crate::state::*;
use std::collections::BTreeMap;

// #region 🔖Pool
// Pool MUST create a connection pool from a DATABASE_URL.

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("failed to connect to PostgreSQL")
}

// #endregion 🔖Pool

// #region 🔖Session CRUD
// Session CRUD MUST create, load, and update session metadata.

pub async fn create_session(
    pool: &PgPool,
    session_id: Uuid,
    kit_id: Uuid,
    kit_name: &str,
) -> Result<(), SessionError> {
    let mut tx = pool.begin().await?;

    sqlx_core::query::query(
        "INSERT INTO runtime.session (session_id, root_kit_id)
         VALUES ($1, $2)"
    )
    .bind(session_id)
    .bind(kit_id)
    .execute(&mut *tx)
    .await?;

    sqlx_core::query::query(
        "INSERT INTO core.kit (session_id, kit_id, name)
         VALUES ($1, $2, $3)"
    )
    .bind(session_id)
    .bind(kit_id)
    .bind(kit_name)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn load_session_meta(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<(DomainVersion, SemioVersion), SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (i64, i64)>(
        "SELECT domain_version, semio_version FROM runtime.session
         WHERE session_id = $1"
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| SessionError::SessionNotFound(session_id.to_string()))?;
    Ok(row)
}

pub async fn bump_domain_version(
    pool: &PgPool,
    session_id: Uuid,
    new_version: DomainVersion,
) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "UPDATE runtime.session SET domain_version = $2, updated_at = now()
         WHERE session_id = $1"
    )
    .bind(session_id)
    .bind(new_version)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn bump_semio_version(
    pool: &PgPool,
    session_id: Uuid,
    new_version: SemioVersion,
) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "UPDATE runtime.session SET semio_version = $2, updated_at = now()
         WHERE session_id = $1"
    )
    .bind(session_id)
    .bind(new_version)
    .execute(pool)
    .await?;
    Ok(())
}

// #endregion 🔖Session CRUD

// #region 🔖Snapshot Loading
// Snapshot Loading MUST reconstruct full SessionState from DB.

pub async fn load_session_state(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<SessionState, SessionError> {
    let (domain_version, semio_version) = load_session_meta(pool, session_id).await?;

    let kit = load_kit(pool, session_id).await?;
    let authors = load_authors(pool, session_id).await?;
    let tags = load_tags(pool, session_id).await?;
    let concepts = load_concepts(pool, session_id).await?;
    let ports = load_ports(pool, session_id).await?;
    let qualities = load_qualities(pool, session_id).await?;
    let folders = load_folders(pool, session_id).await?;
    let files = load_files(pool, session_id).await?;
    let types = load_types(pool, session_id).await?;
    let designs = load_designs(pool, session_id).await?;

    Ok(SessionState {
        session_id: SessionId(session_id),
        domain_version,
        semio_version,
        status: SessionStatus::Active,
        kit,
        authors,
        locations: BTreeMap::new(),
        folders,
        files,
        tags,
        concepts,
        ports,
        qualities,
        types,
        designs,
        semio_people: BTreeMap::new(),
    })
}

async fn load_kit(pool: &PgPool, sid: Uuid) -> Result<KitState, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<String>, Option<String>)>(
        "SELECT kit_id, name, version, description, icon, image, preview,
                remote, homepage, license
         FROM core.kit WHERE session_id = $1 AND lifecycle = 'active' LIMIT 1"
    )
    .bind(sid)
    .fetch_one(pool)
    .await?;

    Ok(KitState {
        kit_id: row.0,
        name: row.1,
        version: row.2,
        description: row.3,
        icon: row.4,
        image: row.5,
        preview: row.6,
        remote: row.7,
        homepage: row.8,
        license: row.9,
        lifecycle: Lifecycle::Active,
    })
}

async fn load_authors(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, AuthorState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>)>(
        "SELECT author_id, name, email FROM core.author
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, AuthorState {
        author_id: r.0, name: r.1, email: r.2,
        lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_tags(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, TagState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT tag_id, name, description, icon FROM core.tag
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, TagState {
        tag_id: r.0, name: r.1, description: r.2, icon: r.3,
        lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_concepts(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, ConceptState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT concept_id, name, description, icon FROM core.concept
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, ConceptState {
        concept_id: r.0, name: r.1, description: r.2, icon: r.3,
        lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_ports(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, PortState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
        "SELECT port_id, name, description, icon FROM core.port
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, PortState {
        port_id: r.0, name: r.1, description: r.2, icon: r.3,
        compatible_port_ids: vec![], lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_qualities(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, QualityState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, String, Option<String>, Option<String>, Option<String>)>(
        "SELECT quality_id, key, name, description, icon, unit FROM core.quality
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, QualityState {
        quality_id: r.0, key: r.1, name: r.2, description: r.3,
        icon: r.4, unit: r.5, lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_folders(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, FolderState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>)>(
        "SELECT folder_id, name, parent_folder_id, description FROM core.folder
         WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, FolderState {
        folder_id: r.0, name: r.1, parent_folder_id: r.2,
        description: r.3, lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_files(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, FileState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<String>, Option<Uuid>,
        Option<i64>, Option<String>, Option<String>)>(
        "SELECT file_id, name, remote, folder_id, size_bytes, hash, blob_ref
         FROM core.file WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, FileState {
        file_id: r.0, name: r.1, remote: r.2, folder_id: r.3,
        size: r.4, hash: r.5, blob: r.6,
        lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_types(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, TypeState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<i32>, Option<bool>, Option<bool>, Option<Uuid>)>(
        "SELECT type_id, name, parent_type_id, description, icon, image,
                folder, unit, stock, is_abstract, virtual_type, location_id
         FROM core.type_entity WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.0, TypeState {
        type_id: r.0, name: r.1, parent_type_id: r.2,
        description: r.3, icon: r.4, image: r.5, folder: r.6,
        unit: r.7, stock: r.8, is_abstract: r.9, virtual_type: r.10,
        location_id: r.11,
        connectors: BTreeMap::new(), models: BTreeMap::new(),
        props: BTreeMap::new(), lifecycle: Lifecycle::Active,
    })).collect())
}

async fn load_designs(pool: &PgPool, sid: Uuid) -> Result<BTreeMap<Uuid, DesignState>, SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, String, Option<Uuid>, Option<String>,
        Option<String>, Option<String>, Option<String>, Option<String>,
        Option<bool>, Option<bool>, Option<bool>, Option<Uuid>, Option<Uuid>)>(
        "SELECT design_id, name, parent_design_id, description, icon, image,
                folder, unit, is_abstract, can_scale, can_mirror,
                active_layer_id, location_id
         FROM core.design WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    let mut designs: BTreeMap<Uuid, DesignState> = rows.into_iter().map(|r| {
        (r.0, DesignState {
            design_id: r.0, name: r.1, parent_design_id: r.2,
            description: r.3, icon: r.4, image: r.5, folder: r.6,
            unit: r.7, is_abstract: r.8, can_scale: r.9, can_mirror: r.10,
            active_layer_id: r.11, location_id: r.12,
            pieces: BTreeMap::new(), connections: BTreeMap::new(),
            layers: BTreeMap::new(), groups: BTreeMap::new(),
            stats: BTreeMap::new(), props: BTreeMap::new(),
            lifecycle: Lifecycle::Active,
        })
    }).collect();

    load_pieces_into_designs(pool, sid, &mut designs).await?;
    load_connections_into_designs(pool, sid, &mut designs).await?;
    load_layers_into_designs(pool, sid, &mut designs).await?;

    Ok(designs)
}

async fn load_pieces_into_designs(
    pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>,
) -> Result<(), SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, Uuid, Option<String>, Option<Uuid>, Option<Uuid>,
        Option<f64>, Option<f64>, Option<bool>, Option<bool>,
        Option<String>, Option<String>)>(
        "SELECT piece_id, design_id, name, type_id, design_ref_id,
                center_u, center_v, is_hidden, is_locked, color, description
         FROM core.piece WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    for r in rows {
        if let Some(design) = designs.get_mut(&r.1) {
            design.pieces.insert(r.0, PieceState {
                piece_id: r.0, name: r.2, type_id: r.3,
                design_ref_id: r.4, plane: None,
                center: match (r.5, r.6) {
                    (Some(u), Some(v)) => Some([u, v]),
                    _ => None,
                },
                scale: None, mirror_plane: None,
                is_hidden: r.7, is_locked: r.8,
                color: r.9, description: r.10,
                lifecycle: Lifecycle::Active,
            });
        }
    }
    Ok(())
}

async fn load_connections_into_designs(
    pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>,
) -> Result<(), SessionError> {
    use sqlx_core::row::Row;
    let rows = sqlx_core::query::query(
        "SELECT connection_id, design_id,
                connected_piece_id, connected_design_piece_id, connected_connector_id,
                connecting_piece_id, connecting_design_piece_id, connecting_connector_id,
                gap, shift_val, rise, rotation, turn, tilt, u, v, description
         FROM core.connection WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    for r in rows {
        let connection_id: Uuid = r.get(0);
        let design_id: Uuid = r.get(1);
        if let Some(design) = designs.get_mut(&design_id) {
            design.connections.insert(connection_id, ConnectionState {
                connection_id,
                connected_piece_id: r.get(2),
                connected_design_piece_id: r.get(3),
                connected_connector_id: r.get(4),
                connecting_piece_id: r.get(5),
                connecting_design_piece_id: r.get(6),
                connecting_connector_id: r.get(7),
                gap: r.get(8), shift: r.get(9), rise: r.get(10),
                rotation: r.get(11), turn: r.get(12), tilt: r.get(13),
                u: r.get(14), v: r.get(15), description: r.get(16),
                lifecycle: Lifecycle::Active,
            });
        }
    }
    Ok(())
}

async fn load_layers_into_designs(
    pool: &PgPool, sid: Uuid, designs: &mut BTreeMap<Uuid, DesignState>,
) -> Result<(), SessionError> {
    let rows = sqlx_core::query_as::query_as::<_, (Uuid, Uuid, String, Option<bool>,
        Option<bool>, Option<String>, Option<String>)>(
        "SELECT layer_id, design_id, path, is_hidden, is_locked, color, description
         FROM core.layer WHERE session_id = $1 AND lifecycle = 'active'"
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;

    for r in rows {
        if let Some(design) = designs.get_mut(&r.1) {
            design.layers.insert(r.0, LayerState {
                layer_id: r.0, path: r.2,
                is_hidden: r.3, is_locked: r.4,
                color: r.5, description: r.6,
                lifecycle: Lifecycle::Active,
            });
        }
    }
    Ok(())
}

// #endregion 🔖Snapshot Loading

// #region 🔖Property Clock
// Property Clock MUST check and update per-property version tracking.

pub async fn check_property_clock(
    pool: &PgPool,
    session_id: Uuid,
    entity_kind: &str,
    entity_id: Uuid,
    property_key: &str,
    base_version: DomainVersion,
) -> Result<bool, SessionError> {
    let row = sqlx_core::query_as::query_as::<_, (i64,)>(
        "SELECT last_changed_domain_version FROM runtime.property_clock
         WHERE session_id = $1 AND entity_kind = $2
           AND entity_id = $3 AND property_key = $4"
    )
    .bind(session_id)
    .bind(entity_kind)
    .bind(entity_id)
    .bind(property_key)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((v,)) => Ok(v <= base_version),
        None => Ok(true),
    }
}

pub async fn upsert_property_clock(
    pool: &PgPool,
    session_id: Uuid,
    entity_kind: &str,
    entity_id: Uuid,
    property_key: &str,
    domain_version: DomainVersion,
    command_id: Uuid,
) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "INSERT INTO runtime.property_clock
            (session_id, entity_kind, entity_id, property_key,
             last_changed_domain_version, last_command_id)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (session_id, entity_kind, entity_id, property_key)
         DO UPDATE SET last_changed_domain_version = $5, last_command_id = $6"
    )
    .bind(session_id)
    .bind(entity_kind)
    .bind(entity_id)
    .bind(property_key)
    .bind(domain_version)
    .bind(command_id)
    .execute(pool)
    .await?;
    Ok(())
}

// #endregion 🔖Property Clock

// #region 🔖Command Idempotency
// Command Idempotency MUST record processed commands to reject duplicates.

pub async fn record_command(
    pool: &PgPool,
    session_id: Uuid,
    command_id: Uuid,
    client_id: Uuid,
    request_id: Uuid,
    base_version: DomainVersion,
    command_kind: &str,
    actor_person_id: Uuid,
) -> Result<bool, SessionError> {
    let result = sqlx_core::query::query(
        "INSERT INTO runtime.session_command
            (command_id, session_id, client_id, request_id,
             base_domain_version, command_kind, actor_person_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (session_id, client_id, request_id) DO NOTHING"
    )
    .bind(command_id)
    .bind(session_id)
    .bind(client_id)
    .bind(request_id)
    .bind(base_version)
    .bind(command_kind)
    .bind(actor_person_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn mark_command_accepted(
    pool: &PgPool,
    command_id: Uuid,
    accepted_version: DomainVersion,
) -> Result<(), SessionError> {
    sqlx_core::query::query(
        "UPDATE runtime.session_command
         SET status = 'accepted', accepted_domain_version = $2, applied_at = now()
         WHERE command_id = $1"
    )
    .bind(command_id)
    .bind(accepted_version)
    .execute(pool)
    .await?;
    Ok(())
}

// #endregion 🔖Command Idempotency
