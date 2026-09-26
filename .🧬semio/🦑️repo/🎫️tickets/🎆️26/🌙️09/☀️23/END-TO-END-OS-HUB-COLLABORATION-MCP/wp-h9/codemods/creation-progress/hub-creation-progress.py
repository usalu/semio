#!/usr/bin/env python3
"""📈️ Hub half of the creation-progress patch (ticket 26/09/23 H9 session 12).

The os-kernel half (`os-kernel-and-creation.diff`, apply with `patch -p1`) adds
`SpaceArtifactCreationProgressV1` to the directory schema; this script wires the hub:
the creation control records the authority's progress, the status route attaches it for
running phases, and `AuthorityProgressStage::GuestCompiling` exists for the catalog's
compile step to report.

usage: hub-creation-progress.py [--reverse] [--dry-run]
"""
import pathlib
import sys

ROOT = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / ".git").exists())
BOOTSTRAP = ROOT / "🌎️hub/🏗️bootstrap/🦀️.rs"
AUTHORITY = ROOT / "🌎️hub/🗿️artifact-authority/🦀️.rs"
CONFIG = ROOT / "🌎️hub/🧪️tests/🎚️config/🟦️.ts"
ORACLE = ROOT / "🌎️hub/🧪️tests/🌱️creation-progress/🟦️.ts"
ORACLE_SOURCE = pathlib.Path(__file__).resolve().parent / "creation-progress-oracle.ts"
P = "directory::os_directory::schema::space_artifact_creation"

