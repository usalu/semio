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
	guid TEXT NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	version TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	uri TEXT,
	created_at TEXT,
	updated_at TEXT,
	PRIMARY KEY (guid)
);

CREATE TABLE IF NOT EXISTS folder (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	path TEXT NOT NULL,
	description TEXT,
	kit_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	url TEXT NOT NULL,
	mime TEXT,
	size INTEGER,
	hash TEXT,
	description TEXT,
	created_at TEXT,
	updated_at TEXT,
	kit_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS type (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	variant TEXT,
	stock INTEGER,
	virtual INTEGER,
	unit TEXT,
	location_x REAL,
	location_y REAL,
	created_at TEXT,
	updated_at TEXT,
	kit_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS port (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	id TEXT,
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
	type_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS port_compatible_family (
	port_guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	family TEXT NOT NULL,
	PRIMARY KEY (port_guid, ordinal),
	FOREIGN KEY (port_guid) REFERENCES port (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS connector (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	code TEXT NOT NULL,
	description TEXT,
	port_guid TEXT,
	type_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (port_guid) REFERENCES port (guid),
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS representation (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	url TEXT NOT NULL,
	description TEXT,
	file_guid TEXT,
	type_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (file_guid) REFERENCES file (guid),
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS design (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	icon TEXT,
	image TEXT,
	variant TEXT,
	view TEXT,
	location_x REAL,
	location_y REAL,
	camera_position_x REAL,
	camera_position_y REAL,
	camera_position_z REAL,
	camera_target_x REAL,
	camera_target_y REAL,
	camera_target_z REAL,
	camera_up_x REAL,
	camera_up_y REAL,
	camera_up_z REAL,
	camera_fov REAL,
	unit TEXT,
	created_at TEXT,
	updated_at TEXT,
	kit_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS layer (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	color TEXT,
	order_index INTEGER,
	visible INTEGER,
	locked INTEGER,
	design_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS piece (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	id TEXT,
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
	type_guid TEXT,
	design_ref_guid TEXT,
	design_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (type_guid) REFERENCES type (guid),
	FOREIGN KEY (design_ref_guid) REFERENCES design (guid),
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "group" (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	color TEXT,
	icon TEXT,
	design_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS group_piece (
	group_guid TEXT NOT NULL,
	piece_guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	PRIMARY KEY (group_guid, piece_guid),
	FOREIGN KEY (group_guid) REFERENCES "group" (guid) ON DELETE CASCADE,
	FOREIGN KEY (piece_guid) REFERENCES piece (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS connection (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	connected_side_guid TEXT NOT NULL,
	connected_piece_guid TEXT NOT NULL,
	connected_port_guid TEXT,
	connected_design_piece_guid TEXT,
	connecting_side_guid TEXT NOT NULL,
	connecting_piece_guid TEXT NOT NULL,
	connecting_port_guid TEXT,
	connecting_design_piece_guid TEXT,
	gap REAL,
	shift REAL,
	rise REAL,
	rotation REAL,
	turn REAL,
	tilt REAL,
	x REAL,
	y REAL,
	description TEXT,
	design_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (connected_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY (connected_port_guid) REFERENCES port (guid),
	FOREIGN KEY (connected_design_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY (connecting_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY (connecting_port_guid) REFERENCES port (guid),
	FOREIGN KEY (connecting_design_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS stat (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	unit TEXT,
	description TEXT,
	design_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS author (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	email TEXT NOT NULL,
	role TEXT,
	rank INTEGER,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS concept (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	description TEXT,
	order_index INTEGER,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS tag (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	order_index INTEGER,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	representation_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	FOREIGN KEY (representation_guid) REFERENCES representation (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS quality (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT,
	unit TEXT,
	definition TEXT,
	description TEXT,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	port_guid TEXT,
	connector_guid TEXT,
	representation_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	FOREIGN KEY (port_guid) REFERENCES port (guid) ON DELETE CASCADE,
	FOREIGN KEY (connector_guid) REFERENCES connector (guid) ON DELETE CASCADE,
	FOREIGN KEY (representation_guid) REFERENCES representation (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN port_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connector_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS benchmark (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	name TEXT NOT NULL,
	min_value REAL,
	max_value REAL,
	min_excluded INTEGER,
	max_excluded INTEGER,
	quality_guid TEXT NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY (quality_guid) REFERENCES quality (guid) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS prop (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	unit TEXT,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	piece_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	FOREIGN KEY (piece_guid) REFERENCES piece (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN piece_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE TABLE IF NOT EXISTS attribute (
	guid TEXT NOT NULL,
	ordinal INTEGER NOT NULL,
	key TEXT NOT NULL,
	value TEXT NOT NULL,
	definition TEXT,
	kit_guid TEXT,
	type_guid TEXT,
	design_guid TEXT,
	piece_guid TEXT,
	port_guid TEXT,
	connector_guid TEXT,
	representation_guid TEXT,
	connection_guid TEXT,
	PRIMARY KEY (guid),
	FOREIGN KEY (kit_guid) REFERENCES kit (guid) ON DELETE CASCADE,
	FOREIGN KEY (type_guid) REFERENCES type (guid) ON DELETE CASCADE,
	FOREIGN KEY (design_guid) REFERENCES design (guid) ON DELETE CASCADE,
	FOREIGN KEY (piece_guid) REFERENCES piece (guid) ON DELETE CASCADE,
	FOREIGN KEY (port_guid) REFERENCES port (guid) ON DELETE CASCADE,
	FOREIGN KEY (connector_guid) REFERENCES connector (guid) ON DELETE CASCADE,
	FOREIGN KEY (representation_guid) REFERENCES representation (guid) ON DELETE CASCADE,
	FOREIGN KEY (connection_guid) REFERENCES connection (guid) ON DELETE CASCADE,
	CHECK (
		(CASE WHEN kit_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN type_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN design_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN piece_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN port_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connector_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN representation_guid IS NOT NULL THEN 1 ELSE 0 END) +
		(CASE WHEN connection_guid IS NOT NULL THEN 1 ELSE 0 END) = 1
	)
);

CREATE INDEX IF NOT EXISTS idx_folder_kit ON folder (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_file_kit ON file (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_type_kit ON type (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_port_type ON port (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_connector_type ON connector (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_representation_type ON representation (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_design_kit ON design (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_layer_design ON layer (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_piece_design ON piece (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_group_design ON "group" (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_connection_design ON connection (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_stat_design ON stat (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_kit ON author (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_type ON author (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_author_design ON author (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_kit ON concept (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_type ON concept (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_concept_design ON concept (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_kit ON tag (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_type ON tag (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_design ON tag (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_tag_representation ON tag (representation_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_kit ON quality (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_type ON quality (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_design ON quality (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_port ON quality (port_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_connector ON quality (connector_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_quality_representation ON quality (representation_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_benchmark_quality ON benchmark (quality_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_kit ON prop (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_type ON prop (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_design ON prop (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_prop_piece ON prop (piece_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_kit ON attribute (kit_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_type ON attribute (type_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_design ON attribute (design_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_piece ON attribute (piece_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_port ON attribute (port_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_connector ON attribute (connector_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_representation ON attribute (representation_guid, ordinal);
CREATE INDEX IF NOT EXISTS idx_attribute_connection ON attribute (connection_guid, ordinal);
