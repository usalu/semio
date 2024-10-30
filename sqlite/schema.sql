CREATE TABLE semio (
	release VARCHAR NOT NULL, 
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
	"lastUpdateAt" DATETIME NOT NULL, 
	"createdAt" DATETIME NOT NULL, 
	homepage VARCHAR(1024) NOT NULL, 
	remote VARCHAR(1024) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	name VARCHAR(64) NOT NULL, 
	uri VARCHAR(4096) NOT NULL, 
	id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	UNIQUE (uri)
);
CREATE TABLE type (
	unit VARCHAR(64) NOT NULL, 
	variant VARCHAR(64) NOT NULL, 
	icon VARCHAR(1024) NOT NULL, 
	description VARCHAR(4096) NOT NULL, 
	name VARCHAR(64) NOT NULL, 
	id INTEGER NOT NULL, 
	"kitId" INTEGER, 
	PRIMARY KEY (id), 
	CONSTRAINT "Unique name and variant" UNIQUE (name, variant, "kitId"), 
	FOREIGN KEY("kitId") REFERENCES kit (id)
);
CREATE TABLE representation (
	url VARCHAR(1024) NOT NULL, 
	lod VARCHAR(64) NOT NULL, 
	mime VARCHAR(64) NOT NULL, 
	id INTEGER NOT NULL, 
	"encodedTags" VARCHAR(1039) NOT NULL, 
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
	CONSTRAINT "Unique localId" UNIQUE ("localId", "typeId"), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE quality (
	unit VARCHAR(64) NOT NULL, 
	definition VARCHAR(4096) NOT NULL, 
	value VARCHAR(64) NOT NULL, 
	name VARCHAR(64) NOT NULL, 
	id INTEGER NOT NULL, 
	"typeId" INTEGER, 
	PRIMARY KEY (id), 
	FOREIGN KEY("typeId") REFERENCES type (id)
);
CREATE TABLE locator (
	subgroup VARCHAR(64) NOT NULL, 
	"groupName" VARCHAR(64) NOT NULL, 
	"portId" INTEGER NOT NULL, 
	PRIMARY KEY ("groupName", "portId"), 
	FOREIGN KEY("portId") REFERENCES port (id)
);
