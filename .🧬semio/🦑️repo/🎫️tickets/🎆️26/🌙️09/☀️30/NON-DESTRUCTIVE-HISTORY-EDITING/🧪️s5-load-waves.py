#!/usr/bin/env python3
"""🧪️ S5-LOAD landing waves (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, rule 51).

Each wave is a list of whole-file writes prepared in memory from unique anchors, so a wave is either fully applicable or
not applied at all. `check` verifies every anchor against the live tree and writes nothing; `land` keeps a copy of every
file it replaces under `🗑️generated/s5-load/pre-<wave>/` and then writes each file once; `restore` puts those copies back
and removes the files the wave created.

    python3 🧪️s5-load-waves.py <p1|f1|f12|persist|attach|f16|f6|merge-guest|checkin|merge-host|merge-shell|merge-law|receipt|golden|f9|attach-told|f9-test|f13|detach> <check|land|restore>

Waves: `p1` = the folder reload route law (new files + its two registrations), `f1` = audit F1 caller half (`🏃️run` and
the MCP gateway keep a guest's structured admission fault), `f12` = audit F12 (the composed `artifact:out` export carrier
and its law are deleted; the one carrier is the recursive archive `produce_media` answers).
"""
import pathlib
import shutil
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
STAGE = TICKET / "🗑️generated" / "s5-load"
PLUGIN = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
MODULES = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules"


def replace_once(text: str, old: str, new: str, what: str) -> str:
    """🔁️ Replaces the one occurrence of `old`; an edit already applied (the new text present, the old absent) is kept."""
    if new and new in text:
        return text
    count = text.count(old)
    if count == 0 and not new:
        return text
    if count != 1:
        raise SystemExit(f"{what}: expected exactly one anchor, found {count}")
    return text.replace(old, new)


def cut_between(text: str, start: str, end: str, what: str) -> str:
    """✂️ Removes the one span from `start` up to (not including) `end`; a span already gone is kept."""
    if start not in text:
        return text
    if text.count(start) != 1:
        raise SystemExit(f"{what}: start anchor is not unique")
    begin = text.index(start)
    finish = text.index(end, begin + len(start))
    return text[:begin] + text[finish:]


def wave_p1() -> tuple[dict, list]:
    """📁️ The folder reload route law, its fixture, schema, twin and registrations."""
    writes, created = {}, []
    for relative in ["🧪️tests/🧪️folder-reload-route/🦀️.rs", "🧪️tests/🧪️folder-reload-route/🟦️.ts", "🧫️fixtures/🧫️folder-reload-route/🔣️.json", "🧫️fixtures/🧫️folder-reload-route/🧬️schema/🔣️.json"]:
        target = PLUGIN / relative
        writes[target] = (STAGE / "wave-p1" / relative).read_text()
        created.append(target)
    laws = PLUGIN / "🧪️tests/🧪️time-travel/🦀️.rs"
    text = laws.read_text()
    registration = '\n//#region 📁️FolderReloadRoute\n#[path = "../🧪️folder-reload-route/🦀️.rs"]\nmod folder_reload_route;\n//#endregion 📁️FolderReloadRoute\n'
    if "mod folder_reload_route;" not in text:
        if not text.endswith("\n"):
            raise SystemExit("time-travel laws: the file does not end with a newline")
        text += registration
    writes[laws] = text
    script = PLUGIN / "📦️packages/🦀️rust/📜️script.ts"
    text = script.read_text()
    text = replace_once(
        text,
        'import { historyLabelReloadOracle } from "../../🧪️tests/🧪️history-label-reload/🟦️.ts";\n',
        'import { historyLabelReloadOracle } from "../../🧪️tests/🧪️history-label-reload/🟦️.ts";\nimport { folderReloadRouteOracle } from "../../🧪️tests/🧪️folder-reload-route/🟦️.ts";\n',
        "plugin script: oracle import",
    )
    text = replace_once(
        text,
        "    console.log(`history-label-reload-oracle cases=${historyLabelReloadOracle(this.repoRoot)}`);\n",
        "    console.log(`history-label-reload-oracle cases=${historyLabelReloadOracle(this.repoRoot)}`);\n    console.log(`folder-reload-route-oracle steps=${folderReloadRouteOracle(this.repoRoot)}`);\n",
        "plugin script: oracle row",
    )
    writes[script] = text
    return writes, created


def wave_f1() -> tuple[dict, list]:
    """🤝️ `🏃️run` and the MCP gateway keep the guest's structured admission fault instead of its flattened reason."""
    writes = {}
    run = MODULES / "🏃️run/🦀️.rs"
    text = run.read_text()
    text = replace_once(
        text,
        "    Cycle(Vec<String>),\n    Host(String),\n",
        "    Cycle(Vec<String>),\n    Host(String),\n    /// 🤝️ The guest itself was refused at admission (`ActivationRefusal.fault`, e.g. `plugin.channel-mismatch` with its\n    /// `guest`/`host` params): the structured fault, so the runner's caller tells its localized notice.\n    Refused(semio_framework::Fault),\n",
        "run: RunError variant",
    )
    text = replace_once(
        text,
        '            Self::Host(message) => write!(formatter, "host error: {message}"),\n',
        '            Self::Host(message) => write!(formatter, "host error: {message}"),\n            Self::Refused(fault) => write!(formatter, "{}: {}", fault.code.0, fault.message),\n',
        "run: RunError display",
    )
    text = replace_once(
        text,
        ".map_err(|refusal| RunError::Host(refusal.reason))?;",
        ".map_err(activation_refusal_error)?;",
        "run: activation refusal mapping",
    )
    text = replace_once(
        text,
        "impl From<MediaError> for RunError {\n",
        "/// 🚪️ An activation that installed no actor, as the runner reports it: the guest's own admission fault when it carries\n/// one (audit F1), else the host's reason in words.\npub fn activation_refusal_error(refusal: semio_framework_plugin_host::activation::ActivationRefusal) -> RunError {\n    match refusal.fault {\n        Some(fault) => RunError::Refused(fault),\n        None => RunError::Host(refusal.reason),\n    }\n}\n\nimpl From<MediaError> for RunError {\n",
        "run: activation refusal helper",
    )
    writes[run] = text
    laws = MODULES / "🏃️run/🧪️tests/🔬️unit/🦀️.rs"
    text = laws.read_text()
    if "an_admission_refusal_reaches_the_runner_as_the_guests_structured_fault" not in text:
        if not text.endswith("\n"):
            raise SystemExit("run laws: the file does not end with a newline")
        text += (
            "\n//#region 🤝️AdmissionRefusal\n"
            "/// 🤝️ LAW (audit F1, caller half): an activation the guest itself was refused at reaches the runner's caller as that\n"
            "/// structured fault — its code and `guest`/`host` params intact for the localized notice — while a refusal the host\n"
            "/// raised stays its reason in words.\n"
            "#[test]\n"
            "fn an_admission_refusal_reaches_the_runner_as_the_guests_structured_fault() {\n"
            "    use semio_framework_plugin_host::activation::ActivationRefusal;\n"
            "    let fault = protocol::admit_guest_channel_version(20, 21).expect_err(\"a guest of another channel version is refused\");\n"
            "    let refused = activation_refusal_error(ActivationRefusal::instantiation(semio_framework_plugin_host::PluginHostError::Refused(Box::new(fault.clone()))));\n"
            "    assert!(matches!(&refused, RunError::Refused(kept) if *kept == fault), \"the guest's fault is kept whole: {refused:?}\");\n"
            "    assert_eq!(refused.to_string(), format!(\"{}: {}\", protocol::CHANNEL_MISMATCH_CODE, fault.message));\n"
            "    let host = activation_refusal_error(ActivationRefusal::host(\"Kernel activation refused: Saturated\"));\n"
            "    assert!(matches!(&host, RunError::Host(reason) if reason == \"Kernel activation refused: Saturated\"), \"a host refusal stays words: {host:?}\");\n"
            "}\n"
            "//#endregion 🤝️AdmissionRefusal\n"
        )
    writes[laws] = text
    workspace = MODULES / "🌉️mcp/🏠️workspace/🦀️.rs"
    text = workspace.read_text()
    text = replace_once(
        text,
        '        let mut guest = semio_framework_async::block_on(self.runtime.instantiate(&self.compiled, actor, &caps, &budget)).map_err(|error| Self::not_wired("instantiate", error))?;\n',
        '        let mut guest = semio_framework_async::block_on(self.runtime.instantiate(&self.compiled, actor, &caps, &budget)).map_err(|error| match error {\n            semio_framework_plugin_host::PluginHostError::Refused(fault) => Fault { code: fault.code.0.clone(), message: fault.message.clone() },\n            other => Self::not_wired("instantiate", other),\n        })?;\n',
        "mcp: instantiate refusal",
    )
    writes[workspace] = text
    return writes, []


def wave_f12() -> tuple[dict, list]:
    """🪆️ Deletes the composed `artifact:out` export carrier, its override and its law."""
    writes = {}
    runtime = PLUGIN / "🦀️.rs"
    text = runtime.read_text()
    text = cut_between(
        text,
        "        /// 🪆️ The native `artifact:out` carrier of a composed document (design §20.15): `encode_document_archive_bytes` of the parent's\n",
        "    }\n\n    /// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.",
        "plugin: composed_artifact_media",
    )
    text = replace_once(
        text,
        "            Ok(members)\n        }\n\n    }\n\n    /// 🆔️ Deterministic session-local `ArtifactHandle`",
        "            Ok(members)\n        }\n    }\n\n    /// 🆔️ Deterministic session-local `ArtifactHandle`",
        "plugin: blank line left by the cut",
    )
    text = replace_once(
        text,
        "        /// with dense ordinals — the member roster both the document archive and the composed `artifact:out` carrier hold.\n",
        "        /// with dense ordinals — the member roster of the document archive, which is also the one `artifact:out` carrier.\n",
        "plugin: archive_member_entries doc",
    )
    text = replace_once(
        text,
        '            if port == "artifact:out" && !self.children.is_empty() {\n                return self.composed_artifact_media().await.map_err(|fault| MediaError::Payload(port.to_string(), fault.message));\n            }\n',
        "",
        "plugin: export_media override",
    )
    if "composed_artifact_media" in text:
        raise SystemExit("plugin: composed_artifact_media is still referenced")
    writes[runtime] = text
    laws = PLUGIN / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
    text = laws.read_text()
    text = cut_between(
        text,
        "    /// 🪆️ LAW (design §20.15, W-b): the framework's `artifact:out` export of a composed document is the composed carrier",
        "    /// 🎞️ LAW (W2A-6 P5, `📓️api-stepped-document-load.md` §4/§9): consuming a whole document",
        "laws: composed carrier law",
    )
    if "the_artifact_out_export_of_a_composed_document" in text:
        raise SystemExit("laws: the composed carrier law is still present")
    writes[laws] = text
    return writes, []


def wave_persist() -> tuple[dict, list]:
    """📥️ The folder archive persistence policy (design §22.22): the class beside `restoreDocumentArchiveV1`, its corpus and law.
    Served TypeScript: take the `serve` lock. The ShellHost hunks that use it land with the merge route, not here."""
    writes, created = {}, []
    renderer = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document"
    stage = STAGE / "wave-persist"
    module = renderer / "🟦️.ts"
    text = module.read_text()
    policy = (stage / "policy.ts").read_text()
    head, body = policy.split("\n\n", 1)
    if "export class FolderArchivePersistenceV1" not in text:
        if text.startswith("import "):
            raise SystemExit("document admission: the module gained imports, merge the policy import by hand")
        text = head + "\n\n" + text
        text = replace_once(text, "/** 🧊️ Retains one active and one latest cold pair; superseded work cannot publish a binding. */\n", body.rstrip("\n") + "\n\n/** 🧊️ Retains one active and one latest cold pair; superseded work cannot publish a binding. */\n", "document admission: policy anchor")
    writes[module] = text
    for relative in ["🔣️.json", "🧬️schema/🔣️.json"]:
        target = renderer / "🧫️fixtures/🧫️folder-archive-persistence" / relative
        writes[target] = (stage / "🧫️folder-archive-persistence" / relative).read_text()
        created.append(target)
    law = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-archive-persistence/🟦️.ts"
    writes[law] = (stage / "law.ts").read_text().replace("POLICY_MODULE", "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts").replace("FIXTURE_DIRECTORY", "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🧫️folder-archive-persistence/")
    created.append(law)
    return writes, created


def wave_attach() -> tuple[dict, list]:
    """🔁️ Live fault F4, host half (served TypeScript: `serve` lock). (1) A control turn of the document port may carry the
    program's own frames beside its receipt: `splitDocumentBackboneControlTurnV1` in the binding module, used by the React
    plugin runtime and the wgpu plugin bridge, which route those frames like any other turn's. (2) A rejected archive load
    keeps the document attached and bound: `replaceAttachedDocumentV1` beside `restoreDocumentArchiveV1`, used by ShellHost
    `loadDocumentArchive`. (3) A read-back that cannot be restored is a console error with its reason. Both laws join the os
    `test-channel-oracles` command."""
    writes, created = {}, []
    renderer = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost"
    stage = STAGE / "wave-attach"
    module = renderer / "🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts"
    text = module.read_text()
    if "export async function replaceAttachedDocumentV1" not in text:
        text = replace_once(text, "/** 🗃️ What restoring one read-back document archive drives:", (stage / "helper.ts").read_text().rstrip("\n") + "\n\n/** 🗃️ What restoring one read-back document archive drives:", "document admission: helper anchor")
    writes[module] = text
    shell = renderer / "🟦️.tsx"
    text = shell.read_text()
    text = replace_once(
        text,
        "    return entry.replacements.replace(archive, async (candidate, latest) => {\n"
        "      const exact = () => latest() && current() && openDocumentSessionsRef.current.get(runtimeKey) === entry;\n"
        "      await lane.replace(entry.clientInstanceId, exact, async () => {\n"
        "        await retirement;\n"
        "        if (!exact()) return;\n"
        "        try {\n"
        "          await load(instanceId, candidate, task?.signal, task?.progress);\n"
        "        } finally {\n"
        "          if (exact()) await bindDocumentBackbone(runtimeKey, entry, latest);\n"
        "        }\n"
        "      });\n"
        "    });\n",
        "    return entry.replacements.replace(archive, (candidate, latest) => {\n"
        "      const exact = () => latest() && current() && openDocumentSessionsRef.current.get(runtimeKey) === entry;\n"
        "      return replaceAttachedDocumentV1(lane, entry.clientInstanceId, exact, { retired: retirement, load: () => load(instanceId, candidate, task?.signal, task?.progress), bind: () => bindDocumentBackbone(runtimeKey, entry, latest) });\n"
        "    });\n",
        "ShellHost: loadDocumentArchive replacement",
    )
    text = replace_once(text, "parkDocumentOpeningReplacementV1, restoreDocumentArchiveV1, runDocumentOpeningAttemptV1,", "parkDocumentOpeningReplacementV1, replaceAttachedDocumentV1, restoreDocumentArchiveV1, runDocumentOpeningAttemptV1,", "ShellHost: helper import")
    text = replace_once(
        text,
        "          } catch (replacementError) {\n            if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== message.clientInstanceId) return;\n            if (entry.creationMount !== null) {\n",
        "          } catch (replacementError) {\n            if (openDocumentSessionsRef.current.get(runtimeKey)?.clientInstanceId !== message.clientInstanceId) return;\n            console.error(\"[os-shell] a folder read-back could not be restored\", message.documentId, replacementError instanceof Error ? replacementError.message : String(replacementError));\n            if (entry.creationMount !== null) {\n",
        "ShellHost: loud read-back failure",
    )
    writes[shell] = text
    binding = MODULES / "🔌️plugin/📡️backbone/🔗️binding/🟦️.ts"
    text = binding.read_text()
    if "export function splitDocumentBackboneControlTurnV1" not in text:
        text = replace_once(text, "/** 🧾️ Accepts an exact successful receipt and surfaces a verified refusal code. */\n", (stage / "binding-helper.ts").read_text().rstrip("\n") + "\n\n/** 🧾️ Accepts an exact successful receipt and surfaces a verified refusal code. */\n", "binding: helper anchor")
    writes[binding] = text
    runtime = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"
    text = runtime.read_text()
    text = replace_once(
        text,
        'import { ActorDocumentBindingV1, type ActorDocumentMessagePortV1, type ActorDocumentSourceV1, encodeDocumentBackboneControlV1 } from "../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";',
        'import { ActorDocumentBindingV1, type ActorDocumentMessagePortV1, type ActorDocumentSourceV1, encodeDocumentBackboneControlV1, splitDocumentBackboneControlTurnV1 } from "../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";',
        "PluginRuntime: import",
    )
    text = replace_once(
        text,
        "        return result.effects.flatMap(effect => {\n"
        "          const bytes = shellFrameBytes(effect, instanceId);\n"
        "          if (bytes) return [bytes];\n"
        "          if (effect.tag === \"send-message\" && (effect.val as { target?: WireVariant } | undefined)?.target?.tag === \"backbone\") throw new Error(\"actor-document-control.data-before-receipt\");\n"
        "          return [];\n"
        "        });\n",
        "        const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(result.effects.flatMap(effect => {\n"
        "          const bytes = shellFrameBytes(effect, instanceId);\n"
        "          if (bytes) return [bytes];\n"
        "          if (effect.tag === \"send-message\" && (effect.val as { target?: WireVariant } | undefined)?.target?.tag === \"backbone\") throw new Error(\"actor-document-control.data-before-receipt\");\n"
        "          return [];\n"
        "        }));\n"
        "        if (unsolicited.length > 0) turnOutcomes.push({ instanceId, frames: [...unsolicited] });\n"
        "        return receipts;\n",
        "PluginRuntime: control exchange",
    )
    writes[runtime] = text
    bridge = MODULES / "📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts"
    text = bridge.read_text()
    text = replace_once(
        text,
        'import { DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, encodeDocumentBackboneControlV1, requireDocumentBackboneReceiptV1 } from "../../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";',
        'import { DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, encodeDocumentBackboneControlV1, requireDocumentBackboneReceiptV1, splitDocumentBackboneControlTurnV1 } from "../../../../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts";',
        "wgpu bridge: import",
    )
    text = replace_once(
        text,
        "    if (settled.frames.length !== 1) throw new Error(`actor-document-control.receipt-count:${settled.frames.length}`);\n"
        "    requireDocumentBackboneReceiptV1(settled.frames[0]!, command);\n",
        "    const { receipts, unsolicited } = splitDocumentBackboneControlTurnV1(settled.frames);\n"
        "    if (receipts.length !== 1) throw new Error(`actor-document-control.receipt-count:${receipts.length}`);\n"
        "    requireDocumentBackboneReceiptV1(receipts[0]!, command);\n"
        "    if (unsolicited.length > 0) turnOutcomes.push({ instanceId, frames: [...unsolicited] });\n",
        "wgpu bridge: control exchange",
    )
    writes[bridge] = text
    tests = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests"
    law = tests / "🧪️attached-document-replacement/🟦️.ts"
    writes[law] = (stage / "law.ts").read_text().replace("DOCUMENT_MODULE", "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts")
    created.append(law)
    law = tests / "🧪️document-port-control-turn/🟦️.ts"
    writes[law] = (stage / "binding-law.ts").read_text().replace("BINDING_MODULE", "../../🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts")
    created.append(law)
    package = REPO / "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript"
    script = package / "📜️script.ts"
    text = script.read_text()
    text = replace_once(
        text,
        "/** 🗃️ Runs the archive-load host twin against the channel's language-neutral corpus under `bun:test`. */\n",
        "/** 🗃️ Runs the archive-load host twin against the channel's language-neutral corpus, the attached-document replacement law and the document-port control turn law under `bun:test`. */\n",
        "os script: oracle docstring",
    )
    text = replace_once(
        text,
        '    await runOwnedCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts")], this.repoRoot, "os-channel-oracles", TEST_LEVEL_BUDGET_MS.fundamental);\n',
        '    const oracles = ["🧪️document-archive-load-host", "🧪️attached-document-replacement", "🧪️document-port-control-turn"].map((oracle) => join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests", oracle, "🟦️.ts"));\n'
        '    await runOwnedCommand(process.execPath, ["test", ...oracles], this.repoRoot, "os-channel-oracles", TEST_LEVEL_BUDGET_MS.fundamental);\n',
        "os script: oracle list",
    )
    writes[script] = text
    project = package / "📋️project.json"
    text = project.read_text()
    text = replace_once(
        text,
        '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/**/*",\n',
        '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/**/*",\n        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️attached-document-replacement/**/*",\n        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-port-control-turn/**/*",\n',
        "os project: oracle inputs",
    )
    writes[project] = text
    return writes, created


