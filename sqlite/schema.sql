CREATE TABLE kit (
	name VARCHAR NOT NULL,
	description VARCHAR NOT NULL,
	icon VARCHAR NOT NULL,
	"createdAt" DATETIME NOT NULL,
	"lastUpdateAt" DATETIME NOT NULL,
	url VARCHAR(1000) NOT NULL,
	homepage VARCHAR(1000) NOT NULL,
	id INTEGER NOT NULL,
	PRIMARY KEY (id),
	UNIQUE (name),
	UNIQUE (url)
);
CREATE TABLE type (
	name VARCHAR NOT NULL,
	description VARCHAR NOT NULL,
	icon VARCHAR NOT NULL,
	"createdAt" DATETIME NOT NULL,
	"lastUpdateAt" DATETIME NOT NULL,
	variant VARCHAR NOT NULL,
	id INTEGER NOT NULL,
	"kitPk" INTEGER,
	PRIMARY KEY (id),
	FOREIGN KEY("kitPk") REFERENCES kit (id)
);
CREATE TABLE representation (
	url VARCHAR NOT NULL,
	lod VARCHAR NOT NULL,
	id INTEGER NOT NULL,
	"localId" VARCHAR(100),
	"typePk" INTEGER,
	PRIMARY KEY (id),
	UNIQUE (url),
	FOREIGN KEY("typePk") REFERENCES type (id)
);
CREATE TABLE tag (
	value VARCHAR NOT NULL,
	"representationPk" INTEGER NOT NULL,
	PRIMARY KEY (value, "representationPk"),
	FOREIGN KEY("representationPk") REFERENCES representation (id)
);
CREATE TABLE port (
	id INTEGER NOT NULL,
	"localId" VARCHAR(512),
	"pointX" FLOAT NOT NULL,
	"pointY" FLOAT NOT NULL,
	"pointZ" FLOAT NOT NULL,
	"directionX" FLOAT NOT NULL,
	"directionY" FLOAT NOT NULL,
	"directionZ" FLOAT NOT NULL,
	"typeId" VARCHAR(512),
	PRIMARY KEY (id),
	UNIQUE ("localId", "typeId"),
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE locator (
	subgroup VARCHAR(512) NOT NULL,
	"groupName" VARCHAR(512) NOT NULL,
	"portId" VARCHAR(512) NOT NULL,
	PRIMARY KEY ("groupName", "portId"),
	FOREIGN KEY("portId") REFERENCES port (id)
);
CREATE TABLE design (
	name VARCHAR(512) NOT NULL,
	description VARCHAR(4096) NOT NULL,
	icon VARCHAR(1024) NOT NULL,
	"createdAt" DATETIME NOT NULL,
	"lastUpdateAt" DATETIME NOT NULL,
	variant VARCHAR(512) NOT NULL,
	id INTEGER NOT NULL,
	"kitId" INTEGER,
	PRIMARY KEY (id),
	FOREIGN KEY("kitId") REFERENCES kit (id)
);
CREATE TABLE quality (
	name VARCHAR(512) NOT NULL,
	value VARCHAR(512) NOT NULL,
	definition VARCHAR(4096) NOT NULL,
	unit VARCHAR(512) NOT NULL,
	id INTEGER NOT NULL,
	"typeId" INTEGER,
	"designId" INTEGER,
	PRIMARY KEY (id),
	CONSTRAINT "typeOrDesignSet" CHECK (
		typeId IS NOT NULL
		AND designId IS NULL
		OR typeId IS NULL
		AND designId IS NOT NULL
	),
	UNIQUE (name, "typeId", "designId"),
	FOREIGN KEY("typeId") REFERENCES type (id),
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE piece (
	id INTEGER NOT NULL,
	"localId" VARCHAR(512),
	"planeOriginX" FLOAT NOT NULL,
	"planeOriginY" FLOAT NOT NULL,
	"planeOriginZ" FLOAT NOT NULL,
	"planeXAxisX" FLOAT NOT NULL,
	"planeXAxisY" FLOAT NOT NULL,
	"planeXAxisZ" FLOAT NOT NULL,
	"planeYAxisX" FLOAT NOT NULL,
	"planeYAxisY" FLOAT NOT NULL,
	"planeYAxisZ" FLOAT NOT NULL,
	"diagramPointX" FLOAT NOT NULL,
	"diagramPointY" FLOAT NOT NULL,
	"designId" INTEGER,
	PRIMARY KEY (id),
	UNIQUE ("localId", "designId"),
	CONSTRAINT "planeSetOrNotSet" CHECK (
		(
			(
				planeOriginX IS NULL
				AND planeOriginY IS NULL
				AND planeOriginZ IS NULL
				AND planeXAxisX IS NULL
				AND planeXAxisY IS NULL
				AND planeXAxisZ IS NULL
				AND planeYAxisX IS NULL
				AND planeYAxisY IS NULL
				AND planeYAxisZ IS NULL
			)
			OR (
				planeOriginX IS NOT NULL
				AND planeOriginY IS NOT NULL
				AND planeOriginZ IS NOT NULL
				AND planeXAxisX IS NOT NULL
				AND planeXAxisY IS NOT NULL
				AND planeXAxisZ IS NOT NULL
				AND planeYAxisX IS NOT NULL
				AND planeYAxisY IS NOT NULL
				AND planeYAxisZ IS NOT NULL
			)
		)
	),
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE connection (
	rotation FLOAT NOT NULL,
	"offset" FLOAT NOT NULL,
	"connectedPieceId" INTEGER NOT NULL,
	"connectedPieceTypePortId" INTEGER NOT NULL,
	"connectingPieceId" INTEGER NOT NULL,
	"connectingPieceTypePortId" INTEGER NOT NULL,
	"designId" INTEGER NOT NULL,
	PRIMARY KEY (
		"connectedPieceId",
		"connectedPieceTypePortId",
		"connectingPieceId",
		"connectingPieceTypePortId",
		"designId"
	),
	CONSTRAINT "noReflexiveConnection" CHECK (connectingPieceId != connectedPieceId),
	FOREIGN KEY("connectedPieceId") REFERENCES piece (id),
	FOREIGN KEY("connectedPieceTypePortId") REFERENCES port (id),
	FOREIGN KEY("connectingPieceId") REFERENCES piece (id),
	FOREIGN KEY("connectingPieceTypePortId") REFERENCES port (id),
	FOREIGN KEY("designId") REFERENCES design (id)
);