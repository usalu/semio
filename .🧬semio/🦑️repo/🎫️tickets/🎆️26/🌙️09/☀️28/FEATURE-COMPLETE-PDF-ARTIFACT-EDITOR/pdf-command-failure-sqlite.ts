import { deepStrictEqual } from "node:assert";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/⚖️invocation-failure/🔣️.json", "utf8"));
const db = new Database(":memory:");
db.exec(`
    CREATE TABLE scenario (
        ordinal INTEGER PRIMARY KEY,
        grant_items INTEGER NOT NULL CHECK (grant_items >= 0),
        fuel INTEGER NOT NULL CHECK (fuel >= 0),
        close_refusal TEXT,
        input_closed INTEGER NOT NULL CHECK (input_closed IN (0, 1))
    );
    INSERT INTO scenario VALUES
        (1, 0, 0, 'retirement refused', 0),
        (2, 1, 1, 'retirement refused', 0),
        (3, 1, 1, NULL, 1);
`);
const observed = db.query(`
    SELECT
        CASE WHEN fuel = 0 THEN 'pending' ELSE CASE
            WHEN close_refusal IS NOT NULL THEN 'refused'
            WHEN grant_items > 0 AND input_closed = 1 THEN 'failed'
            ELSE 'pending'
        END END AS step,
        (fuel = 0 OR close_refusal IS NOT NULL OR grant_items = 0 OR input_closed = 0) AS kept,
        CASE WHEN fuel > 0 THEN COALESCE(close_refusal, CASE WHEN grant_items > 0 AND input_closed = 1 THEN
            'original decoded-command failure' END) END AS cause
    FROM scenario ORDER BY ordinal
`).all() as { step: string; kept: number; cause: string | null }[];
deepStrictEqual(observed.map(row => ({ ...row, kept: Boolean(row.kept) })), fixture.cases);
db.close();
