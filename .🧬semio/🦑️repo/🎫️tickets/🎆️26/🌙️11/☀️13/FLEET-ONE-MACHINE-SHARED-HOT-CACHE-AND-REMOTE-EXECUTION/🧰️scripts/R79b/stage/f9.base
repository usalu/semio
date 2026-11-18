import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeSqliteDatabaseV1, RuntimeSqliteOptionsV1 } from "../📋️contract/🟦️.ts";

/** 🗃️ A SQLite connection whose implementation is chosen by the hosting runtime. */
export interface SqliteDatabaseV1 extends RuntimeSqliteDatabaseV1 {}
export class SqliteDatabaseV1 {
  constructor(path?: string, options?: RuntimeSqliteOptionsV1) { return new (requireRuntimeMemberV1("sqlite")())(path, options) as SqliteDatabaseV1; }
}
