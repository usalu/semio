-- #region 🧲Header
-- semio/postgres/schema.sql
-- 2026 Ueli Saluz <ueli@semio-tech.de>
-- Canonical PostgreSQL schema for semio kit persistence, including normalized kit snapshots and kit version-control state.
-- #endregion 🧲Header

-- #region 🧱Schemas
CREATE SCHEMA IF NOT EXISTS core;
CREATE SCHEMA IF NOT EXISTS history;
CREATE SCHEMA IF NOT EXISTS runtime;
-- #endregion 🧱Schemas

-- #region 🗂️Catalog
CREATE TABLE IF NOT EXISTS core.semio_schema (
	schema_version TEXT PRIMARY KEY,
	engine TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS core.kit (
	kit_id TEXT PRIMARY KEY,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	uri TEXT,
	initial_snapshot_id TEXT,
	current_checkpoint_id TEXT,
	current_materialized_snapshot_id TEXT,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS core.kit_snapshot (
	snapshot_id TEXT PRIMARY KEY,
	kit_id TEXT NOT NULL REFERENCES core.kit (kit_id) ON DELETE CASCADE,
	snapshot_kind TEXT NOT NULL CHECK (snapshot_kind IN ('initial', 'materialized', 'checkpoint')),
	source_checkpoint_id TEXT,
	label TEXT,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 🗂️Catalog

-- #region 🧬Definitions
CREATE TABLE IF NOT EXISTS core.quality (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	quality_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	key TEXT NOT NULL,
	name TEXT NOT NULL,
	uri TEXT,
	unit TEXT,
	definition TEXT,
	description TEXT,
	PRIMARY KEY (snapshot_id, quality_id)
);

CREATE TABLE IF NOT EXISTS core.benchmark (
	snapshot_id TEXT NOT NULL,
	benchmark_id TEXT NOT NULL,
	quality_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	icon TEXT,
	min_value DOUBLE PRECISION,
	min_excluded BOOLEAN,
	max_value DOUBLE PRECISION,
	max_excluded BOOLEAN,
	definition TEXT,
	PRIMARY KEY (snapshot_id, benchmark_id),
	FOREIGN KEY (snapshot_id, quality_id) REFERENCES core.quality (snapshot_id, quality_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.family (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	PRIMARY KEY (snapshot_id, family_id)
);

CREATE TABLE IF NOT EXISTS core.port (
	snapshot_id TEXT NOT NULL,
	port_id TEXT NOT NULL,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	PRIMARY KEY (snapshot_id, port_id),
	FOREIGN KEY (snapshot_id, family_id) REFERENCES core.family (snapshot_id, family_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.port_compatibility (
	snapshot_id TEXT NOT NULL,
	port_id TEXT NOT NULL,
	compatible_port_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, port_id, compatible_port_id),
	FOREIGN KEY (snapshot_id, port_id) REFERENCES core.port (snapshot_id, port_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, compatible_port_id) REFERENCES core.port (snapshot_id, port_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.folder (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	folder_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	parent_folder_id TEXT,
	path TEXT,
	description TEXT,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ,
	PRIMARY KEY (snapshot_id, folder_id),
	FOREIGN KEY (snapshot_id, parent_folder_id) REFERENCES core.folder (snapshot_id, folder_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.file (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	file_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	folder_id TEXT,
	remote_url TEXT,
	mime_kind TEXT,
	size_bytes BIGINT,
	checksum TEXT,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ,
	PRIMARY KEY (snapshot_id, file_id),
	FOREIGN KEY (snapshot_id, folder_id) REFERENCES core.folder (snapshot_id, folder_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.author (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	author_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	email TEXT,
	role TEXT,
	PRIMARY KEY (snapshot_id, author_id)
);

CREATE TABLE IF NOT EXISTS core.tag (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	tag_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	PRIMARY KEY (snapshot_id, tag_id)
);

CREATE TABLE IF NOT EXISTS core.concept (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	concept_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	PRIMARY KEY (snapshot_id, concept_id)
);
-- #endregion 🧬Definitions

-- #region 🧩KindsAndDesigns
CREATE TABLE IF NOT EXISTS core.type_entity (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	type_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	parent_type_id TEXT,
	variant TEXT,
	is_abstract BOOLEAN NOT NULL DEFAULT false,
	folder TEXT,
	stock BIGINT,
	virtual_type BOOLEAN NOT NULL DEFAULT false,
	unit TEXT,
	description TEXT,
	icon TEXT,
	image TEXT,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ,
	PRIMARY KEY (snapshot_id, type_id),
	FOREIGN KEY (snapshot_id, parent_type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.type_family (
	snapshot_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, type_id, family_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, family_id) REFERENCES core.family (snapshot_id, family_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.type_author (
	snapshot_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	author_id TEXT NOT NULL,
	rank INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, type_id, author_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, author_id) REFERENCES core.author (snapshot_id, author_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.type_concept (
	snapshot_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	concept_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, type_id, concept_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, concept_id) REFERENCES core.concept (snapshot_id, concept_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.type_tag (
	snapshot_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	tag_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, type_id, tag_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, tag_id) REFERENCES core.tag (snapshot_id, tag_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.connector (
	snapshot_id TEXT NOT NULL,
	connector_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT,
	code TEXT,
	point_x DOUBLE PRECISION,
	point_y DOUBLE PRECISION,
	point_z DOUBLE PRECISION,
	direction_x DOUBLE PRECISION,
	direction_y DOUBLE PRECISION,
	direction_z DOUBLE PRECISION,
	t DOUBLE PRECISION,
	mandatory BOOLEAN,
	max_children INTEGER,
	port_id TEXT,
	description TEXT,
	PRIMARY KEY (snapshot_id, connector_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, port_id) REFERENCES core.port (snapshot_id, port_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.representation (
	snapshot_id TEXT NOT NULL,
	representation_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	file_id TEXT,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT,
	description TEXT,
	uri TEXT,
	media_kind TEXT,
	payload_json JSONB,
	PRIMARY KEY (snapshot_id, representation_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, file_id) REFERENCES core.file (snapshot_id, file_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.representation_tag (
	snapshot_id TEXT NOT NULL,
	representation_id TEXT NOT NULL,
	tag_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, representation_id, tag_id),
	FOREIGN KEY (snapshot_id, representation_id) REFERENCES core.representation (snapshot_id, representation_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, tag_id) REFERENCES core.tag (snapshot_id, tag_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.design (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	design_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT NOT NULL,
	parent_design_id TEXT,
	variant TEXT,
	view_center_u DOUBLE PRECISION,
	view_center_v DOUBLE PRECISION,
	view_zoom DOUBLE PRECISION,
	unit TEXT,
	active_layer_id TEXT,
	is_abstract BOOLEAN,
	folder TEXT,
	can_scale BOOLEAN,
	can_mirror BOOLEAN,
	description TEXT,
	icon TEXT,
	image TEXT,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ,
	PRIMARY KEY (snapshot_id, design_id),
	FOREIGN KEY (snapshot_id, parent_design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.design_family (
	snapshot_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, design_id, family_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, family_id) REFERENCES core.family (snapshot_id, family_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.design_author (
	snapshot_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	author_id TEXT NOT NULL,
	rank INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, design_id, author_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, author_id) REFERENCES core.author (snapshot_id, author_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.design_concept (
	snapshot_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	concept_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, design_id, concept_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, concept_id) REFERENCES core.concept (snapshot_id, concept_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.design_tag (
	snapshot_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	tag_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, design_id, tag_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, tag_id) REFERENCES core.tag (snapshot_id, tag_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.layer (
	snapshot_id TEXT NOT NULL,
	layer_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	path TEXT NOT NULL,
	name TEXT,
	is_hidden BOOLEAN NOT NULL DEFAULT false,
	is_locked BOOLEAN NOT NULL DEFAULT false,
	color TEXT,
	description TEXT,
	PRIMARY KEY (snapshot_id, layer_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.piece (
	snapshot_id TEXT NOT NULL,
	piece_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT,
	type_id TEXT,
	design_ref_id TEXT,
	plane_origin_x DOUBLE PRECISION,
	plane_origin_y DOUBLE PRECISION,
	plane_origin_z DOUBLE PRECISION,
	plane_x_axis_x DOUBLE PRECISION,
	plane_x_axis_y DOUBLE PRECISION,
	plane_x_axis_z DOUBLE PRECISION,
	plane_y_axis_x DOUBLE PRECISION,
	plane_y_axis_y DOUBLE PRECISION,
	plane_y_axis_z DOUBLE PRECISION,
	center_u DOUBLE PRECISION,
	center_v DOUBLE PRECISION,
	scale DOUBLE PRECISION,
	mirror_plane_origin_x DOUBLE PRECISION,
	mirror_plane_origin_y DOUBLE PRECISION,
	mirror_plane_origin_z DOUBLE PRECISION,
	mirror_plane_x_axis_x DOUBLE PRECISION,
	mirror_plane_x_axis_y DOUBLE PRECISION,
	mirror_plane_x_axis_z DOUBLE PRECISION,
	mirror_plane_y_axis_x DOUBLE PRECISION,
	mirror_plane_y_axis_y DOUBLE PRECISION,
	mirror_plane_y_axis_z DOUBLE PRECISION,
	is_hidden BOOLEAN NOT NULL DEFAULT false,
	is_locked BOOLEAN NOT NULL DEFAULT false,
	color TEXT,
	description TEXT,
	PRIMARY KEY (snapshot_id, piece_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE SET NULL,
	FOREIGN KEY (snapshot_id, design_ref_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.group_entity (
	snapshot_id TEXT NOT NULL,
	group_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	name TEXT,
	color TEXT,
	icon TEXT,
	description TEXT,
	PRIMARY KEY (snapshot_id, group_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.group_piece (
	snapshot_id TEXT NOT NULL,
	group_id TEXT NOT NULL,
	piece_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, group_id, piece_id),
	FOREIGN KEY (snapshot_id, group_id) REFERENCES core.group_entity (snapshot_id, group_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.connection (
	snapshot_id TEXT NOT NULL,
	connection_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	connected_piece_id TEXT NOT NULL,
	connected_design_piece_id TEXT,
	connected_connector_id TEXT,
	connecting_piece_id TEXT NOT NULL,
	connecting_design_piece_id TEXT,
	connecting_connector_id TEXT,
	gap DOUBLE PRECISION NOT NULL DEFAULT 0,
	shift DOUBLE PRECISION NOT NULL DEFAULT 0,
	rise DOUBLE PRECISION NOT NULL DEFAULT 0,
	rotation DOUBLE PRECISION NOT NULL DEFAULT 0,
	turn DOUBLE PRECISION NOT NULL DEFAULT 0,
	tilt DOUBLE PRECISION NOT NULL DEFAULT 0,
	u DOUBLE PRECISION,
	v DOUBLE PRECISION,
	description TEXT,
	PRIMARY KEY (snapshot_id, connection_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, connected_piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, connected_design_piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE SET NULL,
	FOREIGN KEY (snapshot_id, connected_connector_id) REFERENCES core.connector (snapshot_id, connector_id) ON DELETE SET NULL,
	FOREIGN KEY (snapshot_id, connecting_piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, connecting_design_piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE SET NULL,
	FOREIGN KEY (snapshot_id, connecting_connector_id) REFERENCES core.connector (snapshot_id, connector_id) ON DELETE SET NULL,
	CHECK (connected_piece_id <> connecting_piece_id)
);

CREATE TABLE IF NOT EXISTS core.stat (
	snapshot_id TEXT NOT NULL,
	stat_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	quality_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	unit TEXT,
	min_value DOUBLE PRECISION,
	min_excluded BOOLEAN,
	max_value DOUBLE PRECISION,
	max_excluded BOOLEAN,
	PRIMARY KEY (snapshot_id, stat_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, quality_id) REFERENCES core.quality (snapshot_id, quality_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.prop (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	prop_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	key TEXT NOT NULL,
	value_text TEXT,
	value_json JSONB,
	unit TEXT,
	quality_id TEXT,
	connector_id TEXT,
	PRIMARY KEY (snapshot_id, prop_id),
	FOREIGN KEY (snapshot_id, quality_id) REFERENCES core.quality (snapshot_id, quality_id) ON DELETE SET NULL,
	FOREIGN KEY (snapshot_id, connector_id) REFERENCES core.connector (snapshot_id, connector_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS core.type_prop (
	snapshot_id TEXT NOT NULL,
	type_id TEXT NOT NULL,
	prop_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, type_id, prop_id),
	FOREIGN KEY (snapshot_id, type_id) REFERENCES core.type_entity (snapshot_id, type_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, prop_id) REFERENCES core.prop (snapshot_id, prop_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.design_prop (
	snapshot_id TEXT NOT NULL,
	design_id TEXT NOT NULL,
	prop_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, design_id, prop_id),
	FOREIGN KEY (snapshot_id, design_id) REFERENCES core.design (snapshot_id, design_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, prop_id) REFERENCES core.prop (snapshot_id, prop_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.piece_prop (
	snapshot_id TEXT NOT NULL,
	piece_id TEXT NOT NULL,
	prop_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, piece_id, prop_id),
	FOREIGN KEY (snapshot_id, piece_id) REFERENCES core.piece (snapshot_id, piece_id) ON DELETE CASCADE,
	FOREIGN KEY (snapshot_id, prop_id) REFERENCES core.prop (snapshot_id, prop_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.attribute (
	snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	attribute_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	key TEXT NOT NULL,
	value_text TEXT,
	definition TEXT,
	owner_kind TEXT NOT NULL,
	owner_id TEXT NOT NULL,
	PRIMARY KEY (snapshot_id, attribute_id)
);

CREATE TABLE IF NOT EXISTS core.kit_author (
	snapshot_id TEXT NOT NULL,
	author_id TEXT NOT NULL,
	rank INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, author_id),
	FOREIGN KEY (snapshot_id, author_id) REFERENCES core.author (snapshot_id, author_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.kit_concept (
	snapshot_id TEXT NOT NULL,
	concept_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, concept_id),
	FOREIGN KEY (snapshot_id, concept_id) REFERENCES core.concept (snapshot_id, concept_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS core.kit_tag (
	snapshot_id TEXT NOT NULL,
	tag_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (snapshot_id, tag_id),
	FOREIGN KEY (snapshot_id, tag_id) REFERENCES core.tag (snapshot_id, tag_id) ON DELETE CASCADE
);
-- #endregion 🧩KindsAndDesigns

-- #region 🌿VersionControl
CREATE TABLE IF NOT EXISTS history.kit_checkpoint (
	checkpoint_id TEXT PRIMARY KEY,
	kit_id TEXT NOT NULL REFERENCES core.kit (kit_id) ON DELETE CASCADE,
	parent_checkpoint_id TEXT REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE SET NULL,
	message TEXT,
	checkpoint_time TIMESTAMPTZ,
	hash TEXT NOT NULL UNIQUE,
	changes_json JSONB NOT NULL DEFAULT '[]'::jsonb,
	is_release BOOLEAN NOT NULL DEFAULT false,
	release_snapshot_id TEXT,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS history.kit_checkpoint_author (
	checkpoint_id TEXT NOT NULL REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE CASCADE,
	ordinal INTEGER NOT NULL DEFAULT 0,
	author_ref TEXT NOT NULL,
	PRIMARY KEY (checkpoint_id, ordinal)
);

CREATE TABLE IF NOT EXISTS history.kit_alternative (
	alternative_id TEXT PRIMARY KEY,
	kit_id TEXT NOT NULL REFERENCES core.kit (kit_id) ON DELETE CASCADE,
	name TEXT NOT NULL,
	root_checkpoint_id TEXT NOT NULL REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE CASCADE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	UNIQUE (kit_id, name)
);

CREATE TABLE IF NOT EXISTS history.kit_alternative_checkpoint (
	alternative_id TEXT NOT NULL REFERENCES history.kit_alternative (alternative_id) ON DELETE CASCADE,
	ordinal INTEGER NOT NULL,
	checkpoint_id TEXT NOT NULL REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE CASCADE,
	PRIMARY KEY (alternative_id, ordinal),
	UNIQUE (alternative_id, checkpoint_id)
);

CREATE TABLE IF NOT EXISTS history.kit_release (
	checkpoint_id TEXT PRIMARY KEY REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE CASCADE,
	materialized_snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	change_list_json JSONB NOT NULL DEFAULT '[]'::jsonb,
	released_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 🌿VersionControl

-- #region ⚙️Runtime
CREATE TABLE IF NOT EXISTS runtime.kit_session (
	session_id TEXT PRIMARY KEY,
	kit_id TEXT NOT NULL REFERENCES core.kit (kit_id) ON DELETE CASCADE,
	status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'passivated', 'closed')),
	head_checkpoint_id TEXT,
	head_alternative_id TEXT,
	metadata_json JSONB NOT NULL DEFAULT '{}'::jsonb,
	opened_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	closed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS runtime.kit_draft (
	draft_id TEXT PRIMARY KEY,
	session_id TEXT NOT NULL REFERENCES runtime.kit_session (session_id) ON DELETE CASCADE,
	parent_checkpoint_id TEXT REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE SET NULL,
	target_alternative_id TEXT REFERENCES history.kit_alternative (alternative_id) ON DELETE SET NULL,
	before_snapshot_id TEXT NOT NULL REFERENCES core.kit_snapshot (snapshot_id) ON DELETE CASCADE,
	status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'aborted', 'finalized')),
	current_transaction_ordinal INTEGER NOT NULL DEFAULT 0,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	finalized_checkpoint_id TEXT REFERENCES history.kit_checkpoint (checkpoint_id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS runtime.kit_transaction (
	transaction_id TEXT PRIMARY KEY,
	draft_id TEXT NOT NULL REFERENCES runtime.kit_draft (draft_id) ON DELETE CASCADE,
	ordinal INTEGER NOT NULL,
	state TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'finalized', 'aborted')),
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	finalized_at TIMESTAMPTZ,
	UNIQUE (draft_id, ordinal)
);

CREATE TABLE IF NOT EXISTS runtime.kit_transaction_change (
	transaction_id TEXT NOT NULL REFERENCES runtime.kit_transaction (transaction_id) ON DELETE CASCADE,
	ordinal INTEGER NOT NULL,
	change_json JSONB NOT NULL,
	PRIMARY KEY (transaction_id, ordinal)
);
-- #endregion ⚙️Runtime

-- #region 📚Indexes
CREATE INDEX IF NOT EXISTS idx_kit_snapshot_kit_kind ON core.kit_snapshot (kit_id, snapshot_kind, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_family_snapshot_ordinal ON core.family (snapshot_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_port_snapshot_family ON core.port (snapshot_id, family_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_folder_snapshot_parent ON core.folder (snapshot_id, parent_folder_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_file_snapshot_folder ON core.file (snapshot_id, folder_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_type_snapshot_ordinal ON core.type_entity (snapshot_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_connector_snapshot_type ON core.connector (snapshot_id, type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_representation_snapshot_type ON core.representation (snapshot_id, type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_design_snapshot_ordinal ON core.design (snapshot_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_layer_snapshot_design ON core.layer (snapshot_id, design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_piece_snapshot_design ON core.piece (snapshot_id, design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_connection_snapshot_design ON core.connection (snapshot_id, design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_snapshot_owner ON core.attribute (snapshot_id, owner_kind, owner_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_checkpoint_kit_parent ON history.kit_checkpoint (kit_id, parent_checkpoint_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_checkpoint_kit_time ON history.kit_checkpoint (kit_id, checkpoint_time DESC);
CREATE INDEX IF NOT EXISTS idx_alternative_kit_name ON history.kit_alternative (kit_id, name);
CREATE INDEX IF NOT EXISTS idx_alt_checkpoint_checkpoint ON history.kit_alternative_checkpoint (checkpoint_id);
CREATE INDEX IF NOT EXISTS idx_session_kit_status ON runtime.kit_session (kit_id, status, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_draft_session_status ON runtime.kit_draft (session_id, status, updated_at DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_draft_one_active_branch ON runtime.kit_draft (session_id, parent_checkpoint_id, target_alternative_id) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_transaction_draft_state ON runtime.kit_transaction (draft_id, state, ordinal DESC);
-- #endregion 📚Indexes