def wave_f16() -> tuple[dict, list]:
    """🪧️ Audit F16: every refusal of the whole-document load and save surface carries its `plugin.document-load.*` code (the
    notices are in the kernel table since 04:59; this wave adds the one row `plugin.document-load.members-differ` to the Rust
    table, its TS twin and the fixture). `landing` + `serve`: one helper and the codes beside
    `MEDIA_SCHEMA_MISMATCH_CODE`, 71 sites of the load surface selected by their own message text, and the law."""
    writes = {}
    runtime = PLUGIN / "🦀️.rs"
    text = runtime.read_text()
    if "pub fn document_load_fault(" in text:
        writes[runtime] = text
    else:
        helper = (
            "    /// 🪧️ The codes of the whole-document load and save surface; each has its framework notice in every locale (kernel table\n"
            "    /// `FRAMEWORK_FAULT_NOTICE_LABELS`).\n"
            '    pub const DOCUMENT_LOAD_UNAVAILABLE_CODE: &str = "plugin.document-load.unavailable";\n'
            '    pub const DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE: &str = "plugin.document-load.too-many-members";\n'
            '    pub const DOCUMENT_LOAD_TOO_LARGE_CODE: &str = "plugin.document-load.too-large";\n'
            '    pub const DOCUMENT_LOAD_BUSY_CODE: &str = "plugin.document-load.busy";\n'
            '    pub const DOCUMENT_LOAD_INCOMPLETE_CODE: &str = "plugin.document-load.incomplete";\n'
            '    pub const DOCUMENT_LOAD_MEMBER_INVALID_CODE: &str = "plugin.document-load.member-invalid";\n'
            '    pub const DOCUMENT_LOAD_HISTORY_INVALID_CODE: &str = "plugin.document-load.history-invalid";\n'
            '    pub const DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE: &str = "plugin.document-load.operation-unknown";\n'
            '    pub const DOCUMENT_LOAD_CHANGED_CODE: &str = "plugin.document-load.changed";\n'
            '    pub const DOCUMENT_LOAD_FAILED_CODE: &str = "plugin.document-load.failed";\n'
            "\n"
            "    /// 🚫️ A named refusal of the whole-document load and save surface: `code` is one of the `DOCUMENT_LOAD_*_CODE`s, the message\n"
            "    /// keeps the engineering detail a log reads.\n"
            "    pub fn document_load_fault(code: &'static str, message: impl Into<String>) -> Fault {\n"
            "        Fault::new(FaultOrigin::Framework, FaultCode::new(code), message)\n"
            "    }\n"
            "\n"
        )
        text = replace_once(text, "    /// 🎞️ A whole document on a media edge (`artifact:out`): the recursive document archive (parent pack + `.spr` + owned members) as\n", helper + "    /// 🎞️ A whole document on a media edge (`artifact:out`): the recursive document archive (parent pack + `.spr` + owned members) as\n", "plugin: helper anchor")
        before = text.count("plugin_sdk_fault(")
        exact = [
            ('plugin_sdk_fault("recursive document archive loading is unavailable for this app")', 'document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive loading is unavailable for this app")', 4),
            ('plugin_sdk_fault(format!("recursive document archive operation {operation} is unavailable for this app"))', 'document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, format!("recursive document archive operation {operation} is unavailable for this app"))', 1),
            ('plugin_sdk_fault("recursive document archive reading is unavailable for this app")', 'document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, "recursive document archive reading is unavailable for this app")', 1),
            ('return Err(plugin_sdk_fault("document archive exceeds its fixed 1024-member authority"));', 'return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive exceeds its fixed 1024-member authority").with_param("count", archive.members.len().to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));', 1),
            ('return Err(plugin_sdk_fault("document archive typed payload exceeds its fixed byte authority"));', 'return Err(document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive typed payload exceeds its fixed byte authority").with_param("bytes", archive_payload_bytes.unwrap_or(usize::MAX).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()));', 1),
            ('plugin_sdk_fault("document archive operation identity is already live or its fixed slot is occupied")', 'document_load_fault(DOCUMENT_LOAD_BUSY_CODE, "document archive operation identity is already live or its fixed slot is occupied")', 1),
            ('plugin_sdk_fault("document archive parent pack and SPR must both be present")', 'document_load_fault(DOCUMENT_LOAD_INCOMPLETE_CODE, "document archive parent pack and SPR must both be present")', 1),
            ('return Err(plugin_sdk_fault("document archive member ordinal, identity, owner, or envelope is invalid"));', 'return Err(document_load_fault(DOCUMENT_LOAD_MEMBER_INVALID_CODE, "document archive member ordinal, identity, owner, or envelope is invalid").with_param("ordinal", (ordinal + 1).to_string()));', 1),
            ('protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(), limits).map_err(|error| plugin_sdk_fault(error.to_string()))?', 'protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(), limits).map_err(|error| document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, error.to_string()))?', 1),
            ('plugin_sdk_fault("recursive document archive authority changed before status publication")', 'document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive authority changed before status publication")', 1),
            ('plugin_sdk_fault("unknown recursive document archive operation")', 'document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation")', 2),
            ('plugin_sdk_fault("terminal recursive document archive operation cannot be cancelled")', 'document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive operation cannot be cancelled")', 1),
            ('plugin_sdk_fault("terminal recursive document archive authority changed before acknowledgement")', 'document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive authority changed before acknowledgement")', 1),
            ('plugin_sdk_fault("document archive authority changed during generation-fenced export")', 'document_load_fault(DOCUMENT_LOAD_CHANGED_CODE, "document archive authority changed during generation-fenced export")', 1),
            ('return Err(plugin_sdk_fault("document archive genesis exceeds its fixed 1024-member authority"));', 'return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive genesis exceeds its fixed 1024-member authority").with_param("count", (ordinal + 1).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));', 1),
            ('.ok_or_else(|| plugin_sdk_fault("document archive genesis exceeds its fixed byte authority"))?', '.ok_or_else(|| document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive genesis exceeds its fixed byte authority").with_param("bytes", payload_bytes.saturating_add(envelope_pack.len()).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()))?', 1),
            ('plugin_sdk_fault(format!("document archive parent SPR was rejected: {error}"))', 'document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent SPR was rejected: {error}"))', 1),
            ('plugin_sdk_fault(format!("document archive parent Pack and SPR hydration was rejected: {diagnostic:?}"))', 'document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent Pack and SPR hydration was rejected: {diagnostic:?}"))', 1),
        ]
        converted = 0
        for old, new, expected in exact:
            if text.count(old) != expected:
                raise SystemExit(f"plugin f16: expected {expected} of {old[:70]!r}, found {text.count(old)}")
            text = text.replace(old, new)
            converted += expected
        for prefix in ['"document archive ', '"recursive document archive ', '"ready recursive document archive ', '"terminal recursive document archive ', 'format!("document archive ', 'format!("recursive document archive ']:
            old, new = "plugin_sdk_fault(" + prefix, "document_load_fault(DOCUMENT_LOAD_FAILED_CODE, " + prefix
            converted += text.count(old)
            text = text.replace(old, new)
        if before - text.count("plugin_sdk_fault(") != converted or converted != 71:
            raise SystemExit(f"plugin f16: converted {converted} sites, plugin_sdk_fault fell by {before - text.count('plugin_sdk_fault(')}; the reviewed census is 71")
        writes[runtime] = text
    laws = PLUGIN / "🧪️tests/🧪️folder-reload-route/🦀️.rs"
    text = laws.read_text()
    if "every_refused_whole_document_load_is_named_and_told_in_every_locale" not in text:
        text = text.rstrip("\n") + "\n" + (STAGE / "wave-f16/law.rs").read_text()
    writes[laws] = text
    kernel = REPO / "🧰️framework/🔨️modules/🎠️kernel"
    en = "Parts of this document changed elsewhere, so it is loaded again instead of merged."
    de = "Teile dieses Dokuments wurden anderswo geändert, daher wird es neu geladen statt zusammengeführt."
    table = kernel / "🦀️.rs"
    text = table.read_text()
    if '"plugin.document-load.members-differ"' not in text:
        import re
        declared = re.findall(r"pub const FRAMEWORK_FAULT_NOTICE_LABELS: \[\(&str, &str, &str\); (\d+)\] = \[", text)
        if len(declared) != 1:
            raise SystemExit(f"kernel: table length: expected exactly one declaration, found {len(declared)}")
        text = replace_once(text, f"pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); {declared[0]}] = [", f"pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); {int(declared[0]) + 1}] = [", "kernel: table length")
        row = '    ("plugin.document-load.other-document", "This is another document, so it cannot be merged into the open one.", "Das ist ein anderes Dokument, es lässt sich nicht mit dem geöffneten zusammenführen."),\n'
        text = replace_once(text, row, row + f'    ("plugin.document-load.members-differ", "{en}", "{de}"),\n', "kernel: row")
    writes[table] = text
    twin = kernel / "🟦️.ts"
    text = twin.read_text()
    if '"plugin.document-load.members-differ"' not in text:
        row = '  { code: "plugin.document-load.other-document", en: "This is another document, so it cannot be merged into the open one.", de: "Das ist ein anderes Dokument, es lässt sich nicht mit dem geöffneten zusammenführen." },\n'
        text = replace_once(text, row, row + f'  {{ code: "plugin.document-load.members-differ", en: "{en}", de: "{de}" }},\n', "kernel twin: row")
    writes[twin] = text
    fixture = kernel / "🧫️fixtures/🧫️framework-notices/🔣️.json"
    text = fixture.read_text()
    if '"plugin.document-load.members-differ"' not in text:
        row = '    { "code": "plugin.document-load.other-document", "en": "This is another document, so it cannot be merged into the open one.", "de": "Das ist ein anderes Dokument, es lässt sich nicht mit dem geöffneten zusammenführen." },\n'
        text = replace_once(text, row, row + f'    {{ "code": "plugin.document-load.members-differ", "en": "{en}", "de": "{de}" }},\n', "kernel fixture: row")
    writes[fixture] = text
    return writes, []


def wave_f6() -> tuple[dict, list]:
    """⏳️ Live fault F6 (Rust, `landing` lock): a cancelled or finished-and-retiring load no longer reads as loading (the
    history body's loading row and the `document.loading` verb refusal ask the machine's own `loading()`), and the load
    counts the history records it decodes into its progress, so a document with a history never reads "0 of 1". One line of
    `⏪️time-travel/🦀️.rs` (`live_document_load`) follows the predicate. Two laws in the route law file."""
    writes = {}
    runtime = PLUGIN / "🦀️.rs"
    text = runtime.read_text()
    if "fn loading(&self) -> bool {" not in text:
        text = replace_once(text, "        fold: (u64, u64),\n        fault: Vec<u8>,\n", "        fold: (u64, u64),\n        decoded: u64,\n        fault: Vec<u8>,\n", "plugin f6: field")
        text = replace_once(text, "                fold: (0, 0),\n                fault: Vec::new(),\n", "                fold: (0, 0),\n                decoded: 0,\n                fault: Vec::new(),\n", "plugin f6: field init")
        text = replace_once(
            text,
            "                        protocol::RetainedHistoryDecodeStep::Pending { .. } => Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: maximum_bytes.min(archive.parent_spr.len()) }),\n",
            "                        protocol::RetainedHistoryDecodeStep::Pending { decoded_records, .. } => {\n"
            "                            let released_bytes = maximum_bytes.min(archive.parent_spr.len());\n"
            "                            let discovered = decoded_records.saturating_sub(active.decoded);\n"
            "                            active.decoded = decoded_records;\n"
            "                            active.completed = active.completed.saturating_add(discovered);\n"
            "                            active.total = active.total.saturating_add(discovered);\n"
            "                            Ok(PluginCloseStep::Pending { released_items: 0, released_bytes })\n"
            "                        }\n",
            "plugin f6: decode progress",
        )
        text = replace_once(text, "                            active.completed = 1;\n", "                            active.completed = active.completed.saturating_add(1);\n", "plugin f6: parent unit")
        text = replace_once(
            text,
            "        fn terminal(&self) -> bool {\n            matches!(self.state, ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Cancelled | ActiveDocumentArchiveLoadState::Fault)\n        }\n",
            "        fn terminal(&self) -> bool {\n            matches!(self.state, ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Cancelled | ActiveDocumentArchiveLoadState::Fault)\n        }\n\n"
            "        /// ⏳️ Whether the document is still being loaded: not once the load ended, and not while a cancelled, faulted or\n"
            "        /// committed load only retires what it held — the document is then already the one it will stay.\n"
            "        fn loading(&self) -> bool {\n            !self.terminal() && self.terminal_target.is_none()\n        }\n",
            "plugin f6: loading predicate",
        )
        text = replace_once(
            text,
            "            self.document_archive_loads.each_id(|operation| loading |= self.document_archive_loads.get(operation).is_some_and(|load| !load.terminal()));\n",
            "            self.document_archive_loads.each_id(|operation| loading |= self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading));\n",
            "plugin f6: verb refusal",
        )
    writes[runtime] = text
    travel = PLUGIN / "⏪️time-travel/🦀️.rs"
    text = travel.read_text()
    text = replace_once(
        text,
        "            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(|load| !load.terminal()) {\n",
        "            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading) {\n",
        "time travel f6: live load",
    )
    writes[travel] = text
    laws = PLUGIN / "🧪️tests/🧪️folder-reload-route/🦀️.rs"
    text = laws.read_text()
    if "a_cancelled_load_stops_reading_as_loading_at_once_and_retires_without_a_host_poll" not in text:
        text = text.rstrip("\n") + "\n" + (STAGE / "wave-f6/law.rs").read_text()
    writes[laws] = text
    return writes, []


