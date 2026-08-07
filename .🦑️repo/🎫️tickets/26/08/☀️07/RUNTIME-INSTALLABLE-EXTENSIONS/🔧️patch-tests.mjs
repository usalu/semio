import fs from "fs";

const spacePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs";
let text = fs.readFileSync(spacePath, "utf8");

const helperOld = `    fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
        SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
    }`;

const helperNew = `    fn demo_user(id: &str, role: SpaceRole) -> SpaceUser {
        SpaceUser { id: id.into(), name: format!("User {id}"), avatar: None, role }
    }

    fn demo_extension(extension_id: &str, enabled: bool) -> InstalledExtension {
        InstalledExtension {
            extension_id: extension_id.into(),
            version: "1.0.0".into(),
            source_uri: format!("https://example.test/{extension_id}.sxt"),
            package_hash: format!("hash-{extension_id}"),
            enabled,
        }
    }`;

if (!text.includes(helperOld)) {
  // try to find demo_user
  const idx = text.indexOf("fn demo_user");
  console.log("demo_user at", idx, text.slice(idx, idx + 200));
  throw new Error("helperOld not found");
}
text = text.replace(helperOld, helperNew);

const opTextOld = `        store::test_support::assert_op_line_round_trip(&SpaceOperation::InstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceOperation::UninstallProgram { plugin_id: "cad".into() });
    }`;

const opTextNew = `        store::test_support::assert_op_line_round_trip(&SpaceOperation::InstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceOperation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_op_line_round_trip(&SpaceOperation::InstallExtension {
            extension_id: "flow-math".into(),
            version: "1.0.0".into(),
            source_uri: "https://example.test/flow-math.sxt".into(),
            package_hash: "hash-flow-math".into(),
            enabled: true,
        });
        store::test_support::assert_op_line_round_trip(&SpaceOperation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_op_line_round_trip(&SpaceOperation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
    }`;

if (!text.includes(opTextOld)) throw new Error("opTextOld not found");
text = text.replace(opTextOld, opTextNew);

const backwardsOld = `        store::test_support::assert_operation_round_trip(&base, SpaceOperation::InstallProgram { plugin_id: "cad".into() });
        let mut with_program = base.clone();
        with_program.programs.push("cad".into());
        store::test_support::assert_operation_round_trip(&with_program, SpaceOperation::UninstallProgram { plugin_id: "cad".into() });
    }`;

const backwardsNew = `        store::test_support::assert_operation_round_trip(&base, SpaceOperation::InstallProgram { plugin_id: "cad".into() });
        let mut with_program = base.clone();
        with_program.programs.push("cad".into());
        store::test_support::assert_operation_round_trip(&with_program, SpaceOperation::UninstallProgram { plugin_id: "cad".into() });
        store::test_support::assert_operation_round_trip(
            &base,
            SpaceOperation::InstallExtension {
                extension_id: "flow-math".into(),
                version: "1.0.0".into(),
                source_uri: "https://example.test/flow-math.sxt".into(),
                package_hash: "hash-flow-math".into(),
                enabled: true,
            },
        );
        let mut with_extension = base.clone();
        with_extension.extensions.push(demo_extension("flow-math", true));
        store::test_support::assert_operation_round_trip(&with_extension, SpaceOperation::UninstallExtension { extension_id: "flow-math".into() });
        store::test_support::assert_operation_round_trip(&with_extension, SpaceOperation::SetExtensionEnabled { extension_id: "flow-math".into(), enabled: false });
        store::test_support::assert_operation_round_trip(
            &with_extension,
            SpaceOperation::InstallExtension {
                extension_id: "flow-math".into(),
                version: "2.0.0".into(),
                source_uri: "https://example.test/flow-math-v2.sxt".into(),
                package_hash: "hash-flow-math-v2".into(),
                enabled: false,
            },
        );
    }`;

if (!text.includes(backwardsOld)) throw new Error("backwardsOld not found");
text = text.replace(backwardsOld, backwardsNew);

const diffOld = `            SpaceDiff { install_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { uninstall_program: Some("cad".into()), ..Default::default() },
            SpaceDiff::default(),
        ];`;

const diffNew = `            SpaceDiff { install_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { uninstall_program: Some("cad".into()), ..Default::default() },
            SpaceDiff { install_extension: Some(demo_extension("flow-math", true)), ..Default::default() },
            SpaceDiff { uninstall_extension_id: Some("flow-math".into()), ..Default::default() },
            SpaceDiff { set_extension_enabled_id: Some("flow-math".into()), set_extension_enabled: Some(false), ..Default::default() },
            SpaceDiff::default(),
        ];`;

if (!text.includes(diffOld)) throw new Error("diffOld not found");
text = text.replace(diffOld, diffNew);

fs.writeFileSync(spacePath, text);
console.log("patched tests");
