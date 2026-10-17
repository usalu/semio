import type { SQLQueryBindings } from "bun:sqlite";
declare module "bun:sqlite" {
  interface Statement<ReturnType, ParamsType extends SQLQueryBindings[]> {
    safeIntegers(enabled?: boolean): this;
  }
}
