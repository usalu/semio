// #region 🔖Header
// [👤semio📚server💻semio-session🔖schema](repo://p/u/semio/b/l/server/f/schema.rs)
// 2026 Ueli Saluz <ueli@semio-tech.de>
// AGPL-3.0
// SQL schema creation and migration for the session backend.
// #endregion 🔖Header

use sqlx_postgres::PgPool;

// #region 🔖Migrations
// Migrations MUST create all schemas, enums, and tables on startup.

pub async fn run_migrations(pool: &PgPool) {
    create_schemas(pool).await;
    create_enums(pool).await;
    create_runtime_tables(pool).await;
    create_core_tables(pool).await;
    create_semio_tables(pool).await;
    tracing::info!("database migrations complete");
}

async fn create_schemas(pool: &PgPool) {
    for schema in &["runtime", "core", "history", "semio"] {
        sqlx_core::query::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema))
            .execute(pool)
            .await
            .expect("failed to create schema");
    }
}

async fn create_enums(pool: &PgPool) {
    let stmts = [
        "DO $$ BEGIN
            CREATE TYPE lifecycle_status AS ENUM ('active', 'tombstoned');
        EXCEPTION WHEN duplicate_object THEN NULL; END $$",
        "DO $$ BEGIN
            CREATE TYPE session_status AS ENUM ('active', 'passivated', 'closed');
        EXCEPTION WHEN duplicate_object THEN NULL; END $$",
        "DO $$ BEGIN
            CREATE TYPE command_status AS ENUM ('pending', 'accepted', 'rejected');
        EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    ];
    for s in &stmts {
        sqlx_core::query::query(s).execute(pool).await.expect("failed to create enum");
    }
}

// #endregion 🔖Migrations

// #region 🔖Runtime Tables

async fn create_runtime_tables(pool: &PgPool) {
    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.session (
            session_id UUID PRIMARY KEY,
            root_kit_id UUID NOT NULL,
            domain_version BIGINT NOT NULL DEFAULT 0,
            semio_version BIGINT NOT NULL DEFAULT 0,
            status session_status NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
        )"
    ).execute(pool).await.expect("runtime.session");

    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.session_command (
            command_id UUID PRIMARY KEY,
            session_id UUID NOT NULL REFERENCES runtime.session(session_id),
            client_id UUID NOT NULL,
            request_id UUID NOT NULL,
            base_domain_version BIGINT NOT NULL,
            accepted_domain_version BIGINT,
            command_kind TEXT NOT NULL,
            actor_person_id UUID NOT NULL,
            received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            applied_at TIMESTAMPTZ,
            status command_status NOT NULL DEFAULT 'pending',
            UNIQUE (session_id, client_id, request_id)
        )"
    ).execute(pool).await.expect("runtime.session_command");

    sqlx_core::query::query(
        "CREATE TABLE IF NOT EXISTS runtime.property_clock (
            session_id UUID NOT NULL,
            entity_kind TEXT NOT NULL,
            entity_id UUID NOT NULL,
            property_key TEXT NOT NULL,
            last_changed_domain_version BIGINT NOT NULL,
            last_command_id UUID NOT NULL,
            PRIMARY KEY (session_id, entity_kind, entity_id, property_key)
        )"
    ).execute(pool).await.expect("runtime.property_clock");
}

// #endregion 🔖Runtime Tables

// #region 🔖Core Tables

async fn create_core_tables(pool: &PgPool) {
    create_core_kit(pool).await;
    create_core_author(pool).await;
    create_core_location(pool).await;
    create_core_folder(pool).await;
    create_core_file(pool).await;
    create_core_tag(pool).await;
    create_core_concept(pool).await;
    create_core_port(pool).await;
    create_core_quality(pool).await;
    create_core_type(pool).await;
    create_core_connector(pool).await;
    create_core_model(pool).await;
    create_core_prop(pool).await;
    create_core_attribute(pool).await;
    create_core_design(pool).await;
    create_core_layer(pool).await;
    create_core_piece(pool).await;
    create_core_group(pool).await;
    create_core_connection(pool).await;
    create_core_stat(pool).await;
}

async fn exec(pool: &PgPool, sql: &str, name: &str) {
    sqlx_core::query::query(sql).execute(pool).await.expect(name);
}

async fn create_core_kit(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.kit (
        session_id UUID NOT NULL, kit_id UUID NOT NULL,
        name TEXT NOT NULL, version TEXT, description TEXT,
        icon TEXT, image TEXT, preview TEXT, remote TEXT,
        homepage TEXT, license TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, kit_id)
    )", "core.kit").await;
}

async fn create_core_author(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.author (
        session_id UUID NOT NULL, author_id UUID NOT NULL,
        name TEXT NOT NULL, email TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, author_id)
    )", "core.author").await;
}

async fn create_core_location(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.location (
        session_id UUID NOT NULL, location_id UUID NOT NULL,
        name TEXT NOT NULL, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, location_id)
    )", "core.location").await;
}

async fn create_core_folder(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.folder (
        session_id UUID NOT NULL, folder_id UUID NOT NULL,
        name TEXT NOT NULL, parent_folder_id UUID,
        description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, folder_id)
    )", "core.folder").await;
}

