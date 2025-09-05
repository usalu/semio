CREATE TABLE semio (
	release VARCHAR NOT NULL, 
	engine VARCHAR NOT NULL, 
	created DATETIME NOT NULL, 
	PRIMARY KEY (release)
);
CREATE TABLE plane (
	id INTEGER NOT NULL, 
	origin_x FLOAT, 
	origin_y FLOAT, 
	origin_z FLOAT, 
	x_axis_x FLOAT, 
	x_axis_y FLOAT, 
	x_axis_z FLOAT, 
	y_axis_x FLOAT, 
	y_axis_y FLOAT, 
	y_axis_z FLOAT, 
	PRIMARY KEY (id)
);
CREATE TABLE kit (
	uri VARCHAR(2048) NOT NULL, 
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(512) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	image VARCHAR(1024) NOT NULL, 
	preview VARCHAR(1024) NOT NULL, 
	version VARCHAR(64) NOT NULL, 
	remote VARCHAR(1024) NOT NULL, 
	homepage VARCHAR(1024) NOT NULL, 
	license VARCHAR(1024) NOT NULL, 
	created DATETIME NOT NULL, 
	updated DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	UNIQUE (uri)
);
CREATE TABLE type (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(512) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	image VARCHAR(1024) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	stock INTEGER NOT NULL, 
	"virtual" BOOLEAN NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	created DATETIME NOT NULL, 
	updated DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	location_longitude FLOAT, 
	location_latitude FLOAT, 
	kit_id INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
CREATE TABLE design (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(512) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	image VARCHAR(1024) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	"view" VARCHAR(64) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	created DATETIME NOT NULL, 
	updated DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	location_longitude FLOAT, 
	location_latitude FLOAT, 
	kit_id INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE (name, variant, "view", kit_id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
CREATE TABLE representation (
	url VARCHAR(1024) NOT NULL, 
	description VARCHAR(512) NOT NULL, 
	id INTEGER NOT NULL, 
	type_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_id) REFERENCES type (id)
);
CREATE TABLE port (
	description VARCHAR(512) NOT NULL, 
	mandatory BOOLEAN NOT NULL, 
	family VARCHAR(64) NOT NULL, 
	t FLOAT NOT NULL, 
	id INTEGER NOT NULL, 
	local_id VARCHAR(128), 
	point_x VARCHAR(128), 
	point_y FLOAT, 
	point_z FLOAT, 
	direction_x FLOAT, 
	direction_y FLOAT, 
	direction_z FLOAT, 
	type_id INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "Unique local_id" UNIQUE (local_id, type_id), 
	FOREIGN KEY(type_id) REFERENCES type (id)
);
CREATE TABLE author (
	name VARCHAR(64) NOT NULL, 
	email VARCHAR(128) NOT NULL, 
	rank INTEGER NOT NULL, 
	id INTEGER NOT NULL, 
	type_id INTEGER, 
	design_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_id) REFERENCES type (id), 
	FOREIGN KEY(design_id) REFERENCES design (id)
);
CREATE TABLE piece (
	description VARCHAR(512) NOT NULL, 
	id INTEGER NOT NULL, 
	local_id VARCHAR(128), 
	type_id INTEGER, 
	plane_id INTEGER, 
	center_x FLOAT, 
	center_y FLOAT, 
	design_id INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE (local_id, design_id), 
	FOREIGN KEY(type_id) REFERENCES type (id), 
	FOREIGN KEY(plane_id) REFERENCES plane (id), 
	FOREIGN KEY(design_id) REFERENCES design (id)
);
CREATE TABLE tag (
	name VARCHAR(64) NOT NULL, 
	"order" INTEGER NOT NULL, 
	id INTEGER NOT NULL, 
	representation_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(representation_id) REFERENCES representation (id)
);
CREATE TABLE compatible_family (
	name VARCHAR(64) NOT NULL, 
	"order" INTEGER NOT NULL, 
	id INTEGER NOT NULL, 
	port_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(port_id) REFERENCES port (id)
);
CREATE TABLE connection (
	description VARCHAR(512) NOT NULL, 
	gap FLOAT NOT NULL, 
	shift FLOAT NOT NULL, 
	rise FLOAT NOT NULL, 
	rotation FLOAT NOT NULL, 
	turn FLOAT NOT NULL, 
	tilt FLOAT NOT NULL, 
	x FLOAT NOT NULL, 
	y FLOAT NOT NULL, 
	id INTEGER NOT NULL, 
	connected_piece_id INTEGER, 
	connected_port_id INTEGER, 
	connecting_piece_id INTEGER, 
	connecting_port_id INTEGER, 
	design_id INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "no reflexive connection" CHECK (connecting_piece_id != connected_piece_id), 
	FOREIGN KEY(connected_piece_id) REFERENCES piece (id), 
	FOREIGN KEY(connected_port_id) REFERENCES port (id), 
	FOREIGN KEY(connecting_piece_id) REFERENCES piece (id), 
	FOREIGN KEY(connecting_port_id) REFERENCES port (id), 
	FOREIGN KEY(design_id) REFERENCES design (id)
);
CREATE TABLE concept (
	name VARCHAR(64) NOT NULL, 
	"order" INTEGER NOT NULL, 
	id INTEGER NOT NULL, 
	kit_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
CREATE TABLE attribute (
	name VARCHAR(64) NOT NULL, 
	value VARCHAR(64) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	definition VARCHAR(512) NOT NULL, 
	id INTEGER NOT NULL, 
	representation_id INTEGER, 
	port_id INTEGER, 
	type_id INTEGER, 
	piece_id INTEGER, 
	connection_id INTEGER, 
	design_id INTEGER, 
	kit_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(representation_id) REFERENCES representation (id), 
	FOREIGN KEY(port_id) REFERENCES port (id), 
	FOREIGN KEY(type_id) REFERENCES type (id), 
	FOREIGN KEY(piece_id) REFERENCES piece (id), 
	FOREIGN KEY(connection_id) REFERENCES connection (id), 
	FOREIGN KEY(design_id) REFERENCES design (id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
