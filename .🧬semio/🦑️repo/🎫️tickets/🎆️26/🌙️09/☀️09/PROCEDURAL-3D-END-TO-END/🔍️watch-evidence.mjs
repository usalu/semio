/** 🎯️ Validates evidence tied to one explicitly named mounted probe run. */
export function runId(value) {
  if (!value || !/^[A-Za-z0-9_-]{1,64}$/.test(value)) throw Error("SEMIO_PROBE_RUN_ID must be a unique safe run identifier");
  return value;
}
export function deadlineMs(value) {
  const seconds = Number(value ?? 10800);
  if (!Number.isFinite(seconds) || seconds <= 0) throw Error("SEMIO_PROBE_DEADLINE must be positive seconds");
  return seconds * 1000;
}
export function reactEvidence(id, directory, summary, hover, started, modified, now) {
  return started?.id === id && Number.isFinite(started.started) && Number.isFinite(started.deadline) && now <= started.deadline && modified >= started.started && directory.startsWith(`probe-canonical-react-${id}-`) && summary?.label === `canonical-react-${id}` && summary.ok === true && hover?.graphToPreview === true && hover?.previewToGraph === true && hover?.cleared === true && !hover.error;
}
export function wgpuEvidence(boot, rows, only) {
  if (!boot?.booted || !Array.isArray(rows) || rows.some(row => /^(fail|blocked)/.test(row.verdict ?? ""))) return false;
  const required = only === "boot" ? ["boot:edit-editor"] : ["boot:edit-editor", "edit:box-shell-bidirectional-hover", "world3d:hover", "world3d:selection-click", "world3d:wheel-orbit"];
  return required.every(label => rows.some(row => row.label === label && row.verdict === "pass"));
}