def wave_merge_guest() -> tuple[dict, list]:
    """🔀️ Design §22.22, guest half (Rust, `landing`; needs waves f16 and f6, S5-STORE's `merge_persisted_history` and wave B's
    `AppCommand::MergeDocumentArchive` + `DocumentArchiveLoadStatus.ahead`): `PluginApp::begin_document_archive_merge` admits a
    read-back like a load and flags it a merge; the machine decodes the `.spr` stepped, then merges the decoded log in ONE
    store call (`MergeParent`), delivers the base move to an open history edit and ends `Ready` with `ahead`; its
    `completed == total` is the number of events it took. A merge never
    reads as loading. An archive of another document is refused `plugin.document-load.other-document`; one whose members
    differ from the live members `plugin.document-load.members-differ` (the member merge is S5-NESTED's)."""
    writes = {}
    runtime = PLUGIN / "🦀️.rs"
    text = runtime.read_text()
    for needed in ["pub fn document_load_fault(", "fn loading(&self) -> bool {", "protocol::AppCommand::MergeDocumentArchive", "ahead: 0"]:
        if needed not in text:
            raise SystemExit(f"plugin merge: {needed!r} is not on disk yet (waves f16, f6 and wave B land first)")
    if "fn begin_document_archive_merge(&mut self, operation: u64" not in text:
        text = replace_once(text, '    pub const DOCUMENT_LOAD_FAILED_CODE: &str = "plugin.document-load.failed";\n', '    pub const DOCUMENT_LOAD_FAILED_CODE: &str = "plugin.document-load.failed";\n    pub const DOCUMENT_LOAD_OTHER_DOCUMENT_CODE: &str = "plugin.document-load.other-document";\n    pub const DOCUMENT_LOAD_MEMBERS_DIFFER_CODE: &str = "plugin.document-load.members-differ";\n', "plugin merge: codes")
        text = replace_once(text, "        decoded: u64,\n        fault: Vec<u8>,\n", "        decoded: u64,\n        merge: bool,\n        ahead: u64,\n        fault: Vec<u8>,\n", "plugin merge: fields")
        text = replace_once(text, "                decoded: 0,\n                fault: Vec::new(),\n", "                decoded: 0,\n                merge: false,\n                ahead: 0,\n                fault: Vec::new(),\n", "plugin merge: field init")
        text = replace_once(text, "            !self.terminal() && self.terminal_target.is_none()\n", "            !self.merge && !self.terminal() && self.terminal_target.is_none()\n", "plugin merge: a merge never loads")
        text = replace_once(
            text,
            "        /// ⏳️ Whether the document is still being loaded: not once the load ended, and not while a cancelled, faulted or\n        /// committed load only retires what it held — the document is then already the one it will stay.\n",
            "        /// ⏳️ Whether the document is still being loaded: not once the load ended, not while a cancelled, faulted or\n        /// committed load only retires what it held — the document is then already the one it will stay — and never for a\n        /// read-back merge, which replaces nothing.\n",
            "plugin merge: loading doc",
        )
        head = text.index("        fn status(&self) -> protocol::DocumentArchiveLoadStatus {")
        tail = text.index("        fn terminal(&self) -> bool {", head)
        body = text[head:tail]
        if body.count("ahead: 0") != 1:
            raise SystemExit(f"plugin merge: the machine's status carries {body.count('ahead: 0')} `ahead: 0`")
        text = text[:head] + body.replace("ahead: 0", "ahead: self.ahead") + text[tail:]
        text = replace_once(
            text,
            "                            let discovered = decoded_records.saturating_sub(active.decoded);\n",
            "                            let discovered = if active.merge { 0 } else { decoded_records.saturating_sub(active.decoded) };\n",
            "plugin merge: a merge counts merged events only",
        )
        text = replace_once(text, "        DecodeParent,\n        RetireParentHistoryAuxiliary,\n        HydrateParent,\n", "        DecodeParent,\n        RetireParentHistoryAuxiliary,\n        MergeParent,\n        HydrateParent,\n", "plugin merge: phase")
        text = replace_once(
            text,
            "                            active.phase = ActiveDocumentArchiveLoadPhase::HydrateParent;\n",
            "                            active.phase = if active.merge { ActiveDocumentArchiveLoadPhase::MergeParent } else { ActiveDocumentArchiveLoadPhase::HydrateParent };\n",
            "plugin merge: phase switch",
        )
        text = replace_once(
            text,
            "                ActiveDocumentArchiveLoadPhase::HydrateParent => {\n",
            "                ActiveDocumentArchiveLoadPhase::MergeParent => Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }),\n"
            "                ActiveDocumentArchiveLoadPhase::HydrateParent => {\n",
            "plugin merge: the decoded read-back waits for the host's poll",
        )
        text = replace_once(
            text,
            "        fn request_fault(&mut self, fault: &Fault) {\n",
            "        /// 🔀️ Whether a read-back merge decoded its log and waits for the host's poll turn to merge it.\n"
            "        fn awaits_merge(&self) -> bool {\n"
            "            self.phase == ActiveDocumentArchiveLoadPhase::MergeParent && self.terminal_target.is_none()\n"
            "        }\n\n"
            "        fn request_fault(&mut self, fault: &Fault) {\n",
            "plugin merge: awaits_merge",
        )
        text = replace_once(
            text,
            "        fn drive_document_archive_load_retirements(&mut self, maximum_items: usize, maximum_bytes: usize, closing: bool) -> Result<PluginCloseStep, Fault> {\n",
            "        /// 🔀️ Merges a decoded read-back into the live document (design §22.22) in the host's own poll turn and requests the\n"
            "        /// operation's terminal: `Ready` with what it took (`completed == total`) and is ahead by, or the fault that\n"
            "        /// refused it. Nothing is replaced.\n"
            "        async fn merge_document_archive(&mut self, operation: u64) -> Result<(), Fault> {\n"
            "            let merged = self.merge_decoded_document_archive(operation).await;\n"
            "            let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, \"recursive document archive merge authority changed during its merge\"))?;\n"
            "            match merged {\n"
            "                Ok((taken, ahead)) => {\n"
            "                    active.completed = taken;\n"
            "                    active.total = taken;\n"
            "                    active.ahead = ahead;\n"
            "                    active.request_terminal(ActiveDocumentArchiveLoadState::Ready);\n"
            "                }\n"
            "                Err(fault) => active.request_fault(&fault),\n"
            "            }\n"
            "            Ok(())\n"
            "        }\n\n"
            "        /// 🧬️ The merge itself, answering the events taken and the events this program holds that the archive lacks: the\n"
            "        /// archive's members must be the live ones (a member merge is the member's own, not yet available), its decoded\n"
            "        /// log joins the store's in one store call — an archive of another document is refused — and an open history edit\n"
            "        /// hears the base move like after any remote edit.\n"
            "        async fn merge_decoded_document_archive(&mut self, operation: u64) -> Result<(u64, u64), Fault> {\n"
            "            let live = self.archive_member_entries().await?;\n"
            "            let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, \"recursive document archive merge authority changed before its merge\"))?;\n"
            "            let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, \"recursive document archive merge input owner is absent\"))?;\n"
            "            if !live.iter().eq(archive.members.iter().rev()) {\n"
            "                return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, \"the archive's members differ from this document's live members; a member merge is not available\"));\n"
            "            }\n"
            "            let pack = std::mem::take(&mut archive.parent_pack);\n"
            "            let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, \"recursive document archive decoded history owner is absent\"))?;\n"
            "            let generation = self.store.generation();\n"
            "            let merge = match self.store.merge_persisted_history(&pack, history).await {\n"
            "                Ok(merge) => merge,\n"
            "                Err(store::VcsError::ValidationFailed(detail)) => return Err(document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail)),\n"
            "                Err(error) => return Err(error.into_fault()),\n"
            "            };\n"
            "            self.cache = None;\n"
            "            if self.store.generation() != generation {\n"
            "                self.deliver_base_moved().await?;\n"
            "            }\n"
            "            self.follow_derivable_children().await?;\n"
            "            Ok((u64::try_from(merge.merged).unwrap_or(u64::MAX), u64::try_from(merge.ahead).unwrap_or(u64::MAX)))\n"
            "        }\n\n"
            "        fn drive_document_archive_load_retirements(&mut self, maximum_items: usize, maximum_bytes: usize, closing: bool) -> Result<PluginCloseStep, Fault> {\n",
            "plugin merge: the merge runs in the host's poll turn",
        )
        text = replace_once(
            text,
            "            while self.document_archive_loads.get(operation).is_some_and(|active| !active.terminal()) {\n                self.maintenance_step(1, DOCUMENT_ARCHIVE_POLL_STEP_BYTES)?;\n",
            "            while self.document_archive_loads.get(operation).is_some_and(|active| !active.terminal()) {\n"
            "                if self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::awaits_merge) {\n"
            "                    self.merge_document_archive(operation).await?;\n"
            "                }\n"
            "                self.maintenance_step(1, DOCUMENT_ARCHIVE_POLL_STEP_BYTES)?;\n",
            "plugin merge: poll drives the merge",
        )
        text = replace_once(
            text,
            "        /// 🏃️ A host poll is the interactive drive of a whole-document load. The background\n",
            "        /// 🏃️ A host poll is the interactive drive of a whole-document load, and the only drive of a read-back merge once\n"
            "        /// its log is decoded (`merge_document_archive`: the merge awaits the store and the app, which a maintenance step\n"
            "        /// cannot). The background\n",
            "plugin merge: poll doc",
        )
        text = replace_once(
            text,
            "        /// 📬️ Advances and observes one exact retained recursive archive operation.\n        async fn poll_document_archive_load(&mut self, operation: u64) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {\n",
            "        /// 🔀️ Admits a read-back of the document this program already shows as a MERGE (design §22.22): its events join the\n"
            "        /// log, nothing is replaced. Polled, cancelled and acknowledged like a load under the same operation.\n"
            "        fn begin_document_archive_merge(&mut self, _operation: u64, _archive: protocol::DocumentArchivePack) -> Result<(), Fault> {\n"
            "            Err(document_load_fault(DOCUMENT_LOAD_UNAVAILABLE_CODE, \"recursive document archive merging is unavailable for this app\"))\n"
            "        }\n"
            "        /// 📬️ Advances and observes one exact retained recursive archive operation.\n        async fn poll_document_archive_load(&mut self, operation: u64) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {\n",
            "plugin merge: trait default",
        )
        text = replace_once(
            text,
            "        /// 🏃️ A host poll is the interactive drive of a whole-document load, and",
            "        fn begin_document_archive_merge(&mut self, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {\n"
            "            PluginApp::begin_document_archive_load(self, operation, archive)?;\n"
            "            self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, \"recursive document archive merge admission changed before it was flagged\"))?.merge = true;\n"
            "            Ok(())\n"
            "        }\n\n"
            "        /// 🏃️ A host poll is the interactive drive of a whole-document load, and",
            "plugin merge: impl",
        )
        text = replace_once(
            text,
            "                protocol::AppCommand::MergeDocumentArchive { seq, .. } => {\n"
            "                    push_app_fault(&mut frames, Some(seq), Fault::new(FaultOrigin::Framework, FaultCode::new(\"app.command.unsupported\"), \"this program does not merge a document archive yet\")).await;\n"
            "                }\n",
            "                protocol::AppCommand::MergeDocumentArchive { seq, archive } => {\n"
            "                    let admitted = with_instances_mut(runtime, |list| {\n"
            "                        let mut instance = find_instance(list, instance_id)?;\n"
            "                        instance.app.begin_document_archive_merge(seq, archive)\n"
            "                    });\n"
            "                    match admitted.await {\n"
            "                        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),\n"
            "                        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,\n"
            "                    }\n"
            "                }\n",
            "plugin merge: channel arm",
        )
    writes[runtime] = text
    dispatch = PLUGIN / "🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"
    text = dispatch.read_text()
    if "a_merge_archive_command_is_admitted_under_its_own_sequence_on_both_routes" not in text:
        text = replace_once(
            text,
            "/// 🧲️ Until the merge handlers land (design §22.22) a program answers `MergeDocumentArchive` with its own typed refusal on\n/// the encoded and on the decoded route: an `Error` frame naming `app.command.unsupported`, never `Done`, never a decode fault.\n#[semio_framework_async_macros::async_test]\nasync fn a_merge_archive_command_is_refused_as_unsupported_until_its_handler_lands() {\n",
            "/// 🧲️ `MergeDocumentArchive` reaches its handler on the encoded and on the decoded route (design §22.22): the program admits\n/// the read-back under the command's own sequence and answers `Done`, never an error frame and never a decode fault. What\n/// the merge then does is the folder route law's (`🧪️tests/🧪️folder-reload-route`); closing the program cancels what it admitted.\n#[semio_framework_async_macros::async_test]\nasync fn a_merge_archive_command_is_admitted_under_its_own_sequence_on_both_routes() {\n",
            "dispatch law: head",
        )
        text = replace_once(
            text,
            "        let refusal = frames\n            .iter()\n            .find_map(|frame| match frame {\n                protocol::AppFrame::Error { in_reply_to: Some(answered), fault, .. } if *answered == seq => Some(fault),\n                _ => None,\n            })\n            .unwrap_or_else(|| panic!(\"the merge is refused through its own error frame: {frames:?}\"));\n        let fault: Fault = super::super::decode_wire_serialized(refusal).await.unwrap();\n        assert_eq!(fault.code.0, \"app.command.unsupported\");\n        assert!(!frames.iter().any(|frame| matches!(frame, protocol::AppFrame::Done { .. })), \"a refused merge is not done: {frames:?}\");\n",
            "        assert!(frames.iter().any(|frame| matches!(frame, protocol::AppFrame::Done { in_reply_to } if *in_reply_to == seq)), \"the merge is admitted under its own sequence: {frames:?}\");\n        assert!(!frames.iter().any(|frame| matches!(frame, protocol::AppFrame::Error { .. })), \"an admitted merge answers no error frame: {frames:?}\");\n",
            "dispatch law: body",
        )
    writes[dispatch] = text
    return writes, []


def wave_checkin() -> tuple[dict, list]:
    """⏳️ Live finding O4 (served TypeScript: `serve` lock): an automatic check-in never surfaces a refusal. It waits while its
    document is loading or attached with an unbound port and is asked for again one idle period later
    (`AutoCheckinScheduler.defer`, `automaticCheckinWaitsV1`); ShellHost's `dispatchCheckpoint` answers whether it dispatched.
    Corpus `🛠️ShellHelpers/🧫️fixtures/🧫️automatic-checkin` + schema; the law joins the scheduler's suite."""
    writes, created = {}, []
    elements = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements"
    stage = STAGE / "wave-checkin"
    helpers = elements / "🛠️ShellHelpers/🟦️.tsx"
    text = helpers.read_text()
    if "export function automaticCheckinWaitsV1" not in text:
        text = replace_once(
            text,
            "    this.cancel();\n    this.timer = setTimeout(() => {\n      this.timer = null;\n      this.pending = true;\n      this.onCheckpoint();\n    }, this.idleMs);\n  }\n\n  cancel(): void {\n",
            "    this.cancel();\n    this.arm();\n  }\n\n"
            "  /** ⏳️ The checkpoint just asked for cannot be dispatched now — its document is loading, its port is not bound, its\n"
            "   * history is under edit: the latch is released and the idle period starts again, so the check-in is asked for again\n"
            "   * instead of being lost or told to a person who pressed nothing (live finding O4). */\n"
            "  defer(): void {\n    this.cancel();\n    this.pending = false;\n    this.arm();\n  }\n\n"
            "  private arm(): void {\n    this.timer = setTimeout(() => {\n      this.timer = null;\n      this.pending = true;\n      this.onCheckpoint();\n    }, this.idleMs);\n  }\n\n  cancel(): void {\n",
            "ShellHelpers: scheduler defer",
        )
        text = replace_once(
            text,
            "/** 📌️ The framework-reserved controller of the history body's explicit `#s-checkin` button",
            "/** ⏳️ Whether an AUTOMATIC check-in waits (live finding O4): an automatic action never surfaces a refusal, so it is not\n"
            " * dispatched into a document that cannot take a checkpoint now — one that is loading (the guest would answer\n"
            " * `document.loading`), or one that is attached while its port is not bound (what it published would reach no folder and\n"
            " * no hub). Corpus `🧫️fixtures/🧫️automatic-checkin/🔣️.json`. */\n"
            "export function automaticCheckinWaitsV1(document: Readonly<{ loading: boolean; attached: boolean; bound: boolean }>): boolean {\n"
            "  return document.loading || (document.attached && !document.bound);\n}\n\n"
            "/** 📌️ The framework-reserved controller of the history body's explicit `#s-checkin` button",
            "ShellHelpers: predicate",
        )
    writes[helpers] = text
    shell = elements / "🏛️ShellHost/🟦️.tsx"
    text = shell.read_text()
    if "automaticCheckinWaitsV1" not in text:
        text = replace_once(text, "  AutoCheckinScheduler,\n", "  AutoCheckinScheduler,\n  automaticCheckinWaitsV1,\n", "ShellHost: import")
        text = replace_once(
            text,
            "    (message: string) => {\n      if (!session) return;\n      const gate = checkpointGateV1(focusedHistoryV1().timeTravel, message);\n      if (gate === \"frozen\") {\n        const notice = historyRefusalNoticeV1(\"timeTravel.frozen\");\n        showTransientNoticeRef.current(notice.text, notice.kind, notice.code);\n      }\n      if (gate !== \"dispatch\") return;\n      checkpointDispatchedRef.current = true;\n",
            "    (message: string): boolean => {\n      if (!session) return false;\n      const gate = checkpointGateV1(focusedHistoryV1().timeTravel, message);\n      if (gate === \"frozen\") {\n        const notice = historyRefusalNoticeV1(\"timeTravel.frozen\");\n        showTransientNoticeRef.current(notice.text, notice.kind, notice.code);\n      }\n      if (gate !== \"dispatch\") return false;\n"
            "      if (message === \"auto\") {\n"
            "        const attached = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId);\n"
            "        const loading = focusedHistoryV1().reprojection?.kind === \"load\" || attached?.replacements.pending === true;\n"
            "        if (automaticCheckinWaitsV1({ loading, attached: attached !== undefined, bound: attached !== undefined && attached.port !== null && !attached.port.closing })) return false;\n"
            "      }\n"
            "      checkpointDispatchedRef.current = true;\n",
            "ShellHost: dispatchCheckpoint",
        )
        text = replace_once(
            text,
            "      onAction({ controllerId: session.app.controllerId, action: \"commitCheckpoint\", args: { message, authors } });\n    },\n    [focusedHistoryV1, session, onAction],\n",
            "      onAction({ controllerId: session.app.controllerId, action: \"commitCheckpoint\", args: { message, authors } });\n      return true;\n    },\n    [focusedHistoryV1, session, onAction],\n",
            "ShellHost: dispatchCheckpoint result",
        )
        text = replace_once(
            text,
            "    const scheduler = new AutoCheckinScheduler(() => dispatchCheckpoint(\"auto\"));\n",
            "    const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {\n      if (!dispatchCheckpoint(\"auto\")) scheduler.defer();\n    });\n",
            "ShellHost: scheduler callback",
        )
    writes[shell] = text
    fixture = elements / "🛠️ShellHelpers/🧫️fixtures/🧫️automatic-checkin/🔣️.json"
    writes[fixture] = (stage / "fixture.json").read_text()
    created.append(fixture)
    schema = elements / "🛠️ShellHelpers/🧬️schema/🔣️automatic-checkin/🔣️.json"
    writes[schema] = (stage / "schema.json").read_text()
    created.append(schema)
    suite = MODULES / "📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts"
    text = suite.read_text()
    if "automaticCheckinCorpus" not in text:
        text = replace_once(
            text,
            'import Ajv2020 from "ajv/dist/2020";\n',
            'import Ajv2020 from "ajv/dist/2020";\nimport automaticCheckinCorpus from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧫️automatic-checkin/🔣️.json";\nimport automaticCheckinSchema from "../../🧱️elements/🛠️ShellHelpers/🧬️schema/🔣️automatic-checkin/🔣️.json";\n',
            "engine-contract: corpus imports",
        )
        direct = '} from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";\n'
        first = text.index(direct)
        text = text[:first] + "  automaticCheckinWaitsV1,\n" + text[first:]
        text = replace_once(
            text,
            "      scheduler.notify(1);\n      scheduler.cancel();\n      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);\n      expect(onCheckpoint).not.toHaveBeenCalled();\n    });\n",
            "      scheduler.notify(1);\n      scheduler.cancel();\n      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);\n      expect(onCheckpoint).not.toHaveBeenCalled();\n    });\n\n"
            "    it(\"an automatic check-in waits while its document cannot take it and is asked for again, per the shared corpus (live finding O4)\", () => {\n"
            "      const validate = new Ajv2020({ strict: true, allErrors: true }).compile(automaticCheckinSchema);\n"
            "      expect(validate(automaticCheckinCorpus), JSON.stringify(validate.errors)).toBe(true);\n"
            "      for (const row of automaticCheckinCorpus.waits) expect(automaticCheckinWaitsV1(row.document), row.id).toBe(row.waits);\n"
            "      for (const timeline of automaticCheckinCorpus.timelines) {\n"
            "        vi.useFakeTimers();\n"
            "        const start = Date.now();\n"
            "        let document = { loading: false, attached: false, bound: false };\n"
            "        const dispatched: number[] = [];\n"
            "        const deferred: number[] = [];\n"
            "        const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {\n"
            "          if (automaticCheckinWaitsV1(document)) {\n"
            "            deferred.push(Date.now() - start);\n"
            "            scheduler.defer();\n"
            "          } else dispatched.push(Date.now() - start);\n"
            "        }, automaticCheckinCorpus.idleMs, automaticCheckinCorpus.threshold);\n"
            "        for (const event of timeline.events as readonly { readonly notify?: number; readonly advance?: number; readonly document?: typeof document }[]) {\n"
            "          if (event.document !== undefined) document = event.document;\n"
            "          else if (event.notify !== undefined) scheduler.notify(event.notify);\n"
            "          else vi.advanceTimersByTime(event.advance ?? 0);\n"
            "        }\n"
            "        expect([dispatched, deferred], timeline.id).toEqual([timeline.dispatched, timeline.deferred]);\n"
            "        scheduler.cancel();\n"
            "        vi.useRealTimers();\n"
            "      }\n"
            "    });\n",
            "engine-contract: law",
        )
    writes[suite] = text
    return writes, created


