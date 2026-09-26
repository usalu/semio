#!/usr/bin/env python3
"""🧬️ H10 post-publish GUEST patch set (ticket 26/09/23 session 12): a codec call constructs only the apps that own its schema.

Measured (scratch probe `wp-h10/q1-interpreter`, B2 components, owned interpreter): `codec.pack-schema-hash` for a
schema NO app owns costs 829.4 M instructions on puzzle vs 830.2 M for `puzzle.3d`, and 34.5 M vs 35.3 M on note —
~99.9 % of every codec call is `plugin_artifact_codec_app` constructing (and closing) EVERY app of the bundle to read
each one's `artifact_schema()`, because the registry kept no static record of which schema an app opens.

Every registration site already holds the app's concrete type, so the patch records `X::DOCUMENT_SCHEMA` there:
  * `AppFactory<PA>` becomes a struct `{ definition, create, document_schema }` (was a tuple);
  * `SurfaceDeclaration` carries `document_schema` (stamped by `editor_surface` / `viewer_surface`);
  * `PluginBuilder::{document_app, viewer, editor}` record `A/V/E::DOCUMENT_SCHEMA`;
  * `Plugin::app_document_schema(app_id)` answers it; `artifact_codec_candidates(program, schema)` is the pure
    owner set (declared schema or dialect artifact kind) and `artifact_codec_owner` picks the editor, else the viewer,
    refusing two owners of one role — so `plugin_artifact_codec_app` constructs exactly ONE app per codec call
    (was: every app of the bundle, e.g. 6 for puzzle, 10 for wfc).
No wire, WIT, descriptor or manifest change: the recorded schema never leaves the guest. Guest code changes, so this
lands with the consolidated post-publish restage (rule 20 guest freeze until then).
Law (native, `🧪️tests/🔬️app-declarations-fixture`): every registered app's recorded schema IS its constructed
`artifact_schema()`, and each schema's candidates are exactly the apps owning it (2 of 6 in the fixture).

usage (every mode takes --root <repository copy> to patch a copy instead of the tree): codec-app-resolution.py            dry run: every hunk found exactly once
       codec-app-resolution.py --apply    write the tree (post-publish window only), then:
         nice -n 15 cargo test -p semio-framework-plugin --lib -- app_declarations  (+ wasm32-wasip2 check via the fleet mutex)
"""
import pathlib, sys

REPOSITORY = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
ROOT = REPOSITORY / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PLUGIN = ROOT / "🦀️.rs"
BUILDER = ROOT / "🏗️builder/🦀️.rs"
FIXTURE = ROOT / "🧪️tests/🔬️app-declarations-fixture/🦀️.rs"

