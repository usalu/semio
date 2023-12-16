CREATE TABLE kit (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	PRIMARY KEY (id)
);
CREATE TABLE script (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	uri VARCHAR(1000) NOT NULL, 
	kind VARCHAR(14) NOT NULL, 
	kit_id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
CREATE TABLE type (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	kit_id INTEGER NOT NULL, 
	prototype_script_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id), 
	FOREIGN KEY(prototype_script_id) REFERENCES script (id)
);
CREATE TABLE formation (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	choreography_script_id INTEGER, 
	transformation_script_id INTEGER, 
	kit_id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(choreography_script_id) REFERENCES script (id), 
	FOREIGN KEY(transformation_script_id) REFERENCES script (id), 
	FOREIGN KEY(kit_id) REFERENCES kit (id)
);
CREATE TABLE port (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	origin_x NUMERIC, 
	origin_y NUMERIC, 
	origin_z NUMERIC, 
	x_axis_x NUMERIC, 
	x_axis_y NUMERIC, 
	x_axis_z NUMERIC, 
	y_axis_x NUMERIC, 
	y_axis_y NUMERIC, 
	y_axis_z NUMERIC, 
	z_axis_x NUMERIC, 
	z_axis_y NUMERIC, 
	z_axis_z NUMERIC, 
	type_id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_id) REFERENCES type (id)
);
CREATE TABLE piece (
	id INTEGER NOT NULL, 
	formation_id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(formation_id) REFERENCES formation (id)
);
CREATE TABLE property (
	id INTEGER NOT NULL, 
	name VARCHAR(100) NOT NULL, 
	explanation TEXT, 
	datatype VARCHAR(100) NOT NULL, 
	value TEXT NOT NULL, 
	synthesis_script_id INTEGER, 
	type_id INTEGER, 
	port_id INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY(synthesis_script_id) REFERENCES script (id), 
	FOREIGN KEY(type_id) REFERENCES type (id), 
	FOREIGN KEY(port_id) REFERENCES port (id)
);
CREATE TABLE attraction (
	attracting_piece_id INTEGER NOT NULL, 
	attracting_piece_type_port_id INTEGER NOT NULL, 
	attracted_piece_id INTEGER NOT NULL, 
	attracted_piece_type_port_id INTEGER NOT NULL, 
	formation_id INTEGER NOT NULL, 
	PRIMARY KEY (attracting_piece_id, attracting_piece_type_port_id, attracted_piece_id, attracted_piece_type_port_id, formation_id), 
	FOREIGN KEY(attracting_piece_id) REFERENCES piece (id), 
	FOREIGN KEY(attracting_piece_type_port_id) REFERENCES port (id), 
	FOREIGN KEY(attracted_piece_id) REFERENCES piece (id), 
	FOREIGN KEY(attracted_piece_type_port_id) REFERENCES port (id), 
	FOREIGN KEY(formation_id) REFERENCES formation (id)
);
