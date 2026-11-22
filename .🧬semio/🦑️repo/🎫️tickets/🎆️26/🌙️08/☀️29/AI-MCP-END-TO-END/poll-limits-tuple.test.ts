import { Database } from "bun:sqlite"
import { expect, test } from "bun:test"
import Ajv from "ajv"
import law from "./poll-limits-tuple.json"
import schema from "./poll-limits-tuple.schema.json"

const validateShape = new Ajv().compile(schema)
const unsigned = /^(0|[1-9][0-9]*)$/

function exactTuple(wire: string): string[] | null {
  if (!validateShape(JSON.parse(wire))) return null
  const tuple = JSON.parse(wire, (_key, value, context) => typeof value === "number" ? context.source : value) as string[]
  return tuple.every((word, index) =>
    unsigned.test(word) && BigInt(word) <= (index >= 5 && index <= 7 ? 18446744073709551615n : 4294967295n)
  ) ? tuple : null
}

test("owned Core Poll uses eleven positional unsigned numeric fields", () => {
  expect(schema.items.map(item => item.title)).toEqual(law.fields)
  for (const row of law.cases) expect(exactTuple(row.wire) !== null, row.name).toBe(row.valid)
})

test("Poll data narrows against authenticated caps without supplying identity", () => {
  const db = new Database(":memory:")
  const capForPosition = [0, 1, 2, 3, 4, 5, 5, 6, null, null, null]
  const decimal = (value: string) => value.padStart(20, "0")
  try {
    db.exec("create table loan(identity text); create table position(rank integer primary key, requested text, ceiling text)")
    const insert = db.query("insert into position values (?, ?, ?)")
    const original = db.query("select identity from loan")
    const narrow = db.query("select case when ceiling is null or requested <= ceiling then requested else ceiling end as value from position order by rank")
    for (const row of law.narrowing) {
      db.exec("delete from position; delete from loan")
      db.query("insert into loan values (?)").run(JSON.stringify(row.identity))
      for (const [index, value] of exactTuple(row.wire)!.entries()) {
        const cap = capForPosition[index]
        insert.run(index, decimal(value), cap === null ? null : decimal(row.authority[cap]))
      }
      expect(narrow.all().map(row => BigInt((row as { value: string }).value).toString())).toEqual(row.expected)
      expect(JSON.parse((original.get() as { identity: string }).identity)).toEqual(row.identity)
    }
  } finally {
    db.close()
  }
})
