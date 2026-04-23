-- #region 🧲Header
-- semio/sqlite/schema.sql
-- 2025 Ueli Saluz <ueli@semio-tech.com>
-- This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
-- Normalized SQLite schema for semio kit persistence aligned with the current Rust kit DTO graph.
-- #endregion 🧲Header

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS semio_schema (
	schema_version TEXT NOT NULL,
	engine TEXT NOT NULL,
	created_at TEXT NOT NULL,
	PRIMARY KEY (schema_version)
);

CREATE TABLE IF NOT EXISTS kit (
	id TEXT NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	uri TEXT,
	created_at TEXT,
	updated_at TEXT,
	PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS folder (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	path TEXT NOT NULL,
	description TEXT,
	kit_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	url TEXT NOT NULL,
	mime TEXT,
	size INTEGER,
	hash TEXT,
	description TEXT,
	created_at TEXT,
	updated_at TEXT,
	kit_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS type (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	stock INTEGER,
	virtual INTEGER,
	unit TEXT,
	location_id TEXT,
	created_at TEXT,
	updated_at TEXT,
	kit_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS family (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	kit_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS type_family (
	type_id TEXT NOT NULL,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	PRIMARY KEY (type_id, family_id),
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (family_id) REFERENCES family (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS port (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL DEFAULT '',
	icon TEXT,
	family TEXT,
	mandatory INTEGER,
	t REAL,
	description TEXT,
	point_x REAL,
	point_y REAL,
	point_z REAL,
	direction_x REAL,
	direction_y REAL,
	direction_z REAL,
	kit_id TEXT NOT NULL,
	parent_family_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (parent_family_id) REFERENCES family (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS port_compatible_family (
	port_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	family TEXT NOT NULL,
	PRIMARY KEY (port_id, ordinal),
	FOREIGN KEY (port_id) REFERENCES port (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS connector (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	code TEXT NOT NULL,
	description TEXT,
	port_id TEXT,
	type_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (port_id) REFERENCES port (id),
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS representation (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	url TEXT NOT NULL,
	description TEXT,
	file_id TEXT,
	type_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (file_id) REFERENCES file (id),
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS design (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	location_id TEXT,
	unit TEXT,
	created_at TEXT,
	updated_at TEXT,
	kit_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS design_family (
	design_id TEXT NOT NULL,
	family_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	PRIMARY KEY (design_id, family_id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	FOREIGN KEY (family_id) REFERENCES family (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS layer (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	color TEXT,
	order_index INTEGER,
	visible INTEGER,
	locked INTEGER,
	design_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS piece (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT,
	description TEXT,
	plane_origin_x REAL,
	plane_origin_y REAL,
	plane_origin_z REAL,
	plane_x_axis_x REAL,
	plane_x_axis_y REAL,
	plane_x_axis_z REAL,
	plane_y_axis_x REAL,
	plane_y_axis_y REAL,
	plane_y_axis_z REAL,
	center_x REAL,
	center_y REAL,
	center_z REAL,
	scale REAL,
	mirror_plane_origin_x REAL,
	mirror_plane_origin_y REAL,
	mirror_plane_origin_z REAL,
	mirror_plane_x_axis_x REAL,
	mirror_plane_x_axis_y REAL,
	mirror_plane_x_axis_z REAL,
	mirror_plane_y_axis_x REAL,
	mirror_plane_y_axis_y REAL,
	mirror_plane_y_axis_z REAL,
	hidden INTEGER,
	locked INTEGER,
	color TEXT,
	type_id TEXT,
	design_ref_id TEXT,
	design_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (type_id) REFERENCES type (id),
	FOREIGN KEY (design_ref_id) REFERENCES design (id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "group" (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	color TEXT,
	icon TEXT,
	design_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS group_piece (
	group_id TEXT NOT NULL,
	piece_id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	PRIMARY KEY (group_id, piece_id),
	FOREIGN KEY (group_id) REFERENCES "group" (id) ON DELETE CASCADE,
	FOREIGN KEY (piece_id) REFERENCES piece (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS connection (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	connected_side_id TEXT NOT NULL,
	connected_piece_id TEXT NOT NULL,
	connected_port_id TEXT,
	connected_design_piece_id TEXT,
	connecting_side_id TEXT NOT NULL,
	connecting_piece_id TEXT NOT NULL,
	connecting_port_id TEXT,
	connecting_design_piece_id TEXT,
	gap REAL,
	shift REAL,
	rise REAL,
	rotation REAL,
	turn REAL,
	tilt REAL,
	x REAL,
	y REAL,
	description TEXT,
	design_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (connected_piece_id) REFERENCES piece (id),
	FOREIGN KEY (connected_port_id) REFERENCES port (id),
	FOREIGN KEY (connected_design_piece_id) REFERENCES piece (id),
	FOREIGN KEY (connecting_piece_id) REFERENCES piece (id),
	FOREIGN KEY (connecting_port_id) REFERENCES port (id),
	FOREIGN KEY (connecting_design_piece_id) REFERENCES piece (id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS stat (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	unit TEXT,
	description TEXT,
	design_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS author (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	email TEXT NOT NULL,
	role TEXT,
	rank INTEGER,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS concept (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	order_index INTEGER,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS tag (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	order_index INTEGER,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	representation_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	FOREIGN KEY (representation_id) REFERENCES representation (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS quality (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT,
	unit TEXT,
	definition TEXT,
	description TEXT,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	port_id TEXT,
	connector_id TEXT,
	representation_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	FOREIGN KEY (port_id) REFERENCES port (id) ON DELETE CASCADE,
	FOREIGN KEY (connector_id) REFERENCES connector (id) ON DELETE CASCADE,
	FOREIGN KEY (representation_id) REFERENCES representation (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN port_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connector_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS benchmark (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	min_value REAL,
	max_value REAL,
	min_excluded INTEGER,
	max_excluded INTEGER,
	quality_id TEXT NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (quality_id) REFERENCES quality (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS prop (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	unit TEXT,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	piece_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	FOREIGN KEY (piece_id) REFERENCES piece (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN piece_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS attribute (
	id TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	definition TEXT,
	kit_id TEXT,
	type_id TEXT,
	design_id TEXT,
	piece_id TEXT,
	port_id TEXT,
	connector_id TEXT,
	representation_id TEXT,
	connection_id TEXT,
	family_id TEXT,
	PRIMARY KEY (id),
	FOREIGN KEY (kit_id) REFERENCES kit (id) ON DELETE CASCADE,
	FOREIGN KEY (type_id) REFERENCES type (id) ON DELETE CASCADE,
	FOREIGN KEY (design_id) REFERENCES design (id) ON DELETE CASCADE,
	FOREIGN KEY (piece_id) REFERENCES piece (id) ON DELETE CASCADE,
	FOREIGN KEY (port_id) REFERENCES port (id) ON DELETE CASCADE,
	FOREIGN KEY (connector_id) REFERENCES connector (id) ON DELETE CASCADE,
	FOREIGN KEY (representation_id) REFERENCES representation (id) ON DELETE CASCADE,
	FOREIGN KEY (connection_id) REFERENCES connection (id) ON DELETE CASCADE,
	FOREIGN KEY (family_id) REFERENCES family (id) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN piece_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN port_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connector_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connection_id IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN family_id IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE INDEX IF NOT EXISTS idx_folder_kit ON folder (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_file_kit ON file (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_type_kit ON type (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_port_kit ON port (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_port_parent_family ON port (parent_family_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_family_kit ON family (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_type_family_type ON type_family (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_design_family_design ON design_family (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_connector_type ON connector (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_representation_type ON representation (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_design_kit ON design (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_layer_design ON layer (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_piece_design ON piece (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_group_design ON "group" (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_connection_design ON connection (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_stat_design ON stat (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_kit ON author (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_type ON author (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_design ON author (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_kit ON concept (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_type ON concept (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_design ON concept (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_kit ON tag (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_type ON tag (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_design ON tag (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_representation ON tag (representation_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_kit ON quality (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_type ON quality (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_design ON quality (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_port ON quality (port_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_connector ON quality (connector_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_representation ON quality (representation_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_benchmark_quality ON benchmark (quality_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_kit ON prop (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_type ON prop (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_design ON prop (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_piece ON prop (piece_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_kit ON attribute (kit_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_type ON attribute (type_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_design ON attribute (design_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_piece ON attribute (piece_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_port ON attribute (port_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_connector ON attribute (connector_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_representation ON attribute (representation_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_connection ON attribute (connection_id, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_family ON attribute (family_id, ordinal);
