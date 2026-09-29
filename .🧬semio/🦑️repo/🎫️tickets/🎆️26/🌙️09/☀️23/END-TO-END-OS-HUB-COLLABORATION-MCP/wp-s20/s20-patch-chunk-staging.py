"""📥️ S20 window-3 prepared patch (phase 1): the framework reassembles every chunked file-open import (coordinator decision
28 13:5x) — domain-neutral, bounded memory, progress + cancellation, laws.

A shell dispatches a picked file as `import_chunk_arguments` chunks of ≤ 32 KiB (`{payload, name, chunk, chunkCount}`,
kernel region 📤️FileOpenImport). Until now every app staged them itself or not at all: generation3d and puzzle3d each
hand-rolled a staging, architect CSV / puzzle2d / puzzle5d / note / shooting refused or misread anything above one chunk
(io-matrix 27 19:5x). This set:

- kernel (`🎠️kernel/🦀️.rs`, beside `import_chunk_arguments`): `ImportStaging` — one app instance's open runs
  (`IMPORT_STAGING_RUNS` = 2 slots, ≤ `IMPORT_STAGING_MAXIMUM_CHUNKS` = 128 chunks = 4 MiB each, LRU slot reuse, restart on a
  re-pick, retransmission acknowledged, gaps refused with typed codes `file-import.*`); `admit_args` turns an invocation's
  arguments into `NotAnImport` / `Staged {next, count}` / `Whole(args)` where `args` = `payload` + `name` (+ fan-out + every
  other argument) and never the chunk envelope; `cancel_all`; `open_runs` census.
- laws: a fixture-driven Rust law over 10 language-agnostic `stagingCases` added to `🧫️fixtures/📤️file-open-import/🔣️.json`
  (the fixture both twins already drive), plus pass-through and cancel/byte-authority laws.
- SDK (`🔌️plugin/🦀️.rs`): `VcsArtifactApp.import_staging`; `dispatch_action` and `dispatch_command` admit the arguments
  before the app decodes them — a staged chunk answers with an empty result, a whole file reaches the app's action as one
  invocation.
- host (`🛠️ShellHelpers` `dispatchOpenedFiles` + `🏛️ShellHost`): every picked-file import is a Tasks-window task
  (`documentTransfer` lane, chunk progress, Cancel stops sending; the guest's unfinished run gives its slot to the next pick),
  sharing one tracker with Import Document.

Phase 2 (same window-3 landing, after phase 1 compiles): generation3d + puzzle3d drop their private stagings (their import
actions then decode `{payload, name}`), architect gets a CSV picker verb, puzzle2d/5d raise their import contracts.

Usage: python3 s20-patch-chunk-staging.py [--dry-run]   (idempotent)
"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent / "s20-chunk-staging"
DRY = "--dry-run" in sys.argv
KERNEL = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs"
KERNEL_TESTS = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/📤️file-open-import/🦀️.rs"
FIXTURE = "🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/📤️file-open-import/🔣️.json"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
HELPERS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx"
HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"

KERNEL_ANCHOR = '#[cfg(test)]\n#[path = "🧪️tests/📤️file-open-import/🦀️.rs"]\nmod file_open_import_tests;\n//#endregion 📤️FileOpenImport\n'
REFUSAL_FAULT = "|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message())"

def hunks() -> list[tuple[str, str, str]]:
    kernel_code = (HERE / "kernel.rs.txt").read_text()
    return [
        (KERNEL, KERNEL_ANCHOR, kernel_code.lstrip("\n") + "\n" + KERNEL_ANCHOR),
        (SDK, "        close_store_replacement_jobs_drained: bool,\n",
         "        close_store_replacement_jobs_drained: bool,\n        /// 📥️ This instance's open file-import runs: every chunked pick is reassembled here, so an app's import\n        /// action only ever decodes a whole file (`semio_framework::kernel::ImportStaging`).\n        import_staging: semio_framework::kernel::ImportStaging,\n"),
        (SDK, "                close_store_replacement_jobs_drained: false,\n",
         "                close_store_replacement_jobs_drained: false,\n                import_staging: semio_framework::kernel::ImportStaging::default(),\n"),
        (SDK,
         '            validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;\n            if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {\n',
         '            validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;\n'
         f"            let whole_import = match self.import_staging.admit_args(args).map_err({REFUSAL_FAULT})? {{\n"
         "                semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Self::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),\n"
         "                semio_framework::kernel::ImportArguments::Whole(whole) => Some(whole),\n"
         "                semio_framework::kernel::ImportArguments::NotAnImport => None,\n"
         "            };\n"
         "            let args = whole_import.as_ref().or(args);\n"
         "            if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {\n"),
        (SDK,
         "            let args = DslValue::Object(invocation.arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());\n            if let Some(config_mutation) = A::host_configuration_mutation(command_id, Some(&args))? {\n",
         "            let args = DslValue::Object(invocation.arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());\n"
         f"            let args = match self.import_staging.admit_args(Some(&args)).map_err({REFUSAL_FAULT})? {{\n"
         "                semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Self::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),\n"
         "                semio_framework::kernel::ImportArguments::Whole(whole) => whole,\n"
         "                semio_framework::kernel::ImportArguments::NotAnImport => args,\n"
         "            };\n"
         "            if let Some(config_mutation) = A::host_configuration_mutation(command_id, Some(&args))? {\n"),
        (HELPERS,
         """ * 🧯 The chunks of one file are dispatched in ORDER and awaited one at a time: the guest's staging area
 * refuses a gap rather than resuming into bytes nobody can account for, so a concurrent fan-out would
 * cost the whole file. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
): Promise<void> {
  const total = opened.length;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    for (const page of importPayloadChunks(file.contents)) {
      await dispatchOne(importAction, importChunkArguments(file.name, page, multiple ? { index, total } : undefined));
    }
  }
}
""",
         """ * 🧯 The chunks of one file are dispatched in ORDER and awaited one at a time: the guest's staging
 * (`ImportStaging`, `🎠️kernel/🦀️.rs`) refuses a gap rather than resuming into bytes nobody can account for, so a
 * concurrent fan-out would cost the whole file. `progress(completed, total)` counts chunks over every picked file; an
 * aborted `signal` stops before the next chunk and the guest's unfinished run gives its slot to the next pick. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
  signal?: AbortSignal,
  progress?: (completed: number, total: number) => void,
): Promise<void> {
  const total = opened.length;
  const pages = opened.map((file) => importPayloadChunks(file.contents));
  const chunks = pages.reduce((sum, filePages) => sum + filePages.length, 0);
  let completed = 0;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    for (const page of pages[index]!) {
      signal?.throwIfAborted();
      await dispatchOne(importAction, importChunkArguments(file.name, page, multiple ? { index, total } : undefined));
      completed += 1;
      progress?.(completed, chunks);
    }
  }
}
"""),
        (HOST,
         """    const id = crypto.randomUUID();
    const abort = new AbortController();
    const transfer = (patch: Partial<TaskManagerDocumentTransferV1>): void => {
      const entry = documentTransfersRef.current.get(id);
      if (entry === undefined) return;
      documentTransfersRef.current.set(id, { ...entry, transfer: { ...entry.transfer, ...patch } });
      publishDocumentTransfers();
    };
    documentTransfersRef.current.set(id, { transfer: { id, file: picked.name, pluginId: owned.target.pluginId, startedAtMs: Date.now(), progress: null, cancelling: false }, abort });
    abort.signal.addEventListener("abort", () => transfer({ cancelling: true }), { once: true });
    publishDocumentTransfers();
    let instanceId: number | null = null;
