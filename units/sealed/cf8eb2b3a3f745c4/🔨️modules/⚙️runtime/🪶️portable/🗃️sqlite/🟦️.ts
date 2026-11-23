import { existsSync } from "node:fs";
import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
import type { RuntimeSqliteChangesV1, RuntimeSqliteConstructorV1, RuntimeSqliteDatabaseV1, RuntimeSqliteOptionsV1, RuntimeSqliteStatementV1 } from "../../📋️contract/🟦️.ts";

type NodeStatementV1 = { all(...parameters: any[]): any[]; get(...parameters: any[]): any; run(...parameters: any[]): { changes: number | bigint; lastInsertRowid: number | bigint }; setReturnArrays?(enabled: boolean): void; setReadBigInts?(enabled: boolean): void };
type NodeDatabaseV1 = { prepare(sql: string): NodeStatementV1; exec(sql: string): void; close(): void };
type NodeModuleV1 = { DatabaseSync: new (path: string, options?: { readOnly?: boolean }) => NodeDatabaseV1 };

function wrapStatement(prepare: () => NodeStatementV1): RuntimeSqliteStatementV1 {
  const changes = (row: { changes: number | bigint; lastInsertRowid: number | bigint }): RuntimeSqliteChangesV1 => ({ changes: Number(row.changes), lastInsertRowid: row.lastInsertRowid });
  const statement = prepare();
  return {
    all: (...parameters) => statement.all(...parameters),
    get: (...parameters) => statement.get(...parameters) ?? null,
    run: (...parameters) => changes(statement.run(...parameters)),
    values: (...parameters) => {
      if (statement.setReturnArrays !== undefined) { statement.setReturnArrays(true); try { return statement.all(...parameters) as any[][]; } finally { statement.setReturnArrays(false); } }
      return statement.all(...parameters).map(row => Object.values(row));
    },
    safeIntegers(enabled = true) { statement.setReadBigInts?.(enabled); return this; },
    finalize: () => undefined,
  };
}

/** 🗃️ Builds the connection class over `node:sqlite` for runtimes that lack their own; same method set and `null` for a missing row. */
export function nodeSqliteConstructorV1(): RuntimeSqliteConstructorV1 {
  const loaded = process.getBuiltinModule("node:sqlite") as unknown as NodeModuleV1 | undefined;
  if (loaded === undefined) throw new Error(runtimeMessageV1("sqliteUnavailable", {}));
  const module: NodeModuleV1 = loaded;
  return class NodeSqliteDatabaseV1 implements RuntimeSqliteDatabaseV1 {
    private readonly database: NodeDatabaseV1;
    constructor(path: string = ":memory:", options: RuntimeSqliteOptionsV1 = {}) {
      if (options.create === false && path !== ":memory:" && !existsSync(path)) throw new Error(runtimeMessageV1("sqliteCannotOpen", { path }));
      this.database = new module.DatabaseSync(path, { readOnly: options.readonly === true });
    }
    query<Row = any, Params extends any[] = any[]>(sql: string): RuntimeSqliteStatementV1<Row, Params> { return wrapStatement(() => this.database.prepare(sql)); }
    prepare<Row = any, Params extends any[] = any[]>(sql: string): RuntimeSqliteStatementV1<Row, Params> { return wrapStatement(() => this.database.prepare(sql)); }
    run(sql: string, ...parameters: any[]): RuntimeSqliteChangesV1 { const row = this.database.prepare(sql).run(...parameters); return { changes: Number(row.changes), lastInsertRowid: row.lastInsertRowid }; }
    exec(sql: string): unknown { return this.database.exec(sql); }
    transaction<T extends (...arguments_: any[]) => any>(body: T): T {
      const database = this.database;
      return ((...arguments_: unknown[]) => {
        database.exec("BEGIN");
        try { const value = body(...arguments_); database.exec("COMMIT"); return value; } catch (error) { database.exec("ROLLBACK"); throw error; }
      }) as T;
    }
    close(): void { this.database.close(); }
  };
}
