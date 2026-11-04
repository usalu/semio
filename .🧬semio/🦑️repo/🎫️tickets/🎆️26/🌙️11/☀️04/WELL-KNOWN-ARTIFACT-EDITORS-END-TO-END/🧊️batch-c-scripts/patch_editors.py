import re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from wire_paint_commands import VARIANTS, ROOT

def sub1(s, pattern, repl, flags=re.S, what=""):
    new, n = re.subn(pattern, lambda m: repl(m) if callable(repl) else repl, s, count=1, flags=flags)
    assert n == 1, f"no match for {what or pattern[:60]}"
    return new

for v in VARIANTS:
    cfg = v["cfg"]
    path = ROOT + v["path"] + "🦀️.rs"
    s = open(path).read()
    if "mod paint_region;" in s:
        print("already patched", cfg["name"]); continue
    E, C = cfg["editor"], cfg["command"]
    prefix = E.replace("Editor", "")
    # A module declaration
    s = sub1(s, r"(use semio_s_artifact_stdio_contract::editing;\n)", lambda m: m.group(1) + '\n#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs"]\npub(crate) mod paint_region;\n', what="editing use")
    # B enum variant
    s = sub1(s, r"SetPixelRegion\s*\{[^}]*\},?", "PaintRegion(paint_region::PaintRegionCommand),", what="variant")
    # C tool ids
    s = sub1(s, r"(COMMAND_TOOL_IDS: &\[&str\] = &\[\n)", lambda m: m.group(1) + "    paint_region::ACTION_ID,\n", what="tool ids")
    # D command id
    s = sub1(s, r'_ => "other"', f'{C}::PaintRegion(_) => paint_region::ACTION_ID,\n        _ => "other"', what="command id")
    # E from action
    s = sub1(s, r"(\n(\s*)semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok\()", lambda m: f"\n{m.group(2)}paint_region::ACTION_ID => Ok({C}::PaintRegion(paint_region::PaintRegionCommand::from_action(args)?)),{m.group(1)}", what="from action")
    # F handle arm
    s = sub1(s, rf"\n(\s*){C}::SetPixelRegion \{{ \w+ \}} => (?:\{{.*?\n\1\}}|Ok\(Emit::mutations\(.*?\)\]\)\)),?\n", lambda m: f'\n{m.group(1)}{C}::PaintRegion(_) => Err(Fault::from("region painting requires the cancellable retained route")),\n', what="handle arm")
    # G proofs
    def proofs(m):
        body = m.group(0)
        owner_file = re.search(r'owner_file: ("[^"]*")', body).group(1)
        controller = re.search(r'controller: ("[^"]*")', body).group(1)
        schema = re.search(r'artifact_schema: ("[^"]*")', body).group(1)
        factory = re.search(r'factory: "([^"]*)"', body).group(1)
        contract = re.search(r'contract: (ToolExecutionContract::[^\n]*),\n', body).group(1)
        paint = f"{prefix}PaintRegionFactory"
        return f'''fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {{
        const OWNER_FILE: &str = {owner_file};
        const CONTROLLER: &str = {controller};
        let mut proofs = vec![
            ArtifactBoundedFirstStepProof::new::<EditorApp<{E}>>(OWNER_FILE, CONTROLLER, "{factory}", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, {schema}, {contract}).with_factory_type::<EditorApp<{E}>, {factory}>(),
            ArtifactBoundedFirstStepProof::new::<EditorApp<{E}>>(OWNER_FILE, CONTROLLER, "{paint}", paint_region::ACTION_ID, {schema}, {prefix.lower()}_editor_paint_region_execution_contract()).with_factory_type::<EditorApp<{E}>, {paint}>(),
        ];
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, {schema}));
        proofs
    }}
'''
    s = sub1(s, r"semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! \{.*?\n    \}\n", proofs, what="proofs")
    # H factory + contract
    paint = f"{prefix}PaintRegionFactory"
    fn = f"{prefix.lower()}_editor_paint_region_execution_contract"
    factory = f'''const STDIO_PAINT_REGION_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract {{ tool_id: paint_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }};
const fn {fn}() -> ToolExecutionContract {{
    ToolExecutionContract::bounded_first_step(paint_region::MAXIMUM_RAW_BYTES, 64, paint_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}}
struct {paint} {{ keys: Vec<ToolFactoryKey> }}
impl {paint} {{
    fn new(controller_id: &str) -> Self {{ Self {{ keys: paint_region::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }} }}
}}
impl ToolJobFactory for {paint} {{
    type Payload = ArtifactRetainedCommandPayload<EditorApp<{E}>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<{E}>>;
    fn keys(&self) -> &[ToolFactoryKey] {{ &self.keys }}
    fn payload_schema_id(&self) -> &str {{ paint_region::PAYLOAD_SCHEMA }}
    fn classification(&self) -> InteractiveJobClassification {{ InteractiveJobClassification::Migrated }}
    fn execution_contract(&self) -> ToolExecutionContract {{ {fn}() }}
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: paint_region::MAXIMUM_RAW_BYTES, checkpoint_resume: false);
}}
impl ArtifactOwnedToolJobFactory for {paint} {{
    type Owner = EditorApp<{E}>;
    const TOOL_IDS: &'static [&'static str] = paint_region::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DOCSCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_PAINT_REGION_CONTRACT];
}}
'''
    docschema = re.search(r"const DOCUMENT_SCHEMA: &'static str = (\w+);", s).group(1)
    factory = factory.replace("DOCSCHEMA", docschema)
    s = sub1(s, r"(//#region 🔖️Editor)", lambda m: factory + m.group(1), what="editor region")
    # I register
    s = sub1(s, r"(registry\.register\(\w+ExampleFactory::new\(registry\.controller_id\(\)\)\)\?;\n)", lambda m: m.group(1) + f"        registry.register({paint}::new(registry.controller_id()))?;\n", what="register")
    # J build tool job
    guard = f'''if original.tool_id == paint_region::ACTION_ID {{
            if {prefix.lower()}_editor_command_id(&original.command) != original.tool_id {{ return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-paint-region-tool-mismatch").into()); }}
            return semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, {prefix.lower()}_editor_command_id, paint_region::MAXIMUM_RAW_BYTES, paint_region::CAPACITY.work_items(), false, |request| paint_region::PaintRegionWork::new());
        }}
'''
    s = sub1(s, r"(if editing::is_snapshot_edit_action\(&original\.tool_id\) \{.*?\}\n)(\s*)", lambda m: m.group(1) + m.group(2) + guard + m.group(2), what="build tool job")
    # K definition
    s = sub1(s, r"\n    editing::snapshot_edit_actions_with\(builder\)\.build_definition\(\)", f'''
    let builder = builder.action_with(paint_region::action())
        .action_describe(paint_region::ACTION_ID, semio_framework_ui_locale::LocalizedLabel::native("Paints a checked solid color or palette index into a rectangular region and records exact undo history.", "Malt eine geprüfte Volltonfarbe oder einen Palettenindex in einen rechteckigen Bereich und speichert einen exakten Rückgängig-Verlauf."))
        .action_interactive_job(paint_region::ACTION_ID, InteractiveJobClassification::Migrated);
    editing::snapshot_edit_actions_with(builder).build_definition()''', what="definition")
    if "use semio_framework_plugin::ArtifactBoundedFirstStepProof;" not in s:
        s = sub1(s, r"(use semio_framework_plugin::ArtifactOwnedToolJobFactory;\n)", lambda m: m.group(1) + "use semio_framework_plugin::ArtifactBoundedFirstStepProof;\n", what="proof import")
    open(path, "w").write(s)
    print("patched", cfg["name"])
