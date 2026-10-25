//! 🏭️ Exported command and tool job macros of the norm app surface: the action to typed command bridge and the owned tool job factory every norm app declares. See [`super`].


//#region 🔖️Commands
/// 🌉️ Installs the `{action, args}` → typed-`Command` bridge every norm editor needs, for the eight
/// verbs all fifteen declare (`evaluate`/`setSelectedCheckIndex`/`setField`/`insertItem`/`removeItem`/`applyRemedy`). A whole-document
/// replacement (`setSnapshot`) and an example load (`setActiveExample`) are not mutations: the shell's document load path and the
/// framework's catalogue route own them.
///
/// 🩹️ `ArtifactEditor::command_from_action`'s default refuses EVERY id — `app.command.unsupported:
/// action '…' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand)`
/// — so an editor that does not override it has an Actions rail whose every row is inert, however
/// green its `--lib` tests are. Exactly ONE of the fifteen norm editors (🧱️din4108) carried the
/// bridge, hand-written; the other fourteen were dispatch-dead in every shell. Measured inside the
/// real `s` host on a spawned `norm-din16798` (ticket 26/09/18, S7 §4 / S8 §4.3): `setSnapshot
/// refused: dispatch-failed (user window=norm-10::norm-din16798-inputs)`, which S8 attributed to the
/// HOST's routing. It is not the host: the refusal text is this trait default's own, raised by the
/// guest, and the fix is the bridge each editor owes.
///
/// `$command` is the editor's aggregated command enum; `$decode` is that artifact's own
/// `decode_<variant>_snapshot_json`. The three payload modules (`evaluate`, `selected_check`,
/// `set_snapshot`) resolve at the CALL site, which every norm editor already imports.
#[macro_export]
macro_rules! norm_command_from_action {
    ($command:ident, text, $decode:path) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command,
        /// for the two editors whose `setSnapshot` payload carries DSL TEXT rather than a decoded
        /// snapshot struct.
        fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework_plugin::Fault>) -> Result<$command, semio_framework_plugin::Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
            let args = args.as_ref();
            match action {
                "evaluate" => Ok($command::Evaluate(evaluate::Evaluate {})),
                "setSelectedCheckIndex" => Ok($command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: $crate::selected_check_index_arg(args) })),
                "setField" => Ok($command::SetField(set_field::SetField {
                    path: $crate::path_arg(args),
                    value_json: $crate::value_arg_json(args),
                })),
                "insertItem" => Ok($command::InsertItem(insert_item::InsertItem {
                    path: $crate::path_arg(args),
                    index: $crate::index_arg(args, "index") as u32,
                    value_json: {
                        let raw = $crate::value_arg_json(args);
                        if raw == "null" { None } else { Some(raw) }
                    },
                })),
                "removeItem" => Ok($command::RemoveItem(remove_item::RemoveItem {
                    path: $crate::path_arg(args),
                    index: $crate::index_arg(args, "index") as u32,
                })),
                "applyRemedy" => Ok($command::ApplyRemedy(apply_remedy::ApplyRemedy {
                    check_id: $crate::check_id_arg(args),
                    remedy_index: $crate::index_arg(args, "remedyIndex") as u32,
                })),
                other => Err(semio_framework_plugin::Fault::new(
                    semio_framework_plugin::FaultOrigin::App,
                    semio_framework_plugin::FaultCode::new("norm.unhandled-action"),
                    format!("action '{other}' is not one of this app's declared verbs (evaluate/setSelectedCheckIndex/setField/insertItem/removeItem/applyRemedy)"),
                )),
            }
        }
    };
    ($command:ident, $decode:path) => {
        /// 🌉️ Resolves the React/wgpu shells' `{action, args}` pair into this editor's typed command.
        /// `setSnapshot` carries the whole compliance document, so its argument is that document's own
        /// camelCase JSON projection (exactly what the Inputs window renders).
        fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<semio_framework_plugin::Fault>) -> Result<$command, semio_framework_plugin::Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
            let args = args.as_ref();
            match action {
                "evaluate" => Ok($command::Evaluate(evaluate::Evaluate {})),
                "setSelectedCheckIndex" => Ok($command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: $crate::selected_check_index_arg(args) })),
                "setField" => Ok($command::SetField(set_field::SetField {
                    path: $crate::path_arg(args),
                    value_json: $crate::value_arg_json(args),
                })),
                "insertItem" => Ok($command::InsertItem(insert_item::InsertItem {
                    path: $crate::path_arg(args),
                    index: $crate::index_arg(args, "index") as u32,
                    value_json: {
                        let raw = $crate::value_arg_json(args);
                        if raw == "null" { None } else { Some(raw) }
                    },
                })),
                "removeItem" => Ok($command::RemoveItem(remove_item::RemoveItem {
                    path: $crate::path_arg(args),
                    index: $crate::index_arg(args, "index") as u32,
                })),
                "applyRemedy" => Ok($command::ApplyRemedy(apply_remedy::ApplyRemedy {
                    check_id: $crate::check_id_arg(args),
                    remedy_index: $crate::index_arg(args, "remedyIndex") as u32,
                })),
                other => Err(semio_framework_plugin::Fault::new(
                    semio_framework_plugin::FaultOrigin::App,
                    semio_framework_plugin::FaultCode::new("norm.unhandled-action"),
                    format!("action '{other}' is not one of this app's declared verbs (evaluate/setSelectedCheckIndex/setField/insertItem/removeItem/applyRemedy)"),
                )),
            }
        }
    };
}
//#endregion 🔖️Commands