""",
         """    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
"""),
        (HOST,
         "      await load(instanceId, archive, abort.signal, (status) => transfer({ progress: { completed: status.completed, total: status.total > 0 ? status.total : null, text: `${status.completed}/${status.total}` } }));\n",
         "      await load(instanceId, archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));\n"),
        (HOST,
         """      const notice: DocumentTransferNoticeV1 = abort.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, abort.signal.aborted ? "info" : "error");
    } finally {
      documentTransfersRef.current.delete(id);
      publishDocumentTransfers();
    }
  };
  const documentTransferRef = useRef({ exportDocumentArchive, importDocumentArchive });
  documentTransferRef.current = { exportDocumentArchive, importDocumentArchive };
""",
         """      const notice: DocumentTransferNoticeV1 = task.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  const documentTransferRef = useRef({ exportDocumentArchive, importDocumentArchive, trackDocumentTransfer, notifyDocumentTransfer });
  documentTransferRef.current = { exportDocumentArchive, importDocumentArchive, trackDocumentTransfer, notifyDocumentTransfer };
"""),
        (HOST,
         "  /** 📤️ The focused document program, or `null` when the Home or the space host has the canvas. */\n",
         """  /** 🧵️ One file moving into a program as a cancellable Tasks-window task (`documentTransfer` lane): `progress` states
   * `completed` of `total` units (`null` = open-ended), Cancel aborts `signal`, `finish` removes the row. */
  const trackDocumentTransfer = (file: string, pluginId: string): { readonly signal: AbortSignal; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void } => {
    const id = crypto.randomUUID();
    const abort = new AbortController();
    const update = (patch: Partial<TaskManagerDocumentTransferV1>): void => {
      const entry = documentTransfersRef.current.get(id);
      if (entry === undefined) return;
      documentTransfersRef.current.set(id, { ...entry, transfer: { ...entry.transfer, ...patch } });
      publishDocumentTransfers();
    };
    documentTransfersRef.current.set(id, { transfer: { id, file, pluginId, startedAtMs: Date.now(), progress: null, cancelling: false }, abort });
    abort.signal.addEventListener("abort", () => update({ cancelling: true }), { once: true });
    publishDocumentTransfers();
    return {
      signal: abort.signal,
      progress: (completed, total) => update({ progress: { completed, total, text: `${completed}/${total ?? "?"}` } }),
      finish: () => {
        documentTransfersRef.current.delete(id);
        publishDocumentTransfers();
      },
    };
  };
  /** 📤️ The focused document program, or `null` when the Home or the space host has the canvas. */
