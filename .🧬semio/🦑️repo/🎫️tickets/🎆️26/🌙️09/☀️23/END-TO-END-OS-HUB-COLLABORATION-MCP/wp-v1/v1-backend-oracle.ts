import { SQL } from "bun";
import { connect } from "node:net";
const [pgUrl, boltPort] = process.argv.slice(2);
const sql = new SQL(pgUrl!);
const [row] = await sql`SELECT current_database() AS db, version() AS v, 1 + 1 AS two`;
console.log(`postgres via Bun.SQL: db=${row.db} two=${row.two} ${String(row.v).split(" ").slice(0, 2).join(" ")}`);
await sql.close();
const reply = await new Promise<Buffer>((resolve, reject) => {
  const socket = connect(Number(boltPort), "127.0.0.1", () => socket.write(Buffer.from([0x60, 0x60, 0xb0, 0x17, 0, 0, 4, 5, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 0])));
  socket.once("data", (data) => { resolve(data); socket.end(); });
  socket.once("error", reject);
});
console.log(`neo4j bolt handshake agreed version bytes=${[...reply].join(",")}`);
