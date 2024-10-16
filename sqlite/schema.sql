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
	CONSTRAINT "typeOrDesignSet" CHECK (typeId IS NOT NULL AND designId IS NULL OR typeId IS NULL AND designId IS NOT NULL), 
	UNIQUE (name, "typeId", "designId"), 
	FOREIGN KEY("typeId") REFERENCES type (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE piece (
	id INTEGER NOT NULL, 
	"localId" VARCHAR(512), 
	"coordinateSystemOriginX" FLOAT NOT NULL, 
	"coordinateSystemOriginY" FLOAT NOT NULL, 
	"coordinateSystemOriginZ" FLOAT NOT NULL, 
	"coordinateSystemXAxisX" FLOAT NOT NULL, 
	"coordinateSystemXAxisY" FLOAT NOT NULL, 
	"coordinateSystemXAxisZ" FLOAT NOT NULL, 
	"coordinateSystemYAxisX" FLOAT NOT NULL, 
	"coordinateSystemYAxisY" FLOAT NOT NULL, 
	"coordinateSystemYAxisZ" FLOAT NOT NULL, 
	"diagramPointX" FLOAT NOT NULL, 
	"diagramPointY" FLOAT NOT NULL, 
	"designId" INTEGER, 
	PRIMARY KEY (id), 
	UNIQUE ("localId", "designId"), 
	CONSTRAINT "coordinateSystemSetOrNotSet" CHECK (
            (
                (coordinateSystemOriginX IS NULL AND coordinateSystemOriginY IS NULL AND coordinateSystemOriginZ IS NULL AND
                 coordinateSystemXAxisX IS NULL AND coordinateSystemXAxisY IS NULL AND coordinateSystemXAxisZ IS NULL AND
                 coordinateSystemYAxisX IS NULL AND coordinateSystemYAxisY IS NULL AND coordinateSystemYAxisZ IS NULL)
            OR
                (coordinateSystemOriginX IS NOT NULL AND coordinateSystemOriginY IS NOT NULL AND coordinateSystemOriginZ IS NOT NULL AND
                 coordinateSystemXAxisX IS NOT NULL AND coordinateSystemXAxisY IS NOT NULL AND coordinateSystemXAxisZ IS NOT NULL AND
                 coordinateSystemYAxisX IS NOT NULL AND coordinateSystemYAxisY IS NOT NULL AND coordinateSystemYAxisZ IS NOT NULL)
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
	PRIMARY KEY ("connectedPieceId", "connectedPieceTypePortId", "connectingPieceId", "connectingPieceTypePortId", "designId"), 
	CONSTRAINT "noReflexiveConnection" CHECK (connectingPieceId != connectedPieceId), 
	FOREIGN KEY("connectedPieceId") REFERENCES piece (id), 
	FOREIGN KEY("connectedPieceTypePortId") REFERENCES port (id), 
	FOREIGN KEY("connectingPieceId") REFERENCES piece (id), 
	FOREIGN KEY("connectingPieceTypePortId") REFERENCES port (id), 
	FOREIGN KEY("designId") REFERENCES design (id)
);
CREATE TABLE semio (
	release VARCHAR(100) NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	PRIMARY KEY (release)
);
