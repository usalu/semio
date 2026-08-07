//! 🦀️ semio_compose_rs rust control plane — in-memory Arc-reference architecture (code-first GraphQL).

#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::duplicated_attributes)]

//#region 🧬️ entity_dsl

/// @emoji 🧩️ Chains `SchemaBuilder::register_output_type` so macro-emitted shapes stay reachable in `Schema::sdl()`.
#[macro_export]
macro_rules! register_output_types {
    ($builder:expr, $( $ty:ty ),+ $(,)? ) => {{
        $builder $( .register_output_type::<$ty>() )+
    }};
}

/// @emoji 🧩️ Chains `SchemaBuilder::register_input_type` for `InputObject` shapes not yet referenced by `Query`/`Mutation` fields (keeps golden `input` lines in `gql::sdl()`).
#[macro_export]
macro_rules! register_input_types {
    ($builder:expr, $( $ty:ty ),+ $(,)? ) => {{
        $builder $( .register_input_type::<$ty>() )+
    }};
}

/// @emoji 🪢️ `_entity_relay_shell!` — shared relay Edge/Connection structs; `from_entities` body is supplied by [`entity_relay!`] or [`entity_relay_sync!`].
#[macro_export]
macro_rules! _entity_relay_shell {
    ($Conn:ident, $Edge:ident, $Node:ty, $($from_entities:tt)*) => {
        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Edge {
            pub cursor: String,
            pub node: $Node,
        }

        #[derive(Clone, async_graphql::SimpleObject)]
        pub struct $Conn {
            pub edges: Vec<$Edge>,
            #[graphql(name = "pageInfo")]
            pub page_info: std::sync::Arc<$crate::gql_relay::PageInfo>,
            pub hash: String,
        }

        impl $Conn {
            $($from_entities)*
        }
    };
}

/// @emoji 🪢️ `entity_relay_sync!` — relay Edge/Connection for `SimpleObject` entities with sync child digests (`compute_entity_hash`, …).
#[macro_export]
macro_rules! entity_relay_sync {
    ($Conn:ident, $Edge:ident, $Node:ty, $hash_fn:expr) => {
        $crate::_entity_relay_shell!(
            $Conn,
            $Edge,
            $Node,
            pub fn from_entities(entities: Vec<$Node>) -> Self {
                let mut child_hashes = Vec::with_capacity(entities.len());
                for r in &entities {
                    child_hashes.push($hash_fn(r));
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = entities.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        );
    };
}

/// @emoji 🪢️ `entity_full_family!` — relay Edge/Connection for geometry (`VectorEdge`…`LocationEdge`).
#[macro_export]
macro_rules! entity_full_family {
    (
        $_base:ident,
        $node:ty,
        relay = ($conn:ident, $edge:ident)
    ) => {
        $crate::entity_relay!($conn, $edge, $node);
    };
}

/// @emoji 🪢️ `entity_relay!` — relay Edge/Connection for `Arc` graph nodes with async `compute_hash` digests.
#[macro_export]
macro_rules! entity_relay {
    ($Conn:ident, $Edge:ident, $Node:ty) => {
        $crate::_entity_relay_shell!(
            $Conn,
            $Edge,
            $Node,
            pub async fn from_entities(entities: Vec<$Node>) -> Self {
                let mut child_hashes = Vec::with_capacity(entities.len());
                for r in &entities {
                    child_hashes.push(r.compute_hash().await);
                }
                let hash = $crate::hash::merkle_collection(child_hashes);
                let edges = entities.into_iter().enumerate().map(|(i, node)| $Edge { cursor: $crate::gql_relay::edge_cursor(i), node }).collect();
                Self { edges, page_info: std::sync::Arc::new($crate::gql_relay::PageInfo::default()), hash }
            }
        );
    };
}

/// @emoji 🪜️ `_ladder_relay_lite!` — golden **3-shell** relay (`NameEdge` + `NameConnection`); delegates to [`entity_relay!`].
#[macro_export]
macro_rules! _ladder_relay_lite {
    ($Conn:ident, $Edge:ident, $Node:ty) => {
        $crate::entity_relay!($Conn, $Edge, $Node);
    };
}

/// @emoji 🪜️ `_ladder_relay_full!` — golden **12-shell** tail: `*Diff*` + `*Modification*` + `*Modifications*` relay triple (after main `entity_relay!`).
#[macro_export]
macro_rules! _ladder_relay_full {
    (
        $diff_conn:ident,
        $diff_edge:ident,
        $diff_node:ty,
        $mod_conn:ident,
        $mod_edge:ident,
        $mod_node:ty,
        $mods_conn:ident,
        $mods_edge:ident,
        $mods_node:ty
    ) => {
        $crate::entity_relay!($diff_conn, $diff_edge, $diff_node);
        $crate::entity_relay!($mod_conn, $mod_edge, $mod_node);
        $crate::entity_relay!($mods_conn, $mods_edge, $mods_node);
    };
}

/// @emoji 🧱️ `entity_bare!` — splices `item` tokens (structs, impls, consts) with **no** relay shells; use for roots and `LocalProvider`-class bare nodes.
#[macro_export]
macro_rules! entity_bare {
    ($($item:item)*) => {
        $($item)*
    };
}

/// @emoji 🪢️ `entity_lite!` — golden **3-ladder** (`Name` + `NameEdge` + `NameConnection`); expands relay via [`_ladder_relay_lite!`].
#[macro_export]
macro_rules! entity_lite {
    ($Conn:ident, $Edge:ident, $Node:ty $(,)?) => {
        $crate::_ladder_relay_lite!($Conn, $Edge, $Node);
    };
}

/// @emoji 🏗️ `entity_full!` — golden **12-ladder** contract: main `entity_relay!` plus optional [`_ladder_relay_full!`] when diff/mod families are wired.
#[macro_export]
macro_rules! entity_full {
    (
        relay = ($conn:ident, $edge:ident, $node:ty)
        $(, ladder_full = (
            diff = ($diff_conn:ident, $diff_edge:ident, $diff_node:ty),
            modification = ($mod_conn:ident, $mod_edge:ident, $mod_node:ty),
            modifications = ($mods_conn:ident, $mods_edge:ident, $mods_node:ty)
        ))?
        $(,)?
    ) => {
        $crate::entity_relay!($conn, $edge, $node);
        $(
            $crate::_ladder_relay_full!(
                $diff_conn,
                $diff_edge,
                $diff_node,
                $mod_conn,
                $mod_edge,
                $mod_node,
                $mods_conn,
                $mods_edge,
                $mods_node
            );
        )?
    };
}

/// @emoji ⚙️ `operation_with_input!` — splices `item` tokens for an operation plus its `*Input` surface; combine with [`entity_lite!`] for `*Edge`/`*Connection` where those Rust types exist.
#[macro_export]
macro_rules! operation_with_input {
    ($($item:item)*) => {
        $($item)*
    };
}

/// @emoji ⚙️ `operation_no_input!` — splices `item` tokens for inputless operations; combine with [`entity_lite!`] for relay shells.
#[macro_export]
macro_rules! operation_no_input {
    ($($item:item)*) => {
        $($item)*
    };
}

/// @emoji 🧾️ `entity_input!` — GraphQL `InputObject` with explicit SDL `name` (no serde; control plane is GraphQL-native).
#[macro_export]
macro_rules! entity_input {
    (
        $(#[$sm:meta])*
        $vis:vis struct $Name:ident as $gql:literal {
            $($(#[$fm:meta])* $fvis:vis $field:ident : $ftype:ty),* $(,)?
        }
    ) => {
        $(#[$sm])*
        #[derive(Clone, Debug, Default, PartialEq, async_graphql::InputObject, dsl::DslRecord)]
        #[graphql(name = $gql)]
        $vis struct $Name {
            $($(#[$fm])* $fvis $field : $ftype),*
        }
    };
}

/// @emoji 📁️ VFS computed fields for types implementing golden `FileSystemNode`.
#[macro_export]
macro_rules! file_system_node_complex_methods {
    ($variant:ident) => {
        pub async fn file_system_parent(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::parent(&iface).await)
        }
        pub async fn file_system_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeConnection> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::children(&iface).await)
        }
        pub async fn file_system_child(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>, id: $crate::id::Id) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::child(&iface, &id).await)
        }
        pub async fn file_system_path(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::path(&iface).await)
        }
        pub async fn file_system_name(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::name(&iface).await)
        }
        pub async fn is_file_system_root(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
            let _ = ctx;
            Ok(matches!($crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone()), $crate::gql::interfaces::FileSystemNodeInterface::Kit(_)))
        }
        pub async fn file_system_kind(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeKind> {
            let _ = ctx;
            Ok($crate::gql::interfaces::file_system_vfs::kind(&$crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone())))
        }
        pub async fn file_system_has_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
            let _ = ctx;
            let iface = $crate::gql::interfaces::FileSystemNodeInterface::$variant(self.clone());
            Ok($crate::gql::interfaces::file_system_vfs::has_children(&iface).await)
        }
    };
}

/// @emoji 📁️ `#[Object]` VFS fields resolved via `file_system_vfs::node_for_*` (must live on the GraphQL object, not a detached `ComplexObject`).
#[macro_export]
macro_rules! file_system_node_object_methods {
    ($ty:ty, $node_for:path, $default_kind:ident) => {
        pub async fn file_system_parent(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok(None);
            };
            Ok($crate::gql::interfaces::file_system_vfs::parent(&node).await)
        }
        pub async fn file_system_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeConnection> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok($crate::gql::interfaces::file_system_vfs::empty_connection());
            };
            Ok($crate::gql::interfaces::file_system_vfs::children(&node).await)
        }
        pub async fn file_system_child(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>, id: $crate::id::Id) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok(None);
            };
            Ok($crate::gql::interfaces::file_system_vfs::child(&node, &id).await)
        }
        pub async fn file_system_path(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok(String::new());
            };
            Ok($crate::gql::interfaces::file_system_vfs::path(&node).await)
        }
        pub async fn file_system_name(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok(String::new());
            };
            Ok($crate::gql::interfaces::file_system_vfs::name(&node).await)
        }
        pub async fn is_file_system_root(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
            Ok(matches!($node_for(self, ctx).await, Some($crate::gql::interfaces::FileSystemNodeInterface::Kit(_))))
        }
        pub async fn file_system_kind(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeKind> {
            Ok($node_for(self, ctx).await.map(|node| $crate::gql::interfaces::file_system_vfs::kind(&node)).unwrap_or($crate::gql::interfaces::FileSystemNodeKind::$default_kind))
        }
        pub async fn file_system_has_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
            let Some(node) = $node_for(self, ctx).await else {
                return Ok(false);
            };
            Ok($crate::gql::interfaces::file_system_vfs::has_children(&node).await)
        }
    };
}

/// @emoji 📁️ `#[ComplexObject]` VFS fields resolved via `file_system_vfs::node_for_*` (for `#[Object]` types without a direct `Arc` handle).
#[macro_export]
macro_rules! file_system_node_vfs_complex_ctx {
    ($ty:ty, $node_for:path) => {
        #[$crate::external_adapters::async_graphql::ComplexObject]
        impl $ty {
            pub async fn file_system_parent(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok(None);
                };
                Ok($crate::gql::interfaces::file_system_vfs::parent(&node).await)
            }
            pub async fn file_system_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeConnection> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok($crate::gql::interfaces::file_system_vfs::empty_connection());
                };
                Ok($crate::gql::interfaces::file_system_vfs::children(&node).await)
            }
            pub async fn file_system_child(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>, id: $crate::id::Id) -> $crate::external_adapters::async_graphql::Result<Option<$crate::gql::interfaces::FileSystemNodeInterface>> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok(None);
                };
                Ok($crate::gql::interfaces::file_system_vfs::child(&node, &id).await)
            }
            pub async fn file_system_path(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok(String::new());
                };
                Ok($crate::gql::interfaces::file_system_vfs::path(&node).await)
            }
            pub async fn file_system_name(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<String> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok(String::new());
                };
                Ok($crate::gql::interfaces::file_system_vfs::name(&node).await)
            }
            pub async fn is_file_system_root(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
                Ok(matches!($node_for(self, ctx).await, Some($crate::gql::interfaces::FileSystemNodeInterface::Kit(_))))
            }
            pub async fn file_system_kind(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<$crate::gql::interfaces::FileSystemNodeKind> {
                Ok(match $node_for(self, ctx).await {
                    Some(node) => $crate::gql::interfaces::file_system_vfs::kind(&node),
                    None => $crate::gql::interfaces::FileSystemNodeKind::Kit,
                })
            }
            pub async fn file_system_has_children(&self, ctx: &$crate::external_adapters::async_graphql::Context<'_>) -> $crate::external_adapters::async_graphql::Result<bool> {
                let Some(node) = $node_for(self, ctx).await else {
                    return Ok(false);
                };
                Ok($crate::gql::interfaces::file_system_vfs::has_children(&node).await)
            }
        }
    };
}