HUNKS = {
    PLUGIN: [
        ("AppFactory type",
         """        pub(crate) type AppFactory<PA> = (AppDefinition, fn(&AppDefinition) -> PA);""",
         """        pub(crate) struct AppFactory<PA> {
            pub definition: AppDefinition,
            // 🚫️async: E4 fn-pointer slot
            pub create: fn(&AppDefinition) -> PA,
            pub document_schema: &'static str,
        }"""),
        ("SurfaceDeclaration field",
         """            pub app_schema: fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>,
            pub mutation_roster: Option<OwnerMutationRoster>,
            pub rights: Rights,
        }""",
         """            pub app_schema: fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>,
            pub document_schema: &'static str,
            pub mutation_roster: Option<OwnerMutationRoster>,
            pub rights: Rights,
        }"""),
        ("editor_surface literal",
         """            SurfaceDeclaration { definition: def, factory: factory::<E, PA>, app_schema: app_schema::<E>, mutation_roster: None, rights: Rights::Write }""",
         """            SurfaceDeclaration { definition: def, factory: factory::<E, PA>, app_schema: app_schema::<E>, document_schema: E::DOCUMENT_SCHEMA, mutation_roster: None, rights: Rights::Write }"""),
        ("viewer_surface literal",
         """            SurfaceDeclaration { definition: def, factory: factory::<V, PA>, app_schema: app_schema::<V>, mutation_roster: None, rights: Rights::Read }""",
         """            SurfaceDeclaration { definition: def, factory: factory::<V, PA>, app_schema: app_schema::<V>, document_schema: V::DOCUMENT_SCHEMA, mutation_roster: None, rights: Rights::Read }"""),
        ("declared projection push",
         """                            result.app_defs.push((App { definition: definition.clone(), examples }, (definition, surface.factory)));""",
         """                            result.app_defs.push((App { definition: definition.clone(), examples }, AppFactory { definition, create: surface.factory, document_schema: surface.document_schema }));"""),
        ("register_app_factory destructure",
         """            let (mut factory_definition, factory_create) = factory;
            join_framework_shared_action_dispositions(&mut app.definition);
            join_framework_shared_action_dispositions(&mut factory_definition);""",
         """            let mut factory = factory;
            join_framework_shared_action_dispositions(&mut app.definition);
            join_framework_shared_action_dispositions(&mut factory.definition);"""),
        ("register_app_factory insert",
         """            self.apps.insert(self.manifest.apps.last().unwrap().id.clone(), (factory_definition, factory_create));""",
         """            self.apps.insert(self.manifest.apps.last().unwrap().id.clone(), factory);"""),
        ("create_app + app_document_schema",
         """        pub fn create_app(&self, app_id: &str) -> Option<PA> {
            self.apps.get(app_id).map(|(definition, factory)| factory(definition))
        }""",
         """        pub fn create_app(&self, app_id: &str) -> Option<PA> {
            self.apps.get(app_id).map(|factory| (factory.create)(&factory.definition))
        }

        /// 🪪️ The document schema the registered app opens — its type's own `DOCUMENT_SCHEMA`, recorded
        /// at registration, so nothing has to be constructed to learn it.
        pub fn app_document_schema(&self, app_id: &str) -> Option<&'static str> {
            self.apps.get(app_id).map(|factory| factory.document_schema)
        }"""),
        ("codec app resolution",
         """        let mut editor = None;
        let mut viewer = None;
        let mut failure: Option<Fault> = None;
        for definition in &program.manifest.apps {
            if failure.is_some() {
                break;
            }
            let Some(app) = program.create_app(&definition.id) else { continue };
            // 🪪️ The document schema is the primary key — it is what `store::ArtifactCodec` and the
            // hub's trusted catalog are keyed by. The dialect's ARTIFACT KIND is admitted as a second
            // key because it is the only document identity a package's own manifest publishes: an app
            // definition carries `dialect.artifact_kind`, never `A::DOCUMENT_SCHEMA`. Build tooling
            // that has nothing but a compiled descriptor therefore asks by kind, reads the schema back
            // out of `codec.genesis`'s history, and asks everything after that by schema. A kind and a
            // schema never collide by construction: a kind is `s.<plugin>.<artifact>` and a schema is
            // the artifact's own `DOCUMENT_SCHEMA` spelling.
            let schema_matches = app.artifact_schema().await == artifact_schema;
            let owned = schema_matches || definition.dialect.artifact_kind == artifact_schema;
            let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };
            if owned && slot.is_none() {
                *slot = Some(app);
                continue;
            }
            if owned {
                failure = Some(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));
            }
            // 🪦️ Reading a candidate's schema costs a whole constructed app, and a constructed app
            // owns an `ArtifactStore` whose `Drop` asserts an exact terminal-empty shallow-shell
            // witness. Every app this resolution builds and does not return therefore leaves through
            // the same bounded close cursor the runtime's own instance-close job runs.
            if let Err(error) = close_artifact_codec_app(app) {
                failure.get_or_insert(error);
            }
        }
        let (selected, rejected) = match (editor, viewer) {
            (Some(editor), viewer) => (Some(editor), viewer),
            (None, viewer) => (viewer, None),
        };
        if let Some(rejected) = rejected {
            if let Err(error) = close_artifact_codec_app(rejected) {
                failure.get_or_insert(error);
            }
        }
        if let Some(failure) = failure {
            if let Some(selected) = selected {
                close_artifact_codec_app(selected)?;
            }
            return Err(failure);
        }
        selected.ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))
    }""",
         """        let definition = artifact_codec_owner(program, artifact_schema)?;
        let app = program.create_app(&definition.id).ok_or_else(|| plugin_internal_fault("artifact codec owner has no registered factory"))?;
        if app.artifact_schema().await == artifact_schema || definition.dialect.artifact_kind == artifact_schema {
            return Ok(app);
        }
        close_artifact_codec_app(app)?;
        Err(plugin_internal_fault("artifact codec owner does not open the schema its registration declares"))
    }"""),
        ("candidate function",
         """    /// 🧹️ Drains one THROWAWAY codec app to its exact terminal-empty shell before releasing it —""",
         """    /// 🪪️ The apps of `program` that own `artifact_schema`: those whose registered type opens that document
    /// schema — the primary key `store::ArtifactCodec` and the hub's trusted catalog use — and those whose
    /// dialect names it as an ARTIFACT KIND, the identity build tooling with nothing but a compiled
    /// descriptor asks by (it reads the schema back out of `codec.genesis` and asks by schema after that; a
    /// kind `s.<plugin>.<artifact>` and a `DOCUMENT_SCHEMA` never collide). Pure: nothing is constructed.
    pub(crate) fn artifact_codec_candidates<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> impl Iterator<Item = &'a crate::app::AppDefinition> + 'a {
        program.manifest.apps.iter().filter(move |definition| program.app_document_schema(&definition.id) == Some(artifact_schema) || definition.dialect.artifact_kind == artifact_schema)
    }

    /// 🎯️ The ONE app a codec call for `artifact_schema` constructs: the owning editor — only an editor's
    /// snapshot is the kind's creation authority — else the owning viewer. Two owners of the same role are
    /// refused rather than resolved by order, and no owner is refused; all of it decided on the declarations.
    pub(crate) fn artifact_codec_owner<'a, PA: PluginApp>(program: &'a Plugin<PA>, artifact_schema: &'a str) -> Result<&'a crate::app::AppDefinition, Fault> {
        let (mut editor, mut viewer) = (None, None);
        for definition in artifact_codec_candidates(program, artifact_schema) {
            let slot = if definition.role == semio_framework::AppRole::Editor { &mut editor } else { &mut viewer };
            if slot.replace(definition).is_some() {
                return Err(plugin_internal_fault("artifact codec schema resolves more than one app of the same role"));
            }
        }
        editor.or(viewer).ok_or_else(|| plugin_internal_fault("artifact codec schema is owned by no app of this bundle"))
    }

    /// 🧹️ Drains one THROWAWAY codec app to its exact terminal-empty shell before releasing it —"""),
        ("close cursor visibility",
         """    fn close_artifact_codec_app<PA: PluginApp>(mut app: PA) -> Result<(), Fault> {""",
         """    pub(crate) fn close_artifact_codec_app<PA: PluginApp>(mut app: PA) -> Result<(), Fault> {"""),
    ],
    BUILDER: [
        ("document_app push",
         """        self.app_defs.push((app, (definition, factory::<A, PA>)));""",
         """        self.app_defs.push((app, crate::app::declarations::AppFactory { definition, create: factory::<A, PA>, document_schema: A::DOCUMENT_SCHEMA }));"""),
        ("viewer push",
         """        self.app_defs.push((app, (def, factory::<V, PA>)));""",
         """        self.app_defs.push((app, crate::app::declarations::AppFactory { definition: def, create: factory::<V, PA>, document_schema: V::DOCUMENT_SCHEMA }));"""),
        ("editor push",
         """        self.app_defs.push((app, (def, factory::<E, PA>)));""",
         """        self.app_defs.push((app, crate::app::declarations::AppFactory { definition: def, create: factory::<E, PA>, document_schema: E::DOCUMENT_SCHEMA }));"""),
    ],
}

