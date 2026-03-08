-- semio Kit Database Schema (GUID-based)
-- This schema supports the complete semio data model with GUIDs as primary identifiers

CREATE TABLE semio (
	release VARCHAR NOT NULL,
	engine VARCHAR NOT NULL,
	created DATETIME NOT NULL,
	PRIMARY KEY (release)
);

CREATE TABLE kit (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	version VARCHAR(64),
	description TEXT,
	icon TEXT,
	image TEXT,
	preview TEXT,
	remote TEXT,
	homepage TEXT,
	license TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	PRIMARY KEY (guid)
);

CREATE TABLE quality (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	name VARCHAR(256) NOT NULL,
	kind INTEGER NOT NULL,
	default_value FLOAT,
	formula TEXT,
	default_si_unit VARCHAR(64),
	default_imperial_unit VARCHAR(64),
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	can_scale BOOLEAN NOT NULL DEFAULT 0,
	definition TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE benchmark (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	icon TEXT,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	definition TEXT,
	quality_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE port (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE port_compatibility (
	port_guid VARCHAR(36) NOT NULL,
	compatible_port_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (port_guid, compatible_port_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(compatible_port_guid) REFERENCES port (guid)
);



CREATE TABLE author (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	email VARCHAR(256),
	kit_guid VARCHAR(36),
	type_guid VARCHAR(36),
	design_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE folder (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(parent_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE file (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	mime VARCHAR(256),
	folder_guid VARCHAR(36),
	size INTEGER,
	hash VARCHAR(256),
	remote_url TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(folder_guid) REFERENCES folder (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type (
	guid VARCHAR(36) NOT NULL UNIQUE,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	is_abstract BOOLEAN NOT NULL DEFAULT 0,
	folder VARCHAR(256),
	stock INTEGER,
	virtual BOOLEAN NOT NULL DEFAULT 0,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES type (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE tag (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE model (
	guid VARCHAR(36) NOT NULL,
	file_guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE model_tag (
	model_guid VARCHAR(36) NOT NULL,
	tag_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (model_guid, tag_guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(tag_guid) REFERENCES tag (guid)
);

CREATE TABLE prop (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(128) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	quality_guid VARCHAR(36),
	connector_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE connector (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	point_x FLOAT NOT NULL,
	point_y FLOAT NOT NULL,
	point_z FLOAT NOT NULL,
	direction_x FLOAT NOT NULL,
	direction_y FLOAT NOT NULL,
	direction_z FLOAT NOT NULL,
	t FLOAT NOT NULL,
	mandatory BOOLEAN NOT NULL DEFAULT 0,
	port_guid VARCHAR(36),
	description TEXT,
	type_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, type_guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid)
);

CREATE TABLE design (
	guid VARCHAR(36) NOT NULL UNIQUE,
	name VARCHAR(256) NOT NULL,
	parent_guid VARCHAR(36),
	variant VARCHAR(256),
	view_center_u FLOAT,
	view_center_v FLOAT,
	view_zoom FLOAT,
	unit VARCHAR(64),
	location_guid VARCHAR(36),
	active_layer_guid VARCHAR(36),
	is_abstract BOOLEAN,
	folder VARCHAR(256),
	can_scale BOOLEAN,
	can_mirror BOOLEAN,
	description TEXT,
	icon TEXT,
	image TEXT,
	created DATETIME NOT NULL,
	updated DATETIME NOT NULL,
	kit_guid VARCHAR(36) NOT NULL,
	row_id INTEGER PRIMARY KEY AUTOINCREMENT,
	UNIQUE (guid, kit_guid, parent_guid),
	FOREIGN KEY(parent_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE design_prop (
	design_guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	value FLOAT NOT NULL,
	unit VARCHAR(64),
	PRIMARY KEY (design_guid, quality_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid)
);

CREATE TABLE design_author (
	design_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (design_guid, author_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);

CREATE TABLE layer (
	guid VARCHAR(36) NOT NULL,
	path VARCHAR(512) NOT NULL,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	type_guid VARCHAR(36),
	design_guid_ref VARCHAR(36),
	plane_origin_x FLOAT,
	plane_origin_y FLOAT,
	plane_origin_z FLOAT,
	plane_x_axis_x FLOAT,
	plane_x_axis_y FLOAT,
	plane_x_axis_z FLOAT,
	plane_y_axis_x FLOAT,
	plane_y_axis_y FLOAT,
	plane_y_axis_z FLOAT,
	center_u FLOAT,
	center_v FLOAT,
	scale FLOAT,
	mirror_plane_origin_x FLOAT,
	mirror_plane_origin_y FLOAT,
	mirror_plane_origin_z FLOAT,
	mirror_plane_x_axis_x FLOAT,
	mirror_plane_x_axis_y FLOAT,
	mirror_plane_x_axis_z FLOAT,
	mirror_plane_y_axis_x FLOAT,
	mirror_plane_y_axis_y FLOAT,
	mirror_plane_y_axis_z FLOAT,
	is_hidden BOOLEAN NOT NULL DEFAULT 0,
	is_locked BOOLEAN NOT NULL DEFAULT 0,
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(design_guid_ref) REFERENCES design (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE piece_prop (
	piece_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (piece_guid, prop_guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
);

CREATE TABLE "group" (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256),
	color VARCHAR(32),
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE group_piece (
	group_guid VARCHAR(36) NOT NULL,
	piece_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (group_guid, piece_guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid)
);

CREATE TABLE connection (
	guid VARCHAR(36) NOT NULL,
	connected_piece_guid VARCHAR(36) NOT NULL,
	connected_design_piece_guid VARCHAR(36),
	connected_connector_guid VARCHAR(36) NOT NULL,
	connecting_piece_guid VARCHAR(36) NOT NULL,
	connecting_design_piece_guid VARCHAR(36),
	connecting_connector_guid VARCHAR(36) NOT NULL,
	gap FLOAT NOT NULL DEFAULT 0,
	shift FLOAT NOT NULL DEFAULT 0,
	rise FLOAT NOT NULL DEFAULT 0,
	rotation FLOAT NOT NULL DEFAULT 0,
	turn FLOAT NOT NULL DEFAULT 0,
	tilt FLOAT NOT NULL DEFAULT 0,
	u FLOAT,
	v FLOAT,
	description TEXT,
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	CHECK (connecting_piece_guid != connected_piece_guid),
	FOREIGN KEY(connected_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connected_connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(connecting_piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(connecting_connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE stat (
	guid VARCHAR(36) NOT NULL,
	quality_guid VARCHAR(36) NOT NULL,
	min_value FLOAT,
	min_excluded BOOLEAN,
	max_value FLOAT,
	max_excluded BOOLEAN,
	unit VARCHAR(64),
	design_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid)
);

CREATE TABLE concept (
	guid VARCHAR(36) NOT NULL,
	name VARCHAR(256) NOT NULL,
	description TEXT,
	icon TEXT,
	kit_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);

CREATE TABLE type_concept (
	type_guid VARCHAR(36) NOT NULL,
	concept_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (type_guid, concept_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(concept_guid) REFERENCES concept (guid)
);

CREATE TABLE type_author (
	type_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL,
	PRIMARY KEY (type_guid, author_guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);

CREATE TABLE design_concept (
	design_guid VARCHAR(36) NOT NULL,
	concept_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (design_guid, concept_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(concept_guid) REFERENCES concept (guid)
);

CREATE TABLE attribute (
	guid VARCHAR(36) NOT NULL,
	key VARCHAR(256) NOT NULL,
	value TEXT,
	definition TEXT,
	quality_guid VARCHAR(36),
	benchmark_guid VARCHAR(36),
	port_guid VARCHAR(36),
	tag_guid VARCHAR(36),
	concept_guid VARCHAR(36),
	folder_guid VARCHAR(36),
	file_guid VARCHAR(36),
	author_guid VARCHAR(36),
	model_guid VARCHAR(36),
	prop_guid VARCHAR(36),
	connector_guid VARCHAR(36),
	type_guid VARCHAR(36),
	layer_guid VARCHAR(36),
	piece_guid VARCHAR(36),
	group_guid VARCHAR(36),
	connection_guid VARCHAR(36),
	stat_guid VARCHAR(36),
	design_guid VARCHAR(36),
	kit_guid VARCHAR(36),
	PRIMARY KEY (guid),
	FOREIGN KEY(quality_guid) REFERENCES quality (guid),
	FOREIGN KEY(benchmark_guid) REFERENCES benchmark (guid),
	FOREIGN KEY(port_guid) REFERENCES port (guid),
	FOREIGN KEY(tag_guid) REFERENCES tag (guid),
	FOREIGN KEY(concept_guid) REFERENCES concept (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid),
	FOREIGN KEY(model_guid) REFERENCES model (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid),
	FOREIGN KEY(connector_guid) REFERENCES connector (guid),
	FOREIGN KEY(type_guid) REFERENCES type (guid),
	FOREIGN KEY(layer_guid) REFERENCES layer (guid),
	FOREIGN KEY(piece_guid) REFERENCES piece (guid),
	FOREIGN KEY(group_guid) REFERENCES "group" (guid),
	FOREIGN KEY(connection_guid) REFERENCES connection (guid),
	FOREIGN KEY(stat_guid) REFERENCES stat (guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(kit_guid) REFERENCES kit (guid)
);
