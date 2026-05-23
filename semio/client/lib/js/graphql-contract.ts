//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — canonical GraphQL wire contract between `@semio/js` and `@semio/rs`.
//#endregion 🧲Header

/** @emoji 📜 Golden SDL path (single schema source for tooling and embedded tests). */
export const SEMIO_GRAPHQL_GOLDEN_SCHEMA_PATH = "semio/client/schema/graphql/schema.golden.graphql" as const;

/** @emoji 🧵 GraphQL-over-HTTP POST body — the only payload shape across the rs/js boundary. */
export type GraphqlWirePostBody = Readonly<{
  query: string;
  variables: Readonly<Record<string, unknown>>;
  operationName: string | null;
}>;

/** @emoji 🧪 Empty in-memory WASM store URI (host lifecycle only; kit state changes use GraphQL). */
export const RS_WASM_EMPTY_STORE_URI = "dev://empty" as const;

/** @emoji 🛑 Rejects non-empty WASM bootstrap URIs; kit JSON must use {@link Store.installProjection}. */
export function assertRsJsSessionOpenUri(uri: string): void {
  const t = uri.trim();
  if (t !== RS_WASM_EMPTY_STORE_URI) {
    throw new Error(
      `Session.open: only ${RS_WASM_EMPTY_STORE_URI} is allowed; use Session.openInMemory() and store.installProjection(json) for kit JSON`,
    );
  }
  if (t.startsWith("{") || t.startsWith("[")) {
    throw new Error("Session.open: inline JSON is not part of the rs/js contract; use Session.openInMemory() and store.installProjection(json)");
  }
  if (t.includes("dev+json:")) {
    throw new Error("Session.open: dev+json bootstrap is not part of the rs/js contract; use Session.openInMemory() and store.installProjection(json)");
  }
}