async fn create_core_file(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.file (
        session_id UUID NOT NULL, file_id UUID NOT NULL,
        name TEXT NOT NULL, remote TEXT, folder_id UUID,
        size_bytes BIGINT, hash TEXT, blob_ref TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, file_id)
    )", "core.file").await;
}

async fn create_core_tag(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.tag (
        session_id UUID NOT NULL, tag_id UUID NOT NULL,
        name TEXT NOT NULL, description TEXT, icon TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, tag_id)
    )", "core.tag").await;
}

async fn create_core_concept(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.concept (
        session_id UUID NOT NULL, concept_id UUID NOT NULL,
        name TEXT NOT NULL, description TEXT, icon TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, concept_id)
    )", "core.concept").await;
}

async fn create_core_port(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.port (
        session_id UUID NOT NULL, port_id UUID NOT NULL,
        name TEXT NOT NULL, description TEXT, icon TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, port_id)
    )", "core.port").await;
    exec(pool, "CREATE TABLE IF NOT EXISTS core.port_compatibility (
        session_id UUID NOT NULL, port_id UUID NOT NULL,
        compatible_port_id UUID NOT NULL,
        PRIMARY KEY (session_id, port_id, compatible_port_id)
    )", "core.port_compatibility").await;
}

async fn create_core_quality(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.quality (
        session_id UUID NOT NULL, quality_id UUID NOT NULL,
        key TEXT NOT NULL, name TEXT NOT NULL,
        description TEXT, icon TEXT, unit TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, quality_id)
    )", "core.quality").await;
}

async fn create_core_type(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.type_entity (
        session_id UUID NOT NULL, type_id UUID NOT NULL,
        name TEXT NOT NULL, parent_type_id UUID,
        description TEXT, icon TEXT, image TEXT,
        folder TEXT, unit TEXT, stock INTEGER,
        is_abstract BOOLEAN, virtual_type BOOLEAN,
        location_id UUID,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, type_id)
    )", "core.type_entity").await;
}

async fn create_core_connector(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.connector (
        session_id UUID NOT NULL, connector_id UUID NOT NULL,
        type_id UUID NOT NULL, name TEXT,
        t DOUBLE PRECISION NOT NULL DEFAULT 0,
        point_x DOUBLE PRECISION NOT NULL DEFAULT 0,
        point_y DOUBLE PRECISION NOT NULL DEFAULT 0,
        point_z DOUBLE PRECISION NOT NULL DEFAULT 0,
        direction_x DOUBLE PRECISION NOT NULL DEFAULT 0,
        direction_y DOUBLE PRECISION NOT NULL DEFAULT 0,
        direction_z DOUBLE PRECISION NOT NULL DEFAULT 1,
        description TEXT, port_id UUID, mandatory BOOLEAN,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, connector_id)
    )", "core.connector").await;
}

async fn create_core_model(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.model (
        session_id UUID NOT NULL, model_id UUID NOT NULL,
        type_id UUID NOT NULL, file_id UUID NOT NULL,
        name TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, model_id)
    )", "core.model").await;
}

async fn create_core_prop(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.prop (
        session_id UUID NOT NULL, prop_id UUID NOT NULL,
        quality_id UUID NOT NULL, value TEXT NOT NULL,
        unit TEXT, owner_kind TEXT NOT NULL, owner_id UUID NOT NULL,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, prop_id)
    )", "core.prop").await;
}

async fn create_core_attribute(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.attribute (
        session_id UUID NOT NULL, attribute_id UUID NOT NULL,
        key TEXT NOT NULL, value TEXT, definition TEXT,
        owner_kind TEXT NOT NULL, owner_id UUID NOT NULL,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, attribute_id)
    )", "core.attribute").await;
}

// #endregion 🔖Core Tables

// #region 🔖Core Design Tables

async fn create_core_design(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.design (
        session_id UUID NOT NULL, design_id UUID NOT NULL,
        name TEXT NOT NULL, parent_design_id UUID,
        description TEXT, icon TEXT, image TEXT,
        folder TEXT, unit TEXT,
        is_abstract BOOLEAN, can_scale BOOLEAN, can_mirror BOOLEAN,
        active_layer_id UUID, location_id UUID,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, design_id)
    )", "core.design").await;
}

async fn create_core_layer(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.layer (
        session_id UUID NOT NULL, layer_id UUID NOT NULL,
        design_id UUID NOT NULL, path TEXT NOT NULL,
        is_hidden BOOLEAN, is_locked BOOLEAN,
        color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, layer_id)
    )", "core.layer").await;
}

