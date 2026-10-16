import re
REPO = "/Users/ueli/Documents/semio/"
FW = "🧰️framework/"
M = FW + "🔨️modules/"
SRC = {
 "root": FW + "📦️packages/🦀️rust/🦀️.rs",
 "abi": M + "🌉️abi/🦀️.rs",
 "action_bus": M + "🎯️action-bus/🦀️.rs",
 "source_projection": M + "📁️filesystem/📷️snapshot/🦀️.rs",
 "platform": M + "🖥️platform/🦀️.rs",
 "interaction": M + "🕹️interaction/🦀️.rs",
 "interaction_gesture": M + "🕹️interaction/👆️gesture/🦀️.rs",
 "interaction_schema": M + "🕹️interaction/🧬️schema/🦀️.rs",
 "kernel": M + "🎠️kernel/🦀️.rs",
 "k_clipboard": M + "🎠️kernel/📋️clipboard/🦀️.rs",
 "k_content": M + "🎠️kernel/📤️return/📦️content/🦀️.rs",
 "k_message": M + "🎠️kernel/📤️return/📦️content/💌️message/🦀️.rs",
 "k_cold": M + "🎠️kernel/📥️cold-pair/🦀️.rs",
 "manifest": M + "🛂️manifest/🦀️.rs",
 "m_canonical": M + "🛂️manifest/🌲️canonical/🦀️.rs",
 "m_media": M + "🛂️manifest/🎞️media/🦀️.rs",
 "m_space": M + "🛂️manifest/📇️space-index/🦀️.rs",
 "m_utilinit": M + "🛂️manifest/🪛️utilities/🌅️initial/🦀️.rs",
 "m_argschema": M + "🛂️manifest/🧮️arg-schema/🦀️.rs",
 "m_actions": M + "🛂️manifest/🕹️actions/🦀️.rs",
 "onboarding": M + "🎓️onboarding/🦀️.rs",
 "ta_actions": M + "⏪️time-travel/🕹️actions/🦀️.rs",
 "tr_actions": M + "⏯️tool-run/🕹️actions/🦀️.rs",
 "m_surface": M + "🛂️manifest/🧱️app-surface/🦀️.rs",
 "m_plugin": M + "🛂️manifest/🧾️plugin-manifest/🦀️.rs",
 "m_view": M + "🛂️manifest/🖼️view-model/🦀️.rs",
}
FAMILY = lambda f: 'kernel' if (f == 'kernel' or f.startswith('k_')) else ('manifest' if (f == 'manifest' or f.startswith('m_')) else f.split('_')[0] if f.startswith('interaction') else f)
# cluster -> dict(pkg, node, root (relative to REPO), emoji-doc, files order)
def C(id_, node, root=None, emoji='', doc='', pkg=None):
    return dict(id=id_, pkg=pkg or 'semio-framework-' + id_, node=node, root=root or node + '/🦀️.rs', emoji=emoji, doc=doc)
K = M + "🎠️kernel/"
MF = M + "🛂️manifest/"
CLUSTERS = {c['id']: c for c in [
 C('kernel-identity', K + "🪪️identity", emoji='🪪️', doc="Kernel identifiers, rights, capabilities, invocation records, broker quotas and activation events: the vocabulary every kernel contract names."),
 C('kernel-history', K + "📜️history", emoji='📜️', doc="Kernel history wire, history and fault notices, and the manifest fault notice definitions they are localized against."),
 C('kernel-ui', K + "🖼️ui", emoji='🖼️', doc="Kernel UI delta: dirty scopes and the per-turn UI patch owners with their transports."),
 C('kernel-media-jobs', K + "🎞️media-jobs", emoji='🎞️', doc="Kernel media and job programs: spawned job drive, media export encoding, video render program and ledger, file import staging."),
 C('kernel', K[:-1], root=K + "🦀️.rs", emoji='🎠️', doc="Kernel effects, events, turn results, invocation contexts, broker hooks and the return content wire."),
 C('abi', M + "🌉️abi", emoji='🌉️', doc="Language-neutral component ABI: handles, pages, messages, ledgers and ports."),
 C('action-bus', M + "🎯️action-bus", emoji='🎯️', doc="Action bus and tool job registry, plus the root platform shell that owns one bus."),
 C('source-projection', M + "📁️filesystem/📷️snapshot", emoji='📷️', doc="Source projection snapshot over a path-keyed text tree."),
 C('interaction', M + "🕹️interaction", emoji='🕹️', doc="Hover and selection interaction declarations, gesture recognizer and schema exports."),
 C('manifest-address', MF + "🧭️address", emoji='🧭️', doc="Manifest addressing: action, tool and utility references, command addresses, keybindings and element ids."),
 C('manifest-media', MF + "🎞️media", emoji='🎚️', doc="Typed media vocabulary: media types, ports, compatibility, converters and artifact kind specs."),
 C('manifest-view-model', MF + "🖼️view-model", emoji='🪟️', doc="View model handed to guests: view context capacities, public invocation capacity and UI refresh sections."),
 C('manifest-contribution', MF + "🧩️contribution", emoji='🧩️', doc="Plugin contributions: topic rows, versions and pins, inference and mutation metadata, space index and canonical trees."),
 C('manifest-arg-schema', MF + "🧮️arg-schema", emoji='🧮️', doc="Action argument schema: formats, schemas, controls and argument definitions."),
 C('manifest-mutation-inputs', MF + "🔡️mutation-inputs", emoji='🔡️', doc="Mutation input schema reader: input definitions, audits and instances derived from JSON schema documents."),
 C('manifest-actions', MF + "🕹️actions", emoji='⚡️', doc="Action semantics, action and command definitions, framework action families and utilities."),
 C('manifest-app-surface', MF + "🧱️app-surface", emoji='🧱️', doc="App surface: dialogs, modes, window kinds, panels, fault notice wiring and the app definition."),
 C('manifest-plugin', MF + "🧾️plugin-manifest", emoji='🧾️', doc="Plugin manifest, dependency graph, artifact and surface app choices and package descriptors."),
 C('onboarding', M + "🎓️onboarding", emoji='🎓️', doc="App onboarding: the step-gated first-run introduction, the recorded tutorial timeline with its playback engine and the reserved actions that start or record them."),
 C('time-travel-actions', M + "⏪️time-travel/🕹️actions", emoji='⏪️', doc="The reserved history-edit verbs of the time-travel session and their finalize dialog, contributed to every hosting app."),
 C('tool-run-actions', M + "⏯️tool-run/🕹️actions", emoji='⏯️', doc="The framework surface of the tool run feature: the run vocabulary a tool or utility declares and the reserved tool run actions."),
]}
