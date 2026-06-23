from pathlib import Path

new_gql = r"""pub mod gql {
    //! 🌐 Dynamic GraphQL schema from `gql_target` (matches `compose/graphql/target.schema.graphql`).
    use std::sync::Arc;

    use async_graphql::dynamic::Schema;

    use crate::event::EventBus;
    use crate::gql_target::{build_executable_target_schema, TARGET_GRAPHQL_SDL};
    use crate::worker::ParentRuntime;

    /// @emoji 🧩 Executable schema type (async-graphql dynamic).
    pub type AppSchema = Schema;

    /// 📜 Canonical SDL string (embedded target contract).
    pub fn target_schema_sdl() -> String {
        TARGET_GRAPHQL_SDL.to_string()
    }

    /// 📜 Same as [`target_schema_sdl`] (async for historical call sites).
    pub async fn sdl() -> String {
        target_schema_sdl()
    }

    /// 🧱 Build schema with parent runtime + bus.
    pub async fn build_schema_for(rt: Arc<ParentRuntime>) -> AppSchema {
        let bus: Arc<EventBus> = rt.bus.clone();
        build_executable_target_schema(rt, bus).expect("compose GraphQL target schema")
    }

    /// 🧱 Default schema (fresh runtime).
    pub async fn build_schema() -> AppSchema {
        build_schema_for(ParentRuntime::spawn().await).await
    }
}

"""

root = Path(__file__).resolve().parents[6]  # .../git root (compose)
lib = root / "compose" / "rs" / "lib.rs"
text = lib.read_text(encoding="utf-8")
start = text.index("pub mod gql {")
end = text.index("//#endregion 🌐 gql", start)
lib.write_text(text[:start] + new_gql + text[end:], encoding="utf-8")
print("patched", lib)
