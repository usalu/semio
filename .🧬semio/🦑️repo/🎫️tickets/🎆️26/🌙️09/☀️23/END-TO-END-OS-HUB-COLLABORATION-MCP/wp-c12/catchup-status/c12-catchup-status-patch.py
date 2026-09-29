"""⏳️ C12 T4 set — the "catching up" execution-target status (coordinator 14c): while a mounted document drains the hub tail retained
before its actor was bound, the shell shows "Catching up with the hub…" with progress (tail messages delivered / retained) and the
notice's Cancel (closing the document cleanly, the existing `docAbort` → `cancelled`); it clears when the surface mounts. Input in
that phase is already refused typed (`action-catching-up`, landed 05:1x).

Schema-first, all twins at once:
  * vocabulary fixture `🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json` (`expected.status` +
    `expected.statusRoles`) gains `catching-up` (en/de, role `status`);
  * TS twin (`📇️directory/🧬️schema/🟦️.ts`: code, text, role, progress stage `catch-up`) — its law already replays the fixture;
  * Rust twin (`📇️directory/🧬️schema/🦀️.rs`): the enum had drifted to 5 of 8 codes (no `retrying`, `link-expired`,
    `access-revoked`) → all 9 codes, texts, roles; NEW Rust law replaying the fixture's status + roles against the enum;
  * hub oracle (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`): status inventory + role rule;
  * worker: `flushPendingBackbone` reports `catching-up` progress per delivered tail message.
Idempotent; usage: python3 c12-catchup-status-patch.py [--apply]   (default dry run; C12_REPO overrides the tree)"""
import json
import os
import sys

REPO = os.environ.get("C12_REPO", "/Users/ueli/Documents/semio")
APPLY = "--apply" in sys.argv
FIXTURE = "🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json"
TS = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts"
RS = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"
RS_LAW = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs"
HUB = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
WORKER = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"

EN = "Catching up with the hub…"
DE = "Gleiche mit dem Hub ab…"