MERGE_HOST_ROWS = """,
    {
      "name": "a-read-back-merge-admits-with-its-own-command-and-ends-ready-with-what-the-program-is-ahead-by",
      "merging": true,
      "cancel": null,
      "exchanges": [
        { "sends": "mergeDocumentArchive", "answer": { "kind": "done" } },
        { "sends": "pollDocumentArchiveLoad", "answer": { "kind": "status", "state": "running", "completed": 0, "total": 3 } },
        { "sends": "pollDocumentArchiveLoad", "answer": { "kind": "status", "state": "ready", "completed": 3, "total": 3, "ahead": 2 } },
        { "sends": "acknowledgeDocumentArchiveLoad", "answer": { "kind": "done" } }
      ],
      "outcome": { "kind": "ready", "ahead": 2 }
    },
    {
      "name": "a-read-back-merge-of-another-document-ends-in-the-guests-own-fault",
      "merging": true,
      "cancel": null,
      "exchanges": [
        { "sends": "mergeDocumentArchive", "answer": { "kind": "done" } },
        { "sends": "pollDocumentArchiveLoad", "answer": { "kind": "status", "state": "fault", "completed": 0, "total": 1, "fault": "plugin.document-load.other-document" } },
        { "sends": "acknowledgeDocumentArchiveLoad", "answer": { "kind": "done" } }
      ],
      "outcome": { "kind": "fault", "fault": "plugin.document-load.other-document" }
    },
    {
      "name": "a-cancelled-read-back-merge-is-polled-to-its-cancelled-terminal",
      "merging": true,
      "cancel": { "beforeStep": 1 },
      "exchanges": [
        { "sends": "mergeDocumentArchive", "answer": { "kind": "done" } },
        { "sends": "cancelDocumentArchiveLoad", "answer": { "kind": "done" } },
        { "sends": "pollDocumentArchiveLoad", "answer": { "kind": "status", "state": "cancelled", "completed": 0, "total": 1 } },
        { "sends": "acknowledgeDocumentArchiveLoad", "answer": { "kind": "done" } }
      ],
      "outcome": { "kind": "cancelled" }
    }"""


MERGE_TYPES = """/** 🔀️ How one read-back merge ended (design §22.22): `merged` events of the archive joined the program's log and the program
 * holds `ahead` events the archive lacks — or the program cannot merge the archive into the document it shows and `code` is
 * its own refusal ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}); nothing was touched, and loading the archive is the way on. */
export type DocumentArchiveMergeV1 = { readonly kind: "merged"; readonly merged: number; readonly ahead: number } | { readonly kind: "unmergeable"; readonly code: string };

/** 🚫️ The guest's refusals of a read-back merge that leave a load as the way on: the archive is another document's, or its
 * members differ from the live ones (twins of `DOCUMENT_LOAD_OTHER_DOCUMENT_CODE` and `DOCUMENT_LOAD_MEMBERS_DIFFER_CODE`,
 * `🔨️modules/🔌️plugin/🦀️.rs`). */
export const DOCUMENT_ARCHIVE_UNMERGEABLE_CODES: readonly string[] = ["plugin.document-load.other-document", "plugin.document-load.members-differ"];

"""

MERGE_CLIENT = """  async loadDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<void> {
    const root = { pack: Uint8Array.from(archive.parent_pack), spr: Uint8Array.from(archive.parent_spr) };
    await this.driveDocumentArchive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);
    this.cachedPack = root.pack;
    this.cachedSpr = root.spr;
  }

  /** 🔀️ Merges a read-back of the document this program already shows (design §22.22), stepped through
   * {@link DocumentArchiveLoadHost.merging}: the archive's events join the program's log, nothing is replaced, an open history
   * edit and the program's viewed alternative stay. Resolves `merged` with the events it took and the events the program is
   * `ahead` of the archive by — the cached root pair is dropped when it took any — or `unmergeable` with the guest's code
   * ({@link DOCUMENT_ARCHIVE_UNMERGEABLE_CODES}), which touched nothing. Cancels, refusals and every other fault reject like
   * {@link loadDocumentArchive}'s. */
  async mergeDocumentArchive(
    archive: DocumentArchivePack,
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<DocumentArchiveMergeV1> {
    const merge = DocumentArchiveLoadHost.merging(archive);
    const ended = await this.driveDocumentArchive("mergeDocumentArchive", merge, DOCUMENT_ARCHIVE_UNMERGEABLE_CODES, signal, progress);
    if ("code" in ended) return { kind: "unmergeable", code: ended.code };
    if (ended.completed > 0) {
      this.cachedPack = null;
      this.cachedSpr = null;
    }
    return { kind: "merged", merged: ended.completed, ahead: merge.ahead };
  }

  /** 🚶️ Drives one whole-document load or merge to its acknowledged end: resolves with the count its ready terminal completed,
   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason
   * (an `AbortError` when the guest cancelled on its own); a refusal, a reply that answers nothing and every other fault
   * reject with the guest's own fault text. */
  private async driveDocumentArchive(
    name: string,
    host: DocumentArchiveLoadHost,
    unmergeable: readonly string[],
    signal?: AbortSignal,
    progress?: (status: DocumentArchiveLoadStatus) => void,
  ): Promise<{ readonly completed: number } | { readonly code: string }> {
    let completed = 0;
    for (;;) {
      if (signal?.aborted) host.requestCancel();
      const step = host.step(() => this.nextSeq());
      if (step.kind === "finished") {
        if (step.outcome.kind === "ready") return { completed };
        if (step.outcome.kind === "cancelled") throw signal?.reason ?? new DOMException(`AppChannelClient.${name}(${this.appId}): cancelled`, "AbortError");
        const code = decodeFaultFromWire(step.outcome.fault, decodePackValue)?.code;
        if (typeof code === "string" && unmergeable.includes(code)) return { code };
        throw new Error(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`);
      }
      const seq = step.seq;
      const replyFrames = await this.sendCommand(step.command);
      const frame = documentArchiveLoadReplyV1(replyFrames, seq);
      const answer: DocumentArchiveLoadAnswer = frame ? host.answer(seq, frame) : { kind: "unanswered" };
      if (answer.kind === "refused") throw new Error(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(answer.fault, decodePackValue)}`);
      if (answer.kind === "unanswered") throw new Error(`AppChannelClient.${name}(${this.appId}): no answer to seq ${seq} of operation ${host.operation}`);
      if (answer.status) {
        completed = answer.status.completed;
        progress?.(answer.status);
      }
      if (answer.status ? answer.status.state === "pending" || answer.status.state === "running" : frame !== undefined && "Error" in frame) await nextArchivePollTurnV1();
    }
  }
"""

MERGE_LAW_CLIENT = """/** 🧯️ The guest's own pack-encoded fault naming `code`, as a status of a real program carries it. */
const packedFault = (code: string | undefined): number[] => Array.from(encodePackValue({ origin: "framework", code: code ?? "", severity: "error", message: `scripted ${code}`, retryable: false }));

/** 📡️ A client over a channel that answers every command with its scripted frame and aborts where the row cancels. */
function scripted(row: Case, fault: (text: string | undefined) => number[]): { readonly client: AppChannelClient; readonly controller: AbortController; readonly reason: Error; readonly seen: AppCommandValue[]; readonly operation: () => number } {
  const controller = new AbortController();
  const reason = new Error(`${row.name}: caller cancelled`);
  const seen: AppCommandValue[] = [];
  let operation = 0;
  const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
  const handle: AppChannelHandle = {
    enqueue: (instanceId, events) => {
      const command = events.map(decodeAppCommand)[0]!;
      const index = seen.length;
      seen.push(command);
      const exchange = row.exchanges[index];
      if (!exchange) throw new Error(`${row.name}: unscripted command ${JSON.stringify(command)}`);
      if ("LoadDocumentArchive" in command) operation = command.LoadDocumentArchive.seq;
      if ("MergeDocumentArchive" in command) operation = command.MergeDocumentArchive.seq;
      if (cancelBefore(row) === index + 1 || cancelInFlight(row) === index) controller.abort(reason);
      const seq = Object.values(command)[0]!.seq;
      broadcast.push({ instanceId, frames: [encodeAppFrame(frame(exchange.answer, Number(seq), operation, fault))] });
    },
    outcomes: broadcast.stream,
  };
  if (cancelBefore(row) === 0) controller.abort(reason);
  return { client: new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.archive-law"), controller, reason, seen, operation: () => operation };
}

test("AppChannelClient.loadDocumentArchive replays the corpus over a scripted channel", async () => {
  for (const row of law.cases.filter((row) => !row.admittedByGuest && !row.merging)) {
    const { client, controller, reason, seen, operation } = scripted(row, bytes);
    const statuses: (readonly [string, number, number])[] = [];
    const settled = await client.loadDocumentArchive(archive, row.cancel ? controller.signal : undefined, (status) => statuses.push([status.state, status.completed, status.total])).then(
      () => ({ kind: "ready" }) as const,
      (error: unknown) => ({ kind: "rejected", error }) as const,
    );
    expect([row.name, seen.map((command) => sent(command)[0])]).toEqual([row.name, row.exchanges.map((exchange) => exchange.sends)]);
    expect([row.name, seen.every((command) => sent(command)[1] === operation())]).toEqual([row.name, true]);
    expect([row.name, statuses]).toEqual([row.name, statusesOf(row).map((status) => [...status])]);
    const expected = row.outcome;
    if (expected.kind === "ready") {
      expect([row.name, settled.kind, client.documentPack()]).toEqual([row.name, "ready", { pack: new Uint8Array([7]), spr: new Uint8Array([9]) }]);
      continue;
    }
    if (settled.kind !== "rejected") throw new Error(`${row.name}: resolved, scripted ${expected.kind}`);
    expect([row.name, client.documentPack()]).toEqual([row.name, null]);
    if (expected.kind === "cancelled") {
      const error = settled.error;
      expect([row.name, row.cancel ? error === reason : error instanceof DOMException && error.name === "AbortError"]).toEqual([row.name, true]);
    } else {
      const message = settled.error instanceof Error ? settled.error.message : String(settled.error);
      expect([row.name, message.includes(expected.kind === "unanswered" ? "no answer" : expected.fault)]).toEqual([row.name, true]);
    }
  }
});

test("AppChannelClient.mergeDocumentArchive replays the merging rows: what it took and is ahead by, an unmergeable archive as its code, every other fault a rejection", async () => {
  const rows = law.cases.filter((row) => row.merging);
  expect(rows.map((row) => row.outcome.kind).sort()).toEqual(["cancelled", "fault", "ready"]);
  for (const row of rows) {
    const { client, controller, reason, seen, operation } = scripted(row, packedFault);
    const settled = await client.mergeDocumentArchive(archive, row.cancel ? controller.signal : undefined).then(
      (merge) => ({ kind: "resolved", merge }) as const,
      (error: unknown) => ({ kind: "rejected", error }) as const,
    );
    expect([row.name, seen.map((command) => sent(command)[0])]).toEqual([row.name, row.exchanges.map((exchange) => exchange.sends)]);
    expect([row.name, seen.every((command) => sent(command)[1] === operation())]).toEqual([row.name, true]);
    expect([row.name, client.documentPack()]).toEqual([row.name, null]);
    const expected = row.outcome;
    if (expected.kind === "ready") expect([row.name, settled]).toEqual([row.name, { kind: "resolved", merge: { kind: "merged", merged: statusesOf(row).at(-1)?.[1] ?? 0, ahead: expected.ahead ?? 0 } }]);
    else if (expected.kind === "fault") expect([row.name, settled]).toEqual([row.name, { kind: "resolved", merge: { kind: "unmergeable", code: expected.fault } }]);
    else expect([row.name, settled.kind === "rejected" && settled.error === reason]).toEqual([row.name, true]);
  }
  const refused = rows.find((row) => row.outcome.kind === "fault")!;
  const broken: Case = { ...refused, exchanges: refused.exchanges.map((exchange) => (exchange.answer.kind === "status" && exchange.answer.fault ? { ...exchange, answer: { ...exchange.answer, fault: "plugin.document-load.history-invalid" } } : exchange)) };
  const failure = await scripted(broken, packedFault).client.mergeDocumentArchive(archive).then(() => "resolved", (error: unknown) => (error instanceof Error ? error.message : String(error)));
  expect(failure).toContain("plugin.document-load.history-invalid");
});

test("a merge that took events drops the cached root pair, and one that took none keeps it", async () => {
  for (const merged of [0, 2]) {
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    let operation = 0;
    let merging = false;
    const handle: AppChannelHandle = {
      enqueue: (instanceId, events) => {
        const command = decodeAppCommand(events[0]!);
        const seq = Number(Object.values(command)[0]!.seq);
        if ("LoadDocumentArchive" in command || "MergeDocumentArchive" in command) {
          operation = seq;
          merging = "MergeDocumentArchive" in command;
        }
        const count = merging ? merged : 1;
        const answer: AppFrameValue = "PollDocumentArchiveLoad" in command ? { DocumentArchiveLoad: { in_reply_to: seq, status: { operation, state: "ready", completed: count, total: count, ahead: 0, fault: [] } } } : { Done: { in_reply_to: seq } };
        broadcast.push({ instanceId, frames: [encodeAppFrame(answer)] });
      },
      outcomes: broadcast.stream,
    };
    const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.archive-law");
    await client.loadDocumentArchive(archive);
    expect([merged, client.documentPack()]).toEqual([merged, { pack: new Uint8Array([7]), spr: new Uint8Array([9]) }]);
    expect([merged, await client.mergeDocumentArchive(archive)]).toEqual([merged, { kind: "merged", merged, ahead: 0 }]);
    expect([merged, client.documentPack()]).toEqual([merged, merged === 0 ? { pack: new Uint8Array([7]), spr: new Uint8Array([9]) } : null]);
  }
});
"""


