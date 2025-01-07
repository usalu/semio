CREATE TABLE semio (
	release VARCHAR NOT NULL, 
	engine VARCHAR NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	PRIMARY KEY (release)
);
CREATE TABLE plane (
	id INTEGER NOT NULL, 
	"originX" FLOAT NOT NULL, 
	"originY" FLOAT NOT NULL, 
	"originZ" FLOAT NOT NULL, 
	"xAxisX" FLOAT NOT NULL, 
	"xAxisY" FLOAT NOT NULL, 
	"xAxisZ" FLOAT NOT NULL, 
	"yAxisX" FLOAT NOT NULL, 
	"yAxisY" FLOAT NOT NULL, 
	"yAxisZ" FLOAT NOT NULL, 
	PRIMARY KEY (id), 
	CONSTRAINT "planeSetOrNotSet" CHECK (
            (
                (originX IS NULL AND originY IS NULL AND originZ IS NULL AND
                 xAxisX IS NULL AND xAxisY IS NULL AND xAxisZ IS NULL AND
                 yAxisX IS NULL AND yAxisY IS NULL AND yAxisZ IS NULL)
            OR
                (originX IS NOT NULL AND originY IS NOT NULL AND originZ IS NOT NULL AND
                 xAxisX IS NOT NULL AND xAxisY IS NOT NULL AND xAxisZ IS NOT NULL AND
                 yAxisX IS NOT NULL AND yAxisY IS NOT NULL AND yAxisZ IS NOT NULL)
            )
            )
);
CREATE TABLE kit (
	uri VARCHAR(4096) NOT NULL, 
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	icon VARCHAR(2048) NOT NULL, 
	image VARCHAR(2048) NOT NULL, 
	preview VARCHAR(2048) NOT NULL, 
	version VARCHAR(64) NOT NULL, 
	remote VARCHAR(2048) NOT NULL, 
	homepage VARCHAR(2048) NOT NULL, 
	license VARCHAR(2048) NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	"lastUpdateAt" DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	UNIQUE (uri)
);
CREATE TABLE type (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	icon VARCHAR(2048) NOT NULL, 
	image VARCHAR(2048) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	"lastUpdateAt" DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	"kitId" INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "Unique name and variant" UNIQUE (name, variant, "kitId"), 
	FOREIGN KEY("kitId") REFERENCES kit (id)
);
CREATE TABLE design (
	name VARCHAR(64) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	icon VARCHAR(2048) NOT NULL, 
	image VARCHAR(2048) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	"view" VARCHAR(64) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	"lastUpdateAt" DATETIME NOT NULL, 
	id INTEGER NOT NULL, 
	"kitId" INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE (name, variant, "kitId"), 
	FOREIGN KEY("kitId") REFERENCES kit (id)
);
CREATE TABLE representation (
	mime VARCHAR(64) NOT NULL, 
	lod VARCHAR(64) NOT NULL, 
	url VARCHAR(2048) NOT NULL, 
	id INTEGER NOT NULL, 
	"encodedTags" VARCHAR(1039) NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE port (
	description VARCHAR(4096) NOT NULL, 
	id INTEGER NOT NULL, 
	"localId" VARCHAR(128), 
	"pointX" FLOAT NOT NULL, 
	"pointY" FLOAT NOT NULL, 
	"pointZ" FLOAT NOT NULL, 
	"directionX" FLOAT NOT NULL, 
	"directionY" FLOAT NOT NULL, 
	"directionZ" FLOAT NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "Unique localId" UNIQUE ("localId", "typeId"), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE quality (
	name VARCHAR(64) NOT NULL, 
	value VARCHAR(64) NOT NULL, 
	unit VARCHAR(64) NOT NULL, 
	definition VARCHAR(4096) NOT NULL, 
	id INTEGER NOT NULL, 
	"typeId" INTEGER, 
	"designId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE author (
	name VARCHAR(64) NOT NULL, 
	email VARCHAR(128) NOT NULL, 
	rank INTEGER NOT NULL, 
	id INTEGER NOT NULL, 
	"typeId" INTEGER, 
	"designId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE piece (
	id INTEGER NOT NULL, 
	"localId" VARCHAR(128), 
	"typeId" INTEGER, 
	"planeId" INTEGER, 
	"centerX" FLOAT, 
	"centerY" FLOAT, 
	"designId" INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE ("localId", "designId"), 
	FOREIGN KEY("typeId") REFERENCES type (id), 
	FOREIGN KEY("planeId") REFERENCES plane (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE locator (
	subgroup VARCHAR(64) NOT NULL, 
	"groupName" VARCHAR(64) NOT NULL, 
	"portId" INTEGER NOT NULL, 
	PRIMARY KEY ("groupName", "portId"), 
	FOREIGN KEY("portId") REFERENCES port (id)
);
CREATE TABLE connection (
	rotation FLOAT NOT NULL, 
	tilt FLOAT NOT NULL, 
	gap FLOAT NOT NULL, 
	shift FLOAT NOT NULL, 
	x FLOAT NOT NULL, 
	y FLOAT NOT NULL, 
	"connectedPieceId" INTEGER NOT NULL, 
	"connectedPortId" INTEGER NOT NULL, 
	"connectingPieceId" INTEGER NOT NULL, 
	"connectingPortId" INTEGER NOT NULL, 
	"designId" INTEGER NOT NULL, 
	PRIMARY KEY ("connectedPieceId", "connectedPortId", "connectingPieceId", "connectingPortId", "designId"), 
	CONSTRAINT "noReflexiveConnection" CHECK (connectingPieceId != connectedPieceId), 
	FOREIGN KEY("connectedPieceId") REFERENCES piece (id), 
	FOREIGN KEY("connectedPortId") REFERENCES port (id), 
	FOREIGN KEY("connectingPieceId") REFERENCES piece (id), 
	FOREIGN KEY("connectingPortId") REFERENCES port (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