/// @emoji 🏷️ `entity_family!` — `SimpleObject` + sync `compute_entity_hash` + `ComplexObject::hash` resolver shell.
#[macro_export]
macro_rules! entity_family {
    (
        $(#[$sm:meta])*
        $vis:vis struct $Name:ident {
            $($(#[$fm:meta])* $fvis:vis $field:ident : $ftype:ty),* $(,)?
        }
        hash = |$this:ident| $body:block
        $(, extra = ( $($extra:tt)* ))?
        , vfs = $vfs_variant:ident
    ) => {
        $(#[$sm])*
        #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
        #[graphql(complex)]
        $vis struct $Name {
            $($(#[$fm])* $fvis $field : $ftype),*
        }
        impl $Name {
            pub fn compute_entity_hash(&self) -> String {
                let $this = self;
                $body
            }
        }
        #[async_graphql::ComplexObject]
        impl $Name {
            pub async fn hash(&self) -> String {
                self.compute_entity_hash()
            }
            $($($extra)*)?
            $crate::file_system_node_complex_methods!($vfs_variant);
        }
    };
    (
        $(#[$sm:meta])*
        $vis:vis struct $Name:ident {
            $($(#[$fm:meta])* $fvis:vis $field:ident : $ftype:ty),* $(,)?
        }
        hash = |$this:ident| $body:block
        $(, extra = ( $($extra:tt)* ))?
    ) => {
        $(#[$sm])*
        #[derive(Clone, Debug, Default, async_graphql::SimpleObject)]
        #[graphql(complex)]
        $vis struct $Name {
            $($(#[$fm])* $fvis $field : $ftype),*
        }
        impl $Name {
            pub fn compute_entity_hash(&self) -> String {
                let $this = self;
                $body
            }
        }
        #[async_graphql::ComplexObject]
        impl $Name {
            pub async fn hash(&self) -> String {
                self.compute_entity_hash()
            }
            $($($extra)*)?
        }
    };
}

/// @emoji 🏷️ `meta_arc_titled_entity!` — shared Arc/RwLock tag/concept entity (`new`, `new_with_id`, `compute_hash`, `Default`); owner is always [`crate::gql::interfaces::KitGraphParentWeak`] per golden `Entity.owner`.
#[macro_export]
macro_rules! meta_arc_titled_entity {
    (
        $(#[$sm:meta])*
        $N:ident,
        $tag:literal
    ) => {
        $(#[$sm])*
        #[derive(Debug)]
        pub struct $N {
            pub id: $crate::id::Id,
            pub owner: async_lock::RwLock<$crate::gql::interfaces::KitGraphParentWeak>,
            pub name: async_lock::RwLock<String>,
            pub description: async_lock::RwLock<Option<String>>,
            pub icon: async_lock::RwLock<Option<String>>,
            pub order: async_lock::RwLock<Option<i32>>,
            pub attributes: async_lock::RwLock<Vec<$crate::meta::Attribute>>,
        }

        impl $N {
            pub async fn new(
                owner: $crate::gql::interfaces::KitGraphParentWeak,
                name: String,
                description: Option<String>,
                icon: Option<String>,
                order: Option<i32>,
                attributes: Vec<$crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id: $crate::id::Id::new().await,
                    owner: async_lock::RwLock::new(owner),
                    name: async_lock::RwLock::new(name),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    order: async_lock::RwLock::new(order),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub fn new_with_id(
                owner: $crate::gql::interfaces::KitGraphParentWeak,
                id: $crate::id::Id,
                name: String,
                description: Option<String>,
                icon: Option<String>,
                order: Option<i32>,
                attributes: Vec<$crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id,
                    owner: async_lock::RwLock::new(owner),
                    name: async_lock::RwLock::new(name),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    order: async_lock::RwLock::new(order),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let n = self.name.read().await;
                let d = self.description.read().await.clone().unwrap_or_default();
                let ic = self.icon.read().await.clone().unwrap_or_default();
                let ord = self.order.read().await.map(|o| o.to_string()).unwrap_or_default();
                let attrs = self.attributes.read().await;
                let mut child_hashes: Vec<String> = attrs.iter().map($crate::meta::Attribute::compute_entity_hash).collect();
                child_hashes.sort();
                $crate::hash::merkle_node_str(&[$tag, self.id.as_str(), n.as_str(), d.as_str(), ic.as_str(), ord.as_str()], child_hashes)
            }
        }

        impl Default for $N {
            fn default() -> Self {
                Self {
                    id: $crate::id::Id::default(),
                    owner: async_lock::RwLock::new($crate::gql::interfaces::KitGraphParentWeak::default()),
                    name: async_lock::RwLock::new(String::new()),
                    description: async_lock::RwLock::new(None),
                    icon: async_lock::RwLock::new(None),
                    order: async_lock::RwLock::new(None),
                    attributes: async_lock::RwLock::new(Vec::new()),
                }
            }
        }
    };
}

/// @emoji 🏷️ `meta_quality_entity!` — Arc/RwLock quality entity (`new`, `new_with_id`, `compute_hash`, `Default`).
#[macro_export]
macro_rules! meta_quality_entity {
    () => {
        /// @emoji 🏷️ SDL `Quality` entity (benchmarks stay value-typed [`Benchmark`] entities).
        #[derive(Debug)]
        pub struct Quality {
            pub id: $crate::id::Id,
            pub owner: async_lock::RwLock<$crate::gql::interfaces::KitGraphParentWeak>,
            pub key: async_lock::RwLock<String>,
            pub value: async_lock::RwLock<Option<String>>,
            pub unit: async_lock::RwLock<Option<String>>,
            pub definition: async_lock::RwLock<Option<String>>,
            pub description: async_lock::RwLock<Option<String>>,
            pub icon: async_lock::RwLock<Option<String>>,
            pub benchmarks: async_lock::RwLock<Vec<$crate::meta::Benchmark>>,
            pub attributes: async_lock::RwLock<Vec<$crate::meta::Attribute>>,
        }

        impl Quality {
            pub async fn new(
                owner: $crate::gql::interfaces::KitGraphParentWeak,
                key: String,
                value: Option<String>,
                unit: Option<String>,
                definition: Option<String>,
                description: Option<String>,
                icon: Option<String>,
                benchmarks: Vec<$crate::meta::Benchmark>,
                attributes: Vec<$crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id: $crate::id::Id::new().await,
                    owner: async_lock::RwLock::new(owner),
                    key: async_lock::RwLock::new(key),
                    value: async_lock::RwLock::new(value),
                    unit: async_lock::RwLock::new(unit),
                    definition: async_lock::RwLock::new(definition),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    benchmarks: async_lock::RwLock::new(benchmarks),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub fn new_with_id(
                owner: $crate::gql::interfaces::KitGraphParentWeak,
                id: $crate::id::Id,
                key: String,
                value: Option<String>,
                unit: Option<String>,
                definition: Option<String>,
                description: Option<String>,
                icon: Option<String>,
                benchmarks: Vec<$crate::meta::Benchmark>,
                attributes: Vec<$crate::meta::Attribute>,
            ) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    id,
                    owner: async_lock::RwLock::new(owner),
                    key: async_lock::RwLock::new(key),
                    value: async_lock::RwLock::new(value),
                    unit: async_lock::RwLock::new(unit),
                    definition: async_lock::RwLock::new(definition),
                    description: async_lock::RwLock::new(description),
                    icon: async_lock::RwLock::new(icon),
                    benchmarks: async_lock::RwLock::new(benchmarks),
                    attributes: async_lock::RwLock::new(attributes),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let k = self.key.read().await;
                let v = self.value.read().await.clone().unwrap_or_default();
                let u = self.unit.read().await.clone().unwrap_or_default();
                let def = self.definition.read().await.clone().unwrap_or_default();
                let desc = self.description.read().await.clone().unwrap_or_default();
                let ic = self.icon.read().await.clone().unwrap_or_default();
                let bm = self.benchmarks.read().await;
                let av = self.attributes.read().await;
                let mut child_hashes: Vec<String> = bm.iter().map($crate::meta::Benchmark::compute_entity_hash).collect();
                child_hashes.extend(av.iter().map($crate::meta::Attribute::compute_entity_hash));
                child_hashes.sort();
                $crate::hash::merkle_node_str(&["Quality", self.id.as_str(), k.as_str(), v.as_str(), u.as_str(), def.as_str(), desc.as_str(), ic.as_str()], child_hashes)
            }
        }

        impl Default for Quality {
            fn default() -> Self {
                Self {
                    id: $crate::id::Id::default(),
                    owner: async_lock::RwLock::new($crate::gql::interfaces::KitGraphParentWeak::default()),
                    key: async_lock::RwLock::new(String::new()),
                    value: async_lock::RwLock::new(None),
                    unit: async_lock::RwLock::new(None),
                    definition: async_lock::RwLock::new(None),
                    description: async_lock::RwLock::new(None),
                    icon: async_lock::RwLock::new(None),
                    benchmarks: async_lock::RwLock::new(Vec::new()),
                    attributes: async_lock::RwLock::new(Vec::new()),
                }
            }
        }
    };
}

//#endregion 🧬️ entity_dsl

//#region 🔌️ExternalAdapters
/// @emoji 🔌️ Sole import surface for third-party crates used by `semio_compose_rs` (domain code uses `crate::external_adapters::*`).
pub mod external_adapters {
    pub use async_broadcast;
    pub use async_channel;
    pub use async_executor;
    pub use async_graphql;
    pub use async_lock;
    pub use async_stream;
    pub use blake3;
    pub use futures_channel;
    pub use futures_lite;
    pub use futures_util;
    pub use hex;
    pub use nalgebra;
    pub use paste;
    pub use serde;
    pub use serde_json;
    pub use sha2;
    pub use thiserror;
    pub use uuid;
    #[cfg(target_arch = "wasm32")]
    pub use {base64, getrandom, js_sys, serde_wasm_bindgen, wasm_bindgen, wasm_bindgen_futures, web_sys};
    #[cfg(not(target_arch = "wasm32"))]
    pub use {chrono, rusqlite, tempfile, ureq, walkdir, zip};
}
//#endregion 🔌️ExternalAdapters

//#region 🆔️ id

pub mod id {
    //! 🆔️ Immutable uuid-v7 wrapper used by every entity.
    use crate::external_adapters::async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
    use std::fmt;

    /// @emoji 🆔️ Opaque node identifier (uuidv7 string); GraphQL wire name `ID` per target schema.
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Id(pub String);

    impl Id {
        /// 🆕️ Mint a fresh uuid-v7 (timestamped, monotonic).
        pub async fn new() -> Self {
            Self(crate::external_adapters::uuid::Uuid::now_v7().to_string())
        }

        pub(crate) fn new_sync() -> Self {
            Self(crate::external_adapters::uuid::Uuid::now_v7().to_string())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl From<String> for Id {
        fn from(s: String) -> Self {
            Self(s)
        }
    }

    impl From<&str> for Id {
        fn from(s: &str) -> Self {
            Self(s.to_string())
        }
    }

    impl From<&Id> for Id {
        fn from(id: &Id) -> Self {
            id.clone()
        }
    }

    /// @emoji 🆔️ Wire name `ID` matches relay + [`semio_compose_rs/client/schema/graphql/schema.golden.graphql`](../../../schema/graphql/schema.golden.graphql) `scalar`/Node ids.
    #[Scalar(name = "ID")]
    impl ScalarType for Id {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok(Self(s)),
                _ => Err(InputValueError::expected_type(value)),
            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }

    //#region 🔖️Dsl
    /// @emoji 🧬️ `Id` is a bare `pub String` newtype with no `serde`/derive machinery of its own (it's
    /// a GraphQL custom scalar, not a `dsl::Dsl*`-derivable record/enum), so it gets a small hand
    /// `dsl::DslField` bridge — binds as `Shape::Text`, mirroring `String`'s own blanket impl — rather
    /// than a derive, exactly like the `HashMap`/`Box` bridges other converted crates hand-write at
    /// their derive engine boundary.
    impl dsl::DslField for Id {
        fn shape() -> dsl::Shape {
            dsl::Shape::Text
        }
        fn to_value(&self) -> dsl::FieldValue {
            dsl::FieldValue::Text(self.0.clone())
        }
        fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
            match value {
                dsl::FieldValue::Text(s) => Ok(Self(s.clone())),
                other => Err(format!("expected Text, found {other:?}")),
            }
        }
    }
    //#endregion 🔖️Dsl
}

//#endregion 🆔️ id

//#region ⏱️ timestamp

pub mod timestamp {
    //! ⏱️ ISO-8601 millisecond-precision timestamp scalar.
    use crate::external_adapters::async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Timestamp(pub String);

    #[Scalar(name = "Timestamp")]
    impl ScalarType for Timestamp {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok(Self(s)),
                _ => Err(InputValueError::expected_type(value)),
            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }
}

//#endregion ⏱️ timestamp

//#region 🎨️ color

pub mod color {
    //! 🎨️ Hex/CSS color token scalar matching golden `scalar Color`.
    use crate::external_adapters::async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Color(pub String);

    #[Scalar(name = "Color")]
    impl ScalarType for Color {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok(Self(s)),
                _ => Err(InputValueError::expected_type(value)),
            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }
}

//#endregion 🎨️ color

//#region 🚨️ error

pub mod error {
    //! 🚨️ Crate-wide error type wired through the event bus as `OperationFailed`.
    use crate::external_adapters::async_graphql::Object;
    use crate::external_adapters::thiserror::Error;

    #[derive(Clone, Debug, Default, Error)]
    #[error("{kind}: {message}")]
    pub struct ComposeError {
        pub kind: String,
        pub message: String,
        pub request_id: Option<String>,
    }

    #[Object(name = "Error")]
    impl ComposeError {
        async fn id(&self) -> crate::id::Id {
            crate::id::Id(crate::hash::h(&[self.kind.as_str(), self.message.as_str()]))
        }
        async fn hash(&self) -> String {
            crate::hash::h(&[self.kind.as_str(), self.message.as_str()])
        }
        async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
            None
        }
        #[graphql(name = "owns")]
        async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
            Some(crate::gql::interfaces::empty_entity_connection())
        }
        async fn kind(&self) -> &str {
            &self.kind
        }
        async fn message(&self) -> &str {
            &self.message
        }
        #[graphql(name = "requestId")]
        async fn request_id_field(&self) -> Option<crate::id::Id> {
            self.request_id.as_ref().map(|s| crate::id::Id(s.clone()))
        }
    }

    impl ComposeError {
        pub fn invalid<S: Into<String>>(msg: S) -> Self {
            Self { kind: "Invalid".to_string(), message: msg.into(), request_id: None }
        }
        pub fn not_found<S: Into<String>>(kind: S, id: S) -> Self {
            Self { kind: "NotFound".to_string(), message: format!("{}({})", kind.into(), id.into()), request_id: None }
        }
        pub fn with_request(mut self, id: crate::id::Id) -> Self {
            self.request_id = Some(id.0);
            self
        }
    }

    pub type Result<T> = std::result::Result<T, ComposeError>;
}

//#endregion 🚨️ error

//#region 📐️ geom

pub mod geom {
    //! 📐️ Geometry: wire [`VectorInput`], [`PositionInput`], … for GraphQL kit inputs; canonical live weak entities live in [`entity`] as `Arc` graph nodes with one Rust kind per SDL weak entity.
    use crate::external_adapters::async_graphql::InputObject;

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "VectorInput")]
    pub struct VectorInput {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "PointInput")]
    pub struct PointInput {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "CoordinateInput")]
    pub struct CoordinateInput {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "OffsetInput")]
    pub struct OffsetInput {
        pub u: f64,
        pub v: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "PlaneInput")]
    pub struct PlaneInput {
        pub origin: PointInput,
        #[graphql(name = "xAxis")]
        #[dsl(key = "xAxis")]
        pub x_axis: VectorInput,
        #[graphql(name = "yAxis")]
        #[dsl(key = "yAxis")]
        pub y_axis: VectorInput,
    }

    impl Default for PlaneInput {
        /// @emoji ◭️ World XY plane through origin; hydrates kit JSON that omits plane axes.
        fn default() -> Self {
            Self { origin: PointInput::default(), x_axis: VectorInput { x: 1.0, y: 0.0, z: 0.0 }, y_axis: VectorInput { x: 0.0, y: 1.0, z: 0.0 } }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject, dsl::DslRecord)]
    #[graphql(name = "PositionInput")]
    pub struct PositionInput {
        pub center: CoordinateInput,
        pub plane: PlaneInput,
    }

    /// @emoji 🌍️ Wire `LocationInput` (lon/lat/alt) for [`entity::Location`].
    #[derive(Clone, Copy, Debug, Default, PartialEq, InputObject)]
    #[graphql(name = "LocationInput")]
    pub struct LocationInput {
        pub longitude: f64,
        pub latitude: f64,
        pub altitude: f64,
    }

    //#region 📐️ entity
    pub mod entity {
        //! 📐️ `Arc` geometry nodes (target WeakEntity / Entity graph shapes); `#[Object]` impls live after [`crate::interface`].
        use std::sync::Arc;

        use crate::external_adapters::async_lock::RwLock;

        use crate::hash::{h, merkle_node_str};
        use crate::id::Id;

        use super::{CoordinateInput, PlaneInput, PointInput, PositionInput, VectorInput};

        fn weak(prefix: &str, parts: &[&str]) -> Id {
            Id::from(format!("weak:{prefix}:{}", h(parts)))
        }

        /// @emoji 📍️ Canonical weak `Coordinate` (live u/v under `RwLock`).
        #[derive(Debug)]
        pub struct Coordinate {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl Coordinate {
            pub fn from_input(c: CoordinateInput) -> Arc<Self> {
                let id = weak("coordinate", &[&format!("{:.9}", c.u), &format!("{:.9}", c.v)]);
                Arc::new(Self { id, u: RwLock::new(c.u), v: RwLock::new(c.v) })
            }

            /// @emoji 🪪️ Merkle leaf: id + live u/v (matches [`super::CoordinateInput`] payload).
            pub async fn compute_hash(&self) -> String {
                let u = *self.u.read().await;
                let v = *self.v.read().await;
                merkle_node_str(&["Coordinate", self.id.as_str(), &format!("{u:.9}"), &format!("{v:.9}")], Vec::new())
            }
        }

        /// @emoji ↗️ Canonical weak `Vector`.
        #[derive(Debug)]
        pub struct Vector {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl Vector {
            pub fn from_input(v: VectorInput) -> Arc<Self> {
                let id = weak("vector", &[&format!("{:.9}", v.x), &format!("{:.9}", v.y), &format!("{:.9}", v.z)]);
                Arc::new(Self { id, x: RwLock::new(v.x), y: RwLock::new(v.y), z: RwLock::new(v.z) })
            }

            /// @emoji 🪪️ Merkle leaf: id + live x/y/z.
            pub async fn compute_hash(&self) -> String {
                let x = *self.x.read().await;
                let y = *self.y.read().await;
                let z = *self.z.read().await;
                merkle_node_str(&["Vector", self.id.as_str(), &format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")], Vec::new())
            }
        }

        /// @emoji ◆️ Canonical weak `Point`.
        #[derive(Debug)]
        pub struct Point {
            pub id: Id,
            pub x: RwLock<f64>,
            pub y: RwLock<f64>,
            pub z: RwLock<f64>,
        }

        impl Point {
            pub fn from_input(p: PointInput) -> Arc<Self> {
                let id = weak("point", &[&format!("{:.9}", p.x), &format!("{:.9}", p.y), &format!("{:.9}", p.z)]);
                Arc::new(Self { id, x: RwLock::new(p.x), y: RwLock::new(p.y), z: RwLock::new(p.z) })
            }

            /// @emoji 🪪️ Merkle leaf: id + live x/y/z.
            pub async fn compute_hash(&self) -> String {
                let x = *self.x.read().await;
                let y = *self.y.read().await;
                let z = *self.z.read().await;
                merkle_node_str(&["Point", self.id.as_str(), &format!("{x:.9}"), &format!("{y:.9}"), &format!("{z:.9}")], Vec::new())
            }
        }

        /// @emoji ▭️ Canonical weak `Plane` (owns origin + axes).
        #[derive(Debug)]
        pub struct Plane {
            pub id: Id,
            pub origin: Arc<Point>,
            pub x_axis: Arc<Vector>,
            pub y_axis: Arc<Vector>,
        }

        impl Plane {
            pub fn from_input(pl: PlaneInput) -> Arc<Self> {
                let origin = Point::from_input(pl.origin);
                let x_axis = Vector::from_input(pl.x_axis);
                let y_axis = Vector::from_input(pl.y_axis);
                let id = weak("plane", &[origin.id.as_str(), x_axis.id.as_str(), y_axis.id.as_str()]);
                Arc::new(Self { id, origin, x_axis, y_axis })
            }

            /// @emoji 🪪️ Merkle node: sorted child digests of origin + axes.
            pub async fn compute_hash(&self) -> String {
                let mut ch = vec![self.origin.compute_hash().await, self.x_axis.compute_hash().await, self.y_axis.compute_hash().await];
                ch.sort();
                merkle_node_str(&["Plane", self.id.as_str()], ch)
            }
        }

        /// @emoji ↖️ Canonical weak `Offset` (piece drag input echo).
        #[derive(Debug)]
        pub struct Offset {
            pub id: Id,
            pub u: RwLock<f64>,
            pub v: RwLock<f64>,
        }

        impl Offset {
            pub fn from_input(o: super::OffsetInput) -> Arc<Self> {
                let id = weak("offset", &[&format!("{:.9}", o.u), &format!("{:.9}", o.v)]);
                Arc::new(Self { id, u: RwLock::new(o.u), v: RwLock::new(o.v) })
            }

            /// @emoji 🪪️ Merkle leaf: id + live u/v.
            pub async fn compute_hash(&self) -> String {
                let u = *self.u.read().await;
                let v = *self.v.read().await;
                merkle_node_str(&["Offset", self.id.as_str(), &format!("{u:.9}"), &format!("{v:.9}")], Vec::new())
            }
        }

        /// @emoji ⌖️ Canonical weak `Position` (center + plane); live state only in child locks.
        #[derive(Debug)]
        pub struct Position {
            pub id: Id,
            pub center: Arc<Coordinate>,
            pub plane: Arc<Plane>,
        }

        impl Position {
            pub fn from_position_input(value: PositionInput) -> Arc<Self> {
                let center = Coordinate::from_input(value.center);
                let plane = Plane::from_input(value.plane);
                let id = weak("position", &[center.id.as_str(), plane.id.as_str()]);
                Arc::new(Self { id, center, plane })
            }

            /// @emoji 📸️ Wire [`super::PositionInput`] from live center + plane child locks (single source of truth).
            pub async fn snapshot_input(&self) -> PositionInput {
                let u = *self.center.u.read().await;
                let v = *self.center.v.read().await;
                let ox = *self.plane.origin.x.read().await;
                let oy = *self.plane.origin.y.read().await;
                let oz = *self.plane.origin.z.read().await;
                let xx = *self.plane.x_axis.x.read().await;
                let xy = *self.plane.x_axis.y.read().await;
                let xz = *self.plane.x_axis.z.read().await;
                let yx = *self.plane.y_axis.x.read().await;
                let yy = *self.plane.y_axis.y.read().await;
                let yz = *self.plane.y_axis.z.read().await;
                PositionInput { center: CoordinateInput { u, v }, plane: PlaneInput { origin: PointInput { x: ox, y: oy, z: oz }, x_axis: VectorInput { x: xx, y: xy, z: xz }, y_axis: VectorInput { x: yx, y: yy, z: yz } } }
            }

            /// @emoji 🪪️ Merkle node: live position scalars plus sorted digests of center + plane arcs.
            pub async fn compute_hash(&self) -> String {
                let p = self.snapshot_input().await;
                let flat = format!(
                    "{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}\x1f{:.9}",
                    p.center.u, p.center.v, p.plane.origin.x, p.plane.origin.y, p.plane.origin.z, p.plane.x_axis.x, p.plane.x_axis.y, p.plane.x_axis.z, p.plane.y_axis.x, p.plane.y_axis.y, p.plane.y_axis.z,
                );
                let mut ch = vec![self.center.compute_hash().await, self.plane.compute_hash().await];
                ch.sort();
                merkle_node_str(&["Position", self.id.as_str(), flat.as_str()], ch)
            }
        }

        /// @emoji 🌍️ Canonical weak `Location` (lon/lat/alt).
        #[derive(Debug)]
        pub struct Location {
            pub id: Id,
            pub longitude: RwLock<f64>,
            pub latitude: RwLock<f64>,
            pub altitude: RwLock<f64>,
        }

        impl Location {
            pub fn from_location_input(loc: super::LocationInput) -> Arc<Self> {
                let id = weak("location", &[&format!("{:.9}", loc.longitude), &format!("{:.9}", loc.latitude), &format!("{:.9}", loc.altitude)]);
                Arc::new(Self { id, longitude: RwLock::new(loc.longitude), latitude: RwLock::new(loc.latitude), altitude: RwLock::new(loc.altitude) })
            }

            /// @emoji 🪪️ Merkle leaf over lon/lat/alt fields.
            pub async fn compute_hash(&self) -> String {
                let lo = *self.longitude.read().await;
                let la = *self.latitude.read().await;
                let al = *self.altitude.read().await;
                merkle_node_str(&["Location", self.id.as_str(), &format!("{lo:.9}"), &format!("{la:.9}"), &format!("{al:.9}")], Vec::new())
            }
        }

        /// @emoji 🧭️ Placeholder shell for `Place` (full meta wiring lands with meta lift).
        #[derive(Debug)]
        pub struct Place {
            pub id: Id,
            pub label: RwLock<Option<String>>,
        }

        impl Place {
            pub async fn new() -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, label: RwLock::new(None) })
            }

            /// @emoji 🪪️ Merkle leaf: id + optional label.
            pub async fn compute_hash(&self) -> String {
                let lb = self.label.read().await.clone().unwrap_or_default();
                merkle_node_str(&["Place", self.id.as_str(), lb.as_str()], Vec::new())
            }
        }

        impl Default for Coordinate {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for Vector {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for Point {
            fn default() -> Self {
                Self { id: Id::default(), x: RwLock::new(0.0), y: RwLock::new(0.0), z: RwLock::new(0.0) }
            }
        }

        impl Default for Plane {
            fn default() -> Self {
                Self { id: Id::default(), origin: Arc::new(Point::default()), x_axis: Arc::new(Vector::default()), y_axis: Arc::new(Vector::default()) }
            }
        }

        impl Default for Offset {
            fn default() -> Self {
                Self { id: Id::default(), u: RwLock::new(0.0), v: RwLock::new(0.0) }
            }
        }

        impl Default for Position {
            fn default() -> Self {
                Self { id: Id::default(), center: Arc::new(Coordinate::default()), plane: Arc::new(Plane::default()) }
            }
        }

        impl Default for Location {
            fn default() -> Self {
                Self { id: Id::default(), longitude: RwLock::new(0.0), latitude: RwLock::new(0.0), altitude: RwLock::new(0.0) }
            }
        }

        impl Default for Place {
            fn default() -> Self {
                Self { id: Id::default(), label: RwLock::new(None) }
            }
        }
        //#endregion
    }
    //#endregion 📐️ entity

    //#region 🌤️FlattenDesign
    /// @emoji 🌤️ Computes absolute piece planes and centers from relative connections.
    pub mod flatten {
        use std::collections::{HashMap, HashSet, VecDeque};
        use std::sync::Arc;

        use crate::geom::{CoordinateInput, PlaneInput, PointInput, PositionInput, VectorInput};
        use crate::id::Id;
        use crate::kit::design::connection::Connection;
        use crate::kit::design::piece::Piece;
        use crate::kit::design::Design;
        use crate::kit::r#type::{Connector, Type};
        use crate::kit::Kit;

        const TOLERANCE: f64 = 0.01;
        const DIAGRAM_RADIUS: f64 = 2.697;
        const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;
        const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633;

        fn normalize(v: &mut [f64; 3]) {
            let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if len > 0.0 {
                v[0] /= len;
                v[1] /= len;
                v[2] /= len;
            }
        }

        fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
            [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
        }

        fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
            a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
        }

        fn deg_to_rad(deg: f64) -> f64 {
            deg * std::f64::consts::PI / 180.0
        }

        fn round_f(v: f64) -> f64 {
            (v * 1_000_000.0).round() / 1_000_000.0
        }

        fn plane_input_to_matrix(p: PlaneInput) -> [f64; 16] {
            let x = [p.x_axis.x, p.x_axis.y, p.x_axis.z];
            let y = [p.y_axis.x, p.y_axis.y, p.y_axis.z];
            let z = cross(x, y);
            [x[0], y[0], z[0], p.origin.x, x[1], y[1], z[1], p.origin.y, x[2], y[2], z[2], p.origin.z, 0.0, 0.0, 0.0, 1.0]
        }

        fn matrix_to_plane(m: [f64; 16]) -> PlaneInput {
            PlaneInput { origin: PointInput { x: m[3], y: m[7], z: m[11] }, x_axis: VectorInput { x: m[0], y: m[4], z: m[8] }, y_axis: VectorInput { x: m[1], y: m[5], z: m[9] } }
        }

        fn mul_mat(a: [f64; 16], b: [f64; 16]) -> [f64; 16] {
            let mut out = [0.0; 16];
            for col in 0..4 {
                for row in 0..4 {
                    out[col * 4 + row] = a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
                }
            }
            out
        }

        fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
            [1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0]
        }

        fn rotation_axis(axis: [f64; 3], angle: f64) -> [f64; 16] {
            let (x, y, z) = (axis[0], axis[1], axis[2]);
            let c = angle.cos();
            let s = angle.sin();
            let t = 1.0 - c;
            [t * x * x + c, t * x * y + s * z, t * x * z - s * y, 0.0, t * x * y - s * z, t * y * y + c, t * y * z + s * x, 0.0, t * x * z + s * y, t * y * z - s * x, t * z * z + c, 0.0, 0.0, 0.0, 0.0, 1.0]
        }

        fn apply_mat_vec3(m: [f64; 16], v: [f64; 3]) -> [f64; 3] {
            [m[0] * v[0] + m[4] * v[1] + m[8] * v[2], m[1] * v[0] + m[5] * v[1] + m[9] * v[2], m[2] * v[0] + m[6] * v[1] + m[10] * v[2]]
        }

        fn quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
            let r = dot(from, to) + 1.0;
            let quat = if r < 0.000_001 {
                if from[0].abs() > from[2].abs() {
                    [-from[1], from[0], 0.0, 0.0]
                } else {
                    [0.0, -from[2], from[1], 0.0]
                }
            } else {
                let c = cross(from, to);
                [c[0], c[1], c[2], r]
            };
            let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
            [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
        }

        fn quaternion_to_matrix(q: [f64; 4]) -> [f64; 16] {
            let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
            let (x2, y2, z2) = (x + x, y + y, z + z);
            let (xx, xy, xz) = (x * x2, x * y2, x * z2);
            let (yy, yz, zz) = (y * y2, y * z2, z * z2);
            let (wx, wy, wz) = (w * x2, w * y2, w * z2);
            [1.0 - (yy + zz), xy + wz, xz - wy, 0.0, xy - wz, 1.0 - (xx + zz), yz + wx, 0.0, xz + wy, yz - wx, 1.0 - (xx + yy), 0.0, 0.0, 0.0, 0.0, 1.0]
        }

        async fn connector_geom(c: &Arc<Connector>) -> (PointInput, VectorInput, f64) {
            let point = *c.point.read().await;
            let mut direction = *c.direction.read().await;
            let mut dir = [direction.x, direction.y, direction.z];
            normalize(&mut dir);
            direction = VectorInput { x: dir[0], y: dir[1], z: dir[2] };
            let t = c.t_param.read().await.unwrap_or(0.0);
            (point, direction, t)
        }

        async fn compute_child_plane(parent_plane: PlaneInput, parent_connector: &Arc<Connector>, child_connector: &Arc<Connector>, connection: &Arc<Connection>) -> PlaneInput {
            let parent_matrix = plane_input_to_matrix(parent_plane);
            let (parent_point, parent_direction, _) = connector_geom(parent_connector).await;
            let (child_point, child_direction, _) = connector_geom(child_connector).await;
            let mut parent_dir = [parent_direction.x, parent_direction.y, parent_direction.z];
            let mut child_dir = [child_direction.x, child_direction.y, child_direction.z];
            normalize(&mut parent_dir);
            normalize(&mut child_dir);
            let gap = connection.gap.read().await.unwrap_or(0.0);
            let shift = connection.shift.read().await.unwrap_or(0.0);
            let rise = connection.rise.read().await.unwrap_or(0.0);
            let rotation_rad = deg_to_rad(connection.rotation.read().await.unwrap_or(0.0));
            let turn_rad = deg_to_rad(connection.turn.read().await.unwrap_or(0.0));
            let tilt_rad = deg_to_rad(connection.tilt.read().await.unwrap_or(0.0));
            let reverse_child = [-child_dir[0], -child_dir[1], -child_dir[2]];
            let cross_vec = cross(parent_dir, reverse_child);
            let cross_len = (cross_vec[0] * cross_vec[0] + cross_vec[1] * cross_vec[1] + cross_vec[2] * cross_vec[2]).sqrt();
            let align_quat = if cross_len < TOLERANCE {
                if parent_dir[2].abs() < TOLERANCE {
                    quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
                } else {
                    let mut axis = cross([0.0, 0.0, 1.0], parent_dir);
                    normalize(&mut axis);
                    let half = std::f64::consts::PI / 2.0;
                    [axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()]
                }
            } else {
                quaternion_from_unit_vectors(reverse_child, parent_dir)
            };
            let direction_t = quaternion_to_matrix(align_quat);
            let y_axis = [0.0, 1.0, 0.0];
            let parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors(y_axis, parent_dir));
            let gap_direction = apply_mat_vec3(parent_rotation_t, [0.0, 1.0, 0.0]);
            let shift_direction = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
            let raise_direction = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
            let mut turn_axis = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
            let mut tilt_axis = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
            let mut orientation_t = direction_t;
            let rotate_t = rotation_axis(parent_dir, -rotation_rad);
            orientation_t = mul_mat(rotate_t, orientation_t);
            turn_axis = apply_mat_vec3(rotate_t, turn_axis);
            tilt_axis = apply_mat_vec3(rotate_t, tilt_axis);
            orientation_t = mul_mat(rotation_axis(turn_axis, turn_rad), orientation_t);
            orientation_t = mul_mat(rotation_axis(tilt_axis, tilt_rad), orientation_t);
            let center_child_t = translation(-child_point.x, -child_point.y, -child_point.z);
            let mut transform = mul_mat(orientation_t, center_child_t);
            let gap_transform = translation(gap_direction[0] * gap, gap_direction[1] * gap, gap_direction[2] * gap);
            let shift_transform = translation(shift_direction[0] * shift, shift_direction[1] * shift, shift_direction[2] * shift);
            let raise_transform = translation(raise_direction[0] * rise, raise_direction[1] * rise, raise_direction[2] * rise);
            transform = mul_mat(mul_mat(raise_transform, mul_mat(shift_transform, gap_transform)), transform);
            transform = mul_mat(translation(parent_point.x, parent_point.y, parent_point.z), transform);
            matrix_to_plane(mul_mat(parent_matrix, transform))
        }

        async fn resolve_connector(ty: Option<&Arc<Type>>, connector_id: Option<&Id>, kit: &Arc<Kit>) -> Option<Arc<Connector>> {
            if let Some(id) = connector_id {
                if let Some(c) = kit.find_connector(id).await {
                    return Some(c);
                }
                if let Some(t) = ty {
                    for c in t.has_connectors().await {
                        if &c.id == id {
                            return Some(c);
                        }
                    }
                }
            }
            if let Some(t) = ty {
                return t.has_connectors().await.into_iter().next();
            }
            None
        }

        async fn piece_stored_position(piece: &Arc<Piece>) -> Option<PositionInput> {
            if let Some(n) = piece.position.read().await.as_ref() {
                return Some(n.snapshot_input().await);
            }
            None
        }

        async fn piece_is_fixed(piece: &Arc<Piece>) -> bool {
            matches!(*piece.connection_kind.read().await, Some(crate::kit::design::piece::PieceConnectionKind::Fixed))
        }

        /// @emoji 🌤️ Absolute positions for every piece in a design.
        pub async fn flatten_design_positions(kit: &Arc<Kit>, design: &Arc<Design>) -> HashMap<Id, PositionInput> {
            let pieces = design.has_pieces().await;
            if pieces.is_empty() {
                return HashMap::new();
            }
            let mut piece_map: HashMap<String, Arc<Piece>> = HashMap::new();
            for p in &pieces {
                piece_map.insert(p.id.as_str().to_string(), p.clone());
            }
            let connections = design.has_connections().await;
            let mut adjacency: HashMap<String, Vec<(String, Arc<Connection>)>> = HashMap::new();
            for conn in &connections {
                let parent_id = conn.parent.read().await.references_piece().await.id.as_str().to_string();
                let child_id = conn.child.read().await.references_piece().await.id.as_str().to_string();
                if piece_map.contains_key(&parent_id) && piece_map.contains_key(&child_id) {
                    adjacency.entry(parent_id.clone()).or_default().push((child_id.clone(), conn.clone()));
                    adjacency.entry(child_id.clone()).or_default().push((parent_id.clone(), conn.clone()));
                }
            }
            let mut original_centers: HashMap<String, CoordinateInput> = HashMap::new();
            for p in &pieces {
                if let Some(pos) = piece_stored_position(p).await {
                    original_centers.insert(p.id.as_str().to_string(), pos.center);
                }
            }
            let mut piece_planes: HashMap<String, PlaneInput> = HashMap::new();
            let mut piece_centers: HashMap<String, CoordinateInput> = HashMap::new();
            let mut visited: HashSet<String> = HashSet::new();

            async fn bfs_root(
                root_id: &str,
                piece_map: &HashMap<String, Arc<Piece>>,
                adjacency: &HashMap<String, Vec<(String, Arc<Connection>)>>,
                kit: &Arc<Kit>,
                visited: &mut HashSet<String>,
                piece_planes: &mut HashMap<String, PlaneInput>,
                piece_centers: &mut HashMap<String, CoordinateInput>,
                _original_centers: &HashMap<String, CoordinateInput>,
            ) {
                let mut queue: VecDeque<String> = VecDeque::new();
                queue.push_back(root_id.to_string());
                visited.insert(root_id.to_string());
                let root_piece = piece_map.get(root_id).expect("root_id is drawn from the same `pieces` list piece_map was populated from");
                if let Some(pos) = piece_stored_position(root_piece).await {
                    if piece_is_fixed(root_piece).await {
                        piece_planes.insert(root_id.to_string(), pos.plane);
                        piece_centers.insert(root_id.to_string(), pos.center);
                    } else {
                        piece_planes.insert(root_id.to_string(), PlaneInput::default());
                        piece_centers.insert(root_id.to_string(), pos.center);
                    }
                } else {
                    piece_planes.insert(root_id.to_string(), PlaneInput::default());
                    piece_centers.insert(root_id.to_string(), CoordinateInput::default());
                }
                while let Some(current_id) = queue.pop_front() {
                    let current_plane = *piece_planes.get(&current_id).unwrap_or(&PlaneInput::default());
                    let current_piece = piece_map.get(&current_id).expect("queue only ever holds ids sourced from piece_map/adjacency").clone();
                    let parent_center = piece_centers.get(&current_id).copied().unwrap_or_default();
                    for (neighbor_id, conn) in adjacency.get(&current_id).into_iter().flatten() {
                        if visited.contains(neighbor_id) {
                            continue;
                        }
                        visited.insert(neighbor_id.clone());
                        let neighbor_piece = piece_map.get(neighbor_id).expect("adjacency only links ids already verified present in piece_map").clone();
                        let parent_side = conn.parent.read().await.clone();
                        let child_side = conn.child.read().await.clone();
                        let (parent_piece_id, _child_piece_id) = (parent_side.references_piece().await.id.as_str().to_string(), child_side.references_piece().await.id.as_str().to_string());
                        let (parent_side_ref, child_side_ref) = if parent_piece_id == current_id { (&parent_side, &child_side) } else { (&child_side, &parent_side) };
                        let parent_ty = current_piece.is_type().await;
                        let child_ty = neighbor_piece.is_type().await;
                        let parent_connector = resolve_connector(parent_ty.as_ref(), parent_side_ref.references_connector().await.as_ref().map(|c| &c.id), kit).await;
                        let child_connector = resolve_connector(child_ty.as_ref(), child_side_ref.references_connector().await.as_ref().map(|c| &c.id), kit).await;
                        let (Some(parent_connector), Some(child_connector)) = (parent_connector, child_connector) else {
                            piece_planes.insert(neighbor_id.clone(), PlaneInput::default());
                            piece_centers.insert(neighbor_id.clone(), CoordinateInput::default());
                            queue.push_back(neighbor_id.clone());
                            continue;
                        };
                        let child_plane = compute_child_plane(current_plane, &parent_connector, &child_connector, conn).await;
                        piece_planes.insert(neighbor_id.clone(), child_plane);
                        let (_, parent_direction, parent_t) = connector_geom(&parent_connector).await;
                        let connection_u = conn.u.read().await.unwrap_or(0.0);
                        let connection_v = conn.v.read().await.unwrap_or(0.0);
                        let (child_u, child_v) = if parent_center.u == 0.0 && parent_center.v == 0.0 {
                            let angle = 2.0 * std::f64::consts::PI * parent_t;
                            (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
                        } else if parent_direction.z.abs() > 0.5 {
                            (parent_center.u + connection_u, parent_center.v + connection_v + DIAGRAM_VERTICAL_V_EXTRA)
                        } else {
                            (parent_center.u + connection_u * DIAGRAM_HORIZONTAL_SCALE, parent_center.v + connection_v * DIAGRAM_HORIZONTAL_SCALE)
                        };
                        piece_centers.insert(neighbor_id.clone(), CoordinateInput { u: round_f(child_u), v: round_f(child_v) });
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }

            for p in &pieces {
                let pid = p.id.as_str().to_string();
                if !visited.contains(&pid) {
                    bfs_root(&pid, &piece_map, &adjacency, kit, &mut visited, &mut piece_planes, &mut piece_centers, &original_centers).await;
                }
            }
            let mut out = HashMap::new();
            for p in &pieces {
                let pid = p.id.clone();
                let plane = piece_planes.get(p.id.as_str()).copied().unwrap_or_default();
                let center = piece_centers.get(p.id.as_str()).copied().or_else(|| original_centers.get(p.id.as_str()).copied()).unwrap_or_default();
                out.insert(pid, PositionInput { center, plane });
            }
            out
        }
    }
    //#endregion 🌤️FlattenDesign
}

//#endregion 📐️ geom

//#region 🪢️ gql_relay

/// 🪢️ Relay `PageInfo` + connection shells for static GraphQL (edges, pageInfo, hash).
pub mod gql_relay {
    use std::sync::Arc;

    use crate::external_adapters::async_graphql::{Object, SimpleObject};
    use crate::external_adapters::async_lock::RwLock;

    use crate::hash::{h, merkle_collection};
    use crate::id::Id;
    use crate::kit::design::connection::Side;
    use crate::kit::design::piece::Piece;
    use crate::kit::design::Design;
    use crate::kit::r#type::{Connector, Representation, Type};
    use crate::meta::{Author, Benchmark, Concept, File, Folder, Group, Layer, Prop, Quality, Stat, Tag};
    use crate::vcs::{Alternative, Change, Checkpoint, Conflict};

    pub(crate) fn edge_cursor(i: usize) -> String {
        format!("e{i}")
    }

    #[derive(Clone, Debug, Default, SimpleObject)]
    pub struct PageInfo {
        #[graphql(name = "hasNextPage")]
        pub has_next_page: bool,
        #[graphql(name = "hasPreviousPage")]
        pub has_previous_page: bool,
        #[graphql(name = "startCursor")]
        pub start_cursor: Option<String>,
        #[graphql(name = "endCursor")]
        pub end_cursor: Option<String>,
    }

    /// @emoji 🪢️ Golden `PageInfoEdge` / `PageInfoConnection` (relay shells around [`PageInfo`]).
    #[derive(Clone, Debug, Default, SimpleObject)]
    #[graphql(name = "PageInfoEdge")]
    pub struct PageInfoEdge {
        pub cursor: String,
        pub node: Arc<PageInfo>,
    }

    #[derive(Clone, Debug, Default, SimpleObject)]
    #[graphql(name = "PageInfoConnection")]
    pub struct PageInfoConnection {
        pub edges: Vec<PageInfoEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: Arc<PageInfo>,
        pub hash: String,
    }

    impl PageInfoConnection {
        /// @emoji 🪢️ Golden `PageInfoConnection` with no edges — valid `EntityConnection` implementor for empty `owns` shells.
        pub fn empty_entity_shell() -> Self {
            Self { edges: vec![], page_info: Arc::new(PageInfo::default()), hash: h(&["EntityConnection", "PageInfo", "empty"]) }
        }
    }

    crate::entity_relay!(DesignConnection, DesignEdge, Arc<Design>);
    impl DesignConnection {
        pub async fn from_designs(entities: Vec<Arc<Design>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(PieceConnection, PieceEdge, Arc<Piece>);
    impl PieceConnection {
        pub async fn from_pieces(entities: Vec<Arc<Piece>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(TypeConnection, TypeEdge, Arc<Type>);
    impl TypeConnection {
        pub async fn from_types(entities: Vec<Arc<Type>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(ConnectorConnection, ConnectorEdge, Arc<Connector>);
    impl ConnectorConnection {
        pub async fn from_connectors(entities: Vec<Arc<Connector>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(RepresentationConnection, RepresentationEdge, Arc<Representation>);
    impl RepresentationConnection {
        pub async fn from_representations(entities: Vec<Arc<Representation>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(SideConnection, SideEdge, Arc<Side>);
    impl SideConnection {
        pub async fn from_sides(entities: Vec<Arc<Side>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct BlueprintEdge {
        pub cursor: String,
        pub node: crate::kit::r#type::Blueprint,
    }

    #[derive(Clone, SimpleObject)]
    pub struct BlueprintConnection {
        pub edges: Vec<BlueprintEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl BlueprintConnection {
        pub async fn from_blueprints(entities: Vec<crate::kit::r#type::Blueprint>) -> Self {
            let mut child_hashes = Vec::with_capacity(entities.len());
            for b in &entities {
                let h = match b {
                    crate::kit::r#type::Blueprint::Type(t) => t.compute_hash().await,
                    crate::kit::r#type::Blueprint::Design(d) => d.compute_hash().await,
                };
                child_hashes.push(h);
            }
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, node)| BlueprintEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    crate::entity_relay!(ConflictConnection, ConflictEdge, Arc<Conflict>);
    impl ConflictConnection {
        pub async fn from_conflicts(entities: Vec<Arc<Conflict>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(AlternativeConnection, AlternativeEdge, Arc<Alternative>);
    impl AlternativeConnection {
        pub async fn from_alternatives(entities: Vec<Arc<Alternative>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    crate::entity_relay!(ChangeConnection, ChangeEdge, Arc<Change>);
    impl ChangeConnection {
        pub async fn from_changes(entities: Vec<Arc<Change>>) -> Self {
            Self::from_entities(entities).await
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    crate::entity_relay!(EditConnection, EditEdge, Arc<crate::vcs::Edit>);
    impl EditConnection {
        pub async fn from_edits(entities: Vec<Arc<crate::vcs::Edit>>) -> Self {
            Self::from_entities(entities).await
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationEdge {
        pub cursor: String,
        pub node: Arc<crate::operation::OperationInterface>,
    }

    #[derive(Clone, SimpleObject)]
    pub struct OperationConnection {
        pub edges: Vec<OperationEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl OperationConnection {
        pub fn from_interface_entities(entities: Vec<Arc<crate::operation::OperationInterface>>) -> Self {
            let child_hashes: Vec<String> = entities.iter().map(|o| h(&[o.entity_id().as_str()])).collect();
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, o)| OperationEdge { cursor: edge_cursor(i), node: o }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }

        pub fn empty() -> Self {
            Self { edges: Vec::new(), page_info: std::sync::Arc::new(PageInfo::default()), hash: merkle_collection(Vec::new()) }
        }
    }

    crate::entity_relay!(CheckpointConnection, CheckpointEdge, Arc<Checkpoint>);
    impl CheckpointConnection {
        pub async fn from_checkpoints(entities: Vec<Arc<Checkpoint>>) -> Self {
            Self::from_entities(entities).await
        }
    }

    #[derive(Clone, SimpleObject)]
    #[graphql(name = "ConnectionEdge")]
    pub struct ConnectionEdge {
        pub cursor: String,
        pub node: Arc<crate::kit::design::connection::Connection>,
    }

    #[derive(Clone, SimpleObject)]
    #[graphql(name = "ConnectionConnection")]
    pub struct ConnectionConnection {
        pub edges: Vec<ConnectionEdge>,
        #[graphql(name = "pageInfo")]
        pub page_info: std::sync::Arc<PageInfo>,
        pub hash: String,
    }

    impl ConnectionConnection {
        pub async fn from_connections(entities: Vec<Arc<crate::kit::design::connection::Connection>>) -> Self {
            let mut child_hashes = Vec::with_capacity(entities.len());
            for c in &entities {
                child_hashes.push(c.compute_hash().await);
            }
            let hash = merkle_collection(child_hashes);
            let edges = entities.into_iter().enumerate().map(|(i, node)| ConnectionEdge { cursor: edge_cursor(i), node }).collect();
            Self { edges, page_info: std::sync::Arc::new(PageInfo::default()), hash }
        }
    }

    crate::entity_relay_sync!(FileConnection, FileEdge, File, |f: &File| f.compute_entity_hash());
    crate::entity_relay_sync!(FolderConnection, FolderEdge, Folder, |f: &Folder| f.compute_entity_hash());
    crate::entity_relay_sync!(AuthorConnection, AuthorEdge, Author, |a: &Author| a.compute_entity_hash());
    crate::entity_relay!(ConceptConnection, ConceptEdge, std::sync::Arc<Concept>);
    crate::entity_relay!(TagConnection, TagEdge, std::sync::Arc<Tag>);
    crate::entity_relay!(QualityConnection, QualityEdge, std::sync::Arc<Quality>);
    crate::entity_relay!(PortConnection, PortEdge, std::sync::Arc<crate::kit::r#type::Port>);
    crate::entity_relay!(PlaceConnection, PlaceEdge, std::sync::Arc<crate::geom::entity::Place>);
    crate::entity_relay_sync!(BenchmarkConnection, BenchmarkEdge, Benchmark, |b: &Benchmark| b.compute_entity_hash());
    crate::entity_relay_sync!(PropConnection, PropEdge, Prop, |p: &Prop| p.compute_entity_hash());
    crate::entity_relay_sync!(AttributeConnection, AttributeEdge, crate::meta::Attribute, |a: &crate::meta::Attribute| a.compute_entity_hash());
    crate::entity_relay_sync!(StatConnection, StatEdge, Stat, |s: &Stat| s.compute_entity_hash());
    crate::entity_relay_sync!(LayerConnection, LayerEdge, Layer, |l: &Layer| l.compute_entity_hash());
    crate::entity_relay_sync!(GroupConnection, GroupEdge, Group, |g: &Group| g.compute_entity_hash());

    crate::entity_full_family!(Vector, Arc<crate::geom::entity::Vector>, relay = (VectorConnection, VectorEdge));
    crate::entity_full_family!(Point, Arc<crate::geom::entity::Point>, relay = (PointConnection, PointEdge));
    crate::entity_full_family!(Coordinate, Arc<crate::geom::entity::Coordinate>, relay = (CoordinateConnection, CoordinateEdge));
    crate::entity_full_family!(Offset, Arc<crate::geom::entity::Offset>, relay = (OffsetConnection, OffsetEdge));
    crate::entity_full_family!(Plane, Arc<crate::geom::entity::Plane>, relay = (PlaneConnection, PlaneEdge));
    crate::entity_full_family!(Position, Arc<crate::geom::entity::Position>, relay = (PositionConnection, PositionEdge));
    crate::entity_full_family!(Location, Arc<crate::geom::entity::Location>, relay = (LocationConnection, LocationEdge));

    crate::entity_family! {
        /// @emoji 🧷️ Kit [`Family`] SDL shell — Artifact [`name`]/[`description`]/[`icon`] are persisted kit fields.
        pub struct Family {
            pub id: Id,
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub folder_id: Option<Id>,
            #[graphql(skip)]
            pub owner_kit: std::sync::Weak<crate::kit::Kit>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "Family",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.icon.as_deref().unwrap_or(""),
                    this.folder_id.as_ref().map(|id| id.as_str()).unwrap_or(""),
                ],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_kit.upgrade().map(crate::gql::interfaces::EntityInterface::Kit)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                None
            }
        )
        , vfs = Family
    }

    crate::entity_relay_sync!(FamilyConnection, FamilyEdge, Family, |f: &Family| f.compute_entity_hash());

    //#region 🏛️ typology
    /// @emoji 🏛️ Kit [`Typology`] — owns [`Type`] and [`Design`] entities; [`Family`] stays at kit root for port compatibility.
    pub struct Typology {
        pub id: Id,
        pub name: RwLock<String>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub folder_id: RwLock<Option<Id>>,
        pub owner_kit: std::sync::Weak<crate::kit::Kit>,
        pub types: RwLock<Vec<std::sync::Arc<crate::kit::r#type::Type>>>,
        pub designs: RwLock<Vec<std::sync::Arc<crate::kit::design::Design>>>,
    }

    impl Default for Typology {
        fn default() -> Self {
            Self {
                id: Id::default(),
                name: RwLock::new(String::new()),
                description: RwLock::new(None),
                icon: RwLock::new(None),
                folder_id: RwLock::new(None),
                owner_kit: std::sync::Weak::new(),
                types: RwLock::new(Vec::new()),
                designs: RwLock::new(Vec::new()),
            }
        }
    }

    impl Typology {
        pub async fn new(owner_kit: std::sync::Weak<crate::kit::Kit>, name: String) -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self { id: Id::new().await, name: RwLock::new(name), owner_kit, ..Default::default() })
        }

        pub async fn new_with_external_id(owner_kit: std::sync::Weak<crate::kit::Kit>, id: Id, name: String) -> std::sync::Arc<Self> {
            std::sync::Arc::new(Self { id, name: RwLock::new(name), owner_kit, ..Default::default() })
        }

        pub async fn compute_entity_hash(&self) -> String {
            let name = self.name.read().await;
            let desc = self.description.read().await;
            let icon = self.icon.read().await;
            let folder_id = self.folder_id.read().await;
            crate::hash::merkle_node_str(&["Typology", self.id.as_str(), name.as_str(), desc.as_deref().unwrap_or(""), icon.as_deref().unwrap_or(""), folder_id.as_ref().map(|id| id.as_str()).unwrap_or("")], Vec::new())
        }

        pub async fn compute_hash(&self) -> String {
            self.compute_entity_hash().await
        }
    }

    #[Object(name = "Typology")]
    impl Typology {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_entity_hash().await
        }
        pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
            self.owner_kit.upgrade().map(crate::gql::interfaces::EntityInterface::Kit)
        }
        pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
            None
        }
        pub async fn name(&self) -> String {
            self.name.read().await.clone()
        }
        pub async fn description(&self) -> Option<String> {
            self.description.read().await.clone()
        }
        pub async fn icon(&self) -> Option<String> {
            self.icon.read().await.clone()
        }
        #[graphql(name = "folderId")]
        pub async fn folder_id(&self) -> Option<Id> {
            self.folder_id.read().await.clone()
        }
        /// @emoji 🧰️ Kinds owned by this typology.
        #[graphql(name = "hasTypes")]
        pub async fn has_types(&self) -> TypeConnection {
            TypeConnection::from_types(self.types.read().await.clone()).await
        }
        /// @emoji 🏘️ Designs owned by this typology.
        #[graphql(name = "hasDesigns")]
        pub async fn has_designs(&self) -> DesignConnection {
            DesignConnection::from_designs(self.designs.read().await.clone()).await
        }
    }

    crate::file_system_node_vfs_complex_ctx!(Typology, crate::gql::interfaces::file_system_vfs::node_for_typology);

    crate::entity_relay!(TypologyConnection, TypologyEdge, std::sync::Arc<Typology>);
    impl TypologyConnection {
        pub async fn from_typologies(entities: Vec<std::sync::Arc<Typology>>) -> Self {
            Self::from_entities(entities).await
        }
    }
    //#endregion 🏛️ typology
}

//#endregion 🪢️ gql_relay

//#region 🩹️ schema_gap_surfaces

pub mod schema_gap_surfaces {
    //! 🩹️ SDL-only synthetic relay surfaces for long-tail golden declarations; registered into `Schema::sdl()` so the exported schema reaches the current target declaration set.

    use std::sync::Arc;

    use crate::external_adapters::async_graphql::SimpleObject;

    use crate::gql_relay::PageInfo;

    macro_rules! gap_surface_family {
        ($Name:ident) => {
            #[derive(Clone, Debug, Default, SimpleObject)]
            pub struct $Name {
                pub hash: String,
            }

            crate::external_adapters::paste::paste! {
                #[derive(Clone, Debug, Default, SimpleObject)]
                pub struct [<$Name Edge>] {
                    pub cursor: String,
                    pub node: $Name,
                }

                #[derive(Clone, Debug, SimpleObject)]
                pub struct [<$Name Connection>] {
                    pub edges: Vec<[<$Name Edge>]>,
                    #[graphql(name = "pageInfo")]
                    pub page_info: Arc<PageInfo>,
                    pub hash: String,
                }

                impl Default for [<$Name Connection>] {
                    fn default() -> Self {
                        Self {
                            edges: Vec::new(),
                            page_info: Arc::new(PageInfo::default()),
                            hash: String::new(),
                        }
                    }
                }
            }
        };
    }

    macro_rules! gap_surface_family_named {
        (
            $base_name:literal,
            $BaseRust:ident,
            $edge_name:literal,
            $EdgeRust:ident,
            $conn_name:literal,
            $ConnRust:ident
        ) => {
            #[derive(Clone, Debug, Default, SimpleObject)]
            #[graphql(name = $base_name)]
            pub struct $BaseRust {
                pub hash: String,
            }

            #[derive(Clone, Debug, Default, SimpleObject)]
            #[graphql(name = $edge_name)]
            pub struct $EdgeRust {
                pub cursor: String,
                pub node: $BaseRust,
            }

            #[derive(Clone, Debug, SimpleObject)]
            #[graphql(name = $conn_name)]
            pub struct $ConnRust {
                pub edges: Vec<$EdgeRust>,
                #[graphql(name = "pageInfo")]
                pub page_info: Arc<PageInfo>,
                pub hash: String,
            }

            impl Default for $ConnRust {
                fn default() -> Self {
                    Self { edges: Vec::new(), page_info: Arc::new(PageInfo::default()), hash: String::new() }
                }
            }
        };
    }

    macro_rules! gap_surface_existing_relay {
        ($Base:ident) => {
            crate::external_adapters::paste::paste! {
                #[derive(Clone, Debug, Default, SimpleObject)]
                pub struct [<$Base Edge>] {
                    pub cursor: String,
                    pub hash: String,
                }

                #[derive(Clone, Debug, SimpleObject)]
                pub struct [<$Base Connection>] {
                    pub edges: Vec<[<$Base Edge>]>,
                    #[graphql(name = "pageInfo")]
                    pub page_info: Arc<PageInfo>,
                    pub hash: String,
                }

                impl Default for [<$Base Connection>] {
                    fn default() -> Self {
                        Self {
                            edges: Vec::new(),
                            page_info: Arc::new(PageInfo::default()),
                            hash: String::new(),
                        }
                    }
                }
            }
        };
    }

    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_family_name_list {
        (@apply_families) => {
            gap_surface_families! {
            AddedAttributeToConcept,
            AddedAttributeToDesign,
            AddedAttributeToDesignInput,
            AddedAttributeToPiece,
            AddedAttributeToPieceInput,
            AddedAttributeToPort,
            AddedAttributeToQuality,
            AddedAttributeToTag,
            AddedAttributeToType,
            AddedAttributeToTypeInput,
            AddedAttributesToConcept,
            AddedAttributesToDesign,
            AddedAttributesToDesignInput,
            AddedAttributesToPiece,
            AddedAttributesToPieceInput,
            AddedAttributesToPort,
            AddedAttributesToQuality,
            AddedAttributesToTag,
            AddedAttributesToType,
            AddedAttributesToTypeInput,
            AddedChildPieceWithParentConnection,
            AddedChildPieceWithParentConnectionInput,
            AddedChildPiecesWithParentConnections,
            AddedChildPiecesWithParentConnectionsInput,
            AddedConnector,
            AddedConnectorInput,
            AddedConnectors,
            AddedConnectorsInput,
            AddedHangingChildPieceWithParentConnectionInput,
            AddedHangingChildPieceWithParentConnection,
            AddedHangingChildPiecesWithParentConnections,
            AddedHangingChildPiecesWithParentConnectionsInput,
            AttributeDiff,
            AttributeModification,
            AttributeModifications,
            AuthorDiff,
            AuthorModification,
            AuthorModifications,
            BenchmarkDiff,
            BenchmarkModification,
            BenchmarkModifications,
            ChangedPieceToType,
            ChangedPieceToTypeInput,
            ChangedPiecesToType,
            ChangedPiecesToTypeInput,
            ConceptDiff,
            ConceptModification,
            ConceptModifications,
            ConceptOperation,
            ConnectionDiff,
            ConnectionModification,
            ConnectionModifications,
            ConnectorDiff,
            ConnectorModification,
            ConnectorModifications,
            ConnectorOperation,
            CreatedConcept,
            CreatedConcepts,
            CreatedDesign,
            CreatedDesignInput,
            CreatedDesigns,
            CreatedDesignsInput,
            CreatedFolder,
            CreatedFolderInput,
            MovedToFolder,
            MovedToFolderInput,
            CreatedPort,
            CreatedPorts,
            CreatedQualities,
            CreatedQuality,
            CreatedTag,
            CreatedTags,
            CreatedType,
            CreatedTypeInput,
            CreatedTypes,
            CreatedTypesInput,
            DeletedConcept,
            DeletedConcepts,
            DeletedDesign,
            DeletedDesigns,
            DeletedPiece,
            DeletedPieces,
            DeletedPiecesAndConnections,
            DeletedPort,
            DeletedPorts,
            DeletedQualities,
            DeletedQuality,
            DeletedTag,
            DeletedTags,
            DeletedType,
            DeletedTypes,
            DesignModification,
            DesignModifications,
            DesignOperation,
            DraggedPieces,
            DraggedPiecesInput,
            FamilyDiff,
            FamilyModification,
            FamilyModifications,
            FileDiff,
            FileModification,
            FileModifications,
            FixedPieces,
            FlattenedDesign,
            FolderDiff,
            FolderModification,
            FolderModifications,
            GroupDiff,
            GroupModification,
            GroupModifications,
            KitModification,
            KitModifications,
            KitOperation,
            LayerDiff,
            LayerModification,
            LayerModifications,
            MovedPiece,
            MovedPieceInput,
            MovedPieces,
            MovedPiecesInput,
            PieceDiff,
            PieceModification,
            PieceModifications,
            PieceOperation,
            PiecesOperation,
            PlaceDiff,
            PlaceModification,
            PlaceModifications,
            PortDiff,
            PortModification,
            PortModifications,
            PortOperation,
            PropDiff,
            PropModification,
            PropModifications,
            QualityDiff,
            QualityModification,
            QualityModifications,
            QualityOperation,
            RemovedAttributeFromConcept,
            RemovedAttributeFromDesign,
            RemovedAttributeFromPiece,
            RemovedAttributeFromPort,
            RemovedAttributeFromQuality,
            RemovedAttributeFromTag,
            RemovedAttributeFromType,
            RemovedAttributesFromConcept,
            RemovedAttributesFromDesign,
            RemovedAttributesFromPiece,
            RemovedAttributesFromPort,
            RemovedAttributesFromQuality,
            RemovedAttributesFromTag,
            RemovedAttributesFromType,
            RemovedConnector,
            RemovedConnectors,
            RenamedConcept,
            RenamedConnector,
            RenamedConnectorInput,
            RenamedPiece,
            RenamedPieceInput,
            RenamedPort,
            RenamedQuality,
            RenamedTag,
            RenamedType,
            RenamedTypeInput,
            RepresentationDiff,
            RepresentationModification,
            RepresentationModifications,
            SideDiff,
            SideModification,
            SideModifications,
            StatDiff,
            StatModification,
            StatModifications,
            TagDiff,
            TagModification,
            TagModifications,
            TagOperation,
            TypeDiff,
            TypeModification,
            TypeModifications,
            TypeOperation,
            UpdatedConceptDescription,
            UpdatedConceptIcon,
            UpdatedConnectorDescription,
            UpdatedConnectorDescriptionInput,
            UpdatedConnectorIcon,
            UpdatedConnectorIconInput,
            UpdatedPieceDescription,
            UpdatedPieceDescriptionInput,
            UpdatedPortDescription,
            UpdatedPortIcon,
            UpdatedQualityDescription,
            UpdatedQualityIcon,
            UpdatedTagDescription,
            UpdatedTagIcon,
            UpdatedTypeDescription,
            UpdatedTypeDescriptionInput,
            UpdatedTypeIcon,
            UpdatedTypeIconInput
                }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_family_connections!(
                $builder,
                AddedAttributeToConcept,
                AddedAttributeToDesign,
                AddedAttributeToDesignInput,
                AddedAttributeToPiece,
                AddedAttributeToPieceInput,
                AddedAttributeToPort,
                AddedAttributeToQuality,
                AddedAttributeToTag,
                AddedAttributeToType,
                AddedAttributeToTypeInput,
                AddedAttributesToConcept,
                AddedAttributesToDesign,
                AddedAttributesToDesignInput,
                AddedAttributesToPiece,
                AddedAttributesToPieceInput,
                AddedAttributesToPort,
                AddedAttributesToQuality,
                AddedAttributesToTag,
                AddedAttributesToType,
                AddedAttributesToTypeInput,
                AddedChildPieceWithParentConnection,
                AddedChildPieceWithParentConnectionInput,
                AddedChildPiecesWithParentConnections,
                AddedChildPiecesWithParentConnectionsInput,
                AddedConnector,
                AddedConnectorInput,
                AddedConnectors,
                AddedConnectorsInput,
                AddedHangingChildPieceWithParentConnectionInput,
                AddedHangingChildPieceWithParentConnection,
                AddedHangingChildPiecesWithParentConnections,
                AddedHangingChildPiecesWithParentConnectionsInput,
                AttributeDiff,
                AttributeModification,
                AttributeModifications,
                AuthorDiff,
                AuthorModification,
                AuthorModifications,
                BenchmarkDiff,
                BenchmarkModification,
                BenchmarkModifications,
                ChangedPieceToType,
                ChangedPieceToTypeInput,
                ChangedPiecesToType,
                ChangedPiecesToTypeInput,
                ConceptDiff,
                ConceptModification,
                ConceptModifications,
                ConceptOperation,
                ConnectionDiff,
                ConnectionModification,
                ConnectionModifications,
                ConnectorDiff,
                ConnectorModification,
                ConnectorModifications,
                ConnectorOperation,
                CreatedConcept,
                CreatedConcepts,
                CreatedDesign,
                CreatedDesignInput,
                CreatedDesigns,
                CreatedDesignsInput,
                CreatedFolder,
                CreatedFolderInput,
                MovedToFolder,
                MovedToFolderInput,
                CreatedPort,
                CreatedPorts,
                CreatedQualities,
                CreatedQuality,
                CreatedTag,
                CreatedTags,
                CreatedType,
                CreatedTypeInput,
                CreatedTypes,
                CreatedTypesInput,
                DeletedConcept,
                DeletedConcepts,
                DeletedDesign,
                DeletedDesigns,
                DeletedPiece,
                DeletedPieces,
                DeletedPiecesAndConnections,
                DeletedPort,
                DeletedPorts,
                DeletedQualities,
                DeletedQuality,
                DeletedTag,
                DeletedTags,
                DeletedType,
                DeletedTypes,
                DesignModification,
                DesignModifications,
                DesignOperation,
                DraggedPieces,
                DraggedPiecesInput,
                FamilyDiff,
                FamilyModification,
                FamilyModifications,
                FileDiff,
                FileModification,
                FileModifications,
                FixedPieces,
                FlattenedDesign,
                FolderDiff,
                FolderModification,
                FolderModifications,
                GroupDiff,
                GroupModification,
                GroupModifications,
                KitModification,
                KitModifications,
                KitOperation,
                LayerDiff,
                LayerModification,
                LayerModifications,
                MovedPiece,
                MovedPieceInput,
                MovedPieces,
                MovedPiecesInput,
                PieceDiff,
                PieceModification,
                PieceModifications,
                PieceOperation,
                PiecesOperation,
                PlaceDiff,
                PlaceModification,
                PlaceModifications,
                PortDiff,
                PortModification,
                PortModifications,
                PortOperation,
                PropDiff,
                PropModification,
                PropModifications,
                QualityDiff,
                QualityModification,
                QualityModifications,
                QualityOperation,
                RemovedAttributeFromConcept,
                RemovedAttributeFromDesign,
                RemovedAttributeFromPiece,
                RemovedAttributeFromPort,
                RemovedAttributeFromQuality,
                RemovedAttributeFromTag,
                RemovedAttributeFromType,
                RemovedAttributesFromConcept,
                RemovedAttributesFromDesign,
                RemovedAttributesFromPiece,
                RemovedAttributesFromPort,
                RemovedAttributesFromQuality,
                RemovedAttributesFromTag,
                RemovedAttributesFromType,
                RemovedConnector,
                RemovedConnectors,
                RenamedConcept,
                RenamedConnector,
                RenamedConnectorInput,
                RenamedPiece,
                RenamedPieceInput,
                RenamedPort,
                RenamedQuality,
                RenamedTag,
                RenamedType,
                RenamedTypeInput,
                RepresentationDiff,
                RepresentationModification,
                RepresentationModifications,
                SideDiff,
                SideModification,
                SideModifications,
                StatDiff,
                StatModification,
                StatModifications,
                TagDiff,
                TagModification,
                TagModifications,
                TagOperation,
                TypeDiff,
                TypeModification,
                TypeModifications,
                TypeOperation,
                UpdatedConceptDescription,
                UpdatedConceptIcon,
                UpdatedConnectorDescription,
                UpdatedConnectorDescriptionInput,
                UpdatedConnectorIcon,
                UpdatedConnectorIconInput,
                UpdatedPieceDescription,
                UpdatedPieceDescriptionInput,
                UpdatedPortDescription,
                UpdatedPortIcon,
                UpdatedQualityDescription,
                UpdatedQualityIcon,
                UpdatedTagDescription,
                UpdatedTagIcon,
                UpdatedTypeDescription,
                UpdatedTypeDescriptionInput,
                UpdatedTypeIcon,
                UpdatedTypeIconInput
            )
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@apply_relays) => {
            gap_surface_existing_relays! {
            AddedAttributeToConceptInput,
            AddedAttributeToPortInput,
            AddedAttributeToQualityInput,
            AddedAttributeToTagInput,
            AddedAttributesToConceptInput,
            AddedAttributesToPortInput,
            AddedAttributesToQualityInput,
            AddedAttributesToTagInput,
            AlternativeCommand,
            ChangedDescription,
            CreatedConceptInput,
            CreatedConceptsInput,
            CreatedFixedPiece,
            CreatedPortInput,
            CreatedPortsInput,
            CreatedQualitiesInput,
            CreatedQualityInput,
            CreatedTagInput,
            CreatedTagsInput,
            DraggedPiece,
            FileBackbone,
            FileBackboneCommand,
            FixedPiece,
            Graph,
            Kit,
            LocalProviderCommand,
            Place,
            RemoteProviderCommand,
            RenamedConceptInput,
            RenamedKit,
            RenamedPortInput,
            RenamedQualityInput,
            RenamedTagInput,
            Session,
            SessionCommand,
            Side,
            StoreCommand,
            TheKit,
            UnsavedChangeCommand,
            UpdatedConceptDescriptionInput,
            UpdatedConceptIconInput,
            UpdatedPortDescriptionInput,
            UpdatedPortIconInput,
            UpdatedQualityDescriptionInput,
            UpdatedQualityIconInput,
            UpdatedTagDescriptionInput,
            UpdatedTagIconInput,
            VersionCommand,
            WebsocketBackbone,
            WebsocketBackboneCommand
                }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections!(
                $builder,
                AddedAttributeToConceptInput,
                AddedAttributeToPortInput,
                AddedAttributeToQualityInput,
                AddedAttributeToTagInput,
                AddedAttributesToConceptInput,
                AddedAttributesToPortInput,
                AddedAttributesToQualityInput,
                AddedAttributesToTagInput,
                AlternativeCommand,
                ChangedDescription,
                CreatedConceptInput,
                CreatedConceptsInput,
                CreatedFixedPiece,
                CreatedPortInput,
                CreatedPortsInput,
                CreatedQualitiesInput,
                CreatedQualityInput,
                CreatedTagInput,
                CreatedTagsInput,
                DraggedPiece,
                FileBackbone,
                FileBackboneCommand,
                FixedPiece,
                Graph,
                Kit,
                LocalProviderCommand,
                Place,
                RemoteProviderCommand,
                RenamedConceptInput,
                RenamedKit,
                RenamedPortInput,
                RenamedQualityInput,
                RenamedTagInput,
                Session,
                SessionCommand,
                Side,
                StoreCommand,
                TheKit,
                UnsavedChangeCommand,
                UpdatedConceptDescriptionInput,
                UpdatedConceptIconInput,
                UpdatedPortDescriptionInput,
                UpdatedPortIconInput,
                UpdatedQualityDescriptionInput,
                UpdatedQualityIconInput,
                UpdatedTagDescriptionInput,
                UpdatedTagIconInput,
                VersionCommand,
                WebsocketBackbone,
                WebsocketBackboneCommand
            )
        };
    }

    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::gap_surface_family_name_list!(@apply_families);
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::external_adapters::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

    with_gap_surface_family_names!(gap_surface_families);

    gap_surface_family_named!("ChangedDescriptionInput", GapChangedDescriptionInput, "ChangedDescriptionInputEdge", GapChangedDescriptionInputEdge, "ChangedDescriptionInputConnection", GapChangedDescriptionInputConnection);
    gap_surface_family_named!("Clump", GapClump, "ClumpEdge", GapClumpEdge, "ClumpConnection", GapClumpConnection);
    gap_surface_family_named!("CreatedFixedPieceInput", GapCreatedFixedPieceInput, "CreatedFixedPieceInputEdge", GapCreatedFixedPieceInputEdge, "CreatedFixedPieceInputConnection", GapCreatedFixedPieceInputConnection);
    gap_surface_family_named!("DesignDiff", GapDesignDiff, "DesignDiffEdge", GapDesignDiffEdge, "DesignDiffConnection", GapDesignDiffConnection);
    gap_surface_family_named!("DraggedPieceInput", GapDraggedPieceInput, "DraggedPieceInputEdge", GapDraggedPieceInputEdge, "DraggedPieceInputConnection", GapDraggedPieceInputConnection);
    gap_surface_family_named!("KitDiff", GapKitDiff, "KitDiffEdge", GapKitDiffEdge, "KitDiffConnection", GapKitDiffConnection);
    gap_surface_family_named!("RenamedKitInput", GapRenamedKitInput, "RenamedKitInputEdge", GapRenamedKitInputEdge, "RenamedKitInputConnection", GapRenamedKitInputConnection);
    gap_surface_family_named!("Version", GapVersion, "VersionEdge", GapVersionEdge, "VersionConnection", GapVersionConnection);

    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<$crate::external_adapters::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);
}

//#endregion schema_gap_surfaces

//#region 🏷️ meta

pub mod meta {
    //! 🏷️ Metadata: DTO [`SimpleObject`] shells plus Arc-backed [`Tag`]/[`Concept`]/[`Quality`] entities (SDL `Entity`).
    use crate::external_adapters::async_graphql::Object;

    use crate::id::Id;
    use crate::timestamp::Timestamp;

    //#region 🧾️ graphql inputs
    crate::entity_input! {
        /// @emoji 🧾️ SDL `AttributeInput` — instantiates [`Attribute`] entities on entity create/update paths.
        pub struct AttributeInput as "AttributeInput" {
            pub key: String,
            pub value: Option<String>,
            pub definition: Option<String>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾️ SDL `TagInput`.
        pub struct TagInput as "TagInput" {
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub order: Option<i32>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾️ SDL `ConceptInput`.
        pub struct ConceptInput as "ConceptInput" {
            pub name: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub order: Option<i32>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
    }

    crate::entity_input! {
        /// @emoji 🧾️ SDL `QualityInput` (subset aligned to persisted kit fields).
        pub struct QualityInput as "QualityInput" {
            pub key: String,
            pub value: Option<String>,
            pub unit: Option<String>,
            pub definition: Option<String>,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub attributes: Option<Vec<AttributeInput>>,
        }
    }
    //#endregion 🧾️ graphql inputs

    impl AttributeInput {
        /// @emoji ➕️ Mint a persisted [`Attribute`] from GraphQL input (fresh [`Id`]).
        pub async fn into_attribute(self) -> Attribute {
            Attribute { id: Id::new().await, key: self.key, value: self.value.unwrap_or_default(), definition: self.definition }
        }

        /// @emoji 🪪️ Rebuild a persisted [`Attribute`] using a caller-supplied id from a normalized operation scope.
        pub fn into_attribute_with_id(self, id: Id) -> Attribute {
            Attribute { id, key: self.key, value: self.value.unwrap_or_default(), definition: self.definition }
        }
    }

    /// @emoji ➕️ Expand optional GraphQL attribute entities into minted [`Attribute`] entities.
    pub async fn attributes_from_inputs(inp: Option<Vec<AttributeInput>>) -> Vec<Attribute> {
        let mut v = Vec::new();
        for a in inp.into_iter().flatten() {
            v.push(a.into_attribute().await);
        }
        v
    }

    /// @emoji 🪪️ Rebuild optional GraphQL attribute entities using the ids already recorded in operation scope.
    pub fn attributes_from_inputs_with_ids(inp: Option<Vec<AttributeInput>>, ids: &[Id]) -> Result<Vec<Attribute>, crate::error::ComposeError> {
        let attrs = inp.unwrap_or_default();
        if attrs.len() != ids.len() {
            return Err(crate::error::ComposeError::invalid(format!("attribute id count mismatch: expected {}, got {}", attrs.len(), ids.len())));
        }
        Ok(attrs.into_iter().zip(ids.iter().cloned()).map(|(attr, id)| attr.into_attribute_with_id(id)).collect())
    }

    crate::entity_family! {
        pub struct File {
            pub id: Id,
            pub name: String,
            pub url: String,
            pub mime: Option<String>,
            pub size: Option<i32>,
            /// @emoji 📎️ Blob/content digest on the wire (`hash` in JSON); omitted from GraphQL in favor of entity [`File::hash`] resolver.
            #[graphql(skip)]
            pub hash: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub folder_id: Option<Id>,
            #[graphql(skip)]
            pub owner_kit: std::sync::Weak<crate::kit::Kit>,
            pub created: Option<Timestamp>,
            pub updated: Option<Timestamp>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "File",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.url.as_str(),
                    this.mime.as_deref().unwrap_or(""),
                    &this.size.map(|sz| sz.to_string()).unwrap_or_default(),
                    this.hash.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.icon.as_deref().unwrap_or(""),
                    this.folder_id.as_ref().map(|id| id.as_str()).unwrap_or(""),
                    this.created.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                    this.updated.as_ref().map(|t| t.0.as_str()).unwrap_or(""),
                ],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_kit.upgrade().map(crate::gql::interfaces::EntityInterface::Kit)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                None
            }
            pub async fn tag(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<Tag>> {
                None
            }
            pub async fn quality(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<Quality>> {
                None
            }
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
            /// @emoji 💾️ Representations that reference this file.
            #[graphql(name = "hasRepresentations")]
            pub async fn has_representations_field(&self) -> crate::gql_relay::RepresentationConnection {
                crate::gql_relay::RepresentationConnection::from_representations(self.has_representations().await).await
            }
            /// @emoji 🧰️ Kinds that own a representation referencing this file.
            #[graphql(name = "referencesTypes")]
            pub async fn references_types_field(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.references_types().await).await
            }
            /// @emoji 🏘️ Designs with a direct piece blueprinting a kind that references this file.
            #[graphql(name = "referencesDesigns")]
            pub async fn references_designs_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.references_designs().await).await
            }
            /// @emoji 🧰️ Kinds that reference this file (kinds do not nest; same as direct).
            #[graphql(name = "referencesTypesTransitive")]
            pub async fn references_types_transitive_field(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.references_types_transitive().await).await
            }
            /// @emoji 🏘️ Designs that reference this file transitively through kinds and nested designs.
            #[graphql(name = "referencesDesignsTransitive")]
            pub async fn references_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.references_designs_transitive().await).await
            }
        )
        , vfs = File
    }

    impl File {
        async fn owner_kit_arc(&self) -> Option<std::sync::Arc<crate::kit::Kit>> {
            self.owner_kit.upgrade()
        }

        /// @emoji 💾️ Representations on kit kinds that reference this file.
        pub async fn has_representations(&self) -> Vec<std::sync::Arc<crate::kit::r#type::Representation>> {
            let Some(kit) = self.owner_kit_arc().await else {
                return Vec::new();
            };
            kit.representations_for_file(&self.id).await
        }

        /// @emoji 🧰️ Kinds that own a representation referencing this file.
        pub async fn references_types(&self) -> Vec<std::sync::Arc<crate::kit::r#type::Type>> {
            let Some(kit) = self.owner_kit_arc().await else {
                return Vec::new();
            };
            kit.types_for_file(&self.id).await
        }

        /// @emoji 🏘️ Designs with a direct piece blueprinting a kind that references this file.
        pub async fn references_designs(&self) -> Vec<std::sync::Arc<crate::kit::design::Design>> {
            let Some(kit) = self.owner_kit_arc().await else {
                return Vec::new();
            };
            kit.designs_with_direct_file_reference(&self.id).await
        }

        /// @emoji 🧰️ Kinds that reference this file (kinds do not nest; same as direct).
        pub async fn references_types_transitive(&self) -> Vec<std::sync::Arc<crate::kit::r#type::Type>> {
            self.references_types().await
        }

        /// @emoji 🏘️ Designs that reference this file transitively through kinds and nested designs.
        pub async fn references_designs_transitive(&self) -> Vec<std::sync::Arc<crate::kit::design::Design>> {
            let Some(kit) = self.owner_kit_arc().await else {
                return Vec::new();
            };
            kit.designs_referencing_file_transitive(&self.id).await
        }
    }

    crate::entity_family! {
        pub struct Folder {
            pub id: Id,
            pub name: String,
            pub path: String,
            pub description: Option<String>,
            pub icon: Option<String>,
            pub parent_folder_id: Option<Id>,
            #[graphql(skip)]
            pub owner_kit: std::sync::Weak<crate::kit::Kit>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "Folder",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.path.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.icon.as_deref().unwrap_or(""),
                    this.parent_folder_id.as_ref().map(|id| id.as_str()).unwrap_or(""),
                ],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_kit.upgrade().map(crate::gql::interfaces::EntityInterface::Kit)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                None
            }
            pub async fn file(&self, #[graphql(name = "id")] id: Id) -> Option<File> {
                let kit = self.owner_kit.upgrade()?;
                let rows = kit.files.read().await;
                rows.iter().find(|f| f.id == id && f.folder_id.as_ref() == Some(&self.id)).cloned()
            }
            pub async fn files(&self) -> crate::gql_relay::FileConnection {
                let rows = if let Some(kit) = self.owner_kit.upgrade() {
                    kit.files.read().await.iter().filter(|f| f.folder_id.as_ref() == Some(&self.id)).cloned().collect()
                } else {
                    Vec::new()
                };
                crate::gql_relay::FileConnection::from_entities(rows)
            }
            #[graphql(name = "subFolder")]
            pub async fn sub_folder(&self, #[graphql(name = "id")] id: Id) -> Option<Folder> {
                let kit = self.owner_kit.upgrade()?;
                let rows = kit.folders.read().await;
                rows.iter().find(|f| f.id == id && f.parent_folder_id.as_ref() == Some(&self.id)).cloned()
            }
            #[graphql(name = "subFolders")]
            pub async fn sub_folders(&self) -> crate::gql_relay::FolderConnection {
                let rows = if let Some(kit) = self.owner_kit.upgrade() {
                    kit.folders.read().await.iter().filter(|f| f.parent_folder_id.as_ref() == Some(&self.id)).cloned().collect()
                } else {
                    Vec::new()
                };
                crate::gql_relay::FolderConnection::from_entities(rows)
            }
            pub async fn family(&self, #[graphql(name = "id")] id: Id) -> Option<crate::gql_relay::Family> {
                let kit = self.owner_kit.upgrade()?;
                let rows = kit.families.read().await;
                rows.iter().find(|f| f.id == id && f.folder_id.as_ref() == Some(&self.id)).cloned()
            }
            pub async fn families(&self) -> crate::gql_relay::FamilyConnection {
                let rows = if let Some(kit) = self.owner_kit.upgrade() {
                    kit.families.read().await.iter().filter(|f| f.folder_id.as_ref() == Some(&self.id)).cloned().collect()
                } else {
                    Vec::new()
                };
                crate::gql_relay::FamilyConnection::from_entities(rows)
            }
            #[graphql(name = "type")]
            pub async fn type_(&self, #[graphql(name = "id")] id: Id) -> Option<std::sync::Arc<crate::kit::r#type::Type>> {
                let kit = self.owner_kit.upgrade()?;
                let ty = kit.type_by_external_id(&id).await?;
                if ty.folder_id.read().await.as_ref() == Some(&self.id) {
                    Some(ty)
                } else {
                    None
                }
            }
            /// @emoji 🧰️ Kinds in this folder.
            #[graphql(name = "hasTypes")]
            pub async fn has_types_field(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.has_types().await).await
            }
            pub async fn design(&self, #[graphql(name = "id")] id: Id) -> Option<std::sync::Arc<crate::kit::design::Design>> {
                let kit = self.owner_kit.upgrade()?;
                let d = kit.design_by_external_id(&id).await?;
                if d.folder_id.read().await.as_ref() == Some(&self.id) {
                    Some(d)
                } else {
                    None
                }
            }
            /// @emoji 🏘️ Designs in this folder.
            #[graphql(name = "hasDesigns")]
            pub async fn has_designs_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.has_designs().await).await
            }
        )
        , vfs = Folder
    }

    impl Folder {
        /// @emoji 🧰️ Kinds assigned to this folder.
        pub async fn has_types(&self) -> Vec<std::sync::Arc<crate::kit::r#type::Type>> {
            let Some(kit) = self.owner_kit.upgrade() else {
                return Vec::new();
            };
            let mut out = Vec::new();
            for ty in kit.has_types().await.iter() {
                if ty.folder_id.read().await.as_ref() == Some(&self.id) {
                    out.push(ty.clone());
                }
            }
            out
        }

        /// @emoji 🏘️ Designs assigned to this folder.
        pub async fn has_designs(&self) -> Vec<std::sync::Arc<crate::kit::design::Design>> {
            let Some(kit) = self.owner_kit.upgrade() else {
                return Vec::new();
            };
            let mut out = Vec::new();
            for design in kit.has_designs().await.iter() {
                if design.folder_id.read().await.as_ref() == Some(&self.id) {
                    out.push(design.clone());
                }
            }
            out
        }
    }

    crate::entity_family! {
        pub struct Author {
            pub id: Id,
            pub name: String,
            pub email: String,
            pub role: Option<String>,
            pub rank: Option<i32>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &[
                    "Author",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.email.as_str(),
                    this.role.as_deref().unwrap_or(""),
                    &this.rank.map(|r| r.to_string()).unwrap_or_default(),
                ],
                Vec::new(),
            )
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Attribute {
        pub id: Id,
        pub key: String,
        pub value: String,
        pub definition: Option<String>,
    }

    impl Attribute {
        /// @emoji 🌿️ Blake3 leaf over persisted attribute columns (no owner weak refs).
        pub fn compute_entity_hash(&self) -> String {
            crate::hash::merkle_node_str(&["Attribute", self.id.as_str(), self.key.as_str(), self.value.as_str(), self.definition.as_deref().unwrap_or("")], Vec::new())
        }
    }

    #[Object(name = "Attribute")]
    impl Attribute {
        pub async fn id(&self) -> Id {
            self.id.clone()
        }
        pub async fn hash(&self) -> String {
            self.compute_entity_hash()
        }
        pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
            None
        }
        pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
            Some(crate::gql::interfaces::empty_entity_connection())
        }
        pub async fn key(&self) -> String {
            self.key.clone()
        }
        pub async fn value(&self) -> Option<String> {
            Some(self.value.clone())
        }
        pub async fn definition(&self) -> Option<String> {
            self.definition.clone()
        }
    }

    crate::entity_family! {
        pub struct Benchmark {
            pub id: Id,
            pub name: String,
            pub min: Option<f64>,
            pub max: Option<f64>,
            #[graphql(name = "minExcluded")]
            pub min_excluded: Option<bool>,
            #[graphql(name = "maxExcluded")]
            pub max_excluded: Option<bool>,
        }
        hash = |this| {
            let min = this.min.map(|v| format!("{v:.9}")).unwrap_or_default();
            let max = this.max.map(|v| format!("{v:.9}")).unwrap_or_default();
            let minx = this.min_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let maxx = this.max_excluded.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(&["Benchmark", this.id.as_str(), this.name.as_str(), min.as_str(), max.as_str(), minx, maxx], Vec::new())
        }
    }

    crate::entity_family! {
        pub struct Prop {
            pub id: Id,
            pub key: String,
            pub value: String,
            pub unit: Option<String>,
            #[graphql(skip)]
            pub quality: Option<std::sync::Arc<Quality>>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(&["Prop", this.id.as_str(), this.key.as_str(), this.value.as_str(), this.unit.as_deref().unwrap_or("")], Vec::new())
        }
        , extra = (
            /// @emoji 🔎️ SDL `Prop.attribute(id)` — props carry no attribute bag yet; reserved for kit snapshots.
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
        )
    }

    crate::meta_arc_titled_entity!(Tag, "Tag");

    crate::meta_arc_titled_entity!(Concept, "Concept");

    crate::meta_quality_entity!();

    crate::entity_family! {
        pub struct Stat {
            pub id: Id,
            pub key: String,
            pub value: String,
            pub unit: Option<String>,
            pub description: Option<String>,
        }
        hash = |this| {
            crate::hash::merkle_node_str(
                &["Stat", this.id.as_str(), this.key.as_str(), this.value.as_str(), this.unit.as_deref().unwrap_or(""), this.description.as_deref().unwrap_or("")],
                Vec::new(),
            )
        }
        , extra = (
            /// @emoji 🔎️ SDL `Stat.attribute(id)` — stats carry no attribute bag yet; reserved for kit snapshots.
            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<Attribute> {
                None
            }
        )
    }

    crate::entity_family! {
        pub struct Layer {
            pub id: Id,
            pub name: String,
            pub description: Option<String>,
            /// @emoji 🏷️ Artifact `icon` on SDL `Layer`.
            pub icon: String,
            pub color: Option<String>,
            pub order: Option<i32>,
            pub visible: Option<bool>,
            pub locked: Option<bool>,
        }
        hash = |this| {
            let vis = this.visible.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            let lck = this.locked.map(|b| if b { "1" } else { "0" }).unwrap_or_default();
            crate::hash::merkle_node_str(
                &[
                    "Layer",
                    this.id.as_str(),
                    this.name.as_str(),
                    this.description.as_deref().unwrap_or(""),
                    this.icon.as_str(),
                    this.color.as_deref().unwrap_or(""),
                    &this.order.map(|o| o.to_string()).unwrap_or_default(),
                    vis,
                    lck,
                ],
                Vec::new(),
            )
        }
    }

    crate::entity_family! {
        pub struct Group {
            pub id: Id,
            pub name: String,
            pub description: Option<String>,
            pub color: Option<String>,
            pub icon: Option<String>,
            #[graphql(skip)]
            pub piece_ids: Vec<Id>,
        }
        hash = |this| {
            let mut ids: Vec<String> = this.piece_ids.iter().map(|i| i.as_str().to_string()).collect();
            ids.sort();
            let joined = ids.join("\x1e");
            crate::hash::merkle_node_str(
                &["Group", this.id.as_str(), this.name.as_str(), this.description.as_deref().unwrap_or(""), this.color.as_deref().unwrap_or(""), this.icon.as_deref().unwrap_or(""), joined.as_str()],
                Vec::new(),
            )
        }
        , extra = (
            pub async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
            pub async fn piece(&self, #[graphql(name = "id")] _id: Id) -> Option<std::sync::Arc<crate::kit::design::piece::Piece>> {
                None
            }
        )
    }
}

//#endregion 🏷️ meta

//#region 🪪️ hash

pub mod hash {
    //! 🪪️ Blake3 Merkle helpers: [`h`] for delimiter-joined parts; [`merkle_node_str`] for ordered own fields plus sorted child digests; [`merkle_collection`] for relay connection hashes.
    use crate::external_adapters::blake3::Hasher;

    pub fn h<S: AsRef<[u8]>>(parts: &[S]) -> String {
        let mut hasher = Hasher::new();
        for p in parts {
            hasher.update(p.as_ref());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
    }

    /// @emoji 🔢️ Canonical `f64` text for hash joins (integral-ish values without trailing `.0`; trims fractional noise).
    pub fn format_number_for_hash(n: f64) -> String {
        if n.is_nan() {
            return "nan".to_string();
        }
        if n.is_infinite() {
            return if n.is_sign_positive() { "inf".into() } else { "-inf".into() };
        }
        if n == 0.0 {
            return "0".into();
        }
        if (n - n.round()).abs() < 1e-9 && n.abs() < 1e15 {
            return format!("{:.0}", n);
        }
        let mut s = format!("{n:.12}");
        if s.contains('.') {
            while s.ends_with('0') {
                s.pop();
            }
            if s.ends_with('.') {
                s.pop();
            }
        }
        if s == "-0" {
            "0".into()
        } else {
            s
        }
    }

    /// @emoji 🌳️ Merkle fold: concatenates `own` in order, then **sorted** `children` hex digests (order-independent set hashing).
    pub fn merkle_node_str(own: &[&str], mut children: Vec<String>) -> String {
        children.sort();
        let mut hasher = Hasher::new();
        for s in own {
            hasher.update(s.as_bytes());
            hasher.update(b"\x1f");
        }
        for c in &children {
            hasher.update(c.as_bytes());
            hasher.update(b"\x1f");
        }
        hasher.finalize().to_hex().to_string()
    }

    /// @emoji 🪢️ Relay collection hash: sorted child entity hashes under a stable collection tag.
    pub fn merkle_collection(children: Vec<String>) -> String {
        merkle_node_str(&["RelayCollection"], children)
    }
}

//#endregion 🪪️ hash

//#region 📦️ kit

pub mod kit {
    //! 📦️ Kit ↔ Type ↔ Design entity tree (Arc + interior RwLock per mutable field).

    //#region 🏠️ type
    pub mod r#type {
        //! 🏠️ Types, their connectors and representations.
        use std::sync::{Arc, Weak};

        use crate::external_adapters::async_graphql::{Object, Union};
        use crate::external_adapters::async_lock::RwLock;

        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Concept, File, Prop, Quality, Stat, Tag};
        use crate::timestamp::Timestamp;

        //#region 🛟️ port
        /// 🔌️ Kit-level named attachment point; referenced by [`Connector`] and [`super::connection::Side`].
        #[derive(Debug)]
        pub struct Port {
            pub id: Id,
            pub owner_type: Weak<Type>,
            pub code: RwLock<Option<String>>,
            pub label: RwLock<Option<String>>,
            pub order: RwLock<Option<i32>>,
            pub compatible_with: RwLock<Vec<Arc<Port>>>,
        }

        impl Default for Port {
            fn default() -> Self {
                Self { id: Id::default(), owner_type: Weak::new(), code: RwLock::new(None), label: RwLock::new(None), order: RwLock::new(None), compatible_with: RwLock::new(Vec::new()) }
            }
        }

        impl Port {
            pub async fn new(owner_type: Weak<Type>) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_type, ..Default::default() })
            }

            pub async fn new_with_external_id(owner_type: Weak<Type>, id: Id) -> Arc<Self> {
                Arc::new(Self { id, owner_type, ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let code = self.code.read().await.clone().unwrap_or_default();
                let label = self.label.read().await.clone().unwrap_or_default();
                let ord = self.order.read().await.map(|i| i.to_string()).unwrap_or_default();
                h(&[self.id.as_str(), code.as_str(), label.as_str(), ord.as_str()])
            }
        }

        #[Object(name = "Port")]
        impl Port {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_type.upgrade().map(crate::gql::interfaces::EntityInterface::Type)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            pub async fn code(&self) -> Option<String> {
                self.code.read().await.clone()
            }
            pub async fn label(&self) -> Option<String> {
                self.label.read().await.clone()
            }
            pub async fn order(&self) -> Option<i32> {
                *self.order.read().await
            }

            #[graphql(name = "copatibleWith")]
            pub async fn copatible_with(&self) -> crate::gql_relay::PortConnection {
                crate::gql_relay::PortConnection::from_entities(self.compatible_with.read().await.clone()).await
            }

            pub async fn attribute(&self, #[graphql(name = "id")] _id: Id) -> Option<crate::meta::Attribute> {
                None
            }
        }

        crate::file_system_node_vfs_complex_ctx!(Port, crate::gql::interfaces::file_system_vfs::node_for_port);
        //#endregion 🛟️ port

        //#region ⚓️ connector
        pub struct Connector {
            pub id: Id,
            pub owner_type: Weak<Type>,
            /// @emoji 🏷️ SDL `Connector.name` (Artifact).
            pub name: RwLock<String>,
            pub code: RwLock<String>,
            pub description: RwLock<String>,
            /// @emoji 🏷️ SDL `Connector.icon` (Artifact).
            pub icon: RwLock<String>,
            /// @emoji 📍️ Connector attachment point in type-local space.
            pub point: RwLock<crate::geom::PointInput>,
            /// @emoji ➡️ Connector outward direction in type-local space.
            pub direction: RwLock<crate::geom::VectorInput>,
            /// @emoji 🎯️ Parametric position on connector arc for diagram layout.
            pub t_param: RwLock<Option<f64>>,
            /// @emoji ✅️ Whether this connector must be connected.
            pub mandatory: RwLock<Option<bool>>,
            /// @emoji 🔗️ Resolved port pointer (`# data` on the wire).
            pub port: RwLock<Option<Arc<Port>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Connector {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_type: Weak::new(),
                    name: RwLock::new(String::new()),
                    code: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    point: RwLock::new(crate::geom::PointInput::default()),
                    direction: RwLock::new(crate::geom::VectorInput { x: 0.0, y: 0.0, z: 1.0 }),
                    t_param: RwLock::new(None),
                    mandatory: RwLock::new(None),
                    port: RwLock::new(None),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                }
            }
        }

        impl Connector {
            pub async fn new(owner_type: Weak<Type>, code: String) -> Arc<Self> {
                Arc::new(Self {
                    id: Id::new().await,
                    owner_type,
                    name: RwLock::new(String::new()),
                    code: RwLock::new(code),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    point: RwLock::new(crate::geom::PointInput::default()),
                    direction: RwLock::new(crate::geom::VectorInput { x: 0.0, y: 0.0, z: 1.0 }),
                    t_param: RwLock::new(None),
                    mandatory: RwLock::new(None),
                    port: RwLock::new(None),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            pub async fn new_with_external_id(owner_type: Weak<Type>, id: Id, code: String) -> Arc<Self> {
                Arc::new(Self {
                    id,
                    owner_type,
                    name: RwLock::new(code.clone()),
                    code: RwLock::new(code),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    point: RwLock::new(crate::geom::PointInput::default()),
                    direction: RwLock::new(crate::geom::VectorInput { x: 0.0, y: 0.0, z: 1.0 }),
                    t_param: RwLock::new(None),
                    mandatory: RwLock::new(None),
                    port: RwLock::new(None),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                let code = self.code.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                h(&[self.id.as_str(), name.as_str(), code.as_str(), desc.as_str(), icon.as_str()])
            }
        }

        #[Object(name = "Connector")]
        impl Connector {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_type.upgrade().map(crate::gql::interfaces::EntityInterface::Type)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn code(&self) -> String {
                self.code.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn port(&self) -> Option<Arc<Port>> {
                self.port.read().await.clone()
            }
            pub async fn point(&self) -> Arc<crate::geom::entity::Point> {
                let p = *self.point.read().await;
                crate::geom::entity::Point::from_input(p)
            }
            pub async fn direction(&self) -> Arc<crate::geom::entity::Vector> {
                let v = *self.direction.read().await;
                crate::geom::entity::Vector::from_input(v)
            }
            pub async fn t(&self) -> Option<f64> {
                *self.t_param.read().await
            }
            pub async fn mandatory(&self) -> Option<bool> {
                *self.mandatory.read().await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
            }
        }

        crate::file_system_node_vfs_complex_ctx!(Connector, crate::gql::interfaces::file_system_vfs::node_for_connector);
        //#endregion ⚓️ connector

        //#region 💾️ representation
        pub struct Representation {
            pub id: Id,
            pub owner_type: Weak<Type>,
            /// @emoji 🏷️ SDL `Representation.name` (Artifact).
            pub name: RwLock<String>,
            pub url: RwLock<String>,
            pub description: RwLock<String>,
            /// @emoji 🏷️ SDL `Representation.icon` (Artifact).
            pub icon: RwLock<String>,
            pub file: RwLock<Option<File>>,
            pub tags: RwLock<Vec<Arc<Tag>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub attributes: RwLock<Vec<Attribute>>,
        }

        impl Default for Representation {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_type: Weak::new(),
                    name: RwLock::new(String::new()),
                    url: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    file: RwLock::new(None),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                }
            }
        }

        impl Representation {
            pub async fn new(owner_type: Weak<Type>, url: String) -> Arc<Self> {
                Arc::new(Self {
                    id: Id::new().await,
                    owner_type,
                    name: RwLock::new(String::new()),
                    url: RwLock::new(url),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    file: RwLock::new(None),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                })
            }

            /// 🧾️ Insert a representation with caller-controlled external [`Id`] (JSON snapshot hydration).
            pub async fn new_with_external_id(owner_type: Weak<Type>, id: Id, url: String) -> Arc<Self> {
                Arc::new(Self { id, owner_type, url: RwLock::new(url), ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let url = self.url.read().await;
                let name = self.name.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                h(&[self.id.as_str(), name.as_str(), url.as_str(), desc.as_str(), icon.as_str()])
            }

            /// @emoji 🏘️ Designs with a direct piece blueprinting this representation's owner kind.
            pub async fn referenced_by_designs_direct(&self) -> Vec<Arc<super::design::Design>> {
                let Some(ty) = self.owner_type.upgrade() else {
                    return Vec::new();
                };
                let Some(kit) = ty.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_with_direct_blueprint_type(&ty.id).await
            }

            /// @emoji 🏘️ Designs that reference this representation's owner kind transitively.
            pub async fn referenced_by_designs_transitive(&self) -> Vec<Arc<super::design::Design>> {
                let Some(ty) = self.owner_type.upgrade() else {
                    return Vec::new();
                };
                let Some(kit) = ty.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_referencing_type_transitive(&ty.id).await
            }

            /// @emoji 🪢️ Pieces in the owner kit whose blueprint is this representation's owner kind.
            pub async fn referenced_by_pieces(&self) -> Vec<Arc<super::design::piece::Piece>> {
                let Some(ty) = self.owner_type.upgrade() else {
                    return Vec::new();
                };
                let Some(kit) = ty.owner_kit().await else {
                    return Vec::new();
                };
                kit.pieces_with_blueprint_type(&ty.id).await
            }

            /// @emoji 🧰️ Owner kind of this representation (zero or one).
            pub async fn owner_types(&self) -> Vec<Arc<Type>> {
                self.owner_type.upgrade().into_iter().collect()
            }

            /// @emoji 📄️ Linked file for this representation (zero or one).
            pub async fn linked_files(&self) -> Vec<File> {
                self.file.read().await.clone().into_iter().collect()
            }
        }

        #[Object(name = "Representation")]
        impl Representation {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_type.upgrade().map(crate::gql::interfaces::EntityInterface::Type)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn url(&self) -> String {
                self.url.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn file(&self) -> Option<File> {
                self.file.read().await.clone()
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_entities(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }

            pub async fn tag(&self, id: Id) -> Option<Arc<Tag>> {
                self.tags.read().await.iter().find(|t| t.id == id).cloned()
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
            }
            /// @emoji 🪢️ Pieces whose blueprint is this representation's owner kind.
            #[graphql(name = "referencedBy")]
            pub async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.referenced_by_pieces().await).await
            }
            /// @emoji 🧰️ Owner kind of this representation.
            #[graphql(name = "hasTypes")]
            pub async fn has_types(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.owner_types().await).await
            }
            /// @emoji 📄️ File linked from this representation.
            #[graphql(name = "referencesFiles")]
            pub async fn references_files(&self) -> crate::gql_relay::FileConnection {
                crate::gql_relay::FileConnection::from_entities(self.linked_files().await)
            }
            /// @emoji 🏘️ Designs with a direct piece blueprinting this representation's owner kind.
            #[graphql(name = "referencedByDesigns")]
            pub async fn referenced_by_designs(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_direct().await).await
            }
            /// @emoji 🏘️ Designs that reference this representation's owner kind transitively.
            #[graphql(name = "referencedByDesignsTransitive")]
            pub async fn referenced_by_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_transitive().await).await
            }
        }

        crate::file_system_node_vfs_complex_ctx!(Representation, crate::gql::interfaces::file_system_vfs::node_for_representation);
        //#endregion 💾️ representation

        //#region 🏠️ type
        pub struct Type {
            pub id: Id,
            pub owner_typology: Weak<crate::gql_relay::Typology>,
            pub name: RwLock<String>,
            pub description: RwLock<String>,
            pub icon: RwLock<String>,
            pub image: RwLock<String>,
            pub unit: RwLock<String>,
            pub created: RwLock<Option<Timestamp>>,
            pub updated: RwLock<Option<Timestamp>>,
            pub connectors: RwLock<Vec<Arc<Connector>>>,
            pub ports: RwLock<Vec<Arc<Port>>>,
            pub representations: RwLock<Vec<Arc<Representation>>>,
            /// 🧷️ Refreshed from `connectors` before single-id GraphQL lookups (no stale `Id` scans in resolvers).
            pub connector_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Connector>>>,
            pub port_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Port>>>,
            pub representation_weak_by_id: RwLock<std::collections::HashMap<Id, Weak<Representation>>>,
            pub authors: RwLock<Vec<Author>>,
            pub concepts: RwLock<Vec<Arc<Concept>>>,
            pub tags: RwLock<Vec<Arc<Tag>>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub props: RwLock<Vec<Prop>>,
            pub attributes: RwLock<Vec<Attribute>>,
            pub stats: RwLock<Vec<Stat>>,
            pub folder_id: RwLock<Option<Id>>,
        }

        impl Default for Type {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_typology: Weak::new(),
                    name: RwLock::new(String::new()),
                    description: RwLock::new(String::new()),
                    icon: RwLock::new(String::new()),
                    image: RwLock::new(String::new()),
                    unit: RwLock::new(String::new()),
                    created: RwLock::new(None),
                    updated: RwLock::new(None),
                    connectors: RwLock::new(Vec::new()),
                    ports: RwLock::new(Vec::new()),
                    representations: RwLock::new(Vec::new()),
                    connector_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    port_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    representation_weak_by_id: RwLock::new(std::collections::HashMap::new()),
                    authors: RwLock::new(Vec::new()),
                    concepts: RwLock::new(Vec::new()),
                    tags: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    props: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                    stats: RwLock::new(Vec::new()),
                    folder_id: RwLock::new(None),
                }
            }
        }

        impl Type {
            pub async fn owner_kit(&self) -> Option<Arc<crate::kit::Kit>> {
                self.owner_typology.upgrade()?.owner_kit.upgrade()
            }

            pub async fn new(owner_typology: Weak<crate::gql_relay::Typology>, name: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_typology, name: RwLock::new(name), ..Default::default() })
            }

            /// 🧾️ Insert a workspace kind with caller-controlled external [`Id`] (wasm / JSON snapshot hydration).
            pub async fn new_with_external_id(owner_typology: Weak<crate::gql_relay::Typology>, id: Id, name: String) -> Arc<Self> {
                Arc::new(Self { id, owner_typology, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                let desc = self.description.read().await;
                let icon = self.icon.read().await;
                let image = self.image.read().await;
                let unit = self.unit.read().await;
                h(&[self.id.as_str(), name.as_str(), desc.as_str(), icon.as_str(), image.as_str(), unit.as_str()])
            }

            /// 🧷️ Rebuild weak maps from the live vecs (call before `connector` / `representation` field resolution).
            pub async fn refresh_connector_child_weak_maps(&self) {
                {
                    let v = self.connectors.read().await;
                    let mut m = self.connector_weak_by_id.write().await;
                    m.clear();
                    for c in v.iter() {
                        m.insert(c.id.clone(), Arc::downgrade(c));
                    }
                }
                {
                    let v = self.ports.read().await;
                    let mut m = self.port_weak_by_id.write().await;
                    m.clear();
                    for p in v.iter() {
                        m.insert(p.id.clone(), Arc::downgrade(p));
                    }
                }
                {
                    let v = self.representations.read().await;
                    let mut m = self.representation_weak_by_id.write().await;
                    m.clear();
                    for r in v.iter() {
                        m.insert(r.id.clone(), Arc::downgrade(r));
                    }
                }
            }
            pub async fn best_representation_for_tags(&self, tag_ids: &[Id]) -> Option<Arc<Representation>> {
                use std::collections::HashSet;
                let want: HashSet<&Id> = tag_ids.iter().collect();
                let reps = self.representations.read().await;
                let mut best: Option<(usize, Arc<Representation>)> = None;
                for r in reps.iter() {
                    let tags = r.tags.read().await;
                    let score = tags.iter().filter(|t| want.contains(&t.id)).count();
                    if best.as_ref().is_none_or(|(s, _)| score > *s) {
                        best = Some((score, r.clone()));
                    }
                }
                best.map(|(_, r)| r)
            }

            /// @emoji 📄️ Distinct kit [`File`] nodes linked from this kind's representations (order preserved).
            pub async fn files_from_representations(&self) -> Vec<File> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for r in self.representations.read().await.iter() {
                    if let Some(f) = r.file.read().await.clone() {
                        if seen.insert(f.id.clone()) {
                            out.push(f);
                        }
                    }
                }
                out
            }

            /// @emoji 🪢️ Pieces in the owner kit whose blueprint is this kind.
            pub async fn referenced_by_pieces(&self) -> Vec<Arc<super::design::piece::Piece>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.pieces_with_blueprint_type(&self.id).await
            }

            /// @emoji 🏘️ Designs with a direct piece blueprinting this kind.
            pub async fn referenced_by_designs_direct(&self) -> Vec<Arc<super::design::Design>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_with_direct_blueprint_type(&self.id).await
            }

            /// @emoji 🏘️ Designs that reference this kind transitively through nested design blueprints.
            pub async fn referenced_by_designs_transitive(&self) -> Vec<Arc<super::design::Design>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_referencing_type_transitive(&self.id).await
            }

            /// @emoji ⚓️ Connectors owned directly by this kind.
            pub async fn has_connectors(&self) -> Vec<Arc<Connector>> {
                self.connectors.read().await.clone()
            }

            /// @emoji 🔘️ Ports owned directly by this kind.
            pub async fn has_ports(&self) -> Vec<Arc<Port>> {
                self.ports.read().await.clone()
            }

            /// @emoji 💾️ Representations owned directly by this kind.
            pub async fn has_representations(&self) -> Vec<Arc<Representation>> {
                self.representations.read().await.clone()
            }
        }

        #[Object(name = "Type")]
        impl Type {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_typology.upgrade().map(crate::gql::interfaces::EntityInterface::Typology)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn description(&self) -> String {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> String {
                self.icon.read().await.clone()
            }
            pub async fn image(&self) -> String {
                self.image.read().await.clone()
            }
            pub async fn unit(&self) -> String {
                self.unit.read().await.clone()
            }
            pub async fn created(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            pub async fn updated(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            /// @emoji ⚓️ Connectors owned directly by this kind.
            #[graphql(name = "hasConnectors")]
            pub async fn has_connectors_field(&self) -> crate::gql_relay::ConnectorConnection {
                crate::gql_relay::ConnectorConnection::from_connectors(self.has_connectors().await).await
            }
            /// @emoji 🔘️ Ports owned directly by this kind.
            #[graphql(name = "hasPorts")]
            pub async fn has_ports_field(&self) -> crate::gql_relay::PortConnection {
                crate::gql_relay::PortConnection::from_entities(self.has_ports().await).await
            }
            pub async fn port(&self, id: Id) -> Option<Arc<Port>> {
                self.refresh_connector_child_weak_maps().await;
                self.port_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            pub async fn connector(&self, id: Id) -> Option<Arc<Connector>> {
                self.refresh_connector_child_weak_maps().await;
                self.connector_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            /// @emoji 💾️ Representations owned directly by this kind.
            #[graphql(name = "hasRepresentations")]
            pub async fn has_representations_field(&self) -> crate::gql_relay::RepresentationConnection {
                crate::gql_relay::RepresentationConnection::from_representations(self.has_representations().await).await
            }
            pub async fn representation(&self, id: Id) -> Option<Arc<Representation>> {
                self.refresh_connector_child_weak_maps().await;
                self.representation_weak_by_id.read().await.get(&id).and_then(|w| w.upgrade())
            }
            #[graphql(name = "bestRepresentation")]
            pub async fn best_representation(&self, tag_ids: Vec<Id>) -> Option<Arc<Representation>> {
                self.best_representation_for_tags(&tag_ids).await
            }
            /// @emoji 📄️ Files referenced indirectly via representations on this kind.
            #[graphql(name = "referencesFiles")]
            pub async fn references_files(&self) -> crate::gql_relay::FileConnection {
                crate::gql_relay::FileConnection::from_entities(self.files_from_representations().await)
            }
            /// @emoji 🪢️ Pieces in the owner kit whose blueprint is this kind.
            #[graphql(name = "referencedBy")]
            pub async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.referenced_by_pieces().await).await
            }
            /// @emoji 🏘️ Designs with a direct piece blueprinting this kind.
            #[graphql(name = "referencedByDesigns")]
            pub async fn referenced_by_designs(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_direct().await).await
            }
            /// @emoji 🏘️ Designs that reference this kind transitively through nested design blueprints.
            #[graphql(name = "referencedByDesignsTransitive")]
            pub async fn referenced_by_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_transitive().await).await
            }
            pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
            }
            pub async fn concepts(&self) -> crate::gql_relay::ConceptConnection {
                crate::gql_relay::ConceptConnection::from_entities(self.concepts.read().await.clone()).await
            }
            pub async fn tags(&self) -> crate::gql_relay::TagConnection {
                crate::gql_relay::TagConnection::from_entities(self.tags.read().await.clone()).await
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_entities(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_entities(self.stats.read().await.clone())
            }

            pub async fn author(&self, id: Id) -> Option<Author> {
                self.authors.read().await.iter().find(|a| a.id == id).cloned()
            }

            pub async fn concept(&self, id: Id) -> Option<Arc<Concept>> {
                self.concepts.read().await.iter().find(|c| c.id == id).cloned()
            }

            pub async fn tag(&self, id: Id) -> Option<Arc<Tag>> {
                self.tags.read().await.iter().find(|t| t.id == id).cloned()
            }

            pub async fn quality(&self, id: Id) -> Option<Arc<Quality>> {
                self.qualities.read().await.iter().find(|q| q.id == id).cloned()
            }

            pub async fn prop(&self, id: Id) -> Option<Prop> {
                self.props.read().await.iter().find(|p| p.id == id).cloned()
            }

            pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                self.attributes.read().await.iter().find(|a| a.id == id).cloned()
            }

            pub async fn stat(&self, id: Id) -> Option<Stat> {
                self.stats.read().await.iter().find(|s| s.id == id).cloned()
            }
        }

        crate::file_system_node_vfs_complex_ctx!(Type, crate::gql::interfaces::file_system_vfs::node_for_type);

        //#endregion 🏠️ type

        //#region 🧩️ blueprint
        #[derive(Clone, Union)]
        #[graphql(name = "Blueprint")]
        pub enum Blueprint {
            Type(Arc<Type>),
            Design(Arc<super::design::Design>),
        }

        impl Default for Blueprint {
            fn default() -> Self {
                Blueprint::Type(Arc::default())
            }
        }
        //#endregion 🧩️ blueprint
    }
    //#endregion 🏠️ type

    //#region 🏘️ design
    pub mod design {
        //! 🏘️ Designs and their pieces, connections, layers, groups.

        //#region ⭕️ piece
        pub mod piece {
            //! ⭕️ Piece (instance of a Type or Design within a Design).
            use std::sync::{Arc, Weak};

            use crate::external_adapters::async_graphql::{Enum, Object};
            use crate::external_adapters::async_lock::RwLock;

            use crate::geom::entity::Position as PositionEntity;
            use crate::geom::PositionInput;
            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::{Attribute, Prop};

            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Enum)]
            #[graphql(name = "PieceConnectionKind")]
            pub enum PieceConnectionKind {
                #[graphql(name = "FIXED")]
                #[default]
                Fixed,
                #[graphql(name = "CONNECTED")]
                Connected,
            }

            pub struct Piece {
                pub id: Id,
                pub owner_design: Weak<super::Design>,
                pub name: RwLock<Option<String>>,
                pub description: RwLock<Option<String>>,
                pub position: RwLock<Option<Arc<PositionEntity>>>,
                pub scale: RwLock<Option<f64>>,
                pub blueprint: RwLock<super::super::r#type::Blueprint>,
                pub connection_kind: RwLock<Option<PieceConnectionKind>>,
                pub parent_piece: RwLock<Weak<Piece>>,
                pub parent_connection: RwLock<Weak<super::connection::Connection>>,
                pub child_pieces: RwLock<Vec<Arc<Piece>>>,
                pub child_connections: RwLock<Vec<Arc<super::connection::Connection>>>,
                pub depth: RwLock<i32>,
                pub path: RwLock<Vec<Weak<Piece>>>,
                pub props: RwLock<Vec<Prop>>,
                pub attributes: RwLock<Vec<Attribute>>,
            }

            impl Default for Piece {
                fn default() -> Self {
                    Self {
                        id: Id::default(),
                        owner_design: Weak::new(),
                        name: RwLock::new(None),
                        description: RwLock::new(None),
                        position: RwLock::new(None),
                        scale: RwLock::new(None),
                        blueprint: RwLock::new(super::super::r#type::Blueprint::default()),
                        connection_kind: RwLock::new(None),
                        parent_piece: RwLock::new(Weak::new()),
                        parent_connection: RwLock::new(Weak::new()),
                        child_pieces: RwLock::new(Vec::new()),
                        child_connections: RwLock::new(Vec::new()),
                        depth: RwLock::new(0),
                        path: RwLock::new(Vec::new()),
                        props: RwLock::new(Vec::new()),
                        attributes: RwLock::new(Vec::new()),
                    }
                }
            }

            impl Piece {
                pub async fn new_fixed(owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: PositionInput) -> Arc<Self> {
                    let pos_node = PositionEntity::from_position_input(position);
                    Arc::new(Self { id: Id::new().await, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                /// 🧾️ Hydrated workspace piece aligned to external JSON id (facade snapshot hydration).
                pub async fn new_fixed_with_external_id(id: Id, owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint, position: PositionInput) -> Arc<Self> {
                    let pos_node = PositionEntity::from_position_input(position);
                    Arc::new(Self { id, owner_design, position: RwLock::new(Some(pos_node)), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Fixed)), ..Default::default() })
                }

                /// @emoji 🔗️ Linked piece without stored absolute pose (position resolved via flatten).
                pub async fn new_connected_with_external_id(id: Id, owner_design: Weak<super::Design>, blueprint: super::super::r#type::Blueprint) -> Arc<Self> {
                    Arc::new(Self { id, owner_design, position: RwLock::new(None), blueprint: RwLock::new(blueprint), connection_kind: RwLock::new(Some(PieceConnectionKind::Connected)), ..Default::default() })
                }

                pub async fn set_name(&self, name: Option<String>) {
                    *self.name.write().await = name;
                }
                pub async fn set_description(&self, description: Option<String>) {
                    *self.description.write().await = description;
                }
                pub async fn set_position(&self, position: Option<PositionInput>) {
                    let mut g = self.position.write().await;
                    *g = position.map(PositionEntity::from_position_input);
                }

                pub async fn compute_hash(&self) -> String {
                    let name = self.name.read().await;
                    h(&[self.id.as_str(), name.as_deref().unwrap_or("")])
                }

                pub async fn compute_flat_position(&self) -> PositionInput {
                    if let Some(design) = self.owner_design.upgrade() {
                        if let Some(topo) = design.owner_typology.upgrade() {
                            if let Some(kit) = topo.owner_kit.upgrade() {
                                let positions = design.flatten_positions(&kit).await;
                                if let Some(pos) = positions.get(&self.id) {
                                    return *pos;
                                }
                            }
                        }
                    }
                    if let Some(n) = self.position.read().await.as_ref() {
                        return n.snapshot_input().await;
                    }
                    PositionInput::default()
                }

                /// @emoji 🧰️ Direct blueprint when this piece is a kind instance.
                pub async fn is_type(&self) -> Option<Arc<super::super::r#type::Type>> {
                    match self.blueprint.read().await.clone() {
                        super::super::r#type::Blueprint::Type(t) => Some(t),
                        super::super::r#type::Blueprint::Design(_) => None,
                    }
                }

                /// @emoji 🏘️ Direct blueprint when this piece is a nested design instance.
                pub async fn is_design(&self) -> Option<Arc<super::Design>> {
                    match self.blueprint.read().await.clone() {
                        super::super::r#type::Blueprint::Design(d) => Some(d),
                        super::super::r#type::Blueprint::Type(_) => None,
                    }
                }

                /// @emoji 🧰️ Kinds reachable through this piece's blueprint, expanding nested designs.
                pub async fn is_types_transitive(&self) -> Vec<Arc<super::super::r#type::Type>> {
                    use std::collections::{HashSet, VecDeque};
                    let mut type_seen = HashSet::new();
                    let mut out = Vec::new();
                    let mut pending: VecDeque<Arc<Piece>> = VecDeque::new();
                    if let Some(t) = self.is_type().await {
                        if type_seen.insert(t.id.clone()) {
                            out.push(t);
                        }
                    }
                    if let Some(d) = self.is_design().await {
                        pending.extend(d.has_pieces().await);
                    }
                    while let Some(piece) = pending.pop_front() {
                        if let Some(t) = piece.is_type().await {
                            if type_seen.insert(t.id.clone()) {
                                out.push(t);
                            }
                        }
                        if let Some(d) = piece.is_design().await {
                            pending.extend(d.has_pieces().await);
                        }
                    }
                    out
                }

                /// @emoji 🏘️ Designs reachable through this piece's blueprint, expanding nested designs.
                pub async fn is_designs_transitive(&self) -> Vec<Arc<super::Design>> {
                    use std::collections::{HashSet, VecDeque};
                    let mut design_seen = HashSet::new();
                    let mut out = Vec::new();
                    let mut pending: VecDeque<Arc<Piece>> = VecDeque::new();
                    if let Some(d) = self.is_design().await {
                        if design_seen.insert(d.id.clone()) {
                            out.push(d.clone());
                            pending.extend(d.has_pieces().await);
                        }
                    }
                    while let Some(piece) = pending.pop_front() {
                        if let Some(d) = piece.is_design().await {
                            if design_seen.insert(d.id.clone()) {
                                out.push(d.clone());
                                pending.extend(d.has_pieces().await);
                            }
                        }
                    }
                    out
                }

                /// @emoji 🪢️ Direct child pieces in the piece tree.
                pub async fn has_pieces(&self) -> Vec<Arc<Piece>> {
                    self.child_pieces.read().await.clone()
                }

                /// @emoji 🔗️ Direct child connections in the piece tree.
                pub async fn has_connections(&self) -> Vec<Arc<super::connection::Connection>> {
                    self.child_connections.read().await.clone()
                }

                /// @emoji 🪢️ All descendant pieces in the piece tree.
                pub async fn has_pieces_transitive(&self) -> Vec<Arc<Piece>> {
                    use std::collections::{HashSet, VecDeque};
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    let mut queue: VecDeque<Arc<Piece>> = self.has_pieces().await.into_iter().collect();
                    while let Some(piece) = queue.pop_front() {
                        if seen.insert(piece.id.clone()) {
                            let children = piece.has_pieces().await;
                            out.push(piece);
                            for child in children {
                                queue.push_back(child);
                            }
                        }
                    }
                    out
                }

                /// @emoji 🔗️ All descendant connections in the piece tree.
                pub async fn has_connections_transitive(&self) -> Vec<Arc<super::connection::Connection>> {
                    use std::collections::{HashSet, VecDeque};
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    let mut queue: VecDeque<Arc<Piece>> = self.has_pieces().await.into_iter().collect();
                    while let Some(piece) = queue.pop_front() {
                        for conn in piece.has_connections().await {
                            if seen.insert(conn.id.clone()) {
                                out.push(conn);
                            }
                        }
                        for child in piece.has_pieces().await {
                            queue.push_back(child);
                        }
                    }
                    out
                }
            }

            #[Object(name = "Piece")]
            impl Piece {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                    if let Some(p) = self.parent_piece.read().await.upgrade() {
                        return Some(crate::gql::interfaces::EntityInterface::Piece(p));
                    }
                    self.owner_design.upgrade().map(crate::gql::interfaces::EntityInterface::Design)
                }
                pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                    Some(crate::gql::interfaces::empty_entity_connection())
                }
                pub async fn blueprint(&self) -> crate::gql::interfaces::EntityInterface {
                    match self.blueprint.read().await.clone() {
                        super::super::r#type::Blueprint::Type(t) => crate::gql::interfaces::EntityInterface::Type(t),
                        super::super::r#type::Blueprint::Design(d) => crate::gql::interfaces::EntityInterface::Design(d),
                    }
                }
                /// @emoji 🧰️ Kind blueprint when this piece instances a kind.
                #[graphql(name = "isType")]
                pub async fn is_type_field(&self) -> Option<Arc<super::super::r#type::Type>> {
                    self.is_type().await
                }
                /// @emoji 🏘️ Design blueprint when this piece instances a nested design.
                #[graphql(name = "isDesign")]
                pub async fn is_design_field(&self) -> Option<Arc<super::Design>> {
                    self.is_design().await
                }
                /// @emoji 🧰️ Kinds reachable transitively through nested design blueprints on this piece.
                #[graphql(name = "isTypesTransitive")]
                pub async fn is_types_transitive_field(&self) -> crate::gql_relay::TypeConnection {
                    crate::gql_relay::TypeConnection::from_types(self.is_types_transitive().await).await
                }
                /// @emoji 🏘️ Designs reachable transitively through nested design blueprints on this piece.
                #[graphql(name = "isDesignsTransitive")]
                pub async fn is_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                    crate::gql_relay::DesignConnection::from_designs(self.is_designs_transitive().await).await
                }
                /// @emoji 🪢️ Direct child pieces in the piece tree.
                #[graphql(name = "hasPieces")]
                pub async fn has_pieces_field(&self) -> crate::gql_relay::PieceConnection {
                    crate::gql_relay::PieceConnection::from_pieces(self.has_pieces().await).await
                }
                /// @emoji 🔗️ Direct child connections in the piece tree.
                #[graphql(name = "hasConnections")]
                pub async fn has_connections_field(&self) -> crate::gql_relay::ConnectionConnection {
                    crate::gql_relay::ConnectionConnection::from_connections(self.has_connections().await).await
                }
                /// @emoji 🪢️ All descendant pieces in the piece tree.
                #[graphql(name = "hasPiecesTransitive")]
                pub async fn has_pieces_transitive_field(&self) -> crate::gql_relay::PieceConnection {
                    crate::gql_relay::PieceConnection::from_pieces(self.has_pieces_transitive().await).await
                }
                /// @emoji 🔗️ All descendant connections in the piece tree.
                #[graphql(name = "hasConnectionsTransitive")]
                pub async fn has_connections_transitive_field(&self) -> crate::gql_relay::ConnectionConnection {
                    crate::gql_relay::ConnectionConnection::from_connections(self.has_connections_transitive().await).await
                }
                pub async fn name(&self) -> Option<String> {
                    self.name.read().await.clone()
                }
                pub async fn description(&self) -> Option<String> {
                    self.description.read().await.clone()
                }
                pub async fn position(&self) -> Option<Arc<PositionEntity>> {
                    self.position.read().await.clone()
                }
                pub async fn scale(&self) -> Option<f64> {
                    *self.scale.read().await
                }
                #[graphql(name = "connectionKind")]
                pub async fn connection_kind(&self) -> Option<PieceConnectionKind> {
                    *self.connection_kind.read().await
                }
                #[graphql(name = "flatPosition")]
                pub async fn flat_position(&self) -> Arc<PositionEntity> {
                    PositionEntity::from_position_input(self.compute_flat_position().await)
                }
                #[graphql(name = "replaceableBlueprints")]
                pub async fn replaceable_blueprints(&self) -> crate::gql_relay::BlueprintConnection {
                    crate::gql_relay::BlueprintConnection::from_blueprints(Vec::new()).await
                }
                #[graphql(name = "parentConnection")]
                pub async fn parent_connection(&self) -> Option<Arc<super::connection::Connection>> {
                    self.parent_connection.read().await.upgrade()
                }
                #[graphql(name = "parentPiece")]
                pub async fn parent_piece(&self) -> Option<Arc<Piece>> {
                    self.parent_piece.read().await.upgrade()
                }
                pub async fn depth(&self) -> i32 {
                    *self.depth.read().await
                }
                pub async fn path(&self) -> Vec<Arc<Piece>> {
                    self.path.read().await.iter().filter_map(|w| w.upgrade()).collect()
                }
                pub async fn props(&self) -> Vec<Prop> {
                    self.props.read().await.clone()
                }

                pub async fn prop(&self, id: Id) -> Option<Prop> {
                    self.props.read().await.iter().find(|p| p.id == id).cloned()
                }

                pub async fn attributes(&self) -> Vec<Attribute> {
                    self.attributes.read().await.clone()
                }

                pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                    self.attributes.read().await.iter().find(|a| a.id == id).cloned()
                }

                #[graphql(name = "childConnection")]
                pub async fn child_connection(&self, id: Id) -> Option<Arc<super::connection::Connection>> {
                    self.child_connections.read().await.iter().find(|c| c.id == id).cloned()
                }

                #[graphql(name = "childPiece")]
                pub async fn child_piece(&self, id: Id) -> Option<Arc<Piece>> {
                    self.child_pieces.read().await.iter().find(|p| p.id == id).cloned()
                }
            }

            crate::file_system_node_vfs_complex_ctx!(Piece, crate::gql::interfaces::file_system_vfs::node_for_piece);
        }
        //#endregion ⭕️ piece

        //#region 🔗️ connection
        pub mod connection {
            //! 🔗️ Connection between two piece sides + the Side value.
            use std::sync::{Arc, Weak};

            use crate::external_adapters::async_graphql::Object;
            use crate::external_adapters::async_lock::RwLock;

            use crate::hash::h;
            use crate::id::Id;
            use crate::meta::Attribute;

            //#region ⛓️ side
            pub struct Side {
                pub id: Id,
                /// @emoji 🔗️ Owning connection when sides are wired into a [`Connection`].
                pub owner_connection: RwLock<Weak<Connection>>,
                pub piece: RwLock<Arc<super::piece::Piece>>,
                pub port: RwLock<Option<Arc<super::super::r#type::Port>>>,
                pub design_piece: RwLock<Option<Arc<super::piece::Piece>>>,
                pub connector: RwLock<Option<Arc<super::super::r#type::Connector>>>,
            }

            impl Default for Side {
                fn default() -> Self {
                    Self { id: Id::default(), owner_connection: RwLock::new(Weak::new()), piece: RwLock::new(Arc::new(super::piece::Piece::default())), port: RwLock::new(None), design_piece: RwLock::new(None), connector: RwLock::new(None) }
                }
            }

            impl Side {
                pub async fn new(piece: Arc<super::piece::Piece>) -> Arc<Self> {
                    Arc::new(Self { id: Id::new().await, piece: RwLock::new(piece), ..Default::default() })
                }

                /// @emoji 🪪️ Blake3 digest over `Side` wire shape: optional `connector` + `designPiece`, then `piece` (matches kit JSON `parent` / `child` objects).
                pub async fn compute_hash(&self) -> String {
                    let mut parts: Vec<String> = vec!["Side".into()];
                    if let Some(co) = self.connector.read().await.as_ref() {
                        parts.push("connector".into());
                        parts.push(co.id.as_str().to_string());
                    }
                    if let Some(dp) = self.design_piece.read().await.as_ref() {
                        parts.push("designPiece".into());
                        parts.push(dp.id.as_str().to_string());
                    }
                    parts.push("piece".into());
                    parts.push(self.piece.read().await.id.as_str().to_string());
                    let refs: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
                    h(&refs)
                }

                /// @emoji 🪢️ Piece referenced by this connection end.
                pub async fn references_piece(&self) -> Arc<super::piece::Piece> {
                    self.piece.read().await.clone()
                }

                /// @emoji ⚓️ Connector referenced by this connection end.
                pub async fn references_connector(&self) -> Option<Arc<super::super::r#type::Connector>> {
                    self.connector.read().await.clone()
                }

                /// @emoji 🔘️ Port referenced by this connection end.
                pub async fn references_port(&self) -> Option<Arc<super::super::r#type::Port>> {
                    self.port.read().await.clone()
                }

                /// @emoji 🪢️ Nested design piece referenced by this connection end.
                pub async fn references_design_piece(&self) -> Option<Arc<super::piece::Piece>> {
                    self.design_piece.read().await.clone()
                }

                /// @emoji 🧰️ Kinds reachable over the referenced piece, expanding nested design blueprints.
                pub async fn references_types_transitive(&self) -> Vec<Arc<super::super::r#type::Type>> {
                    self.references_piece().await.is_types_transitive().await
                }

                /// @emoji ⚓️ Connectors on kinds reachable over the referenced piece (transitive over nested designs).
                pub async fn references_connectors_transitive(&self) -> Vec<Arc<super::super::r#type::Connector>> {
                    use std::collections::HashSet;
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    for ty in self.references_types_transitive().await {
                        for connector in ty.has_connectors().await {
                            if seen.insert(connector.id.clone()) {
                                out.push(connector);
                            }
                        }
                    }
                    out
                }
            }

            //#endregion ⛓️ side

            //#region 🔗️ connection
            pub struct Connection {
                pub id: Id,
                pub owner_design: Weak<super::Design>,
                /// @emoji 🏷️ SDL Artifact `name` / `description` / `icon`.
                pub name: RwLock<String>,
                pub description: RwLock<String>,
                pub icon: RwLock<String>,
                pub parent: RwLock<Arc<Side>>,
                pub child: RwLock<Arc<Side>>,
                pub gap: RwLock<Option<f64>>,
                pub shift: RwLock<Option<f64>>,
                pub rise: RwLock<Option<f64>>,
                pub rotation: RwLock<Option<f64>>,
                pub turn: RwLock<Option<f64>>,
                pub tilt: RwLock<Option<f64>>,
                pub u: RwLock<Option<f64>>,
                pub v: RwLock<Option<f64>>,
                pub attributes: RwLock<Vec<Attribute>>,
            }

            impl Default for Connection {
                fn default() -> Self {
                    Self {
                        id: Id::default(),
                        owner_design: Weak::new(),
                        name: RwLock::new(String::new()),
                        description: RwLock::new(String::new()),
                        icon: RwLock::new(String::new()),
                        parent: RwLock::new(Arc::new(Side::default())),
                        child: RwLock::new(Arc::new(Side::default())),
                        gap: RwLock::new(None),
                        shift: RwLock::new(None),
                        rise: RwLock::new(None),
                        rotation: RwLock::new(None),
                        turn: RwLock::new(None),
                        tilt: RwLock::new(None),
                        u: RwLock::new(None),
                        v: RwLock::new(None),
                        attributes: RwLock::new(Vec::new()),
                    }
                }
            }

            impl Connection {
                /// @emoji 🪪️ Blake3 digest over `Connection` wire shape: sorted `attributes`, `parent` / `child` [`Side`] digests, optional `description`, then scalar join fields.
                pub async fn compute_hash(&self) -> String {
                    use crate::hash::{format_number_for_hash, merkle_collection};
                    let mut parts: Vec<String> = vec!["Connection".into()];
                    let attrs = self.attributes.read().await;
                    if !attrs.is_empty() {
                        let hs: Vec<String> = attrs.iter().map(|a| a.compute_entity_hash()).collect();
                        parts.push("attributes".into());
                        parts.push(merkle_collection(hs));
                    }
                    let parent = self.parent.read().await;
                    let child = self.child.read().await;
                    parts.push("parent".into());
                    parts.push(parent.compute_hash().await);
                    parts.push("child".into());
                    parts.push(child.compute_hash().await);
                    let desc = self.description.read().await.clone();
                    if !desc.is_empty() {
                        parts.push("description".into());
                        parts.push(desc);
                    }
                    parts.push("gap".into());
                    parts.push(format_number_for_hash(self.gap.read().await.unwrap_or(0.0)));
                    parts.push("id".into());
                    parts.push(self.id.as_str().to_string());
                    parts.push("rise".into());
                    parts.push(format_number_for_hash(self.rise.read().await.unwrap_or(0.0)));
                    parts.push("rotation".into());
                    parts.push(format_number_for_hash(self.rotation.read().await.unwrap_or(0.0)));
                    parts.push("shift".into());
                    parts.push(format_number_for_hash(self.shift.read().await.unwrap_or(0.0)));
                    parts.push("tilt".into());
                    parts.push(format_number_for_hash(self.tilt.read().await.unwrap_or(0.0)));
                    parts.push("turn".into());
                    parts.push(format_number_for_hash(self.turn.read().await.unwrap_or(0.0)));
                    parts.push("u".into());
                    parts.push(format_number_for_hash(self.u.read().await.unwrap_or(0.0)));
                    parts.push("v".into());
                    parts.push(format_number_for_hash(self.v.read().await.unwrap_or(0.0)));
                    let refs: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
                    h(&refs)
                }

                /// @emoji ⛓️ Parent and child sides owned by this connection.
                pub async fn has_sides(&self) -> Vec<Arc<Side>> {
                    vec![self.parent.read().await.clone(), self.child.read().await.clone()]
                }

                /// @emoji 🪢️ Pieces referenced by the parent and child sides (deduplicated).
                pub async fn references_pieces(&self) -> Vec<Arc<super::piece::Piece>> {
                    use std::collections::HashSet;
                    let parent = self.parent.read().await;
                    let child = self.child.read().await;
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    for piece in [parent.references_piece().await, child.references_piece().await] {
                        if seen.insert(piece.id.clone()) {
                            out.push(piece);
                        }
                    }
                    out
                }

                /// @emoji ⚓️ Connectors referenced by the parent and child sides (deduplicated).
                pub async fn references_connectors(&self) -> Vec<Arc<super::super::r#type::Connector>> {
                    use std::collections::HashSet;
                    let parent = self.parent.read().await;
                    let child = self.child.read().await;
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    for connector in [parent.references_connector().await, child.references_connector().await].into_iter().flatten() {
                        if seen.insert(connector.id.clone()) {
                            out.push(connector);
                        }
                    }
                    out
                }

                /// @emoji 🪢️ Pieces referenced transitively through nested design blueprints on both sides.
                pub async fn references_pieces_transitive(&self) -> Vec<Arc<super::piece::Piece>> {
                    use std::collections::HashSet;
                    let parent = self.parent.read().await;
                    let child = self.child.read().await;
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    for side in [parent.as_ref(), child.as_ref()] {
                        let mut queue = vec![side.references_piece().await];
                        if let Some(dp) = side.references_design_piece().await {
                            queue.push(dp);
                        }
                        while let Some(piece) = queue.pop() {
                            if seen.insert(piece.id.clone()) {
                                out.push(piece.clone());
                            }
                            for nested in piece.has_pieces_transitive().await {
                                if seen.insert(nested.id.clone()) {
                                    out.push(nested);
                                }
                            }
                            if let Some(d) = piece.is_design().await {
                                for p in d.has_pieces_transitive().await {
                                    if seen.insert(p.id.clone()) {
                                        out.push(p);
                                    }
                                }
                            }
                        }
                    }
                    out
                }

                /// @emoji ⚓️ Connectors referenced transitively on kinds reachable from both sides.
                pub async fn references_connectors_transitive(&self) -> Vec<Arc<super::super::r#type::Connector>> {
                    use std::collections::HashSet;
                    let parent = self.parent.read().await;
                    let child = self.child.read().await;
                    let mut out = Vec::new();
                    let mut seen = HashSet::new();
                    for side in [parent.as_ref(), child.as_ref()] {
                        for connector in side.references_connectors_transitive().await {
                            if seen.insert(connector.id.clone()) {
                                out.push(connector);
                            }
                        }
                    }
                    out
                }
            }

            #[Object(name = "Connection")]
            impl Connection {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                    self.owner_design.upgrade().map(crate::gql::interfaces::EntityInterface::Design)
                }
                pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                    Some(crate::gql::interfaces::empty_entity_connection())
                }
                pub async fn name(&self) -> String {
                    self.name.read().await.clone()
                }
                pub async fn description(&self) -> String {
                    self.description.read().await.clone()
                }
                pub async fn icon(&self) -> String {
                    self.icon.read().await.clone()
                }
                pub async fn parent(&self) -> Arc<Side> {
                    self.parent.read().await.clone()
                }
                pub async fn child(&self) -> Arc<Side> {
                    self.child.read().await.clone()
                }
                pub async fn gap(&self) -> Option<f64> {
                    *self.gap.read().await
                }
                pub async fn shift(&self) -> Option<f64> {
                    *self.shift.read().await
                }
                pub async fn rise(&self) -> Option<f64> {
                    *self.rise.read().await
                }
                pub async fn rotation(&self) -> Option<f64> {
                    *self.rotation.read().await
                }
                pub async fn turn(&self) -> Option<f64> {
                    *self.turn.read().await
                }
                pub async fn tilt(&self) -> Option<f64> {
                    *self.tilt.read().await
                }
                pub async fn u(&self) -> Option<f64> {
                    *self.u.read().await
                }
                pub async fn v(&self) -> Option<f64> {
                    *self.v.read().await
                }
                pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                    crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
                }

                pub async fn attribute(&self, id: Id) -> Option<Attribute> {
                    self.attributes.read().await.iter().find(|a| a.id == id).cloned()
                }

                /// @emoji ⛓️ Parent and child sides owned by this connection.
                #[graphql(name = "hasSides")]
                pub async fn has_sides_field(&self) -> Vec<Arc<Side>> {
                    self.has_sides().await
                }

                /// @emoji 🪢️ Pieces referenced by the parent and child sides.
                #[graphql(name = "referencesPieces")]
                pub async fn references_pieces_field(&self) -> crate::gql_relay::PieceConnection {
                    crate::gql_relay::PieceConnection::from_pieces(self.references_pieces().await).await
                }

                /// @emoji ⚓️ Connectors referenced by the parent and child sides.
                #[graphql(name = "referencesConnectors")]
                pub async fn references_connectors_field(&self) -> crate::gql_relay::ConnectorConnection {
                    crate::gql_relay::ConnectorConnection::from_connectors(self.references_connectors().await).await
                }

                /// @emoji 🪢️ Pieces referenced transitively through nested design blueprints on both sides.
                #[graphql(name = "referencesPiecesTransitive")]
                pub async fn references_pieces_transitive_field(&self) -> crate::gql_relay::PieceConnection {
                    crate::gql_relay::PieceConnection::from_pieces(self.references_pieces_transitive().await).await
                }

                /// @emoji ⚓️ Connectors referenced transitively on kinds reachable from both sides.
                #[graphql(name = "referencesConnectorsTransitive")]
                pub async fn references_connectors_transitive_field(&self) -> crate::gql_relay::ConnectorConnection {
                    crate::gql_relay::ConnectorConnection::from_connectors(self.references_connectors_transitive().await).await
                }
            }

            crate::file_system_node_vfs_complex_ctx!(Connection, crate::gql::interfaces::file_system_vfs::node_for_connection);

            #[Object(name = "Side")]
            impl Side {
                pub async fn id(&self) -> Id {
                    self.id.clone()
                }
                pub async fn hash(&self) -> String {
                    self.compute_hash().await
                }
                pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                    self.owner_connection.read().await.upgrade().map(crate::gql::interfaces::EntityInterface::Connection)
                }
                pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                    Some(crate::gql::interfaces::empty_entity_connection())
                }
                /// @emoji 🪢️ Piece referenced by this connection end.
                #[graphql(name = "referencesPiece")]
                pub async fn references_piece_field(&self) -> Arc<super::piece::Piece> {
                    self.references_piece().await
                }
                /// @emoji 🔘️ Port referenced by this connection end.
                #[graphql(name = "referencesPort")]
                pub async fn references_port_field(&self) -> Option<Arc<super::super::r#type::Port>> {
                    self.references_port().await
                }
                /// @emoji 🪢️ Nested design piece referenced by this connection end.
                #[graphql(name = "referencesDesignPiece")]
                pub async fn references_design_piece_field(&self) -> Option<Arc<super::piece::Piece>> {
                    self.references_design_piece().await
                }
                /// @emoji ⚓️ Connector referenced by this connection end.
                #[graphql(name = "referencesConnector")]
                pub async fn references_connector_field(&self) -> Option<Arc<super::super::r#type::Connector>> {
                    self.references_connector().await
                }
                /// @emoji 🧰️ Kinds reachable over the referenced piece, expanding nested design blueprints.
                #[graphql(name = "referencesTypesTransitive")]
                pub async fn references_types_transitive_field(&self) -> crate::gql_relay::TypeConnection {
                    crate::gql_relay::TypeConnection::from_types(self.references_types_transitive().await).await
                }
                /// @emoji ⚓️ Connectors on kinds reachable over the referenced piece (transitive).
                #[graphql(name = "referencesConnectorsTransitive")]
                pub async fn references_connectors_transitive_field(&self) -> crate::gql_relay::ConnectorConnection {
                    crate::gql_relay::ConnectorConnection::from_connectors(self.references_connectors_transitive().await).await
                }
            }
            //#endregion 🔗️ connection
        }

        //#region 🏘️ design
        use std::collections::HashMap;
        use std::sync::{Arc, Weak};

        use crate::external_adapters::async_graphql::Object;
        use crate::external_adapters::async_lock::RwLock;

        use crate::geom::entity::Location;
        use crate::hash::h;
        use crate::id::Id;
        use crate::meta::{Attribute, Author, Group, Layer, Prop, Quality, Stat};
        use crate::timestamp::Timestamp;

        //#region 🧱️ clump
        /// @emoji 🧱️ SDL `Clump` — connected-component bucket for layout (`WeakEntity` projection hook).
        pub struct Clump {
            pub id: Id,
            pub owner_design: Weak<Design>,
        }

        impl Default for Clump {
            fn default() -> Self {
                Self { id: Id::default(), owner_design: Weak::new() }
            }
        }

        #[Object(name = "Clump")]
        impl Clump {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                h(&[self.id.as_str()])
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_design.upgrade().map(crate::gql::interfaces::EntityInterface::Design)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            #[graphql(name = "fixedPiece")]
            pub async fn fixed_piece(&self) -> Option<Arc<piece::Piece>> {
                None
            }
            #[graphql(name = "connectedPieces")]
            pub async fn connected_pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
            pub async fn pieces(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(Vec::new()).await
            }
        }
        //#endregion 🧱️ clump

        pub struct Design {
            pub id: Id,
            pub owner_typology: Weak<crate::gql_relay::Typology>,
            pub name: RwLock<String>,
            pub description: RwLock<Option<String>>,
            pub icon: RwLock<Option<String>>,
            pub image: RwLock<Option<String>>,
            pub location: RwLock<Option<Arc<Location>>>,
            pub unit: RwLock<Option<String>>,
            pub created: RwLock<Option<Timestamp>>,
            pub updated: RwLock<Option<Timestamp>>,
            pub pieces: RwLock<Vec<Arc<piece::Piece>>>,
            /// 🧷️ Write-side only: external piece [`Id`] → `Weak` (GraphQL `piece(id:)` upgrades here; no vec index table).
            pub piece_weak_by_external_id: RwLock<HashMap<Id, Weak<piece::Piece>>>,
            pub connections: RwLock<Vec<Arc<connection::Connection>>>,
            flat_positions_cache: RwLock<Option<HashMap<Id, crate::geom::PositionInput>>>,
            pub layers: RwLock<Vec<Layer>>,
            pub groups: RwLock<Vec<Group>>,
            pub authors: RwLock<Vec<Author>>,
            pub qualities: RwLock<Vec<Arc<Quality>>>,
            pub props: RwLock<Vec<Prop>>,
            pub attributes: RwLock<Vec<Attribute>>,
            pub stats: RwLock<Vec<Stat>>,
            pub folder_id: RwLock<Option<Id>>,
        }

        impl Default for Design {
            fn default() -> Self {
                Self {
                    id: Id::default(),
                    owner_typology: Weak::new(),
                    name: RwLock::new(String::new()),
                    description: RwLock::new(None),
                    icon: RwLock::new(None),
                    image: RwLock::new(None),
                    location: RwLock::new(None),
                    unit: RwLock::new(None),
                    created: RwLock::new(None),
                    updated: RwLock::new(None),
                    pieces: RwLock::new(Vec::new()),
                    piece_weak_by_external_id: RwLock::new(HashMap::new()),
                    connections: RwLock::new(Vec::new()),
                    flat_positions_cache: RwLock::new(None),
                    layers: RwLock::new(Vec::new()),
                    groups: RwLock::new(Vec::new()),
                    authors: RwLock::new(Vec::new()),
                    qualities: RwLock::new(Vec::new()),
                    props: RwLock::new(Vec::new()),
                    attributes: RwLock::new(Vec::new()),
                    stats: RwLock::new(Vec::new()),
                    folder_id: RwLock::new(None),
                }
            }
        }

        impl Design {
            pub async fn owner_kit(&self) -> Option<Arc<crate::kit::Kit>> {
                self.owner_typology.upgrade()?.owner_kit.upgrade()
            }

            pub async fn new(owner_typology: Weak<crate::gql_relay::Typology>, name: String) -> Arc<Self> {
                Arc::new(Self { id: Id::new().await, owner_typology, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn with_id(owner_typology: Weak<crate::gql_relay::Typology>, id: Id, name: String) -> Arc<Self> {
                Arc::new(Self { id, owner_typology, name: RwLock::new(name), ..Default::default() })
            }

            pub async fn compute_hash(&self) -> String {
                let name = self.name.read().await;
                h(&[self.id.as_str(), name.as_str()])
            }

            /// @emoji 🌤️ Cached absolute positions for every piece in this design.
            pub async fn flatten_positions(self: &Arc<Self>, kit: &Arc<crate::kit::Kit>) -> HashMap<Id, crate::geom::PositionInput> {
                if let Some(cached) = self.flat_positions_cache.read().await.clone() {
                    return cached;
                }
                let computed = crate::geom::flatten::flatten_design_positions(kit, self).await;
                *self.flat_positions_cache.write().await = Some(computed.clone());
                computed
            }

            /// @emoji 🧹️ Drops cached flatten output after topology edits.
            pub async fn invalidate_flat_positions_cache(&self) {
                *self.flat_positions_cache.write().await = None;
            }

            /// 🆕️ Push a piece into this design's pieces; returns the same Arc (refcount + 1) for the caller.
            pub async fn insert_piece(&self, piece: Arc<piece::Piece>) -> Arc<piece::Piece> {
                self.invalidate_flat_positions_cache().await;
                let mut pieces = self.pieces.write().await;
                let mut weak_ix = self.piece_weak_by_external_id.write().await;
                let pid = piece.id.clone();
                weak_ix.insert(pid, Arc::downgrade(&piece));
                pieces.push(piece.clone());
                piece
            }

            /// @emoji 🗑️ Remove a piece from this design's ordered list and external-id index.
            pub async fn delete_piece_by_external_id(&self, piece_id: &Id) -> Result<(), crate::error::ComposeError> {
                self.invalidate_flat_positions_cache().await;
                let mut pieces = self.pieces.write().await;
                let start_len = pieces.len();
                pieces.retain(|piece| &piece.id != piece_id);
                if pieces.len() == start_len {
                    return Err(crate::error::ComposeError::not_found("Piece", piece_id.as_str()));
                }
                self.piece_weak_by_external_id.write().await.remove(piece_id);
                Ok(())
            }

            /// @emoji 🪢️ Command / GraphQL boundary: resolve a piece by external [`Id`] via the write-side weak map.
            pub async fn piece_by_external_id(&self, id: &Id) -> Option<Arc<piece::Piece>> {
                self.piece_weak_by_external_id.read().await.get(id).and_then(|w| w.upgrade())
            }

            /// @emoji 🧰️ Distinct [`Type`] blueprints on this design's own pieces (one hop).
            pub async fn references_types(&self) -> Vec<Arc<super::r#type::Type>> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for piece in self.has_pieces().await {
                    if let Some(t) = piece.is_type().await {
                        if seen.insert(t.id.clone()) {
                            out.push(t);
                        }
                    }
                }
                out
            }

            /// @emoji 🏘️ Distinct [`Design`] blueprints on this design's own pieces (one hop).
            pub async fn references_designs(&self) -> Vec<Arc<Design>> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for piece in self.has_pieces().await {
                    if let Some(d) = piece.is_design().await {
                        if seen.insert(d.id.clone()) {
                            out.push(d);
                        }
                    }
                }
                out
            }

            /// @emoji 📄️ Files from representations of direct [`Type`] blueprints on this design's pieces.
            pub async fn references_files(&self) -> Vec<crate::meta::File> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for t in self.references_types().await {
                    for f in t.files_from_representations().await {
                        if seen.insert(f.id.clone()) {
                            out.push(f);
                        }
                    }
                }
                out
            }

            /// @emoji 💾️ Representations on direct [`Type`] blueprints on this design's pieces.
            pub async fn references_representations(&self) -> Vec<Arc<super::r#type::Representation>> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for t in self.references_types().await {
                    for r in t.representations.read().await.iter() {
                        if seen.insert(r.id.clone()) {
                            out.push(r.clone());
                        }
                    }
                }
                out
            }

            /// @emoji 💾️ Representations on [`Type`] blueprints reachable transitively through nested designs.
            pub async fn references_representations_transitive(&self) -> Vec<Arc<super::r#type::Representation>> {
                use std::collections::HashSet;
                let mut out = Vec::new();
                let mut seen = HashSet::new();
                for t in self.references_types_transitive().await {
                    for r in t.representations.read().await.iter() {
                        if seen.insert(r.id.clone()) {
                            out.push(r.clone());
                        }
                    }
                }
                out
            }

            async fn collect_transitive_references_from_design(
                design: &Design,
                root_design_id: &Id,
                type_seen: &mut std::collections::HashSet<Id>,
                design_seen: &mut std::collections::HashSet<Id>,
                file_seen: &mut std::collections::HashSet<Id>,
                all_types: &mut Vec<Arc<super::r#type::Type>>,
                all_designs: &mut Vec<Arc<Design>>,
                all_files: &mut Vec<crate::meta::File>,
                pending_designs: &mut std::collections::VecDeque<Arc<Design>>,
            ) {
                for piece in design.has_pieces().await {
                    if let Some(t) = piece.is_type().await {
                        if type_seen.insert(t.id.clone()) {
                            all_types.push(t.clone());
                            for f in t.files_from_representations().await {
                                if file_seen.insert(f.id.clone()) {
                                    all_files.push(f);
                                }
                            }
                        }
                    } else if let Some(d) = piece.is_design().await {
                        if d.id == *root_design_id {
                            continue;
                        }
                        if design_seen.insert(d.id.clone()) {
                            pending_designs.push_back(d.clone());
                            all_designs.push(d);
                        }
                    }
                }
            }

            /// @emoji 🧰️ Kinds referenced transitively through nested design blueprints.
            pub async fn references_types_transitive(&self) -> Vec<Arc<super::r#type::Type>> {
                self.transitive_reference_closure().await.0
            }

            /// @emoji 🏘️ Designs referenced transitively through nested design blueprints.
            pub async fn references_designs_transitive(&self) -> Vec<Arc<Design>> {
                self.transitive_reference_closure().await.1
            }

            /// @emoji 📄️ Files referenced transitively through nested design and kind blueprints.
            pub async fn references_files_transitive(&self) -> Vec<crate::meta::File> {
                self.transitive_reference_closure().await.2
            }

            /// @emoji 🪢️ Pieces owned directly by this design.
            pub async fn has_pieces(&self) -> Vec<Arc<piece::Piece>> {
                self.pieces.read().await.clone()
            }

            /// @emoji 🔗️ Connections owned directly by this design.
            pub async fn has_connections(&self) -> Vec<Arc<connection::Connection>> {
                self.connections.read().await.clone()
            }

            /// @emoji 🎨️ Layers owned directly by this design.
            pub async fn has_layers(&self) -> Vec<Layer> {
                self.layers.read().await.clone()
            }

            /// @emoji 👥️ Groups owned directly by this design.
            pub async fn has_groups(&self) -> Vec<Group> {
                self.groups.read().await.clone()
            }

            /// @emoji 🪢️ Pieces in this design and nested design blueprints (transitive).
            pub async fn has_pieces_transitive(&self) -> Vec<Arc<piece::Piece>> {
                use std::collections::{HashSet, VecDeque};
                let mut piece_seen = HashSet::new();
                let mut design_seen = HashSet::new();
                let mut out = Vec::new();
                let mut pending: VecDeque<Arc<Design>> = VecDeque::new();
                design_seen.insert(self.id.clone());
                for piece in self.has_pieces().await {
                    let nested = piece.is_design().await;
                    if piece_seen.insert(piece.id.clone()) {
                        out.push(piece);
                    }
                    if let Some(nested) = nested {
                        if design_seen.insert(nested.id.clone()) {
                            pending.push_back(nested);
                        }
                    }
                }
                while let Some(design) = pending.pop_front() {
                    for piece in design.has_pieces().await {
                        let nested = piece.is_design().await;
                        if piece_seen.insert(piece.id.clone()) {
                            out.push(piece);
                        }
                        if let Some(nested) = nested {
                            if design_seen.insert(nested.id.clone()) {
                                pending.push_back(nested);
                            }
                        }
                    }
                }
                out
            }

            /// @emoji 🔗️ Connections in this design and nested design blueprints (transitive).
            pub async fn has_connections_transitive(&self) -> Vec<Arc<connection::Connection>> {
                use std::collections::{HashSet, VecDeque};
                let mut connection_seen = HashSet::new();
                let mut design_seen = HashSet::new();
                let mut out = Vec::new();
                let mut pending: VecDeque<Arc<Design>> = VecDeque::new();
                design_seen.insert(self.id.clone());
                for connection in self.has_connections().await {
                    if connection_seen.insert(connection.id.clone()) {
                        out.push(connection);
                    }
                }
                for piece in self.has_pieces().await {
                    if let Some(nested) = piece.is_design().await {
                        if design_seen.insert(nested.id.clone()) {
                            pending.push_back(nested);
                        }
                    }
                }
                while let Some(design) = pending.pop_front() {
                    for connection in design.has_connections().await {
                        if connection_seen.insert(connection.id.clone()) {
                            out.push(connection);
                        }
                    }
                    for piece in design.has_pieces().await {
                        if let Some(nested) = piece.is_design().await {
                            if design_seen.insert(nested.id.clone()) {
                                pending.push_back(nested);
                            }
                        }
                    }
                }
                out
            }

            /// @emoji 🔗️ Transitive closure of referenced types, designs, and files through nested design blueprints.
            pub async fn transitive_reference_closure(&self) -> (Vec<Arc<super::r#type::Type>>, Vec<Arc<Design>>, Vec<crate::meta::File>) {
                use std::collections::{HashSet, VecDeque};
                let mut all_types = Vec::new();
                let mut all_designs = Vec::new();
                let mut all_files = Vec::new();
                let mut type_seen = HashSet::new();
                let mut design_seen = HashSet::new();
                let mut file_seen = HashSet::new();
                let mut pending = VecDeque::new();
                let root_id = self.id.clone();
                Self::collect_transitive_references_from_design(self, &root_id, &mut type_seen, &mut design_seen, &mut file_seen, &mut all_types, &mut all_designs, &mut all_files, &mut pending).await;
                while let Some(d) = pending.pop_front() {
                    Self::collect_transitive_references_from_design(d.as_ref(), &root_id, &mut type_seen, &mut design_seen, &mut file_seen, &mut all_types, &mut all_designs, &mut all_files, &mut pending).await;
                }
                (all_types, all_designs, all_files)
            }

            /// @emoji 🪢️ Pieces anywhere in the owner kit whose blueprint is this design.
            pub async fn referenced_by_pieces(&self) -> Vec<Arc<piece::Piece>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.pieces_with_blueprint_design(&self.id).await
            }

            /// @emoji 🏘️ Designs with a direct piece blueprinting this design.
            pub async fn referenced_by_designs_direct(&self) -> Vec<Arc<Design>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_with_direct_blueprint_design(&self.id).await
            }

            /// @emoji 🏘️ Designs that reference this design transitively through nested design blueprints.
            pub async fn referenced_by_designs_transitive(&self) -> Vec<Arc<Design>> {
                let Some(kit) = self.owner_kit().await else {
                    return Vec::new();
                };
                kit.designs_referencing_design_transitive(&self.id).await
            }
        }

        #[Object(name = "Design")]
        impl Design {
            pub async fn id(&self) -> Id {
                self.id.clone()
            }
            pub async fn hash(&self) -> String {
                self.compute_hash().await
            }
            pub async fn owner(&self) -> Option<crate::gql::interfaces::EntityInterface> {
                self.owner_typology.upgrade().map(crate::gql::interfaces::EntityInterface::Typology)
            }
            pub async fn owns(&self) -> Option<crate::gql::interfaces::EntityConnectionInterface> {
                Some(crate::gql::interfaces::empty_entity_connection())
            }
            pub async fn name(&self) -> String {
                self.name.read().await.clone()
            }
            pub async fn description(&self) -> Option<String> {
                self.description.read().await.clone()
            }
            pub async fn icon(&self) -> Option<String> {
                self.icon.read().await.clone()
            }
            pub async fn image(&self) -> Option<String> {
                self.image.read().await.clone()
            }
            pub async fn location(&self) -> Option<Arc<Location>> {
                self.location.read().await.clone()
            }
            pub async fn unit(&self) -> Option<String> {
                self.unit.read().await.clone()
            }
            #[graphql(name = "createdAt")]
            pub async fn created_at(&self) -> Option<Timestamp> {
                self.created.read().await.clone()
            }
            #[graphql(name = "updatedAt")]
            pub async fn updated_at(&self) -> Option<Timestamp> {
                self.updated.read().await.clone()
            }
            /// @emoji 🪢️ Pieces owned directly by this design.
            #[graphql(name = "hasPieces")]
            pub async fn has_pieces_field(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.has_pieces().await).await
            }
            pub async fn piece(&self, id: Id) -> Option<Arc<piece::Piece>> {
                self.piece_by_external_id(&id).await
            }
            /// @emoji 🔗️ Connections owned directly by this design.
            #[graphql(name = "hasConnections")]
            pub async fn has_connections_field(&self) -> crate::gql_relay::ConnectionConnection {
                crate::gql_relay::ConnectionConnection::from_connections(self.has_connections().await).await
            }
            pub async fn connection(&self, id: Id) -> Option<Arc<connection::Connection>> {
                self.connections.read().await.iter().find(|c| c.id == id).cloned()
            }
            /// @emoji 🎨️ Layers owned directly by this design.
            #[graphql(name = "hasLayers")]
            pub async fn has_layers_field(&self) -> crate::gql_relay::LayerConnection {
                crate::gql_relay::LayerConnection::from_entities(self.has_layers().await)
            }
            /// @emoji 👥️ Groups owned directly by this design.
            #[graphql(name = "hasGroups")]
            pub async fn has_groups_field(&self) -> crate::gql_relay::GroupConnection {
                crate::gql_relay::GroupConnection::from_entities(self.has_groups().await)
            }
            /// @emoji 🪢️ Pieces in this design and nested design blueprints (transitive).
            #[graphql(name = "hasPiecesTransitive")]
            pub async fn has_pieces_transitive_field(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.has_pieces_transitive().await).await
            }
            /// @emoji 🔗️ Connections in this design and nested design blueprints (transitive).
            #[graphql(name = "hasConnectionsTransitive")]
            pub async fn has_connections_transitive_field(&self) -> crate::gql_relay::ConnectionConnection {
                crate::gql_relay::ConnectionConnection::from_connections(self.has_connections_transitive().await).await
            }
            pub async fn authors(&self) -> crate::gql_relay::AuthorConnection {
                crate::gql_relay::AuthorConnection::from_entities(self.authors.read().await.clone())
            }
            pub async fn qualities(&self) -> crate::gql_relay::QualityConnection {
                crate::gql_relay::QualityConnection::from_entities(self.qualities.read().await.clone()).await
            }
            pub async fn props(&self) -> crate::gql_relay::PropConnection {
                crate::gql_relay::PropConnection::from_entities(self.props.read().await.clone())
            }
            pub async fn attributes(&self) -> crate::gql_relay::AttributeConnection {
                crate::gql_relay::AttributeConnection::from_entities(self.attributes.read().await.clone())
            }
            pub async fn stats(&self) -> crate::gql_relay::StatConnection {
                crate::gql_relay::StatConnection::from_entities(self.stats.read().await.clone())
            }
            #[graphql(name = "qualitySum")]
            pub async fn quality_sum(&self, _quality_id: Id) -> f64 {
                0.0
            }

            /// @emoji 🧰️ Kinds referenced by this design's pieces (one hop over blueprints).
            #[graphql(name = "referencesTypes")]
            pub async fn references_types_field(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.references_types().await).await
            }
            /// @emoji 🏘️ Designs referenced by this design's pieces (one hop over blueprints).
            #[graphql(name = "referencesDesigns")]
            pub async fn references_designs_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.references_designs().await).await
            }
            /// @emoji 📄️ Files referenced via kind blueprints on this design's pieces (one hop).
            #[graphql(name = "referencesFiles")]
            pub async fn references_files_field(&self) -> crate::gql_relay::FileConnection {
                crate::gql_relay::FileConnection::from_entities(self.references_files().await)
            }
            /// @emoji 💾️ Representations on kind blueprints referenced by this design's pieces (one hop).
            #[graphql(name = "referencesRepresentations")]
            pub async fn references_representations_field(&self) -> crate::gql_relay::RepresentationConnection {
                crate::gql_relay::RepresentationConnection::from_representations(self.references_representations().await).await
            }
            /// @emoji 💾️ Representations on kinds referenced transitively through nested design blueprints.
            #[graphql(name = "referencesRepresentationsTransitive")]
            pub async fn references_representations_transitive_field(&self) -> crate::gql_relay::RepresentationConnection {
                crate::gql_relay::RepresentationConnection::from_representations(self.references_representations_transitive().await).await
            }
            /// @emoji 🧰️ Kinds referenced transitively through nested design blueprints.
            #[graphql(name = "referencesTypesTransitive")]
            pub async fn references_types_transitive_field(&self) -> crate::gql_relay::TypeConnection {
                crate::gql_relay::TypeConnection::from_types(self.references_types_transitive().await).await
            }
            /// @emoji 🏘️ Designs referenced transitively through nested design blueprints.
            #[graphql(name = "referencesDesignsTransitive")]
            pub async fn references_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.references_designs_transitive().await).await
            }
            /// @emoji 📄️ Files referenced transitively through nested design and kind blueprints.
            #[graphql(name = "referencesFilesTransitive")]
            pub async fn references_files_transitive_field(&self) -> crate::gql_relay::FileConnection {
                crate::gql_relay::FileConnection::from_entities(self.references_files_transitive().await)
            }
            /// @emoji 🪢️ Pieces in the owner kit whose blueprint is this design.
            #[graphql(name = "referencedBy")]
            pub async fn referenced_by(&self) -> crate::gql_relay::PieceConnection {
                crate::gql_relay::PieceConnection::from_pieces(self.referenced_by_pieces().await).await
            }
            /// @emoji 🏘️ Designs with a direct piece blueprinting this design.
            #[graphql(name = "referencedByDesigns")]
            pub async fn referenced_by_designs(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_direct().await).await
            }
            /// @emoji 🏘️ Designs that reference this design transitively through nested design blueprints.
            #[graphql(name = "referencedByDesignsTransitive")]
            pub async fn referenced_by_designs_transitive_field(&self) -> crate::gql_relay::DesignConnection {
                crate::gql_relay::DesignConnection::from_designs(self.referenced_by_designs_transitive().await).await
            }
        }

        crate::file_system_node_vfs_complex_ctx!(Design, crate::gql::interfaces::file_system_vfs::node_for_design);
        //#endregion 🏘️ design
    }
    //#endregion 🏘️ design

    //#region 📚️ kit_target_operations
    /// 🧾️ Arc-backed operation `*Input` shells for Quality / Tag / Concept / Port (`schema.golden.graphql` nested Operations block).
    pub mod target_operations {
        use std::sync::Arc;

        use crate::external_adapters::async_graphql::SimpleObject;

        use crate::gql_relay::{AttributeConnection, ConceptConnection, PortConnection, QualityConnection, TagConnection};
        use crate::kit::r#type::Port;
        use crate::meta::{Attribute, Concept, Quality, Tag};

        //#region 🔖️ Quality inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedQualityInput")]
        pub struct CreatedQualityInput {
            pub quality: Arc<Quality>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedQualitiesInput")]
        pub struct CreatedQualitiesInput {
            pub qualities: QualityConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedQualityInput")]
        pub struct RenamedQualityInput {
            pub key: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedQualityDescriptionInput")]
        pub struct UpdatedQualityDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedQualityIconInput")]
        pub struct UpdatedQualityIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToQualityInput")]
        pub struct AddedAttributeToQualityInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToQualityInput")]
        pub struct AddedAttributesToQualityInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromQualityInput")]
        pub struct RemovedAttributeFromQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromQualityInput")]
        pub struct RemovedAttributesFromQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedQualityInput")]
        pub struct DeletedQualityInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedQualitiesInput")]
        pub struct DeletedQualitiesInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🔖️ Quality inputs

        //#region 🏷️ Tag inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedTagInput")]
        pub struct CreatedTagInput {
            pub tag: Arc<Tag>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedTagsInput")]
        pub struct CreatedTagsInput {
            pub tags: TagConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedTagInput")]
        pub struct RenamedTagInput {
            pub name: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedTagDescriptionInput")]
        pub struct UpdatedTagDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedTagIconInput")]
        pub struct UpdatedTagIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToTagInput")]
        pub struct AddedAttributeToTagInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToTagInput")]
        pub struct AddedAttributesToTagInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromTagInput")]
        pub struct RemovedAttributeFromTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromTagInput")]
        pub struct RemovedAttributesFromTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedTagInput")]
        pub struct DeletedTagInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedTagsInput")]
        pub struct DeletedTagsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🏷️ Tag inputs

        //#region 💡️ Concept inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedConceptInput")]
        pub struct CreatedConceptInput {
            pub concept: Arc<Concept>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedConceptsInput")]
        pub struct CreatedConceptsInput {
            pub concepts: ConceptConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedConceptInput")]
        pub struct RenamedConceptInput {
            pub name: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedConceptDescriptionInput")]
        pub struct UpdatedConceptDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedConceptIconInput")]
        pub struct UpdatedConceptIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToConceptInput")]
        pub struct AddedAttributeToConceptInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToConceptInput")]
        pub struct AddedAttributesToConceptInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromConceptInput")]
        pub struct RemovedAttributeFromConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromConceptInput")]
        pub struct RemovedAttributesFromConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedConceptInput")]
        pub struct DeletedConceptInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedConceptsInput")]
        pub struct DeletedConceptsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 💡️ Concept inputs

        //#region 🔌️ Port inputs
        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedPortInput")]
        pub struct CreatedPortInput {
            pub port: Arc<Port>,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "CreatedPortsInput")]
        pub struct CreatedPortsInput {
            pub ports: PortConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RenamedPortInput")]
        pub struct RenamedPortInput {
            pub code: String,
            pub label: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedPortDescriptionInput")]
        pub struct UpdatedPortDescriptionInput {
            pub description: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "UpdatedPortIconInput")]
        pub struct UpdatedPortIconInput {
            pub icon: String,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributeToPortInput")]
        pub struct AddedAttributeToPortInput {
            pub attribute: Attribute,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "AddedAttributesToPortInput")]
        pub struct AddedAttributesToPortInput {
            pub attributes: AttributeConnection,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributeFromPortInput")]
        pub struct RemovedAttributeFromPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "RemovedAttributesFromPortInput")]
        pub struct RemovedAttributesFromPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedPortInput")]
        pub struct DeletedPortInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }

        #[derive(Clone, SimpleObject)]
        #[graphql(name = "DeletedPortsInput")]
        pub struct DeletedPortsInput {
            #[graphql(name = "hasInput")]
            pub has_input: bool,
        }
        //#endregion 🔌️ Port inputs
    }
    //#endregion 📚️ kit_target_operations

    //#region 📦️ kit
    use std::collections::HashMap;
    use std::sync::{Arc, Weak};

    use crate::external_adapters::async_graphql::Object;
    use crate::external_adapters::async_lock::RwLock;

    use crate::gql_relay::{Family, Typology};
    use crate::hash::h;
    use crate::id::Id;
    use crate::meta::{Attribute, Author, Concept, File, Folder, Prop, Quality, Stat, Tag};
    use crate::timestamp::Timestamp;

    pub struct Kit {
        pub id: Id,
        pub owner_graph: Weak<crate::vcs::Graph>,
        pub name: RwLock<String>,
        pub description: RwLock<Option<String>>,
        pub icon: RwLock<Option<String>>,
        pub image: RwLock<Option<String>>,
        pub preview: RwLock<Option<String>>,
        pub remote: RwLock<Option<String>>,
        pub homepage: RwLock<Option<String>>,
        pub license: RwLock<Option<String>>,
        pub uri: RwLock<Option<String>>,
        pub created: RwLock<Option<Timestamp>>,
        pub updated: RwLock<Option<Timestamp>>,
        pub version: RwLock<Option<String>>,
        /// 🏛️ Typologies own kit [`Type`] and [`Design`] entities (kit no longer stores them directly).
        pub typologies: RwLock<Vec<Arc<Typology>>>,
        /// 🧷️ Kit-wide type [`Id`] → `Weak` for GraphQL `type(id:)` across all typologies.
        pub type_weak_by_id: RwLock<HashMap<Id, Weak<r#type::Type>>>,
        /// 🧷️ Kit-wide design [`Id`] → `Weak` for GraphQL `design(id:)` across all typologies.
        pub design_weak_by_id: RwLock<HashMap<Id, Weak<design::Design>>>,
        pub files: RwLock<Vec<File>>,
        pub folders: RwLock<Vec<Folder>>,
        pub families: RwLock<Vec<Family>>,
        pub authors: RwLock<Vec<Author>>,
        pub concepts: RwLock<Vec<Arc<Concept>>>,
        pub tags: RwLock<Vec<Arc<Tag>>>,
        pub qualities: RwLock<Vec<Arc<Quality>>>,
        pub props: RwLock<Vec<Prop>>,
        pub attributes: RwLock<Vec<Attribute>>,
        pub stats: RwLock<Vec<Stat>>,
        /// 🧷️ Kit-wide tag identity map (all tag owners).
        pub tag_by_id: RwLock<HashMap<Id, Arc<Tag>>>,
        pub concept_by_id: RwLock<HashMap<Id, Arc<Concept>>>,
        pub quality_by_id: RwLock<HashMap<Id, Arc<Quality>>>,
        /// @emoji 🔢️ Monotonic counter bumped by every GraphQL graph mutation (test / backbone observability).
        pub touch_epoch: RwLock<u64>,
        /// 🧭️ Optional client-facing kit id from WASM/JSON hydration (`@semio_compose_rs/js` DTO `id`); when None, fall back to internally minted [`Kit::id`].
        pub snapshot_external_kit_id: RwLock<Option<Id>>,
        /// @emoji 👨️‍👩️‍👦️ Preserved `families` projection subtree (kit-level ports) for `initialKit` round-trips.
        pub snapshot_families_projection: RwLock<Option<crate::external_adapters::serde_json::Value>>,
    }

    impl Default for Kit {
        fn default() -> Self {
            Self {
                id: Id::default(),
                owner_graph: Weak::new(),
                name: RwLock::new(String::new()),
                description: RwLock::new(None),
                icon: RwLock::new(None),
                image: RwLock::new(None),
                preview: RwLock::new(None),
                remote: RwLock::new(None),
                homepage: RwLock::new(None),
                license: RwLock::new(None),
                uri: RwLock::new(None),
                created: RwLock::new(None),
                updated: RwLock::new(None),
                version: RwLock::new(None),
                typologies: RwLock::new(Vec::new()),
                type_weak_by_id: RwLock::new(HashMap::new()),
                design_weak_by_id: RwLock::new(HashMap::new()),
                files: RwLock::new(Vec::new()),
                folders: RwLock::new(Vec::new()),
                families: RwLock::new(Vec::new()),
                authors: RwLock::new(Vec::new()),
                concepts: RwLock::new(Vec::new()),
                tags: RwLock::new(Vec::new()),
                qualities: RwLock::new(Vec::new()),
                props: RwLock::new(Vec::new()),
                attributes: RwLock::new(Vec::new()),
                stats: RwLock::new(Vec::new()),
                tag_by_id: RwLock::new(HashMap::new()),
                concept_by_id: RwLock::new(HashMap::new()),
                quality_by_id: RwLock::new(HashMap::new()),
                touch_epoch: RwLock::new(0),
                snapshot_external_kit_id: RwLock::new(None),
                snapshot_families_projection: RwLock::new(None),
            }
        }
    }

    impl Kit {
        pub async fn new(owner_graph: Weak<crate::vcs::Graph>, name: String) -> Arc<Self> {
            Arc::new(Self { id: Id::new().await, owner_graph, name: RwLock::new(name), ..Default::default() })
        }

        pub(crate) fn new_sync(owner_graph: Weak<crate::vcs::Graph>, name: String) -> Arc<Self> {
            Arc::new(Self { id: Id::new_sync(), owner_graph, name: RwLock::new(name), ..Default::default() })
        }

        pub async fn workspace_kit_id(&self) -> Id {
            self.snapshot_external_kit_id.read().await.clone().unwrap_or_else(|| self.id.clone())
        }

        pub async fn bump_touch_epoch(&self) {
            let mut g = self.touch_epoch.write().await;
            *g = g.saturating_add(1);
        }

        /// @emoji 🏛️ Flatten all types owned by typologies (computed kit view).
        pub async fn types_flat(&self) -> Vec<Arc<r#type::Type>> {
            let mut out = Vec::new();
            for topo in self.typologies.read().await.iter() {
                out.extend(topo.types.read().await.iter().cloned());
            }
            out
        }

        /// @emoji 🏛️ Flatten all designs owned by typologies (computed kit view).
        pub async fn designs_flat(&self) -> Vec<Arc<design::Design>> {
            let mut out = Vec::new();
            for topo in self.typologies.read().await.iter() {
                out.extend(topo.designs.read().await.iter().cloned());
            }
            out
        }

        /// @emoji 🏘️ Designs contained in this kit (in-memory projection).
        pub async fn has_designs(&self) -> Vec<Arc<design::Design>> {
            self.designs_flat().await
        }

        /// @emoji 🧰️ Kinds contained in this kit (in-memory projection).
        pub async fn has_types(&self) -> Vec<Arc<r#type::Type>> {
            self.types_flat().await
        }

        /// @emoji 🏛️ Ensure a default typology exists when legacy flat snapshots omit `typologies`.
        pub async fn ensure_default_typology(self: &Arc<Self>) -> Arc<Typology> {
            {
                let tops = self.typologies.read().await;
                if let Some(t) = tops.first() {
                    return t.clone();
                }
            }
            let topo = Typology::new(Arc::downgrade(self), "Default".to_string()).await;
            self.typologies.write().await.push(topo.clone());
            topo
        }

        pub async fn typology_by_id(&self, id: &Id) -> Option<Arc<Typology>> {
            self.typologies.read().await.iter().find(|t| t.id == *id).cloned()
        }

        /// @emoji 🧬️ Deep-clone this kit graph (dev-backbone `initialKit` projection round-trip) for immutable graph `initialKit` baselines / operation replay.
        pub async fn deep_clone(self: &Arc<Self>) -> Arc<Kit> {
            let snap = crate::kit_backbone::initial_kit_projection_value(self).await;
            let owner = self.owner_graph.clone();
            let nm = self.name.read().await.clone();
            let entity = Kit::new_sync(owner, nm);
            let _ = crate::kit_backbone::hydrate_kit_from_initial_projection_value(&entity, &snap).await;
            entity
        }

        /// @emoji 📦️ Single mutation entry: walks canonical [`crate::operation::CanonicalKitDiff`] from [`crate::operation::Operation::to_diff`].
        pub async fn apply_diff(self: &Arc<Self>, diff: &crate::operation::KitDiff) -> Result<(), crate::error::ComposeError> {
            let d = &diff.0;
            if let Some(s) = &d.name {
                *self.name.write().await = s.clone();
            }
            if let Some(s) = &d.version {
                *self.version.write().await = Some(s.clone());
            }
            if let Some(s) = &d.description {
                *self.description.write().await = Some(s.clone());
            }
            if let Some(s) = &d.icon {
                *self.icon.write().await = Some(s.clone());
            }
            if let Some(s) = &d.image {
                *self.image.write().await = Some(s.clone());
            }
            if let Some(s) = &d.preview {
                *self.preview.write().await = Some(s.clone());
            }
            if let Some(s) = &d.remote {
                *self.remote.write().await = Some(s.clone());
            }
            if let Some(s) = &d.homepage {
                *self.homepage.write().await = Some(s.clone());
            }
            if let Some(s) = &d.license {
                *self.license.write().await = Some(s.clone());
            }
            if let Some(t) = &d.types {
                self.apply_types_collection_diff(t).await?;
            }
            if let Some(ds) = &d.designs {
                self.apply_designs_collection_diff(ds).await?;
            }
            if let Some(t) = &d.tags {
                self.apply_tags_collection_diff(t).await?;
            }
            if let Some(c) = &d.concepts {
                self.apply_concepts_collection_diff(c).await?;
            }
            if let Some(q) = &d.qualities {
                self.apply_qualities_collection_diff(q).await?;
            }
            if let Some(f) = &d.files {
                self.apply_files_collection_diff(f).await?;
            }
            if let Some(f) = &d.folders {
                self.apply_folders_collection_diff(f).await?;
            }
            if let Some(f) = &d.families {
                self.apply_families_collection_diff(f).await?;
            }
            if let Some(v) = &d.authors {
                if *v {
                    return Err(crate::error::ComposeError::invalid("kit diff `authors` subtree apply not implemented"));
                }
            }
            self.bump_touch_epoch().await;
            Ok(())
        }

        async fn resolve_typology_owner(self: &Arc<Self>, owner_id: &Id) -> Arc<Typology> {
            if let Some(topo) = self.typology_by_id(owner_id).await {
                return topo;
            }
            self.ensure_default_typology().await
        }

        async fn apply_types_collection_diff(self: &Arc<Self>, t: &crate::operation::TypesCollectionDiff) -> Result<(), crate::error::ComposeError> {
            for r in &t.removed {
                let id = r.id.clone();
                for topo in self.typologies.read().await.iter() {
                    topo.types.writ