def merge_twin_writes(writes: dict) -> None:
    """🔀️ Design §22.22, host half in TypeScript (`serve`; lands in the same hold as the corpus rows it replays): the twin's
    `DocumentArchiveLoadHost.merging` / `ahead`, `AppChannelClient.mergeDocumentArchive`, the plugin handle's
    `mergeAppDocumentArchive` and the law's merging rows."""
    twin = REPO / "🧰️framework/🛍️products/💻️os/🟦️.ts"
    text = twin.read_text()
    if "readonly MergeDocumentArchive: { readonly seq: number; readonly archive: DocumentArchivePack }" not in text:
        raise SystemExit("os twin: wave B (`MergeDocumentArchive` command value) is not on disk yet")
    if "static merging(archive: DocumentArchivePack): DocumentArchiveLoadHost {" not in text:
        text = replace_once(text, 'type DocumentArchiveLoadSent = "admit" | "poll" | "cancel" | "acknowledge";\n', MERGE_TYPES + 'type DocumentArchiveLoadSent = "admit" | "poll" | "cancel" | "acknowledge";\n', "os twin: merge types")
        text = replace_once(
            text,
            " * cancelled terminal whose acknowledgement is refused still owns retained input, so it is polled again. */\nexport class DocumentArchiveLoadHost {\n",
            " * cancelled terminal whose acknowledgement is refused still owns retained input, so it is polled again. A read-back of the\n"
            " * document the program already shows is a merge ({@link merging}): the same exchange under its own admission command. */\nexport class DocumentArchiveLoadHost {\n",
            "os twin: class doc",
        )
        text = replace_once(
            text,
            "  private outcome: DocumentArchiveLoadOutcome | null = null;\n\n  constructor(archive: DocumentArchivePack) {\n    this.archive = archive;\n  }\n",
            "  private outcome: DocumentArchiveLoadOutcome | null = null;\n  private merge = false;\n  private aheadOfArchive = 0;\n\n  constructor(archive: DocumentArchivePack) {\n    this.archive = archive;\n  }\n\n"
            "  /** 🔀️ A read-back merge of the document the program already shows (design §22.22): admitted with `MergeDocumentArchive`,\n"
            "   * then polled, cancelled and acknowledged exactly like a load. Its events join the program's log and nothing is\n"
            "   * replaced; {@link ahead} answers what the program then holds that the archive lacks. */\n"
            "  static merging(archive: DocumentArchivePack): DocumentArchiveLoadHost {\n    const host = new DocumentArchiveLoadHost(archive);\n    host.merge = true;\n    return host;\n  }\n",
            "os twin: merging",
        )
        text = replace_once(
            text,
            "  get operation(): number | null {\n    return this.admitted;\n  }\n",
            "  get operation(): number | null {\n    return this.admitted;\n  }\n\n"
            "  /** ⏭️ The events the program holds that the archive lacks, as its acknowledged ready terminal counted them: what a writer\n"
            "   * persists again for. 0 for a replacement and before the operation ended ready. */\n"
            "  get ahead(): number {\n    return this.aheadOfArchive;\n  }\n",
            "os twin: ahead",
        )
        text = replace_once(
            text,
            "    const operation = this.admitted;\n    const [kind, command]: readonly [DocumentArchiveLoadSent, AppCommandValue] =\n"
            '      operation === null ? ["admit", { LoadDocumentArchive: { seq, archive: this.archive ?? { parent_pack: [], parent_spr: [], members: [] } } }]\n',
            "    const operation = this.admitted;\n    const archive = this.archive ?? { parent_pack: [], parent_spr: [], members: [] };\n    const [kind, command]: readonly [DocumentArchiveLoadSent, AppCommandValue] =\n"
            '      operation === null ? ["admit", this.merge ? { MergeDocumentArchive: { seq, archive } } : { LoadDocumentArchive: { seq, archive } }]\n',
            "os twin: admit",
        )
        text = replace_once(
            text,
            '        this.terminal = null;\n        this.outcome = terminal.state === "ready" ? { kind: "ready" }',
            '        this.terminal = null;\n        if (terminal.state === "ready") this.aheadOfArchive = terminal.ahead;\n        this.outcome = terminal.state === "ready" ? { kind: "ready" }',
            "os twin: acknowledge",
        )
        head = text.index("  async loadDocumentArchive(\n    archive: DocumentArchivePack,\n")
        tail = text.index("\n  /** 🗃️ Reads one generation-fenced root envelope", head)
        if "nextArchivePollTurnV1();\n    }\n  }\n" not in text[head:tail]:
            raise SystemExit("os twin: the client load method changed shape")
        text = text[:head] + MERGE_CLIENT + text[tail:]
        text = replace_once(
            text,
            "   * resolves, because the document did load. A refusal or a guest fault rejects with the guest's own fault text. */\n  async loadDocumentArchive(\n",
            "   * resolves, because the document did load. A refusal or a guest fault rejects with the guest's own fault text. Driven by\n   * {@link driveDocumentArchive}, like a merge. */\n  async loadDocumentArchive(\n",
            "os twin: load doc",
        )
    writes[twin] = text
    runtime = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"
    text = runtime.read_text()
    if "mergeAppDocumentArchive" not in text:
        text = replace_once(text, "type DocumentArchiveLoadStatus, type DocumentArchivePack,", "type DocumentArchiveLoadStatus, type DocumentArchiveMergeV1, type DocumentArchivePack,", "plugin runtime: import")
        text = replace_once(
            text,
            "  readonly loadAppDocumentArchive?: (instanceId: number, archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void) => Promise<void>;\n",
            "  readonly loadAppDocumentArchive?: (instanceId: number, archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void) => Promise<void>;\n"
            "  /** 🔀️ Merges a read-back of the document the instance already shows: its events join the instance's log and nothing is\n"
            "   * replaced; answers what was taken and what the instance is ahead by, or the code of an archive it cannot merge. */\n"
            "  readonly mergeAppDocumentArchive?: (instanceId: number, archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void) => Promise<DocumentArchiveMergeV1>;\n",
            "plugin runtime: handle type",
        )
        text = replace_once(
            text,
            "    loadAppDocumentArchive: (instanceId, archive, signal, progress) => requireChannel(instanceId).loadDocumentArchive(archive, signal, progress),\n",
            "    loadAppDocumentArchive: (instanceId, archive, signal, progress) => requireChannel(instanceId).loadDocumentArchive(archive, signal, progress),\n"
            "    mergeAppDocumentArchive: (instanceId, archive, signal, progress) => requireChannel(instanceId).mergeDocumentArchive(archive, signal, progress),\n",
            "plugin runtime: handle",
        )
    writes[runtime] = text
    law = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts"
    text = law.read_text()
    if "mergeDocumentArchive" not in text:
        text = replace_once(
            text,
            "`MediaIn`) replays through `DocumentArchiveLoadHost.admitted`; the client admits every load it drives, so it skips those rows. */\n",
            "`MediaIn`) replays through `DocumentArchiveLoadHost.admitted`; the client admits every load it drives, so it skips those rows.\n"
            " * A `merging` row (design §22.22) replays through `DocumentArchiveLoadHost.merging` and `AppChannelClient.mergeDocumentArchive`. */\n",
            "host law: doc",
        )
        text = replace_once(text, "import { AppChannelClient, AppChannelRequestSequence, DocumentArchiveLoadHost, decodeAppCommand, encodeAppFrame, type", "import { AppChannelClient, AppChannelRequestSequence, DocumentArchiveLoadHost, decodeAppCommand, encodeAppFrame, encodePackValue, type", "host law: import")
        text = replace_once(text, "readonly completed: number; readonly total: number; readonly fault?: string };\n", "readonly completed: number; readonly total: number; readonly ahead?: number; readonly fault?: string };\n", "host law: answer")
        text = replace_once(text, 'type Outcome = { readonly kind: "ready" } | ', 'type Outcome = { readonly kind: "ready"; readonly ahead?: number } | ', "host law: outcome")
        text = replace_once(text, "type Case = { readonly name: string; readonly admittedByGuest?: boolean;", "type Case = { readonly name: string; readonly merging?: boolean; readonly admittedByGuest?: boolean;", "host law: case")
        text = replace_once(
            text,
            '  if ("LoadDocumentArchive" in command) return ["loadDocumentArchive", command.LoadDocumentArchive.seq];\n',
            '  if ("LoadDocumentArchive" in command) return ["loadDocumentArchive", command.LoadDocumentArchive.seq];\n  if ("MergeDocumentArchive" in command) return ["mergeDocumentArchive", command.MergeDocumentArchive.seq];\n',
            "host law: sent",
        )
        text = replace_once(
            text,
            "/** 📬️ The scripted guest frame answering `seq` for an operation admitted under `operation`. */\nfunction frame(answer: Answer, seq: number, operation: number): AppFrameValue {\n",
            "/** 📬️ The scripted guest frame answering `seq` for an operation admitted under `operation`; `fault` encodes a status's fault. */\nfunction frame(answer: Answer, seq: number, operation: number, fault: (text: string | undefined) => number[] = bytes): AppFrameValue {\n",
            "host law: frame head",
        )
        text = replace_once(text, "completed: answer.completed, total: answer.total, ahead: 0, fault: bytes(answer.fault) };", "completed: answer.completed, total: answer.total, ahead: answer.ahead ?? 0, fault: fault(answer.fault) };", "host law: frame status")
        text = replace_once(
            text,
            "    const host = row.admittedByGuest ? DocumentArchiveLoadHost.admitted(law.firstSequence) : new DocumentArchiveLoadHost(archive);\n",
            "    const host = row.admittedByGuest ? DocumentArchiveLoadHost.admitted(law.firstSequence) : row.merging ? DocumentArchiveLoadHost.merging(archive) : new DocumentArchiveLoadHost(archive);\n",
            "host law: host",
        )
        text = replace_once(
            text,
            '      if ("LoadDocumentArchive" in step.command) expect(step.command.LoadDocumentArchive.archive).toEqual(archive);\n',
            '      if ("LoadDocumentArchive" in step.command) expect([row.name, row.merging ?? false, step.command.LoadDocumentArchive.archive]).toEqual([row.name, false, archive]);\n'
            '      if ("MergeDocumentArchive" in step.command) expect([row.name, row.merging ?? false, step.command.MergeDocumentArchive.archive]).toEqual([row.name, true, archive]);\n',
            "host law: admission",
        )
        text = replace_once(
            text,
            '      return step.outcome.kind === "fault" ? { kind: "fault", fault: text(step.outcome.fault) } : { kind: step.outcome.kind };\n',
            '      if (step.outcome.kind === "fault") return { kind: "fault", fault: text(step.outcome.fault) };\n'
            '      return step.outcome.kind === "ready" && host.ahead > 0 ? { kind: "ready", ahead: host.ahead } : { kind: step.outcome.kind };\n',
            "host law: outcome",
        )
        head = text.index('test("AppChannelClient.loadDocumentArchive replays the corpus over a scripted channel", async () => {\n')
        text = text[:head] + MERGE_LAW_CLIENT
    writes[law] = text


def wave_merge_host() -> tuple[dict, list]:
    """🔀️ Design §22.22, host half in the kernel (needs wave B). Rust (`landing`): `DocumentArchiveLoadHost::merging` admits with
    `MergeDocumentArchive`, `ahead()` answers the ready terminal's count; its corpus law follows. JSON (`serve`): three merge
    cases in `🧫️document-archive-load-host` and the schema fields `merging`, `mergeDocumentArchive`, `ahead`."""
    import json
    writes = {}
    channel = MODULES / "📡️spr/🧵️channel"
    kernel = channel / "🦀️.rs"
    text = kernel.read_text()
    if "AppCommand::MergeDocumentArchive" not in text:
        raise SystemExit("kernel merge host: wave B (`AppCommand::MergeDocumentArchive`) is not on disk yet")
    if "pub fn merging(archive: DocumentArchivePack) -> Self {" not in text:
        text = replace_once(text, "pub struct DocumentArchiveLoadHost {\n    archive: Option<DocumentArchivePack>,\n", "pub struct DocumentArchiveLoadHost {\n    archive: Option<DocumentArchivePack>,\n    merge: bool,\n    ahead: u64,\n", "kernel: host fields")
        text = replace_once(
            text,
            "        Self { archive: Some(archive), operation: None, sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }\n    }\n",
            "        Self { archive: Some(archive), merge: false, ahead: 0, operation: None, sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }\n    }\n\n"
            "    /// 🔀️ A read-back merge of the document the program already shows (design §22.22): admitted with\n"
            "    /// `AppCommand::MergeDocumentArchive`, then polled, cancelled and acknowledged exactly like a load. Its events join the\n"
            "    /// program's log and nothing is replaced; [`Self::ahead`] answers what the program then holds that the archive lacks.\n"
            "    pub fn merging(archive: DocumentArchivePack) -> Self {\n        Self { merge: true, ..Self::new(archive) }\n    }\n",
            "kernel: host new + merging",
        )
        text = replace_once(
            text,
            "        Self { archive: None, operation: Some(operation), sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }\n",
            "        Self { archive: None, merge: false, ahead: 0, operation: Some(operation), sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }\n",
            "kernel: host admitted",
        )
        text = replace_once(
            text,
            "    /// 🔢️ The admitted operation, once the admission was sent.\n    pub fn operation(&self) -> Option<u64> {\n        self.operation\n    }\n",
            "    /// 🔢️ The admitted operation, once the admission was sent.\n    pub fn operation(&self) -> Option<u64> {\n        self.operation\n    }\n\n"
            "    /// ⏭️ The events the program holds that the archive lacks, as its acknowledged `Ready` terminal counted them: what a\n"
            "    /// writer persists again for. 0 for a replacement and before the operation ended ready.\n"
            "    pub fn ahead(&self) -> u64 {\n        self.ahead\n    }\n",
            "kernel: host ahead",
        )
        text = replace_once(
            text,
            "            (None, _) => (DocumentArchiveLoadSent::Admit, AppCommand::LoadDocumentArchive { seq, archive: self.archive.take().unwrap_or_default() }),\n",
            "            (None, _) if self.merge => (DocumentArchiveLoadSent::Admit, AppCommand::MergeDocumentArchive { seq, archive: self.archive.take().unwrap_or_default() }),\n"
            "            (None, _) => (DocumentArchiveLoadSent::Admit, AppCommand::LoadDocumentArchive { seq, archive: self.archive.take().unwrap_or_default() }),\n",
            "kernel: host admit",
        )
        text = replace_once(
            text,
            "                let terminal = self.terminal.take().ok_or(DocumentArchiveLoadRefusal::Unanswered)?;\n                self.outcome = Some(match terminal.state {\n",
            "                let terminal = self.terminal.take().ok_or(DocumentArchiveLoadRefusal::Unanswered)?;\n                if terminal.state == DocumentArchiveLoadState::Ready {\n                    self.ahead = terminal.ahead;\n                }\n                self.outcome = Some(match terminal.state {\n",
            "kernel: host acknowledge",
        )
    writes[kernel] = text
    law = channel / "🧪️tests/🔬️unit/🦀️.rs"
    text = law.read_text()
    if "DocumentArchiveLoadHost::merging(" not in text:
        text = replace_once(
            text,
            "        let mut host = if admitted_by_guest { DocumentArchiveLoadHost::admitted(first) } else { DocumentArchiveLoadHost::new(DocumentArchivePack { parent_pack: vec![7], parent_spr: vec![9], members: Vec::new() }) };\n",
            "        let merging = case[\"merging\"].as_bool().unwrap_or(false);\n"
            "        let scripted = DocumentArchivePack { parent_pack: vec![7], parent_spr: vec![9], members: Vec::new() };\n"
            "        let mut host = if admitted_by_guest {\n            DocumentArchiveLoadHost::admitted(first)\n        } else if merging {\n            DocumentArchiveLoadHost::merging(scripted)\n        } else {\n            DocumentArchiveLoadHost::new(scripted)\n        };\n",
            "kernel law: host",
        )
        text = replace_once(
            text,
            "                    (\"loadDocumentArchive\", first)\n                }\n",
            "                    (\"loadDocumentArchive\", first)\n                }\n"
            "                AppCommand::MergeDocumentArchive { seq: sent, archive } => {\n"
            "                    assert_eq!((*sent, archive.parent_pack.as_slice(), archive.parent_spr.as_slice(), archive.members.len(), merging), (seq, &[7u8][..], &[9u8][..], 0, true), \"{name}: only a merge admits with the merge command, under its own sequence\");\n"
            "                    (\"mergeDocumentArchive\", first)\n                }\n",
            "kernel law: merge command",
        )
        text = replace_once(
            text,
            "                        total: answer[\"total\"].as_u64().unwrap(),\n                        ahead: 0,\n",
            "                        total: answer[\"total\"].as_u64().unwrap(),\n                        ahead: answer[\"ahead\"].as_u64().unwrap_or(0),\n",
            "kernel law: scripted ahead",
        )
        text = replace_once(
            text,
            "            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Ready) => serde_json::json!({ \"kind\": \"ready\" }),\n",
            "            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Ready) if host.ahead() == 0 => serde_json::json!({ \"kind\": \"ready\" }),\n"
            "            DocumentArchiveLoadStep::Finished(DocumentArchiveLoadOutcome::Ready) => serde_json::json!({ \"kind\": \"ready\", \"ahead\": host.ahead() }),\n",
            "kernel law: ready outcome",
        )
    writes[law] = text
    corpus = channel / "🧫️fixtures/🧫️document-archive-load-host/🔣️.json"
    text = corpus.read_text()
    if '"merging": true' not in text:
        tail = "\n    }\n  ]\n}\n"
        if not text.endswith(tail):
            raise SystemExit("load host corpus: unexpected end of file")
        text = text[: -len("\n  ]\n}\n")] + MERGE_HOST_ROWS + "\n  ]\n}\n"
        text = replace_once(
            text,
            "and a frame that answers another command is no answer.\"",
            "and a frame that answers another command is no answer. A read-back of the document the program already shows is a merge: it admits with its own command, is polled, cancelled and acknowledged like a load, and its ready terminal says how many events the program is ahead of the archive.\"",
            "load host corpus: law text",
        )
        json.loads(text)
    writes[corpus] = text
    schema = channel / "🧬️schema/🔣️document-archive-load-host/🔣️.json"
    text = schema.read_text()
    if "mergeDocumentArchive" not in text:
        text = replace_once(text, '"sends": { "enum": ["loadDocumentArchive", "pollDocumentArchiveLoad", "cancelDocumentArchiveLoad", "acknowledgeDocumentArchiveLoad"] },', '"sends": { "enum": ["loadDocumentArchive", "mergeDocumentArchive", "pollDocumentArchiveLoad", "cancelDocumentArchiveLoad", "acknowledgeDocumentArchiveLoad"] },', "load host schema: sends")
        text = replace_once(text, '            "total": { "$ref": "#/$defs/count" },\n            "fault": { "$ref": "#/$defs/fault" }\n', '            "total": { "$ref": "#/$defs/count" },\n            "ahead": { "$ref": "#/$defs/count" },\n            "fault": { "$ref": "#/$defs/fault" }\n', "load host schema: status ahead")
        text = replace_once(text, '"properties": { "kind": { "enum": ["ready", "cancelled", "unanswered"] } } },', '"properties": { "kind": { "enum": ["ready", "cancelled", "unanswered"] }, "ahead": { "$ref": "#/$defs/count" } } },', "load host schema: ready ahead")
        text = replace_once(text, '        "admittedByGuest": { "type": "boolean",', '        "merging": { "type": "boolean", "description": "The host merges the archive into the document the program already shows (design §22.22): the admission is mergeDocumentArchive, never loadDocumentArchive; a status may state `ahead`, the events the program holds that the archive lacks, and a ready outcome repeats a non-zero one." },\n        "admittedByGuest": { "type": "boolean",', "load host schema: merging")
        json.loads(text)
    writes[schema] = text
    merge_twin_writes(writes)
    return writes, []


def merge_shell_module() -> tuple[dict, list]:
    """📥️ Design §22.22, the read-back rule of a folder attachment as code the laws can drive (served TypeScript: `serve`):
    `FolderArchivePersistenceV1` (wave `persist`) and `FolderReadBackRouteV1` beside `restoreDocumentArchiveV1`, their corpora
    and bun laws, both laws in the os `test-channel-oracles` command."""
    writes, created = wave_persist()
    renderer = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document"
    stage = STAGE / "wave-merge"
    module = renderer / "🟦️.ts"
    text = writes[module]
    if "export class FolderReadBackRouteV1" not in text:
        text = replace_once(text, "/** 🏘️ Operations on one reusable background document admission. */\n", (stage / "route.ts").read_text().rstrip("\n") + "\n\n/** 🏘️ Operations on one reusable background document admission. */\n", "document admission: route anchor")
    writes[module] = text
    for relative in ["🔣️.json", "🧬️schema/🔣️.json"]:
        target = renderer / "🧫️fixtures/🧫️folder-read-back" / relative
        writes[target] = (stage / "🧫️folder-read-back" / relative).read_text()
        created.append(target)
    tests = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests"
    law = tests / "🧪️folder-read-back/🟦️.ts"
    relative = "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/"
    writes[law] = (stage / "law.ts").read_text().replace("DOCUMENT_MODULE", relative + "🟦️.ts").replace("FIXTURE_DIRECTORY", relative + "🧫️fixtures/🧫️folder-read-back/")
    created.append(law)
    package = REPO / "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript"
    script = package / "📜️script.ts"
    text = script.read_text()
    if '"🧪️folder-read-back"' not in text:
        text = replace_once(
            text,
            "the attached-document replacement law and the document-port control turn law under `bun:test`. */\n",
            "the attached-document replacement law, the document-port control turn law, the folder archive persistence law and the folder read-back law under `bun:test`. */\n",
            "os script: oracle docstring",
        )
        text = replace_once(
            text,
            '["🧪️document-archive-load-host", "🧪️attached-document-replacement", "🧪️document-port-control-turn"]',
            '["🧪️document-archive-load-host", "🧪️attached-document-replacement", "🧪️document-port-control-turn", "🧪️folder-archive-persistence", "🧪️folder-read-back"]',
            "os script: oracle list",
        )
    writes[script] = text
    project = package / "📋️project.json"
    text = project.read_text()
    if "🧪️folder-read-back" not in text:
        fixtures = "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/"
        text = replace_once(
            text,
            '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-port-control-turn/**/*",\n',
            '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-port-control-turn/**/*",\n'
            '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-archive-persistence/**/*",\n'
            '        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-read-back/**/*",\n'
            f'        "{fixtures}🟦️.ts",\n'
            f'        "{fixtures}🧫️fixtures/**/*",\n',
            "os project: oracle inputs",
        )
    writes[project] = text
    return writes, created