LAW_ANCHOR = """    #[semio_framework_async_macros::async_test]
    async fn a_conflicting_declaration_leaves_zero_rows_behind() {"""
LAW = """    /// 🪪️ A codec call constructs exactly one app, chosen on the declarations: every registered app's
    /// recorded document schema is exactly the one its constructed app opens, each schema's owners are
    /// exactly its own editor and viewer (two of the fixture's six apps), and the one constructed is the
    /// editor; a schema nobody owns is refused without constructing anything.
    #[semio_framework_async_macros::async_test]
    async fn codec_calls_construct_exactly_the_one_app_that_owns_their_schema() {
        let projected = project_artifact_declarations(&[build_declaration()]);
        let mut plugin = crate::app::Plugin::<FixtureApps>::new("testkit", "Testkit", "1.0.0");
        for (app, factory) in projected.app_defs {
            plugin = plugin.register_app_factory(app, factory);
        }
        let mut owners: Vec<(String, &'static str)> = Vec::new();
        for definition in &plugin.manifest.apps {
            let app = plugin.create_app(&definition.id).expect("registered app");
            let constructed = app.artifact_schema().await.to_string();
            assert_eq!(plugin.app_document_schema(&definition.id), Some(constructed.as_str()), "{}", definition.id);
            owners.push((definition.id.clone(), plugin.app_document_schema(&definition.id).expect("recorded schema")));
            crate::plugin_runtime::close_artifact_codec_app(app).expect("close throwaway app");
        }
        assert_eq!(owners.len(), 6);
        for schema in ["semio.testkit.w1c-fixture.std1-any/v1", "semio.testkit.w1c-fixture.std1-strict/v1", "semio.testkit.w1c-fixture.std2-any/v1"] {
            let candidates: Vec<&str> = crate::plugin_runtime::artifact_codec_candidates(&plugin, schema).map(|definition| definition.id.as_str()).collect();
            let expected: Vec<&str> = owners.iter().filter(|(_, owned)| *owned == schema).map(|(id, _)| id.as_str()).collect();
            assert_eq!(candidates, expected, "{schema}");
            assert_eq!(candidates.len(), 2, "{schema}: its editor and its viewer");
            let owner = crate::plugin_runtime::artifact_codec_owner(&plugin, schema).expect("one owner");
            assert_eq!(owner.role, AppRole::Editor, "{schema}: the editor is the creation authority");
            assert!(expected.contains(&owner.id.as_str()));
        }
        assert_eq!(crate::plugin_runtime::artifact_codec_candidates(&plugin, "semio.testkit.nobody/v1").count(), 0);
        assert!(crate::plugin_runtime::artifact_codec_owner(&plugin, "semio.testkit.nobody/v1").is_err(), "no owner is refused");
    }

"""

def patch():
    out = {}
    for path, hunks in HUNKS.items():
        text = path.read_text(encoding="utf-8")
        for label, old, new in hunks:
            count = text.count(old)
            if count != 1:
                raise SystemExit(f"{path.name}: hunk '{label}' found {count}× — re-derive it")
            text = text.replace(old, new)
        out[path] = text
    fixture = FIXTURE.read_text(encoding="utf-8")
    if fixture.count(LAW_ANCHOR) != 1:
        raise SystemExit("fixture law anchor not found exactly once")
    out[FIXTURE] = fixture.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)
    return out

patched = patch()
print(f"hunks OK: plugin {len(HUNKS[PLUGIN])}, builder {len(HUNKS[BUILDER])}, fixture law 1")
if "--apply" in sys.argv:
    for path, text in patched.items():
        path.write_text(text, encoding="utf-8")
    print("applied")
else:
    print("dry run: nothing written")