HUNKS = [
    (TS,
     'export type DocumentExecutionTargetStatusCodeV1 = "verifying" | "retrying" | "integrity-failed" | "stale" | "cancelled" | "renderer-unavailable" | "link-expired" | "access-revoked";\n',
     'export type DocumentExecutionTargetStatusCodeV1 = "verifying" | "retrying" | "catching-up" | "integrity-failed" | "stale" | "cancelled" | "renderer-unavailable" | "link-expired" | "access-revoked";\n'),
    (TS,
     '  retrying: Object.freeze({ en: "The hub is busy. Asking again for the document component…", de: "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…" }),\n',
     '  retrying: Object.freeze({ en: "The hub is busy. Asking again for the document component…", de: "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…" }),\n'
     f'  "catching-up": Object.freeze({{ en: "{EN}", de: "{DE}" }}),\n'),
    (TS,
     '  return code === "verifying" || code === "retrying" ? "status" : "alert";\n',
     '  return code === "verifying" || code === "retrying" || code === "catching-up" ? "status" : "alert";\n'),
    (TS,
     '  | "actor-ready"\n  | "canonical-pair";\n',
     '  | "actor-ready"\n  | "canonical-pair"\n  | "catch-up";\n'),
    (TS,
     '/** 📈️ Bounded install progress. It never carries bytes, paths, receipts or full digests. */\n',
     '/** 📈️ Bounded install progress. It never carries bytes, paths, receipts or full digests. For the `catch-up` stage the two counts\n'
     ' * are hub tail messages (delivered / retained), not bytes. */\n'),
    (RS,
     "pub enum DocumentExecutionTargetStatusCodeV1 {\n    Verifying,\n    IntegrityFailed,\n    Stale,\n    Cancelled,\n    RendererUnavailable,\n}\n",
     "pub enum DocumentExecutionTargetStatusCodeV1 {\n    Verifying,\n    Retrying,\n    CatchingUp,\n    IntegrityFailed,\n    Stale,\n    Cancelled,\n    RendererUnavailable,\n    LinkExpired,\n    AccessRevoked,\n}\n"),
    (RS,
     '            (Self::Verifying, DocumentExecutionTargetLocaleV1::De) => "Dokumentkomponente wird überprüft…",\n',
     '            (Self::Verifying, DocumentExecutionTargetLocaleV1::De) => "Dokumentkomponente wird überprüft…",\n'
     '            (Self::Retrying, DocumentExecutionTargetLocaleV1::En) => "The hub is busy. Asking again for the document component…",\n'
     '            (Self::Retrying, DocumentExecutionTargetLocaleV1::De) => "Der Hub ist ausgelastet. Die Dokumentkomponente wird erneut angefragt…",\n'
     f'            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::En) => "{EN}",\n'
     f'            (Self::CatchingUp, DocumentExecutionTargetLocaleV1::De) => "{DE}",\n'),
    (RS,
     '            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::De) => "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.",\n',
     '            (Self::RendererUnavailable, DocumentExecutionTargetLocaleV1::De) => "Die überprüfte Dokumentkomponente ist bereit, aber dieser Renderer ist nicht verfügbar.",\n'
     '            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::En) => "The connection was lost for too long. Reconnect to keep editing this document.",\n'
     '            (Self::LinkExpired, DocumentExecutionTargetLocaleV1::De) => "Die Verbindung war zu lange unterbrochen. Verbinden Sie sich erneut, um dieses Dokument weiter zu bearbeiten.",\n'
     '            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::En) => "Your access to this document was removed.",\n'
     '            (Self::AccessRevoked, DocumentExecutionTargetLocaleV1::De) => "Ihr Zugriff auf dieses Dokument wurde entfernt.",\n'),
    (RS,
     '            Self::Verifying => "status",\n',
     '            Self::Verifying | Self::Retrying | Self::CatchingUp => "status",\n'),
    (RS_LAW,
     "/// 🪪️ The one shared full-field lease relation, driven by the language-neutral\n",
     "/// 🌐️ The execution-target status vocabulary is ONE vocabulary: every code of the language-neutral corpus\n"
     "/// (`document-execution-target-lease-v1` `expected.status` / `statusRoles`) decodes to the Rust twin with the corpus's exact\n"
     "/// English and German text and live-region role, and the twin has no code the corpus lacks.\n"
     "#[test]\n"
     "fn execution_target_status_vocabulary_matches_the_corpus() {\n"
     "    use crate::os_directory::schema::{DocumentExecutionTargetLocaleV1, DocumentExecutionTargetStatusCodeV1};\n"
     "    let corpus: serde_json::Value = serde_json::from_str(include_str!(\"../../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json\")).expect(\"execution target lease corpus\");\n"
     "    let status = corpus[\"expected\"][\"status\"].as_object().expect(\"corpus status vocabulary\");\n"
     "    for (code, text) in status {\n"
     "        let decoded: DocumentExecutionTargetStatusCodeV1 = crate::os_pack::json::from_json_str(&format!(\"\\\"{code}\\\"\")).unwrap_or_else(|_| panic!(\"the Rust twin lacks status {code}\"));\n"
     "        assert_eq!(decoded.text(DocumentExecutionTargetLocaleV1::En), text[\"en\"].as_str().expect(\"en\"), \"{code} en\");\n"
     "        assert_eq!(decoded.text(DocumentExecutionTargetLocaleV1::De), text[\"de\"].as_str().expect(\"de\"), \"{code} de\");\n"
     "        assert_eq!(decoded.aria_role(), corpus[\"expected\"][\"statusRoles\"][code.as_str()].as_str().expect(\"role\"), \"{code} role\");\n"
     "    }\n"
     "    assert_eq!(status.len(), 9, \"the corpus names every status the twins speak\");\n"
     "}\n\n"
     "/// 🪪️ The one shared full-field lease relation, driven by the language-neutral\n"),
    (HUB,
     '  const leaseStatusKeys = ["verifying", "retrying", "integrity-failed", "stale", "cancelled", "renderer-unavailable", "link-expired", "access-revoked"];\n',
     '  const leaseStatusKeys = ["verifying", "retrying", "catching-up", "integrity-failed", "stale", "cancelled", "renderer-unavailable", "link-expired", "access-revoked"];\n'),
    (HUB,
     '    if (role !== (code === "verifying" || code === "retrying" ? "status" : "alert")) throw new Error(`execution target lease corpus status ${code} has the wrong live-region role`);\n',
     '    if (role !== (code === "verifying" || code === "retrying" || code === "catching-up" ? "status" : "alert")) throw new Error(`execution target lease corpus status ${code} has the wrong live-region role`);\n'),
    (WORKER,
     "      while (this.pendingBackboneBeforeBinding.length > 0) {\n        const queued = this.pendingBackboneBeforeBinding;\n        this.pendingBackboneBeforeBinding = [];\n        this.pendingBackboneBeforeBindingBytes = 0;\n"
     "        for (const payload of queued) if (!(await binding.port.receive(this.currentDocumentSource(), payload))) throw new Error(\"actor-document-port.stale-queued-message\");\n      }\n",
     "      const hub = hubBinding(this.state.config);\n      let delivered = 0;\n"
     "      while (this.pendingBackboneBeforeBinding.length > 0) {\n        const queued = this.pendingBackboneBeforeBinding;\n        this.pendingBackboneBeforeBinding = [];\n        this.pendingBackboneBeforeBindingBytes = 0;\n"
     "        const retained = delivered + queued.length;\n"
     "        for (const payload of queued) {\n"
     "          if (hub !== null) emitExecutionTargetStatus(this.state, hub, \"catching-up\", { stage: \"catch-up\", completedBytes: delivered, totalBytes: retained });\n"
     "          if (!(await binding.port.receive(this.currentDocumentSource(), payload))) throw new Error(\"actor-document-port.stale-queued-message\");\n"
     "          delivered += 1;\n        }\n      }\n"),
    (WORKER,
     "  /** 📥️ Delivers the backbone retained before the binding (a catch-up tail among it) in arrival order, then opens the\n",
     "  /** 📥️ Delivers the backbone retained before the binding (a catch-up tail among it) in arrival order — reporting\n"
     "   * `catching-up` progress per delivered message, which the mounted surface clears — then opens the\n"),
]