"""),
        (HOST,
         """              // 📤️ Single-file (multiple absent/false): identical to the pre-multi-select shape, one
              // `handleAction` call with `{payload, name}`. Multi-file: one sequential call per selected
              // file, each extending args with `{index, total}` so the plugin can stage/merge imports.
              await dispatchOpenedFiles(opened, resolvedImport, Boolean(multiple), makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null }));
""",
         """              const task = documentTransferRef.current.trackDocumentTransfer(opened.map((file) => file.name).join(", "), baseSession.pluginId);
              try {
                await dispatchOpenedFiles(opened, resolvedImport, Boolean(multiple), makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null }), task.signal, task.progress);
              } catch (error) {
                if (!task.signal.aborted) throw error;
                documentTransferRef.current.notifyDocumentTransfer("import-cancelled", opened[0]!.name, "info");
              } finally {
                task.finish();
              }
"""),
    ]


def fixture_text(text: str) -> str:
    cases = json.loads((HERE / "staging-cases.json").read_text())
    if "stagingCases" in json.loads(text):
        return text
    body = text.rstrip()
    assert body.endswith("}"), "fixture must end with its closing brace"
    addition = ",\n".join(f'  {json.dumps(key, ensure_ascii=False)}: {json.dumps(value, ensure_ascii=False, indent=2).replace(chr(10), chr(10) + "  ")}' for key, value in cases.items())
    return body[:-1].rstrip() + ",\n" + addition + "\n}\n"



#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [('🧰️framework/🔨️modules/🎠️kernel/🦀️.rs', 'pub struct ImportStaging {')]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    tree = Path("/Users/ueli/Documents/semio")
    present = [(not (tree / rel).exists()) if marker is None else ((tree / rel).exists() and marker in (tree / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def main() -> None:
    if landed_guard():
        return
    texts: dict[str, str] = {}
    notes = []
    for rel, old, new in hunks():
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        if new in text:
            notes.append(f"applied   {rel.split('/')[-2]}: {old.strip().splitlines()[0][:70]}")
        elif text.count(old) == 1:
            texts[rel] = text.replace(old, new)
            notes.append(f"apply     {rel.split('/')[-2]}: {old.strip().splitlines()[0][:70]}")
        else:
            notes.append(f"CONFLICT  {rel}: anchor found {text.count(old)}× — {old.strip().splitlines()[0][:70]}")
    tests = texts.setdefault(KERNEL_TESTS, (ROOT / KERNEL_TESTS).read_text())
    if "fn every_staging_case_reassembles_or_refuses_exactly_as_the_fixture_states" in tests:
        notes.append("applied   kernel staging laws")
    else:
        texts[KERNEL_TESTS] = tests.rstrip("\n") + "\n" + (HERE / "kernel-tests.rs.txt").read_text()
        notes.append("apply     kernel staging laws")
    fixture = texts.setdefault(FIXTURE, (ROOT / FIXTURE).read_text())
    patched = fixture_text(fixture)
    json.loads(patched)
    notes.append("apply     fixture stagingCases" if patched != fixture else "applied   fixture stagingCases")
    texts[FIXTURE] = patched
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    changed = [rel for rel, text in texts.items() if text != (ROOT / rel).read_text()]
    if not DRY:
        for rel in changed:
            (ROOT / rel).write_text(texts[rel])
    print(f"{'dry-run' if DRY else 'applied'}: {len(changed)} files {'would change' if DRY else 'changed'}")


if __name__ == "__main__":
    main()
