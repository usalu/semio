#!/usr/bin/env node
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function checkDatabase() {
  const JSZip = (await import("jszip")).default;
  const initSqlJs = (await import("sql.js")).default;

  const zipPath = path.join(__dirname, "../../assets/metabolism.zip");
  const zipBuffer = fs.readFileSync(zipPath);
  const zip = await JSZip.loadAsync(zipBuffer);

  const dbFile = zip.file(".semio/kit.db");
  if (!dbFile) {
    console.error("No .semio/kit.db found in zip");
    return;
  }

  const dbArrayBuffer = await dbFile.async("arraybuffer");
  const SQL = await initSqlJs();
  const db = new SQL.Database(new Uint8Array(dbArrayBuffer));

  const result = db.exec("SELECT COUNT(*) as count FROM connection");
  console.log(`Connections in exported DB: ${result[0]?.values[0]?.[0] ?? 0}`);

  const designResult = db.exec("SELECT guid, name FROM design");
  console.log(`\nDesigns in DB:`);
  designResult[0]?.values.forEach((row) => {
    const designGuid = row[0];
    const designName = row[1];
    const connResult = db.exec(`SELECT COUNT(*) FROM connection WHERE design_guid = '${designGuid}'`);
    const connCount = connResult[0]?.values[0]?.[0] ?? 0;
    console.log(`  - ${designName}: ${connCount} connections`);
  });

  db.close();
}

checkDatabase().catch(console.error);