EDITS = [
    (CONFIG,
     "`🛡️access-policy` and `🚧️hostile-input` oracles.",
     "`🛡️access-policy`, `🚧️hostile-input` and `🌱️creation-progress` oracles."),
    (CONFIG,
     'resolve(dir, "../../🧪️tests/🚧️hostile-input/🟦️.ts")]',
     'resolve(dir, "../../🧪️tests/🚧️hostile-input/🟦️.ts"), resolve(dir, "../../🧪️tests/🌱️creation-progress/🟦️.ts")]'),
    (AUTHORITY,
     "    CatalogLoading,\n    GuestCodecExecuting,\n",
     "    CatalogLoading,\n    GuestCompiling,\n    GuestCodecExecuting,\n"),
    (BOOTSTRAP,
     "    fault: Mutex<Option<String>>,\n}\n\n#[cfg(feature = \"native-artifact-execution\")]\nimpl ArtifactCreationHttpControlV1 {\n",
     f"    fault: Mutex<Option<String>>,\n    progress: Mutex<Option<{P}::SpaceArtifactCreationProgressV1>>,\n}}\n\n#[cfg(feature = \"native-artifact-execution\")]\nimpl ArtifactCreationHttpControlV1 {{\n"),
    (BOOTSTRAP,
     "shutdown_cancelled: None, fault: Mutex::new(None) }",
     "shutdown_cancelled: None, fault: Mutex::new(None), progress: Mutex::new(None) }"),
    (BOOTSTRAP,
     "        self.fault.lock().ok().and_then(|mut held| held.take())\n    }\n}\n\n#[cfg(feature = \"native-artifact-execution\")]\nimpl AuthorityOperationControl for ArtifactCreationHttpControlV1 {\n",
     f"""        self.fault.lock().ok().and_then(|mut held| held.take())
    }}

    /// 📈️ Where this running creation is: `queued` until its operation reports anything.
    fn progress(&self) -> {P}::SpaceArtifactCreationProgressV1 {{
        use {P}::{{SpaceArtifactCreationProgressV1, SpaceArtifactCreationStageV1}};
        self.progress.lock().ok().and_then(|held| *held).unwrap_or(SpaceArtifactCreationProgressV1 {{ stage: SpaceArtifactCreationStageV1::Queued, completed_units: 0, total_units: 1 }})
    }}
}}

/// 🧭️ The creation stage an authority progress stage belongs to; a stage that says nothing about
/// where the creation is keeps the stage already reported.
#[cfg(feature = "native-artifact-execution")]
fn artifact_creation_stage(stage: semio_hub::artifact_authority::AuthorityProgressStage) -> Option<{P}::SpaceArtifactCreationStageV1> {{
    use semio_hub::artifact_authority::AuthorityProgressStage;
    use {P}::SpaceArtifactCreationStageV1;
    match stage {{
        AuthorityProgressStage::GuestCompiling => Some(SpaceArtifactCreationStageV1::CompilingGuest),
        AuthorityProgressStage::GuestCodecExecuting => Some(SpaceArtifactCreationStageV1::Genesis),
        AuthorityProgressStage::CasChunkStored
        | AuthorityProgressStage::CasChunkVerified
        | AuthorityProgressStage::CasManifestStored
        | AuthorityProgressStage::CasManifestVerified
        | AuthorityProgressStage::PackStaged
        | AuthorityProgressStage::SprStaged
        | AuthorityProgressStage::PackVerified
        | AuthorityProgressStage::SprVerified
        | AuthorityProgressStage::Published => Some(SpaceArtifactCreationStageV1::Publishing),
        AuthorityProgressStage::Preflight
        | AuthorityProgressStage::CatalogLoading
        | AuthorityProgressStage::CatalogResolved
        | AuthorityProgressStage::InputValidated
        | AuthorityProgressStage::ApplyingOperations
        | AuthorityProgressStage::OutputValidated
        | AuthorityProgressStage::Derived
        | AuthorityProgressStage::CasSweep => None,
    }}
}}

#[cfg(feature = "native-artifact-execution")]
impl AuthorityOperationControl for ArtifactCreationHttpControlV1 {{
"""),
    (BOOTSTRAP,
     "cancelled.load(std::sync::atomic::Ordering::Acquire))\n    }\n\n    fn report(&self, _progress: AuthorityProgress) {}\n\n    fn fault(&self, detail: &str) {\n        if let Ok(mut held) = self.fault.lock() {\n            held.get_or_insert_with(|| detail.to_owned());\n        }\n    }\n}\n",
     f"""cancelled.load(std::sync::atomic::Ordering::Acquire))
    }}

    fn report(&self, progress: AuthorityProgress) {{
        let Some(stage) = artifact_creation_stage(progress.stage) else {{ return }};
        let total_units = progress.total_units.max(1);
        let reported = {P}::SpaceArtifactCreationProgressV1 {{ stage, completed_units: progress.completed_units.min(total_units), total_units }};
        if let Ok(mut held) = self.progress.lock() {{
            *held = Some(reported);
        }}
    }}

    fn fault(&self, detail: &str) {{
        if let Ok(mut held) = self.fault.lock() {{
            held.get_or_insert_with(|| detail.to_owned());
        }}
    }}
}}
"""),
    (BOOTSTRAP,
     "        state.reservations.contains_key(key) || state.tasks.contains_key(key)\n    }\n\n    fn start_recovery(",
     f"""        state.reservations.contains_key(key) || state.tasks.contains_key(key)
    }}

    /// 📈️ The progress of the live execution this process runs for `key`, if it runs one.
    fn live_progress(&self, key: &str) -> Option<{P}::SpaceArtifactCreationProgressV1> {{
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.tasks.retain(|_, task| !task.task.is_finished());
        state.tasks.get(key).map(|task| task.control.progress()).or_else(|| state.reservations.get(key).map(|pending| pending.control.progress()))
    }}

    fn start_recovery("""),
    (BOOTSTRAP,
     "        Ok(status) => artifact_creation_status_response(status),\n        Err(error) => artifact_creation_error_status(error).into_response(),\n    }\n}\n",
     """        Ok(mut status) => {
            if matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing) {
                status.progress = state.artifact_creation_tasks.live_progress(&artifact_creation_task_key_v1(&actor.user_id, &space_id, &request_id));
            }
            artifact_creation_status_response(status)
        }
        Err(error) => artifact_creation_error_status(error).into_response(),
    }
}
"""),
]


def main() -> int:
    reverse = "--reverse" in sys.argv
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS}}
    failures = 0
    for index, (path, before, after) in enumerate(EDITS):
        find, put = (after, before) if reverse else (before, after)
        count = texts[path].count(find)
        if count != 1:
            print(f"edit {index} {path.name}: anchor found {count} times", file=sys.stderr)
            failures += 1
            continue
        texts[path] = texts[path].replace(find, put, 1)
        print(f"edit {index} {path.parent.name}/{path.name}: ok")
    if failures:
        return 1
    if not dry:
        for path, text in texts.items():
            path.write_text(text, encoding="utf-8")
        if reverse:
            if ORACLE.exists() and ORACLE.read_bytes() == ORACLE_SOURCE.read_bytes():
                ORACLE.unlink()
                ORACLE.parent.rmdir()
        else:
            ORACLE.parent.mkdir(exist_ok=True)
            ORACLE.write_bytes(ORACLE_SOURCE.read_bytes())
    return 0


if __name__ == "__main__":
    sys.exit(main())
