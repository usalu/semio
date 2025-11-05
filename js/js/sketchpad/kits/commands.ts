// #region Header

// commands.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import JSZip from "jszip";
import initSqlJs, { Database, SqlJsStatic } from "sql.js";
import { Attribute, Author, AuthorDiff, Connection, Design, DesignDiff, FileDiff, Guid, Kit, Piece, Quality, QualityDiff, File as SemioFile, Type, TypeDiff, findDesignInKit } from "../../semio";
import type { KitCommandContext, KitCommandResult, Url } from "../store";

const sqlWasmUrl = "https://sql.js.org/dist/sql-wasm.wasm";

export const commands = {
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, guid: Guid, diff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { authors: { removed: [guid] } },
    };
  },
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, guid: Guid, diff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { types: { removed: [guid] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, guid: Guid, diff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { designs: { removed: [guid] } },
    };
  },
  "semio.kit.createQuality": (context: KitCommandContext, quality: Quality): KitCommandResult => {
    return {
      diff: { qualities: { added: [quality] } },
    };
  },
  "semio.kit.updateQuality": (context: KitCommandContext, guid: Guid, diff: QualityDiff): KitCommandResult => {
    return {
      diff: { qualities: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteQuality": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { qualities: { removed: [guid] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], file.name)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, fileGuid: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const existing = context.kit.files?.find((f) => f.guid === fileGuid);
    const fileName = fileDiff.name ?? existing?.name ?? "file";
    const files: File[] = blob ? [new File([blob], fileName)] : [];
    return {
      diff: { files: { updated: [{ id: fileGuid, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, fileGuid: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [fileGuid] } },
    };
  },
  "semio.kit.createFolder": (context: KitCommandContext, folder: import("../../semio").Folder): KitCommandResult => {
    return {
      diff: { folders: { added: [folder] } },
    };
  },
  "semio.kit.updateFolder": (context: KitCommandContext, guid: Guid, diff: import("../../semio").FolderDiff): KitCommandResult => {
    return {
      diff: { folders: { updated: [{ id: guid, diff: diff }] } },
    };
  },
  "semio.kit.deleteFolder": (context: KitCommandContext, guid: Guid): KitCommandResult => {
    return {
      diff: { folders: { removed: [guid] } },
    };
  },
  "semio.kit.moveToFolder": (context: KitCommandContext, artifactGuid: Guid, artifactKind: "type" | "design" | "quality" | "file" | "folder", folderGuid?: Guid): KitCommandResult => {
    switch (artifactKind) {
      case "type": {
        const type = context.kit.types?.find((t) => t.guid === artifactGuid);
        if (!type) throw new Error(`Type ${artifactGuid} not found`);
        if (type.parent) throw new Error("Only prototypes (types without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { types: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "design": {
        const design = context.kit.designs?.find((d) => d.guid === artifactGuid);
        if (!design) throw new Error(`Design ${artifactGuid} not found`);
        if (design.parent) throw new Error("Only protodesigns (designs without parent) can be moved to folders");
        const folderDiff = { folder: folderGuid };
        return { diff: { designs: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "quality": {
        const folderDiff = { folder: folderGuid };
        return { diff: { qualities: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "file": {
        const folderDiff = { folder: folderGuid };
        return { diff: { files: { updated: [{ id: artifactGuid, diff: folderDiff }] } } };
      }
      case "folder": {
        const parentDiff = { parent: folderGuid };
        return { diff: { folders: { updated: [{ id: artifactGuid, diff: parentDiff }] } } };
      }
    }
  },
  "semio.kit.import": (context: KitCommandContext, url: string): KitCommandResult => {
    (async () => {
      try {
        if (url.endsWith(".json")) {
          const response = await fetch(url);
          const kit: Kit = await response.json();
          const filesToFetch: { path: string; url: string }[] = [];
          const extractFileUrls = (obj: any) => {
            if (typeof obj === "object" && obj !== null) {
              if (Array.isArray(obj)) {
                obj.forEach((item) => extractFileUrls(item));
              } else {
                Object.entries(obj).forEach(([key, value]) => {
                  if (key === "url" && typeof value === "string" && !value.startsWith("http")) {
                    filesToFetch.push({ path: value, url: new URL(value, url).href });
                  }
                  extractFileUrls(value);
                });
              }
            }
          };
          extractFileUrls(kit);
          const files: KitCommandResult["files"] = [];
          for (const file of filesToFetch) {
            try {
              const fileResponse = await fetch(file.url);
              const fileBlob = await fileResponse.blob();
              files.push(new File([fileBlob], file.path));
            } catch (error) { }
          }
          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types ? { added: kit.types } : undefined,
              designs: kit.designs ? { added: kit.designs } : undefined,
              files: kit.files ? { added: kit.files } : undefined,
            },
            files,
          };
        } else {
          let SQL: SqlJsStatic;
          let db: Database;
          try {
            SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
          } catch (err) {
            throw new Error("SQL.js failed to initialize for import.");
          }
          const response = await fetch(url);
          const zipData = await response.arrayBuffer();
          const zip = await JSZip.loadAsync(zipData);
          let kit: Kit | null = null;
          const files: KitCommandResult["files"] = [];

          const kitDbFile = zip.file("kit.db");
          if (kitDbFile) {
            const dbData = await kitDbFile.async("uint8array");
            db = new SQL.Database(dbData);
            const kitResult = db.exec("SELECT * FROM kit LIMIT 1");
            if (kitResult.length > 0) {
              const kitRow = kitResult[0];
              const kitData = Object.fromEntries(kitRow.columns.map((col, i) => [col, kitRow.values[0][i]]));
              kit = {
                guid: (kitData.uri as string) || `urn:kit:${kitData.name as string}:${kitData.version as string}`,
                name: kitData.name as string,
                description: kitData.description as string,
                version: kitData.version as string,
                icon: kitData.icon as string,
                image: kitData.image as string,
                preview: kitData.preview as string,
                remote: kitData.remote as string,
                homepage: kitData.homepage as string,
                license: kitData.license as string,
                types: [],
                designs: [],
                files: [],
                createdAt: new Date(kitData.createdAt as string),
                updatedAt: new Date(kitData.updatedAt as string),
              };
            }
            db.close();
          } else {
            const kitJsonFile = zip.file("kit.json");
            if (kitJsonFile) {
              const kitData = await kitJsonFile.async("text");
              kit = JSON.parse(kitData);
            }
          }

          for (const [filename, file] of Object.entries(zip.files)) {
            if (!(file as any).dir && filename !== "kit.db" && filename !== "kit.json") {
              const fileData = await (file as any).async("uint8array");
              files.push(new File([new Uint8Array(fileData)], filename));
            }
          }

          if (!kit) {
            throw new Error("No kit.json or kit.db found in ZIP file");
          }

          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types && kit.types.length > 0 ? { added: kit.types } : undefined,
              designs: kit.designs && kit.designs.length > 0 ? { added: kit.designs } : undefined,
              files: kit.files && kit.files.length > 0 ? { added: kit.files } : undefined,
            },
            files,
          };
        }
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.export": (context: KitCommandContext): KitCommandResult => {
    (async () => {
      let SQL: SqlJsStatic;
      let db: Database;
      try {
        SQL = await initSqlJs({ locateFile: () => sqlWasmUrl });
      } catch (err) {
        throw new Error("SQL.js failed to initialize for export.");
      }

      db = new SQL.Database();
      const zip = new JSZip();
      const kit = context.kit;

      const schema = `
        CREATE TABLE kit ( uri VARCHAR(2048) NOT NULL UNIQUE, name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, preview VARCHAR(1024) NOT NULL, version VARCHAR(64) NOT NULL, remote VARCHAR(1024) NOT NULL, homepage VARCHAR(1024) NOT NULL, license VARCHAR(1024) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY );
        CREATE TABLE type ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE design ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, "view" VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, createdAt DATETIME NOT NULL, updatedAt DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, UNIQUE (name, variant, "view", kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE representation ( url VARCHAR(1024) NOT NULL, description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id) );
        CREATE TABLE tag ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id) );
        CREATE TABLE concept ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, type_id INTEGER, design_id INTEGER, FOREIGN KEY(kit_id) REFERENCES kit (id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE port ( description VARCHAR(512) NOT NULL, family VARCHAR(64) NOT NULL, t FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), point_x FLOAT, point_y FLOAT, point_z FLOAT, direction_x FLOAT, direction_y FLOAT, direction_z FLOAT, type_id INTEGER, CONSTRAINT "Unique local_id" UNIQUE (local_id, type_id), FOREIGN KEY(type_id) REFERENCES type (id) );
        CREATE TABLE compatible_family ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, port_id INTEGER, FOREIGN KEY(port_id) REFERENCES port (id) );
        CREATE TABLE plane ( id INTEGER NOT NULL PRIMARY KEY, origin_x FLOAT, origin_y FLOAT, origin_z FLOAT, x_axis_x FLOAT, x_axis_y FLOAT, x_axis_z FLOAT, y_axis_x FLOAT, y_axis_y FLOAT, y_axis_z FLOAT );
        CREATE TABLE piece ( description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), type_id INTEGER, plane_id INTEGER, center_x FLOAT, center_y FLOAT, design_id INTEGER, UNIQUE (local_id, design_id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(plane_id) REFERENCES plane (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE connection ( description VARCHAR(512) NOT NULL, gap FLOAT NOT NULL, shift FLOAT NOT NULL, rise FLOAT NOT NULL, rotation FLOAT NOT NULL, turn FLOAT NOT NULL, tilt FLOAT NOT NULL, x FLOAT NOT NULL, y FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, connected_piece_id INTEGER, connected_port_id INTEGER, connecting_piece_id INTEGER, connecting_port_id INTEGER, design_id INTEGER, CONSTRAINT "no reflexive connection" CHECK (connecting_piece_id != connected_piece_id), FOREIGN KEY(connected_piece_id) REFERENCES piece (id), FOREIGN KEY(connected_port_id) REFERENCES port (id), FOREIGN KEY(connecting_piece_id) REFERENCES piece (id), FOREIGN KEY(connecting_port_id) REFERENCES port (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        CREATE TABLE quality ( name VARCHAR(64) NOT NULL, value VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, definition VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, port_id INTEGER, type_id INTEGER, piece_id INTEGER, connection_id INTEGER, design_id INTEGER, kit_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id), FOREIGN KEY(port_id) REFERENCES port (id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(piece_id) REFERENCES piece (id), FOREIGN KEY(connection_id) REFERENCES connection (id), FOREIGN KEY(design_id) REFERENCES design (id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
        CREATE TABLE author ( name VARCHAR(64) NOT NULL, email VARCHAR(128) NOT NULL, rank INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, design_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(design_id) REFERENCES design (id) );
      `;

      try {
        db.run(schema);
        const insertQualities = (qualities: Attribute[] | undefined, fkColumn: string, fkValue: number) => {
          if (!qualities) return;
          const stmt = db.prepare(`INSERT INTO quality (name, value, unit, definition, ${fkColumn}) VALUES (?, ?, ?, ?, ?)`);
          qualities.forEach((q) => stmt.run([q.key, q.value ?? "", "", q.definition ?? "", fkValue]));
          stmt.free();
        };
        const insertAuthors = (authorGuids: string[] | undefined, fkColumn: string, fkValue: number) => {
          if (!authorGuids) return;
          const stmt = db.prepare(`INSERT INTO author (name, email, rank, ${fkColumn}) VALUES (?, ?, ?, ?)`);
          let rank = 0;
          authorGuids.forEach((authorGuid) => {
            const author = kit.authors?.find((a) => a.guid === authorGuid);
            if (author) {
              stmt.run([author.name, author.email ?? "", rank++, fkValue]);
            }
          });
          stmt.free();
        };

        const kitStmt = db.prepare("INSERT INTO kit (uri, name, description, icon, image, preview, version, remote, homepage, license, createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        const nowIso = new Date().toISOString();
        kitStmt.run([`urn:kit:${kit.name}:${kit.version || ""}`, kit.name, kit.description || "", kit.icon || "", kit.image || "", kit.preview || "", kit.version || "", kit.remote || "", kit.homepage || "", kit.license || "", nowIso, nowIso]);
        kitStmt.free();
        const Guid = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
        insertQualities(kit.attributes, "kit_id", Guid);

        if (kit.concepts) {
          const conceptStmt = db.prepare('INSERT INTO concept (name, "order", kit_id) VALUES (?, ?, ?)');
          kit.concepts.forEach((concept, index) => conceptStmt.run([concept, index, Guid]));
          conceptStmt.free();
        }

        if (kit.types) {
          const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, parent_id, is_abstract, unit, createdAt, updatedAt, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
          const repStmt = db.prepare("INSERT INTO representation (url, description, type_id) VALUES (?, ?, ?)");
          const tagStmt = db.prepare('INSERT INTO tag (name, "order", representation_id) VALUES (?, ?, ?)');
          const portStmt = db.prepare("INSERT INTO port (local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

          for (const type of kit.types) {
            typeStmt.run([type.name, type.description || "", type.icon || "", type.image || "", type.parent || null, type.isAbstract ? 1 : 0, type.unit || "", nowIso, nowIso, Guid]);
            const typeDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
            insertQualities(type.attributes, "type_id", typeDbId);
            insertAuthors(type.authors, "type_id", typeDbId);

            if (type.representations) {
              for (const rep of type.representations) {
                repStmt.run([rep.file, rep.description ?? "", typeDbId]);
                const repDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                insertQualities(rep.attributes, "representation_id", repDbId);
                if (rep.tags) {
                  rep.tags.forEach((tag, index) => tagStmt.run([tag, index, repDbId]));
                }
                const fileUrl = context.fileUrls.get(rep.file);
                if (fileUrl) {
                  try {
                    const response = await fetch(fileUrl);
                    const fileBlob = await response.blob();
                    const fileData = await fileBlob.arrayBuffer();
                    zip.file(rep.file, fileData);
                  } catch (error) { }
                }
              }
            }

            if (type.ports) {
              for (const port of type.ports) {
                portStmt.run([
                  port.guid || "",
                  port.description || "",
                  port.family || "default",
                  port.t || 0,
                  port.point?.x || 0,
                  port.point?.y || 0,
                  port.point?.z || 0,
                  port.direction?.x || 0,
                  port.direction?.y || 0,
                  port.direction?.z || 1,
                  typeDbId,
                ]);
                const portDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                insertQualities(port.attributes, "port_id", portDbId);
              }
            }
          }
          typeStmt.free();
          repStmt.free();
          tagStmt.free();
          portStmt.free();
        }

        const dbBuffer = db.export();
        zip.file("kit.db", dbBuffer);
        zip.file("kit.json", JSON.stringify(kit, null, 2));

        const blob = await zip.generateAsync({ type: "blob" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${kit.name}-${kit.version || "latest"}.zip`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        throw error;
      } finally {
        if (db) {
          db.close();
        }
      }
    })();
    return { diff: {} };
  },
  "semio.kit.addPiece": (context: KitCommandContext, guid: Guid, piece: Piece): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, guid)?.connections ?? []).some((connection) => connection.connected.piece === piece.guid || connection.connecting.piece === piece.guid)
                      ? piece
                      : {
                        ...piece,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addPieces": (context: KitCommandContext, guid: Guid, pieces: Piece[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece === candidate.guid || connection.connecting.piece === candidate.guid)
                      ? candidate
                      : {
                        ...candidate,
                        plane: {
                          origin: { x: 0, y: 0, z: 0 },
                          xAxis: { x: 1, y: 0, z: 0 },
                          yAxis: { x: 0, y: 1, z: 0 },
                        },
                      },
                  ),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePiece": (context: KitCommandContext, guid: Guid, piece: Guid): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { pieces: { removed: [piece] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePieces": (context: KitCommandContext, guid: Guid, pieces: Guid[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { pieces: { removed: pieces } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnection": (context: KitCommandContext, guid: Guid, connection: Connection): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnections": (context: KitCommandContext, guid: Guid, connections: Connection[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnection": (context: KitCommandContext, guid: Guid, connectionGuid: Guid): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    const connection = design?.connections?.find((c) => c.guid === connectionGuid);
    if (!connection) return { diff: {} };
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { connections: { removed: [{ connected: { piece: connection.connected.piece }, connecting: { piece: connection.connecting.piece } }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, guid: Guid, connectionGuids: Guid[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, guid);
    const connectionsToRemove =
      connectionGuids
        .map((connGuid) => design?.connections?.find((c) => c.guid === connGuid))
        .filter((c): c is Connection => c !== undefined)
        .map((c) => ({ connected: { piece: c.connected.piece }, connecting: { piece: c.connecting.piece } })) ?? [];
    return {
      diff: {
        designs: {
          updated: [
            {
              id: guid,
              diff: { connections: { removed: connectionsToRemove } },
            },
          ],
        },
      },
    };
  },
};
