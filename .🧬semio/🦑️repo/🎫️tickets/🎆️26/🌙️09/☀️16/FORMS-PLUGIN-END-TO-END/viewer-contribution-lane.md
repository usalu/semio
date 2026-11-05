# Viewer Contribution Configuration Lane

The existing React host can install contributions into an active viewer. It does not filter by editor/viewer role. It requires that the active app declare a setContributions command and that the handle support handleCommand. At audit time, before the parent implementation, FormsViewer declared only Noop and NoConfig, so it was skipped.

## Host Ingress

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔁️contributions-refresh/🟦️.ts:119`: createContributionsPublisher cuts scopeContributionsJson from loaded topicContributions and consumedTopics.
- Same file line139: for the active plugin, targetApp is environment.session.app (therefore the active viewer app). Nonactive contributors select an app owning setContributions.
- Lines142–149: appOwnsCommand is the capability gate; appCommandTakesPageRun selects `{json,page:0,pageCount:1}` versus `{json}`; encodeAppCommandInvocation then handleCommand(instanceId,wire,targetViewState) is the actual crossing.
- Line181: consumedTopics comes from the receiver plugin registry consumes row, including catalogModule identity. It is not scoped by artifact document contents or app role.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts`: createContributionsPublisher owns pack generation and per-instance/content install coalescing; no React or role policy exists in that helper.

## Viewer SDK

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/👁️artifact-viewer-trait/🦀️.rs:97`: ArtifactViewer owns Config and ConfigMutation; viewers are not restricted to NoConfig.
- Same file lines245/254: config_schema defaults config.empty, and initial_config returns Config::default. Override these for a real domain configuration.
- Same file lines268–281: command_id, original_action_command_producer, command_from_action, and config_spec are supported viewer hooks. The default action decoder refuses unsupported commands.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/📖️viewer-app/🦀️.rs:183`: ViewerApp forwards Config/ConfigMutation. Lines197–226 forward configuration, initialization and command hooks. handle lifts only ViewEmit config_mutations/effects/ui_dirty into the runtime emission; artifact/draft mutations remain structurally absent.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📨️emission/🛬️lifted/🪟️surface/🦀️.rs`: ViewEmit::config(Vec<ConfigMutation>) is the supported configuration-only emission factory.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/🏗️surface-builders/🦀️.rs`: ViewerBuilder forwards config, command_grammar, command, app_command, command_args and view_action. It intentionally omits mutation.

## Forms Ownership

At audit time FormsConfig and SetContributions lived under the editor. The command handler emits only a configuration mutation and needs no artifact write. A viewer should use a common artifact-level configuration/payload owner, declare its own typed command/decode and configuration schema, and emit ViewEmit::config. Importing the sibling editor is prohibited by the viewer purity rule. With that ownership and a regenerated viewer command descriptor, the existing active-app host crossing suffices; no new host special case or viewer mutation API is needed.

This is a bounded source audit. No runtime viewer contribution installation has yet been observed in this session.

A subsequent read observes the parent viewer implementation now explicitly supplies config_apply_preparation_factory through build_config_store_one_item_preparation_factory, covering the SDK default None. Runtime validation remains pending.