async fn create_core_piece(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.piece (
        session_id UUID NOT NULL, piece_id UUID NOT NULL,
        design_id UUID NOT NULL, name TEXT,
        type_id UUID, design_ref_id UUID,
        plane_origin_x DOUBLE PRECISION, plane_origin_y DOUBLE PRECISION,
        plane_origin_z DOUBLE PRECISION,
        plane_x_axis_x DOUBLE PRECISION, plane_x_axis_y DOUBLE PRECISION,
        plane_x_axis_z DOUBLE PRECISION,
        plane_y_axis_x DOUBLE PRECISION, plane_y_axis_y DOUBLE PRECISION,
        plane_y_axis_z DOUBLE PRECISION,
        center_u DOUBLE PRECISION, center_v DOUBLE PRECISION,
        scale DOUBLE PRECISION,
        mirror_origin_x DOUBLE PRECISION, mirror_origin_y DOUBLE PRECISION,
        mirror_origin_z DOUBLE PRECISION,
        mirror_x_axis_x DOUBLE PRECISION, mirror_x_axis_y DOUBLE PRECISION,
        mirror_x_axis_z DOUBLE PRECISION,
        mirror_y_axis_x DOUBLE PRECISION, mirror_y_axis_y DOUBLE PRECISION,
        mirror_y_axis_z DOUBLE PRECISION,
        is_hidden BOOLEAN, is_locked BOOLEAN,
        color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, piece_id)
    )", "core.piece").await;
}

async fn create_core_group(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.group_entity (
        session_id UUID NOT NULL, group_id UUID NOT NULL,
        design_id UUID NOT NULL, name TEXT,
        color TEXT, description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, group_id)
    )", "core.group_entity").await;
    exec(pool, "CREATE TABLE IF NOT EXISTS core.group_piece (
        session_id UUID NOT NULL, group_id UUID NOT NULL,
        piece_id UUID NOT NULL, ordinal INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (session_id, group_id, piece_id)
    )", "core.group_piece").await;
}

async fn create_core_connection(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.connection (
        session_id UUID NOT NULL, connection_id UUID NOT NULL,
        design_id UUID NOT NULL,
        connected_piece_id UUID NOT NULL,
        connected_design_piece_id UUID,
        connected_connector_id UUID,
        connecting_piece_id UUID NOT NULL,
        connecting_design_piece_id UUID,
        connecting_connector_id UUID,
        gap DOUBLE PRECISION NOT NULL DEFAULT 0,
        shift_val DOUBLE PRECISION NOT NULL DEFAULT 0,
        rise DOUBLE PRECISION NOT NULL DEFAULT 0,
        rotation DOUBLE PRECISION NOT NULL DEFAULT 0,
        turn DOUBLE PRECISION NOT NULL DEFAULT 0,
        tilt DOUBLE PRECISION NOT NULL DEFAULT 0,
        u DOUBLE PRECISION, v DOUBLE PRECISION,
        description TEXT,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, connection_id)
    )", "core.connection").await;
}

async fn create_core_stat(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS core.stat (
        session_id UUID NOT NULL, stat_id UUID NOT NULL,
        design_id UUID NOT NULL, quality_id UUID NOT NULL,
        unit TEXT, min DOUBLE PRECISION, min_excluded BOOLEAN,
        max DOUBLE PRECISION, max_excluded BOOLEAN,
        lifecycle lifecycle_status NOT NULL DEFAULT 'active',
        PRIMARY KEY (session_id, stat_id)
    )", "core.stat").await;
}

// #endregion 🔖Core Design Tables

// #region 🔖Semio Tables

async fn create_semio_tables(pool: &PgPool) {
    exec(pool, "CREATE TABLE IF NOT EXISTS semio.person (
        session_id UUID NOT NULL,
        person_id UUID NOT NULL,
        frontend_id TEXT NOT NULL,
        display_name TEXT, color TEXT,
        is_present BOOLEAN NOT NULL DEFAULT true,
        last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.person").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.cursor (
        session_id UUID NOT NULL,
        person_id UUID NOT NULL,
        frontend_id TEXT NOT NULL,
        u DOUBLE PRECISION NOT NULL,
        v DOUBLE PRECISION NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.cursor").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.look (
        session_id UUID NOT NULL,
        person_id UUID NOT NULL,
        frontend_id TEXT NOT NULL,
        position_x DOUBLE PRECISION NOT NULL,
        position_y DOUBLE PRECISION NOT NULL,
        position_z DOUBLE PRECISION NOT NULL,
        forward_x DOUBLE PRECISION NOT NULL,
        forward_y DOUBLE PRECISION NOT NULL,
        forward_z DOUBLE PRECISION NOT NULL,
        up_x DOUBLE PRECISION NOT NULL,
        up_y DOUBLE PRECISION NOT NULL,
        up_z DOUBLE PRECISION NOT NULL,
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        PRIMARY KEY (session_id, person_id, frontend_id)
    )", "semio.look").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.selection_piece (
        session_id UUID NOT NULL,
        person_id UUID NOT NULL,
        frontend_id TEXT NOT NULL,
        piece_id UUID NOT NULL,
        PRIMARY KEY (session_id, person_id, frontend_id, piece_id)
    )", "semio.selection_piece").await;

    exec(pool, "CREATE TABLE IF NOT EXISTS semio.selection_design (
        session_id UUID NOT NULL,
        person_id UUID NOT NULL,
        frontend_id TEXT NOT NULL,
        design_id UUID NOT NULL,
        PRIMARY KEY (session_id, person_id, frontend_id, design_id)
    )", "semio.selection_design").await;
}

// #endregion 🔖Semio Tables
