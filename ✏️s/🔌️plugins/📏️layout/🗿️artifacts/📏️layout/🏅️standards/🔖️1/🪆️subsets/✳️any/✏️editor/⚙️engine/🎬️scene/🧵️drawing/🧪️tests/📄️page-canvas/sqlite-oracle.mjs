/** ⚖️ The independent SQLite engine verifies member DDL, identity and parent ownership. */
import { DatabaseSync } from "node:sqlite";
const quote = name => '"' + name.replaceAll('"', '""') + '"';
export function roundTrip(input) {
  const db = new DatabaseSync(":memory:");
  try {
    db.exec("PRAGMA foreign_keys=OFF");
    for (const table of input.tables) db.exec(table.sql);
    for (const table of input.tables) for (const row of table.rows) db.prepare("INSERT INTO " + quote(table.name) + " VALUES (" + row.values.map(() => "?").join(",") + ")").run(...row.values);
    db.exec("PRAGMA foreign_keys=ON");
    if (db.prepare("PRAGMA foreign_key_check").all().length) throw Error("SQLite foreign-key violation");
    const invalid = "SELECT id FROM layout_drawing_member WHERE (document_id IS NULL)=(page_id IS NULL) OR child_id='' OR artifact_id='' UNION ALL SELECT MIN(id) FROM layout_drawing_member WHERE page_id IS NOT NULL GROUP BY page_id HAVING COUNT(*)>1 UNION ALL SELECT MIN(id) FROM layout_drawing_member WHERE document_id IS NOT NULL GROUP BY document_id HAVING COUNT(*)>1 UNION ALL SELECT MIN(id) FROM layout_drawing_member GROUP BY child_id HAVING COUNT(*)>1 UNION ALL SELECT MIN(id) FROM layout_drawing_member GROUP BY artifact_id HAVING COUNT(*)>1";
    if (db.prepare(invalid).all().length) throw Error("ambiguous Layout Drawing membership");
    return { tables: input.tables.map(table => {
      const statement = db.prepare("SELECT * FROM " + quote(table.name) + " ORDER BY id");
      statement.setReadBigInts(true);
      return { name: table.name, sql: db.prepare("SELECT sql FROM sqlite_master WHERE name=?").get(table.name).sql, rows: statement.all().map(row => ({ rowid: row.id, values: Object.values(row) })) };
    }) };
  } finally { db.close(); }
}