//#region 🔌️EditorOverrides
/// 🏭️ Declares one norm app's concrete owned factory as a newtype over the shared generic
/// [`NormBoundedCommandJobFactory`], plus its `register_tool_job_factories` entry point. The newtype is
/// required, not decorative: `ArtifactBoundedFirstStepProof` joins the `factory:` literal against the
/// last `::` segment of `std::any::type_name`, which for a generic instantiation is the owner app's own
/// name followed by `>`. Every behavioural line still lives once, in the generic base this delegates to.
#[macro_export]
macro_rules! norm_owned_tool_job_factory {
    ($factory:ident, $app:ty) => {
        pub struct $factory($crate::NormBoundedCommandJobFactory<$app>);

        impl $factory {
            pub fn new(controller_id: &str) -> Self {
                Self($crate::NormBoundedCommandJobFactory::<$app>::new(controller_id))
            }

            pub fn register(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<$app>>) -> Result<(), semio_framework_plugin::Fault> {
                let controller = registry.controller_id().to_string();
                registry.register(Self::new(&controller))
            }
        }

        impl semio_framework::ToolJobFactory for $factory {
            type Payload = <$crate::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Payload;
            type Job = <$crate::NormBoundedCommandJobFactory<$app> as semio_framework::ToolJobFactory>::Job;

            fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
                semio_framework::ToolJobFactory::keys(&self.0)
            }

            fn payload_schema_id(&self) -> &str {
                semio_framework::ToolJobFactory::payload_schema_id(&self.0)
            }

            fn classification(&self) -> semio_framework_plugin::InteractiveJobClassification {
                semio_framework::ToolJobFactory::classification(&self.0)
            }

            fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
                semio_framework::ToolJobFactory::execution_contract(&self.0)
            }

            fn admission_demands(&self, operation: &semio_framework_job::Operation, payload: &Self::Payload, input: Option<&semio_framework_action_bus::RetainedToolWireInput>, checkpoint: Option<&semio_framework_action_bus::RetainedToolWireInput>) -> Result<semio_framework_action_bus::ToolJobAdmissionDemand, semio_framework_value::ValueError> {
                semio_framework::ToolJobFactory::admission_demands(&self.0, operation, payload, input, checkpoint)
            }

            fn admit_job(&mut self, context: &mut semio_framework_job::StepContext<'_>, operation: &semio_framework_job::Operation, payload: &mut Option<Box<Self::Payload>>, input: &mut Option<semio_framework_action_bus::RetainedToolWireInput>, checkpoint: &mut Option<semio_framework_action_bus::RetainedToolWireInput>, output: &mut Option<Box<Self::Job>>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
                semio_framework::ToolJobFactory::admit_job(&mut self.0, context, operation, payload, input, checkpoint, output)
            }
        }

        impl semio_framework_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = semio_framework_plugin::EditorApp<$app>;
            const TOOL_IDS: &'static [&'static str] = $crate::NORM_RETAINED_TOOL_IDS;
            const DOCUMENT_SCHEMA: &'static str = <$app as semio_framework_plugin::ArtifactEditor>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = $crate::NORM_PUBLICATION_CONTRACTS;
        }
    };
}
//#endregion 🔌️EditorOverrides