def wave_merge_shell() -> tuple[dict, list]:
    """📥️ Design §22.22, the React shell's folder read-back route (served TypeScript: `serve`). Needs the merge-host wave (the
    plugin handle's `mergeAppDocumentArchive`) and S5-STORE's wave AA (`documentArchiveAbsent`) on disk. On top of
    `merge_shell_module`: every open document owns a `FolderArchivePersistenceV1` and a `FolderReadBackRouteV1`; a published
    batch, a folded foreign batch and an empty folder write through the policy; the first archive read back after an attach
    is loaded and every later one merged, its port staying bound."""
    writes, created = merge_shell_module()
    twin = (REPO / "🧰️framework/🛍️products/💻️os/🟦️.ts").read_text()
    runtime = (MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx").read_text()
    if '{ readonly kind: "documentArchiveAbsent" }' not in twin:
        raise SystemExit("shell read-back: S5-STORE's wave AA (`documentArchiveAbsent`) is not on disk yet")
    if "readonly mergeAppDocumentArchive?:" not in runtime:
        raise SystemExit("shell read-back: the merge-host wave (`mergeAppDocumentArchive`) is not on disk yet")
    shell = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
    text = shell.read_text()
    if "readBack: FolderReadBackRouteV1<DocumentArchivePack>;" not in text:
        text = replace_once(text, "DocumentAttachmentLaneV1, LatestDocumentReplacementV1,", "DocumentAttachmentLaneV1, FolderArchivePersistenceV1, FolderReadBackRouteV1, LatestDocumentReplacementV1,", "shell: import")
        text = replace_once(text, "    archivePersistence: () => Promise<void>;\n", "    persistence: FolderArchivePersistenceV1;\n    readBack: FolderReadBackRouteV1<DocumentArchivePack>;\n", "shell: session fields")
        text = replace_once(
            text,
            "      } else void entry.port.receive({ runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null }, message).catch(error => failDocumentBackbone(runtimeKey, entry, error));\n",
            "      } else void entry.persistence.ingested(entry.port.receive({ runtimeKey, clientInstanceId: entry.clientInstanceId, scope: entry.scope ?? null }, message)).catch(error => failDocumentBackbone(runtimeKey, entry, error));\n",
            "shell: a folded foreign batch is persisted",
        )
        text = replace_once(
            text,
            "        void entry.archivePersistence().catch(error => failDocumentBackbone(runtimeKey, entry, error));\n",
            "        void entry.persistence.published().catch(error => failDocumentBackbone(runtimeKey, entry, error));\n",
            "shell: a published batch is persisted",
        )
        head = text.index("  /** 🗃️ Restores one archive into a program and hydrates it like a fresh load ({@link restoreDocumentArchiveV1}): the folder\n")
        end = "  }, [captureProgramEffectOwner, loadDocumentArchive, refreshHistorySnapshot]);\n"
        tail = text.index(end, head) + len(end)
        if "const restoreDocumentArchive = useCallback(" not in text[head:tail] or text[head:tail].count("useCallback(") != 1:
            raise SystemExit("shell: the restore callback changed shape")
        text = text[:head] + (
            "  /** 🚪️ What restoring an archive into a program drives ({@link restoreDocumentArchiveV1}, {@link FolderReadBackRouteV1}): the\n"
            "   * stepped load through the document's attachment lane, the program's history re-read and a full refresh of every surface\n"
            "   * it renders, through the live session when the program is the primary one. */\n"
            "  const documentArchivePorts = useCallback((plugin: PluginWasmHandle, program: ActiveSession, current: () => boolean) => {\n"
            "    const live = (): ActiveSession => {\n"
            "      const primary = sessionRef.current;\n"
            "      return primary !== null && primary.pluginId === program.pluginId && primary.instanceId === program.instanceId ? primary : program;\n"
            "    };\n"
            "    return {\n"
            "      load: (value: DocumentArchivePack) => loadDocumentArchive(plugin, program.instanceId, value, current),\n"
            "      history: () => refreshHistorySnapshot(program),\n"
            "      refresh: () => applyHostEffectsRef.current([], live(), { kind: \"full\" }, captureProgramEffectOwner(live())),\n"
            "    };\n"
            "  }, [captureProgramEffectOwner, loadDocumentArchive, refreshHistorySnapshot]);\n"
            "  /** 🗃️ Restores one archive into a program and hydrates it like a fresh load ({@link restoreDocumentArchiveV1}): a tutorial's\n"
            "   * restore replaces the program's document, re-reads its history and refreshes every surface it renders. A folder\n"
            "   * read-back goes through the document's own {@link FolderReadBackRouteV1} instead. */\n"
            "  const restoreDocumentArchive = useCallback((plugin: PluginWasmHandle, program: ActiveSession, archive: DocumentArchivePack, current: () => boolean): Promise<boolean> =>\n"
            "    restoreDocumentArchiveV1(archive, current, documentArchivePorts(plugin, program, current)), [documentArchivePorts]);\n"
        ) + text[tail:]
        text = replace_once(
            text,
            "            if (!await restoreDocumentArchive(entry.plugin, entry.session, archive, () => openDocumentSessionsRef.current.get(runtimeKey) === entry)) return;\n",
            "            const held = () => openDocumentSessionsRef.current.get(runtimeKey) === entry;\n"
            "            const merge = async (value: DocumentArchivePack) => {\n"
            "              if (!entry.plugin.mergeAppDocumentArchive) throw new Error(\"document-backbone.archive-merger-unavailable\");\n"
            "              return entry.plugin.mergeAppDocumentArchive(entry.session.instanceId, value);\n"
            "            };\n"
            "            if (!await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge })) return;\n",
            "shell: a read-back goes through the route",
        )
        text = replace_once(
            text,
            "      } else if (event.kind === \"commandOutcome\") {\n        // 🌐️ A refused or transformed batch",
            "      } else if (event.kind === \"documentArchiveAbsent\") {\n"
            "        void entry.readBack.absent().catch(error => failDocumentBackbone(runtimeKey, entry, error));\n"
            "      } else if (event.kind === \"commandOutcome\") {\n        // 🌐️ A refused or transformed batch",
            "shell: an empty folder takes the program's document",
        )
        text = replace_once(text, "receiveDocumentBackbone, restoreDocumentArchive, retireBrowserActorUi]", "documentArchivePorts, receiveDocumentBackbone, retireBrowserActorUi]", "shell: deps")
        start = "      entry.archivePersistence = latestWins(async () => {\n"
        head = text.index(start)
        tail = text.index("      });\n", head) + len("      });\n")
        body = text[head + len(start):tail]
        if body.count("\n") != 7 or "localDocumentArchive" not in body:
            raise SystemExit("shell: the archive write changed shape")
        text = text[:head] + text[tail:]
        text = replace_once(text, "archivePersistence: async () => {}, creationMount,", "persistence, readBack: new FolderReadBackRouteV1<DocumentArchivePack>(persistence), creationMount,", "shell: session construction")
        text = replace_once(
            text,
            "      const entry: OpenDocumentSession = { openingReference: { ...ref },",
            "      const persistence = new FolderArchivePersistenceV1(async () => {\n" + body + "      const entry: OpenDocumentSession = { openingReference: { ...ref },",
            "shell: the folder write",
        )
    writes[shell] = text
    return writes, created


ROUTE_LAW_BASES = {"🧪️tests/🧪️folder-reload-route/🟦️.ts": "365fa6ca63569799", "🧫️fixtures/🧫️folder-reload-route/🔣️.json": "57f35bd1b26eb229", "🧫️fixtures/🧫️folder-reload-route/🧬️schema/🔣️.json": "e5a0e9080949cbc9"}


def wave_merge_law() -> tuple[dict, list]:
    """⚖️ Design §22.22, the two-programs law (Rust law: `landing`; fixture + schema: `serve`; needs merge-guest and merge-host
    on disk): `two_programs_on_one_folder_converge_through_an_open_history_edit` joins the folder reload route law, its
    `twoPeers` block joins the route fixture and schema, and the TS oracle derives the block's expectations. The law file is
    edited in place (f16 and f6 appended their laws to it); fixture, schema and twin are replaced whole and only when they
    still are the files this wave was written against."""
    import hashlib
    writes = {}
    stage = STAGE / "wave-merge/route-law"
    staged = (stage / "🧪️tests/🧪️folder-reload-route/🦀️.rs").read_text()

    def between(start: str, end: str) -> str:
        head = staged.index(start)
        return staged[head:staged.index(end, head)]

    law = PLUGIN / "🧪️tests/🧪️folder-reload-route/🦀️.rs"
    text = law.read_text()
    if "async fn two_programs_on_one_folder_converge_through_an_open_history_edit()" not in text:
        for needed, where in [("fn begin_document_archive_merge(&mut self, operation: u64", PLUGIN / "🦀️.rs"), ("pub fn merging(archive: DocumentArchivePack) -> Self {", MODULES / "📡️spr/🧵️channel/🦀️.rs")]:
            if needed not in where.read_text():
                raise SystemExit(f"route law: {needed!r} is not on disk yet (merge-guest and merge-host land first)")
        text = replace_once(
            text,
            "//! restores that content through the kernel's own stepped load driver (`DocumentArchiveLoadHost`) and re-reads its history.\n",
            "//! restores that content through the kernel's own stepped load driver (`DocumentArchiveLoadHost`) and re-reads its history.\n"
            "//! A read-back of the document a program already shows is a MERGE (design §22.22, live fault F4): the two-programs law\n"
            "//! walks two programs on one folder through an open history edit.\n",
            "route law: head",
        )
        text = replace_once(
            text,
            "        let archive = PluginApp::document_archive(app).await.expect(\"the program archives its document\");\n"
            "        let bytes = protocol::encode_document_archive_bytes(&archive).expect(\"the archive encodes\");\n"
            "        findings.holds(bytes.len() <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES, || format!(\"{who}: the archive ({} bytes) exceeds the folder's bound\", bytes.len()));\n"
            "        folder.archive = Some(bytes);\n"
            "        folder.admitted += batches.len();\n",
            "        save_archive(findings, app, folder, who).await;\n        folder.admitted += batches.len();\n",
            "route law: publish saves through one helper",
        )
        text = replace_once(
            text,
            "/// 🔀️ Delivers the author's admitted batches to the reader,",
            between("/// 💾️ Saves `app`'s whole archive as the folder's content", "/// 🔀️ Delivers the author's admitted batches to the reader,") + "/// 🔀️ Delivers the author's admitted batches to the reader,",
            "route law: save_archive",
        )
        text = replace_once(text, (stage / "region-load.old.rs").read_text(), between("/// 🧭️ Drives one host load driver against `app`", "/// 🔁️ Detaches `app` from its folder and closes it,"), "route law: one driver for loads and merges")
        tail = staged[staged.index("\n/// 🕒️ The positions, in applied order"):]
        text = text.rstrip("\n") + "\n" + tail
    writes[law] = text
    for relative, base in ROUTE_LAW_BASES.items():
        target = PLUGIN / relative
        current = target.read_text()
        replacement = (stage / relative).read_text()
        if current != replacement and hashlib.sha256(current.encode()).hexdigest()[:16] != base:
            raise SystemExit(f"route law: {relative} is no longer the file this wave was written against; merge by hand")
        writes[target] = replacement
    return writes, []


def wave_receipt() -> tuple[dict, list]:
    """🧾️ Live fault on build B1 (served TypeScript: `serve`): attaching a fresh folder threw an unhandled
    `actor-document-control.receipt-count` out of the retirement of a binding that had failed to bind. (1) Binding module: a
    frame is a receipt when it names the receipt schema, so a damaged receipt fails under its own code; a turn says how many
    receipts it carried; `ActorDocumentBindingV1.retire` can be asked again and takes the program's stale-generation refusal
    as a retirement; `bindActorDocumentV1` settles a failed bind as itself and `releaseActorDocumentBindingV1` releases a
    predecessor the program did not let go of. (2) React plugin runtime binds through both. (3) ShellHost: a port that
    cannot be retired never replaces the failure it follows, and a sync attach that fails is told and logged instead of
    left unhandled. (4) The label `ui.sync.attachFailed`. (5) The control turn law with its corpus."""
    writes, created = {}, []
    stage = STAGE / "wave-receipt"
    module = PLUGIN / "📡️backbone/🔗️binding/🟦️.ts"
    text = module.read_text()
    if "export async function bindActorDocumentV1(" not in text:
        text = replace_once(
            text,
            "export const DOCUMENT_BACKBONE_CONTROL_MAXIMUM_BYTES = 4096;\n",
            "export const DOCUMENT_BACKBONE_CONTROL_MAXIMUM_BYTES = 4096;\n"
            "/** 🚫️ The program's refusal of a control for a generation it holds no binding of (Rust `decide_document_backbone_binding_v1`). */\n"
            'export const DOCUMENT_BACKBONE_STALE_GENERATION_CODE = "plugin.document-backbone.stale-generation";\n'
            "const DOCUMENT_BACKBONE_RECEIPT_SCHEMA_BYTES_V1 = new TextEncoder().encode(DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1);\n",
            "binding: constants",
        )
        head = text.index("/** 🧾️ One control turn's shell frames, split in their order into its document-port receipts and the frames the turn\n")
        tail = text.index("/** 🧾️ Accepts an exact successful receipt and surfaces a verified refusal code. */\n", head)
        if "export function splitDocumentBackboneControlTurnV1(" not in text[head:tail]:
            raise SystemExit("binding: the split changed shape")
        text = text[:head] + (
            "/** 🧾️ One control turn's shell frames, split in their order into its document-port receipts and the frames the turn\n"
            " * carried beside them. A control turn is a turn like any other: the program may answer it with frames of its own — the\n"
            " * status of an open history edit whose base the rebinding moved, an operation's result page — and those are routed by the\n"
            " * host like any other turn's, never counted as receipts (live fault F4: such a frame made a bind or a retire fail\n"
            " * `actor-document-control.receipt-count`, after which the document stayed listed as attached with a dead port). A frame is a\n"
            " * receipt exactly when it names the receipt schema; whether it is a sound one is its reader's to say, so a damaged receipt\n"
            " * fails the control turn under its own code and is never routed on as a frame of the program's. */\n"
            "export function splitDocumentBackboneControlTurnV1(frames: readonly Uint8Array[]): Readonly<{ receipts: readonly Uint8Array[]; unsolicited: readonly Uint8Array[] }> {\n"
            "  const receipts: Uint8Array[] = [];\n"
            "  const unsolicited: Uint8Array[] = [];\n"
            "  const schema = DOCUMENT_BACKBONE_RECEIPT_SCHEMA_BYTES_V1;\n"
            "  for (const frame of frames) {\n"
            "    let named = false;\n"
            "    for (let start = 0; !named && start + schema.length <= frame.length; start++) named = schema.every((byte, index) => frame[start + index] === byte);\n"
            "    (named ? receipts : unsolicited).push(frame);\n"
            "  }\n"
            "  return { receipts, unsolicited };\n"
            "}\n\n"
            "/** 🧾️ The one receipt a control turn owes. A turn that carried none or several fails with how many it carried (live fault on\n"
            " * build B1: a bare `receipt-count` could not say whether the program had answered nothing or twice). */\n"
            "export function soleDocumentBackboneReceiptV1(receipts: readonly Uint8Array[]): Uint8Array {\n"
            "  if (receipts.length !== 1) throw new Error(`actor-document-control.receipt-count:${receipts.length}`);\n"
            "  return receipts[0]!;\n"
            "}\n\n"
        ) + text[tail:]
        text = replace_once(
            text,
            "      retire: async () => {\n"
            "        if (this.#binding === null) return;\n"
            "        await this.#binding.catch(() => {});\n"
            '        if (this.#remote === "unsent" || this.#remote === "refused") return;\n'
            '        await this.#exchange({ ...this.#command, operation: "retire" });\n'
            '        this.#remote = "retired";\n'
            "        this.#bound = false;\n"
            "      },\n"
            "    });\n"
            "  }\n\n"
            "  async #exchange(command: DocumentBackboneControlV1): Promise<void> {\n"
            "    const receipts = await this.#ports.exchange(command);\n"
            '    if (receipts.length !== 1) throw new Error("actor-document-control.receipt-count");\n'
            "    const receipt = readDocumentBackboneReceiptV1(receipts[0]!, command);\n"
            '    if (command.operation === "bind") this.#remote = receipt.operation === "refused" ? "refused" : "bound";\n'
            '    if (receipt.operation === "refused") throw new Error(receipt.code);\n'
            "  }\n",
            "      retire: () => this.retire(),\n"
            "    });\n"
            "  }\n\n"
            "  /** 🚪️ Makes the program let go of this binding. A no-op for a binding the program never held or already let go of, one\n"
            "   * attempt at a time, and askable again after an attempt whose control turn failed — the port's own retirement asks once\n"
            "   * and keeps that answer, so a successor asks here ({@link releaseActorDocumentBindingV1}). The program's `retired`\n"
            "   * receipt and its stale-generation refusal both say what retiring is for: it holds no binding of this generation.\n"
            "   * Admission is closed before the turn, whatever the turn answers. */\n"
            "  retire(): Promise<void> {\n"
            "    this.#retiring ??= this.#retire().finally(() => { this.#retiring = null; });\n"
            "    return this.#retiring;\n"
            "  }\n\n"
            "  async #retire(): Promise<void> {\n"
            "    if (this.#binding === null) return;\n"
            "    await this.#binding.catch(() => {});\n"
            '    if (this.#remote === "unsent" || this.#remote === "refused" || this.#remote === "retired") return;\n'
            "    this.#bound = false;\n"
            '    const receipt = await this.#exchange({ ...this.#command, operation: "retire" });\n'
            '    if (receipt.operation === "refused" && receipt.code !== DOCUMENT_BACKBONE_STALE_GENERATION_CODE) throw new Error(receipt.code);\n'
            '    this.#remote = "retired";\n'
            "  }\n\n"
            "  async #exchange(command: DocumentBackboneControlV1): Promise<DocumentBackboneControlV1> {\n"
            "    return readDocumentBackboneReceiptV1(soleDocumentBackboneReceiptV1(await this.#ports.exchange(command)), command);\n"
            "  }\n",
            "binding: retire and exchange",
        )
        text = replace_once(
            text,
            '  #remote: "unsent" | "refused" | "possibly-bound" | "bound" | "retired" = "unsent";\n',
            '  #remote: "unsent" | "refused" | "possibly-bound" | "bound" | "retired" = "unsent";\n  #retiring: Promise<void> | null = null;\n',
            "binding: retiring field",
        )
        text = replace_once(
            text,
            '      this.#remote = "possibly-bound";\n      await this.#exchange(this.#command);\n',
            '      this.#remote = "possibly-bound";\n'
            "      const receipt = await this.#exchange(this.#command);\n"
            '      this.#remote = receipt.operation === "refused" ? "refused" : "bound";\n'
            '      if (receipt.operation === "refused") throw new Error(receipt.code);\n',
            "binding: bind",
        )
        text = replace_once(
            text,
            "/** 📡️ Owns bounded document messages until one captured actor and session have drained. */\n",
            "/** 🚪️ Lets a predecessor go before its successor binds. Its port's retirement is awaited whatever it answered — whoever\n"
            " * asked for it was told — and the program is asked again when it did not let go, so one failed control turn never fails\n"
            " * every later bind of the program. Rejects when the program still does not let go. */\n"
            "export async function releaseActorDocumentBindingV1(previous: ActorDocumentBindingV1): Promise<void> {\n"
            "  await previous.port.retire().catch(() => {});\n"
            "  await previous.retire();\n"
            "}\n\n"
            "/** 🔗️ Binds a fresh binding and answers its port. A bind that fails closes the port, makes the program let go of what it\n"
            " * may have bound and rejects with the BIND's own failure: a retirement that fails too is handed to `settled` beside it and\n"
            " * never thrown in its place (live fault on build B1: the retirement's `receipt-count` hid why the bind had failed, went\n"
            " * unhandled, and was kept as the answer to every later bind). `settled(null)` says the program let go, so the binding can\n"
            " * be forgotten; one it did not let go of is the caller's to keep for {@link releaseActorDocumentBindingV1}. */\n"
            "export async function bindActorDocumentV1(binding: ActorDocumentBindingV1, prepared: ((port: ActorDocumentMessagePortV1) => void) | undefined, settled: (retirement: Readonly<{ error: unknown }> | null) => void): Promise<ActorDocumentMessagePortV1> {\n"
            "  try {\n"
            "    prepared?.(binding.port);\n"
            "    await binding.bind();\n"
            "    return binding.port;\n"
            "  } catch (error) {\n"
            "    settled(await binding.port.retire().then(() => null, (retirement: unknown) => ({ error: retirement })));\n"
            "    throw error;\n"
            "  }\n"
            "}\n\n"
            "/** 📡️ Owns bounded document messages until one captured actor and session have drained. */\n",
            "binding: bind and release",
        )
    writes[module] = text
    runtime = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"
    text = runtime.read_text()
    if "bindActorDocumentV1(binding, prepared" not in text:
        text = replace_once(text, "import { ActorDocumentBindingV1, type ActorDocumentMessagePortV1, type ActorDocumentSourceV1, encodeDocumentBackboneControlV1, splitDocumentBackboneControlTurnV1 }", "import { ActorDocumentBindingV1, type ActorDocumentMessagePortV1, type ActorDocumentSourceV1, bindActorDocumentV1, encodeDocumentBackboneControlV1, releaseActorDocumentBindingV1, splitDocumentBackboneControlTurnV1 }", "plugin runtime: import")
        text = replace_once(text, "    if (previous) await previous.port.retire();\n", "    if (previous) await releaseActorDocumentBindingV1(previous);\n", "plugin runtime: release")
        text = replace_once(
            text,
            "    try {\n      prepared?.(binding.port);\n      await binding.bind();\n      return binding.port;\n    } catch (error) {\n      await binding.port.retire();\n      if (documentBindings.get(instanceId) === binding) documentBindings.delete(instanceId);\n      throw error;\n    }\n  };\n",
            "    return bindActorDocumentV1(binding, prepared, (retirement) => {\n"
            '      if (retirement !== null) console.error("[plugin-runtime] a document port that failed to bind could not be retired", binding.port.uri, retirement.error);\n'
            "      else if (documentBindings.get(instanceId) === binding) documentBindings.delete(instanceId);\n"
            "    });\n  };\n",
            "plugin runtime: bind",
        )
    writes[runtime] = text
    shell = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
    text = shell.read_text()
    if "function tellSyncAttachFailureV1(" not in text:
        text = replace_once(
            text,
            "function logUnlessRetiredV1(what: string): (error: unknown) => void {\n  return (error) => {\n    if (!isPluginInstanceRetiredV1(error)) console.error(what, error);\n  };\n}\n",
            "function logUnlessRetiredV1(what: string): (error: unknown) => void {\n  return (error) => {\n    if (!isPluginInstanceRetiredV1(error)) console.error(what, error);\n  };\n}\n\n"
            "/** 📎️ A sync attach that failed is told to the person and logged with its cause; the attach of a program retired under it\n"
            " * is neither. Nothing is saved to a target that did not attach, so the failure is never left to the console alone. */\n"
            "function tellSyncAttachFailureV1(notice: (message: string, kind?: Severity, code?: string) => void): (error: unknown) => void {\n"
            "  return (error) => {\n"
            "    if (isPluginInstanceRetiredV1(error)) return;\n"
            '    console.error("[os-shell] sync attach failed", error);\n'
            '    notice(shellLabel("ui.sync.attachFailed"), "error", "sync.attach.failed");\n'
            "  };\n"
            "}\n",
            "shell: attach failure teller",
        )
        text = replace_once(
            text,
            "        await port?.retire();\n        if (port !== undefined && ports?.get(instanceId) === port) ports.delete(instanceId);\n",
            '        await port?.retire().catch(logUnlessRetiredV1("[os-shell] a document port could not be retired"));\n        if (port !== undefined && ports?.get(instanceId) === port) ports.delete(instanceId);\n',
            "shell: lane detach is total",
        )
        text = replace_once(text, "      const candidate = preparation.port;\n      await candidate?.retire();\n", "      const candidate = preparation.port;\n      await candidate?.retire().catch(() => {});\n", "shell: a failed bind stays the failure")
        text = replace_once(text, "            void openSyncTarget(target);\n            return applied();\n", "            void openSyncTarget(target).catch(tellSyncAttachFailureV1(showTransientNoticeRef.current));\n            return applied();\n", "shell: attach is told")
        text = replace_once(
            text,
            '.catch(logUnlessRetiredV1("[os-shell] folder reconnect failed")).finally(() => setFolderReconnectBusy(false));',
            ".catch(tellSyncAttachFailureV1(showTransientNoticeRef.current)).finally(() => setFolderReconnectBusy(false));",
            "shell: reconnect is told",
        )
    writes[shell] = text
    ui = REPO / "🧰️framework/🔨️modules/🖱️ui"
    labels = ui / "🎯️targets/⚛️react/🌐️i18n/🟦️.ts"
    text = labels.read_text()
    if "attachFailed:" not in text:
        text = replace_once(
            text,
            '          documentUnidentified: { label: { normal: "Dieses Programm hat kein Dokument zum Verbinden",',
            '          attachFailed: { label: { normal: "Das Dokument konnte nicht verbunden werden", beginner: "Das Dokument konnte nicht mit diesem Ordner, dieser Datei oder diesem Hub verbunden werden. Dort wird nichts gespeichert, bis das Verbinden gelingt." } },\n'
            '          documentUnidentified: { label: { normal: "Dieses Programm hat kein Dokument zum Verbinden",',
            "labels: de",
        )
        text = replace_once(
            text,
            '          documentUnidentified: { label: { normal: "This program has no document to attach",',
            '          attachFailed: { label: { normal: "The document could not be attached", beginner: "The document could not be attached to this folder, file or hub. Nothing is saved there until attaching works." } },\n'
            '          documentUnidentified: { label: { normal: "This program has no document to attach",',
            "labels: en",
        )
    writes[labels] = text
    contract = ui / "🧱️elements/📚️I18n/🟦️.tsx"
    text = contract.read_text()
    if "readonly attachFailed: UiLabelValue;" not in text:
        text = replace_once(
            text,
            "      /** 🪪️ The focused program has no document of its own, so there is nothing to attach to a folder, a file or a hub. */\n",
            "      /** 📎️ Attaching the document to a folder, a file or a hub failed; nothing is saved there. */\n      readonly attachFailed: UiLabelValue;\n"
            "      /** 🪪️ The focused program has no document of its own, so there is nothing to attach to a folder, a file or a hub. */\n",
            "labels: contract",
        )
    writes[contract] = text
    tests = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-port-control-turn"
    writes[tests / "🟦️.ts"] = (stage / "law.ts").read_text()
    for name, source in [("🧫️fixtures/🔣️.json", "corpus.json"), ("🧫️fixtures/🧬️schema/🔣️.json", "schema.json")]:
        writes[tests / name] = (stage / source).read_text()
        created.append(tests / name)
    return writes, created


def wave_golden() -> tuple[dict, list]:
    """🥇️ The document-port control maps' wire bytes, pinned for both languages (fixture: `serve`; the Rust law is in wave `f9`). The
    binding fixture pinned the control VALUES only, so a drift between the Rust pack wire and the TypeScript closed grammar
    could pass every law while a live bind found no receipt it could read. `codec.control` holds each map with its bytes; the
    Rust unit law encodes and decodes them, the control turn law does the same through the TypeScript codec."""
    import json
    writes = {}
    binding = PLUGIN / "📡️backbone/🔗️binding"
    fixture = binding / "🧫️fixtures/🔣️.json"
    text = fixture.read_text()
    value = json.loads(text)
    if "control" not in value["codec"]:
        rows = json.loads((STAGE / "wave-receipt/golden.json").read_text())
        anchor = '\n  "codec": {\n'
        if text.count(anchor) != 1:
            raise SystemExit("binding fixture: the codec block changed shape")
        lines = ",\n".join("      " + json.dumps(row, ensure_ascii=False) for row in rows)
        text = text.replace(anchor, anchor + '    "control": [\n' + lines + "\n    ],\n")
        if json.loads(text)["codec"]["control"] != rows:
            raise SystemExit("binding fixture: the control rows did not land as written")
    writes[fixture] = text
    suite = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-port-control-turn/🟦️.ts"
    text = suite.read_text()
    if "the control maps are the golden wire bytes" not in text:
        text = replace_once(
            text,
            " * bind, and fast-check holds over arbitrary answers and steps that every step settles and no rejection is left unhandled. */\n",
            " * bind, and fast-check holds over arbitrary answers and steps that every step settles and no rejection is left unhandled.\n"
            " * The control maps' wire bytes are the binding fixture's `codec.control` rows, which the Rust unit law pins too. */\n",
            "control turn law: doc",
        )
        text = replace_once(text, "import { ActorDocumentBindingV1, DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1, DOCUMENT_BACKBONE_STALE_GENERATION_CODE, bindActorDocumentV1, encodeDocumentBackboneControlV1,", "import { ActorDocumentBindingV1, DOCUMENT_BACKBONE_RECEIPT_SCHEMA_V1, DOCUMENT_BACKBONE_STALE_GENERATION_CODE, bindActorDocumentV1, decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1,", "control turn law: import")
        text = text.rstrip("\n") + "\n\n" + (STAGE / "wave-receipt/golden-law.ts").read_text()
    writes[suite] = text
    return writes, []


def wave_f9() -> tuple[dict, list]:
    """🔤️ Live fault F9 (Rust, `landing`, apply only; needs wave `golden` on disk): the document-port control codec owns its
    canonical form. The pack encoder stopped ordering an object's members (a peer's deliberate change, saved between builds
    B0 and B1), so the guest's receipts went out in struct order and the TypeScript closed grammar refused them
    `actor-document-control.noncanonical` — no document port could be bound. `canonical_control_bytes` orders the control
    map's members by key bytes for both encoders, and both readers compare against that form (their old comparison had
    become vacuous for member order). The unit laws pin the golden wire bytes and the order."""
    import json
    binding = PLUGIN / "📡️backbone/🔗️binding"
    if "control" not in json.loads((binding / "🧫️fixtures/🔣️.json").read_text())["codec"]:
        raise SystemExit("f9: wave `golden` (the fixture's `codec.control` rows) is not on disk yet")
    writes = {}
    codec = binding / "🦀️.rs"
    text = codec.read_text()
    if "fn canonical_control_bytes(" not in text:
        text = replace_once(
            text,
            "impl DocumentBackboneBindingCommandV1 {\n    pub fn encode(&self) -> Result<Vec<u8>, String> {\n",
            "/// 🔤️ The control codec's canonical form: the flat control map with its members ordered by their key's bytes — the order\n"
            "/// the TypeScript closed grammar (`🟦️.ts`) writes and demands. The pack encoder writes an object's members as it is\n"
            "/// given them, so the control codec orders them itself: both encoders go through here, and both readers compare what\n"
            "/// they were handed against this form (golden bytes: `🧫️fixtures/🔣️.json` `codec.control`).\n"
            "fn canonical_control_bytes(value: semio_framework_value::DslValue) -> Vec<u8> {\n"
            "    match value {\n"
            "        semio_framework_value::DslValue::Object(mut members) => {\n"
            "            members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));\n"
            "            store::pack_rt::encode_wire_value(&semio_framework_value::DslValue::Object(members))\n"
            "        }\n"
            "        other => store::pack_rt::encode_wire_value(&other),\n"
            "    }\n"
            "}\n\n"
            "impl DocumentBackboneBindingCommandV1 {\n    pub fn encode(&self) -> Result<Vec<u8>, String> {\n",
            "f9: canonical form",
        )
        head = text.index("        let bytes = store::pack_rt::encode_wire_value(\n            &DocumentBackboneBindingWireV1 {")
        tail = text.index("        );\n        if bytes.len() > DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES {", head)
        inner = text[head + len("        let bytes = store::pack_rt::encode_wire_value(\n"):tail]
        if inner.count("\n") != 1 or not inner.startswith("            &DocumentBackboneBindingWireV1 {") or not inner.endswith(".to_value(),\n"):
            raise SystemExit("f9: the command encoder changed shape")
        text = text[:head] + "        let bytes = canonical_control_bytes(" + inner.strip()[1:-1] + ");\n" + text[tail + len("        );\n"):]
        text = replace_once(text, "        store::pack_rt::encode_wire_value(\n            &DocumentBackboneBindingReceiptWireV1 {\n", "        canonical_control_bytes(\n            DocumentBackboneBindingReceiptWireV1 {\n", "f9: receipt encoder")
        text = replace_once(
            text,
            '    if store::pack_rt::encode_wire_value(&value) != payload {\n        return Err("plugin.document-backbone.binding-noncanonical".into());\n',
            '    if canonical_control_bytes(value.clone()) != payload {\n        return Err("plugin.document-backbone.binding-noncanonical".into());\n',
            "f9: command reader",
        )
        text = replace_once(
            text,
            '    if store::pack_rt::encode_wire_value(&value) != payload {\n        return Err("plugin.document-backbone.receipt-noncanonical".into());\n',
            '    if canonical_control_bytes(value.clone()) != payload {\n        return Err("plugin.document-backbone.receipt-noncanonical".into());\n',
            "f9: receipt reader",
        )
    writes[codec] = text
    law = binding / "🧪️tests/🔬️unit-standalone/🦀️.rs"
    text = law.read_text()
    if "fn document_backbone_control_maps_are_the_golden_wire_bytes()" not in text:
        text = text.rstrip("\n") + "\n\n" + (STAGE / "wave-receipt/golden.rs").read_text()
    writes[law] = text
    return writes, []


def wave_attach_told() -> tuple[dict, list]:
    """📎️ Live fault F9, what the person sees (served TypeScript: `serve`): the sync card's Attach handed `openSyncTarget`
    straight to `onAttach`, which drops the promise — a failed attach was an unhandled rejection and told nobody. Every
    gesture (the card, the attach action, the reconnect offer) now goes through `attachSyncTarget`, which tells and logs."""
    shell = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
    text = shell.read_text()
    if "const attachSyncTarget = useCallback(" not in text:
        text = replace_once(
            text,
            "    [loadedPlugins, openDocument, rememberLocalFolder, resolveSyncTargetSession],\n  );\n",
            "    [loadedPlugins, openDocument, rememberLocalFolder, resolveSyncTargetSession],\n  );\n"
            "  /** 📎️ Attaches the focused program's document to a sync target as the person's gesture. Every gesture goes through here —\n"
            "   * the card's Attach, the attach action, the reconnect offer — so a failure is told and logged\n"
            "   * ({@link tellSyncAttachFailureV1}) and never dropped as an unhandled rejection (live fault F9: the card's `onAttach`\n"
            "   * dropped the promise, so a document that did not attach said nothing). */\n"
            "  const attachSyncTarget = useCallback((target: SyncAttachTargetV1): Promise<void> => openSyncTarget(target).catch(tellSyncAttachFailureV1(showTransientNoticeRef.current)), [openSyncTarget]);\n",
            "attach told: entry point",
        )
        text = replace_once(
            text,
            '    void openSyncTarget({ kind: "folder", path: binding.folder.path }).catch(tellSyncAttachFailureV1(showTransientNoticeRef.current)).finally(() => setFolderReconnectBusy(false));\n  }, [openSyncTarget]);\n',
            '    void attachSyncTarget({ kind: "folder", path: binding.folder.path }).finally(() => setFolderReconnectBusy(false));\n  }, [attachSyncTarget]);\n',
            "attach told: reconnect",
        )
        text = replace_once(text, "            void openSyncTarget(target).catch(tellSyncAttachFailureV1(showTransientNoticeRef.current));\n            return applied();\n", "            void attachSyncTarget(target);\n            return applied();\n", "attach told: action")
        text = replace_once(text, "      openSyncTarget,\n", "      attachSyncTarget,\n", "attach told: action deps")
        text = replace_once(text, "onAttach={openSyncTarget}", "onAttach={attachSyncTarget}", "attach told: card")
        text = replace_once(text, "  }, [openSyncTarget, browseSyncBackbonePath,", "  }, [attachSyncTarget, browseSyncBackbonePath,", "attach told: card deps")
        if text.count("openSyncTarget") != 3:
            raise SystemExit(f"attach told: {text.count('openSyncTarget')} references to openSyncTarget remain, expected its definition and its one caller with its dependency")
    return {shell: text}, []


def wave_f9_test() -> tuple[dict, list]:
    """🔤️ F9, test only (`landing`, apply only): the binding's reducer law wrote its instance-zero command through the pack
    encoder in struct order, which the control codec refuses since wave `f9` (`binding-noncanonical`). It now writes the
    command through the codec's own encoder, as every host does."""
    law = PLUGIN / "📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs"
    text = law.read_text()
    if "let zero_wire = " in text:
        text = replace_once(
            text,
            '    let zero_wire = DocumentBackboneBindingWireV1 { schema: DOCUMENT_BACKBONE_BINDING_SCHEMA_V1.into(), operation: "bind".into(), instance_id: 0, binding_generation: 1, uri: "actor://v1:0:3:space-amap".into() };\n'
            "    let zero_payload = store::pack_rt::encode_wire_value(&zero_wire.to_value());\n",
            '    let zero_payload = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Bind, instance_id: 0, binding_generation: 1, uri: "actor://v1:0:3:space-amap".into() }.encode().expect("canonical instance zero command encodes");\n',
            "f9 test: the instance-zero command",
        )
    return {law: text}, []


def wave_f13() -> tuple[dict, list]:
    """🛑️ Live fault F13 + F5 remainder (served TypeScript: `serve`; needs wave `merge-shell` on disk). A load the person
    cancelled was announced as a failure: `[role=alert]` "Document restore failed: AppChannelClient.loadDocumentArchive(…):
    cancelled" and a console error. (1) `FolderReadBackRouteV1.archive` answers how a read-back ended — `held`, `superseded`
    or `cancelled` — and only a real failure rejects; corpus + law follow. (2) A refused or faulted load rejects with
    `DocumentArchiveFaultError`, which carries the program's own fault. (3) ShellHost: a cancelled read-back raises the polite
    `load-cancelled` notice the file-open path already uses, no alert and no console error; a failed one is told by the
    fault's own notice, else the `load-failed` label — never the failure's engineering text; and a read-back load feeds the
    shell's `load` status like a file-open load does (it fed none, so the status outside the History panel never showed)."""
    writes, created = {}, []
    stage = STAGE / "wave-merge"
    renderer = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost"
    module = renderer / "🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts"
    text = module.read_text()
    if "readonly detach: (failure" in text:
        return {}, []
    if "export type FolderReadBackOutcomeV1" not in text:
        head = text.index("/** 🔀️ What one read-back merge answered:")
        tail = text.index("/** 🏘️ Operations on one reusable background document admission. */\n", head)
        if "export class FolderReadBackRouteV1" not in text[head:tail] or text[head:tail].count("export ") != 3:
            raise SystemExit("f13: the read-back route changed shape")
        text = text[:head] + (stage / "route.ts").read_text().rstrip("\n") + "\n\n" + text[tail:]
    writes[module] = text
    for relative in ["🔣️.json", "🧬️schema/🔣️.json"]:
        writes[renderer / "🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🧫️folder-read-back" / relative] = (stage / "🧫️folder-read-back" / relative).read_text()
    relative = "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/"
    writes[REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-read-back/🟦️.ts"] = (stage / "law.ts").read_text().replace("DOCUMENT_MODULE", relative + "🟦️.ts").replace("FIXTURE_DIRECTORY", relative + "🧫️fixtures/🧫️folder-read-back/")
    twin = REPO / "🧰️framework/🛍️products/💻️os/🟦️.ts"
    text = twin.read_text()
    if "export class DocumentArchiveFaultError" not in text:
        text = replace_once(
            text,
            "/** 🚫️ The guest's refusals of a read-back merge that leave a load as the way on:",
            "/** 🧯️ A whole-document load or merge the program refused or faulted. `fault` is the program's own fault when its bytes decode\n"
            " * — its code names the notice a person is told — and `message` keeps the engineering text for a log, never for a person. */\n"
            "export class DocumentArchiveFaultError extends Error {\n"
            "  readonly fault: Fault | null;\n\n"
            "  constructor(message: string, fault: Fault | null) {\n"
            "    super(message);\n"
            '    this.name = "DocumentArchiveFaultError";\n'
            "    this.fault = fault;\n"
            "  }\n"
            "}\n\n"
            "/** 🚫️ The guest's refusals of a read-back merge that leave a load as the way on:",
            "os twin: fault error",
        )
        text = replace_once(
            text,
            "        throw new Error(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`);\n",
            "        throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(step.outcome.fault, decodePackValue)}`, decodeFaultFromWire(step.outcome.fault, decodePackValue));\n",
            "os twin: fault outcome",
        )
        text = replace_once(
            text,
            '      if (answer.kind === "refused") throw new Error(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(answer.fault, decodePackValue)}`);\n',
            '      if (answer.kind === "refused") throw new DocumentArchiveFaultError(`AppChannelClient.${name}(${this.appId}): ${faultDisplayMessage(answer.fault, decodePackValue)}`, decodeFaultFromWire(answer.fault, decodePackValue));\n',
            "os twin: refusal",
        )
        text = replace_once(
            text,
            "   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason\n   * (an `AbortError` when the guest cancelled on its own); a refusal, a reply that answers nothing and every other fault\n   * reject with the guest's own fault text. */\n",
            "   * or with the guest's fault code when it is one of `unmergeable`. A cancelled operation rejects with the signal's reason\n   * (an `AbortError` when the guest cancelled on its own); a refusal and every other fault reject with a\n   * {@link DocumentArchiveFaultError} carrying the guest's own fault, a reply that answers nothing with a plain error. */\n",
            "os twin: drive doc",
        )
    writes[twin] = text
    law = REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️document-archive-load-host/🟦️.ts"
    text = law.read_text()
    if "DocumentArchiveFaultError" not in text:
        text = replace_once(text, "import { AppChannelClient, AppChannelRequestSequence, DocumentArchiveLoadHost, decodeAppCommand,", "import { AppChannelClient, AppChannelRequestSequence, DocumentArchiveFaultError, DocumentArchiveLoadHost, decodeAppCommand,", "host law: import")
        text = replace_once(
            text,
            '  const failure = await scripted(broken, packedFault).client.mergeDocumentArchive(archive).then(() => "resolved", (error: unknown) => (error instanceof Error ? error.message : String(error)));\n  expect(failure).toContain("plugin.document-load.history-invalid");\n',
            '  const failure = await scripted(broken, packedFault).client.mergeDocumentArchive(archive).then(() => null, (error: unknown) => error);\n'
            '  expect(failure instanceof DocumentArchiveFaultError ? [failure.fault?.code, failure.message.includes("plugin.document-load.history-invalid")] : failure).toEqual(["plugin.document-load.history-invalid", true]);\n'
            '  const unread = await scripted(broken, bytes).client.loadDocumentArchive(archive).then(() => null, (error: unknown) => error);\n'
            '  expect(unread instanceof DocumentArchiveFaultError ? [unread.fault, unread.message.includes("plugin.document-load.history-invalid")] : unread).toEqual([null, true]);\n',
            "host law: the fault a failure carries",
        )
    writes[law] = text
    shell = renderer / "🟦️.tsx"
    text = shell.read_text()
    if "cancelled: documentLoadCancelledV1 }" not in text:
        for needed in ["entry.readBack.archive(archive, held,", "  documentLoadCancelledV1,\n", "  documentTransferNoticeTextV1,\n", "  programHistoryProjectionsWithLoadV1,\n"]:
            if needed not in text:
                raise SystemExit(f"f13: {needed!r} is not in ShellHost")
        text = replace_once(text, "  decodeDocumentArchiveBytes,\n", "  decodeDocumentArchiveBytes,\n  DocumentArchiveFaultError,\n", "shell: import fault error")
        if "  appFaultNoticeV1,\n" not in text:
            text = replace_once(text, "  documentLoadCancelledV1,\n", "  appFaultNoticeV1,\n  documentLoadCancelledV1,\n", "shell: import fault notice")
        text = replace_once(
            text,
            "  /** 🚪️ What restoring an archive into a program drives ({@link restoreDocumentArchiveV1}, {@link FolderReadBackRouteV1}): the\n"
            "   * stepped load through the document's attachment lane, the program's history re-read and a full refresh of every surface\n"
            "   * it renders, through the live session when the program is the primary one. */\n",
            "  /** 🚪️ What restoring an archive into a program drives ({@link restoreDocumentArchiveV1}, {@link FolderReadBackRouteV1}): the\n"
            "   * stepped load through the document's attachment lane — every polled status is the program's `load` status outside the\n"
            "   * History panel until the load ends, as for a file-open load — the program's history re-read and a full refresh of\n"
            "   * every surface it renders, through the live session when the program is the primary one. */\n",
            "shell: ports doc",
        )
        text = replace_once(
            text,
            "    return {\n      load: (value: DocumentArchivePack) => loadDocumentArchive(plugin, program.instanceId, value, current),\n",
            "    const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(program), load));\n"
            "    return {\n      load: (value: DocumentArchivePack) => loadDocumentArchive(plugin, program.instanceId, value, current, { progress: showLoad }).finally(() => showLoad(null)),\n",
            "shell: a restore load shows its status",
        )
        text = replace_once(text, "  }, [captureProgramEffectOwner, loadDocumentArchive, refreshHistorySnapshot]);\n  /** 🗃️ Restores one archive into a program and hydrates it like a fresh load", "  }, [captureProgramEffectOwner, historyStore, loadDocumentArchive, refreshHistorySnapshot]);\n  /** 🗃️ Restores one archive into a program and hydrates it like a fresh load", "shell: ports deps")
        text = replace_once(
            text,
            "            if (!await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge })) return;\n",
            "            const outcome = await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge, cancelled: documentLoadCancelledV1 });\n"
            '            if (outcome === "cancelled" && held()) documentTransferRef.current.notifyDocumentTransfer("load-cancelled", resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id, "info");\n'
            '            if (outcome !== "held") return;\n',
            "shell: a cancelled read-back is no failure",
        )
        text = replace_once(
            text,
            "              code: \"invalid-bootstrap\",\n              message: (replacementError instanceof Error ? replacementError.message : String(replacementError)).slice(0, 4_096),\n",
            "              code: \"invalid-bootstrap\",\n"
            "              message: (replacementError instanceof DocumentArchiveFaultError && replacementError.fault !== null ? appFaultNoticeV1(replacementError.fault, entry.session.app, uiTerminologyRef.current)?.text : undefined)\n"
            '                ?? documentTransferNoticeTextV1("load-failed", resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id, uiLocaleRef.current),\n',
            "shell: a failed read-back is told by a label",
        )
    writes[shell] = text
    return writes, created


def wave_detach() -> tuple[dict, list]:
    """🔌️ The coordinator's decision on the hazard found with F13 (served TypeScript: `serve`; needs wave `f13` on disk): a
    FIRST archive that is not loaded — cancelled or failed — DETACHES the folder and keeps it remembered, so nothing of the
    program is written over an archive it did not take and the reconnect offer returns. The route asks its `detach` port and
    ends `detached`; an adopted folder stays attached after a cancelled later read-back. Corpus rows, law, the polite notice
    `shell.documentTransfer.folder-detached` (en / de) and the ShellHost port follow."""
    writes = {}
    stage = STAGE / "wave-merge"
    renderer = MODULES / "📺️renderer/🧑‍🎨engine/🧱️elements"
    module = renderer / "🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts"
    text = module.read_text()
    if "readonly detach: (failure" not in text:
        if "export type FolderReadBackOutcomeV1" not in text:
            raise SystemExit("detach: wave `f13` is not on disk")
        head = text.index("/** 🔀️ What one read-back merge answered:")
        tail = text.index("/** 🏘️ Operations on one reusable background document admission. */\n", head)
        if text[head:tail].count("export ") != 4:
            raise SystemExit("detach: the read-back route changed shape")
        text = text[:head] + (stage / "route.ts").read_text().rstrip("\n") + "\n\n" + text[tail:]
    writes[module] = text
    for relative in ["🔣️.json", "🧬️schema/🔣️.json"]:
        writes[renderer / "🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🧫️folder-read-back" / relative] = (stage / "🧫️folder-read-back" / relative).read_text()
    relative = "../../🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/"
    writes[REPO / "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️folder-read-back/🟦️.ts"] = (stage / "law.ts").read_text().replace("DOCUMENT_MODULE", relative + "🟦️.ts").replace("FIXTURE_DIRECTORY", relative + "🧫️fixtures/🧫️folder-read-back/")
    helpers = renderer / "🛠️ShellHelpers/🟦️.tsx"
    text = helpers.read_text()
    if '"folder-detached"' not in text:
        text = replace_once(text, '  | "load-cancelled"\n  | "load-failed";\n', '  | "load-cancelled"\n  | "load-failed"\n  | "folder-detached";\n', "helpers: notice kind")
        head = text.index('  "load-failed": { en: "“{file}” could not be loaded; the previous document is unchanged."')
        tail = text.index("\n", head) + 1
        text = text[:tail] + (
            '  "folder-detached": {\n'
            '    en: "The folder was detached because its document was not loaded; nothing is saved to it. Reconnect the folder to load its document.",\n'
            '    de: "Der Ordner wurde getrennt, weil sein Dokument nicht geladen wurde; es wird nichts darin gespeichert. Verbinde den Ordner erneut, um sein Dokument zu laden.",\n'
            "  },\n"
        ) + text[tail:]
    writes[helpers] = text
    shell = renderer / "🏛️ShellHost/🟦️.tsx"
    text = shell.read_text()
    if "cancelled: documentLoadCancelledV1, detach }" not in text:
        text = replace_once(
            text,
            "            const outcome = await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge, cancelled: documentLoadCancelledV1 });\n"
            '            if (outcome === "cancelled" && held()) documentTransferRef.current.notifyDocumentTransfer("load-cancelled", resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id, "info");\n',
            "            const file = resolveManifestLabel(entry.session.app.label, uiTerminologyRef.current, uiLocaleRef.current) || entry.session.app.id;\n"
            "            const detach = async (failure: Readonly<{ error: unknown }> | null): Promise<void> => {\n"
            "              if (!held()) return;\n"
            "              if (failure !== null) {\n"
            '                console.error("[os-shell] a folder read-back could not be restored", message.documentId, failure.error instanceof Error ? failure.error.message : String(failure.error));\n'
            "                const told = failure.error instanceof DocumentArchiveFaultError && failure.error.fault !== null ? appFaultNoticeV1(failure.error.fault, entry.session.app, uiTerminologyRef.current) : null;\n"
            '                if (told === null) documentTransferRef.current.notifyDocumentTransfer("load-failed", file, "error");\n'
            '                else showTransientNoticeRef.current(told.text, "error", told.code);\n'
            "              }\n"
            '              documentTransferRef.current.notifyDocumentTransfer("folder-detached", file, "info");\n'
            "              closeDocumentRef.current(runtimeKey, entry.clientInstanceId);\n"
            "              if (shellStateRef.current.sync.syncBackboneUri === `actor://${runtimeKey}`) {\n"
            '                dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });\n'
            '                dispatch({ type: "SET_SYNC_CARD_KIND", value: null });\n'
            "              }\n"
            "            };\n"
            "            const outcome = await entry.readBack.archive(archive, held, { ...documentArchivePorts(entry.plugin, entry.session, held), merge, cancelled: documentLoadCancelledV1, detach });\n"
            '            if (outcome === "cancelled" && held()) documentTransferRef.current.notifyDocumentTransfer("load-cancelled", file, "info");\n',
            "shell: a first archive that is not taken detaches the folder",
        )
    writes[shell] = text
    return writes, []


WAVES = {"p1": wave_p1, "f1": wave_f1, "f12": wave_f12, "persist": wave_persist, "attach": wave_attach, "f16": wave_f16, "f6": wave_f6, "merge-guest": wave_merge_guest, "checkin": wave_checkin, "merge-host": wave_merge_host, "merge-shell": wave_merge_shell, "merge-law": wave_merge_law, "receipt": wave_receipt, "golden": wave_golden, "f9": wave_f9, "attach-told": wave_attach_told, "f9-test": wave_f9_test, "f13": wave_f13, "detach": wave_detach}


def main() -> None:
    if len(sys.argv) != 3 or sys.argv[1] not in WAVES or sys.argv[2] not in ("check", "land", "restore"):
        raise SystemExit(__doc__)
    wave, verb = sys.argv[1], sys.argv[2]
    if verb != "check" and (TICKET / "🗑️generated" / "coord" / "activation.flag").exists():
        raise SystemExit(f"{wave}: REFUSED — an activation is running (`🗑️generated/coord/activation.flag`, rule 68): no save until it ends")
    backup = STAGE / f"pre-{wave}"
    if verb == "restore":
        manifest = backup / "created.txt"
        for line in manifest.read_text().splitlines() if manifest.exists() else []:
            pathlib.Path(line).unlink(missing_ok=True)
        for saved in sorted(path for path in backup.rglob("*") if path.is_file() and path.name != "created.txt"):
            target = REPO / saved.relative_to(backup)
            shutil.copyfile(saved, target)
            print(f"restored {target.relative_to(REPO)}")
        return
    writes, created = WAVES[wave]()
    changed = {path: text for path, text in writes.items() if not path.exists() or path.read_text() != text}
    for path in changed:
        print(f"{'would write' if verb == 'check' else 'write'} {path.relative_to(REPO)} ({len(changed[path].splitlines())} lines)")
    if verb == "check":
        print(f"{wave}: {len(changed)} file(s) to write, anchors hold")
        return
    backup.mkdir(parents=True, exist_ok=True)
    (backup / "created.txt").write_text("\n".join(str(path) for path in created if not path.exists()))
    for path, text in changed.items():
        if path.exists():
            saved = backup / path.relative_to(REPO)
            saved.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, saved)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
    print(f"{wave}: landed {len(changed)} file(s); copies under {backup.relative_to(TICKET)}")


main()
