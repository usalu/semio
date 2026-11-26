/** 📖️ Reads the saved drawing's document archive through the folder and wire readers. */
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { eventLogPath, readLatestArchive } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🗄️backbone-storage/📜️event-log/🟦️.ts";
import { decodeDocumentArchiveBytes } from "../../../../../../../🧰️framework/🔨️modules/🪢️wire-contracts/🟦️.ts";


const [folder, id] = Bun.argv.slice(2);
const archive = readLatestArchive(eventLogPath(resolve(folder)),id);
if (!archive) throw Error("No archive for the saved document");
const document = decodeDocumentArchiveBytes(archive);
const pack = Buffer.from(document.parent_pack);
const token = pack.subarray(12,12+pack.readUInt32LE(8)).toString();
console.log(JSON.stringify({id, archive:archive.length, digest:createHash("sha256").update(archive).digest("hex"), pack:pack.length, token, spr:document.parent_spr.length,members:document.members.length}));