def main():
    problems, planned, texts = [], [], {}
    fixture_path = os.path.join(REPO, FIXTURE)
    raw = open(fixture_path, encoding="utf-8").read()
    corpus = json.loads(raw)
    if json.dumps(corpus, indent=2, ensure_ascii=False) + "\n" != raw:
        problems.append(("fixture-format", "the corpus no longer round-trips at indent 2"))
    elif "catching-up" in corpus["expected"]["status"]:
        planned.append(("present", FIXTURE))
    else:
        status = {}
        for key, value in corpus["expected"]["status"].items():
            status[key] = value
            if key == "retrying":
                status["catching-up"] = {"en": EN, "de": DE}
        roles = {}
        for key, value in corpus["expected"]["statusRoles"].items():
            roles[key] = value
            if key == "retrying":
                roles["catching-up"] = "status"
        corpus["expected"]["status"], corpus["expected"]["statusRoles"] = status, roles
        texts[FIXTURE] = json.dumps(corpus, indent=2, ensure_ascii=False) + "\n"
        planned.append(("fixture", FIXTURE))
    for path, old, new in HUNKS:
        text = texts.get(path) or open(os.path.join(REPO, path), encoding="utf-8").read()
        if new in text:
            planned.append(("present", path))
            continue
        if text.count(old) != 1:
            problems.append((f"anchor-count-{text.count(old)}", f"{path} :: {old[:70]!r}"))
            continue
        texts[path] = text.replace(old, new, 1)
        planned.append(("hunk", path))
    for kind, what in planned:
        print(f"OK   {kind:8} {what}")
    for kind, what in problems:
        print(f"FAIL {kind} {what}")
    print(f"{len(planned)} planned, {len(problems)} problem(s), mode={'apply' if APPLY else 'dry-run'}")
    if problems:
        sys.exit(1)
    if APPLY:
        for path, text in texts.items():
            with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
                handle.write(text)


main()
