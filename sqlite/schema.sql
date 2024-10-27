CREATE TABLE plane (
	id INTEGER NOT NULL, 
	"planeOriginX" FLOAT NOT NULL, 
	"planeOriginY" FLOAT NOT NULL, 
	"planeOriginZ" FLOAT NOT NULL, 
	"xAxisX" FLOAT NOT NULL, 
	"xAxisY" FLOAT NOT NULL, 
	"xAxisZ" FLOAT NOT NULL, 
	"yAxisX" FLOAT NOT NULL, 
	"yAxisY" FLOAT NOT NULL, 
	"yAxisZ" FLOAT NOT NULL, 
	PRIMARY KEY (id)
);
CREATE TABLE kit (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	remote VARCHAR(1024) NOT NULL, 
	homepage VARCHAR(1024) NOT NULL, 
	id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	UNIQUE (name)
);
CREATE TABLE type (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	id INTEGER NOT NULL, 
	"kitId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("kitId") REFERENCES kit (id)
);
CREATE TABLE representation (
	url VARCHAR(1024) NOT NULL, 
	lod VARCHAR NOT NULL, 
	id INTEGER NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE port (
	id INTEGER NOT NULL, 
	"localId" VARCHAR(64), 
	"pointX" FLOAT NOT NULL, 
	"pointY" FLOAT NOT NULL, 
	"pointZ" FLOAT NOT NULL, 
	"directionX" FLOAT NOT NULL, 
	"directionY" FLOAT NOT NULL, 
	"directionZ" FLOAT NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE ("localId", "typeId"), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE quality (
	name VARCHAR(64) NOT NULL, 
	value VARCHAR(64) NOT NULL, 
	definition VARCHAR(4096) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	id INTEGER NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE tag (
	value VARCHAR NOT NULL, 
	"representationId" INTEGER NOT NULL, 
	PRIMARY KEY (value, "representationId"), 
	FOREIGN KEY("representationId") REFERENCES representation (id)
);
CREATE TABLE locator (
	subgroup VARCHAR(64) NOT NULL, 
	"groupName" VARCHAR(64) NOT NULL, 
	"portId" INTEGER NOT NULL, 
	PRIMARY KEY ("groupName", "portId"), 
	FOREIGN KEY("portId") REFERENCES port (id)
);
